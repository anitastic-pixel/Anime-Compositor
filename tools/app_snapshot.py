"""Real frames out of the real program: opens a project in the release build, asks it for frames
and writes what it drew as PNG pictures, so an effect can be looked at as the app renders it.

    python tools/app_snapshot.py --project <file.json> --frames 0,12,24 [--out <folder>]
        [--on cpu|card|both] [--port N] [--exe <app.exe>] [--media <folder>] [--timeout S]

For every frame it writes, in --out (default verification/app_snapshots/<project name>):
  (each PNG is a small preview, --preview px wide; --full also keeps frame_NNNN_<side>_full.png)
  frame_NNNN_cpu.png   what the processor sends for /frame/N?q=full (Full quality, the comp's size)
  frame_NNNN_card.png  what the viewer canvas #view holds after the graphics card drew frame N
  *.json beside each   frame, size, quality, which processor drew it, the app's notes and errors
  summary.json         timings, and with --on both the largest and mean cpu-vs-card difference
Both pictures are 8-bit straight (not premultiplied) sRGB RGBA, which is what the app itself sends
(`to_srgb8_straight`, and the card's `Gpu::draw` gives the same bytes). The card picture goes
through the page's 2D canvas, which keeps colours premultiplied: exact where a pixel is opaque,
within rounding where it is partly see-through.

The card paints the window itself (B-45), under the page, so its picture is not in the canvas; the
tool asks for it the way the viewer's Snapshot button does (an empty /place, then the frame again),
which brings the card's own picture back from the card. The sidecar says whether the card had
painted it into the window first (`painted_in_window`).

It never needs the keyboard, the mouse or the front: everything goes through the web view's
debugging port. Each run copies the exe, the project and the media (--media, default the reference
shot's layer folders: media resolves from the project's folder) into its own folder under %TEMP%,
gives the web view its own fresh profile there, picks a free port unless --port is given, and at
the end stops only its own process tree (never closing the window, so window.txt is not written),
puts %APPDATA%/dev.anitastic.anime-compositor/window.txt and recent.txt back byte for byte, and
deletes its folder. Several runs can go at once. The window does appear on screen while it runs.

Python standard library only, so an isolated agent that may not start PowerShell can run it.
"""

import argparse
import base64
import json
import os
import shutil
import socket
import struct
import subprocess
import sys
import tempfile
import time
import urllib.request
import uuid
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


# ---- a minimal websocket client for the debugging port (text frames only) ----

class Page:
    def __init__(self, ws_url):
        host, _, path = ws_url[len("ws://"):].partition("/")
        h, _, p = host.partition(":")
        self.s = socket.create_connection((h, int(p)), timeout=600)
        key = base64.b64encode(os.urandom(16)).decode()
        self.s.sendall((f"GET /{path} HTTP/1.1\r\nHost: {host}\r\nUpgrade: websocket\r\n"
                        f"Connection: Upgrade\r\nSec-WebSocket-Key: {key}\r\n"
                        "Sec-WebSocket-Version: 13\r\n\r\n").encode())
        head = b""
        while b"\r\n\r\n" not in head:
            head += self.s.recv(1)
        if b" 101 " not in head.split(b"\r\n")[0]:
            raise RuntimeError("the debugging port refused the connection: " + head.decode(errors="replace"))
        self.id = 0

    def _exact(self, n):
        buf = bytearray(n)
        view, got = memoryview(buf), 0
        while got < n:
            k = self.s.recv_into(view[got:])
            if not k:
                raise RuntimeError("the debugging port closed")
            got += k
        return bytes(buf)

    def _send(self, text, opcode=1):
        data = text.encode() if isinstance(text, str) else text
        n = len(data)
        head = bytes([0x80 | opcode])
        head += bytes([0x80 | n]) if n < 126 else (bytes([0x80 | 126]) + struct.pack(">H", n) if n < 65536 else bytes([0x80 | 127]) + struct.pack(">Q", n))
        mask = os.urandom(4)
        masked = bytes(b ^ mask[i & 3] for i, b in enumerate(data))
        self.s.sendall(head + mask + masked)

    def _receive(self):
        parts = []
        while True:
            b0, b1 = self._exact(2)
            n = b1 & 0x7F
            if n == 126:
                n = struct.unpack(">H", self._exact(2))[0]
            elif n == 127:
                n = struct.unpack(">Q", self._exact(8))[0]
            data = self._exact(n)
            opcode = b0 & 0x0F
            if opcode == 9:
                self._send(data, 10)
                continue
            if opcode == 8:
                raise RuntimeError("the debugging port closed")
            parts.append(data)
            if b0 & 0x80:
                return b"".join(parts).decode()

    def eval(self, js):
        """The value a JavaScript expression comes to (a promise is awaited); raises with the
        page's own words if it threw."""
        self.id += 1
        me = self.id
        self._send(json.dumps({"id": me, "method": "Runtime.evaluate",
                               "params": {"expression": js, "returnByValue": True, "awaitPromise": True}}))
        while True:
            msg = json.loads(self._receive())
            if msg.get("id") != me:
                continue
            if "error" in msg:
                raise RuntimeError("debugging port: " + json.dumps(msg["error"]))
            res = msg["result"]
            if "exceptionDetails" in res:
                ex = res["exceptionDetails"]
                raise RuntimeError("the page: " + ex.get("exception", {}).get("description", ex.get("text", "")))
            return res["result"].get("value")

    def close(self):
        try:
            self.s.close()
        except OSError:
            pass


