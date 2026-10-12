"""Timewarp, worked a second way.

D-652 proposes Timewarp (`core.timewarp`), after After Effects' Timewarp (Time): the layer slowed,
sped up or sent to a frame of its own, with the frames between made by repeating, by mixing, or by
moving pixels along the motion this program's own estimate finds (`tools/flow_reference.py`).
Adobe's page names the controls and says what each is for; it publishes no formula, and the source
time, the in-between rule, the error mix and most settings when added are ours (P0-13, D-600:
"Blur + retiming only").

At the holder's composition frame n, with n0 its in point and m1 its last frame (out point - 1):

1. On an adjustment layer the output is the input, and the frame says
   TEMPORAL_SMOOTHING_SKIPPED: the frames beneath at other times are not to hand.
2. The source time t, in composition frames. With `adjust_time_by` `speed`, s(k) = Speed at
   frame k / 100 and t = n0 - 0.5, then s(k) added for k = n0 to n - 1 in turn, then 0.5 s(n);
   the rate is s(n). Constant 50 gives n0 + 0.25, n0 + 0.75 and so on; 100 gives n. With
   `source_frame`, t = n0 + Source Frame at n, and the rate is (Source Frame at n + 1 - Source
   Frame at n - 1) / 2. Then t is held inside n0 to m1.
3. P(m) is the holder at composition frame m with the effects before Timewarp, by the layer's own
   timing, lying on this frame's picture corner to corner. A = floor(t), B = min(A + 1, m1),
   f = t - A.
4. The moments. With Enable Motion Blur `off`, or a shutter angle of 0, one: d = 0. Otherwise
   the angle and the sample count are the composition's motion blur settings with Shutter
   Control `automatic` (whether its switch is on or not; 180 and 16 when it has none), the
   effect's own with `manual`; a = angle / 360, N = floor(samples), and d_k = a ((k + 0.5) / N
   - 0.5) rate, k = 0 to N - 1. The output is the mean of the moments' pictures, in order.
5. One moment's picture:
   - Whole Frames: with t' = t + d held inside n0 to m1, P(floor(t')).
   - Frame Mix: with t' likewise, A' = floor(t'), B' = min(A' + 1, m1), g = t' - A':
     (1 - g) P(A') + g P(B'), all four numbers.
   - Pixel Motion: A, B and their estimates are step 3's; g = f + d, not held. With g = 0 the
     picture is P(A). Otherwise, with F01 the estimate from P(A) to P(B) and F10 from P(B) to
     P(A), both with the Tuning settings:
       F_a = -(1 - g) g F01 + g^2 F10,  F_b = (1 - g)^2 F01 - g (1 - g) F10,
       a = P(A) read at (x + 0.5 + F_a,x, y + 0.5 + F_a,y), b = P(B) at (x + 0.5 + F_b, ...),
     read bilinearly by document 21 with Filtering `normal`, by Keys' cubic with a = -0.5
     (Catmull-Rom, as Transform's bicubic, held at 0 and the covering at 1) with `extreme`;
     transparent outside. With h = g held inside 0 and 1: px = (1 - h) a + h b, or with Build
     From One Image `on`, a when g < 0.5 and b otherwise. The frame mix there is
     fm = (1 - h) P(A) + h P(B). Error Threshold T: e = |w.a - w.b| + |alpha(a) - alpha(b)|,
     with w the Tuning's red, green and blue weights on the premultiplied colour; m = min(1, T e);
     the picture is (1 - m) px + m fm.

Settings: `method` `whole_frames`, `frame_mix` or `pixel_motion`, `pixel_motion` when added;
`adjust_time_by` `speed` or `source_frame`, `speed`; `speed` -1000 to 1000 per cent, 50;
`source_frame` -100000 to 100000 frames, 0; Tuning: `vector_detail` 0 to 100, 20;
`global_smoothness` and `local_smoothness` 0 to 1, 0.5; `smoothing_iterations` 1 to 50, taken
whole, 4; `build_from_one_image` and `correct_luminance_changes` `off` or `on`, `off`; `filtering`
`normal` or `extreme`, `normal`; `error_threshold` 0 to 10, 1; `block_size` 2 to 32, 6;
`red_weight`, `green_weight`, `blue_weight` 0 to 1, 0.3, 0.6 and 0.1; Motion Blur:
`enable_motion_blur` `off` or `on`, `off`; `shutter_control` `automatic` or `manual`,
`automatic`; `shutter_angle` 0 to 720 degrees, 180; `shutter_samples` 2 to 64, 5. The numbers
are keyable and read at frame n (Speed at every frame it sums), the words not.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 32 by 32, six frames, in `Fixtures/timewarp/`: the
drawing `holder`, Pixel Motion Blur's soft disc or textured card (imported from
`tools/pixel_motion_blur_reference.py`), with the effect. The expected pixels are in
`Fixtures/timewarp/expected_timewarp.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/timewarp_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from transform_reference import bicubic  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import flow_reference as flow  # noqa: E402
import pixel_motion_blur_reference as P  # noqa: E402
from pixel_motion_blur_reference import W, H, FRAMES, ONES, TWOS, drawn  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "timewarp"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def source_time(c, n):
    """Step 2: t held inside the layer, and the rate."""
    n0, m1 = c["in"], FRAMES - 1
    if c["adjust_time_by"] == "speed":
        s = lambda k: value_at(c["speed"], k) / 100  # noqa: E731
        t = n0 - 0.5
        for k in range(n0, n):
            t += s(k)
        t += 0.5 * s(n)
        rate = s(n)
    else:
        t = n0 + value_at(c["source_frame"], n)
        rate = (value_at(c["source_frame"], n + 1) - value_at(c["source_frame"], n - 1)) / 2
    return min(max(t, n0), m1), rate


def moments(c, n, rate):
    """Step 4: each moment's shift in source frames."""
    if c["enable_motion_blur"] != "on":
        return [0.0]
    if c["shutter_control"] == "automatic":
        blur = c["composition_blur"] or P.COMPOSITION_BLUR
        a, count = blur["shutter_angle"] / 360, blur["samples"]
    else:
        a, count = value_at(c["shutter_angle"], n) / 360, floor(value_at(c["shutter_samples"], n))
    if a == 0:
        return [0.0]
    return [a * ((k + 0.5) / count - 0.5) * rate for k in range(count)]


