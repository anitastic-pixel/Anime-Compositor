"""Color Link, worked a second way.

`core.color_link`, modelled on After Effects' Color Link: the layer is tinted with one colour
read from a whole layer's picture, its own or another's, so it matches that layer as the other
one changes. Nothing is ported; Adobe does not publish its method, so the rule below is ours
(D-375). Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

The rule. Settings: `layer`, a layer of this composition by id, "" when added, meaning the layer
itself as the effects before this one left it; `sample`, "average", "median", "brightest",
"darkest", "max_rgb" or "min_rgb", "average"; `clip`, 0 to 49 per cent, 5; `stencil`, "off" or
"on", "off"; `opacity`, 0 to 100, 100; `blending_mode`, "normal", "multiply", "screen", "add",
"overlay" or "soft_light", "normal". Words are exact; clip and opacity keyable.

- The picture read: with a layer named, that layer's own picture as document 21's layer setting
  (D-189) gives it, its drawing, masks and effects, but whole, not fitted to the holder, so a
  bigger layer is read to its edges; with "" the holder's picture as the effects before this
  one left it. Its statistics are D-351's: the four histograms of the encoded straight colour
  (red, green, blue and brightness, each pixel adding its covering), and for each brightness bin
  the colour it holds. A picture where nothing shows has no statistics; a name that is not a
  layer of this composition is EFFECT_LAYER_MISSING each frame. Either way the layer is left as
  it is.
- The colour C, each channel 0 to 1, with kb and kw D-351's clip points: "average", per channel
  the mean bin over 255 of the bins kb to kw of its histogram clipped by `clip` each end;
  "median", per channel the lowest bin whose running total is above half, over 255; "max_rgb"
  and "min_rgb", per channel kw and kb over 255, clipped by `clip`; "brightest" and "darkest",
  the colour held by the brightness bin kw and kb, clipped by `clip`. Clip below 50 per cent
  leaves kb at or below kw.
- f = mix(e, C) per channel by the blending mode (Paraffin's six, D-185, as the W3C writes
  them), o = opacity / 100, e the pixel's straight colour through the sRGB curve held inside 0
  to 1, a its covering. Stencil "on": at a > 0, e + o (f - e), at its own covering. Stencil
  "off": the colour is laid over the whole layer, the empty pixels too, as one layer of colour C
  at opacity o over the pixel: covering a' = o + a (1 - o), colour
  (o (1 - a) C + o a f + (1 - o) a e) / a'. Either result held inside 0 to 1, back to linear at
  its covering. Opacity 0 leaves the layer exactly as it is.

**This file never runs the build's code path.** It counts with numpy's bincount and finds the
clip points with cumulative sums (tools/auto_tone_reference.py, which asserts no value lies
within a thousandth of a bin's edge and no running total within a millionth of its clip), in
double precision from the drawings' exact 8-bit values, where the build counts in its own loop
from its single-precision buffers.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/color_link/`: the
layer `holder`, Broadcast Safe's colours drawing (tools/broadcast_safe_reference.py), with an
empty column and a half-covered one; `swatch`, a hidden 6 by 4 drawing, warm for frames 0 and 1
and cold from frame 2; `big`, a hidden 20 by 12 drawing, dark grey inside a magenta border two
pixels wide. The expected pixels are in `Fixtures/color_link/expected_color_link.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_link_reference.py
"""

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import auto_tone_reference as A  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import effect_layer_reference as L  # noqa: E402
from smooth_reference import linear_to_srgb  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_link"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = B.W, B.H, 5
NUMBERS = {"clip": (0, 49), "opacity": (0, 100)}
MIX = {
    "normal": lambda b, c: c,
    "multiply": lambda b, c: b * c,
    "screen": lambda b, c: 1 - (1 - b) * (1 - c),
    "add": lambda b, c: b + c,
    "overlay": lambda b, c: 2 * b * c if b <= 0.5 else 1 - 2 * (1 - b) * (1 - c),
    "soft_light": lambda b, c: b - (1 - 2 * c) * b * (1 - b) if c <= 0.5 else
    b + (2 * c - 1) * ((((16 * b - 12) * b + 4) * b if b <= 0.25 else b ** 0.5) - b),
}


# --- the drawings ---------------------------------------------------------------------------

