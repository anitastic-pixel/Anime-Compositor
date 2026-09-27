"""Threshold, worked a second way.

D-138 adds `core.threshold`. It turns a drawing into pure black and pure white, as a copy
machine or a line test does: each shown pixel whose brightness reaches `level`, a number 0 to 255
starting at 128, turns white, and every darker one turns black. Brightness here is the colour's
encoded luma, so a pure green counts as far brighter than a pure red, and a pure blue as darker
than either. The covering is kept, so a soft edge stays soft, in black or in white, and a pixel
that does not show stays as it is. At level 0 every shown pixel is white; at 255 only white
stays white. It is this program's own method, modelled on After Effects' Threshold; nothing is
ported. Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

The rule. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour and Y(e) = 0.2126 e_r + 0.7152 e_g + 0.0722 e_b its encoded luma: the
output is (a, a, a, a), white, when 255 Y(e) + 1e-4 >= level, and (0, 0, 0, a), black,
otherwise. The 1e-4 decides an 8-bit grey sitting exactly on the level as white, the same way
in the build's single precision as here. A pixel with a == 0 is left as it is. The layer does
not grow. The choice is a cliff: `check` asserts that no pixel it decides sits within 1e-5 of it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of colour from black to white, drawn below. The drawing goes into
`Fixtures/threshold/media`, the projects into `Fixtures/threshold`, and the expected frames into
`Fixtures/threshold/expected_threshold.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/threshold_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "threshold"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGE = (0, 255)
START = 128
NUDGE = 1e-4   # the spec's: an 8-bit value exactly on the level is decided white
CLIFF = 1e-5   # no decided pixel may sit closer than this to the level


# --- the rule -------------------------------------------------------------------------------

def encoded(w):
    """A working pixel's encoded straight colour."""
    return [S.linear_to_srgb(min(1.0, max(0.0, v / w[3]))) for v in w[:3]]


def luma255(e):
    """255 times the encoded luma, Y(e), of an encoded colour."""
    return 255 * (0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2])


def white(e, level):
    return luma255(e) + NUDGE >= level


def threshold(pixels, level):
    out = [R.working(p) for p in pixels]
    for w in out:
        a = w[3]
        if a > 0:
            v = srgb_to_linear(1.0) * a if white(encoded(w), level) else 0.0
            w[:3] = [v, v, v]
    return out


# --- the drawing ----------------------------------------------------------------------------

BLACK = (0, 0, 0, 255)              # luma 0
LINE = R.LINE                       # #1e1a24, luma 27.57
BLUE = (0, 0, 255, 255)             # #0000ff, luma 18.41
RED = (255, 0, 0, 255)              # #ff0000, luma 54.21
DARK = (64, 64, 64, 255)            # #404040, luma 64
DIM = (127, 127, 127, 255)          # #7f7f7f, luma 127
GREY = (128, 128, 128, 255)         # #808080, luma 128: exactly on the starting level
SHADOW = (220, 160, 140, 255)       # #dca08c, the cel's skin in shadow, luma 171.31
GREEN = (0, 255, 0, 255)            # #00ff00, luma 182.38
LIGHT = (192, 192, 192, 255)        # #c0c0c0, luma 192
SKIN = R.SKIN                       # #f6d6be, the lit skin, luma 219.07
WHITE = (255, 255, 255, 255)        # luma 255
SOFT_SKIN = (246, 214, 190, 128)    # the skin at half covering, a soft edge
SOFT_LINE = R.SOFT                  # the line at half covering, its antialiased edge
NONE = S.NONE
BANDS = [NONE, BLACK, BLUE, LINE, RED, DARK, DIM, GREY, SHADOW, GREEN, LIGHT, SKIN, WHITE,
         SOFT_SKIN, SOFT_LINE, NONE]


def bands(x, y):
    """Rows 0 and 9 and columns 0 and 15 are empty. Between, one column each, from dark to
    light: black 1, blue 2, line 3, red 4, dark grey 5, grey 127 6, grey 128 7, shadow skin 8,
    green 9, light grey 10, skin 11, white 12; then the skin at half covering in 13 and the line
    at half covering in 14."""
    return NONE if y in (0, 9) else BANDS[x]


