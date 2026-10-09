"""Color Stabilizer, worked a second way.

`core.color_stabilizer`, modelled on After Effects' Color Stabilizer: the colours at one, two or
three points of a reference frame are kept steady through a flickering shot, each frame's
colours mapped so its samples at those points come back to the reference frame's. Nothing is
ported; Adobe does not publish its method, so the rule below is ours (D-376). Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. Settings: `stabilize`, "brightness", "levels" or "curves", "brightness" when added;
`reference_frame`, the composition frame whose colours are kept, 0 to 1000000, 0, a whole number
taken down (After Effects' Set Frame button sets it to the frame shown); `black_point`,
`mid_point` and `white_point`, points in per cent of the layer's width and height, (25, 50),
(50, 50) and (75, 50), keyable; `sample_size`, the radius of each sample in pixels, 0 to 100, 5,
keyable.

- A point (u, v) per cent is at (u W / 100, v H / 100) in the layer's W by H picture. Its sample
  is the mean of the encoded straight colour, each pixel weighted by its covering, of the pixels
  whose centres (x + 1/2, y + 1/2) lie within the radius of it; when none does, the one pixel
  holding the point (its column and row taken down, held inside the picture). A sample with no
  covering is none.
- The reference samples are taken from the layer's picture at the reference frame with the
  effects before this one, at the points and sample size the settings have at that frame; the
  current samples from this frame's, at this frame's settings. A missing sample, or a reference
  frame outside the layer, leaves the frame as it is.
- "brightness": with Y = 0.2126 R + 0.7152 G + 0.0722 B, every channel moves by Y(reference
  black) - Y(current black). "levels": each channel is mapped by the straight line through the
  pairs (current, reference) of the black and white samples; "curves", through the three pairs
  of black, mid and white, a line joining each pair to the next in order of the current value
  and the outer lines carried on past the ends. A pair whose current value is within a millionth
  of the one before it is left out; with one pair left, the channel moves by reference less
  current. Each result is held inside 0 to 1 and comes back to linear at the pixel's own
  covering (document 21's shared colour rule); a pixel that does not show is left as it is.
- On an adjustment layer the frames beneath at other times are not to hand: there is no
  reference, the frame is left as it is, with TEMPORAL_SMOOTHING_SKIPPED each frame.

**This file never runs the build's code path.** It works in double precision from the drawings'
exact 8-bit values, finding the pixels within each radius by testing every pixel, and asserts no
pixel's centre lies within a millionth of a pixel of a radius, where the build works in single
precision on its buffers.

Every case is a project of one composition 16 by 10, six frames, in `Fixtures/color_stabilizer/`:
the layer `holder`, a run of drawings of a warm grey ramp with an empty column 0 and a
half-covered column 15: the ramp itself (frames 0 and 4), lifted by 20 (frame 1), with a colour
cast and changed contrast (frames 2 and 5), and with its mid-tones raised (frame 3). The expected
pixels are in `Fixtures/color_stabilizer/expected_color_stabilizer.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_stabilizer_reference.py
"""

import json
import sys
from math import floor, hypot
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import auto_tone_reference as A  # noqa: E402
import effect_layer_reference as L  # noqa: E402
from smooth_reference import linear_to_srgb  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_stabilizer"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 6
LUMA = (0.2126, 0.7152, 0.0722)


# --- the drawings ---------------------------------------------------------------------------

def ramp(x, y):
    g = 40 + 11 * (x - 1) + 3 * y
    return (g + 6, g, g - 8)


def cell(x, y, change):
    if x == 0:
        return (0, 0, 0, 0)
    rgb = tuple(max(0, min(255, int(change(c, v) + 0.5))) for c, v in enumerate(ramp(x, y)))
    return rgb + ((128,) if x == 15 else (255,))


