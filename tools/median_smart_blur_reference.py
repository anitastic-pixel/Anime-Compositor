"""Median and Smart Blur, worked a second way.

D-203 adds two effects modelled on After Effects' Median and Smart Blur, by rules of our own;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

Both look at a disc round each pixel: the taps (dx, dy) with dx^2 + dy^2 <= radius^2, the pixel
itself among them. A tap outside the input buffer is transparent. When the disc holds only the
pixel itself, radius below 1, the output is the input exactly. Nothing grows.

  `core.median`: each pixel's straight linear colour is the median of the taps that show, one
  channel at a time; an even count of values takes the mean of the middle two. With Operate on
  Alpha off the pixel keeps its own covering, so a pixel that does not show stays transparent
  and the drawing's outline is kept. With it on its covering is the median of every tap's,
  transparent ones too, so holes fill, specks of covering go and sharp corners are rounded off;
  the colour still comes only from taps that show, so no dark fringe creeps in.

  `core.smart_blur`: each pixel that shows becomes the premultiplied mean of the taps whose four
  8-bit values, straight red, green and blue through the sRGB curve and covering, are each
  within Threshold of its own, as document 21's D-88 rounds them; it counts itself. A tap that
  does not show never counts, and a pixel that does not show stays transparent. Flat colour and
  fine grain are smoothed; an edge more than Threshold across is kept sharp.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: skin with specks, a hole, a line two pixels wide, a patch of grain and a column
at half covering. The drawing goes into `Fixtures/median_smart_blur/media`, the projects into
`Fixtures/median_smart_blur`, and the expected frames into
`Fixtures/median_smart_blur/expected_median_smart_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/median_smart_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "median_smart_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
RADIUS = (0, 10)
THRESHOLD = (0, 255)
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def disc(radius):
    r = math.floor(radius)
    return [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
            if dx * dx + dy * dy <= radius * radius]


def tap(layer, x, y):
    if 0 <= x < layer["w"] and 0 <= y < layer["h"]:
        return layer["px"][y * layer["w"] + x]
    return EMPTY


def middle(values):
    v = sorted(values)
    n = len(v)
    return v[n // 2] if n % 2 else (v[n // 2 - 1] + v[n // 2]) / 2


def median(layer, radius, operate_on_alpha):
    taps = disc(radius)
    if len(taps) == 1:
        return layer
    px = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            near = [tap(layer, x + dx, y + dy) for dx, dy in taps]
            a = middle([p[3] for p in near]) if operate_on_alpha == "on" else tap(layer, x, y)[3]
            if a <= 0:
                px.append(EMPTY)
                continue
            shown = [p for p in near if p[3] > 0]
            px.append([middle([p[i] / p[3] for p in shown]) * a for i in range(3)] + [a])
    return dict(layer, px=px)


def levels(p):
    """D-88's 8-bit values of a pixel that shows: straight colour through the sRGB curve, and
    covering, each rounded half up; None for a pixel that does not show."""
    a = p[3]
    if a <= 0:
        return None
    q = lambda v: math.floor(min(1, max(0, v)) * 255 + 0.5)  # noqa: E731
    return [q(S.linear_to_srgb(p[i] / a)) for i in range(3)] + [q(a)]


def smart_blur(layer, radius, threshold):
    taps = disc(radius)
    if len(taps) == 1:
        return layer
    lv = [levels(p) for p in layer["px"]]
    px = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            own = lv[y * layer["w"] + x]
            if own is None:
                px.append(EMPTY)
                continue
            counted = []
            for dx, dy in taps:
                tx, ty = x + dx, y + dy
                if 0 <= tx < layer["w"] and 0 <= ty < layer["h"]:
                    t = lv[ty * layer["w"] + tx]
                    if t is not None and all(abs(t[k] - own[k]) <= threshold for k in range(4)):
                        counted.append(layer["px"][ty * layer["w"] + tx])
            px.append([sum(p[k] for p in counted) / len(counted) for k in range(4)])
    return dict(layer, px=px)


# --- the drawing ----------------------------------------------------------------------------

SKIN, LINE, NONE = R.SKIN, R.LINE, S.NONE
PALETTE = {
    "s": SKIN,
    "l": LINE,
    "w": (255, 255, 255, 255),  # a white speck: 9, 41 and 65 above the skin
    "+": (254, 222, 198, 255),  # grain, the skin 8 lighter
    "-": (238, 206, 182, 255),  # grain, the skin 8 darker
    "h": (246, 214, 190, 128),  # the skin at half covering
    ".": NONE,
}
# Columns 9 and 10 are the line, column 14 the skin at half covering, column 15 and row 9 empty.
# Dark specks at (2, 1) and (12, 4), a white one at (6, 2), a hole at (12, 1), and grain in
# rows 5 to 8, columns 0 to 7.
MAP = [
    "sssssssssllsssh.",
    "sslsssssslls.sh.",
    "sssssswssllsssh.",
    "sssssssssllsssh.",
    "sssssssssllslsh.",
    "+s-s+-s+sllsssh.",
    "s-+s-s+-sllsssh.",
    "-s+-s+s-sllsssh.",
    "s+s-+s-+sllsssh.",
    "................",
]
DRAWINGS = {"specks": [[PALETTE[ch] for ch in row] for row in MAP]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def median_case(radius=2, operate_on_alpha="off", shift=0):
    return {"type": "core.median", "drawing": "specks", "shift": shift,
            "parameters": {"radius": radius, "operate_on_alpha": operate_on_alpha}}


def smart_case(radius=3, threshold=64, shift=0):
    return {"type": "core.smart_blur", "drawing": "specks", "shift": shift,
            "parameters": {"radius": radius, "threshold": threshold}}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    p = c["parameters"]
    radius = held(value_at(p["radius"], frame_no), RADIUS)
    layer = drawn_layer(c["drawing"])
    if c["type"] == "core.median":
        layer = median(layer, radius, p["operate_on_alpha"])
    else:
        layer = smart_blur(layer, radius, held(value_at(p["threshold"], frame_no), THRESHOLD))
    return frame(layer, c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-MEDIAN-001": ("Median as it starts, Radius 2 and Operate on Alpha off: the three specks "
                      "are gone into the skin and the grain is mostly skin, while the line, two "
                      "pixels wide, is kept, and so is the drawing's outline: the hole stays a "
                      "hole and the half-covered column stays at half.",
                      median_case(), [0]),
    "FX-MEDIAN-002": ("Radius 0: the drawing, untouched.", median_case(0), [0]),
    "FX-MEDIAN-003": ("Radius 1, a disc of five, the pixel and its four neighbours: the specks "
                      "are gone and the line is kept.",
                      median_case(1), [0]),
    "FX-MEDIAN-004": ("Radius 2 with Operate on Alpha on: the specks are gone and the hole is "
                      "filled with skin; the drawing's top-left corner, with more of its disc "
                      "outside the drawing than in, is cut away, and the half-covered column "
                      "takes its covering from its neighbours.",
                      median_case(2, "on"), [0]),
    "FX-MEDIAN-005": ("Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the "
                      "drawing and frame 2 is FX-MEDIAN-001.",
                      median_case(keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-MEDIAN-006": ("Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end "
                      "(about 13 at frame 2): frame 2 is held at 10, the same as frame 4, where "
                      "every pixel that shows is the skin at its own covering: the line, the "
                      "specks and the grain are all gone.",
                      median_case(keyed((0, 0, OVERSHOOT), (4, 10))), [0, 2, 4]),
    "FX-MEDIAN-007": ("Median as it starts, the layer moved three pixels right: FX-MEDIAN-001 "
                      "moved with it.",
                      median_case(shift=3), [0]),
    "FX-SMART-001": ("Smart Blur as it starts, Radius 3 and Threshold 64: the grain, never more "
                     "than 16 apart, is smoothed toward the skin, while the line and the dark "
                     "specks, far more than 64 from the skin, are kept sharp, and so is the "
                     "white speck, 65 from the skin in its blue. The hole stays a hole and the "
                     "half-covered column, 127 from the skin in its covering, is kept apart.",
                     smart_case(), [0]),
    "FX-SMART-002": ("Radius 0: the drawing, untouched.", smart_case(0), [0]),
    "FX-SMART-003": ("Threshold 0: only the very same colour is mixed, so the drawing is "
                     "unchanged.", smart_case(3, 0), [0]),
    "FX-SMART-004": ("Threshold 8: grain 8 from the skin mixes with the skin but not with grain "
                     "8 the other way, so the grain is only partly smoothed.",
                     smart_case(3, 8), [0]),
    "FX-SMART-005": ("Threshold 255: every tap that shows counts, a plain disc blur that stays "
                     "inside the drawing: the line and the specks are blurred into the skin, "
                     "and the hole, the empty column and the empty row stay empty.",
                     smart_case(3, 255), [0]),
    "FX-SMART-006": ("Threshold keyed from 0 at frame 0 to 16 at frame 4, linear: frame 0 is "
                     "the drawing, frame 2 is FX-SMART-004 at 8, and frame 4 is FX-SMART-001, "
                     "for nothing in the drawing lies between 17 and 64 apart.",
                     smart_case(3, keyed((0, 0), (4, 16))), [0, 2, 4]),
    "FX-SMART-007": ("Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end "
                     "(about 13 at frame 2): frame 2 is held at 10, the same as frame 4.",
                     smart_case(keyed((0, 0, OVERSHOOT), (4, 10))), [0, 2, 4]),
    "FX-SMART-008": ("Smart Blur as it starts, the layer moved three pixels right: FX-SMART-001 "
                     "moved with it.",
                     smart_case(shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-MEDIAN-008": ("Median, Radius 11, above 10.", median_case(11)),
    "FX-MEDIAN-009": ("Median, Radius -1, below 0.", median_case(-1)),
    "FX-MEDIAN-010": ("Median, Radius keyed to 20 at frame 4.",
                      median_case(keyed((0, 0), (4, 20)))),
    "FX-MEDIAN-011": ("Median, Operate on Alpha \"sometimes\", which is not one.",
                      median_case(2, "sometimes")),
    "FX-SMART-009": ("Smart Blur, Radius 11, above 10.", smart_case(11)),
    "FX-SMART-010": ("Smart Blur, Threshold 256, above 255.", smart_case(3, 256)),
    "FX-SMART-011": ("Smart Blur, Threshold -1, below 0.", smart_case(3, -1)),
    "FX-SMART-012": ("Smart Blur, Threshold keyed to 300 at frame 4.",
                     smart_case(3, keyed((0, 0), (4, 300)))),
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
        "instance_id": "fx-0-0", "type_id": c["type"], "enabled": True,
        "parameters": {k: v if isinstance(v, str) else setting_json(v)
                       for k, v in c["parameters"].items()}}]
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

    (OUT / "expected_median_smart_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(median_case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    look = lambda ch: R.working(PALETTE[ch])  # noqa: E731
    skin, line, soft = look("s"), look("l"), look("h")
    specks = ((2, 1), (6, 2), (12, 4))
    grain = [(x, y) for x in range(8) for y in range(5, 9) if MAP[y][x] in "+-"]
    lines = [(x, y) for x in (9, 10) for y in range(9)]

    assert len(disc(0)) == 1 and len(disc(0.99)) == 1 and len(disc(1)) == 5
    assert len(disc(1.5)) == 9 and len(disc(2)) == 13
    assert middle([3, 1, 2]) == 2 and middle([4, 1, 3, 2]) == 2.5
    assert levels(look("w")) == [255, 255, 255, 255] and levels(soft) == [246, 214, 190, 128]
    assert levels(EMPTY) is None

    one = c["FX-MEDIAN-001"]["0"]
    assert all(near(one[at(x, y)], skin) for x, y in specks)
    assert all(near(one[at(x, y)], line) for x, y in lines)
    assert one[at(12, 1)] == EMPTY
    assert all(near(one[at(14, y)], soft) for y in range(9))
    assert sum(near(one[at(x, y)], skin) for x, y in grain) > len(grain) / 2
    assert all(one[at(x, 9)] == EMPTY for x in range(W)) and all(
        one[at(15, y)] == EMPTY for y in range(H))
    assert c["FX-MEDIAN-002"]["0"] == drawn
    three = c["FX-MEDIAN-003"]["0"]
    assert all(near(three[at(x, y)], skin) for x, y in specks)
    assert all(near(three[at(x, y)], line) for x, y in lines)
    four = c["FX-MEDIAN-004"]["0"]
    assert all(near(four[at(x, y)], skin) for x, y in specks + ((12, 1),))
    assert four[at(0, 0)][3] == 0 and one[at(0, 0)][3] == 1
    five = c["FX-MEDIAN-005"]
    assert five["0"] == drawn and five["2"] == one and five["4"] not in (drawn, one)
    six = c["FX-MEDIAN-006"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 10)), 2) > 10
    assert six["0"] == drawn and six["2"] == six["4"]
    for i, p in enumerate(six["4"]):
        own = drawn[i][3]
        assert (p == EMPTY) if own == 0 else near(p, [v * own for v in skin[:3]] + [own])
    seven = c["FX-MEDIAN-007"]["0"]
    assert all(seven[at(x + 3, y)] == one[at(x, y)] for x in range(W - 3) for y in range(H))

    s1 = c["FX-SMART-001"]["0"]
    # A mean of equal values may differ from them in the last place, hence 1e-12, not equal.
    same = lambda px, qx: all(near(p, q, 1e-12) for p, q in zip(px, qx))  # noqa: E731
    assert all(near(s1[at(x, y)], drawn[at(x, y)], 1e-12) for x, y in specks + tuple(lines))
    assert all(near(s1[at(14, y)], soft, 1e-12) for y in range(9)) and s1[at(12, 1)] == EMPTY
    assert all(s1[at(x, y)] != drawn[at(x, y)] for x, y in grain)
    spread = lambda px: max(abs(px[at(x, y)][2] - skin[2]) for x, y in grain)  # noqa: E731
    assert spread(s1) < spread(drawn) / 2
    assert c["FX-SMART-002"]["0"] == drawn
    assert same(c["FX-SMART-003"]["0"], drawn)
    s4 = c["FX-SMART-004"]["0"]
    assert spread(s1) < spread(s4) < spread(drawn)
    s5 = c["FX-SMART-005"]["0"]
    assert all(not near(s5[at(x, y)], drawn[at(x, y)], 1e-3) for x, y in specks + tuple(lines))
    assert s5[at(12, 1)] == EMPTY and all(s5[at(x, 9)] == EMPTY for x in range(W))
    assert all(s5[at(15, y)] == EMPTY for y in range(H)) and s5[at(14, 4)][3] > 0.5
    s6 = c["FX-SMART-006"]
    assert same(s6["0"], drawn) and s6["2"] == s4 and s6["4"] == s1
    s7 = c["FX-SMART-007"]
    assert s7["0"] == drawn and s7["2"] == s7["4"] != drawn
    s8 = c["FX-SMART-008"]["0"]
    assert all(s8[at(x + 3, y)] == s1[at(x, y)] for x in range(W - 3) for y in range(H))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


def show(px):
    """A map for working the cases out: . clear, + part covered, l line, s skin, g other."""
    skin = R.working(SKIN)
    rows = []
    for y in range(H):
        row = ""
        for x in range(W):
            p = px[y * W + x]
            row += ("." if p[3] < 1e-9 else "+" if p[3] < 0.99 else "l" if p[0] < 0.1 else
                    "s" if all(abs(a - b) < 1e-6 for a, b in zip(p, skin)) else "g")
        rows.append(row)
    return "\n".join(rows)


if __name__ == "__main__":
    if sys.argv[1:] == ["show"]:
        for fx, (_, c, frames) in CASES.items():
            for f in frames:
                print(fx, f)
                print(show(render(c, f)))
    else:
        main()
