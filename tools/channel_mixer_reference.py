"""Channel Mixer, worked a second way.

D-139 adds `core.channel_mixer`. It rebuilds each of a drawing's red, green and blue from a mix
of all three, as a colourist swaps channels, turns a picture to grey by chosen weights, or
pushes one channel with a constant: each of `red`, `green` and `blue` is four numbers, how much
of the pixel's red, green and blue go into that channel and a constant added to it, each -200 to
200, in per cent of the channel's whole scale. They start at 100, 0, 0, 0 (red from red alone),
0, 100, 0, 0 and 0, 0, 100, 0, which change nothing. `monochrome`, "off" or "on" (off), makes
every channel take the red row, so the picture turns to grey by the red row's weights. The
covering is kept, so a soft edge stays soft, and a pixel that does not show stays as it is. It
is this program's own method, modelled on After Effects' Channel Mixer; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. With the three rows at their starting values and monochrome off, the output is the
input, exactly. Otherwise, at a pixel with covering a > 0, with
e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its encoded straight colour and row_c the row of
channel c (the red row for every channel when monochrome is on):
e'_c = (row_c[0] e_r + row_c[1] e_g + row_c[2] e_b + row_c[3]) / 100, and the output is
(srgb_to_linear(clamp(e'_c, 0, 1)) * a, a). Each row is keyable as one value of four numbers,
each held inside -200..200 at each frame. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: swatches, drawn below. The drawing goes into `Fixtures/channel_mixer/media`, the
projects into `Fixtures/channel_mixer`, and the expected frames into
`Fixtures/channel_mixer/expected_channel_mixer.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/channel_mixer_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "channel_mixer"
TOLERANCE = 2e-5  # document 25's default for a filter
ROWS = ("red", "green", "blue")
NAMES = ROWS + ("monochrome",)
RANGE = (-200, 200)
START = {"red": (100, 0, 0, 0), "green": (0, 100, 0, 0), "blue": (0, 0, 100, 0)}


# --- the rule -------------------------------------------------------------------------------

def mix(e, red, green, blue, monochrome):
    """One encoded straight colour, mixed; not yet held inside 0..1."""
    rows = (red, red, red) if monochrome == "on" else (red, green, blue)
    return [(r[0] * e[0] + r[1] * e[1] + r[2] * e[2] + r[3]) / 100 for r in rows]


def channel_mixer(pixels, red, green, blue, monochrome):
    """`pixels` are 8-bit straight RGBA."""
    out = [R.working(p) for p in pixels]
    if monochrome == "off" and all(list(v) == list(START[k])
                                   for k, v in zip(ROWS, (red, green, blue))):
        return out  # the build exits early
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a
                     for v in mix(e, red, green, blue, monochrome)]
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                    # #1e1a24
SOFT = R.SOFT                    # the line at half covering, its antialiased edge
SKIN = R.SKIN                    # #f6d6be
SOFT_SKIN = SKIN[:3] + (128,)    # the skin at half covering
TRACE = R.TRACE                  # a red, #c82828
GREEN = (60, 180, 80, 255)       # a green, #3cb450
BLUE = (64, 96, 255, 255)        # a blue, #4060ff
WHITE = (255, 255, 255, 255)
GREY = (128, 128, 128, 255)      # #808080
BLACK = (0, 0, 0, 255)
NONE = S.NONE
FILL = {3: SKIN, 4: SKIN, 5: TRACE, 6: TRACE, 7: GREEN, 8: GREEN, 9: BLUE, 10: BLUE,
        11: WHITE, 12: GREY, 13: BLACK}


def swatches(x, y):
    """A box of line in columns 2 to 14 and rows 1 to 8 holding, in rows 2 to 7, skin in
    columns 3 and 4, red in 5 and 6, green in 7 and 8, blue in 9 and 10, white in 11, grey in 12
    and black in 13; down its left side, column 1, the line at half covering in rows 1 to 4 and
    the skin at half covering in rows 5 to 8. Column 0, column 15 and rows 0 and 9 are empty."""
    if not (1 <= x <= 14 and 1 <= y <= 8):
        return NONE
    if x == 1:
        return SOFT if y <= 4 else SOFT_SKIN
    if x in (2, 14) or y in (1, 8):
        return LINE
    return FILL[x]


DRAWINGS = {"swatches": [[swatches(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(red=START["red"], green=START["green"], blue=START["blue"], monochrome="off", shift=0):
    return {"drawing": "swatches", "red": red, "green": green, "blue": blue,
            "monochrome": monochrome, "shift": shift}


def held(c, k, frame_no):
    return [min(RANGE[1], max(RANGE[0], v)) for v in value_at(c[k], frame_no)]


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(channel_mixer(pixels, *(held(c, k, frame_no) for k in ROWS),
                                 c["monochrome"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


SWAP = {"red": (0, 0, 100, 0), "blue": (100, 0, 0, 0)}
GREYS = (30, 59, 11, 0)
NEGATIVE = {"red": (-100, 0, 0, 100), "green": (0, -100, 0, 100), "blue": (0, 0, -100, 100)}

CASES = {
    "FX-MIXER-001": ("The settings as they start, red 100, 0, 0, 0, green 0, 100, 0, 0, blue 0, "
                     "0, 100, 0, monochrome off: the drawing, untouched.",
                     case(), [0]),
    "FX-MIXER-002": ("Red 0, 0, 100, 0 and blue 100, 0, 0, 0, red and blue swapped: the red "
                     "turns blue, #2828c8, the blue turns orange, #ff6040, the green #50b43c and "
                     "the skin a pale blue, #bed6f6; the white, the grey and the black stay as "
                     "they are, as does every grey.",
                     case(**SWAP), [0]),
    "FX-MIXER-003": ("Red 0, 0, 0, 0: every pixel loses its red: the red turns #002828, the "
                     "skin #00d6be and the white cyan, #00ffff; the black stays black.",
                     case(red=(0, 0, 0, 0)), [0]),
    "FX-MIXER-004": ("Red 200, 0, 0, 0: every red doubled and held at the top: the line's red "
                     "goes from #1e to #3c, the blue's from #40 to #80, and the grey, the red and "
                     "the skin reach full red; the white and the black stay as they are.",
                     case(red=(200, 0, 0, 0)), [0]),
    "FX-MIXER-005": ("Green 0, 100, 0, 50, a constant: every shown pixel's green is raised half "
                     "the scale, held at the top: the black turns #008000 but for rounding "
                     "(127.5), the grey's green is full, and the white stays white.",
                     case(green=(0, 100, 0, 50)), [0]),
    "FX-MIXER-006": ("Each row taking its own channel away from a constant of 100, the negative: "
                     "the white turns black and the black white, the grey #7f7f7f, the red cyan, "
                     "#37d7d7, and the line a pale #e1e5db.",
                     case(**NEGATIVE), [0]),
    "FX-MIXER-007": ("Monochrome on, the rows as they start: every channel takes the red row, "
                     "so every pixel turns the grey of its own red: the red #c8c8c8, the green "
                     "#3c3c3c, the blue #404040, the skin #f6f6f6; the white, the grey and the "
                     "black stay as they are.",
                     case(monochrome="on"), [0]),
    "FX-MIXER-008": ("Monochrome on, red 30, 59, 11, 0: every pixel turns grey by those "
                     "weights, the red #585858 (88), the green 133 and the skin 220.96 of 255; "
                     "the weights add up to 100, so the white, the grey and the black stay as "
                     "they are.",
                     case(red=GREYS, monochrome="on"), [0]),
    "FX-MIXER-009": ("FX-MIXER-008 with green 0, 0, 0, 0 and blue 200, -200, 0, 50: with "
                     "monochrome on the green and blue rows are not used, so the same.",
                     case(red=GREYS, green=(0, 0, 0, 0), blue=(200, -200, 0, 50),
                          monochrome="on"), [0]),
    "FX-MIXER-010": ("Green 100, 0, 0, 0, green from red: every pixel's green becomes its red: "
                     "the red turns yellow, #c8c828, the green #3c3c50, the blue #4040ff and the "
                     "skin #f6f6be; every grey stays as it is.",
                     case(green=(100, 0, 0, 0)), [0]),
    "FX-MIXER-011": ("Red 200, -200, 0, 0, the ends of the range: the red is twice the red less "
                     "the green, held inside the scale: the red swatch keeps a full red, #ff2828, "
                     "the green and the blue lose all their red, the skin's red is 64 (#40d6be) "
                     "and every grey loses its red: the white turns cyan, #00ffff.",
                     case(red=(200, -200, 0, 0)), [0]),
    "FX-MIXER-012": ("Red keyed from 100, 0, 0, 0 at frame 0 to 0, 0, 100, 0 at frame 4 and "
                     "blue from 0, 0, 100, 0 to 100, 0, 0, 0, linear, one key holding all four "
                     "numbers: frame 0 untouched, frame 2 red and blue both 50, 0, 50, 0, so the "
                     "red swatch turns purple, #782878, and every grey stays as it is; frame 4 "
                     "is FX-MIXER-002.",
                     case(red=keyed((0, START["red"]), (4, SWAP["red"])),
                          blue=keyed((0, START["blue"]), (4, SWAP["blue"]))), [0, 2, 4]),
    "FX-MIXER-013": ("Green eased from 0, 100, 0, 0 at frame 0 to 0, 100, 0, 200 at frame 4 on "
                     "a curve that overshoots: frame 0 untouched; at frame 2 the constant has "
                     "gone past 200 (265) and is held there, so frames 2 and 4 alike give every "
                     "shown pixel a full green.",
                     case(green=keyed((0, START["green"], OVERSHOOT), (4, (0, 100, 0, 200)))),
                     [0, 2, 4]),
    "FX-MIXER-014": ("Red keyed from 50, 0, 0, 0 at frame 0 to 150, 0, 0, 0 at frame 4: frame 0 "
                     "halves every red, frame 2, the rows as they start, is the drawing "
                     "untouched, and frame 4 raises every red by half, held at the top.",
                     case(red=keyed((0, (50, 0, 0, 0)), (4, (150, 0, 0, 0)))), [0, 2, 4]),
    "FX-MIXER-015": ("Monochrome on, red keyed from 100, 0, 0, 0 at frame 0 to 30, 59, 11, 0 at "
                     "frame 4: frame 0 is FX-MIXER-007, frame 4 FX-MIXER-008, and frame 2 lies "
                     "half way, the weights 65, 29.5, 5.5, 0.",
                     case(red=keyed((0, START["red"]), (4, GREYS)), monochrome="on"), [0, 2, 4]),
    "FX-MIXER-016": ("FX-MIXER-002 moved three pixels right: the same, moved.",
                     case(shift=3, **SWAP), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-MIXER-017": ("Red 201, 0, 0, 0: a number above 200.", case(red=(201, 0, 0, 0))),
    "FX-MIXER-018": ("Green 0, -250, 0, 0: a number below -200.", case(green=(0, -250, 0, 0))),
    "FX-MIXER-019": ("Green 0, 100, 0, -201: a constant below -200.",
                     case(green=(0, 100, 0, -201))),
    "FX-MIXER-020": ("Blue written with three numbers, 0, 0, 100: a row is four.",
                     case(blue=(0, 0, 100))),
    "FX-MIXER-021": ("Red keyed to 0, 0, 300, 0 at frame 4.",
                     case(red=keyed((0, START["red"]), (4, (0, 0, 300, 0))))),
    "FX-MIXER-022": ("Monochrome \"yes\", not \"off\" or \"on\".", case(monochrome="yes")),
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
        "instance_id": "fx-0-0", "type_id": "core.channel_mixer", "enabled": True,
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

    (OUT / "expected_channel_mixer.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    lin = lambda *e8: [srgb_to_linear(v / 255) for v in e8] + [1.0]  # noqa: E731
    is8 = lambda f, xy, *e8: near(f[at(*xy)], lin(*e8))  # noqa: E731
    same = lambda f, xy: near(f[at(*xy)], drawn[at(*xy)])  # noqa: E731
    line, skin, red, green, blue, white, grey, black = (2, 4), (3, 4), (5, 4), (7, 4), (9, 4), \
        (11, 4), (12, 4), (13, 4)
    greys = (white, grey, black)
    soft_line, soft_skin = (1, 2), (1, 6)
    empty = [(0, 4), (15, 4), (4, 0), (4, 9)]

    # The rule's own pieces: the starting rows give each channel back; monochrome takes the red
    # row for all three; the constant is in per cent of the scale.
    e = (0.2, 0.5, 0.9)
    assert mix(e, *START.values(), "off") == [0.2, 0.5, 0.9]
    assert mix(e, *START.values(), "on") == [0.2, 0.2, 0.2]
    assert mix(e, (0, 0, 0, 50), (0, 0, 0, -200), (0, 0, 100, 0), "off") == [0.5, -2, 0.9]
    assert mix((1, 1, 1), (-200, 200, 200, 0), *list(START.values())[1:], "off")[0] == 2

    # A pixel that does not show, and the covering, are kept by every case; a soft pixel is
    # mixed as its colour at full covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-MIXER-016" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx != "FX-MIXER-016":
                for s, full in ((soft_line, line), (soft_skin, skin)):
                    assert near([v / (128 / 255) for v in px[at(*s)][:3]], px[at(*full)][:3])
                assert all(px[at(*xy)] == [0.0] * 4 for xy in empty)

    assert c["FX-MIXER-001"]["0"] == drawn
    two = c["FX-MIXER-002"]["0"]
    assert is8(two, red, 40, 40, 200) and is8(two, blue, 255, 96, 64)
    assert is8(two, green, 80, 180, 60) and is8(two, skin, 190, 214, 246)
    assert is8(two, line, 36, 26, 30) and all(same(two, xy) for xy in greys)
    three = c["FX-MIXER-003"]["0"]
    assert is8(three, red, 0, 40, 40) and is8(three, skin, 0, 214, 190)
    assert is8(three, white, 0, 255, 255) and same(three, black)
    four = c["FX-MIXER-004"]["0"]
    assert is8(four, line, 60, 26, 36) and is8(four, blue, 128, 96, 255)
    assert is8(four, grey, 255, 128, 128) and is8(four, red, 255, 40, 40)
    assert is8(four, skin, 255, 214, 190) and same(four, white) and same(four, black)
    five = c["FX-MIXER-005"]["0"]
    assert is8(five, black, 0, 127.5, 0) and is8(five, grey, 128, 255, 128) and same(five, white)
    assert is8(five, line, 30, 26 + 127.5, 36)
    six = c["FX-MIXER-006"]["0"]
    assert is8(six, white, 0, 0, 0) and is8(six, black, 255, 255, 255)
    assert is8(six, grey, 127, 127, 127) and is8(six, red, 55, 215, 215)
    assert is8(six, line, 225, 229, 219)
    seven = c["FX-MIXER-007"]["0"]
    assert is8(seven, red, 200, 200, 200) and is8(seven, green, 60, 60, 60)
    assert is8(seven, blue, 64, 64, 64) and is8(seven, skin, 246, 246, 246)
    assert all(same(seven, xy) for xy in greys)
    eight = c["FX-MIXER-008"]["0"]
    assert is8(eight, red, 88, 88, 88) and is8(eight, green, 133, 133, 133)
    assert is8(eight, skin, 220.96, 220.96, 220.96)
    assert all(same(eight, xy) for xy in greys)
    assert c["FX-MIXER-009"]["0"] == eight
    ten = c["FX-MIXER-010"]["0"]
    assert is8(ten, red, 200, 200, 40) and is8(ten, green, 60, 60, 80)
    assert is8(ten, blue, 64, 64, 255) and is8(ten, skin, 246, 246, 190)
    assert all(same(ten, xy) for xy in greys)
    eleven = c["FX-MIXER-011"]["0"]
    assert is8(eleven, red, 255, 40, 40) and is8(eleven, green, 0, 180, 80)
    assert is8(eleven, blue, 0, 96, 255) and is8(eleven, skin, 64, 214, 190)
    assert is8(eleven, white, 0, 255, 255) and is8(eleven, grey, 0, 128, 128)
    twelve = c["FX-MIXER-012"]
    assert twelve["0"] == drawn and twelve["4"] == two
    half = (50, 0, 50, 0)
    assert like(twelve["2"], render(case(red=half, blue=half), 0))
    assert is8(twelve["2"], red, 120, 40, 120)
    thirteen = c["FX-MIXER-013"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(green=keyed(
        (0, START["green"], OVERSHOOT), (4, (0, 100, 0, 200))))["green"], 2)[3] > 200
    assert thirteen["0"] == drawn and thirteen["2"] == thirteen["4"]
    assert thirteen["4"] == render(case(green=(0, 100, 0, 200)), 0)
    for i in range(W * H):
        if drawn[i][3] > 0:
            assert abs(thirteen["4"][i][1] - drawn[i][3]) < 1e-12, i
    fourteen = c["FX-MIXER-014"]
    assert fourteen["2"] == drawn
    assert is8(fourteen["0"], red, 100, 40, 40) and is8(fourteen["4"], red, 255, 40, 40)
    assert is8(fourteen["0"], grey, 64, 128, 128) and is8(fourteen["4"], grey, 192, 128, 128)
    fifteen = c["FX-MIXER-015"]
    assert fifteen["0"] == seven and like(fifteen["4"], eight)
    assert like(fifteen["2"], render(case(red=(65, 29.5, 5.5, 0), monochrome="on"), 0))
    moved = c["FX-MIXER-016"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
