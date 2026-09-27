"""Simple choker, worked a second way.

D-159 adds `core.simple_choker`, which tightens or loosens a drawing's edge, as a compositor
chokes a matte: a positive `choke` shrinks the covering by that many pixels, taking away specks
and thin edges and opening holes wider; a negative one spreads it, filling small holes and gaps
and growing the layer so the spread edge is kept. The colour is never mixed where there was
covering: a shrunk pixel keeps its own straight colour at less covering, and a spread pixel that
was empty takes the covering-weighted average colour of the covered pixels within reach. It is
this program's own method, modelled on After Effects' Simple Choker in spirit and not claimed to
match it; nothing is ported. Document 21 is the rule in words; this file is the reference for
the numbers document 25 pins against it.

The rule. Choke 0: the output is the input. R = |choke|; the disc is the whole offsets (dx, dy)
with dx^2 + dy^2 <= R^2 (Outline's steps, imported). Choke > 0 (shrink): a' is the smallest
covering over the disc about the pixel, pixels outside the drawing counting as 0; where a > 0
the output is p a' / a, its straight colour kept; a = 0 pixels stay empty; nothing grows.
Choke < 0 (spread): the input first grows by floor(R) transparent pixels on every side; a' is
the largest covering over the disc; the straight colour is the pixel's own where a > 0, else
the disc's sum p.rgb / sum p.a (0 when that sum is 0); the output is (colour a', a').

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/simple_choker/media`, the projects
into `Fixtures/simple_choker`, and the expected frames into
`Fixtures/simple_choker/expected_simple_choker.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/simple_choker_reference.py
"""

import functools
import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import pixel, EMPTY  # noqa: E402
from outline_reference import steps  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "simple_choker"
TOLERANCE = 2e-5  # document 25's default for a filter
CHOKE = (-100, 100)


# --- the rule -------------------------------------------------------------------------------

disc = functools.lru_cache(maxsize=None)(steps)  # the whole offsets within R, itself included


def growth(choke):
    return math.floor(-choke) if choke < 0 else 0


def choked(layer, choke, x, y):
    """The output at pixel (x, y) of the drawing's own space; empty outside the grown layer."""
    p = pixel(layer, x, y)
    if choke == 0:
        return p
    near = disc(abs(choke))
    if choke > 0:
        if p[3] == 0:
            return p
        a = min(pixel(layer, x + dx, y + dy)[3] for dx, dy in near)
        k = a / p[3]
        return [p[0] * k, p[1] * k, p[2] * k, a]
    g = growth(choke)
    if not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return EMPTY
    got = [pixel(layer, x + dx, y + dy) for dx, dy in near]
    a = max(q[3] for q in got)
    if p[3] > 0:
        k = a / p[3]
        return [p[0] * k, p[1] * k, p[2] * k, a]
    total = sum(q[3] for q in got)
    if total == 0:
        return EMPTY
    return [sum(q[j] for q in got) / total * a for j in range(3)] + [a]


# --- the drawing ----------------------------------------------------------------------------

LINE, SOFT, SKIN, NONE = R.LINE, R.SOFT, R.SKIN, S.NONE
SOFT_SKIN = (246, 214, 190, 128)  # the skin at half covering
BAND = (58, 111, 216, 255)  # #3a6fd8, the ball's blue band


def matte(x, y):
    """A box of line in columns 4 to 10 and rows 2 to 7, filled with skin, with a one-pixel hole
    at (7, 4) and the skin at half covering at (8, 6); the line at half covering at (11, 4), its
    antialiased edge; a one-pixel speck of line at (13, 8); and a blue block in columns 0 to 2
    and rows 6 to 9, touching the drawing's left and bottom edges, one empty column from the
    box. The rest is empty."""
    if (x, y) == (11, 4):
        return SOFT
    if (x, y) == (13, 8):
        return LINE
    if x <= 2 and y >= 6:
        return BAND
    if not (4 <= x <= 10 and 2 <= y <= 7) or (x, y) == (7, 4):
        return NONE
    if (x, y) == (8, 6):
        return SOFT_SKIN
    if x in (4, 10) or y in (2, 7):
        return LINE
    return SKIN


