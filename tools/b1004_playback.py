"""B-1004 (D-1141): where a frame's time goes when the processor plays it from memory, and how many
frames playback passes over.

    python tools/b1004_playback.py measure <app.exe> <tag>   # before / after: into verification/B-1004_playback_measured.json
    python tools/b1004_playback.py report                    # verification/B-1004_playback_table.md

Each run is a fresh copy of the app on its own web view profile, placed off the screens
(ANIME_COMPOSITOR_BACKGROUND=1), driven over its debugging port as tools/p28_media.py does. Play
at Full, wait until every frame of the work area is in memory, then time three loops. The page
is timed from inside by wrapping `fetch`, `putImageData` and `show` (nothing in the app is
changed to measure): for each frame asked for while playing,

    ask        the request's start to the answer's headers, less the window's own time (x-ms)
    window     the window's own time for the frame (x-ms)
    bytes      the answer's headers to its last byte in the page (arrayBuffer)
    put        putImageData onto the canvas
    rest       the rest of the page's work on the answer (show() after the bytes, less put)
    wait       show() done to the next frame asked for (the page's timer, requestAnimationFrame)

Their sum is one round, the time between two frames asked for. The window's session log gives
the frames passed over (`dropped`), the count D-1141's pass line reads.
"""

import json
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
import p28_media as p28  # noqa: E402

OUT = ROOT / "verification"
FACTS = OUT / "B-1004_playback_measured.json"
SHOTS = {  # name: (project, media folder, frames in the work area, fps)
    "P-28 shot": (OUT / "P-28" / "p28_shot.json", ROOT / "target" / "p28", 120, 24),
    "reference shot": (OUT / "B-08a_project.json", ROOT / "Fixtures" / "reference_shot", 240, 24),
}

WATCH = r"""(() => {
  if (window.__m) return true;
  const R = window.__m = { asks: [], raf: [] };
  const of = window.fetch;
  window.fetch = function (u) {
    if (!/\/(at|play)\//.test(String(u))) return of.apply(this, arguments);
    const e = { t0: performance.now() };
    R.asks.push(e);
    return of.apply(this, arguments).then((r) => {
      e.t1 = performance.now();
      e.ms = +(r.headers.get('x-ms') || 0);
      e.frame = +r.headers.get('x-frame');
      e.skipped = +(r.headers.get('x-skipped') || 0);
      e.onscreen = r.headers.get('x-on-screen') === '1';
      const ab = r.arrayBuffer.bind(r);
      r.arrayBuffer = () => ab().then((b) => { e.t2 = performance.now(); e.bytes = b.byteLength; return b; });
      return r;
    });
  };
  const put = CanvasRenderingContext2D.prototype.putImageData;
  CanvasRenderingContext2D.prototype.putImageData = function () {
    const a = performance.now();
    put.apply(this, arguments);
    const e = R.asks[R.asks.length - 1];
    if (e && this === paint) e.put = (e.put || 0) + performance.now() - a;
  };
  const shown = window.show;
  window.show = function () {
    const n = R.asks.length;
    const p = shown.apply(this, arguments);
    if (R.asks.length > n) { const e = R.asks[R.asks.length - 1]; p.then(() => { e.t3 = performance.now(); }); }
    return p;
  };
  const loop = (t) => { R.raf.push(t); requestAnimationFrame(loop); };
  requestAnimationFrame(loop);
  return true;
})()"""

# Played, the clock is stopped, and the same transport is timed by itself on frames from memory:
# a whole frame at each size, an answer with no pixels (D-886: the frame the page already holds),
# and several whole frames asked for at once. Each size is asked once first, so it is in memory.
PROBE = r"""(async () => {
  const one = async (url) => {
    const t0 = performance.now(); const r = await fetch(url); const t1 = performance.now();
    const b = await r.arrayBuffer();
    return { headers: t1 - t0, body: performance.now() - t1, ms: +(r.headers.get('x-ms') || 0),
             bytes: b.byteLength, sent: +(r.headers.get('x-sent') || 0), cached: r.headers.get('x-cached') };
  };
  const out = { sizes: {}, empty: [], parallel: {} };
  for (const q of ['full', 'half', 'third', 'draft']) {
    await one(FRAMES + '/frame/40?q=' + q + '&have=0');
    await one(FRAMES + '/frame/41?q=' + q + '&have=0');
    out.sizes[q] = [];
    for (let i = 0; i < 6; i++) out.sizes[q].push(await one(FRAMES + '/frame/' + (40 + i % 2) + '?q=' + q + '&have=0'));
  }
  let held = (await one(FRAMES + '/frame/40?q=full&have=0')).sent;
  for (let i = 0; i < 6; i++) { const a = await one(FRAMES + '/frame/40?q=full&have=' + held); held = a.sent; out.empty.push(a); }
  for (const k of [1, 2, 3, 4]) {
    out.parallel[k] = [];
    for (let i = 0; i < 3; i++) {
      const t0 = performance.now();
      await Promise.all([...Array(k).keys()].map((j) => fetch(FRAMES + '/frame/' + (40 + j) + '?q=full&have=0').then((r) => r.arrayBuffer())));
      out.parallel[k].push(performance.now() - t0);
    }
  }
  return JSON.stringify(out);
})()"""