def tuning(c, n):
    return {"vector_detail": value_at(c["vector_detail"], n),
            "global_smoothness": value_at(c["global_smoothness"], n),
            "local_smoothness": value_at(c["local_smoothness"], n),
            "iterations": floor(value_at(c["smoothing_iterations"], n)),
            "block_size": value_at(c["block_size"], n),
            "weights": tuple(value_at(c[k], n) for k in ("red_weight", "green_weight", "blue_weight")),
            "correct_luminance": c["correct_luminance_changes"] == "on"}


def mix(p, q, h):
    return [[(1 - h) * u + h * v for u, v in zip(a, b)] for a, b in zip(p["px"], q["px"])]


def held_frames(c, t):
    n0, m1 = c["in"], FRAMES - 1
    t = min(max(t, n0), m1)
    a = floor(t)
    return a, min(a + 1, m1), t - a


def warped(c, n, pa, pb, fwd, back, g):
    """Step 5's Pixel Motion picture at g."""
    if g == 0:
        return [list(q) for q in pa["px"]]
    s = tuning(c, n)
    wr, wg, wb = s["weights"]
    threshold = value_at(c["error_threshold"], n)
    h = min(max(g, 0.0), 1.0)
    extreme = c["filtering"] == "extreme"
    out = []
    for y in range(pa["h"]):
        for x in range(pa["w"]):
            i = y * pa["w"] + x
            fax = -(1 - g) * g * fwd[0][i] + g * g * back[0][i]
            fay = -(1 - g) * g * fwd[1][i] + g * g * back[1][i]
            fbx = (1 - g) ** 2 * fwd[0][i] - g * (1 - g) * back[0][i]
            fby = (1 - g) ** 2 * fwd[1][i] - g * (1 - g) * back[1][i]
            if extreme:
                a = bicubic(dict(pa, left=0, top=0), x + 0.5 + fax, y + 0.5 + fay)
                b = bicubic(dict(pb, left=0, top=0), x + 0.5 + fbx, y + 0.5 + fby)
            else:
                a = L.bilinear(pa, x + 0.5 + fax, y + 0.5 + fay)
                b = L.bilinear(pb, x + 0.5 + fbx, y + 0.5 + fby)
            if c["build_from_one_image"] == "on":
                px = a if g < 0.5 else b
            else:
                px = [(1 - h) * u + h * v for u, v in zip(a, b)]
            fm = [(1 - h) * u + h * v for u, v in zip(pa["px"][i], pb["px"][i])]
            e = abs((wr * a[0] + wg * a[1] + wb * a[2]) - (wr * b[0] + wg * b[1] + wb * b[2])) \
                + abs(a[3] - b[3])
            m = min(1.0, threshold * e)
            out.append([(1 - m) * u + m * v for u, v in zip(px, fm)])
    return out