DRAWINGS = {"matte": [[matte(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(choke=0, shift=0):
    return {"drawing": "matte", "choke": choke, "shift": shift}


def held(c, frame_no):
    return min(CHOKE[1], max(CHOKE[0], value_at(c["choke"], frame_no)))


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    k = held(c, frame_no)
    return [choked(layer, k, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


CASES = {
    "FX-CHOKE-001": ("The settings as they start: choke 0. The output is the drawing, untouched, "
                     "on every frame, and nothing grows.",
                     case(), [0, 4]),
    "FX-CHOKE-002": ("Choke 1 shrinks by the four pixels beside each pixel: every pixel with an "
                     "empty pixel or the drawing's edge beside it loses all its covering, so the "
                     "box's line goes, and with it the soft pixel at (11, 4), the speck at "
                     "(13, 8), the four skin pixels round the hole at (7, 4) and the blue "
                     "block's outer pixels, leaving only (1, 7) and (1, 8) of it; the line at "
                     "(10, 4), beside the soft pixel, keeps half its covering, 128/255, in its "
                     "own colour; the half-covered skin at (8, 6) is unchanged, and its three "
                     "skin neighbours take its half covering. Empty pixels stay empty, and "
                     "nothing grows.",
                     case(choke=1), [0]),
    "FX-CHOKE-003": ("Choke 1.5: the disc now holds the four diagonal pixels too, a square of "
                     "nine. The line at (10, 4) now reaches the empty (11, 3) and (11, 5) and "
                     "goes; the hole takes all eight of its neighbours; the blue block still "
                     "keeps (1, 7) and (1, 8); and the skin is only ever kept or thinned, never "
                     "thickened, against FX-CHOKE-002.",
                     case(choke=1.5), [0]),
    "FX-CHOKE-004": ("Choke 0.5: the disc holds only the pixel itself, so the output is the "
                     "drawing exactly.",
                     case(choke=0.5), [0]),
    "FX-CHOKE-005": ("Choke -0.5: the same, spreading: the disc holds only the pixel itself, "
                     "nothing grows, and the output is the drawing.",
                     case(choke=-0.5), [0]),
    "FX-CHOKE-006": ("Choke -1 spreads by the four pixels beside each pixel, and the layer "
                     "grows by 1. Each empty pixel beside a covered one takes that covering in "
                     "full and the covered neighbours' covering-weighted average colour: the hole "
                     "at (7, 4) fills with skin; (3, 6) and (3, 7), between the blue block and "
                     "the box's line, take half blue and half line; (12, 4), beside only the soft "
                     "pixel, takes the line at half covering; the speck grows to a cross of line. "
                     "A covered pixel keeps its own colour and takes the largest covering in "
                     "reach, so the soft pixel at (11, 4) becomes the line in full and the "
                     "half-covered skin at (8, 6) the skin in full.",
                     case(choke=-1), [0]),
    "FX-CHOKE-007": ("Choke -1.5: the square of nine, so the corners fill too: (3, 1), "
                     "diagonal from the box's corner, takes the line in full, and (3, 8) takes "
                     "three parts blue to one part line, where FX-CHOKE-006 gives it blue alone. "
                     "The layer still grows by 1.",
                     case(choke=-1.5), [0]),
    "FX-CHOKE-008": ("Choke -2: the box's line reaches two pixels out, to row 0 and columns 2 "
                     "and 12, its corners rounded, so (2, 0), two across and two up from the "
                     "box's corner, stays empty; the gap between the blue block and the box "
                     "fills; the layer grows by 2.",
                     case(choke=-2), [0]),
    "FX-CHOKE-009": ("Choke keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is the "
                     "drawing, frame 2 is choke 1.5, FX-CHOKE-003, and at frame 4, choke 3, "
                     "nothing is left: no part of the drawing is seven pixels across both ways.",
                     case(choke=keyed((0, 0), (4, 3))), [0, 2, 4]),
    "FX-CHOKE-010": ("Choke keyed from 0 at frame 0 to -100 at frame 4, eased past its end: "
                     "frame 2 would be below -100 and is held at -100, so it is frame 4. At "
                     "spread 100 the disc about every pixel of the frame holds the whole drawing, "
                     "so the whole frame is covered in full: each covered pixel in its own "
                     "colour, and every empty one in one colour, the whole drawing's "
                     "covering-weighted average.",
                     case(choke=keyed((0, 0, OVERSHOOT), (4, -100))), [0, 2, 4]),
    "FX-CHOKE-011": ("Choke 100, the most: every disc reaches past the drawing's edge, where "
                     "the covering counts as 0, so nothing is left.",
                     case(choke=100), [0]),
    "FX-CHOKE-012": ("FX-CHOKE-002 moved three pixels right: the choke is worked in the "
                     "drawing's own space, so it moves with it, and the drawing's left edge "
                     "still counts as empty beside the blue block.",
                     case(choke=1, shift=3), [0]),
    "FX-CHOKE-013": ("FX-CHOKE-008 moved three pixels right: the layer grew by 2, and the two "
                     "columns left of the drawing show the grown pixels, into which the blue "
                     "block spreads, while the column left of those stays empty.",
                     case(choke=-2, shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CHOKE-014": ("Choke 101, above 100.", case(choke=101)),
    "FX-CHOKE-015": ("Choke -101, below -100.", case(choke=-101)),
    "FX-CHOKE-016": ("Choke keyed to 150 at frame 4.", case(choke=keyed((0, 1), (4, 150)))),
    "FX-CHOKE-017": ("Choke keyed from -120 at frame 0.",
                     case(choke=keyed((0, -120), (4, -1)))),
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
        "instance_id": "fx-0-0", "type_id": "core.simple_choker", "enabled": True,
        "parameters": {"choke": setting_json(c["choke"])}}]
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

    (OUT / "expected_simple_choker.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    line, skin, blue, soft = (R.working(q) for q in (LINE, SKIN, BAND, SOFT))
    half = 128 / 255
    scaled = lambda p, a: [v / p[3] * a for v in p[:3]] + [a]  # noqa: E731  straight colour kept
    mixed = lambda parts, a=1.0: [sum(k * q[j] for q, k in parts) / sum(k for _, k in parts)  # noqa: E731
                                  * a for j in range(3)] + [a]
    covered = lambda f: {(x, y) for y in range(H) for x in range(W) if f[at(x, y)][3] > 0}  # noqa: E731

    # The rule's own pieces.
    assert len(disc(0.5)) == 1 and len(disc(1)) == 5 and len(disc(1.5)) == 9
    assert len(disc(2)) == 13 and (2, 1) in disc(2.5) and (2, 2) not in disc(2.5)
    assert growth(-2) == 2 and growth(-1.5) == 1 and growth(-0.5) == 0 and growth(3) == 0
    # The disc's edge decides yes or no from the choke: every choke the fixtures work, keyed
    # in-between frames included, is a whole number or sits well clear of a whole R^2.
    for fx, (_, cc, frames) in CASES.items():
        for f in frames:
            r2 = held(cc, f) ** 2
            assert r2 == round(r2) or abs(r2 - round(r2)) > 1e-5, (fx, f)

    for fx in ("FX-CHOKE-001", "FX-CHOKE-004", "FX-CHOKE-005"):
        assert all(f == drawn for f in c[fx].values()), fx

    two = c["FX-CHOKE-002"]["0"]
    for x in range(4, 11):
        for y in (2, 7):
            assert two[at(x, y)] == EMPTY  # the box's top and bottom line
    for y in range(2, 8):
        assert two[at(4, y)] == EMPTY and (y == 4 or two[at(10, y)] == EMPTY)
    assert near([two[at(10, 4)]], [scaled(line, half)])  # beside the soft pixel: half, its colour
    for xy in ((11, 4), (13, 8), (6, 4), (8, 4), (7, 3), (7, 5)):
        assert two[at(*xy)] == EMPTY, xy
    assert {(x, y) for (x, y) in covered(two) if x <= 2} == {(1, 7), (1, 8)}
    assert two[at(1, 7)] == blue and two[at(1, 8)] == blue
    assert two[at(8, 6)] == drawn[at(8, 6)]  # the half-covered skin, unchanged
    for xy in ((7, 6), (9, 6), (8, 5)):
        assert near([two[at(*xy)]], [scaled(skin, half)]), xy
    assert all(two[at(x, y)] == skin for x, y in ((5, 3), (9, 5), (6, 6), (5, 5)))
    assert all(two[i] == EMPTY for i in range(W * H) if drawn[i][3] == 0)

    three = c["FX-CHOKE-003"]["0"]
    assert three[at(10, 4)] == EMPTY and three[at(1, 7)] == blue and three[at(1, 8)] == blue
    assert {(x, y) for (x, y) in covered(three) if x <= 2} == {(1, 7), (1, 8)}
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            assert three[at(7 + dx, 4 + dy)] == EMPTY
    assert all(three[i][3] <= two[i][3] for i in range(W * H)) and three != two

    six = c["FX-CHOKE-006"]["0"]
    assert near([six[at(7, 4)]], [skin])  # the hole filled
    for y in (6, 7):
        assert near([six[at(3, y)]], [mixed([(blue, 1), (line, 1)])]), y
    assert near([six[at(12, 4)]], [scaled(line, half)])
    for xy in ((12, 8), (14, 8), (13, 7), (13, 9)):
        assert near([six[at(*xy)]], [line]), xy  # the speck's cross
    for xy in ((12, 7), (14, 9)):
        assert six[at(*xy)] == EMPTY, xy
    assert near([six[at(11, 4)]], [line]) and near([six[at(8, 6)]], [skin])
    for i in range(W * H):
        if drawn[i][3] > 0:  # a covered pixel keeps its own colour, at no less covering
            assert near([six[i]], [scaled(drawn[i], six[i][3])]) and six[i][3] >= drawn[i][3]
    assert six[at(3, 1)] == EMPTY and six[at(15, 8)] == EMPTY

    seven = c["FX-CHOKE-007"]["0"]
    assert near([seven[at(3, 1)]], [line])
    assert near([seven[at(3, 8)]], [mixed([(blue, 3), (line, 1)])])
    assert near([six[at(3, 8)]], [blue])
    assert all(seven[i][3] >= six[i][3] for i in range(W * H))

    eight = c["FX-CHOKE-008"]["0"]
    assert all(eight[at(x, 0)][3] == 1 for x in range(4, 11))
    assert eight[at(2, 0)] == EMPTY and eight[at(12, 0)] == EMPTY
    assert all(eight[at(12, y)][3] == 1 for y in range(2, 8))
    assert all(eight[at(3, y)][3] == 1 for y in range(1, 10)) and eight[at(3, 0)] == EMPTY  # the gap filled
    assert all(eight[at(2, y)][3] == 1 for y in range(2, 10))

    nine = c["FX-CHOKE-009"]
    assert nine["0"] == drawn and nine["2"] == three
    assert all(p == EMPTY for p in nine["4"])

    ten = c["FX-CHOKE-010"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, -100)), 2) < -100
    assert ten["0"] == drawn and ten["2"] == ten["4"]
    layer = drawn_layer("matte")
    whole = [sum(p[j] for p in layer["px"]) / sum(p[3] for p in layer["px"]) for j in range(3)]
    for i, p in enumerate(ten["4"]):
        assert p[3] == 1
        if drawn[i][3] == 0:
            assert near([p], [whole + [1.0]]), i
        else:
            assert near([p], [scaled(drawn[i], 1.0)]), i

    assert all(p == EMPTY for p in c["FX-CHOKE-011"]["0"])

    twelve, thirteen = c["FX-CHOKE-012"]["0"], c["FX-CHOKE-013"]["0"]
    for x in range(3, W):
        for y in range(H):
            assert twelve[at(x, y)] == two[at(x - 3, y)]
            assert thirteen[at(x, y)] == eight[at(x - 3, y)]
    assert all(twelve[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert all(thirteen[at(0, y)] == EMPTY for y in range(H))
    assert all(thirteen[at(x, y)][3] == 1 for x in (1, 2) for y in range(6, 10))  # grown, shown

    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
