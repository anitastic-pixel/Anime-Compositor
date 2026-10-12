"""Pixel Motion Blur, worked a second way.

D-650 proposes Pixel Motion Blur (`core.pixel_motion_blur`), after After Effects' Pixel Motion
Blur (Time): motion blur for footage, the movement found by this program's own estimate between
the present frame and the frames either side (`tools/flow_reference.py`), and the picture smeared
along it. Adobe's page names the four controls and says Automatic takes the composition's motion
blur settings; it publishes no formula, and the estimate, the sampling and the settings when
added are ours (P0-13, D-600: "Blur + retiming only").

At the holder's composition frame n, with its input O, linear and premultiplied, W by H:

1. On an adjustment layer the output is O, and the frame says TEMPORAL_SMOOTHING_SKIPPED: the
   frames beneath at other times are not to hand.
2. With `shutter_control` `automatic` the angle and the sample count are the composition's
   motion blur settings (D-188; 180 and 16 when it has none), whether its switch is on or not;
   with `manual` they are `shutter_angle` and `shutter_samples`. a = angle / 360, the share of
   a frame the shutter is open; N = floor(samples). With a = 0 the output is O.
3. P_- and P_+ are the holder at composition frames n - 1 and n + 1 with the effects before
   Pixel Motion Blur, by the layer's own timing, lying on O corner to corner and cut to W by H,
   transparent outside. A frame outside the layer's in and out points is O itself.
4. F = the estimate from O to P_+ and B = the estimate from O to P_-, both with `vector_detail`
   and the estimate's other settings as added (Global and Local Smoothness 0.5, 4 iterations,
   Block Size 6, weights 0.3, 0.6, 0.1, no luminance correction).
5. The shutter is centred on the frame: s_k = a ((k + 0.5) / N - 0.5), k = 0 to N - 1, in
   frames. V_k = B where s_k < 0, otherwise F.
6. The output at pixel (x, y) = (1/N) sum over k of O read bilinearly by document 21 (pixel
   centres at halves, transparent outside) at (x + 0.5 - |s_k| V_k,x, y + 0.5 - |s_k| V_k,y),
   all four numbers. The pixel gathers what passed over it while the shutter was open, each
   moment read from the present frame along the motion at that pixel. The bounds do not grow.

`shutter_control`, `automatic` or `manual`, `automatic` when added; `shutter_angle`, 0 to 720
degrees, 180; `shutter_samples`, 2 to 64, 16; `vector_detail`, 0 to 100, 20. The numbers are
keyable and read at frame n.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 32 by 32, six frames, in `Fixtures/pixel_motion_blur/`:
the drawing `holder`, either a soft disc running 3 right and 1 down a frame across six drawings
or a textured card sliding 1.5 to the right a frame, with the effect. The expected pixels are in
`Fixtures/pixel_motion_blur/expected_pixel_motion_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/pixel_motion_blur_reference.py
"""

import json
import sys
from math import cos, floor, pi, sin
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import flow_reference as flow  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "pixel_motion_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 32, 32, 6
CLEAR = [0.0, 0.0, 0.0, 0.0]
COMPOSITION_BLUR = {"enabled": False, "shutter_angle": 180, "shutter_phase": -90, "samples": 16}


def clamp8(v):
    return max(0, min(255, round(v)))


def disc(k):
    """Drawing k + 1: a disc of radius 6 with a soft rim, its middle at (8.5 + 3k, 10.5 + k), its
    colour shaded across and down from its middle so the inside has detail to follow."""
    cx, cy = 8.5 + 3 * k, 10.5 + k
    rows = []
    for y in range(H):
        row = []
        for x in range(W):
            dx, dy = x + 0.5 - cx, y + 0.5 - cy
            cover = min(max((6 - (dx * dx + dy * dy) ** 0.5) / 1.5 + 0.5, 0.0), 1.0)
            row.append((240, clamp8(140 + 12 * dx), clamp8(60 + 12 * dy), clamp8(255 * cover)))
        rows.append(row)
    return rows