def retimed(c, n):
    """Steps 2 to 5 at frame n."""
    t, rate = source_time(c, n)
    shifts = moments(c, n, rate)
    pictures = []
    if c["method"] == "pixel_motion":
        a, b, f = held_frames(c, t)
        pa, pb = drawn(c, a), drawn(c, b)
        fwd = back = None
        if any(f + d != 0 for d in shifts):
            s = tuning(c, n)
            fwd, back = flow.estimate(pa, pb, s), flow.estimate(pb, pa, s)
        for d in shifts:
            pictures.append(warped(c, n, pa, pb, fwd, back, f + d))
    else:
        for d in shifts:
            a, b, g = held_frames(c, t + d)
            if c["method"] == "whole_frames":
                pictures.append([list(q) for q in drawn(c, a)["px"]])
            else:
                pictures.append(mix(drawn(c, a), drawn(c, b), g))
    count = len(pictures)
    out = [[0.0] * 4 for _ in range(W * H)]
    for p in pictures:
        for o, q in zip(out, p):
            for k in range(4):
                o[k] += q[k]
    return L.pic(W, H, [[v / count for v in o] for o in out])


# --- the cases ------------------------------------------------------------------------------

ADDED = {"method": "pixel_motion", "adjust_time_by": "speed", "speed": 50, "source_frame": 0,
         "vector_detail": 20, "global_smoothness": 0.5, "local_smoothness": 0.5,
         "smoothing_iterations": 4, "build_from_one_image": "off",
         "correct_luminance_changes": "off", "filtering": "normal", "error_threshold": 1,
         "block_size": 6, "red_weight": 0.3, "green_weight": 0.6, "blue_weight": 0.1,
         "enable_motion_blur": "off", "shutter_control": "automatic", "shutter_angle": 180,
         "shutter_samples": 5}
KEYS = tuple(ADDED)


