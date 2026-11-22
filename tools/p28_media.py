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
    print(f"wrote {OUT / 'p28_shot.json'} and its two check variants")


if __name__ == "__main__":
    {"media": media, "project": project}[sys.argv[1]]()
