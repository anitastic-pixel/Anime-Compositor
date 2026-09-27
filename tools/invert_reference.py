"""Invert, worked a second way.

D-134 adds `core.invert`. It turns a drawing's colours inside out, as a photographic negative
does: black becomes white, white black, and every colour its opposite, skin a deep blue-green and
red a cyan. `channel` chooses what is turned: "rgb" (all three colours, the start), "red",
"green" or "blue" (that one alone, the other two kept exactly), or "alpha" (the covering itself,
so what showed disappears and what was empty inside the layer turns black). `amount`, 0 to 100
(100), is how far: 100 turns fully, 50 meets in the middle, and 0 changes nothing. The covering
is kept by the colour choices, so a soft edge stays soft, and a pixel that does not show stays
as it is. It is this program's own method, modelled on After Effects' Invert; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. t = amount / 100; at amount 0 the output is the input, exactly. For "rgb", "red",
"green" or "blue", at a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1))
its encoded straight colour, each chosen channel e'_c = e_c + t (1 - 2 e_c), and the output
channel is srgb_to_linear(clamp(e'_c, 0, 1)) * a; the channels not chosen, and the covering, are
left exactly as they are. For "alpha", at every pixel of the layer, a' = a + t (1 - 2 a), the
straight colour b = p.rgb / a kept (black where a = 0), and the output is (b a', a'). The layer
does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of colour, drawn below. The drawing goes into `Fixtures/invert/media`, the
projects into `Fixtures/invert`, and the expected frames into
`Fixtures/invert/expected_invert.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/invert_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "invert"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("channel", "amount")
CHANNELS = {"rgb": (0, 1, 2), "red": (0,), "green": (1,), "blue": (2,), "alpha": ()}


# --- the rule -------------------------------------------------------------------------------

def invert(pixels, channel, amount):
    """The drawing's 8-bit pixels, inverted, as working values."""
    out = [R.working(p) for p in pixels]
    t = amount / 100
    if t == 0:
        return out  # the build exits early
    for w in out:
        a = w[3]
        if channel == "alpha":
            b = [v / a for v in w[:3]] if a > 0 else [0.0] * 3
            a2 = a + t * (1 - 2 * a)
            w[:] = [v * a2 for v in b] + [a2]
        elif a > 0:
            for c in CHANNELS[channel]:
                e = S.linear_to_srgb(min(1.0, max(0.0, w[c] / a)))
                w[c] = srgb_to_linear(min(1.0, max(0.0, e + t * (1 - 2 * e)))) * a
    return out


# --- the drawing ----------------------------------------------------------------------------

BLACK = (0, 0, 0, 255)
LINE = R.LINE                    # #1e1a24, the line
TRACE = R.TRACE                  # #c82828, a red colour-trace line
GREY = (128, 128, 128, 255)      # #808080, a hair above half: it barely moves
SKIN = R.SKIN                    # #f6d6be
BLUE = (58, 111, 216, 255)       # #3a6fd8, the ball's blue band
WHITE = (255, 255, 255, 255)
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
FAINT = (246, 214, 190, 64)      # the skin at a quarter covering, the edge's outer ring
NONE = S.NONE
BANDS = [NONE, BLACK, BLACK, LINE, LINE, TRACE, TRACE, GREY, GREY, SKIN, SKIN, BLUE, BLUE,
         WHITE, WHITE, SOFT]


def bands(x, y):
    """Rows 0 and 9 and column 0 are empty. Between, columns in pairs: black 1-2, line 3-4, red
    5-6, grey 7-8, skin 9-10, blue 11-12, white 13-14; column 15 is the skin at half covering in
    rows 1 to 4 and at a quarter in rows 5 to 8."""
    if y in (0, 9):
        return NONE
    if x == 15 and y >= 5:
        return FAINT
    return BANDS[x]


