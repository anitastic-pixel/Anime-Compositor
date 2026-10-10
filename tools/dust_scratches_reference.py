"""Dust & Scratches, worked a second way.

D-453 adds an effect modelled on After Effects' Dust & Scratches (Noise & Grain), by a rule of
our own; nothing is ported. Adobe's words for it are "changing dissimilar pixels within a
specified radius to be more like their neighboring pixels", with a Radius, a Threshold and
Operate on Alpha. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

It is D-203's Median with a threshold. The disc round each pixel is Median's: the taps (dx, dy)
with dx^2 + dy^2 <= radius^2, the pixel itself among them, a tap outside the input buffer
transparent. When the disc holds only the pixel itself, radius below 1, the output is the input
exactly. Nothing grows.

  Each channel of a pixel's straight linear colour is compared with Median's for it, the median
  of the taps that show: when their 8-bit values, through the sRGB curve and rounded half up as
  document 21's D-88 rounds them, are more than Threshold apart, the channel takes the median;
  otherwise it keeps its own. So a speck far from its surroundings goes and grain within
  Threshold of them stays. With Operate on Alpha off the pixel keeps its own covering, so the
  drawing's outline is kept. With it on its covering is compared the same way with the median of
  every tap's covering, transparent ones too, and takes it when more than Threshold apart; a
  pixel that did not show and now does takes Median's colour, having none of its own. Threshold
  0 changes every channel whose 8-bit value differs from the median's, Median but for values
  within one step; Threshold 255 changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding D-203's drawing, `specks`, the same size,
unmoved unless the case says: skin with specks, a hole, a line two pixels wide, a patch of grain
and a column at half covering. The drawing goes into `Fixtures/dust_scratches/media`, the
projects into `Fixtures/dust_scratches`, and the expected frames into
`Fixtures/dust_scratches/expected_dust_scratches.json`. No case has a mask.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/dust_scratches_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import median_smart_blur_reference as M  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402

W, H = M.W, M.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "dust_scratches"
TOLERANCE = 2e-5  # document 25's default for a filter
RADIUS = (0, 10)
THRESHOLD = (0, 255)
EMPTY = M.EMPTY


# --- the rule -------------------------------------------------------------------------------

def q(v):
    """D-88's 8-bit value of a number from 0 to 1, rounded half up."""
    return math.floor(min(1, max(0, v)) * 255 + 0.5)


def level(v):
    """The 8-bit value of a straight linear channel, through the sRGB curve."""
    return q(S.linear_to_srgb(v))


