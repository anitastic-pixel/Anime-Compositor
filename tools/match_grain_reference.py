"""Match Grain, worked a second way.

D-454 adds `core.match_grain`, modelled on After Effects' Match Grain (Noise & Grain), which
"matches the noise between two images": it reads the grain of a Noise Source Layer and adds grain
like it to the layer (Adobe's After Effects help, Match Grain and Compensate For Existing Noise).
Adobe publishes no formula, ranges or starting values, so the rule, ranges and starting values
below are this program's own, and nothing is ported. It is two published pieces put together:

1. The measure. Per channel c of the source's straight colour through the sRGB curve (each held
   in 0 and 1), Immerkaer's fast noise estimate (J. Immerkaer, "Fast Noise Variance Estimation",
   Computer Vision and Image Understanding 64(2):300-302, 1996, doi:10.1006/cviu.1996.0060): the
   picture is filtered with the 3 by 3 mask [[1, -2, 1], [-2, 4, -2], [1, -2, 1]], the difference
   of two Laplacians, which cancels flat areas and steady slopes and leaves the grain, and
       sigma_c = sqrt(pi / 2) / (6 N) * sum |filtered|
   over the N windows of 3 by 3 pixels whose nine pixels all show (covering above 0); no such
   window gives 0. For Gaussian grain of spread s on a flat picture the expected sigma is s.
2. The grain laid: D-443's Add Grain (tools/addgrain_reference.py) with every one of its
   settings, each channel's intensity multiplied by sigma_c 10 sqrt(3). Add Grain's grain at
   softness 0 is P0-19's value noise, even in -1 to 1, of spread 1 / sqrt(3), and its amount is
   0.1 intensity, so at intensity 1, softness 0, Add, the tonal weights 1, the added grain's
   spread in channel c is the measured sigma_c. Film (the starting blend, as Add Grain's) matches
   it in a channel's middle and less toward black and white.

The source is read whole each frame, as document 21's layer setting (D-189) gives it, its
drawing, masks and effects, not fitted to the layer (as Color Link, D-375). Settings: Add Grain's
seventeen (`intensity`, `size`, `softness`, `aspect_ratio`, `red_intensity`, `green_intensity`,
`blue_intensity`, `monochromatic`, `saturation`, `blending_mode`, `shadows`, `midtones`,
`highlights`, `midpoint`, `animation_speed`, `animate_smoothly`, `random_seed`), with its ranges
and starting values, and `layer`, a layer of this composition by id, "" when added, meaning no
source: nothing is measured and the layer is left as it is. A name that is not a layer of the
composition is EFFECT_LAYER_MISSING each frame, and the layer is left as it is. A source with no
grain (one colour) adds nothing.

Known differences, each in the row: Softness above 0 lays softer grain of smaller spread than
measured; a draft reads the smaller source picture, whose grain is weaker; the grain's size is
not measured (Size stays the user's). Not built, logged as gaps: the sample boxes and the
Sampling group, Compensate for Existing Noise, View Mode, Preview Region, Channel Size, Tint,
Channel Balance, Blend with Original.

**This file never runs the build's code path.** It measures with numpy in double precision from
the drawings' exact 8-bit values and lays the grain with tools/addgrain_reference.py's lists.

Every case is a composition 16 by 10, five frames, in `Fixtures/match_grain/`: the layer `holder`,
Noise's card (tools/noise_reference.py) at the top-left; `steady`, a hidden 12 by 8 grainy drawing;
`holes`, the same with one empty pixel and one half-covered column; `seq`, a hidden 12 by 8
drawing with light grain for frames 0 and 1 and heavy grain from frame 2; `flat`, a hidden 6 by 4
drawing of one colour. The expected pixels are in `Fixtures/match_grain/expected_match_grain.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/match_grain_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import addgrain_reference as G  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import recolor_reference as R  # noqa: E402
import smooth_reference as S  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from noise_reference import DRAWINGS as NOISE, u  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "match_grain"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = R.W, R.H, 5
MASK = np.array([[1, -2, 1], [-2, 4, -2], [1, -2, 1]], dtype=float)
SCALE = 10 * math.sqrt(3)


# --- the drawings ---------------------------------------------------------------------------

def d(x, y, k):
    """A fixed grain step in -2..2 for pixel (x, y), `k` choosing the pattern."""
    return ((x * 7 + y * 13 + x * y * 3 + k * 5) % 5) - 2


def grainy(k, amount, holes=False):
    def at(x, y):
        if holes and (x, y) == (4, 3):
            return (0, 0, 0, 0)
        s = d(x, y, k)
        return (120 + amount[0] * s, 128 + amount[1] * s, 136 + amount[2] * d(y, x, k),
                128 if holes and x == 11 else 255)
    return [[at(x, y) for x in range(12)] for y in range(8)]


DRAWINGS = {
    "card": NOISE["card"],
    "steady": grainy(0, (9, 5, 2)),
    "holes": grainy(0, (9, 5, 2), holes=True),
    "seq_1": grainy(1, (2, 2, 1)),
    "seq_2": grainy(1, (14, 10, 6)),
    "flat": [[(90, 140, 60, 255)] * 6 for _ in range(4)],
}
SEQ = [{"start_frame": 0, "end_frame_exclusive": 2, "drawing_number": 1},
       {"start_frame": 2, "end_frame_exclusive": FRAMES, "drawing_number": 2}]


# --- the rule -------------------------------------------------------------------------------

def levels(rows):
    """Immerkaer's sigma per channel of a drawing of 8-bit straight colours."""
    a = np.array([[p[3] for p in row] for row in rows], dtype=float)
    out = []
    for ch in range(3):
        img = np.array([[p[ch] / 255 for p in row] for row in rows], dtype=float)
        total, n = 0.0, 0
        for y in range(img.shape[0] - 2):
            for x in range(img.shape[1] - 2):
                if (a[y:y + 3, x:x + 3] > 0).all():
                    total += abs(float((img[y:y + 3, x:x + 3] * MASK).sum()))
                    n += 1
        out.append(math.sqrt(math.pi / 2) / (6 * n) * total if n else 0.0)
    return out


