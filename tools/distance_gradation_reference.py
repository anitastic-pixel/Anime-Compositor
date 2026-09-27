"""Distance Gradation, worked a second way.

D-123 adds `core.distance_gradation`: shading that fades in from the edge of a drawing's
covering, as a cel's shadow or a coloured rim is laid along its outline in anime finishing. Each
pixel that shows finds how far its centre is from the nearest pixel that is "out": a pixel of
the layer less than half covered, or any pixel past the layer's edge. At the edge the shading is
full; it fades to nothing `width` pixels in, and `invert` turns that round so the shading gathers
in the middle instead. The colour is mixed into each pixel that shows by `blend` at `opacity`
per cent of that strength, and every pixel keeps its own covering, so the shading never spills
past the drawing. It is this program's own method, modelled on OLM's Distance Gradation in
spirit (a distance from the covering's edge, turned into a strength); nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. `d(P)` is the Euclidean distance from pixel P's centre to the nearest centre of an
out pixel, a pixel of the buffer with a < 0.5 or any pixel outside the buffer (so a pixel that
is in on the buffer's edge has d = 1, and an out pixel has d = 0). With `width` 0 the output is
the input. Otherwise, at a pixel with a > 0: g = clamp(1 - (d - 0.5) / width, 0, 1); with invert
"on", g = 1 - g. With b the pixel's straight linear colour, G the colour's linear value and
op = g * opacity / 100: f = G (normal), b * G (multiply), 1 - (1 - b)(1 - G) (screen) or b + G
(add), not clamped, and the output is ((b + op (f - b)) * a, a). A pixel with a = 0 stays as it
is. The layer does not grow. `width` is a distance: a draft scales it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a block against the drawing's left edge, drawn below. The drawing goes into
`Fixtures/distance_gradation/media`, the projects into `Fixtures/distance_gradation`, and the
expected frames into `Fixtures/distance_gradation/expected_distance_gradation.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/distance_gradation_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "distance_gradation"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"width": (0, 1000), "opacity": (0, 100)}
NAMES = ("color", "width", "opacity", "invert", "blend")


# --- the rule -------------------------------------------------------------------------------

def distances(alphas, w, h):
    """d at every pixel, from 8-bit coverings (out is a < 0.5, that is 127 or less).

    Only the out pixels with an in pixel beside them (left, right, above or below) are tried:
    the nearest out pixel to an in pixel always has one, since its neighbour one step toward the
    in pixel is nearer, so it must be in. `check` tries every out pixel and gets the same."""
    out = [a < 128 for a in alphas]
    beside = [(i % w + 0.5, i // w + 0.5) for i in range(w * h) if out[i] and any(
        0 <= x < w and 0 <= y < h and not out[y * w + x]
        for x, y in ((i % w - 1, i // w), (i % w + 1, i // w), (i % w, i // w - 1),
                     (i % w, i // w + 1)))]
    d = []
    for i in range(w * h):
        if out[i]:
            d.append(0.0)
            continue
        x, y = i % w, i // w
        best = float(min(x + 1, w - x, y + 1, h - y))  # the nearest pixel past the buffer's edge
        px, py = x + 0.5, y + 0.5
        for qx, qy in beside:
            best = min(best, math.hypot(px - qx, py - qy))
        d.append(best)
    return d


def strength(d, width, invert):
    g = min(1.0, max(0.0, 1 - (d - 0.5) / width))
    return 1 - g if invert == "on" else g


def distance_gradation(pixels, color, width, opacity, invert, blend, w=W, h=H):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, a drawing w by h."""
    if width == 0:
        return [R.working(p) for p in pixels]
    d = distances([p[3] for p in pixels], w, h)
    G = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    mix = BLENDS[blend]
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        op = strength(d[i], width, invert) * opacity / 100
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) for c in range(3)]
        out.append([(b[c] + op * (mix(b[c], G[c]) - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

SKIN = R.SKIN                          # #f6d6be
SHADE = (220, 160, 140, 255)           # the skin's shadow, #dca08c
HALF = (246, 214, 190, 128)            # the skin at half covering: in, as 128/255 >= 0.5
QUARTER = (246, 214, 190, 64)          # the skin at a quarter covering: out, but it shows
DOT = (200, 40, 40, 255)               # a red dot, #c82828
NONE = S.NONE
VIOLET = "#6450a0"


def block(x, y):
    """A block in rows 1 to 8, skin in columns 0 to 6 and shadow in columns 7 to 9, against the
    drawing's left edge, with a hole, an empty pixel, at (4, 4); its soft right edge is the skin
    at half covering down column 10 and at a quarter down column 11; and a red dot alone at
    (14, 4). Rows 0 and 9 and the rest are empty."""
    if (x, y) == (14, 4):
        return DOT
    if not 1 <= y <= 8 or x > 11 or (x, y) == (4, 4):
        return NONE
    if x == 11:
        return QUARTER
    if x == 10:
        return HALF
    return SHADE if x >= 7 else SKIN


DRAWINGS = {"block": [[block(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(color=VIOLET, width=10, opacity=50, invert="off", blend="multiply", shift=0):
    return {"drawing": "block", "color": color, "width": width, "opacity": opacity,
            "invert": invert, "blend": blend, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(distance_gradation(pixels, c["color"], held(c, "width", frame_no),
                                      held(c, "opacity", frame_no), c["invert"], c["blend"]),
                   c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(width=0, shift=c["shift"]), 0)


FULL = {"opacity": 100, "blend": "normal"}

CASES = {
    "FX-DISTGRAD-001": ("The settings as they start: #6450a0, width 10, opacity 50, invert off, "
                        "multiply. Every pixel that shows is darkened toward violet, the most "
                        "along the block's edge, the dot and the quarter-covered column, and "
                        "less at each pixel further in, and the least at (7, 5), sqrt 10 from "
                        "the hole, the furthest from any out pixel; the empty pixels stay "
                        "empty.",
                        case(), [0]),
    "FX-DISTGRAD-002": ("Width 2, opacity 100, normal: the violet at full strength on the "
                        "quarter-covered column, fading in over two pixels; column 0, on the "
                        "drawing's left edge, is three quarters violet, as the top and bottom "
                        "rows are; the pixels round the hole are shaded; and every pixel 2.5 "
                        "or more from the nearest out pixel is left as it is: (7, 3) to (8, 6) "
                        "in the shadow, and (2, 6) and (6, 6).",
                        case(width=2, **FULL), [0]),
    "FX-DISTGRAD-003": ("Width 1, opacity 100, normal: a pixel beside an out pixel is half "
                        "violet, one out on a diagonal only is shaded by 1 - (sqrt 2 - 0.5), "
                        "about 0.086, and every pixel two or more from the edge is untouched.",
                        case(width=1, **FULL), [0]),
    "FX-DISTGRAD-004": ("Width 0.5, opacity 100, normal: only the pixels that are themselves "
                        "out and show, the quarter-covered column, are shaded, wholly violet at "
                        "their own covering; every pixel that is in is untouched.",
                        case(width=0.5, **FULL), [0]),
    "FX-DISTGRAD-005": ("Width 0: the drawing, untouched.",
                        case(width=0), [0]),
    "FX-DISTGRAD-006": ("Width 0 with invert on: still the drawing, untouched; width 0 comes "
                        "first.",
                        case(width=0, invert="on"), [0]),
    "FX-DISTGRAD-007": ("Opacity 0: the drawing, untouched.",
                        case(opacity=0), [0]),
    "FX-DISTGRAD-008": ("Width 2, opacity 100, normal, invert on: FX-DISTGRAD-002 turned round, "
                        "the quarter-covered column untouched, (7, 3) to (8, 6) wholly violet, "
                        "and each pixel shaded by one less FX-DISTGRAD-002's strength.",
                        case(width=2, invert="on", **FULL), [0]),
    "FX-DISTGRAD-009": ("Invert on, the rest as they start: the middle darkened the most and "
                        "the edge the least; the quarter-covered column is untouched.",
                        case(invert="on"), [0]),
    "FX-DISTGRAD-010": ("Blend normal, opacity 50: the violet laid over, at half the strength "
                        "at most.",
                        case(blend="normal"), [0]),
    "FX-DISTGRAD-011": ("Blend screen: every pixel that shows is lightened toward violet and "
                        "none darkened; no channel passes the pixel's covering.",
                        case(blend="screen"), [0]),
    "FX-DISTGRAD-012": ("Blend add: the violet added, past what a screen gives, the most at "
                        "the edge.",
                        case(blend="add"), [0]),
    "FX-DISTGRAD-013": ("Colour #ffffff, multiply: multiplying by white changes nothing, and "
                        "the drawing is untouched.",
                        case(color="#ffffff"), [0]),
    "FX-DISTGRAD-014": ("FX-DISTGRAD-001 with the colour written in capitals, #6450A0: the "
                        "same.",
                        case(color=VIOLET.upper()), [0]),
    "FX-DISTGRAD-015": ("Width keyed from 0 at frame 0 to 4 at frame 4, linear, opacity 100, "
                        "normal: frame 0 is the drawing, frame 2 is width 2, FX-DISTGRAD-002, "
                        "and frame 4 width 4; the shading reaches further in as the width grows.",
                        case(width=keyed((0, 0), (4, 4)), **FULL), [0, 2, 4]),
    "FX-DISTGRAD-016": ("Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                        "overshoots, width 2, normal: at frame 2 it would pass 100, is held "
                        "at 100, and is FX-DISTGRAD-002; frame 0 is the drawing.",
                        case(width=2, opacity=keyed((0, 0, OVERSHOOT), (4, 100)),
                             blend="normal"), [0, 2]),
    "FX-DISTGRAD-017": ("FX-DISTGRAD-001 moved three pixels right: the same, moved; the "
                        "shading is worked on the drawing before it moves, so the drawing's "
                        "left edge stays an edge.",
                        case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-DISTGRAD-018": ("Width 1001, above 1000.", case(width=1001)),
    "FX-DISTGRAD-019": ("Width -1, below 0.", case(width=-1)),
    "FX-DISTGRAD-020": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-DISTGRAD-021": ("Opacity keyed to 150 at frame 4.",
                        case(opacity=keyed((0, 50), (4, 150)))),
    "FX-DISTGRAD-022": ("Invert \"yes\", which is not \"off\" or \"on\".", case(invert="yes")),
    "FX-DISTGRAD-023": ("Blend \"overlay\", which is not a blend.", case(blend="overlay")),
    "FX-DISTGRAD-024": ("Colour \"#12345\", one digit short.", case(color="#12345")),
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
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.distance_gradation", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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

    (OUT / "expected_distance_gradation.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                          encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    pixels = [p for row in DRAWINGS["block"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    violet = [srgb_to_linear(v / 255) for v in R.hex_color(VIOLET)]
    d = distances([p[3] for p in pixels], W, H)
    # How far each pixel that shows is moved toward violet, when painted normal: its share.
    share = lambda f, i: ((f[i][1] - drawn[i][1]) / (violet[1] * drawn[i][3] - drawn[i][1]))  # noqa: E731,E501

    # The rule's own pieces: the distances, tried against every out pixel.
    outs = [(q % W + 0.5, q // W + 0.5) for q in range(W * H) if pixels[q][3] < 128]
    for i in range(W * H):
        x, y = i % W, i // W
        every = min([math.hypot(x + 0.5 - qx, y + 0.5 - qy) for qx, qy in outs]
                    + [min(x + 1, W - x, y + 1, H - y)])
        assert d[i] == every, i
    assert d[at(0, 4)] == 1 and d[at(14, 4)] == 1 and d[at(11, 4)] == 0 == d[at(4, 4)]
    assert d[at(10, 4)] == 1 and d[at(7, 4)] == 3 and d[at(3, 3)] == math.sqrt(2)
    assert abs(d[at(7, 5)] - math.sqrt(10)) < 1e-15 and max(d) == d[at(7, 5)]
    assert strength(1, 10, "off") == 0.95 and strength(0, 3, "off") == 1
    assert strength(4, 3, "off") == 0 and strength(4, 3, "on") == 1
    assert strength(2, 1, "off") == 0 and strength(1, 1, "off") == 0.5

    # Every case keeps every covering and leaves the empty pixels exactly as they are.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-DISTGRAD-017" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                if fx != "FX-DISTGRAD-012":
                    assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)

    one = c["FX-DISTGRAD-001"]["0"]
    assert all(one[i][1] < drawn[i][1] for i in shows)
    # Less at each pixel further in, along row 5 from the soft edge, and in the dot.
    darker = lambda x, y: 1 - one[at(x, y)][1] / drawn[at(x, y)][1]  # noqa: E731
    assert darker(11, 5) > darker(10, 5) > darker(9, 5) > darker(8, 5) > darker(7, 5)
    assert min((darker(i % W, i // W), i) for i in shows)[1] == at(7, 5)
    assert abs(darker(14, 4) - darker(0, 5)) < 1e-12  # the dot and the left edge, both d = 1
    two = c["FX-DISTGRAD-002"]["0"]
    assert all(close(two[at(11, y)], [v * 64 / 255 for v in violet] + [64 / 255])
               for y in range(1, 9))
    assert all(abs(share(two, at(0, y)) - share(two, at(5, 1))) < 1e-12 for y in range(2, 8))
    assert abs(share(two, at(0, 5)) - 3 / 4) < 1e-12
    untouched = [(x, y) for x in (7, 8) for y in range(3, 7)] + [(2, 6), (6, 6)]
    assert sorted(untouched) == sorted((i % W, i // W) for i in shows if two[i] == drawn[i])
    assert all(two[at(x, y)] != drawn[at(x, y)] for x in (3, 4, 5) for y in (3, 4, 5)
               if (x, y) != (4, 4))
    three = c["FX-DISTGRAD-003"]["0"]
    assert abs(share(three, at(4, 3)) - 0.5) < 1e-12 and abs(share(three, at(0, 5)) - 0.5) < 1e-12
    assert abs(share(three, at(3, 3)) - (1.5 - math.sqrt(2))) < 1e-12
    assert all(three[i] == drawn[i] for i in shows if d[i] >= 2)
    four = c["FX-DISTGRAD-004"]["0"]
    assert all(four[i] == drawn[i] for i in shows if d[i] > 0)
    assert all(close(four[at(11, y)], two[at(11, y)]) for y in range(1, 9))
    for fx in ("FX-DISTGRAD-005", "FX-DISTGRAD-006", "FX-DISTGRAD-007", "FX-DISTGRAD-013"):
        assert c[fx]["0"] == drawn, fx
    eight = c["FX-DISTGRAD-008"]["0"]
    assert eight[at(11, 4)] == drawn[at(11, 4)]
    assert all(close(eight[at(x, y)], violet + [1.0]) for x, y in untouched[:8])
    assert all(abs(share(eight, i) - (1 - share(two, i))) < 1e-9 for i in shows)
    nine = c["FX-DISTGRAD-009"]["0"]
    assert nine[at(11, 4)] == drawn[at(11, 4)]
    assert 1 - nine[at(7, 5)][1] / drawn[at(7, 5)][1] > 1 - nine[at(9, 1)][1] / drawn[at(9, 1)][1]
    ten = c["FX-DISTGRAD-010"]["0"]
    assert all(0 < share(ten, i) <= 0.5 + 1e-12 for i in shows)
    eleven = c["FX-DISTGRAD-011"]["0"]
    assert all(eleven[i][c_] >= drawn[i][c_] for i in shows for c_ in range(3))
    twelve = c["FX-DISTGRAD-012"]["0"]
    assert all(twelve[i][2] > eleven[i][2] for i in shows)
    assert twelve[at(0, 5)][2] - drawn[at(0, 5)][2] > twelve[at(7, 5)][2] - drawn[at(7, 5)][2]
    assert c["FX-DISTGRAD-014"]["0"] == one
    fifteen = c["FX-DISTGRAD-015"]
    assert fifteen["0"] == drawn
    assert fifteen["2"] == two
    assert fifteen["4"] == render(case(width=4, **FULL), 0)
    assert sum(p != q for p, q in zip(fifteen["4"], drawn)) > sum(p != q for p, q in
                                                                  zip(fifteen["2"], drawn))
    sixteen = c["FX-DISTGRAD-016"]
    assert ease(OVERSHOOT, 0.5) > 1 and sixteen["0"] == drawn and sixteen["2"] == two
    moved = c["FX-DISTGRAD-017"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