def dust_scratches(layer, radius, threshold, operate_on_alpha):
    taps = M.disc(radius)
    if len(taps) == 1:
        return layer
    px = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            near = [M.tap(layer, x + dx, y + dy) for dx, dy in taps]
            own = M.tap(layer, x, y)
            a = own[3]
            if operate_on_alpha == "on":
                am = M.middle([p[3] for p in near])
                if abs(q(am) - q(a)) > threshold:
                    a = am
            if a <= 0:
                px.append(EMPTY)
                continue
            shown = [p for p in near if p[3] > 0]
            out = []
            for i in range(3):
                m = M.middle([p[i] / p[3] for p in shown])
                mine = own[i] / own[3] if own[3] > 0 else None
                keep = mine is not None and abs(level(mine) - level(m)) <= threshold
                out.append((mine if keep else m) * a)
            px.append(out + [a])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(radius=1, threshold=0, operate_on_alpha="off", shift=0):
    return {"type": "core.dust_scratches", "drawing": "specks", "shift": shift,
            "parameters": {"radius": radius, "threshold": threshold,
                           "operate_on_alpha": operate_on_alpha}}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    p = c["parameters"]
    layer = M.drawn_layer(c["drawing"])
    layer = dust_scratches(layer, held(value_at(p["radius"], frame_no), RADIUS),
                           held(value_at(p["threshold"], frame_no), THRESHOLD),
                           p["operate_on_alpha"])
    return frame(layer, c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(M.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-DUST-001": ("Dust & Scratches as it starts, Radius 1, Threshold 0 and Operate on Alpha "
                    "off: a disc of five, the pixel and its four neighbours, so the three specks "
                    "are gone into the skin and the line, two pixels wide, is kept; the hole "
                    "stays a hole and the half-covered column stays at half.",
                    case(), [0]),
    "FX-DUST-002": ("Radius 0: the drawing, untouched.", case(0), [0]),
    "FX-DUST-003": ("Radius 2, Threshold 0: D-203's Median at Radius 2, FX-MEDIAN-001, the specks "
                    "gone and the grain mostly skin.",
                    case(2), [0]),
    "FX-DUST-004": ("Radius 2, Threshold 16: the dark specks, far more than 16 from the skin, "
                    "are gone as in FX-DUST-003, and the white speck, 9, 41 and 65 above the "
                    "skin in red, green and blue, keeps its red and takes the skin's green and "
                    "blue; the grain, 8 from the skin, is kept just as drawn.",
                    case(2, 16), [0]),
    "FX-DUST-005": ("Radius 2, Threshold 64: the dark specks are gone; the white speck, 9, 41 "
                    "and 65 above the skin in red, green and blue, keeps its red and green and "
                    "takes the skin's blue, the one channel more than 64 apart, so it turns "
                    "yellow. The grain is kept.",
                    case(2, 64), [0]),
    "FX-DUST-006": ("Radius 2, Threshold 255: nothing is ever more than 255 apart, so the "
                    "drawing is unchanged.", case(2, 255), [0]),
    "FX-DUST-007": ("Radius 2, Threshold 16, Operate on Alpha on: the specks go as in "
                    "FX-DUST-004 and the hole, 255 from the skin round it in its covering, is filled with skin; the "
                    "drawing's top-left corner, with more of its disc outside the drawing than "
                    "in, is cut away, and the grain is kept.",
                    case(2, 16, "on"), [0]),
    "FX-DUST-008": ("Radius 2, Threshold 0, Operate on Alpha off: the hole stays a hole, for a "
                    "pixel keeps its own covering.",
                    case(2, 0, "off"), [0]),
    "FX-DUST-009": ("Threshold keyed from 0 at frame 0 to 16 at frame 4, linear, at Radius 2: "
                    "frame 0 is FX-DUST-003 and frame 4 is FX-DUST-004; at frame 2, Threshold 8, "
                    "the grain 8 from the median is kept.",
                    case(2, keyed((0, 0), (4, 16))), [0, 2, 4]),
    "FX-DUST-010": ("Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about "
                    "13 at frame 2), at Threshold 0: frame 2 is held at 10, the same as frame 4, "
                    "where every pixel that shows is the skin at its own covering.",
                    case(keyed((0, 0, OVERSHOOT), (4, 10))), [0, 2, 4]),
    "FX-DUST-011": ("Dust & Scratches as it starts, the layer moved three pixels right: "
                    "FX-DUST-001 moved with it.",
                    case(shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-DUST-012": ("Radius 11, above 10.", case(11)),
    "FX-DUST-013": ("Radius -1, below 0.", case(-1)),
    "FX-DUST-014": ("Radius keyed to 20 at frame 4.", case(keyed((0, 0), (4, 20)))),
    "FX-DUST-015": ("Threshold 256, above 255.", case(1, 256)),
    "FX-DUST-016": ("Threshold -1, below 0.", case(1, -1)),
    "FX-DUST-017": ("Threshold keyed to 300 at frame 4.", case(1, keyed((0, 0), (4, 300)))),
    "FX-DUST-018": ("Operate on Alpha \"sometimes\", which is not one.", case(1, 0, "sometimes")),
}


# --- the project files ----------------------------------------------------------------------

def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(M.project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in M.DRAWINGS.items():
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

    (OUT / "expected_dust_scratches.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, r, e=1e-9: all(abs(a - b) < e for a, b in zip(p, r))  # noqa: E731
    look = lambda ch: R.working(M.PALETTE[ch])  # noqa: E731
    skin, line, soft = look("s"), look("l"), look("h")
    specks = ((2, 1), (6, 2), (12, 4))
    grain = [(x, y) for x in range(8) for y in range(5, 9) if M.MAP[y][x] in "+-"]
    lines = [(x, y) for x in (9, 10) for y in range(9)]
    median = lambda r, ooa="off": M.median(M.drawn_layer("specks"), r, ooa)["px"]  # noqa: E731

    assert level(1.0) == 255 and level(0.0) == 0 and q(0.5) == 128

    one = c["FX-DUST-001"]["0"]
    assert all(near(one[at(x, y)], skin) for x, y in specks)
    assert all(near(one[at(x, y)], line) for x, y in lines)
    assert one[at(12, 1)] == EMPTY and all(near(one[at(14, y)], soft) for y in range(9))
    assert c["FX-DUST-002"]["0"] == drawn
    # Threshold 0 is Median but where the median is within one 8-bit step of the pixel's own.
    three, m2 = c["FX-DUST-003"]["0"], median(2)
    assert all(near(p, r) or near(p, d) for p, r, d in zip(three, m2, drawn))
    assert sum(near(p, r) for p, r in zip(three, m2)) > W * H * 0.95
    four = c["FX-DUST-004"]["0"]
    white = look("w")
    assert near(four[at(2, 1)], skin) and near(four[at(12, 4)], skin)
    assert four[at(6, 2)][0] == white[0] and near(four[at(6, 2)][1:], skin[1:])
    assert all(four[at(x, y)] == drawn[at(x, y)] for x, y in grain)
    assert all(near(four[at(x, y)], line) for x, y in lines)
    assert any(not near(three[at(x, y)], drawn[at(x, y)]) for x, y in grain)
    five = c["FX-DUST-005"]["0"]
    assert five[at(6, 2)][:2] == white[:2] and near([five[at(6, 2)][2]], [skin[2]])
    assert near(five[at(2, 1)], skin) and near(five[at(12, 4)], skin)
    assert c["FX-DUST-006"]["0"] == drawn
    seven = c["FX-DUST-007"]["0"]
    assert all(near(seven[at(x, y)], skin) for x, y in ((2, 1), (12, 4), (12, 1)))
    assert seven[at(6, 2)] == four[at(6, 2)]
    assert seven[at(0, 0)][3] == 0 and all(seven[at(x, y)] == drawn[at(x, y)] for x, y in grain)
    assert c["FX-DUST-008"]["0"][at(12, 1)] == EMPTY
    nine = c["FX-DUST-009"]
    assert nine["0"] == three and nine["4"] == four and nine["2"] not in (three, four)
    ten = c["FX-DUST-010"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 10)), 2) > 10
    assert ten["0"] == drawn and ten["2"] == ten["4"]
    for i, p in enumerate(ten["4"]):
        own = drawn[i][3]
        assert (p == EMPTY) if own == 0 else near(p, [v * own for v in skin[:3]] + [own])
    eleven = c["FX-DUST-011"]["0"]
    assert all(eleven[at(x + 3, y)] == one[at(x, y)] for x in range(W - 3) for y in range(H))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