def median(xs):
    xs = sorted(xs)
    return xs[len(xs) // 2] if xs else None


def pct(xs, q):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(q * len(xs)))] if xs else None


def play(frames, fps):
    def job(app):
        app.switch("cpu")
        app.eval(WATCH)
        app.get("PROJECT + '/session-log?do=on'")
        app.get("PROJECT + '/session-log?do=clear'")
        app.eval("quality = 'full'; wanted = 0; frame = 0; document.getElementById('play').click(); true")
        t0 = time.perf_counter()
        ready = None
        while time.perf_counter() - t0 < 900:
            ready = json.loads(app.get("FRAMES + '/ready'"))
            if len(ready["frames"]) >= frames:
                break
            time.sleep(0.25)
        filled = round(time.perf_counter() - t0, 2)
        app.get("PROJECT + '/session-log?do=clear'")
        app.eval("window.__m.asks = []; window.__m.raf = []; true")
        t1 = time.perf_counter()
        time.sleep(3 * frames / fps)
        log = json.loads(app.get("PROJECT + '/session-log'"))
        looped = round(time.perf_counter() - t1, 2)
        m = json.loads(app.eval("JSON.stringify(window.__m)"))
        app.eval("if (playing) document.getElementById('play').click(); true")
        time.sleep(0.5)
        probe = json.loads(app.eval(PROBE))
        asks = m["asks"]
        rows = []
        for a, b in zip(asks, asks[1:]):
            if "t3" not in a or "t2" not in a:
                continue
            put = a.get("put", 0.0)
            rows.append({"frame": a["frame"], "skipped": a["skipped"], "bytes": a.get("bytes", 0),
                         "ask": a["t1"] - a["t0"] - a["ms"], "window": a["ms"], "bytes_ms": a["t2"] - a["t1"],
                         "put": put, "rest": a["t3"] - a["t2"] - put, "wait": b["t0"] - a["t3"],
                         "round": b["t0"] - a["t0"]})
        raf = m["raf"]
        gaps = [b - a for a, b in zip(raf, raf[1:])]
        keys = ("frames", "median_ms", "slowest_5_percent_ms", "dropped")
        return {"filled_s": filled, "held": len(ready["frames"]) if ready else None,
                "held_bytes": ready and ready.get("held"), "loops_s": looped, "expected": 3 * frames,
                "log": {k: log.get(k) for k in keys + ("shown",) if k in log},
                "asks": len(asks),
                "page_skipped": sum(r["skipped"] for r in rows), "rows": rows, "probe": probe,
                "raf_median": median(gaps), "raf_95": pct(gaps, 0.95), "raf_max": max(gaps) if gaps else None}
    return job


def measure(exe, tag):
    exe = Path(exe).resolve()
    facts = json.loads(FACTS.read_text(encoding="utf-8")) if FACTS.is_file() else {}
    ps = ("$c = Get-CimInstance Win32_Processor; $m = Get-CimInstance Win32_ComputerSystem; "
          "$o = Get-CimInstance Win32_OperatingSystem; "
          "\"$($c.Name.Trim()) | $($c.NumberOfCores) cores $($c.NumberOfLogicalProcessors) threads | "
          "$([math]::Round($m.TotalPhysicalMemory / 1GB, 1)) GB | $($o.Caption) $($o.Version)\"")
    run = {"exe": exe.name, "exe_written": time.strftime("%Y-%m-%dT%H:%M:%S", time.localtime(exe.stat().st_mtime)),
           "commit": subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"],
                                    capture_output=True, text=True).stdout.strip(),
           "machine": subprocess.run(["powershell", "-NoProfile", "-Command", ps], capture_output=True, text=True).stdout.strip(),
           "measured": time.strftime("%Y-%m-%dT%H:%M:%S"), "shots": {}}
    for name, (project, media, frames, fps) in SHOTS.items():
        p28.MEDIA = media  # App copies the media folders from here
        run["shots"][name] = p28.session(exe, project, play(frames, fps))  # an absolute path survives p28.OUT / it
        r = run["shots"][name]
        print(tag, name, "held", r["held"], "asked", r["asks"], "dropped", r["log"].get("dropped"), flush=True)
    facts[tag] = run
    FACTS.write_text(json.dumps(facts, indent=1) + "\n", encoding="utf-8", newline="\n")


