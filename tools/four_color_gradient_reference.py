"""4-Color Gradient, worked a second way.

D-208 adds `core.four_color_gradient`: four colours pinned to four points of the layer, each
spreading out from its point and running smoothly into the others, laid over each pixel that
shows, for a sky, a glow or a shimmer of colour across a cel. It is After Effects' 4-Color
Gradient in purpose, and this program's own rule. Nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

The rule. `point_1` to `point_4` are points in per cent of the drawing's own size, P1 to P4 in
its own space, the top-left corner (0, 0), and `color_1` to `color_4` their colours, each channel
its 8-bit value / 255. At a pixel with covering a > 0, X its centre, d_k = |X - P_k|. When some
d_k is 0, the points there share the pixel alike and the others have no say: w_k = 1 where d_k
is 0 and 0 elsewhere. Otherwise, with m the least d_k and e = 200 / blend, w_k = (m / d_k)^e:
blend 100 weighs each point by its distance squared, a small blend lets the nearest point all but
decide, and a large one mixes the four nearly evenly. The colour is mixed in encoded values, as
Gradient's is, G_enc = sum(w_k c_k) / sum(w_k), and G = srgb_to_linear(G_enc). With o = opacity
/ 100 and b the pixel's straight linear colour, f = G (normal), b * G (multiply), 1 - (1 - b)(1
- G) (screen) or b + G (add) by `blending_mode`, and out = ((b + o (f - b)) * a, a): every pixel
keeps its own covering, so the gradient never spills past the drawing, and pixels that do not
show stay as they are. Opacity 0 leaves the layer exactly as it is. Nothing grows, and a draft
changes nothing: the points are in per cent.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build writes single precision into its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Gradient's cel, a box of line filled with skin, its right part in shadow, a
half-covering edge down its left side and empty around it. The drawing goes into
`Fixtures/four_color_gradient/media`, the projects into `Fixtures/four_color_gradient`, and the
expected frames into `Fixtures/four_color_gradient/expected_four_color_gradient.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/four_color_gradient_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from gradient_reference import DRAWINGS, BLENDS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "four_color_gradient"
TOLERANCE = 2e-5  # document 25's default for a filter
POINTS = ("point_1", "point_2", "point_3", "point_4")
COLORS = ("color_1", "color_2", "color_3", "color_4")
RANGES = {**{k: (-1000, 1000) for k in POINTS}, "blend": (1, 1000), "opacity": (0, 100)}
WORDS = COLORS + ("blending_mode",)
NAMES = tuple(RANGES) + WORDS
YELLOW, GREEN, MAGENTA, BLUE = "#ffff00", "#00ff00", "#ff00ff", "#0000ff"


# --- the rule -------------------------------------------------------------------------------

def weights(n, X, Y):
    d = [math.hypot(X - n[k][0] / 100 * W, Y - n[k][1] / 100 * H) for k in POINTS]
    if min(d) == 0:
        return [1.0 if v == 0 else 0.0 for v in d]
    e = 200 / n["blend"]
    return [(min(d) / v) ** e for v in d]


def four_color_gradient(layer, n, colors, mode):
    o = n["opacity"] / 100
    if o == 0:
        return layer
    C = [[v / 255 for v in R.hex_color(c.lower())] for c in colors]
    mix = BLENDS[mode]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            a = p[3]
            if a <= 0:
                px.append(p)
                continue
            wk = weights(n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            G = [srgb_to_linear(sum(w * c[ch] for w, c in zip(wk, C)) / sum(wk))
                 for ch in range(3)]
            b = [p[ch] / a for ch in range(3)]
            px.append([(b[ch] + o * (mix(b[ch], G[ch]) - b[ch])) * a for ch in range(3)] + [a])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(point_1=(10, 10), point_2=(90, 10), point_3=(10, 90), point_4=(90, 90),
         color_1=YELLOW, color_2=GREEN, color_3=MAGENTA, color_4=BLUE, blend=100, opacity=100,
         blending_mode="normal", shift=0, tile=False):
    return {"drawing": "cel", "point_1": point_1, "point_2": point_2, "point_3": point_3,
            "point_4": point_4, "color_1": color_1, "color_2": color_2, "color_3": color_3,
            "color_4": color_4, "blend": blend, "opacity": opacity,
            "blending_mode": blending_mode, "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    return min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(four_color_gradient(layer_of(c), n, [c[k] for k in COLORS], c["blending_mode"]),
                 c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-4CG-001": ("The settings as they start: yellow at the top-left, (10, 10) per cent, green "
                   "at the top-right, (90, 10), magenta at the bottom-left, (10, 90), and blue at "
                   "the bottom-right, (90, 90), blend 100, opacity 100, normal: the cel is "
                   "painted over with the four colours running into each other, each strongest "
                   "at its own corner, the line and the shadow gone under it, every pixel at its "
                   "own covering, and the empty pixels empty.",
                   case(), [0]),
    "FX-4CG-002": ("Blend 1: each pixel all but wholly its nearest point's colour, four "
                   "quarters meeting at hard seams down the middle and across it.",
                   case(blend=1), [0]),
    "FX-4CG-003": ("Blend 1000: the four colours mixed nearly evenly, the whole cel close to one "
                   "colour.", case(blend=1000), [0]),
    "FX-4CG-004": ("Point 1 at (28.125, 25) per cent, the centre of the pixel in column 4 and "
                   "row 2: that pixel is yellow exactly.",
                   case(point_1=(28.125, 25)), [0]),
    "FX-4CG-005": ("Opacity 50: the colours laid on at half strength over the cel.",
                   case(opacity=50), [0]),
    "FX-4CG-006": ("Opacity 0: the drawing, untouched.", case(opacity=0), [0]),
    "FX-4CG-007": ("Blending mode multiply: the cel darkened and tinted by the colours, the "
                   "line still showing through.", case(blending_mode="multiply"), [0]),
    "FX-4CG-008": ("Blending mode screen: the cel lightened by the colours, none darkened.",
                   case(blending_mode="screen"), [0]),
    "FX-4CG-009": ("Blending mode add: the colours added to the cel.",
                   case(blending_mode="add"), [0]),
    "FX-4CG-010": ("Colours written in capitals, #FF0000, #00FF00, #0000FF and #FFFFFF: red, "
                   "green, blue and white.",
                   case(color_1="#FF0000", color_2="#00FF00", color_3="#0000FF",
                        color_4="#FFFFFF"), [0]),
    "FX-4CG-011": ("All four colours #6450a0: every pixel that shows is #6450a0 at its own "
                   "covering, wherever the points are.",
                   case(color_1="#6450a0", color_2="#6450a0", color_3="#6450a0",
                        color_4="#6450a0"), [0]),
    "FX-4CG-012": ("All four points at (50, 50): every pixel as far from each, so every pixel "
                   "is the four colours mixed evenly.",
                   case(point_1=(50, 50), point_2=(50, 50), point_3=(50, 50), point_4=(50, 50)),
                   [0]),
    "FX-4CG-013": ("Point 1 keyed from (10, 10) at frame 0 to (90, 90) at frame 4, linear: the "
                   "yellow slides across the cel; frame 0 is FX-4CG-001.",
                   case(point_1=keyed((0, (10, 10)), (4, (90, 90)))), [0, 2, 4]),
    "FX-4CG-014": ("Blend keyed from 100 at frame 0 to 1000 at frame 4, eased past its end: "
                   "frame 2 would pass 1000, is held at 1000, and is blend 1000.",
                   case(blend=keyed((0, 100, OVERSHOOT), (4, 1000))), [0, 2]),
    "FX-4CG-015": ("FX-4CG-001 moved three pixels right: the colours move with the drawing.",
                   case(shift=3), [0]),
    "FX-4CG-016": ("After a Motion Tile that grows the layer: the points are the drawing's "
                   "own, so the frame is FX-4CG-001's.", case(tile=True), [0]),
    "FX-4CG-017": ("The points outside the drawing, at (-100, -100), (200, -100), (-100, 200) "
                   "and (200, 200): the colours mix more evenly across the cel than FX-4CG-001.",
                   case(point_1=(-100, -100), point_2=(200, -100), point_3=(-100, 200),
                        point_4=(200, 200)), [0]),
}

INVALID = {
    "FX-4CG-018": ("Blend 0, below 1.", case(blend=0)),
    "FX-4CG-019": ("Blend 1001, above 1000.", case(blend=1001)),
    "FX-4CG-020": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-4CG-021": ("Opacity -1, below 0.", case(opacity=-1)),
    "FX-4CG-022": ("Point 3 1001 per cent across, above 1000.", case(point_3=(1001, 90))),
    "FX-4CG-023": ("Colour 2 \"#12345\", not six hex digits.", case(color_2="#12345")),
    "FX-4CG-024": ("Colour 4 \"blue\", a word.", case(color_4="blue")),
    "FX-4CG-025": ("Blending mode \"darken\", which is not \"normal\", \"multiply\", "
                   "\"screen\" or \"add\".", case(blending_mode="darken")),
    "FX-4CG-026": ("Opacity keyed to 101 at frame 4, above 100.",
                   case(opacity=keyed((0, 100), (4, 101)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.four_color_gradient",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    comp["layers"][0]["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_four_color_gradient.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                           encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    lin = lambda h, a=1.0: [srgb_to_linear(v / 255) * a for v in R.hex_color(h)] + [a]  # noqa
    art = plain(case())
    empty = [i for i in range(W * H) if art[i][3] == 0]
    shows = [i for i in range(W * H) if art[i][3] > 0]
    assert empty and shows

    # Every covering kept, and the empty pixels left empty, in every case.
    for fx, frames in c.items():
        if fx in ("FX-4CG-015",):
            continue
        for px in frames.values():
            assert all(px[i][3] == art[i][3] for i in range(W * H)), fx
            assert all(px[i] == art[i] for i in empty), fx

    one = c["FX-4CG-001"]["0"]
    # Each corner nearest its own colour: the pixel at (2, 1) is mostly yellow, (13, 1) green,
    # (2, 8) magenta and (13, 8) blue.
    tl, tr, bl, br = one[at(2, 1)], one[at(13, 1)], one[at(2, 8)], one[at(13, 8)]
    assert tl[0] > tl[2] and tl[1] > tl[2]            # yellow: red and green over blue
    assert tr[1] > tr[0] and tr[1] > tr[2]            # green
    assert bl[0] > bl[1] and bl[2] > bl[1]            # magenta: red and blue over green
    assert br[2] > br[0] and br[2] > br[1]            # blue
    # Normal at full opacity: the colour is the gradient's whatever was under it, so the
    # half-covering edge in column 1 is its neighbour's colour at half covering, near enough.
    assert 0 < one[at(1, 4)][3] < 1
    # Blend 1: close to the nearest colour; blend 1000: close to the mix of all four.
    hard = c["FX-4CG-002"]["0"]
    assert near(hard[at(3, 2)], lin(YELLOW), 1e-3) and near(hard[at(12, 7)], lin(BLUE), 1e-3)
    even = c["FX-4CG-003"]["0"]
    spread = lambda px: max(px[i][k] / px[i][3] for i in shows for k in range(3)) - min(  # noqa
        px[i][k] / px[i][3] for i in shows for k in range(3))
    reds = lambda px: [px[i][0] / px[i][3] for i in shows]  # noqa: E731
    assert max(reds(even)) - min(reds(even)) < max(reds(one)) - min(reds(one))
    assert spread(hard) >= spread(one)
    dot = c["FX-4CG-004"]["0"]
    assert near(dot[at(4, 2)], lin(YELLOW), 1e-12) and dot[at(5, 2)] != lin(YELLOW)
    half = c["FX-4CG-005"]["0"]
    for i in shows:
        assert near(half[i], [(art[i][k] + one[i][k]) / 2 for k in range(3)] + [art[i][3]], 1e-12)
    assert c["FX-4CG-006"]["0"] == art
    mult, scr, add = (c[f"FX-4CG-00{k}"]["0"] for k in (7, 8, 9))
    for i in shows:
        assert all(mult[i][k] <= art[i][k] + 1e-12 for k in range(3))
        assert all(scr[i][k] >= art[i][k] - 1e-12 for k in range(3))
        assert all(add[i][k] >= art[i][k] - 1e-12 for k in range(3))
    assert c["FX-4CG-010"]["0"] != one
    flat = c["FX-4CG-011"]["0"]
    assert all(near(flat[i], lin("#6450a0", art[i][3]), 1e-12) for i in shows)
    same = c["FX-4CG-012"]["0"]
    mixed = [srgb_to_linear(sum(R.hex_color(h)[k] / 255 for h in (YELLOW, GREEN, MAGENTA, BLUE))
                            / 4) for k in range(3)]
    assert all(near(same[i], [v * art[i][3] for v in mixed] + [art[i][3]], 1e-12) for i in shows)
    thirteen = c["FX-4CG-013"]
    assert thirteen["0"] == one and thirteen["2"] != one and thirteen["4"] != thirteen["2"]
    fourteen = c["FX-4CG-014"]
    assert ease(OVERSHOOT, 0.5) > 1 and fourteen["0"] == one
    assert fourteen["2"] == render(case(blend=1000), 2)
    moved = c["FX-4CG-015"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-4CG-016"]["0"] == one
    far = c["FX-4CG-017"]["0"]
    assert max(reds(far)) - min(reds(far)) < max(reds(one)) - min(reds(one))
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