def open_page(port, limit):
    t0 = time.time()
    while time.time() - t0 < limit:
        try:
            pages = json.load(urllib.request.urlopen(f"http://127.0.0.1:{port}/json", timeout=2))
            for p in pages:
                if p.get("type") == "page" and p.get("webSocketDebuggerUrl"):
                    return Page(p["webSocketDebuggerUrl"])
        except (OSError, ValueError):
            pass
        time.sleep(0.1)
    raise RuntimeError(f"the web view's debugging port {port} never answered")


# ---- pictures ----

def png(path, rgba, w, h):
    """Straight RGBA, 8 bits a sample, as a PNG."""
    stride = w * 4
    raw = b"".join(b"\x00" + rgba[y * stride:(y + 1) * stride] for y in range(h))

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    Path(path).write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
                           + chunk(b"IDAT", zlib.compress(raw, 6)) + chunk(b"IEND", b""))


def shrink(rgba, w, h, k):
    """Every k-th pixel of every k-th row: a small preview that keeps the disk light."""
    stride, m = w * 4, memoryview(rgba)
    rows = [b"".join(m[y * stride + x * 4:y * stride + x * 4 + 4] for x in range(0, w, k)) for y in range(0, h, k)]
    return b"".join(rows), len(range(0, w, k)), len(rows)


def difference(a, b):
    """Per channel R, G, B, A: the largest and the mean difference, in 8-bit steps."""
    if not a or not b or len(a) != len(b):
        return None
    out = {}
    n = len(a) // 4
    for c, name in enumerate("rgba"):
        d = [abs(x - y) for x, y in zip(a[c::4], b[c::4])]
        out[name] = {"max": max(d), "mean": round(sum(d) / n, 4)}
    return out


# ---- the page's side: each returns its facts as JSON, a newline, then the pixels in base64 ----

B64 = ("const b64 = (u) => { let s = ''; for (let i = 0; i < u.length; i += 0x8000) "
       "s += String.fromCharCode.apply(null, u.subarray(i, i + 0x8000)); return btoa(s); };")
READY = ("typeof doc !== 'undefined' && !!doc && !!composition() && !inFlight && !pending"
         " && document.getElementById('quality').textContent !== ''")


def cpu_js(n):
    return """(async () => { %s
  const said = (t) => { try { return decodeURIComponent(t || '') } catch (e) { return t } };
  const r = await fetch(FRAMES + '/frame/%d?q=full');
  const h = r.headers;
  const meta = { status: r.status, frame: +h.get('x-frame'), width: +h.get('x-width'), height: +h.get('x-height'),
    quality: h.get('x-quality'), drawn_on: h.get('x-drawn-on'), on_screen: h.get('x-on-screen') === '1',
    from_memory: h.get('x-cached') === '1', window_ms: +h.get('x-ms'), frame_notes: said(h.get('x-frame-notes')),
    report: document.getElementById('report').textContent, status_line: document.getElementById('status').textContent };
  const body = new Uint8Array(await r.arrayBuffer());
  if (!r.ok) { meta.error = new TextDecoder().decode(body); return JSON.stringify(meta) + '\\n'; }
  if (meta.on_screen || body.length !== meta.width * meta.height * 4) {
    meta.error = 'the answer carried ' + body.length + ' bytes, not the ' + (meta.width * meta.height * 4) + ' of a whole frame';
    return JSON.stringify(meta) + '\\n';
  }
  return JSON.stringify(meta) + '\\n' + b64(body);
})()""" % (B64, n)