def card(k):
    """Drawing k + 1: an opaque texture of soft stripes slid 1.5k to the right."""
    rows = []
    for y in range(H):
        row = []
        for x in range(W):
            u = x + 0.5 - 1.5 * k
            v = 0.5 + 0.3 * sin(2 * pi * u / 11) + 0.15 * cos(2 * pi * (y + 0.5) / 9 + u / 5)
            row.append((clamp8(255 * v), clamp8(90 + 100 * v), clamp8(230 - 150 * v), 255))
        rows.append(row)
    return rows


DRAWINGS = {**{f"disc_{k + 1}": disc(k) for k in range(FRAMES)},
            **{f"card_{k + 1}": card(k) for k in range(FRAMES)}}
ONES = [{"start_frame": k, "end_frame_exclusive": k + 1, "drawing_number": k + 1}
        for k in range(FRAMES)]
TWOS = [{"start_frame": 2 * i, "end_frame_exclusive": 2 * i + 2, "drawing_number": 2 * i + 1}
        for i in range(FRAMES // 2)]
STILL = [{"start_frame": 0, "end_frame_exclusive": FRAMES, "drawing_number": 3}]


def decoded(asset, number):
    name = f"{asset}_{number}"
    L.DRAWINGS[name] = DRAWINGS[name]
    return L.decoded(name)


# --- the rule -------------------------------------------------------------------------------

def drawn(c, m):
    """The holder at composition frame m with the effects before Pixel Motion Blur, or None
    outside its in and out points."""
    if not c["in"] <= m < FRAMES:
        return None
    local = m - c["in"]
    number = next(s["drawing_number"] for s in c["spans"]
                  if s["start_frame"] <= local < s["end_frame_exclusive"])
    p = decoded(c["asset"], number)
    return L.exposure(p, c["before"]) if c["before"] is not None else p


def shutter(c, n):
    if c["shutter_control"] == "automatic":
        blur = c["composition_blur"] or COMPOSITION_BLUR
        return blur["shutter_angle"] / 360, blur["samples"]
    return value_at(c["shutter_angle"], n) / 360, floor(value_at(c["shutter_samples"], n))


def blurred(c, n):
    """Steps 2 to 6 at frame n."""
    o = drawn(c, n)
    a, count = shutter(c, n)
    if a == 0:
        return o
    before, after = drawn(c, n - 1) or o, drawn(c, n + 1) or o
    settings = {"vector_detail": value_at(c["vector_detail"], n)}
    fwd = flow.estimate(o, after, settings)
    back = flow.estimate(o, before, settings)
    times = [a * ((k + 0.5) / count - 0.5) for k in range(count)]
    out = []
    for y in range(o["h"]):
        for x in range(o["w"]):
            i = y * o["w"] + x
            acc = [0.0] * 4
            for s in times:
                vx, vy = (back[0][i], back[1][i]) if s < 0 else (fwd[0][i], fwd[1][i])
                q = L.bilinear(o, x + 0.5 - abs(s) * vx, y + 0.5 - abs(s) * vy)
                acc = [p + v for p, v in zip(acc, q)]
            out.append([v / count for v in acc])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(shutter_control="automatic", shutter_angle=180, shutter_samples=16, vector_detail=20,
         asset="disc", spans=ONES, start=0, shift=(0, 0), before=None, after=None,
         composition_blur=None, on="holder"):
    return {"shutter_control": shutter_control, "shutter_angle": shutter_angle,
            "shutter_samples": shutter_samples, "vector_detail": vector_detail, "asset": asset,
            "spans": spans, "in": start, "shift": shift, "before": before, "after": after,
            "composition_blur": composition_blur, "on": on}


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        out = drawn(c, frame)
    else:
        out = blurred(c, frame)
        if c["after"] is not None:
            out = L.exposure(out, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    dx, dy = c["shift"]
    p = drawn(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


MANUAL = {"shutter_control": "manual"}
CASES = {
    "FX-PMB-001": ("As added: Automatic, so the composition's 180 degrees and 16 samples, Vector "
                   "Detail 20. The disc is smeared along its run, half a frame's travel; at "
                   "frame 0 the frame before is the layer's first, and at frame 5 the frame "
                   "after is past its out point, so each smears one way only.", case(),
                   (0, 2, 5)),
    "FX-PMB-002": ("Manual, 360 degrees in 8 samples: a whole frame's travel.",
                   case(**MANUAL, shutter_angle=360, shutter_samples=8), (2,)),
    "FX-PMB-003": ("Manual, 0 degrees: the shutter never opens, the layer exactly as it is.",
                   case(**MANUAL, shutter_angle=0), (2,)),
    "FX-PMB-004": ("Manual, 720 degrees: two frames' travel, the farthest it goes.",
                   case(**MANUAL, shutter_angle=720), (2,)),
    "FX-PMB-005": ("Vector Detail 100: a vector found for every pixel.",
                   case(vector_detail=100), (2,)),
    "FX-PMB-006": ("Vector Detail 0: the coarsest vectors, on a picture this small those of "
                   "Vector Detail 20.", case(vector_detail=0), (2,)),
    "FX-PMB-007": ("Vector Detail 50: one vector for every two pixels.",
                   case(vector_detail=50), (2,)),
    "FX-PMB-008": ("A drawing that never moves: the estimate finds no motion and the layer "
                   "stays as it is.", case(spans=STILL), (2,)),
    "FX-PMB-009": ("The textured card sliding 1.5 to the right, as added.",
                   case(asset="card"), (2,)),
    "FX-PMB-010": ("The card, Manual 360 degrees in 16 samples, Vector Detail 100.",
                   case(**MANUAL, shutter_angle=360, asset="card", vector_detail=100), (2,)),
    "FX-PMB-011": ("Automatic with the composition's blur set to 90 degrees and 4 samples, its "
                   "switch off: those are the ones used.",
                   case(composition_blur={"enabled": False, "shutter_angle": 90,
                                          "shutter_phase": -45, "samples": 4},
                        shutter_angle=720, shutter_samples=64), (2,)),
    "FX-PMB-012": ("Manual, 360 degrees in the fewest samples, 2: two copies a half frame "
                   "apart.", case(**MANUAL, shutter_angle=360, shutter_samples=2), (2,)),
    "FX-PMB-013": ("An Exposure of +1 before Pixel Motion Blur: the frames either side are "
                   "brightened too, and the brighter picture is smeared.", case(before=1),
                   (2,)),
    "FX-PMB-014": ("An Exposure of -1 after Pixel Motion Blur darkens the smear too.",
                   case(after=-1), (2,)),
    "FX-PMB-015": ("The holder moved 3 right and 2 down: FX-PMB-001 moved, since the smear lies "
                   "on the layer.", case(shift=(3, 2)), (2,)),
    "FX-PMB-016": ("The effect on an adjustment layer above the holder: nothing changes, and "
                   "the frame says why.", case(on="adjust"), (2,)),
    "FX-PMB-017": ("Manual, the angle keyed from 0 at frame 0 to 360 at frame 4: 180 at frame "
                   "2.", case(**MANUAL, shutter_angle=keyed((0, 0), (4, 360))), (0, 2)),
    "FX-PMB-018": ("Drawings on twos: at frame 2 the frame after shows the same drawing, so "
                   "only the motion from the frame before smears; at frame 3 only the motion "
                   "to the frame after.", case(spans=TWOS), (2, 3)),
    "FX-PMB-019": ("The layer's in point at frame 2: at frame 2 the frame before is outside "
                   "the layer, so only the motion to the frame after smears.",
                   case(start=2), (2, 3)),
}

INVALID = {
    "FX-PMB-020": ("Shutter Angle 721, above 720.", case(**MANUAL, shutter_angle=721)),
    "FX-PMB-021": ("Shutter Samples 1, below 2.", case(**MANUAL, shutter_samples=1)),
    "FX-PMB-022": ("Vector Detail 101, above 100.", case(vector_detail=101)),
    "FX-PMB-023": ("Shutter Control written \"maybe\".", case(shutter_control="maybe")),
    "FX-PMB-024": ("Shutter Samples keyed to 100 at frame 4.",
                   case(**MANUAL, shutter_samples=keyed((0, 16), (4, 100)))),
}

KEYS = ("shutter_control", "shutter_angle", "shutter_samples", "vector_detail")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.pixel_motion_blur", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def project_json(fx, c):
    asset = c["asset"]
    holder = L.raster("holder", "asset-" + asset, spans=c["spans"], position=c["shift"],
                      in_frame=c["in"], out_frame=FRAMES)
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    comp = L.composition("comp-main", W, H, FRAMES, layers)
    if c["composition_blur"] is not None:
        comp["motion_blur"] = c["composition_blur"]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-" + asset, "kind": "image_sequence", "name": asset,
                        "pattern": asset + "_####.png",
                        "frames": {str(k): f"media/{asset}_{k}.png" for k in range(1, FRAMES + 1)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [comp]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)), "frames": rendered}
        if c["on"] == "adjust":
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": plain(c, 0), "4": plain(c, 4)}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_pixel_motion_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    # The estimate finds the disc's run at its middle, and the card's slide.
    o, nxt = drawn(case(), 2), drawn(case(), 3)
    u = flow.estimate(o, nxt, {"vector_detail": 100})
    i = 13 * W + 15
    assert abs(u[0][i] - 3) < 0.35 and abs(u[1][i] - 1) < 0.35, (u[0][i], u[1][i])
    u = flow.estimate(drawn(case(asset="card"), 2), drawn(case(asset="card"), 3),
                      {"vector_detail": 100})
    assert abs(u[0][16 * W + 16] - 1.5) < 0.2 and abs(u[1][16 * W + 16]) < 0.2
    # Changed where it should be, and only so far as the shutter reaches.
    for fx in ("FX-PMB-001", "FX-PMB-002", "FX-PMB-004", "FX-PMB-009", "FX-PMB-012"):
        assert c[fx]["2"] != plain(case(), 2) and c[fx]["2"] != plain(case(asset="card"), 2), fx
    assert c["FX-PMB-003"]["2"] == plain(case(), 2)
    assert near(c["FX-PMB-008"]["2"], plain(case(spans=STILL), 2))
    assert near(c["FX-PMB-006"]["2"], c["FX-PMB-001"]["2"])
    assert c["FX-PMB-005"]["2"] != c["FX-PMB-001"]["2"] != c["FX-PMB-007"]["2"]
    # Automatic ignores the effect's own numbers and takes the composition's.
    assert near(c["FX-PMB-011"]["2"], render(case(**MANUAL, shutter_angle=90,
                                                  shutter_samples=4), 2))
    assert near(c["FX-PMB-001"]["2"], render(case(**MANUAL), 2))
    # A wider shutter smears farther: the leading edge of the disc reaches farther right.
    reach = lambda px: max(x for y in range(H) for x in range(W)  # noqa: E731
                           if px[y * W + x][3] > 0.01)
    assert reach(c["FX-PMB-003"]["2"]) <= reach(c["FX-PMB-001"]["2"]) \
        <= reach(c["FX-PMB-002"]["2"]) <= reach(c["FX-PMB-004"]["2"])
    assert reach(c["FX-PMB-003"]["2"]) < reach(c["FX-PMB-004"]["2"])
    assert near(c["FX-PMB-014"]["2"], [[p[0] / 2, p[1] / 2, p[2] / 2, p[3]]
                                       for p in c["FX-PMB-001"]["2"]])
    assert c["FX-PMB-013"]["2"] != c["FX-PMB-001"]["2"]
    moved = c["FX-PMB-015"]["2"]
    assert all(moved[(y + 2) * W + x + 3] == c["FX-PMB-001"]["2"][y * W + x]
               for y in range(H - 2) for x in range(W - 3))
    assert c["FX-PMB-016"]["2"] == plain(case(), 2)
    assert c["FX-PMB-017"]["0"] == plain(case(), 0)
    assert near(c["FX-PMB-017"]["2"], c["FX-PMB-001"]["2"])
    assert c["FX-PMB-018"]["2"] != plain(case(spans=TWOS), 2)
    assert c["FX-PMB-019"]["2"] != plain(case(start=2), 2)
    # Every pixel a picture can hold: numbers from 0 to 1, colour no more than covering.
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 2 + 1e-12 for a in p) and max(p[:3]) <= 2 * p[3] + 1e-12, fx
                assert p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
