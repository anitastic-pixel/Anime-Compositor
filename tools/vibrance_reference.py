"""Vibrance, worked a second way.

D-140 adds `core.vibrance`. It makes a drawing's colours richer or duller, as a colourist wakes
up a flat shot without burning the colours that are already strong: `vibrance` -100 to 100,
starting at 0, moves the dull colours most and leaves the already vivid ones nearly alone, and
`saturation` -100 to 100, starting at 0, moves every colour alike. Each pixel's colour is pushed
away from its own grey (above 0) or towards it (below 0); the grey is the colour's brightness,
so a grey pixel, black or white stays as it is whatever the settings. The covering is kept, so a
soft edge stays soft, and a pixel that does not show stays as it is. With both at 0 it changes
nothing. It is this program's own method, modelled on After Effects' Vibrance; nothing is
ported. Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

The rule. With both settings 0 the output is the input, exactly. Otherwise, at a pixel with
covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its encoded straight colour,
L = 0.2126 e_r + 0.7152 e_g + 0.0722 e_b, s = max(e) - min(e) and
k = 1 + saturation / 100 + vibrance / 100 (1 - s): per channel e'_c = L + (e_c - L) k, and the
output is (srgb_to_linear(clamp(e'_c, 0, 1)) * a, a). The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of colour from grey to pure red, drawn below. The drawing goes into
`Fixtures/vibrance/media`, the projects into `Fixtures/vibrance`, and the expected frames into
`Fixtures/vibrance/expected_vibrance.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/vibrance_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "vibrance"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("vibrance", "saturation")
RANGE = (-100, 100)


# --- the rule -------------------------------------------------------------------------------

def strength(e, vibrance, saturation):
    """How far an encoded colour is pushed from its grey: 1 keeps it, 0 makes it grey."""
    return 1 + saturation / 100 + vibrance / 100 * (1 - (max(e) - min(e)))


def grade(e, vibrance, saturation):
    """One encoded straight colour, made richer or duller (before the clamp)."""
    L = 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]
    k = strength(e, vibrance, saturation)
    return [L + (v - L) * k for v in e]


def vibrance(pixels, vib, sat):
    out = [R.working(p) for p in pixels]
    if vib == 0 and sat == 0:
        return out  # the build exits early
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a for v in grade(e, vib, sat)]
    return out


# --- the drawing ----------------------------------------------------------------------------

GREY = (128, 128, 128, 255)      # #808080: no colour at all, s = 0
LINE = R.LINE                    # #1e1a24, the line: nearly grey, s = 0.04
SKIN = R.SKIN                    # #f6d6be: a dull colour, s = 0.22
TAN = (160, 140, 120, 255)       # #a08c78: a dull tan, the skin in shade, s = 0.16
BAND = (58, 111, 216, 255)       # #3a6fd8: the ball's blue band, vivid, s = 0.62
TRACE = R.TRACE                  # #c82828: a red trace line, vivid, s = 0.63
RED = (255, 0, 0, 255)           # #ff0000: as vivid as a colour can be, s = 1
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
NONE = S.NONE
BANDS = [NONE, GREY, GREY, LINE, LINE, TAN, TAN, SKIN, SKIN, BAND, BAND,
         TRACE, TRACE, RED, RED, SOFT]


def bands(x, y):
    """Rows 0 and 9 and column 0 are empty. Between, columns in pairs from the dullest colour to
    the most vivid: grey 1-2, line 3-4, tan 5-6, skin 7-8, blue 9-10, red trace 11-12, pure red
    13-14, and the skin at half covering in 15."""
    return NONE if y in (0, 9) else BANDS[x]


DRAWINGS = {"colours": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(vibrance=0, saturation=0, shift=0):
    return {"drawing": "colours", "vibrance": vibrance, "saturation": saturation,
            "shift": shift}


def held(c, k, frame_no):
    return min(RANGE[1], max(RANGE[0], value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(vibrance(pixels, *(held(c, k, frame_no) for k in NAMES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


CASES = {
    "FX-VIBRANCE-001": ("Vibrance 0 and saturation 0, the settings as they start: the drawing, "
                        "untouched.", case(), [0]),
    "FX-VIBRANCE-002": ("Vibrance 100: the dull colours wake up most, the tan pushed 1.84 times "
                        "as far from its grey and the skin 1.78 times (its red held at the "
                        "top), the vivid blue and red trace only about 1.38 times (the blue's "
                        "blue held at the top); the pure red, as vivid as can be, is left "
                        "exactly as it is, and the grey is left as it is, having no colour to "
                        "push.", case(vibrance=100), [0]),
    "FX-VIBRANCE-003": ("Vibrance -100: the dull colours fade most, the tan to a near grey "
                        "(0.16 of its colour left) and the skin to 0.22, while the blue and red "
                        "trace keep about 0.62 of theirs and the pure red keeps all of it.",
                        case(vibrance=-100), [0]),
    "FX-VIBRANCE-004": ("Saturation 100: every colour pushed twice as far from its grey, dull or "
                        "vivid alike, each channel held between 0 and 1 (the skin's red, the "
                        "blue's blue and the red trace's red held at the top); the pure red "
                        "stays pure red, and the grey stays grey.",
                        case(saturation=100), [0]),
    "FX-VIBRANCE-005": ("Saturation -100: every colour becomes its own grey, the brightness "
                        "kept: the pure red turns the grey #363636 but for rounding (54.2), "
                        "and the grey stays as it is.", case(saturation=-100), [0]),
    "FX-VIBRANCE-006": ("Vibrance 50 and saturation -50: the two pull against each other, so "
                        "the dull colours barely move (the line keeps 0.98 of its colour, the "
                        "tan 0.92) while the vivid ones lose most (the blue and red trace keep "
                        "about 0.69, the pure red half).",
                        case(vibrance=50, saturation=-50), [0]),
    "FX-VIBRANCE-007": ("Vibrance 100 and saturation -100: each colour keeps exactly the share "
                        "of its colour that was missing from it, 1 - s: the tan keeps 0.84, the "
                        "skin 0.78, the blue 0.38, and the pure red none, turning grey as in "
                        "FX-VIBRANCE-005.", case(vibrance=100, saturation=-100), [0]),
    "FX-VIBRANCE-008": ("Vibrance -100 and saturation 100: the other way round, each colour "
                        "pushed 1 + s times from its grey, so the vivid colours move most (the "
                        "blue's blue and the red trace's red held at the top) and the line "
                        "barely moves (1.04).", case(vibrance=-100, saturation=100), [0]),
    "FX-VIBRANCE-009": ("Vibrance 40 and saturation 20, an everyday grade: every colour pushed "
                        "1.2 + 0.4 (1 - s) times from its grey, the dull ones more (the tan "
                        "1.54) than the vivid (the blue 1.35), the skin's red held at the top; "
                        "the grey stays, and the pure red, pushed past both its ends, stays "
                        "pure red.", case(vibrance=40, saturation=20), [0]),
    "FX-VIBRANCE-010": ("Vibrance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                        "untouched, frame 2 vibrance 50, frame 4 FX-VIBRANCE-002.",
                        case(vibrance=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-VIBRANCE-011": ("Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is "
                        "FX-VIBRANCE-005, frame 2, both at 0, is the drawing untouched, and "
                        "frame 4 is FX-VIBRANCE-004.",
                        case(saturation=keyed((0, -100), (4, 100))), [0, 2, 4]),
    "FX-VIBRANCE-012": ("Vibrance eased from 0 at frame 0 to -100 at frame 4 on a curve that "
                        "overshoots: at frame 2 it has gone past -100 and is held there, so "
                        "frames 2 and 4 are both FX-VIBRANCE-003.",
                        case(vibrance=keyed((0, 0, OVERSHOOT), (4, -100))), [0, 2, 4]),
    "FX-VIBRANCE-013": ("FX-VIBRANCE-009 moved three pixels right: the same, moved.",
                        case(vibrance=40, saturation=20, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-VIBRANCE-014": ("Vibrance 101, above 100.", case(vibrance=101)),
    "FX-VIBRANCE-015": ("Vibrance -101, below -100.", case(vibrance=-101)),
    "FX-VIBRANCE-016": ("Saturation 150, above 100.", case(saturation=150)),
    "FX-VIBRANCE-017": ("Saturation -101, below -100.", case(saturation=-101)),
    "FX-VIBRANCE-018": ("Vibrance keyed to -150 at frame 4.",
                        case(vibrance=keyed((0, 0), (4, -150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.vibrance", "enabled": True,
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

    (OUT / "expected_vibrance.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    same = lambda f, xy: near(f[at(*xy)], drawn[at(*xy)])  # noqa: E731
    grey, line, tan, skin, band, trace, red, soft = (1, 4), (3, 4), (5, 4), (7, 4), \
        (9, 4), (11, 4), (13, 4), (15, 4)
    colours = (line, tan, skin, band, trace, red)
    empty = [(0, 4), (4, 0), (4, 9)]
    enc = lambda xy: [v / 255 for v in BANDS[xy[0]][:3]]  # noqa: E731
    sat = lambda xy: max(enc(xy)) - min(enc(xy))  # noqa: E731
    luma = lambda e: 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]  # noqa: E731
    lin = lambda e: [srgb_to_linear(min(1.0, max(0.0, v))) for v in e] + [1.0]  # noqa: E731

    def pushed(k, xy):
        """The pixel's encoded colour pushed k times as far from its grey, before the clamp."""
        e = enc(xy)
        L = luma(e)
        return [L + (v - L) * k for v in e]

    def only(f, k, xy):
        """The pixel is its colour pushed exactly k times from its grey, held between 0 and 1."""
        return near(f[at(*xy)], lin(pushed(k, xy)))

    def ends(k, xy):
        """Which channels the push takes past the top (1) or the bottom (-1)."""
        return [1 if v > 1 else -1 if v < 0 else 0 for v in pushed(k, xy)]

    # The rule's own pieces: the colours' spread s, the strength k, and a grey kept as it is.
    assert sat(grey) == 0 and abs(sat(line) - 10 / 255) < 1e-15 and sat(red) == 1
    assert abs(sat(tan) - 40 / 255) < 1e-15 and abs(sat(skin) - 56 / 255) < 1e-15
    assert 0.6 < sat(band) < sat(trace) < 0.64
    assert strength((1, 0, 0), 100, 0) == 1 and strength((0.5, 0.5, 0.5), 100, 0) == 2
    assert abs(strength((0.2, 0.5, 0.9), -100, 100) - 1.7) < 1e-15
    assert all(abs(v - 0.5) < 1e-15 for v in grade((0.5, 0.5, 0.5), 100, 100))
    assert grade((1, 0, 0), 0, -100) == [0.2126] * 3
    e = (0.2, 0.5, 0.9)
    assert all(abs(u - v) < 1e-15 for u, v in zip(grade(e, 0, 0), e))

    # A pixel that does not show, and the covering, are kept by every case; the soft skin is
    # graded as the skin, at half covering; the grey is never moved.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-VIBRANCE-013" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx != "FX-VIBRANCE-013":
                assert near([v / (128 / 255) for v in px[at(*soft)][:3]], px[at(*skin)][:3])
                assert same(px, grey), fx
                assert all(px[at(*xy)] == [0.0] * 4 for xy in empty)

    assert c["FX-VIBRANCE-001"]["0"] == drawn
    two = c["FX-VIBRANCE-002"]["0"]
    for xy in colours:
        assert only(two, 2 - sat(xy), xy), xy
    assert abs(2 - sat(tan) - 1.84) < 5e-3 and abs(2 - sat(skin) - 1.78) < 1e-3
    assert 1.37 < 2 - sat(trace) < 2 - sat(band) < 1.39 and same(two, red)
    assert ends(2 - sat(skin), skin) == [1, 0, 0] and ends(2 - sat(band), band) == [0, 0, 1]
    assert ends(2 - sat(tan), tan) == [0, 0, 0] and ends(2 - sat(trace), trace) == [0, 0, 0]
    three = c["FX-VIBRANCE-003"]["0"]
    for xy in colours:
        assert only(three, sat(xy), xy), xy
    assert abs(sat(tan) - 0.16) < 5e-3 and abs(sat(skin) - 0.22) < 1e-3
    assert abs(sat(band) - 0.62) < 1e-3 and abs(sat(trace) - 0.62) < 1e-2 and same(three, red)
    four = c["FX-VIBRANCE-004"]["0"]
    for xy in colours:
        assert only(four, 2, xy), xy
    assert ends(2, skin) == [1, 0, 0] and ends(2, band) == [0, 0, 1]
    assert ends(2, trace) == [1, 0, 0] and ends(2, tan) == ends(2, line) == [0, 0, 0]
    assert same(four, red)
    five = c["FX-VIBRANCE-005"]["0"]
    for xy in colours:
        assert near(five[at(*xy)], lin([luma(enc(xy))] * 3)), xy
    assert abs(255 * 0.2126 - 54.2) < 0.02 and near(five[at(*red)], lin([0.2126] * 3))
    six = c["FX-VIBRANCE-006"]["0"]
    for xy in colours:
        assert only(six, 1 - sat(xy) / 2, xy), xy
    assert 1 - sat(line) / 2 > 0.98 and abs(1 - sat(tan) / 2 - 0.92) < 5e-3
    assert abs(1 - sat(band) / 2 - 0.69) < 5e-3 and abs(1 - sat(trace) / 2 - 0.69) < 5e-3
    seven = c["FX-VIBRANCE-007"]["0"]
    for xy in colours:
        assert only(seven, 1 - sat(xy), xy), xy
    assert abs(1 - sat(tan) - 0.84) < 5e-3 and abs(1 - sat(skin) - 0.78) < 1e-3
    assert abs(1 - sat(band) - 0.38) < 1e-3 and near(seven[at(*red)], five[at(*red)])
    eight = c["FX-VIBRANCE-008"]["0"]
    for xy in colours:
        assert only(eight, 1 + sat(xy), xy), xy
    assert abs(1 + sat(line) - 1.04) < 1e-3 and same(eight, red)
    assert ends(1 + sat(band), band) == [0, 0, 1] and ends(1 + sat(trace), trace) == [1, 0, 0]
    nine = c["FX-VIBRANCE-009"]["0"]
    k9 = lambda xy: 1.2 + 0.4 * (1 - sat(xy))  # noqa: E731
    for xy in colours:
        assert only(nine, k9(xy), xy), xy
    assert abs(k9(tan) - 1.54) < 5e-3 and abs(k9(band) - 1.35) < 5e-3 and same(nine, red)
    assert ends(k9(skin), skin) == [1, 0, 0] and ends(k9(red), red) == [1, -1, -1]
    assert all(ends(k9(xy), xy) == [0, 0, 0] for xy in (line, tan, band, trace))
    ten = c["FX-VIBRANCE-010"]
    assert ten["0"] == drawn and like(ten["4"], two)
    assert like(ten["2"], render(case(vibrance=50), 0))
    eleven = c["FX-VIBRANCE-011"]
    assert eleven["2"] == drawn and like(eleven["0"], five) and like(eleven["4"], four)
    twelve = c["FX-VIBRANCE-012"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(vibrance=keyed(
        (0, 0, OVERSHOOT), (4, -100)))["vibrance"], 2) < -100
    assert twelve["0"] == drawn and twelve["2"] == three and twelve["4"] == three
    moved = c["FX-VIBRANCE-013"]["0"]
    assert moved == c["FX-VIBRANCE-013"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == nine[at(0, y):at(W - 3, y)]
        assert moved[at(0, y):at(3, y)] == [[0.0] * 4] * 3
    print("checked")


if __name__ == "__main__":
    main()
