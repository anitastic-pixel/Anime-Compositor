"""P-21 (P-27's audit): the page's side of one edit, in a background copy of the app with its own WebView2 profile.

    python -I tools/p21_page_cost.py <app.exe> <out.json>

Opens the reference shot (4 layers) and a 200-layer copy (layer 2 copied, 25 position keys each,
5,000 keys: the same project tests/p20_edit_cost.rs builds), and in each times, through the
web view's debugging port:
  state_fetch   fetch(PROJECT + '/state') until the text has arrived (window work + transfer)
  state_parse   JSON.parse of that text
  draw_panels   drawPanels(), the page's redraw of every panel after an edit
  edit_total    await command('/layer.rename?...'): the page's whole edit path
                (command, /state, parse, drawPanels, drawSketch), the frame request after it not waited
  zoom_redraw   the timeline zoom slider moved, its oninput handler run once
Seven runs each, medians. Never touches the owner's profile, window.txt or recent.txt.
"""
import json, os, shutil, socket, subprocess, sys, tempfile, time, uuid
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "tools"))
from app_snapshot import open_page, READY  # noqa: E402


def big_project(layers):
    p = json.loads((REPO / "verification/B-08a_project.json").read_text(encoding="utf-8"))
    comp = p["compositions"][0]
    model = comp["layers"][1]
    out, order = [], []
    for n in range(layers):
        l = json.loads(json.dumps(model))
        l["id"] = f"layer-p20-{n}"
        l["name"] = f"copy {n}"
        l["transform"]["position"]["keyframes"] = [
            {"frame": k * 9, "value": [k * 3.0, float(n)], "interp": "linear"} for k in range(25)]
        out.append(l)
        order.append(l["id"])
    comp["layers"], comp["layer_order"] = out, order
    return json.dumps(p, indent=2)


JS = r"""(async () => {
  const med = (a) => { const s = [...a].sort((x, y) => x - y); return +s[s.length >> 1].toFixed(2); };
  const r = { layers: layers().length };
  const st = [], pa = [], dp = [], ed = [], zm = [];
  let bytes = 0;
  for (let i = 0; i < 7; i++) {
    let t = performance.now();
    const text = await (await fetch(PROJECT + '/state')).text();
    st.push(performance.now() - t); bytes = text.length;
    t = performance.now(); JSON.parse(text); pa.push(performance.now() - t);
    t = performance.now(); drawPanels(); dp.push(performance.now() - t);
  }
  const first = layers()[0];
  for (let i = 0; i < 7; i++) {
    const t = performance.now();
    await command('/layer.rename?layer=' + encodeURIComponent(first.id) + '&name=' + encodeURIComponent('p27 ' + i));
    ed.push(performance.now() - t);
    while (inFlight || pending) await new Promise((res) => setTimeout(res, 20));
  }
  const z = document.getElementById('timezoom');
  for (let i = 0; i < 7; i++) {
    z.value = String(i % 2 ? z.min || 1 : z.max || 100);
    const t = performance.now(); z.oninput(); zm.push(performance.now() - t);
  }
  r.state_bytes = bytes; r.state_fetch = med(st); r.state_parse = med(pa); r.draw_panels = med(dp);
  r.edit_total = med(ed); r.zoom_redraw = med(zm); r.undo_depth = (doc.undo || []).length;
  r.dom_nodes = document.getElementsByTagName('*').length;
  return JSON.stringify(r);
})()"""


def run(exe, project_text, name):
    app_data = Path(os.environ["APPDATA"]) / "dev.anitastic.anime-compositor"
    kept = {p: (p.read_bytes() if p.is_file() else None) for p in (app_data / "window.txt", app_data / "recent.txt")}
    work = Path(tempfile.gettempdir()) / ("p27_" + uuid.uuid4().hex[:8])
    work.mkdir()
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0)); port = s.getsockname()[1]
    proc = page = None
    try:
        exe_copy = work / Path(exe).name
        shutil.copy2(exe, exe_copy)
        for d in (REPO / "Fixtures/reference_shot").iterdir():
            if d.is_dir():
                shutil.copytree(d, work / d.name)
        proj = work / (name + ".json")
        proj.write_text(project_text, encoding="utf-8")
        env = dict(os.environ, WEBVIEW2_USER_DATA_FOLDER=str(work / "webview"),
                   WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=f"--remote-debugging-port={port}",
                   ANIME_COMPOSITOR_BACKGROUND="1")
        t0 = time.perf_counter()
        proc = subprocess.Popen([str(exe_copy), str(proj)], cwd=work, env=env)
        page = open_page(port, 60)
        while page.eval(READY) is not True:
            if time.perf_counter() - t0 > 180:
                raise RuntimeError("never opened")
            time.sleep(0.1)
        ready_ms = round((time.perf_counter() - t0) * 1000)
        r = json.loads(page.eval(JS))
        r["ready_ms_from_launch"] = ready_ms
        return r
    finally:
        if page:
            page.close()
        if proc:
            subprocess.run(["taskkill", "/PID", str(proc.pid), "/T", "/F"], capture_output=True)
            proc.wait(30)
        for p, was in kept.items():
            now = p.read_bytes() if p.is_file() else None
            if now != was:
                p.unlink() if was is None else p.write_bytes(was)
        for _ in range(50):
            shutil.rmtree(work, ignore_errors=True)
            if not work.exists():
                break
            time.sleep(0.2)


if __name__ == "__main__":
    exe, out = sys.argv[1], sys.argv[2]
    ref = (REPO / "verification/B-08a_project.json").read_text(encoding="utf-8")
    res = {"reference_4": run(exe, ref, "ref4"), "copies_200": run(exe, big_project(200), "big200")}
    Path(out).write_text(json.dumps(res, indent=2), encoding="utf-8")
    print(json.dumps(res, indent=2))
