"""Brightness & Contrast, worked a second way.

D-135 adds `core.brightness_contrast`. It lightens or darkens a drawing, and pushes its tones
apart from the middle or draws them together toward it: `brightness`, -150 to 150, starting at
0, lifts every channel of every shown pixel by that many levels of 255 (or lowers it, below 0),
and `contrast`, -100 to 100, starting at 0, stretches the tones away from half brightness
(above 0) or squeezes them toward it (below 0). At contrast -100 every shown pixel is flat mid
grey; at 100 every channel below half goes to black and every one above to white, all but the
channels a hair from half. The covering is kept, so a soft edge stays soft, and a pixel that
does not show stays as it is. With both at 0 it changes nothing. It is this program's own
method, modelled on After Effects' Brightness & Contrast; nothing is ported. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. With brightness and contrast both 0 the output is the input, exactly. Otherwise, at a
pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its encoded straight
colour: k = 1 + contrast / 100 when contrast <= 0, else k = 1 / (1 - 0.99 contrast / 100); per
channel e'_c = (e_c - 0.5) k + 0.5 + brightness / 255, and the output is
(srgb_to_linear(clamp(e'_c, 0, 1)) * a, a). Contrast works first, about half brightness, and
brightness is added after. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: tones from black to white and four colours, drawn below. The drawing goes into
`Fixtures/brightness_contrast/media`, the projects into `Fixtures/brightness_contrast`, and the
expected frames into `Fixtures/brightness_contrast/expected_brightness_contrast.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/brightness_contrast_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "brightness_contrast"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("brightness", "contrast")
RANGES = {"brightness": (-150, 150), "contrast": (-100, 100)}


# --- the rule -------------------------------------------------------------------------------

def slope(contrast):
    """How far the tones are pushed from half brightness: 0 at -100, 1 at 0, 100 at 100."""
    return 1 + contrast / 100 if contrast <= 0 else 1 / (1 - 0.99 * contrast / 100)


def grade(e, brightness, contrast):
    """One encoded straight colour, before the clamp."""
    k = slope(contrast)
    return [(v - 0.5) * k + 0.5 + brightness / 255 for v in e]


def brightness_contrast(pixels, brightness, contrast):
    out = [R.working(p) for p in pixels]
    if brightness == 0 and contrast == 0:
        return out  # the build exits early
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a
                     for v in grade(e, brightness, contrast)]
    return out


# --- the drawing ----------------------------------------------------------------------------

BLACK = (0, 0, 0, 255)
LINE = R.LINE                    # #1e1a24, the line
DARK = (64, 64, 64, 255)         # #404040
GREY = (128, 128, 128, 255)      # #808080, a hair above half
LIGHT = (192, 192, 192, 255)     # #c0c0c0
SKIN = R.SKIN                    # #f6d6be
WHITE = (255, 255, 255, 255)
SHADE = (220, 160, 140, 255)     # #dca08c, the skin in shadow
BAND = (58, 111, 216, 255)       # #3a6fd8, a blue
BOW = (214, 40, 60, 255)         # #d6283c, a red
CHEEK = (240, 150, 150, 255)     # #f09696, a pink
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
FAINT = (58, 111, 216, 64)       # the blue at a quarter covering
NONE = S.NONE
COLUMNS = [NONE, BLACK, LINE, DARK, GREY, LIGHT, SKIN, WHITE, SHADE, BAND, BOW, CHEEK,
           SOFT, FAINT, NONE, NONE]


def tones(x, y):
    """Rows 0 and 9 and columns 0, 14 and 15 are empty. Between, one column each: black 1, the
    line 2, dark grey 3, grey 4, light grey 5, skin 6, white 7, the shaded skin 8, blue 9, red
    10, pink 11, the skin at half covering 12 and the blue at a quarter covering 13."""
    return NONE if y in (0, 9) else COLUMNS[x]


DRAWINGS = {"tones": [[tones(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(brightness=0, contrast=0, shift=0):
    return {"drawing": "tones", "brightness": brightness, "contrast": contrast, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(brightness_contrast(pixels, *(held(c, k, frame_no) for k in NAMES)),
                   c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


CASES = {
    "FX-BRICON-001": ("Brightness 0 and contrast 0, the settings as they start: the drawing, "
                      "untouched.", case(), [0]),
    "FX-BRICON-002": ("Brightness 50: every channel of every shown pixel is lifted 50 levels "
                      "of 255 as it is written: black turns #323232, the grey #b2b2b2; the "
                      "white, already full, is held there and stays white, and the skin's red "
                      "and green are held at full while its blue rises to 240.",
                      case(brightness=50), [0]),
    "FX-BRICON-003": ("Brightness -50: every channel lowered 50 levels: white turns #cdcdcd, "
                      "the grey #4e4e4e; black, and the line's channels, all below 50, are "
                      "held at 0, so both turn black.",
                      case(brightness=-50), [0]),
    "FX-BRICON-004": ("Brightness 150, the most: black turns #969696, and every channel at or "
                      "above 105 goes to full, so the grey, the light grey, the skin, the "
                      "shaded skin and the pink all turn white; the blue keeps a little of "
                      "its red (58 + 150 = 208).",
                      case(brightness=150), [0]),
    "FX-BRICON-005": ("Brightness -150, the least: white turns #696969 (105), and every "
                      "channel at or below 150 goes to 0, so the tones up to the grey turn "
                      "black and the red keeps only some of its red (214 - 150 = 64).",
                      case(brightness=-150), [0]),
    "FX-BRICON-006": ("Contrast 50: the tones pushed apart from half by 1 / 0.505, about 1.98: "
                      "the dark grey darkens, the light grey lightens, the grey, a hair above "
                      "half, moves a hair up, black and white stay, and the colours grow "
                      "stronger.",
                      case(contrast=50), [0]),
    "FX-BRICON-007": ("Contrast -50: the tones drawn half way to half brightness: black turns "
                      "#404040 but for rounding (63.75), white #bfbfbf (191.25), and the "
                      "colours grow duller.",
                      case(contrast=-50), [0]),
    "FX-BRICON-008": ("Contrast -100, the least: every shown pixel, whatever its colour, turns "
                      "flat mid grey, #808080 but for rounding (127.5), at its own covering.",
                      case(contrast=-100), [0]),
    "FX-BRICON-009": ("Contrast 100, the most: the tones pushed apart 100 times: every channel "
                      "below half goes to 0 and every one above to full, so the colours turn "
                      "the pure colours they lean to, the blue #0000ff and the skin #ffffff; "
                      "only the grey, half a level above half, stays between, at 0.5 + 0.5 "
                      "/ 255 * 100 = 0.696.",
                      case(contrast=100), [0]),
    "FX-BRICON-010": ("Brightness 30 and contrast 40: contrast first, about half, then "
                      "brightness added: the grey turns 0.5 + (0.5 / 255) / 0.604 + 30 / 255 "
                      "of its scale.",
                      case(brightness=30, contrast=40), [0]),
    "FX-BRICON-011": ("Brightness -100 and contrast -100: every shown pixel turns the one flat "
                      "dark grey 0.5 - 100 / 255 of the scale (27.5 levels).",
                      case(brightness=-100, contrast=-100), [0]),
    "FX-BRICON-012": ("Brightness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                      "untouched, frame 2 FX-BRICON-002 (brightness 50), frame 4 brightness "
                      "100.",
                      case(brightness=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-BRICON-013": ("Contrast keyed from -100 at frame 0 to 100 at frame 4: frame 0 "
                      "FX-BRICON-008, frame 2, both at 0, the drawing untouched, frame 4 "
                      "FX-BRICON-009.",
                      case(contrast=keyed((0, -100), (4, 100))), [0, 2, 4]),
    "FX-BRICON-014": ("Brightness eased from 0 at frame 0 to 150 at frame 4 on a curve that "
                      "overshoots: at frame 2 it has gone past 150 and is held there, so frames "
                      "2 and 4 are both FX-BRICON-004.",
                      case(brightness=keyed((0, 0, OVERSHOOT), (4, 150))), [0, 2, 4]),
    "FX-BRICON-015": ("Brightness held at 50 from frame 0 and keyed to -50 at frame 3: frames "
                      "0 and 2 are FX-BRICON-002, frame 4 FX-BRICON-003.",
                      case(brightness=keyed((0, 50, "hold"), (3, -50))), [0, 2, 4]),
    "FX-BRICON-016": ("FX-BRICON-010 moved three pixels right: the same, moved.",
                      case(brightness=30, contrast=40, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BRICON-017": ("Brightness 151, above 150.", case(brightness=151)),
    "FX-BRICON-018": ("Brightness -151, below -150.", case(brightness=-151)),
    "FX-BRICON-019": ("Contrast 101, above 100.", case(contrast=101)),
    "FX-BRICON-020": ("Contrast -101, below -100.", case(contrast=-101)),
    "FX-BRICON-021": ("Contrast keyed to 120 at frame 4.",
                      case(contrast=keyed((0, 0), (4, 120)))),
    "FX-BRICON-022": ("Brightness keyed from -160 at frame 0.",
                      case(brightness=keyed((0, -160), (4, 0)))),
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
        "instance_id": "fx-0-0", "type_id": "core.brightness_contrast", "enabled": True,
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

    (OUT / "expected_brightness_contrast.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    lin = lambda *e8: [srgb_to_linear(v / 255) for v in e8] + [1.0]  # noqa: E731
    col = {q: (COLUMNS.index(q), 4) for q in (BLACK, LINE, DARK, GREY, LIGHT, SKIN, WHITE,
                                             SHADE, BAND, BOW, CHEEK, SOFT, FAINT)}
    px = lambda f, q: f[at(*col[q])]  # noqa: E731
    same = lambda f, q: near(px(f, q), px(drawn, q))  # noqa: E731
    shown = (BLACK, LINE, DARK, GREY, LIGHT, SKIN, WHITE, SHADE, BAND, BOW, CHEEK)
    empty = [(0, 4), (14, 4), (15, 4), (4, 0), (4, 9)]

    # The rule's own pieces: the slope is 0, 1 and 100 at the ends and the middle, and meets
    # itself at 0 from both sides; contrast turns about half, brightness adds levels of 255.
    assert slope(-100) == 0 and slope(0) == 1 and abs(slope(100) - 100) < 1e-9
    assert abs(slope(50) - 1 / 0.505) < 1e-12 and slope(-50) == 0.5
    assert abs(slope(1e-9) - 1) < 1e-9
    assert grade((0.5, 0.5, 0.5), 0, 100) == [0.5, 0.5, 0.5]
    assert grade((0.2, 0.5, 0.9), 0, 0) == [0.2, 0.5, 0.9]
    assert near(grade((0, 1, 0.5), 51, -100), [0.7, 0.7, 0.7], 1e-12)

    # A pixel that does not show, and the covering, are kept by every case; the soft skin and
    # the faint blue are graded as the skin and the blue, at their own covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-BRICON-016" else 0))
        for f in frames.values():
            for i in range(W * H):
                assert f[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert f[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= f[i][3] + 1e-12 for v in f[i][:3]), (fx, i)
            if fx != "FX-BRICON-016":
                assert near([v / (128 / 255) for v in px(f, SOFT)[:3]], px(f, SKIN)[:3])
                assert near([v / (64 / 255) for v in px(f, FAINT)[:3]], px(f, BAND)[:3])
            if fx != "FX-BRICON-016":
                for f2 in frames.values():
                    assert all(f2[at(*xy)] == [0.0] * 4 for xy in empty)

    assert c["FX-BRICON-001"]["0"] == drawn
    two = c["FX-BRICON-002"]["0"]
    assert near(px(two, BLACK), lin(50, 50, 50)) and near(px(two, GREY), lin(178, 178, 178))
    assert px(two, WHITE) == px(drawn, WHITE) and near(px(two, SKIN), lin(255, 255, 240))
    assert all(px(two, q)[k] >= px(drawn, q)[k] for q in shown for k in range(3))
    three = c["FX-BRICON-003"]["0"]
    assert near(px(three, WHITE), lin(205, 205, 205)) and near(px(three, GREY), lin(78, 78, 78))
    assert px(three, BLACK) == px(drawn, BLACK) and px(three, LINE) == px(drawn, BLACK)
    four = c["FX-BRICON-004"]["0"]
    assert near(px(four, BLACK), lin(150, 150, 150)) and near(px(four, BAND), lin(208, 255, 255))
    assert all(px(four, q) == px(drawn, WHITE) for q in (GREY, LIGHT, SKIN, SHADE, CHEEK, WHITE))
    five = c["FX-BRICON-005"]["0"]
    assert near(px(five, WHITE), lin(105, 105, 105)) and near(px(five, BOW), lin(64, 0, 0))
    assert all(px(five, q) == px(drawn, BLACK) for q in (BLACK, LINE, DARK, GREY))
    six = c["FX-BRICON-006"]["0"]
    k = slope(50)
    assert near(px(six, DARK), lin(*[255 * ((64 / 255 - 0.5) * k + 0.5)] * 3))
    assert px(six, DARK)[0] < px(drawn, DARK)[0] and px(six, LIGHT)[0] > px(drawn, LIGHT)[0]
    assert 0 < px(six, GREY)[0] - px(drawn, GREY)[0] < 1e-2
    assert same(six, BLACK) and same(six, WHITE)
    assert px(six, BAND)[2] > px(drawn, BAND)[2] and px(six, BAND)[0] < px(drawn, BAND)[0]
    seven = c["FX-BRICON-007"]["0"]
    assert near(px(seven, BLACK), lin(63.75, 63.75, 63.75))
    assert near(px(seven, WHITE), lin(191.25, 191.25, 191.25))
    assert px(seven, BOW)[0] < px(drawn, BOW)[0] and px(seven, BOW)[1] > px(drawn, BOW)[1]
    eight = c["FX-BRICON-008"]["0"]
    assert all(near(px(eight, q), lin(127.5, 127.5, 127.5)) for q in shown)
    nine = c["FX-BRICON-009"]["0"]
    assert near(px(nine, BAND), lin(0, 0, 255)) and px(nine, SKIN) == px(drawn, WHITE)
    assert near(px(nine, BOW), lin(255, 0, 0)) and px(nine, DARK) == px(drawn, BLACK)
    assert near(px(nine, GREY), lin(*[255 * (0.5 + 0.5 / 255 * slope(100))] * 3))
    assert all(v in (0.0, 1.0) for q in shown if q != GREY for v in px(nine, q))
    ten = c["FX-BRICON-010"]["0"]
    g = 0.5 + (0.5 / 255) * slope(40) + 30 / 255
    assert abs(slope(40) - 1 / 0.604) < 1e-12 and near(px(ten, GREY), lin(*[255 * g] * 3))
    eleven = c["FX-BRICON-011"]["0"]
    assert all(near(px(eleven, q), lin(27.5, 27.5, 27.5)) for q in shown)
    twelve = c["FX-BRICON-012"]
    assert twelve["0"] == drawn and like(twelve["2"], two)
    assert like(twelve["4"], render(case(brightness=100), 0))
    thirteen = c["FX-BRICON-013"]
    assert like(thirteen["0"], eight) and thirteen["2"] == drawn and like(thirteen["4"], nine)
    fourteen = c["FX-BRICON-014"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(keyed((0, 0, OVERSHOOT), (4, 150)), 2) > 150
    assert fourteen["0"] == drawn and fourteen["2"] == four and fourteen["4"] == four
    fifteen = c["FX-BRICON-015"]
    assert fifteen["0"] == two and fifteen["2"] == two and fifteen["4"] == three
    moved = c["FX-BRICON-016"]["0"]
    assert moved == c["FX-BRICON-016"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == ten[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