CHANGES = {
    "flicker_1": lambda c, v: v,
    "flicker_2": lambda c, v: v + 20,
    "flicker_3": lambda c, v: (0.9 * v + 10, 1.05 * v - 5, 0.8 * v + 20)[c],
    "flicker_4": lambda c, v: 255 * (v / 255) ** 0.8,
}
DRAWINGS = {k: [[cell(x, y, f) for x in range(W)] for y in range(H)] for k, f in CHANGES.items()}
RUN = [1, 2, 3, 4, 1, 3]
SPANS = [{"start_frame": f, "end_frame_exclusive": f + 1, "drawing_number": d}
         for f, d in enumerate(RUN)]


def drawing(n):
    """The holder's pixels at frame n, (encoded straight colour, covering), or None outside."""
    if not 0 <= n < FRAMES:
        return None
    return [([v / 255 for v in p[:3]], p[3] / 255)
            for row in DRAWINGS[f"flicker_{RUN[n]}"] for p in row]


def before(c, pic):
    if pic is None or c["before"] is None:
        return pic
    f = A.levels_fn(c["before"])
    return [([float(v) for v in f(np.array(e))] if a > 0 else e, a) for e, a in pic]


# --- the rule -------------------------------------------------------------------------------

def sample(pic, point, r):
    px, py = point[0] * W / 100, point[1] * H / 100
    total, sums = 0.0, [0.0] * 3
    found = False
    for i, (e, a) in enumerate(pic):
        x, y = i % W, i // W
        d = hypot(x + 0.5 - px, y + 0.5 - py)
        assert abs(d - r) > 1e-6, "a pixel's centre lies on the radius"
        if d <= r:
            found = True
            total += a
            sums = [s + a * v for s, v in zip(sums, e)]
    if not found:
        x, y = min(W - 1, max(0, floor(px))), min(H - 1, max(0, floor(py)))
        e, a = pic[y * W + x]
        total, sums = a, [a * v for v in e]
    return None if total <= 0 else [s / total for s in sums]


def settings(c, n):
    return {k: value_at(c[k], n) for k in ("black_point", "mid_point", "white_point",
                                           "sample_size")}


def samples(c, n, at):
    pic = before(c, drawing(n))
    if pic is None:
        return None
    s = settings(c, at)
    r = min(100, max(0, s["sample_size"]))
    names = {"brightness": ("black_point",), "levels": ("black_point", "white_point"),
             "curves": ("black_point", "mid_point", "white_point")}[c["stabilize"]]
    got = [sample(pic, s[k], r) for k in names]
    return None if any(g is None for g in got) else got


def mapping(cur, ref, ch):
    pairs = sorted((c[ch], r[ch]) for c, r in zip(cur, ref))
    kept = [pairs[0]]
    for p in pairs[1:]:
        if p[0] - kept[-1][0] > 1e-6:
            kept.append(p)
        else:
            assert p[0] - kept[-1][0] < 1e-7, "two samples lie on the millionth"
    if len(kept) == 1:
        return lambda v: v + kept[0][1] - kept[0][0]

    def f(v):
        k = 0
        while k < len(kept) - 2 and v > kept[k + 1][0]:
            k += 1
        (x0, y0), (x1, y1) = kept[k], kept[k + 1]
        return y0 + (v - x0) * (y1 - y0) / (x1 - x0)
    return f


def stabilize(c, n):
    pic = before(c, drawing(n))
    ref = None if c["on"] == "adjust" else samples(c, floor(c["reference_frame"]),
                                                    floor(c["reference_frame"]))
    cur = samples(c, n, n)
    out = []
    if ref is None or cur is None:
        maps = None
    elif c["stabilize"] == "brightness":
        d = sum(w * (r - u) for w, r, u in zip(LUMA, ref[0], cur[0]))
        maps = [lambda v, d=d: v + d] * 3
    else:
        maps = [mapping(cur, ref, ch) for ch in range(3)]
    for e, a in pic:
        if a <= 0:
            out.append([0.0] * 4)
            continue
        got = e if maps is None else [m(v) for m, v in zip(maps, e)]
        out.append([A.srgb_to_linear(min(1.0, max(0.0, v))) * a for v in got] + [a])
    return out