DRAWINGS = {"bands": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(channel="rgb", amount=100, shift=0):
    return {"drawing": "bands", "channel": channel, "amount": amount, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    amount = min(100, max(0, value_at(c["amount"], frame_no)))
    return R.frame(invert(pixels, c["channel"], amount), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-INVERT-001": ("Channel rgb, amount 100, the settings as they start: every colour that "
                      "shows turns to its opposite, each 8-bit channel v becoming 255 - v: black "
                      "turns white and white black, the line #1e1a24 a pale #e1e5db, the red "
                      "#c82828 a cyan #37d7d7, the grey #808080 a hair darker, #7f7f7f, the skin "
                      "#f6d6be a deep #092941 and the blue #3a6fd8 an ochre #c59027; the soft "
                      "edge turns #092941 at its own covering, and the empty pixels stay empty.",
                      case(), [0]),
    "FX-INVERT-002": ("Amount 0: the drawing, untouched.",
                      case(amount=0), [0]),
    "FX-INVERT-003": ("Amount 50: every channel of every colour that shows meets the middle, so "
                      "every shown pixel turns the same mid grey, #808080 but for rounding "
                      "(127.5), at its own covering.",
                      case(amount=50), [0]),
    "FX-INVERT-004": ("Amount 25: each channel goes a quarter of the way to its opposite, "
                      "e' = e / 2 + 1 / 4: black lifts to #404040 but for rounding (63.75), white "
                      "falls to #c0c0c0 (191.25), and every colour keeps half its contrast.",
                      case(amount=25), [0]),
    "FX-INVERT-005": ("Channel red: only red is inverted, 255 - v, and green and blue are kept "
                      "exactly: the red #c82828 turns nearly black, #372828, the skin a "
                      "blue-green #09d6be, black turns red #ff0000 and white cyan #00ffff.",
                      case(channel="red"), [0]),
    "FX-INVERT-006": ("Channel green: only green is inverted: the skin turns pink #f629be, "
                      "black green #00ff00 and white magenta #ff00ff.",
                      case(channel="green"), [0]),
    "FX-INVERT-007": ("Channel blue: only blue is inverted: the blue #3a6fd8 turns olive "
                      "#3a6f27, black blue #0000ff and white yellow #ffff00.",
                      case(channel="blue"), [0]),
    "FX-INVERT-008": ("Channel alpha, amount 100: the covering is inverted, 1 - a, the colour "
                      "kept: every pixel that fully showed disappears, every empty pixel inside "
                      "the layer turns black and fully covering, the half-covered skin keeps its "
                      "colour at 127/255 and the quarter-covered skin at 191/255.",
                      case(channel="alpha"), [0]),
    "FX-INVERT-009": ("Channel alpha, amount 50: every pixel's covering meets the middle, one "
                      "half exactly, each keeping its own colour; the empty pixels turn black at "
                      "half covering.",
                      case(channel="alpha", amount=50), [0]),
    "FX-INVERT-010": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                      "untouched, frame 2 FX-INVERT-003 and frame 4 FX-INVERT-001.",
                      case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-INVERT-011": ("Channel alpha, amount keyed from 0 at frame 0 to 100 at frame 4, "
                      "linear: frame 0 untouched, frame 2 FX-INVERT-009 and frame 4 "
                      "FX-INVERT-008.",
                      case(channel="alpha", amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-INVERT-012": ("Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                      "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                      "FX-INVERT-001, as is frame 4; frame 0 is the drawing.",
                      case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-INVERT-013": ("Channel alpha, amount 0: the drawing, untouched; the empty pixels stay "
                      "empty.",
                      case(channel="alpha", amount=0), [0]),
    "FX-INVERT-014": ("FX-INVERT-001 moved three pixels right: the same, moved.",
                      case(shift=3), [0, 3]),
    "FX-INVERT-015": ("FX-INVERT-008 moved three pixels right: the same, moved; the three "
                      "columns on the left, outside the layer, stay empty, as only the layer's "
                      "own pixels are inverted.",
                      case(channel="alpha", shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-INVERT-016": ("Amount 101, above 100.", case(amount=101)),
    "FX-INVERT-017": ("Amount -1, below 0.", case(amount=-1)),
    "FX-INVERT-018": ("Channel \"luma\", which is not a choice.", case(channel="luma")),
    "FX-INVERT-019": ("Channel \"RGB\", in capitals, which is kept as written and is not the "
                      "word.", case(channel="RGB")),
    "FX-INVERT-020": ("Amount keyed to 150 at frame 4.", case(amount=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.invert", "enabled": True,
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

    (OUT / "expected_invert.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    src = [p for row in DRAWINGS["bands"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    shown = [i for i in range(W * H) if src[i][3] > 0]
    empty = [i for i in range(W * H) if src[i][3] == 0]

    def graded(f):
        """The drawing with each shown pixel's encoded channels (8-bit over 255) mapped by f,
        at the pixel's own covering."""
        out = [list(p) for p in drawn]
        for i in shown:
            a = src[i][3] / 255
            out[i] = [srgb_to_linear(f(c, src[i][c] / 255)) * a for c in range(3)] + [a]
        return out

    # The rule's own pieces: an inverted 8-bit value is 255 - v, amount 50 meets the middle
    # whatever the colour, and red, then green, then blue, is rgb.
    for v in range(256):
        e = v / 255
        assert abs(e + (1 - 2 * e) - (255 - v) / 255) < 1e-15
        assert abs(e + 0.5 * (1 - 2 * e) - 0.5) < 1e-15
    rgb = invert(src, "rgb", 100)
    steps = [R.working(p) for p in src]
    for ch in ("red", "green", "blue"):
        once = invert(src, ch, 100)
        for i in shown:
            k = CHANNELS[ch][0]
            steps[i][k] = once[i][k]
    assert steps == rgb

    # A colour choice keeps every pixel's covering and leaves the empty pixels empty.
    for fx, frames in c.items():
        v = expected["cases"][fx]
        if "warning" in v or fx in ("FX-INVERT-008", "FX-INVERT-009", "FX-INVERT-011",
                                    "FX-INVERT-015"):
            continue
        base = plain(case(shift=3 if fx == "FX-INVERT-014" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= u <= px[i][3] + 1e-12 for u in px[i][:3]), (fx, i)

    one = c["FX-INVERT-001"]["0"]
    assert like(one, graded(lambda ch, e: 1 - e))
    assert near(one[at(1, 4)], [1, 1, 1, 1]) and near(one[at(13, 4)], [0, 0, 0, 1])
    assert c["FX-INVERT-002"]["0"] == drawn
    three = c["FX-INVERT-003"]["0"]
    assert like(three, graded(lambda ch, e: 0.5))
    assert all(near(three[i][:3], [srgb_to_linear(0.5) * three[i][3]] * 3) for i in shown)
    four = c["FX-INVERT-004"]["0"]
    assert like(four, graded(lambda ch, e: e / 2 + 0.25))
    assert near(four[at(1, 4)], [srgb_to_linear(63.75 / 255)] * 3 + [1])
    assert near(four[at(13, 4)], [srgb_to_linear(191.25 / 255)] * 3 + [1])
    for fx, k in (("FX-INVERT-005", 0), ("FX-INVERT-006", 1), ("FX-INVERT-007", 2)):
        px = c[fx]["0"]
        assert like(px, graded(lambda ch, e: 1 - e if ch == k else e))
        for i in range(W * H):  # the channels not chosen are kept exactly
            assert all(px[i][j] == drawn[i][j] for j in range(4) if j != k), (fx, i)
            assert near([px[i][k]], [one[i][k]])
    assert near(c["FX-INVERT-005"]["0"][at(1, 4)], [1, 0, 0, 1])
    assert near(c["FX-INVERT-005"]["0"][at(13, 4)], [0, 1, 1, 1])
    assert near(c["FX-INVERT-006"]["0"][at(1, 4)], [0, 1, 0, 1])
    assert near(c["FX-INVERT-007"]["0"][at(13, 4)], [1, 1, 0, 1])

    # The covering turned: what showed disappears, what was empty turns black, the soft edge
    # keeps its colour at the opposite covering.
    eight = c["FX-INVERT-008"]["0"]
    for i in range(W * H):
        a = src[i][3] / 255
        assert abs(eight[i][3] - (1 - a)) < 1e-12, i
        if src[i][3] == 255:
            assert eight[i] == [0.0] * 4, i
        elif a == 0:
            assert eight[i] == [0.0, 0.0, 0.0, 1.0], i
        else:
            assert near([u / eight[i][3] for u in eight[i][:3]], drawn[at(9, 4)][:3])
    assert abs(eight[at(15, 2)][3] - 127 / 255) < 1e-12
    assert abs(eight[at(15, 6)][3] - 191 / 255) < 1e-12
    nine = c["FX-INVERT-009"]["0"]
    for i in range(W * H):
        assert abs(nine[i][3] - 0.5) < 1e-12, i
        want = drawn[i][:3] if i in empty else [u / drawn[i][3] for u in drawn[i][:3]]
        assert near([u / 0.5 for u in nine[i][:3]], want), i
    assert c["FX-INVERT-013"]["0"] == drawn

    ten = c["FX-INVERT-010"]
    assert ten["0"] == drawn and ten["2"] == three and ten["4"] == one
    eleven = c["FX-INVERT-011"]
    assert eleven["0"] == drawn and eleven["2"] == nine and eleven["4"] == eight
    twelve = c["FX-INVERT-012"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(keyed((0, 0, OVERSHOOT), (4, 100)), 2) > 100
    assert twelve["0"] == drawn and twelve["2"] == one and twelve["4"] == one

    moved = c["FX-INVERT-014"]["0"]
    assert moved == c["FX-INVERT-014"]["3"]
    shifted = c["FX-INVERT-015"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
        assert shifted[at(3, y):at(0, y + 1)] == eight[at(0, y):at(W - 3, y)]
        assert all(shifted[at(x, y)] == [0.0] * 4 for x in range(3))
    print("checked")


if __name__ == "__main__":
    main()