def warm(x, y):
    if (x, y) == (5, 3):
        return (0, 0, 0, 0)
    return (150 + 15 * x + 4 * y, 60 + 9 * x + 12 * y, 30 + 5 * x + 3 * y,
            128 if (x, y) == (0, 0) else 255)


def cold(x, y):
    return (30 + 5 * x + 6 * y, 80 + 10 * x + 5 * y, 170 + 12 * x + 8 * y, 255)


def framed(x, y):
    edge = x < 2 or y < 2 or x >= 18 or y >= 10
    return (230, 40, 200, 255) if edge else (64 + x, 60 + y, 60, 255)


DRAWINGS = {
    "colours": B.DRAWINGS["colours"],
    "swatch_1": [[warm(x, y) for x in range(6)] for y in range(4)],
    "swatch_2": [[cold(x, y) for x in range(6)] for y in range(4)],
    "big": [[framed(x, y) for x in range(20)] for y in range(12)],
}
SWATCH = [{"start_frame": 0, "end_frame_exclusive": 2, "drawing_number": 1},
          {"start_frame": 2, "end_frame_exclusive": FRAMES, "drawing_number": 2}]


def picture(name):
    """A drawing's showing pixels, straight encoded colours and coverings."""
    px = [p for row in DRAWINGS[name] for p in row if p[3] > 0]
    return np.array([[v / 255 for v in p[:3]] for p in px]), np.array([p[3] / 255 for p in px])


def read_picture(c, n):
    if c["layer"] == "":
        return picture("colours")
    if c["layer"] == "swatch":
        return picture("swatch_1" if n < 2 else "swatch_2")
    if c["layer"] == "big":
        return picture("big")
    return None


# --- the rule -------------------------------------------------------------------------------

def colour(s, sample, clip):
    hist, sums = s["hist"], s["sums"]
    if sample == "median":
        return np.array([A.clip_points(hist[ch], 50, 50)[0] / 255 for ch in range(3)])
    if sample in ("brightest", "darkest"):
        kb, kw = A.clip_points(hist[3], clip, clip)
        k = kw if sample == "brightest" else kb
        return sums[:, k] / hist[3][k]
    out = []
    for ch in range(3):
        kb, kw = A.clip_points(hist[ch], clip, clip)
        if sample == "max_rgb":
            out.append(kw / 255)
        elif sample == "min_rgb":
            out.append(kb / 255)
        else:
            k = np.arange(kb, kw + 1)
            out.append(float((hist[ch][kb:kw + 1] * k).sum() / hist[ch][kb:kw + 1].sum() / 255))
    return np.array(out)


def link(c, n):
    """The composition's pixels at frame n: the holder sits at (0, 0), the size of it."""
    px = B.pixels()
    s = (min(49, max(0, value_at(c["clip"], n))), min(100, max(0, value_at(c["opacity"], n))))
    pic = read_picture(c, n)
    st = None if pic is None else A.stats(pic)
    o = s[1] / 100
    if st is None or o == 0:
        return [B.back(B.encoded(p), p) if p[3] else [0.0] * 4 for p in px]
    C = colour(st, c["sample"], s[0])
    mix = MIX[c["blending_mode"]]
    out = []
    for p in px:
        a, e = p[3] / 255, B.encoded(p)
        f = [mix(b, k) for b, k in zip(e, C)]
        if c["stencil"] == "on":
            if a == 0:
                out.append([0.0] * 4)
                continue
            r, a2 = [b + o * (g - b) for b, g in zip(e, f)], a
        else:
            a2 = o + a * (1 - o)
            r = [(o * (1 - a) * k + o * a * g + (1 - o) * a * b) / a2 for b, g, k in zip(e, f, C)]
        out.append([A.srgb_to_linear(min(1.0, max(0.0, v))) * a2 for v in r] + [a2])
    return out


