"""P-28: one real 5-second anime shot, made and checked in the app end to end.

    python tools/p28_media.py media      # target/p28/: bg_plate PNGs, fg_subject.png, sfx_impact.wav, cels
    python tools/p28_media.py project    # verification/P-28/p28_shot.json (and its check variants)

Deterministic (fixed seed, numpy and the standard library only). The media is not committed;
run `media` again to make it byte for byte the same.
"""

import json
import shutil
import struct
import sys
import wave
import zlib
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parent.parent
MEDIA = ROOT / "target" / "p28"
OUT = ROOT / "verification" / "P-28"
W, H, FPS, FRAMES, RATE = 1920, 1080, 24, 120, 48000
HIT = 48  # the frame of the impact; the click's first sample is HIT / FPS * RATE = 96000


def png(path, arr):
    """An RGB or RGBA uint8 array as a PNG (straight alpha)."""
    h, w, c = arr.shape
    raw = b"".join(b"\x00" + arr[y].tobytes() for y in range(h))

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    Path(path).write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6 if c == 4 else 2, 0, 0, 0))
                           + chunk(b"IDAT", zlib.compress(raw, 1)) + chunk(b"IEND", b""))


def bg_frame(n):
    """A slowly moving gradient with a grid over it: a stand-in for a background plate."""
    y, x = np.mgrid[0:H, 0:W].astype(np.float32)
    t = n / FPS
    u = (x + 40 * t) / W
    r = 40 + 60 * u + 20 * np.sin(y / H * np.pi)
    g = 70 + 50 * (y / H)
    b = 120 + 70 * (1 - u)
    grid = ((x + 6 * n) % 120 < 3) | ((y + 2 * n) % 120 < 3)
    rgb = np.stack([r, g, b], -1)
    rgb[grid] = (200, 210, 230)
    return np.clip(rgb + 0.5, 0, 255).astype(np.uint8)