DRAWINGS = {"tones": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(level=START, shift=0):
    return {"drawing": "tones", "level": level, "shift": shift}


def held(c, frame_no):
    return min(RANGE[1], max(RANGE[0], value_at(c["level"], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(threshold(pixels, held(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame([R.working(p) for row in DRAWINGS[c["drawing"]] for p in row], c["shift"])


CASES = {
    "FX-THRESH-001": ("Level 128, the setting as it starts: black, blue, the line, red, the dark "
                      "grey and grey 127 turn black; grey 128, exactly on the level, turns "
                      "white, as do the shadow skin, green, the light grey, the skin and white; "
                      "the soft skin turns white and the soft line black, each at its own half "
                      "covering.",
                      case(), [0]),
    "FX-THRESH-002": ("Level 0: every shown pixel turns white, black too; the soft edges are "
                      "white at half covering.",
                      case(level=0), [0]),
    "FX-THRESH-003": ("Level 255: only white stays white; every other shown pixel, the skin "
                      "and the light grey too, turns black.",
                      case(level=255), [0]),
    "FX-THRESH-004": ("Level 20: only black and blue, the darkest, turn black; the line, "
                      "darker to the eye than red but brighter than blue, turns white.",
                      case(level=20), [0]),
    "FX-THRESH-005": ("Level 50: red, at luma 54, turns white while blue and the line stay "
                      "black: a pure red counts as brighter than a pure blue.",
                      case(level=50), [0]),
    "FX-THRESH-006": ("Level 64: the dark grey sits exactly on the level and turns white; red, "
                      "just below, turns black.",
                      case(level=64), [0]),
    "FX-THRESH-007": ("Level 127: grey 127 is now exactly on the level and turns white, with "
                      "grey 128.",
                      case(level=127), [0]),
    "FX-THRESH-008": ("Level 128.5, a level between two 8-bit steps: grey 128 turns black, "
                      "with grey 127.",
                      case(level=128.5), [0]),
    "FX-THRESH-009": ("Level 172: the shadow skin, at luma 171.3, turns black while the lit "
                      "skin stays white, the cel split into its light and its shadow.",
                      case(level=172), [0]),
    "FX-THRESH-010": ("Level 185: pure green, at luma 182.4, turns black; the light grey, at "
                      "192, stays white.",
                      case(level=185), [0]),
    "FX-THRESH-011": ("Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is "
                      "FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, "
                      "and frame 4 FX-THRESH-003; the pixels turn black darkest first.",
                      case(level=keyed((0, 0), (4, 255))), [0, 1, 2, 3, 4]),
    "FX-THRESH-012": ("Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are "
                      "FX-THRESH-005, frames 3 and 4 FX-THRESH-010.",
                      case(level=keyed((0, 50, "hold"), (3, 185))), [0, 2, 3, 4]),
    "FX-THRESH-013": ("Level eased from 128 at frame 0 to 0 at frame 4 on a curve that "
                      "overshoots: at frame 2 the level has gone below 0 and is held there, so "
                      "frames 2 and 4 are both FX-THRESH-002.",
                      case(level=keyed((0, 128, OVERSHOOT), (4, 0))), [0, 2, 4]),
    "FX-THRESH-014": ("FX-THRESH-001 moved three pixels right: the same, moved.",
                      case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-THRESH-015": ("Level -1, below 0.", case(level=-1)),
    "FX-THRESH-016": ("Level 256, above 255.", case(level=256)),
    "FX-THRESH-017": ("Level 255.5, past 255 by half a step.", case(level=255.5)),
    "FX-THRESH-018": ("Level keyed to 300 at frame 4.", case(level=keyed((0, 128), (4, 300)))),
    "FX-THRESH-019": ("Level written \"128\", a word, not a number.", case(level="128")),
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
        "instance_id": "fx-0-0", "type_id": "core.threshold", "enabled": True,
        "parameters": {"level": setting_json(c["level"])}}]
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

    (OUT / "expected_threshold.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    col = {q: x for x, q in enumerate(BANDS) if q != NONE}
    half = 128 / 255

    def shows(f, name):
        """What column `name` turned to in frame f, on row 4: "white", "black" or neither."""
        p = f[at(col[name], 4)]
        if near(p, [p[3]] * 4):
            return "white"
        return "black" if near(p[:3], [0, 0, 0]) else None

    def whites(f):
        return {q for q in col if shows(f, q) == "white"}

    shown = set(col)
    soft = {SOFT_SKIN, SOFT_LINE}

    # The rule's own pieces: the lumas the drawing is built on, and the nudge on the step.
    L = {q: luma255([v / 255 for v in q[:3]]) for q in col}
    for q, v in ((BLACK, 0), (BLUE, 18.411), (LINE, 27.5724), (RED, 54.213), (DARK, 64),
                 (DIM, 127), (GREY, 128), (SHADOW, 171.312), (GREEN, 182.376), (LIGHT, 192),
                 (SKIN, 219.0704), (WHITE, 255)):
        assert abs(L[q] - v) < 1e-9, (q, L[q])
    assert L[BLUE] < L[LINE] < L[RED] < L[DARK] < L[DIM] < L[GREY] < L[SHADOW] < L[GREEN] \
        < L[LIGHT] < L[SKIN] < L[WHITE]
    assert white((0.5, 0.5, 0.5), 127.5) and white([128 / 255] * 3, 128)
    assert not white([128 / 255] * 3, 128.0002) and white((0, 0, 0), 0)
    assert white((1, 1, 1), 255) and not white((1, 1, 254 / 255), 255)

    # Every case keeps every pixel's covering, leaves the empty pixels empty, and turns every
    # shown pixel to pure white or pure black at its own covering; no decided pixel sits within
    # 1e-5 of the level (the cliff).
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-THRESH-014" else 0))
        cc = (CASES.get(fx) or INVALID.get(fx))[1]
        for f, px in frames.items():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                elif fx in CASES:
                    p = px[i]
                    assert near(p, [p[3]] * 4) or p[:3] == [0.0] * 3, (fx, f, i)
                    level = held(cc, int(f))
                    y = luma255(encoded(base[i]))
                    assert abs(y + NUDGE - level) >= CLIFF, (fx, f, i, y, level)
            if fx in CASES and fx != "FX-THRESH-014":
                assert shows(px, SOFT_SKIN) == shows(px, SKIN)
                assert shows(px, SOFT_LINE) == shows(px, LINE)
                assert px[at(col[SOFT_SKIN], 4)][3] == half == px[at(col[SOFT_LINE], 4)][3]
                for x in range(W):  # every drawn row is the same
                    assert all(px[at(x, y)] == px[at(x, 4)] for y in range(1, H - 1))

    one = c["FX-THRESH-001"]["0"]
    assert whites(one) == {GREY, SHADOW, GREEN, LIGHT, SKIN, WHITE, SOFT_SKIN}
    assert near(one[at(col[SOFT_SKIN], 4)], [half] * 4)
    assert near(one[at(col[SOFT_LINE], 4)], [0, 0, 0, half])
    assert one[at(col[WHITE], 4)] == drawn[at(col[WHITE], 4)]
    assert one[at(col[BLACK], 4)] == drawn[at(col[BLACK], 4)]
    assert whites(c["FX-THRESH-002"]["0"]) == shown
    assert whites(c["FX-THRESH-003"]["0"]) == {WHITE}
    assert shown - whites(c["FX-THRESH-004"]["0"]) == {BLACK, BLUE}
    five = whites(c["FX-THRESH-005"]["0"])
    assert RED in five and BLUE not in five and LINE not in five and SOFT_LINE not in five
    assert shown - five == {BLACK, BLUE, LINE, SOFT_LINE}
    six = whites(c["FX-THRESH-006"]["0"])
    assert DARK in six and RED not in six and L[DARK] == 64
    seven = whites(c["FX-THRESH-007"]["0"])
    assert DIM in seven and GREY in seven and seven == whites(one) | {DIM}
    eight = whites(c["FX-THRESH-008"]["0"])
    assert GREY not in eight and DIM not in eight and eight == whites(one) - {GREY}
    nine = whites(c["FX-THRESH-009"]["0"])
    assert SHADOW not in nine and SKIN in nine and SOFT_SKIN in nine and GREEN in nine
    ten = whites(c["FX-THRESH-010"]["0"])
    assert GREEN not in ten and LIGHT in ten and ten == {LIGHT, SKIN, WHITE, SOFT_SKIN}

    eleven = c["FX-THRESH-011"]
    assert eleven["0"] == c["FX-THRESH-002"]["0"] and eleven["4"] == c["FX-THRESH-003"]["0"]
    assert eleven["2"] == render(case(level=127.5), 0)
    assert GREY in whites(eleven["2"]) and DIM not in whites(eleven["2"])
    for a, b in zip("0123", "1234"):  # black spreads from the darkest up, never back
        assert whites(eleven[b]) < whites(eleven[a])
    twelve = c["FX-THRESH-012"]
    assert twelve["0"] == twelve["2"] == c["FX-THRESH-005"]["0"]
    assert twelve["3"] == twelve["4"] == c["FX-THRESH-010"]["0"]
    thirteen = c["FX-THRESH-013"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(level=keyed(
        (0, 128, OVERSHOOT), (4, 0)))["level"], 2) < 0
    assert thirteen["0"] == one
    assert thirteen["2"] == thirteen["4"] == c["FX-THRESH-002"]["0"]
    moved = c["FX-THRESH-014"]["0"]
    assert moved == c["FX-THRESH-014"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