def card_js(n, limit_ms):
    return """(async () => { %s
  const t0 = performance.now();
  const until = async (ok, what) => { while (!ok()) { if (performance.now() - t0 > %d) throw new Error('timed out waiting for ' + what); await new Promise((r) => setTimeout(r, 20)); } };
  const busy = () => inFlight || pending;
  playing = false; stirred = -Infinity; quality = 'full';
  await until(() => !busy(), 'the viewer to be free');
  wanted = %d;
  await show(FRAMES + '/frame/%d?q=full');
  // The first frame on the card is sent to the page; the page then lays out and the card paints
  // the window, which asks for the frame again. Let that happen, as it does for a person.
  // Up to two seconds: a window Windows is not drawing (minimized) never gets there.
  const painting = performance.now();
  while (!document.documentElement.classList.contains('onscreen') && performance.now() - painting < 2000) await new Promise((r) => setTimeout(r, 20));
  await until(() => !busy(), 'frame %d');
  const painted = document.documentElement.classList.contains('onscreen');
  holdingPixels++;
  try {
    await pixelsHere();
    await until(() => !busy() && !drawnPart && !document.documentElement.classList.contains('onscreen'), 'the pixels in the canvas');
    const title = document.getElementById('qualitymenu').title;
    const on = /drawn on (.+?)\\. Click/.exec(title);
    const meta = { frame, width: view.width, height: view.height, quality: document.getElementById('quality').textContent,
      drawn_on: on ? on[1] : null, painted_in_window: painted, quality_tip: title,
      cpu_note_shown: !document.getElementById('cpunote').hidden,
      report: document.getElementById('report').textContent, status_line: document.getElementById('status').textContent };
    if (frame !== %d) meta.error = 'the viewer shows frame ' + frame + ', not %d';
    const px = new Uint8Array(paint.getImageData(0, 0, view.width, view.height).data.buffer);
    return JSON.stringify(meta) + '\\n' + b64(px);
  } finally { holdingPixels--; }
})()""" % (B64, limit_ms, n, n, n, n, n)


# ---- run ----