def case(asset="disc", spans=ONES, start=0, shift=(0, 0), before=None, after=None,
         composition_blur=None, on="holder", **settings):
    c = dict(ADDED)
    c.update(settings)
    c.update({"asset": asset, "spans": spans, "in": start, "shift": shift, "before": before,
              "after": after, "composition_blur": composition_blur, "on": on})
    return c


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        out = drawn(c, frame)
    else:
        out = retimed(c, frame)
        if c["after"] is not None:
            out = L.exposure(out, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    dx, dy = c["shift"]
    p = drawn(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


BLUR_ON = {"enable_motion_blur": "on"}
MANUAL = {"enable_motion_blur": "on", "shutter_control": "manual"}
CASES = {
    "FX-TW-001": ("As added: Pixel Motion at Speed 50. Frame 0 is the first drawing (the source "
                  "time, a quarter frame before the in point, held there); frames 1, 2 and 5 are "
                  "made a quarter, three quarters and a quarter of the way between drawings.",
                  case(), (0, 1, 2, 5)),
    "FX-TW-002": ("Whole Frames at Speed 50: each drawing shown twice, no new pictures.",
                  case(method="whole_frames"), (1, 2, 3)),
    "FX-TW-003": ("Frame Mix at Speed 50: the two drawings either side faded together.",
                  case(method="frame_mix"), (1, 2)),
    "FX-TW-004": ("Speed 100: the layer exactly as it is.", case(speed=100), (0, 3)),
    "FX-TW-005": ("Speed 200: twice as fast, made halfway between drawings 3 and 4 at frame 1; "
                  "from frame 3 on it holds the last drawing.", case(speed=200), (1, 3)),
    "FX-TW-006": ("Speed keyed from 200 at frame 0 down to 0 at frame 4: it slows to a stop at "
                  "source time 4.5.", case(speed=keyed((0, 200), (4, 0))), (2, 4)),
    "FX-TW-007": ("Source Frame keyed from 0 at frame 0 to 5 at frame 4: frame 1 shows source "
                  "frame 1.25 and frame 2 shows 2.5.",
                  case(adjust_time_by="source_frame", source_frame=keyed((0, 0), (4, 5))), (1, 2)),
    "FX-TW-008": ("Source Frame 3.5, Frame Mix: halfway between drawings 4 and 5 on every frame.",
                  case(adjust_time_by="source_frame", source_frame=3.5, method="frame_mix"),
                  (0, 4)),
    "FX-TW-009": ("Build From One Image: a quarter of the way uses the drawing before only, three "
                  "quarters the drawing after only.", case(build_from_one_image="on"), (1, 2)),
    "FX-TW-010": ("Error Threshold 0: never mixed toward Frame Mix.", case(error_threshold=0),
                  (1, 2)),
    "FX-TW-011": ("Error Threshold 10: wherever the two warped pictures disagree, mixed toward "
                  "Frame Mix.", case(error_threshold=10), (1, 2)),
    "FX-TW-012": ("Filtering Extreme: the warped pictures read by the cubic.",
                  case(filtering="extreme"), (1, 2)),
    "FX-TW-013": ("Correct Luminance Changes on, on the textured card, whose mean brightness "
                  "changes as it slides.", case(asset="card", correct_luminance_changes="on"),
                  (2,)),
    "FX-TW-014": ("Tuning changed: Vector Detail 100, Global Smoothness 1, Local Smoothness 0, "
                  "8 iterations, Block Size 4, weights 1, 0 and 0.",
                  case(vector_detail=100, global_smoothness=1, local_smoothness=0,
                       smoothing_iterations=8, block_size=4, red_weight=1, green_weight=0,
                       blue_weight=0), (2,)),
    "FX-TW-015": ("Motion Blur on, Automatic, with the composition's 180 degrees in 8 samples "
                  "(its switch off): the moments spread a quarter frame of source time.",
                  case(**BLUR_ON, composition_blur={"enabled": False, "shutter_angle": 180,
                                                    "shutter_phase": -90, "samples": 8}), (2,)),
    "FX-TW-016": ("Motion Blur on, Manual, 360 degrees in 4 samples, Whole Frames at Speed 200: "
                  "drawings 2, 3, 3 and 4 averaged, the source times 1.75, 2.25, 2.75 and 3.25.",
                  case(**MANUAL, shutter_angle=360, shutter_samples=4, method="whole_frames",
                       speed=200), (1,)),
    "FX-TW-017": ("Motion Blur on, Manual, 360 degrees in 4 samples, Frame Mix on keyed Source "
                  "Frame.", case(**MANUAL, shutter_angle=360, shutter_samples=4,
                                 method="frame_mix", adjust_time_by="source_frame",
                                 source_frame=keyed((0, 0), (4, 5))), (2,)),
    "FX-TW-018": ("Motion Blur on, Manual, 0 degrees: as with it off.",
                  case(**MANUAL, shutter_angle=0), (2,)),
    "FX-TW-019": ("The textured card, as added.", case(asset="card"), (1, 2)),
    "FX-TW-020": ("An Exposure of +1 before Timewarp: the frames it reads are brightened too.",
                  case(before=1), (1,)),
    "FX-TW-021": ("An Exposure of -1 after Timewarp darkens its picture.", case(after=-1), (1,)),
    "FX-TW-022": ("The layer's in point at frame 2: its source time starts there.",
                  case(start=2), (2, 3, 4)),
    "FX-TW-023": ("The holder moved 3 right and 2 down: FX-TW-001 moved.", case(shift=(3, 2)),
                  (1,)),
    "FX-TW-024": ("The effect on an adjustment layer above the holder: nothing changes, and the "
                  "frame says why.", case(on="adjust"), (2,)),
    "FX-TW-025": ("Drawings on twos at Speed 50: between two frames of one drawing nothing "
                  "moves, so that drawing shows exactly.", case(spans=TWOS), (1, 2)),
    "FX-TW-026": ("Speed -100: the source time runs back from the in point, held at the first "
                  "drawing.", case(speed=-100), (0, 3)),
}

INVALID = {
    "FX-TW-027": ("Speed 1001, above 1000.", case(speed=1001)),
    "FX-TW-028": ("Method written \"optical\".", case(method="optical")),
    "FX-TW-029": ("Error Threshold -1, below 0.", case(error_threshold=-1)),
    "FX-TW-030": ("Smoothing Iterations 0, below 1.", case(smoothing_iterations=0)),
    "FX-TW-031": ("Red Weight 2, above 1.", case(red_weight=2)),
    "FX-TW-032": ("Filtering written \"sinc\".", case(filtering="sinc")),
    "FX-TW-033": ("Shutter Samples keyed to 100 at frame 4.",
                  case(shutter_samples=keyed((0, 5), (4, 100)))),
}


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.timewarp", "enabled": True,
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
    for name, pixels in P.DRAWINGS.items():
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

    (OUT / "expected_timewarp.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    d = lambda k, asset="disc": plain(case(asset=asset), k)  # noqa: E731
    # Source times.
    assert source_time(case(), 0) == (0, 0.5) and source_time(case(), 2) == (0.75, 0.5)
    assert source_time(case(speed=200), 1) == (2.5, 2.0)
    assert source_time(case(speed=keyed((0, 200), (4, 0))), 4) == (4.5, 0.0)
    assert source_time(case(adjust_time_by="source_frame",
                            source_frame=keyed((0, 0), (4, 5))), 2) == (2.5, 1.25)
    # Exactly a drawing where the source time is whole, and only there.
    assert c["FX-TW-001"]["0"] == d(0)
    assert c["FX-TW-004"]["0"] == d(0) and c["FX-TW-004"]["3"] == d(3)
    assert c["FX-TW-005"]["3"] == d(5)
    assert c["FX-TW-026"]["0"] == d(0) and c["FX-TW-026"]["3"] == d(0)
    for k in ("1", "2", "5"):
        assert all(c["FX-TW-001"][k] != d(j) for j in range(FRAMES)), k
    # Whole Frames repeats drawings; Frame Mix is the plain fade.
    assert c["FX-TW-002"]["1"] == d(0) and c["FX-TW-002"]["2"] == d(0) \
        and c["FX-TW-002"]["3"] == d(1)
    assert near(c["FX-TW-003"]["1"], [[0.75 * u + 0.25 * v for u, v in zip(p, q)]
                                      for p, q in zip(d(0), d(1))])
    assert near(c["FX-TW-008"]["0"], c["FX-TW-008"]["4"])
    # Pixel Motion moves the disc partway rather than fading two: its middle is solid.
    mid = lambda px: sum(x * p[3] for i, p in enumerate(px) for x in [i % W]) \
        / sum(p[3] for p in px)  # noqa: E731
    assert abs(mid(c["FX-TW-001"]["1"]) - (mid(d(0)) + 0.75)) < 0.4
    assert max(p[3] for p in c["FX-TW-001"]["1"]) > 0.99
    # Build From One Image, the error mix and the filter each change the picture.
    for fx in ("FX-TW-009", "FX-TW-010", "FX-TW-011", "FX-TW-012", "FX-TW-014"):
        assert c[fx]["2"] != c["FX-TW-001"]["2"], fx
    assert c["FX-TW-013"]["2"] != c["FX-TW-019"]["2"]
    assert near(c["FX-TW-018"]["2"], c["FX-TW-001"]["2"])
    assert c["FX-TW-015"]["2"] != c["FX-TW-001"]["2"]
    assert near(c["FX-TW-016"]["1"], [[(p[k] + 2 * q[k] + r[k]) / 4 for k in range(4)]
                                      for p, q, r in zip(d(1), d(2), d(3))])
    assert near(c["FX-TW-021"]["1"], [[p[0] / 2, p[1] / 2, p[2] / 2, p[3]]
                                      for p in c["FX-TW-001"]["1"]])
    assert c["FX-TW-020"]["1"] != c["FX-TW-001"]["1"]
    assert c["FX-TW-022"]["2"] == plain(case(start=2), 2)
    moved = c["FX-TW-023"]["1"]
    assert all(moved[(y + 2) * W + x + 3] == c["FX-TW-001"]["1"][y * W + x]
               for y in range(H - 2) for x in range(W - 3))
    assert c["FX-TW-024"]["2"] == d(2)
    assert near(c["FX-TW-025"]["1"], plain(case(spans=TWOS), 0))
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