def plain(c, n):
    return [[A.srgb_to_linear(v) * a for v in e] + [a] if a > 0 else [0.0] * 4
            for e, a in before(c, drawing(n))]


# --- the cases ------------------------------------------------------------------------------

def case(stabilize="brightness", reference_frame=0, black_point=(25, 50), mid_point=(50, 50),
         white_point=(75, 50), sample_size=5, before=None, on="holder"):
    return {"stabilize": stabilize, "reference_frame": reference_frame,
            "black_point": list(black_point) if isinstance(black_point, tuple) else black_point,
            "mid_point": list(mid_point),
            "white_point": list(white_point) if isinstance(white_point, tuple) else white_point,
            "sample_size": sample_size, "before": before, "on": on}


CASES = {
    "FX-CSTAB-001": ("The settings as added: brightness, reference frame 0, sample size 5. Frame "
                     "0 is the reference, so untouched; frame 1, the ramp lifted by 20, comes "
                     "back to frame 0; frames 2 and 3 move by their black point's brightness "
                     "only.", case(), (0, 1, 2, 3)),
    "FX-CSTAB-002": ("Levels: each channel mapped through the black and white samples, so frame "
                     "2's colour cast and contrast are taken out, to the drawing's own rounding.",
                     case(stabilize="levels"), (1, 2, 3)),
    "FX-CSTAB-003": ("Curves: through black, mid and white, so frame 3's raised mid-tones come "
                     "down too.", case(stabilize="curves"), (2, 3)),
    "FX-CSTAB-004": ("Levels with reference frame 2: frame 0, the plain ramp, is given frame 2's "
                     "cast.", case(stabilize="levels", reference_frame=2), (0, 2)),
    "FX-CSTAB-005": ("Reference frame 2.7, taken down to 2: the same as FX-CSTAB-004.",
                     case(stabilize="levels", reference_frame=2.7), (0,)),
    "FX-CSTAB-006": ("Sample size 0: each sample the one pixel holding its point.",
                     case(stabilize="levels", sample_size=0), (2,)),
    "FX-CSTAB-007": ("Sample size keyed from 1 at frame 0 to 9 at frame 4: the reference samples "
                     "taken at size 1 as frame 0 has it, the current ones at 5 at frame 2 and 7 "
                     "at frame 3.", case(stabilize="curves", sample_size=keyed((0, 1), (4, 9))),
                     (2, 3)),
    "FX-CSTAB-008": ("The black point at (10, 20) and the white point at (90, 80), levels: the "
                     "white sample reaches the half-covered column, counted at half weight.",
                     case(stabilize="levels", black_point=(10, 20), white_point=(90, 80)),
                     (2,)),
    "FX-CSTAB-009": ("The black point keyed from (25, 50) at frame 0 to (5, 50) at frame 4, "
                     "levels: the reference sample is taken where the point is at frame 0, the "
                     "current one where it is at frame 2.",
                     case(stabilize="levels", black_point=keyed((0, [25, 50]), (4, [5, 50]))),
                     (2,)),
    "FX-CSTAB-010": ("Reference frame 20, after the layer's last frame: no reference, so every "
                     "frame as it is.", case(stabilize="levels", reference_frame=20), (1, 2)),
    "FX-CSTAB-011": ("A Levels lowering output white to 200, then the stabilizer, levels: the "
                     "samples are of the darkened pictures, so frame 2 comes back to the "
                     "darkened frame 0.", case(stabilize="levels", before=200), (2,)),
    "FX-CSTAB-012": ("On an adjustment layer above the holder: no reference frame, so each frame "
                     "as it is, with a warning each frame.",
                     case(stabilize="levels", on="adjust"), (1, 2)),
}