def fg_subject():
    """1080x1080: a high-contrast disc (yellow with black stripes, a white core) whose alpha fades
    out over its last 60 px, so a premultiplication error shows as a dark ring."""
    s = 1080
    y, x = np.mgrid[0:s, 0:s].astype(np.float32) + 0.5
    d = np.hypot(x - s / 2, y - s / 2)
    e = np.clip((480 - d) / 60, 0, 1)
    alpha = e * e * (3 - 2 * e)
    rgb = np.empty((s, s, 3), np.float32)
    rgb[:] = (255, 214, 40)
    stripes = (((x + y) // 60) % 2 == 0) & (d < 360)
    rgb[stripes] = (10, 10, 10)
    rgb[d < 120] = (255, 255, 255)
    out = np.concatenate([rgb, alpha[..., None] * 255], -1)
    return np.clip(out + 0.5, 0, 255).astype(np.uint8)


def sfx():
    """5 s of silence at 48 kHz, 16-bit stereo, but for one sharp click starting on sample 96000."""
    n = FRAMES * RATE // FPS
    a = np.zeros(n, np.float64)
    start = HIT * RATE // FPS
    k = np.arange(2400)
    rng = np.random.default_rng(28)
    a[start:start + 2400] = 0.9 * np.exp(-k / 300) * np.where(k < 4, 1.0, rng.uniform(-1, 1, 2400))
    pcm = np.round(a * 32767).astype("<i2")
    return np.repeat(pcm, 2).tobytes(), start


def media():
    MEDIA.mkdir(parents=True, exist_ok=True)
    (MEDIA / "bg_plate").mkdir(exist_ok=True)
    for n in range(FRAMES):
        png(MEDIA / "bg_plate" / f"bg_plate_{n:04d}.png", bg_frame(n))
    (MEDIA / "fg").mkdir(exist_ok=True)
    png(MEDIA / "fg" / "fg_subject.png", fg_subject())
    (MEDIA / "sfx").mkdir(exist_ok=True)
    pcm, start = sfx()
    with wave.open(str(MEDIA / "sfx" / "sfx_impact.wav"), "wb") as w:
        w.setnchannels(2)
        w.setsampwidth(2)
        w.setframerate(RATE)
        w.writeframes(pcm)
    # The reference shot's soft-edged cels (layer 2, 24 drawings), copied, never changed.
    src = ROOT / "Fixtures" / "reference_shot" / "layer2"
    shutil.copytree(src, MEDIA / "layer2", dirs_exist_ok=True)
    print(f"media written to {MEDIA}: 120 plate frames, the subject, the click at sample {start}, 24 cels")


def prop(base, keys=()):
    """A property record. Each key is (frame, value, interp) or (frame, value, "ease", [x1, y1, x2, y2])."""
    out = []
    for k in keys:
        key = {"frame": k[0], "value": k[1], "interp": k[2]}
        if len(k) > 3:
            key["ease"] = k[3]
        out.append(key)
    return {"base": base, "keyframes": out}


def xform(anchor, position, scale=None, opacity=None):
    return {"anchor": prop(anchor), "position": position if isinstance(position, dict) else prop(position),
            "scale": scale or prop([100, 100]), "rotation": prop(0), "opacity": opacity or prop(1)}


def layer(id, kind, a, b, transform, **extra):
    out = {"id": id, "kind": kind, "name": id, "enabled": True, "locked": False, "in_frame": a, "out_frame": b}
    if transform:
        out["transform"] = transform
        out.update({"masks": [], "matte": None, "blend_mode": "normal", "effects": []})
    out.update(extra)
    return out


def fx(n, type_id, **parameters):
    return {"instance_id": f"fx-{n}", "type_id": type_id, "enabled": True, "parameters": parameters}


def rect(x0, y0, x1, y1):
    return {"points": [{"point": p, "in": [0, 0], "out": [0, 0]} for p in ([x0, y0], [x1, y0], [x1, y1], [x0, y1])]}


EASE_OUT = [0.05, 0.8, 0.2, 1]    # fast start, long settle
OVERSHOOT = [0.3, 0, 0.4, 1.45]   # goes past 100% and comes back
# The hit's shake, on the null every picture layer rides. 15 px, gone 12 frames after frame 48. k is
# held at 1 on frames 48 and 49 so the two hold frames are the same picture, shake included.
SHAKE = ("// P-28: 15 px impact shake from frame 48, dying away over 12 frames\n"
         "var k = (Math.max(time, 49 / 24) - 2) * 24;\n"
         "var g = Math.min(1, Math.max(0, (time - 2) * 24 + 1));\n"
         "var a = 15 * g * Math.max(0, 1 - k / 12);\n"
         "value + [a * Math.cos(k * 2.3), a * Math.sin(k * 2.3 + 1)]")


def particles():
    return {"birth_rate": 1.5, "longevity": 3, "producer_x": 0, "producer_y": 0, "producer_z": 0,
            "radius_x": 0.4, "radius_y": 0.25, "radius_z": 0.1, "velocity": 0.15, "inherit_velocity": 0,
            "gravity": 0, "resistance": 0, "axis_x": 0, "axis_y": -1, "axis_z": 0, "extra": 0.5,
            "extra_angle": 360, "gravity_x": 0, "gravity_y": 1, "gravity_z": 0, "floor_position": 0.5,
            "bounciness": 50, "random_bounciness": 0, "bounce_spread": 0, "wind_x": 0.1, "wind_y": 0,
            "wind_z": 0, "turbulence": 0, "turbulence_scale": 0.25, "turbulence_evolution": 0,
            "birth_size": 0.06, "death_size": 0.02, "size_variation": 0, "max_opacity": 75,
            "volume_shade": 0, "camera_distance": 2, "camera_rotation_x": 0, "camera_rotation_y": 0,
            "camera_rotation_z": 0, "camera_fov": 50, "depth_cue_distance": 4, "light_direction": 45,
            "light_height": 45, "random_seed": 28, "animation": "explosive", "floor_action": "none",
            "particle_type": "faded_sphere", "color_map": "birth_to_death", "transfer_mode": "composite",
            "depth_cue": "none", "composite_on_original": "off", "birth_color": "#fff2b3",
            "death_color": "#ff8833", "custom_color_1": "#ffd933", "custom_color_2": "#f2a926",
            "custom_color_3": "#e67a1a", "custom_color_4": "#d94a0d", "custom_color_5": "#cc1a00",
            "depth_cue_color": "#000000"}


def shot():
    cels = sorted(p.name for p in (MEDIA / "layer2").glob("*.png"))
    assets = [
        {"id": "asset-bg", "kind": "image_sequence", "name": "bg_plate", "pattern": "bg_plate_####.png",
         "frames": {str(n + 1): f"bg_plate/bg_plate_{n:04d}.png" for n in range(FRAMES)},
         "interpretation": {"color_space": "srgb", "alpha": "straight"}},
        {"id": "asset-subject", "kind": "still", "name": "fg_subject", "path": "fg/fg_subject.png",
         "interpretation": {"color_space": "srgb", "alpha": "straight"}},
        {"id": "asset-cels", "kind": "image_sequence", "name": "layer2", "pattern": "layer2_###.png",
         "frames": {str(i + 1): f"layer2/{c}" for i, c in enumerate(cels)},
         "interpretation": {"color_space": "srgb", "alpha": "straight"}},
        {"id": "asset-sfx", "kind": "audio", "name": "sfx_impact", "path": "sfx/sfx_impact.wav"},
    ]
    # The plate is one drawing a frame, but for the hit: frames 48 and 49 both show drawing 49.
    plate = [{"start_frame": n, "end_frame_exclusive": n + 1, "drawing_number": n + 1} for n in range(HIT)]
    plate += [{"start_frame": HIT, "end_frame_exclusive": HIT + 2, "drawing_number": HIT + 1}]
    plate += [{"start_frame": n, "end_frame_exclusive": n + 1, "drawing_number": n + 1} for n in range(HIT + 2, FRAMES)]
    subject_pos = prop([2500, 560], [(0, [2500, 560], "ease", EASE_OUT), (40, [1240, 540], "hold"),
                                     (HIT, [1200, 540], "hold"), (HIT + 2, [1300, 540], "ease", EASE_OUT),
                                     (70, [1240, 540], "linear")])
    layers = [
        layer("rig", "null", 0, FRAMES, dict(xform([960, 540], {**prop([960, 540]),
                                                               "expression": {"text": SHAKE, "enabled": True}}))),
        layer("bg", "raster", 0, FRAMES, xform([960, 540], [960, 540],
                                               scale=prop([100, 100], [(0, [100, 100], "linear"), (47, [108, 108], "linear")])),
              asset_id="asset-bg", source_offset_frames=0, exposure_spans=plate, parent="rig",
              effects=[fx("bg-blur", "core.gaussian_blur", sigma_px=2.0)]),
        layer("cels", "raster", 0, HIT, xform([960, 540], [960, 540]), asset_id="asset-cels",
              source_offset_frames=0, parent="rig",
              exposure_spans=[{"start_frame": 2 * i, "end_frame_exclusive": 2 * i + 2, "drawing_number": i + 1}
                              for i in range(HIT // 2)]),
        layer("subject", "raster", 0, FRAMES, xform([540, 540], subject_pos), asset_id="asset-subject",
              source_offset_frames=0, exposure_spans=[], parent="rig"),
        layer("glow", "raster", 55, FRAMES, xform([540, 540], subject_pos, opacity=prop(0.8)),
              asset_id="asset-subject", source_offset_frames=0, exposure_spans=[], parent="rig",
              blend_mode="screen",
              effects=[fx("glow-tint", "core.tint", color=[1.0, 0.45, 0.1], amount=1.0),
                       fx("glow-blur", "core.gaussian_blur", sigma_px=40.0)]),
        layer("title", "text", 0, FRAMES,
              xform([560, 860], [560, 860], scale=prop([100, 100], [(12, [70, 70], "ease", OVERSHOOT),
                                                                    (32, [100, 100], "linear")])),
              parent="rig",
              masks=[{"name": "Wipe", "enabled": True, "inverted": False, "mode": "add", "opacity": 1.0,
                      "feather_px": 24.0, "expansion_px": 0.0,
                      "path": prop(rect(240, 700, 241, 960), [(6, rect(240, 700, 241, 960), "ease", [0.4, 0, 0.2, 1]),
                                                             (30, rect(240, 700, 880, 960), "linear")])}],
              source_text={"text": "IMPACT", "font": "MPLUSRounded1c-Regular.ttf", "size": 180,
                           "color": [1.0, 0.85, 0.3], "at": [560, 920], "align": "center"}),
        layer("flash", "adjustment", HIT, 55,
              xform([960, 540], [960, 540], opacity=prop(0.8, [(HIT, 0.8, "hold"), (HIT + 2, 0.8, "linear"), (54, 0.0, "linear")])),
              blend_mode="add",
              effects=[fx("flash-fill", "core.fill", mask=0, all_masks="off", color="#ffffff", invert="off",
                          horizontal_feather=0, vertical_feather=0, opacity=100)]),
        layer("split", "adjustment", HIT, 55, xform([960, 540], [960, 540]),
              effects=[fx("split-ca", "core.chromatic_aberration", amount=12, center=[50, 50])]),
        layer("sparks", "solid", 55, FRAMES,
              xform([960, 540], prop([960, 540], [(55, [900, 560], "linear"), (119, [1260, 500], "linear")]),
                    opacity=prop(1, [(95, 1, "ease", [0.333, 0, 0.667, 1]), (115, 0, "linear")])),
              solid={"color": [0, 0, 0], "width": W, "height": H}, blend_mode="add", motion_blur=True,
              effects=[fx("sparks-ps", "core.particle_space", **particles())]),
        layer("sound", "audio", 0, FRAMES, None, asset_id="asset-sfx", source_offset_frames=0, gain_db=0),
    ]
    comp = {"id": "comp-shot", "name": "P-28 shot", "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": FPS, "denominator": 1}, "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "motion_blur": {"enabled": True, "shutter_angle": 180, "shutter_phase": -90, "samples": 16},
            "layer_order": [l["id"] for l in layers], "layers": layers}
    return {"schema_version": 0, "project_id": "proj-p28-shot",
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": assets, "compositions": [comp]}


def write(name, doc):
    with open(OUT / name, "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")


def only(doc, keep):
    """The shot with every picture layer but `keep` switched off (the null and the sound stay)."""
    doc = json.loads(json.dumps(doc))
    for l in doc["compositions"][0]["layers"]:
        if l["kind"] not in ("null", "audio"):
            l["enabled"] = l["id"] in keep
    return doc


def project():
    OUT.mkdir(parents=True, exist_ok=True)
    doc = shot()
    write("p28_shot.json", doc)
    # Check variants for the compositing check: the shot without its Screen glow, and the glow alone.
    ids = [l["id"] for l in doc["compositions"][0]["layers"]]
    write("p28_shot_noglow.json", only(doc, set(ids) - {"glow"}))
    write("p28_shot_glowonly.json", only(doc, {"glow"}))
    # And for the soft edge, at frame 47: the subject alone, and everything but the subject.
    write("p28_shot_subjectonly.json", only(doc, {"subject"}))
    write("p28_shot_nosubject.json", only(doc, set(ids) - {"subject"}))
    print(f"wrote {OUT / 'p28_shot.json'} and its four check variants")


# ---- measuring: the release build, driven over its debugging port as tools/app_snapshot.py does ----

RUNS = MEDIA / "runs"
TEN = [0, 24, 47, 48, 49, 50, 54, 55, 90, 119]
SCRAMBLED = [119, 0, 50, 48, 90, 24, 55, 47, 54, 49]
FIVE = [0, 48, 50, 90, 119]


class App:
    """One copy of the app on one project, its own web view profile, off the person's screen."""

    def __init__(self, exe, project):
        import os, socket, subprocess, tempfile, time, uuid
        sys.path.insert(0, str(ROOT / "tools"))
        import app_snapshot
        self.snap = app_snapshot
        with socket.socket() as s:
            s.bind(("127.0.0.1", 0))
            port = s.getsockname()[1]
        data = Path(os.environ["APPDATA"]) / "dev.anitastic.anime-compositor"
        self.kept = {p: (p.read_bytes() if p.is_file() else None) for p in (data / "window.txt", data / "recent.txt")}
        self.work = Path(tempfile.gettempdir()) / ("p28_" + uuid.uuid4().hex[:8])
        self.work.mkdir()
        shutil.copy2(exe, self.work / Path(exe).name)
        for d in MEDIA.iterdir():
            if d.is_dir() and d.name not in ("runs", "export_png", "export_mp4"):
                shutil.copytree(d, self.work / d.name)
        shutil.copy2(project, self.work / Path(project).name)
        env = dict(os.environ, WEBVIEW2_USER_DATA_FOLDER=str(self.work / "webview"),
                   WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=f"--remote-debugging-port={port}",
                   ANIME_COMPOSITOR_BACKGROUND="1")
        self.proc = subprocess.Popen([str(self.work / Path(exe).name), str(self.work / Path(project).name)],
                                     cwd=self.work, env=env)
        self.page = app_snapshot.open_page(port, 60)
        t0 = time.time()
        while self.page.eval(app_snapshot.READY) is not True:
            if time.time() - t0 > 300:
                raise RuntimeError("the page never finished opening the project")
            time.sleep(0.1)

    def eval(self, js):
        return self.page.eval(js)

    def get(self, url):
        """A route's answer as text: url is page JavaScript, e.g. "PROJECT + '/session-log'"."""
        return self.eval(f"fetch({url}).then((r) => r.text())")

    def switch(self, to):
        return self.get(f"PROJECT + '/gpu-switch?to={to}'")

    def frame(self, side, n, q="full"):
        """(facts, H x W x 4 uint8 straight sRGB) of frame n, from the processor or the card."""
        import base64
        if side == "cpu":
            js = self.snap.cpu_js(n).replace("?q=full", f"?q={q}")
        else:
            js = self.snap.card_js(n, 300000).replace("quality = 'full'", f"quality = '{q}'").replace("?q=full", f"?q={q}")
        head, _, tail = self.eval(js).partition("\n")
        meta = json.loads(head)
        if not tail:
            raise RuntimeError(f"frame {n} on {side}: {meta.get('error')}")
        meta["failed"] = self.snap.problems(meta)
        px = np.frombuffer(base64.b64decode(tail), np.uint8).reshape(meta["height"], meta["width"], 4)
        return meta, px

    def close(self):
        import subprocess, time
        self.page.close()
        subprocess.run(["taskkill", "/PID", str(self.proc.pid), "/T", "/F"], capture_output=True)
        self.proc.wait(30)
        for p, was in self.kept.items():
            if (p.read_bytes() if p.is_file() else None) != was:
                p.unlink() if was is None else p.write_bytes(was)
        for _ in range(50):
            shutil.rmtree(self.work, ignore_errors=True)
            if not self.work.exists():
                break
            time.sleep(0.2)


def session(exe, name, job):
    app = App(exe, OUT / name)
    try:
        return job(app)
    finally:
        app.close()


def stats(ms):
    ms = sorted(ms)
    return {"n": len(ms), "median": ms[len(ms) // 2], "min": ms[0], "max": ms[-1]}


# Fill the system's folder dialog by UI Automation (no mouse, no keys): the folder, then Select Folder.
PICK = r"""
param($title, $folder, $procid)
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$A = [System.Windows.Automation.AutomationElement]
$scope = [System.Windows.Automation.TreeScope]
$until = (Get-Date).AddSeconds(30)
do {
  $c = New-Object System.Windows.Automation.AndCondition(
    (New-Object System.Windows.Automation.PropertyCondition($A::NameProperty, $title)),
    (New-Object System.Windows.Automation.PropertyCondition($A::ProcessIdProperty, [int]$procid)))
  $d = $A::RootElement.FindFirst($scope::Children, $c)
  if (-not $d) { Start-Sleep -Milliseconds 200 }
} until ($d -or (Get-Date) -gt $until)
if (-not $d) { 'no dialog'; exit 2 }
$all = $d.FindAll($scope::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
$edits = @($all | Where-Object { $_.Current.ControlType -eq [System.Windows.Automation.ControlType]::Edit -and $_.GetSupportedPatterns() -contains [System.Windows.Automation.ValuePattern]::Pattern })
$box = @($edits | Where-Object { $_.Current.Name -match 'Folder' })[0]
if (-not $box) { $box = $edits[-1] }
if (-not $box) {
  $all | ForEach-Object { "$($_.Current.ControlType.ProgrammaticName) id=$($_.Current.AutomationId) name=$($_.Current.Name)" }
  exit 3
}
'box: ' + $box.Current.Name + ' id=' + $box.Current.AutomationId
$box.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern).SetValue($folder)
$ok = $d.FindFirst($scope::Descendants, (New-Object System.Windows.Automation.PropertyCondition($A::AutomationIdProperty, '1')))
$ok.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
'chosen'
"""


def export(app, query, into):
    """Export through the window's own Export, the folder dialog answered by UI Automation."""
    import subprocess, time
    shutil.rmtree(into, ignore_errors=True)
    into.mkdir(parents=True)
    script = RUNS / "pick_folder.ps1"
    script.write_text(PICK, encoding="utf-8")
    said = app.get(f"PROJECT + '/export?{query}'")
    pid = app.proc.pid
    picked = subprocess.run(["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(script),
                             "Export the frames into a folder", str(into), str(pid)],
                            capture_output=True, text=True, timeout=60)
    t0 = time.perf_counter()
    seen_running = False
    while time.perf_counter() - t0 < 1800:
        w = json.loads(app.get("PROJECT + '/export-watch'"))
        seen_running |= w["running"]
        if seen_running and not w["running"]:
            break
        if not seen_running and time.perf_counter() - t0 > 20:
            break
        time.sleep(0.5)
    took = time.perf_counter() - t0
    status = app.eval("document.getElementById('status').textContent + ' | ' + document.getElementById('report').textContent")
    return {"query": query, "said": said, "dialog": (picked.stdout + picked.stderr).strip(), "ran": seen_running,
            "seconds": round(took, 1), "status": status, "files": sorted(p.name for p in into.iterdir())}


def measure():
    """Every check that needs the app. Raw frames go to target/p28/runs, facts to measured.json."""
    import platform, subprocess, time
    exe = Path(sys.argv[2] if len(sys.argv) > 2 else ROOT / "target" / "release" / "anime_compositor_app.exe")
    RUNS.mkdir(parents=True, exist_ok=True)
    facts = {"exe": str(exe), "exe_written": time.strftime("%Y-%m-%dT%H:%M:%S", time.localtime(exe.stat().st_mtime)),
             "commit": subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()}
    ps = ("$c = Get-CimInstance Win32_Processor; $m = Get-CimInstance Win32_ComputerSystem; "
          "$o = Get-CimInstance Win32_OperatingSystem; "
          "\"$($c.Name.Trim()) | $($c.NumberOfCores) cores $($c.NumberOfLogicalProcessors) threads | "
          "$([math]::Round($m.TotalPhysicalMemory / 1GB, 1)) GB | $($o.Caption) $($o.Version)\"")
    facts["machine"] = subprocess.run(["powershell", "-NoProfile", "-Command", ps], capture_output=True, text=True).stdout.strip()
    facts["python"] = platform.python_version()

    def save(tag, n, px):
        np.save(RUNS / f"{tag}_{n:04d}.npy", px)

    # 1. Asked out of order, a fresh window: the ten frames on the processor, then on the card.
    def scrambled(app):
        out = {"cpu": {}, "card": {}}
        app.switch("cpu")
        for n in SCRAMBLED:
            meta, px = app.frame("cpu", n)
            save("scrambled_cpu", n, px)
            out["cpu"][n] = {k: meta.get(k) for k in ("window_ms", "tool_ms", "drawn_on", "from_memory", "failed", "frame_notes", "note_codes")}
        app.get("PROJECT + '/session-log?do=on'")
        app.get("PROJECT + '/session-log?do=clear'")
        app.switch("gpu")
        for n in SCRAMBLED:
            t = time.perf_counter()
            meta, px = app.frame("card", n)
            save("scrambled_card", n, px)
            out["card"][n] = {"tool_ms": round((time.perf_counter() - t) * 1000), "drawn_on": meta.get("drawn_on"),
                              "failed": meta.get("failed"), "quality": meta.get("quality")}
        out["card_log"] = json.loads(app.get("PROJECT + '/session-log'"))
        return out
    facts["scrambled"] = session(exe, "p28_shot.json", scrambled)
    print("scrambled done", flush=True)

    # 2. In order, a fresh window: all 120 on the processor, each one's time; the sound as the window mixes it.
    def sequence(app):
        app.switch("cpu")
        out = {"cpu": {}, "hashes": {}}
        import hashlib
        for n in range(FRAMES):
            meta, px = app.frame("cpu", n)
            out["hashes"][n] = hashlib.sha256(px.tobytes()).hexdigest()[:16]
            out["cpu"][n] = {k: meta.get(k) for k in ("window_ms", "drawn_on", "from_memory", "failed")}
            if n in TEN:
                save("sequence_cpu", n, px)
        mix = {}
        for a, k in ((0, FRAMES), (47, 1), (48, 1), (24, 24)):
            raw = app.eval(f"fetch(FRAMES + '/mix?from={a}&frames={k}').then((r) => r.arrayBuffer())"
                           ".then((b) => { const f = new Float32Array(b); const h = f.length / 2; let i = 0;"
                           " while (i < h && Math.abs(f[i]) < 0.1) i++; return JSON.stringify({samples: h, first_loud: i < h ? i : null}); })")
            mix[f"from {a}, {k} frames"] = json.loads(raw)
        out["mix"] = mix
        return out
    facts["sequence"] = session(exe, "p28_shot.json", sequence)
    print("sequence done", flush=True)

    # 3. Draft, a fresh window: ten frames on the processor (the window's own time), then on the card.
    def draft(app):
        out = {"cpu": {}, "card": {}}
        app.switch("cpu")
        for n in SCRAMBLED:
            meta, px = app.frame("cpu", n, "draft")
            save("draft_cpu", n, px)
            out["cpu"][n] = {k: meta.get(k) for k in ("window_ms", "drawn_on", "quality", "width", "height", "from_memory")}
        app.get("PROJECT + '/session-log?do=on'")
        app.get("PROJECT + '/session-log?do=clear'")
        app.switch("gpu")
        for n in SCRAMBLED:
            t = time.perf_counter()
            meta, px = app.frame("card", n, "draft")
            save("draft_card", n, px)
            out["card"][n] = {"tool_ms": round((time.perf_counter() - t) * 1000), "drawn_on": meta.get("drawn_on"),
                              "quality": meta.get("quality"), "width": meta.get("width")}
        out["card_log"] = json.loads(app.get("PROJECT + '/session-log'"))
        return out
    facts["draft"] = session(exe, "p28_shot.json", draft)
    print("draft done", flush=True)

    # 4. Playback at Full: the memory filled by the first play, then three loops from it.
    def play(to):
        def job(app):
            app.switch(to)
            app.get("PROJECT + '/session-log?do=on'")
            app.get("PROJECT + '/session-log?do=clear'")
            app.eval("quality = 'full'; wanted = 0; frame = 0; document.getElementById('play').click(); true")
            t0 = time.perf_counter()
            ready, filled = None, None
            while time.perf_counter() - t0 < 600:
                ready = json.loads(app.get("FRAMES + '/ready'"))
                if len(ready["frames"]) >= FRAMES:
                    filled = round(time.perf_counter() - t0, 2)
                    break
                time.sleep(0.25)
            fill_log = json.loads(app.get("PROJECT + '/session-log'"))
            app.get("PROJECT + '/session-log?do=clear'")
            t1 = time.perf_counter()
            time.sleep(3 * FRAMES / FPS)
            loops = json.loads(app.get("PROJECT + '/session-log'"))
            looped = round(time.perf_counter() - t1, 2)
            app.eval("if (playing) document.getElementById('play').click(); true")
            return {"filled_s": filled, "ready_frames": len(ready["frames"]) if ready else None,
                    "held_bytes": ready and ready["held"], "ceiling": ready and ready["setting"],
                    "fill_log": {k: fill_log[k] for k in ("frames", "median_ms", "slowest_5_percent_ms", "dropped")},
                    "loops_s": looped, "loops_log": {k: loops[k] for k in ("frames", "median_ms", "slowest_5_percent_ms", "dropped")},
                    "loops_expected_frames": 3 * FRAMES}
        return job
    facts["play_cpu"] = session(exe, "p28_shot.json", play("cpu"))
    facts["play_card"] = session(exe, "p28_shot.json", play("gpu"))
    print("play done", flush=True)

    # 5. The check variants at the frames the compositing check reads.
    def variant(frames):
        def job(app):
            app.switch("cpu")
            return {n: app.frame("cpu", n)[1] for n in frames}
        return job
    for name, n in (("p28_shot_noglow.json", 90), ("p28_shot_glowonly.json", 90),
                    ("p28_shot_subjectonly.json", 47), ("p28_shot_nosubject.json", 47)):
        save(name[:-5], n, session(exe, name, variant([n]))[n])
    print("variants done", flush=True)

    # 6. Export: a PNG sequence with its WAV, then an MP4 at High.
    def exports(app):
        return [export(app, "format=png&wav=on", MEDIA / "export_png"),
                export(app, "format=mp4&quality=high", MEDIA / "export_mp4")]
    facts["exports"] = session(exe, "p28_shot.json", exports)
    print("exports done", flush=True)
    (OUT / "measured.json").write_text(json.dumps(facts, indent=1) + "\n", encoding="utf-8", newline="\n")
    print("measured.json written")


# ---- the pictures and tables, from what measure() kept: no app needed ----

def lin(c8):
    c = c8.astype(np.float64) / 255
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def enc(c):
    c = np.clip(c, 0, 1)
    return np.where(c <= 0.0031308, c * 12.92, 1.055 * c ** (1 / 2.4) - 0.055) * 255


def mp4_frames(path, wanted):
    import subprocess
    p = subprocess.Popen(["ffmpeg", "-v", "error", "-i", str(path), "-f", "rawvideo", "-pix_fmt", "rgba", "-"],
                         stdout=subprocess.PIPE)
    got, n = {}, 0
    while True:
        b = p.stdout.read(W * H * 4)
        if len(b) < W * H * 4:
            break
        if n in wanted:
            got[n] = np.frombuffer(b, np.uint8).reshape(H, W, 4)
        n += 1
    p.wait()
    return got, n


def first_loud(path, at=0.1):
    """(first sample index at or above `at` in either channel, sample rate) of a sound, decoded by ffmpeg."""
    import subprocess
    raw = subprocess.run(["ffmpeg", "-v", "error", "-i", str(path), "-f", "f32le", "-ac", "2", "-ar", str(RATE), "-"],
                         capture_output=True).stdout
    s = np.abs(np.frombuffer(raw, np.float32).reshape(-1, 2)).max(axis=1)
    hit = np.nonzero(s >= at)[0]
    return (int(hit[0]) if len(hit) else None), len(s)


def report():
    from PIL import Image, ImageDraw
    m = json.loads((OUT / "measured.json").read_text(encoding="utf-8"))
    get = lambda tag, n: np.load(RUNS / f"{tag}_{n:04d}.npy")
    md = []
    row = lambda *c: md.append("| " + " | ".join(str(x) for x in c) + " |")

    # 1. Keys and time.
    md += ["## 1. Keys and time (written by `tools/p28_media.py report`)", "",
           "Each frame asked for out of order in a fresh window, against the same frame in a second fresh window that drew 0..119 in order.", "",
           "| frame | out of order = in order (processor) | largest difference | card out of order vs processor: largest | pixels off by more than 2 |",
           "|---|---|---|---|---|"]
    same_all = True
    for n in TEN:
        a, b, c = get("scrambled_cpu", n), get("sequence_cpu", n), get("scrambled_card", n)
        same = np.array_equal(a, b)
        same_all &= same
        d = np.abs(a.astype(int) - b.astype(int)).max()
        dc = np.abs(c.astype(int) - a.astype(int))
        row(n, "yes, byte for byte" if same else "NO", d, dc.max(), int((dc.max(axis=2) > 2).sum()))
    h_cpu = np.array_equal(get("sequence_cpu", 48), get("sequence_cpu", 49))
    h_card = np.array_equal(get("scrambled_card", 48), get("scrambled_card", 49))
    h_d = np.abs(get("sequence_cpu", 48).astype(int) - get("sequence_cpu", 49).astype(int)).max()
    md += ["", f"Hold frames 48 and 49 identical: processor **{'yes' if h_cpu else 'NO (largest difference %d)' % h_d}**, "
           f"card **{'yes' if h_card else 'no'}**. All ten frames the same in both orders: **{'yes' if same_all else 'NO'}**.", ""]
    fails = {n: v["failed"] for n, v in m["scrambled"]["cpu"].items() if v["failed"]}
    md += [f"Render problems reported by the window: {fails or 'none'}.", ""]

    # 2. Compositing at frame 47 (the soft edge) and 90 (Screen glow).
    full, back = get("sequence_cpu", 47), get("p28_shot_nosubject", 47)
    fg = np.asarray(Image.open(MEDIA / "fg" / "fg_subject.png").convert("RGBA"))
    x0 = 1240 - 540
    S = np.zeros((H, W, 4), np.uint8)
    S[:, x0:x0 + 1080] = fg[:, :W - x0]
    a = S[..., 3:4].astype(np.float64) / 255
    want_lin = enc(lin(S[..., :3]) * a + lin(back[..., :3]) * (1 - a))
    want_srgb = S[..., :3] * a + back[..., :3] * (1 - a)
    edge = (a[..., 0] > 0.02) & (a[..., 0] < 0.98)
    edge[680:, :900] = False            # the title's box
    got = full[..., :3].astype(np.float64)
    dl, ds = (got - want_lin)[edge], (got - want_srgb)[edge]
    solo = get("p28_shot_subjectonly", 47)
    inside = (a[..., 0] >= 0.999)
    inside[680:, :900] = False
    in_d = np.abs(full[..., :3].astype(int) - S[..., :3].astype(int))[inside].max()
    md += ["## 2. Compositing (written by `tools/p28_media.py report`)", "",
           f"Soft edge of the subject at frame 47: {int(edge.sum())} edge pixels (alpha between 2% and 98%), the title's box left out.", "",
           "| expected as | mean difference (8-bit) | darkest miss (8-bit, negative = darker than expected) | largest miss |",
           "|---|---|---|---|"]
    row("mixed in linear light (the project's working space)", f"{dl.mean():+.2f}", f"{dl.min():+.1f}", f"{np.abs(dl).max():.1f}")
    row("mixed in sRGB values (what a fringe or gamma slip looks like)", f"{ds.mean():+.2f}", f"{ds.min():+.1f}", f"{np.abs(ds).max():.1f}")
    md += ["", f"Inside the subject (fully opaque) the picture equals the subject's own pixels to within {in_d} (layer order: subject over the plate). "
           f"The subject-only render has alpha {'with' if (solo[..., 3] < 255).any() else 'without'} see-through pixels.", ""]
    # 4x crop where row 540 crosses the left edge.
    xs = np.nonzero(np.abs(a[540, :, 0] - 0.5) < 0.25)[0]
    cx = int(xs[0]) if len(xs) else x0
    box = (cx - 40, 500, cx + 40, 580)
    crop = lambda px: Image.fromarray(px[box[1]:box[3], box[0]:box[2]]).resize((320, 320), Image.NEAREST)
    sheet = Image.new("RGB", (980, 350), "white")
    for i, px in enumerate((full[..., :3], np.round(want_lin).astype(np.uint8), np.round(want_srgb).astype(np.uint8))):
        sheet.paste(crop(px), (10 + i * 325, 25))
    d = ImageDraw.Draw(sheet)
    for i, t in enumerate(("the app", "expected, linear mix", "expected, sRGB mix (fringe)")):
        d.text((10 + i * 325, 6), t, fill="black")
    sheet.save(OUT / "P-28_edge_4x.png")
    md += [f"4x crop of the edge at x {box[0]}..{box[2]}, y {box[1]}..{box[3]}: `P-28_edge_4x.png` (the app, then the two expectations).", ""]

    full90, ng, go = get("sequence_cpu", 90), get("p28_shot_noglow", 90), get("p28_shot_glowonly", 90)
    ga = go[..., 3:4].astype(np.float64) / 255
    glow = (ga[..., 0] > 0.05)
    Bl, Gl = lin(ng[..., :3]), lin(go[..., :3]) * ga
    scr_lin = enc(Bl + Gl - Bl * Gl)
    Bs, Gs = ng[..., :3] / 255, go[..., :3] / 255 * ga
    scr_srgb = (Bs + Gs - Bs * Gs) * 255
    g90 = full90[..., :3].astype(np.float64)
    e1, e2 = np.abs(g90 - scr_lin)[glow], np.abs(g90 - scr_srgb)[glow]
    md += [f"Screen glow at frame 90: {int(glow.sum())} pixels where the glow copy shows.", "",
           "| Screen worked out | mean miss (8-bit) | largest miss |", "|---|---|---|"]
    row("in linear light", f"{e1.mean():.2f}", f"{e1.max():.0f}")
    row("in sRGB values", f"{e2.mean():.2f}", f"{e2.max():.0f}")
    md += ["", "The red/blue split (Chromatic Aberration on an adjustment layer, frames 48-54): pixels whose blue is 0.", "",
           "| frame | pixels with blue 0 | band from the left / right / top / bottom edge (row 540, column 960) |", "|---|---|---|"]
    for n in (47, 48, 49, 50, 54, 55):
        b = get("sequence_cpu", n)[..., 2]
        band = [int(np.argmax(v > 0)) for v in (b[540], b[540, ::-1], b[:, 960], b[::-1, 960])]
        row(n, int((b == 0).sum()), " / ".join(f"{x} px" for x in band))
    md.append("")

    # 3. Speed.
    s = m["scrambled"]["cpu"]
    seq = m["sequence"]["cpu"]
    fresh = [v["window_ms"] for v in seq.values() if not v["from_memory"]]
    dr = m["draft"]
    md += ["## 3. Speed (written by `tools/p28_media.py report`)", "",
           f"Machine: {m['machine']}. Build: `{m['exe']}` (release, written {m['exe_written']}), checkout {m['commit']}. "
           "Window placed off the screens (ANIME_COMPOSITOR_BACKGROUND=1), own web view profile, 1920x1080, 24 fps, 120 frames.", "",
           "| what | where | frames | median ms | fastest | slowest | source |", "|---|---|---|---|---|---|---|"]
    st = stats(fresh)
    row("Full, each frame drawn fresh, in order", "processor", st["n"], st["median"], st["min"], st["max"], "window's own time (x-ms)")
    st = stats([v["window_ms"] for v in s.values()])
    row("Full, the ten frames out of order", "processor", st["n"], st["median"], st["min"], st["max"], "x-ms")
    st = stats([v["window_ms"] for v in dr["cpu"].values()])
    row(f"Draft ({dr['cpu']['0']['width']}x{dr['cpu']['0']['height']}), ten frames", "processor", st["n"], st["median"], st["min"], st["max"], "x-ms")
    for tag, src in (("scrambled", "Full, ten frames out of order"), ("draft", "Draft, ten frames")):
        lg = m[tag]["card_log"]
        st = stats([v["tool_ms"] for v in m[tag]["card"].values()])
        row(src, "card", f"{lg['frames']} log rows", round(lg["median_ms"], 1), "-", round(lg["slowest_5_percent_ms"], 1), "session log (its slowest 5%)")
        row(src + ", whole round trip incl. reading pixels back", "card", st["n"], st["median"], st["min"], st["max"], "measured by the script")
    md += ["", "| playback at Full | memory filled (all 120) | frames held | memory held | three loops: frames asked for (log rows) | frames passed over (dropped) | median ms |",
           "|---|---|---|---|---|---|---|"]
    for side in ("cpu", "card"):
        p = m["play_" + side]
        L = p["loops_log"]
        row("processor" if side == "cpu" else "card", f"{p['filled_s']} s" if p["filled_s"] else "not filled in 600 s",
            p["ready_frames"], f"{p['held_bytes'] / 1e6:.0f} MB", f"{L['frames']} in {p['loops_s']} s ({p['loops_expected_frames']} frames played)", L["dropped"], round(L["median_ms"], 1))
    md.append("")

    # 4. Sound.
    mix = m["sequence"]["mix"]
    md += ["## 4. Sound (written by `tools/p28_media.py report`)", "",
           "The click starts at sample 96000 of sfx_impact.wav = 2.000 s = frame 48 (2000 samples a frame).", "",
           "| where | first sample at or above 0.1 | means |", "|---|---|---|"]
    for k, v in mix.items():
        a0 = int(k.split()[1].rstrip(","))
        fl = v["first_loud"]
        row(f"window's playback mix, {k}", fl, "silent" if fl is None else f"{a0 * 2000 + fl} from the start = frame {(a0 * 2000 + fl) / 2000:.3f}")
    ex = {e["query"]: e for e in m["exports"]}
    pngdir, mp4dir = MEDIA / "export_png", MEDIA / "export_mp4"
    wavs = sorted(pngdir.glob("*.wav"))
    mp4s = sorted(mp4dir.glob("*.mp4"))
    for f in wavs + mp4s:
        fl, total = first_loud(f)
        row(f"exported {f.name}", fl, "silent" if fl is None else f"{fl / RATE:.4f} s ({total} samples long)")
    if not wavs:
        row("exported WAV", "-", "no WAV in the PNG export folder")
    md.append("")

    # 5. Viewer against export.
    pngs = sorted(pngdir.glob("*.png"))
    md += ["## 5. Viewer against export (written by `tools/p28_media.py report`)", "",
           "| export | how it went | files |", "|---|---|---|"]
    for e in m["exports"]:
        row(e["query"], "dialog filler: " + e["dialog"].replace("\n", "; ") + f"; export started: {e['ran']}", len(e["files"]))
    mp4px, mp4n = mp4_frames(mp4s[0], set(FIVE)) if mp4s else ({}, 0)
    md += ["", f"{len(pngs)} PNG frames, {mp4n} MP4 frames decoded.", "",
           "| frame | PNG = viewer | PNG largest difference | MP4 largest difference | MP4 mean difference |", "|---|---|---|---|---|"]
    side = Image.new("RGB", (3 * 480 + 40, 5 * 290 + 30), "white")
    d = ImageDraw.Draw(side)
    for i, t in enumerate(("viewer (processor)", "PNG export", "MP4 export")):
        d.text((10 + i * 490, 8), t, fill="black")
    for r, n in enumerate(FIVE):
        v = get("sequence_cpu", n)
        p = np.asarray(Image.open(pngs[n]).convert("RGBA")) if n < len(pngs) else None
        q = mp4px.get(n)
        dp = "-" if p is None else int(np.abs(p[..., :3].astype(int) - v[..., :3].astype(int)).max())
        dq = None if q is None else np.abs(q[..., :3].astype(int) - v[..., :3].astype(int))
        row(n, "-" if p is None else ("yes, exactly" if dp == 0 else "NO"), dp, "-" if dq is None else dq.max(),
            "-" if dq is None else f"{dq.mean():.2f}")
        for i, px in enumerate((v, p, q)):
            if px is not None:
                side.paste(Image.fromarray(px[..., :3]).resize((480, 270), Image.BILINEAR), (10 + i * 490, 25 + r * 290))
        d.text((10, 25 + r * 290 + 272), f"frame {n}", fill="black")
    if pngs or mp4s:
        side.save(OUT / "P-28_viewer_vs_export.png")
        md += ["", "Side by side (viewer, PNG, MP4) at frames 0, 48, 50, 90, 119: `P-28_viewer_vs_export.png`.", ""]
    else:
        (OUT / "P-28_viewer_vs_export.png").unlink(missing_ok=True)
        md += ["", "**Not measured:** nothing was exported, so there is no side-by-side picture.", ""]

    sheet = Image.new("RGB", (5 * 390 + 10, 2 * 240 + 10), "white")
    d = ImageDraw.Draw(sheet)
    for i, n in enumerate(TEN):
        x, y = 10 + (i % 5) * 390, 10 + (i // 5) * 240
        sheet.paste(Image.fromarray(get("sequence_cpu", n)[..., :3]).resize((380, 214), Image.BILINEAR), (x, y))
        d.text((x, y + 216), f"frame {n}", fill="black")
    sheet.save(OUT / "P-28_contact_sheet.png")
    md += ["Contact sheet of the ten frames: `P-28_contact_sheet.png`.", ""]
    (OUT / "P-28_measured_tables.md").write_text("\n".join(md) + "\n", encoding="utf-8", newline="\n")
    print("\n".join(md))


def exports_again():
    """Only step 6 of measure(), into the same measured.json (the export dialog is the fragile part)."""
    exe = Path(sys.argv[2])
    f = OUT / "measured.json"
    facts = json.loads(f.read_text(encoding="utf-8"))
    RUNS.mkdir(parents=True, exist_ok=True)
    facts["exports"] = session(exe, "p28_shot.json", lambda app: [
        export(app, "format=png&wav=on", MEDIA / "export_png"),
        export(app, "format=mp4&quality=high", MEDIA / "export_mp4")])
    f.write_text(json.dumps(facts, indent=1) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps(facts["exports"], indent=1))


if __name__ == "__main__":
    {"media": media, "project": project, "measure": measure, "report": report,
     "exports": exports_again}[sys.argv[1]]()