def source(c, n):
    return {"steady": "steady", "holes": "holes", "flat": "flat",
            "seq": "seq_1" if n < 2 else "seq_2"}.get(c["layer"])


def holder():
    return {"px": [R.working(p) for row in DRAWINGS["card"] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def render(c, n):
    name = source(c, n)
    if name is None:
        return frame(holder(), 0)
    sig = levels(DRAWINGS[name])
    nums = {k: G.held(c, k, n) for k in G.RANGES}
    for ch, k in enumerate(("red_intensity", "green_intensity", "blue_intensity")):
        nums[k] *= sig[ch] * SCALE
    return frame(G.add_grain(holder(), nums, {k: c[k] for k in G.WORDS}, n), 0)


# --- the cases ------------------------------------------------------------------------------

def case(layer="steady", **kw):
    c = G.case(**kw)
    c["layer"] = layer
    return c


ADD = {"blending_mode": "add"}

CASES = {
    "FX-MATCHGRAIN-001": ("The settings as added: no source layer, so nothing is measured and the "
                          "card is left as it is.", case(layer=""), (0,)),
    "FX-MATCHGRAIN-002": ("Source `steady`, hidden, the rest as added (Film): the card gains grain "
                          "as strong as steady's, its red grain strongest and blue weakest, a new "
                          "grain each frame.", case(), (0, 2)),
    "FX-MATCHGRAIN-003": ("Source `seq`: light grain at frame 0, heavy grain at frame 3 when the "
                          "source's drawing changes.", case(layer="seq"), (0, 3)),
    "FX-MATCHGRAIN-004": ("Blending Mode Add: the grain's spread per channel is steady's.",
                          case(**ADD), (0,)),
    "FX-MATCHGRAIN-005": ("Intensity 2, Add: twice FX-MATCHGRAIN-004's grain.",
                          case(intensity=2, **ADD), (0,)),
    "FX-MATCHGRAIN-006": ("Monochromatic, Add: one grain pattern in all three channels, each at "
                          "its measured strength.", case(monochromatic="on", **ADD), (0,)),
    "FX-MATCHGRAIN-007": ("Channel intensities 0, 1 and 2, Add: no red grain, green as measured, "
                          "blue doubled.", case(red_intensity=0, green_intensity=1,
                                                blue_intensity=2, **ADD), (0,)),
    "FX-MATCHGRAIN-008": ("Saturation 0, Add.", case(saturation=0, **ADD), (0,)),
    "FX-MATCHGRAIN-009": ("Size 3, Softness 0.5, Aspect Ratio 2: bigger, softer, wider grains.",
                          case(size=3, softness=0.5, aspect_ratio=2), (0,)),
    "FX-MATCHGRAIN-010": ("Shadows 0, Highlights 0, Add: no grain in the card's black and white "
                          "patches.", case(shadows=0, highlights=0, **ADD), (0,)),
    "FX-MATCHGRAIN-011": ("Blending Mode Overlay, intensity 2.",
                          case(intensity=2, blending_mode="overlay"), (0,)),
    "FX-MATCHGRAIN-012": ("Animation Speed 0: the same grain on frames 0, 2 and 4.",
                          case(animation_speed=0), (0, 2, 4)),
    "FX-MATCHGRAIN-013": ("Source `holes`, with one empty pixel and a half-covered column: the "
                          "windows over the empty pixel are not counted.",
                          case(layer="holes", **ADD), (0,)),
    "FX-MATCHGRAIN-014": ("Source `flat`, one colour: no grain measured, the card as it is.",
                          case(layer="flat"), (0, 2)),
    "FX-MATCHGRAIN-015": ("Source `ghost`, not a layer of the composition: the card as it is, with "
                          "EFFECT_LAYER_MISSING each frame.", case(layer="ghost"), (0, 3)),
    "FX-MATCHGRAIN-016": ("Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame "
                          "0 the card; frame 4's grain twice frame 2's.",
                          case(intensity=keyed((0, 0), (4, 4)), animation_speed=0, **ADD),
                          (0, 2, 4)),
    "FX-MATCHGRAIN-017": ("Random Seed 7: a different grain from FX-MATCHGRAIN-002's.",
                          case(random_seed=7), (0,)),
    "FX-MATCHGRAIN-018": ("Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's "
                          "grain.", case(animation_speed=0.5, animate_smoothly="off"), (0, 1, 3)),
}

INVALID = {
    "FX-MATCHGRAIN-019": ("Intensity 11, above 10.", case(intensity=11)),
    "FX-MATCHGRAIN-020": ("Size 0.05, below 0.1.", case(size=0.05)),
    "FX-MATCHGRAIN-021": ("Blending Mode \"screen\", not one of its three words.",
                          case(blending_mode="screen")),
    "FX-MATCHGRAIN-022": ("Monochromatic \"yes\", not off or on.", case(monochromatic="yes")),
    "FX-MATCHGRAIN-023": ("Source layer the number 5, not a layer's id.", case(layer=5)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    held_layer = L.raster("holder", "asset-card", out_frame=FRAMES)
    params = {k: (c[k] if k in G.WORDS else setting_json(c[k])) for k in G.NAMES}
    params["layer"] = c["layer"]
    held_layer["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.match_grain",
                              "enabled": True, "parameters": params}]
    hidden = dict(out_frame=FRAMES, enabled=False)
    layers = [held_layer, L.raster("steady", "asset-steady", **hidden),
              L.raster("holes", "asset-holes", **hidden),
              L.raster("seq", "asset-seq", spans=SEQ, **hidden),
              L.raster("flat", "asset-flat", **hidden)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(k, f"media/{k}.png") for k in ("card", "steady", "holes", "flat")]
            + [{"id": "asset-seq", "kind": "image_sequence", "name": "seq",
                "pattern": "seq_####.png", "frames": {str(k): f"media/seq_{k}.png" for k in (1, 2)},
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
    before = frame(holder(), 0)
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in render(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["layer"] == "ghost":
            expected["cases"][fx]["warning"] = "EFFECT_LAYER_MISSING"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    expected["levels"] = {k: levels(DRAWINGS[k]) for k in ("steady", "holes", "seq_1", "seq_2",
                                                          "flat")}
    (OUT / "expected_match_grain.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    lv = expected["levels"]
    drawn = frame(holder(), 0)
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    step = lambda f, i: [a - b for a, b in zip(enc(f[i]), enc(drawn[i]))]  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g: all(near(f[i], g[i]) for i in range(W * H))  # noqa: E731
    free = lambda f, i: [ch for ch in range(3) if 1e-9 < enc(f[i])[ch] < 1 - 1e-9]  # noqa: E731
    spread = lambda f, ch: sum(abs(step(f, i)[ch]) for i in shown)  # noqa: E731

    # The measure: Gaussian grain of spread 0.05 on a flat grey reads within 5 per cent of 0.05,
    # and a steady slope reads 0.
    rng = np.random.default_rng(1)
    noisy = np.clip(0.5 + rng.normal(0, 0.05, (200, 200)), 0, 1) * 255
    sig = levels([[(v, v, v, 255) for v in row] for row in noisy])
    assert all(abs(s - 0.05) < 0.0025 for s in sig), sig
    slope = levels([[(x * 3 + y * 5, 7 * x, 2 * y, 255) for x in range(20)] for y in range(9)])
    assert max(slope) < 1e-12, slope
    # The mapping: block grain's spread is 1 / sqrt(3), so 0.1 SCALE sigma spreads as sigma.
    us = [u(0, x, y, 0, 0) for x in range(300) for y in range(300)]
    assert abs(float(np.std(us)) * 0.1 * SCALE - 1) < 0.01
    # The drawings read as built: red grain over green over blue; seq's second drawing heavier.
    assert lv["steady"][0] > lv["steady"][1] > lv["steady"][2] > 0
    assert all(a > 3 * b for a, b in zip(lv["seq_2"], lv["seq_1"]))
    assert lv["flat"] == [0.0, 0.0, 0.0] and lv["holes"] != lv["steady"]

    for fx in ("FX-MATCHGRAIN-001", "FX-MATCHGRAIN-014", "FX-MATCHGRAIN-015"):
        assert all(same(f, drawn) for f in c[fx].values()), fx
    two = c["FX-MATCHGRAIN-002"]
    assert not same(two["0"], two["2"]) and not same(two["0"], drawn)
    three = c["FX-MATCHGRAIN-003"]
    assert spread(three["3"], 0) > 3 * spread(three["0"], 0)
    four, five = c["FX-MATCHGRAIN-004"]["0"], c["FX-MATCHGRAIN-005"]["0"]
    for i in shown:
        for ch in range(3):
            if ch in free(four, i) and ch in free(five, i):
                assert abs(step(five, i)[ch] - 2 * step(four, i)[ch]) < 1e-9
    seven = c["FX-MATCHGRAIN-007"]["0"]
    for i in shown:
        assert abs(step(seven, i)[0]) < 1e-12
    assert spread(seven, 2) > spread(four, 2) * 1.5
    ten = c["FX-MATCHGRAIN-010"]["0"]
    for x in (5, 6, 9, 10):
        for y in (3, 4):
            assert near(ten[y * W + x], drawn[y * W + x], 1e-12)
    twelve = c["FX-MATCHGRAIN-012"]
    assert twelve["0"] == twelve["2"] == twelve["4"]
    k16 = c["FX-MATCHGRAIN-016"]
    assert same(k16["0"], drawn)
    for i in shown:
        for ch in free(k16["4"], i):
            assert abs(step(k16["4"], i)[ch] - 2 * step(k16["2"], i)[ch]) < 1e-9
    assert not same(c["FX-MATCHGRAIN-017"]["0"], two["0"])
    k18 = c["FX-MATCHGRAIN-018"]
    assert k18["0"] == k18["1"] and not same(k18["1"], k18["3"])
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), name
    print("checked")


if __name__ == "__main__":
    main()