INVALID = {
    "FX-CSTAB-013": ("Stabilize \"colour\", which is not a choice.", case(stabilize="colour")),
    "FX-CSTAB-014": ("Stabilize \"Levels\": the word is exact, so a capital is not it.",
                     case(stabilize="Levels")),
    "FX-CSTAB-015": ("Reference frame -1, below 0.", case(reference_frame=-1)),
    "FX-CSTAB-016": ("Sample size 101, above 100.", case(sample_size=101)),
}


def levels(fid, output_white):
    return {"instance_id": fid, "type_id": "core.levels", "enabled": True,
            "parameters": {"input_black": 0, "input_white": 255, "gamma": 1, "output_black": 0,
                           "output_white": output_white}}


def effect(fid, c):
    keys = ("stabilize", "reference_frame", "black_point", "mid_point", "white_point",
            "sample_size")
    return {"instance_id": fid, "type_id": "core.color_stabilizer", "enabled": True,
            "parameters": {k: setting_json(c[k]) if k not in ("stabilize", "reference_frame")
                           else c[k] for k in keys}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-flicker", spans=SPANS, out_frame=FRAMES)
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([levels("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-flicker", "kind": "image_sequence", "name": "flicker",
                        "pattern": "flicker_####.png",
                        "frames": {str(k): f"media/flicker_{k}.png" for k in range(1, 5)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in stabilize(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["on"] == "adjust":
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(c, 0), "4": plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_color_stabilizer.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda f, g, e: all(abs(u - v) <= e for p, q in zip(f, g)  # noqa: E731
                               for u, v in zip(p, q))
    enc = lambda f: [[linear_to_srgb(u / p[3]) for u in p[:3]] for p in f if p[3] > 0]  # noqa
    within = lambda f, g, e: near(enc(f), enc(g), e)  # noqa: E731
    p0 = plain(case(), 0)

    one = c["FX-CSTAB-001"]
    assert near(one["0"], p0, 1e-12)
    assert within(one["1"], p0, 1e-9)  # a lift of 20 taken out exactly
    assert not within(one["2"], p0, 2 / 255)  # brightness alone leaves the cast
    two = c["FX-CSTAB-002"]
    assert within(two["1"], p0, 1e-9) and within(two["2"], p0, 1.5 / 255)
    assert not within(two["3"], p0, 2 / 255)  # a curve is not a line
    three = c["FX-CSTAB-003"]
    err = lambda f: max(abs(u - v) for p, q in zip(enc(f), enc(p0))  # noqa: E731
                        for u, v in zip(p, q))
    assert err(three["3"]) < err(two["3"])
    p2 = plain(case(), 2)
    assert within(c["FX-CSTAB-004"]["0"], p2, 1.5 / 255) and near(c["FX-CSTAB-004"]["2"], p2,
                                                                     1e-12)
    assert near(c["FX-CSTAB-005"]["0"], c["FX-CSTAB-004"]["0"], 0)
    assert c["FX-CSTAB-006"]["2"] != two["2"]
    seven = case(stabilize="curves", sample_size=keyed((0, 1), (4, 9)))
    assert stabilize(seven, 2) != stabilize(case(stabilize="curves", sample_size=1), 2)
    assert c["FX-CSTAB-008"]["2"] != two["2"]
    assert c["FX-CSTAB-009"]["2"] != two["2"]
    for f in ("1", "2"):
        assert near(c["FX-CSTAB-010"][f], plain(case(), int(f)), 1e-12)
        assert near(c["FX-CSTAB-012"][f], plain(case(), int(f)), 1e-12)
    dark = case(before=200)
    assert within(c["FX-CSTAB-011"]["2"], plain(dark, 0), 1.5 / 255)
    for fx, v in c.items():
        for f in v.values():
            for p in f:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checked")


if __name__ == "__main__":
    main()