def plain():
    return [B.back(B.encoded(p), p) if p[3] else [0.0] * 4 for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(layer="", sample="average", clip=5, stencil="off", opacity=100, blending_mode="normal"):
    return {"layer": layer, "sample": sample, "clip": clip, "stencil": stencil,
            "opacity": opacity, "blending_mode": blending_mode}


SW = {"layer": "swatch"}
HALF = {"layer": "swatch", "opacity": 50, "stencil": "on"}

CASES = {
    "FX-CLINK-001": ("The settings as added: the layer's own picture, average, clip 5, stencil "
                     "off, opacity 100, normal: the whole layer, its empty column too, one "
                     "colour, the clipped average of its own colours.", case(), (0,)),
    "FX-CLINK-002": ("Source layer `swatch`, hidden: the layer turns the swatch's average "
                     "colour, warm at frame 0 and cold at frame 3 when the swatch's drawing "
                     "changes.", case(**SW), (0, 3)),
    "FX-CLINK-003": ("Sample median.", case(sample="median", **SW), (0,)),
    "FX-CLINK-004": ("Sample brightest: the colour of the swatch's brightest pixels, the top 5 "
                     "per cent clipped.", case(sample="brightest", **SW), (0,)),
    "FX-CLINK-005": ("Sample darkest.", case(sample="darkest", **SW), (0,)),
    "FX-CLINK-006": ("Sample max RGB: each channel's own highest, 5 per cent clipped.",
                     case(sample="max_rgb", **SW), (0,)),
    "FX-CLINK-007": ("Sample min RGB.", case(sample="min_rgb", **SW), (0,)),
    "FX-CLINK-008": ("Average with clip 0: every pixel of the swatch counted.",
                     case(clip=0, **SW), (0,)),
    "FX-CLINK-009": ("Stencil on: the colour only where the layer shows; the empty column stays "
                     "empty and the half-covered one stays half covered.",
                     case(stencil="on", **SW), (0,)),
    "FX-CLINK-010": ("Opacity 50, stencil on: halfway from the drawing to the colour.",
                     case(**HALF), (0,)),
    "FX-CLINK-011": ("Opacity 50, stencil off: the drawing halfway to the colour, and the empty "
                     "column the colour at half covering.", case(opacity=50, **SW), (0,)),
    "FX-CLINK-012": ("Blending mode multiply, opacity 50, stencil on.",
                     case(blending_mode="multiply", **HALF), (0,)),
    "FX-CLINK-013": ("Blending mode screen, opacity 50, stencil on.",
                     case(blending_mode="screen", **HALF), (0,)),
    "FX-CLINK-014": ("Blending mode add, opacity 50, stencil on.",
                     case(blending_mode="add", **HALF), (0,)),
    "FX-CLINK-015": ("Blending mode overlay, opacity 100, stencil on: the drawing keeps its "
                     "light and dark, coloured by the swatch.",
                     case(blending_mode="overlay", stencil="on", **SW), (0,)),
    "FX-CLINK-016": ("Blending mode soft light, opacity 100, stencil off: the empty column the "
                     "plain colour, the rest softly tinted.",
                     case(blending_mode="soft_light", **SW), (0,)),
    "FX-CLINK-017": ("Opacity keyed from 0 at frame 0 to 100 at frame 4, linear, stencil on: "
                     "frame 0 untouched, frame 2 halfway, frame 4 the swatch's cold colour.",
                     case(opacity=keyed((0, 0), (4, 100)), stencil="on", **SW), (0, 2, 4)),
    "FX-CLINK-018": ("Source layer `big`, 20 by 12, bigger than the holder: read whole, so its "
                     "magenta border counts and pulls the average toward magenta.",
                     case(layer="big", clip=0), (0,)),
    "FX-CLINK-019": ("Source layer `ghost`, not a layer of the composition: the layer as it is, "
                     "with EFFECT_LAYER_MISSING each frame.", case(layer="ghost"), (0, 3)),
}

INVALID = {
    "FX-CLINK-020": ("Clip 50, above 49.", case(clip=50, **SW)),
    "FX-CLINK-021": ("Opacity -1, below 0.", case(opacity=-1, **SW)),
    "FX-CLINK-022": ("Sample \"mean\", which is not a choice.", case(sample="mean", **SW)),
    "FX-CLINK-023": ("Stencil \"yes\", which is not a choice.", case(stencil="yes", **SW)),
    "FX-CLINK-024": ("Blending mode \"color_dodge\", which is not a choice.",
                     case(blending_mode="color_dodge", **SW)),
    "FX-CLINK-025": ("Blending mode \"Normal\": the word is exact, so a capital is not it.",
                     case(blending_mode="Normal", **SW)),
}


def project_json(fx, c):
    holder = L.raster("holder", "asset-colours", out_frame=FRAMES)
    holder["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.color_link", "enabled": True,
                          "parameters": {k: setting_json(c[k]) if k in NUMBERS else c[k]
                                         for k in c}}]
    layers = [holder,
              L.raster("swatch", "asset-swatch", spans=SWATCH, out_frame=FRAMES, enabled=False),
              L.raster("big", "asset-big", out_frame=FRAMES, enabled=False)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still("colours", "media/colours.png"), L.still("big", "media/big.png"),
                       {"id": "asset-swatch", "kind": "image_sequence", "name": "swatch",
                        "pattern": "swatch_####.png",
                        "frames": {str(k): f"media/swatch_{k}.png" for k in (1, 2)},
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
    before = plain()
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in link(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["layer"] == "ghost":
            expected["cases"][fx]["warning"] = "EFFECT_LAYER_MISSING"  # D-189, on opening too (D-375)
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
    (OUT / "expected_color_link.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain()
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    enc = lambda p: [linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731
    one = lambda f: all(near(enc(p), enc(f[1])) and p[3] == 1 for p in f)  # noqa: E731

    # Stencil off at full opacity, normal: one colour everywhere, fully covered.
    for fx in ("FX-CLINK-001", "FX-CLINK-002", "FX-CLINK-003", "FX-CLINK-008", "FX-CLINK-018"):
        assert one(c[fx]["0"]), fx
    warm_c, cold_c = enc(c["FX-CLINK-002"]["0"][0]), enc(c["FX-CLINK-002"]["3"][0])
    assert warm_c[0] > warm_c[2] and cold_c[2] > cold_c[0]
    # The six samples differ, and are ordered as their names say.
    sw = A.stats(picture("swatch_1"))
    got = {s: colour(sw, s, 5) for s in ("average", "median", "brightest", "darkest",
                                         "max_rgb", "min_rgb")}
    assert len({tuple(np.round(v, 9)) for v in got.values()}) == 6
    assert all(got["min_rgb"] < got["average"]) and all(got["average"] < got["max_rgb"])
    y = lambda v: float(v @ A.LUMA)  # noqa: E731
    assert y(got["darkest"]) < y(got["average"]) < y(got["brightest"])
    assert near(enc(c["FX-CLINK-006"]["0"][0]), got["max_rgb"])
    # Clip 0 against clip 5: the average moves.
    assert not near(enc(c["FX-CLINK-008"]["0"][0]), warm_c)
    # Stencil on: coverings kept; the empty column empty.
    nine = c["FX-CLINK-009"]["0"]
    assert all(p[3] == q[3] for p, q in zip(nine, drawn))
    assert all(nine[y * W] == [0.0] * 4 for y in range(H))
    # Opacity 50 halfway in display values on a showing pixel; off, the empty column half there.
    ten, eleven = c["FX-CLINK-010"]["0"], c["FX-CLINK-011"]["0"]
    e = B.encoded(px[8])
    assert near(enc(ten[8]), [(u + v) / 2 for u, v in zip(e, warm_c)])
    assert near(enc(eleven[8]), enc(ten[8]))
    assert abs(eleven[0][3] - 0.5) < 1e-12 and near(enc(eleven[0]), warm_c)
    # Modes: multiply darkens and screen lightens every showing pixel; overlay keeps the order.
    for i, p in enumerate(px):
        if p[3]:
            m, s = enc(c["FX-CLINK-012"]["0"][i]), enc(c["FX-CLINK-013"]["0"][i])
            assert all(u <= v + 1e-12 for u, v in zip(m, B.encoded(p)))
            assert all(u >= v - 1e-12 for u, v in zip(s, B.encoded(p)))
    sixteen = c["FX-CLINK-016"]["0"]
    assert near(enc(sixteen[0]), warm_c) and sixteen[0][3] == 1
    # Keyed opacity: untouched at 0, the cold colour at 4 on the stencil.
    seventeen = c["FX-CLINK-017"]
    assert like(seventeen["0"], drawn)
    assert near(enc(seventeen["4"][8]), cold_c)
    # The whole of `big` is read: its average sits between its grey and its magenta.
    big = enc(c["FX-CLINK-018"]["0"][0])
    assert big[0] > big[1] + 0.1 and big[2] > big[1] + 0.1
    assert like(c["FX-CLINK-019"]["0"], drawn) and like(c["FX-CLINK-019"]["3"], drawn)
    for fx, v in c.items():
        for f in v.values():
            for p in f:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checked")


if __name__ == "__main__":
    main()