def report():
    facts = json.loads(FACTS.read_text(encoding="utf-8"))
    md = ["# B-1004: playback on the processor from memory (D-1141)", "",
          "Written by `tools/b1004_playback.py report` from `B-1004_playback_measured.json`; every number measured, none estimated.",
          "Each run: a fresh release build copy on its own web view profile, the window placed off the screens",
          "(ANIME_COMPOSITOR_BACKGROUND=1), 1920x1080, Full, the processor (the card switched off), played until every",
          "frame of the work area was in memory, then three loops timed. **On screen was not measured**: every launch",
          "in this work is kept off the person's screens, so a window on screen could not be started.", ""]
    for tag, run in facts.items():
        md += [f"## {tag}", "",
               f"Machine: {run['machine']}. Build: `{run['exe']}` release, written {run['exe_written']}, checkout {run['commit']}, measured {run['measured']}.", "",
               "| shot | memory filled | frames held | three loops | frames asked for (page) | frames passed over (session log) | frames shown (session log) | page's rounds: median ms | slowest 5% ms |",
               "|---|---|---|---|---|---|---|---|---|"]
        for name, s in run["shots"].items():
            rounds = [r["round"] for r in s["rows"]]
            md.append(f"| {name} | {s['filled_s']} s | {s['held']} ({(s['held_bytes'] or 0) / 1e6:.0f} MB) | {s['loops_s']} s ({s['expected']} frames played) "
                      f"| {s['asks']} | **{s['log'].get('dropped')}** | {s['log'].get('shown', 'not counted')} "
                      f"| {median(rounds):.1f} | {pct(rounds, 0.95):.1f} |")
        md += ["", "Where one round goes, median and slowest 5% in ms (a round is the time between two frames asked for):", "",
               "| shot | ask | window | bytes | put | rest | wait | round | bytes a frame | page timer: median / slowest 5% / longest gap ms |",
               "|---|---|---|---|---|---|---|---|---|---|"]
        for name, s in run["shots"].items():
            R = s["rows"]
            cell = lambda k: f"{median([r[k] for r in R]):.1f} / {pct([r[k] for r in R], 0.95):.1f}"
            md.append(f"| {name} | {cell('ask')} | {cell('window')} | {cell('bytes_ms')} | {cell('put')} | {cell('rest')} | {cell('wait')} "
                      f"| {cell('round')} | {median([r['bytes'] for r in R])} | {s['raf_median']:.1f} / {s['raf_95']:.1f} / {s['raf_max']:.1f} |")
        md += ["", "The same transport timed alone, played and stopped, every frame from memory (median of six; ms from asking to the",
               "last byte in the page, less the window's own time):", "",
               "| shot | Full 1920x1080 | Half | Third | Quarter (Draft) | no pixels (D-886, the frame the page holds) | 1 / 2 / 3 / 4 whole frames asked at once: ms for all |",
               "|---|---|---|---|---|---|---|"]
        for name, s in run["shots"].items():
            if "probe" not in s:
                continue
            P = s["probe"]
            t = lambda xs: f"{median([x['headers'] + x['body'] - x['ms'] for x in xs]):.1f} ({median([x['bytes'] for x in xs]) / 1e6:.2f} MB)"
            par = " / ".join(f"{median(P['parallel'][k]):.1f}" for k in ("1", "2", "3", "4"))
            md.append(f"| {name} | {t(P['sizes']['full'])} | {t(P['sizes']['half'])} | {t(P['sizes']['third'])} | {t(P['sizes']['draft'])} | {t(P['empty'])} | {par} |")
        md.append("")
    md += ["ask: the request's start to the answer's headers, less the window's own time; window: the window's own time (x-ms);",
           "bytes: the headers to the last byte in the page; put: putImageData onto the canvas; rest: the rest of the page's work",
           "on the answer; wait: from that to the next frame asked for (the page's timer). At 24 frames a second a round must",
           "stay under 41.7 ms for nothing to be passed over.", ""]
    (OUT / "B-1004_playback_table.md").write_text("\n".join(md), encoding="utf-8", newline="\n")
    print("wrote", OUT / "B-1004_playback_table.md")


if __name__ == "__main__":
    if sys.argv[1] == "measure":
        measure(sys.argv[2], sys.argv[3])
    else:
        report()