def main():
    a = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    a.add_argument("--project", required=True)
    a.add_argument("--frames", default="0")
    a.add_argument("--out", default="")
    a.add_argument("--on", choices=("cpu", "card", "both"), default="both")
    a.add_argument("--port", type=int, default=0)
    a.add_argument("--exe", default=str(ROOT / "target" / "release" / "anime_compositor_app.exe"))
    a.add_argument("--media", default=str(ROOT / "Fixtures" / "reference_shot"))
    a.add_argument("--timeout", type=float, default=180)
    a.add_argument("--full", action="store_true", help="also keep the full-size PNG (about 3 MB a shot)")
    a.add_argument("--preview", type=int, default=480, help="width of the small preview PNG")
    args = a.parse_args()

    project, exe = Path(args.project).resolve(), Path(args.exe).resolve()
    if not exe.is_file():
        sys.exit(f"no app at {exe}: build it first (cargo build -p anime_compositor_app --release)")
    out = Path(args.out or ROOT / "verification" / "app_snapshots" / project.stem).resolve()
    out.mkdir(parents=True, exist_ok=True)
    frames = [int(f) for f in args.frames.replace(" ", ",").split(",") if f]
    port = args.port
    if port <= 0:
        with socket.socket() as s:
            s.bind(("127.0.0.1", 0))
            port = s.getsockname()[1]

    app_data = Path(os.environ["APPDATA"]) / "dev.anitastic.anime-compositor"
    kept = {p: (p.read_bytes() if p.is_file() else None) for p in (app_data / "window.txt", app_data / "recent.txt")}
    work = Path(tempfile.gettempdir()) / ("app_snapshot_" + uuid.uuid4().hex[:8])
    work.mkdir()
    summary = {"project": str(project), "exe": str(exe),
               "exe_written": time.strftime("%Y-%m-%dT%H:%M:%S", time.localtime(exe.stat().st_mtime)),
               "port": port, "started": time.strftime("%Y-%m-%dT%H:%M:%S"), "shots": [], "differences": []}
    t_all = time.perf_counter()
    proc = page = None
    pixels = {}
    try:
        # The exe is copied so the run never holds target\release's copy open against a rebuild.
        exe_copy = work / exe.name
        shutil.copy2(exe, exe_copy)
        media = Path(args.media)
        if media.is_dir():
            for d in media.iterdir():
                if d.is_dir():
                    shutil.copytree(d, work / d.name)
        proj_copy = work / project.name
        shutil.copy2(project, proj_copy)
        env = dict(os.environ, WEBVIEW2_USER_DATA_FOLDER=str(work / "webview"),
                   WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=f"--remote-debugging-port={port}")
        proc = subprocess.Popen([str(exe_copy), str(proj_copy)], cwd=work, env=env)
        page = open_page(port, 60)
        t0 = time.time()
        while page.eval(READY) is not True:
            if time.time() - t0 > args.timeout:
                raise RuntimeError("the page never finished opening the project")
            time.sleep(0.1)
        summary["ready_ms"] = round((time.perf_counter() - t_all) * 1000)

        for side in (("cpu", "card") if args.on == "both" else (args.on,)):
            to = "cpu" if side == "cpu" else "gpu"
            summary["switch_" + side] = page.eval(f"fetch(PROJECT + '/gpu-switch?to={to}').then((r) => r.text())")
            for n in frames:
                name = f"frame_{n:04d}_{side}"
                t = time.perf_counter()
                body = None
                try:
                    answer = page.eval(cpu_js(n) if side == "cpu" else card_js(n, int(args.timeout * 1000)))
                    head, _, tail = answer.partition("\n")
                    meta = json.loads(head)
                    if tail:
                        body = base64.b64decode(tail)
                        w, h = meta["width"], meta["height"]
                        png(out / f"{name}.png", *shrink(body, w, h, max(1, -(-w // args.preview))))
                        if args.full:
                            png(out / f"{name}_full.png", body, w, h)
                        pixels[(side, n)] = body
                except Exception as e:  # noqa: BLE001 - every failure is written down, per shot
                    meta = {"frame": n, "error": str(e)}
                ms = round((time.perf_counter() - t) * 1000)
                meta.update(asked_frame=n, side=side, picture=f"{name}.png" if body else None,
                            picture_is="preview, every k-th pixel; the cpu-vs-card difference uses the full frame",
                            pixel_format="8-bit straight sRGB RGBA", tool_ms=ms)
                (out / f"{name}.json").write_text(json.dumps(meta, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
                summary["shots"].append({"name": name, "ms": ms, "drawn_on": meta.get("drawn_on"), "error": meta.get("error")})
                print(f"{name}: {ms} ms, {meta.get('width')}x{meta.get('height')}, drawn on {meta.get('drawn_on')}"
                      + (f", ERROR {meta['error']}" if meta.get("error") else ""), flush=True)
        if args.on == "both":
            for n in frames:
                d = difference(pixels.get(("cpu", n)), pixels.get(("card", n)))
                summary["differences"].append({"frame": n, **(d or {"error": "no pair of pictures of one size to compare"})})
                if d:
                    print(f"frame {n} cpu vs card, largest/mean difference in 8-bit steps: "
                          + " ".join(f"{c.upper()} {v['max']}/{v['mean']}" for c, v in d.items()), flush=True)
    except Exception as e:  # noqa: BLE001
        summary["error"] = str(e)
        print("ERROR:", e, flush=True)
    finally:
        if page:
            page.close()
        if proc:
            # Only this run's own process and the web view it started; never WM_CLOSE (window.txt).
            subprocess.run(["taskkill", "/PID", str(proc.pid), "/T", "/F"], capture_output=True)
            proc.wait(30)
        for p, was in kept.items():
            now = p.read_bytes() if p.is_file() else None
            if now != was:
                if was is None:
                    p.unlink()
                else:
                    p.write_bytes(was)
                summary.setdefault("restored", []).append(p.name)
        for _ in range(50):
            shutil.rmtree(work, ignore_errors=True)
            if not work.exists():
                break
            time.sleep(0.2)
        summary["temp_folder_removed"] = not work.exists()
        summary["total_ms"] = round((time.perf_counter() - t_all) * 1000)
        (out / "summary.json").write_text(json.dumps(summary, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        print(f"total {summary['total_ms']} ms; written to {out}", flush=True)
    return 1 if summary.get("error") else 0


if __name__ == "__main__":
    sys.exit(main())
