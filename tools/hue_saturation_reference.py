"""Hue/Saturation, worked a second way.

D-113 adds `core.hue_saturation`. It turns every colour of a drawing round the colour wheel,
makes it more or less vivid, and lightens or darkens it, as a painter shifts a whole palette at
once. It works on each shown pixel's straight colour through the sRGB curve, read as hue,
saturation and lightness (HSL): `hue` adds degrees to the hue, round the wheel, so past 360 comes
back to 0 and below 0 comes back from 360; `saturation` scales the saturation by
(1 + saturation / 100), held inside 0 and 1, so -100 greys everything; `lightness` moves the
lightness toward white by that share of the way when above 0 and toward black when below, so
+100 is white and -100 black. A grey has no hue and stays grey under any hue or saturation. The
covering is kept, so a soft edge stays soft, and a pixel that does not show stays as it is.
With all three at 0 it changes nothing. It is this program's own method, modelled on After
Effects' Hue/Saturation; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of colour, drawn below. The drawing goes into
`Fixtures/hue_saturation/media`, the projects into `Fixtures/hue_saturation`, and the expected
frames into `Fixtures/hue_saturation/expected_hue_saturation.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/hue_saturation_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from color_key_reference import hue as hue8  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "hue_saturation"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"hue": (-180, 180), "saturation": (-100, 100), "lightness": (-100, 100)}


# --- the rule -------------------------------------------------------------------------------

def to_hsl(r, g, b):
    """Encoded red, green and blue in 0..1 as hue in degrees, saturation and lightness. Of equal
    largest channels, red counts before green and green before blue, as colour key's hue."""
    mx, mn = max(r, g, b), min(r, g, b)
    c = mx - mn
    l = (mx + mn) / 2
    if c == 0:
        return 0.0, 0.0, l
    s = c / (1 - abs(2 * l - 1))
    if mx == r:
        h = 60 * (((g - b) / c) % 6)
    elif mx == g:
        h = 60 * ((b - r) / c + 2)
    else:
        h = 60 * ((r - g) / c + 4)
    return h, s, l


def from_hsl(h, s, l):
    """Standard HSL back to red, green and blue, by the sextant of h in 0..360."""
    C = (1 - abs(2 * l - 1)) * s
    X = C * (1 - abs((h / 60) % 2 - 1))
    m = l - C / 2
    sextants = [(C, X, 0), (X, C, 0), (0, C, X), (0, X, C), (X, 0, C), (C, 0, X)]
    r1, g1, b1 = sextants[int(h // 60) % 6]
    return r1 + m, g1 + m, b1 + m


def grade(e, hue, saturation, lightness):
    """One encoded straight colour, graded."""
    h, s, l = to_hsl(*e)
    h = (h + hue) % 360
    s = min(1.0, max(0.0, s * (1 + saturation / 100)))
    l = l + (1 - l) * lightness / 100 if lightness >= 0 else l * (1 + lightness / 100)
    return from_hsl(h, s, l)


def hue_saturation(pixels, hue, saturation, lightness):
    out = [R.working(p) for p in pixels]
    if hue == 0 and saturation == 0 and lightness == 0:
        return out  # the build exits early
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a
                     for v in grade(e, hue, saturation, lightness)]
    return out


# --- the drawing ----------------------------------------------------------------------------

RED, GREEN, BLUE = (255, 0, 0, 255), (0, 255, 0, 255), (0, 0, 255, 255)
GREY = (128, 128, 128, 255)      # #808080
MAGENTA = (255, 0, 255, 255)     # hue 300, the last sextant
SKIN = R.SKIN                    # #f6d6be
LINE = R.LINE                    # #1e1a24
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
NONE = S.NONE
BANDS = [NONE, RED, RED, GREEN, GREEN, BLUE, BLUE, GREY, GREY, SKIN, SKIN,
         MAGENTA, MAGENTA, LINE, LINE, SOFT]


def bands(x, y):
    """Rows 0 and 9 and column 0 are empty. Between, columns in pairs: red 1-2, green 3-4, blue
    5-6, grey 7-8, skin 9-10, magenta 11-12, line 13-14, and the skin at half covering in 15."""
    return NONE if y in (0, 9) else BANDS[x]


DRAWINGS = {"bands": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(hue=0, saturation=0, lightness=0, shift=0):
    return {"drawing": "bands", "hue": hue, "saturation": saturation, "lightness": lightness,
            "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(hue_saturation(pixels, *(held(c, k, frame_no) for k in RANGES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


CASES = {
    "FX-HUESAT-001": ("All three at 0, the defaults: the drawing, untouched.",
                      case(), [0]),
    "FX-HUESAT-002": ("Hue +120: red turns green, green blue, and blue red; magenta, at 300, "
                      "goes past 360 to 60, yellow; the grey and the empty pixels stay as they "
                      "are, and the soft edge keeps its half covering.",
                      case(hue=120), [0]),
    "FX-HUESAT-003": ("Hue -120: red turns blue, green red, and blue green.",
                      case(hue=-120), [0]),
    "FX-HUESAT-004": ("Hue -30: red, at 0, goes below 0 to 330, a pink-red #ff0080.",
                      case(hue=-30), [0]),
    "FX-HUESAT-005": ("Hue +90: magenta, at 300, goes past 360 to 30, orange #ff8000.",
                      case(hue=90), [0]),
    "FX-HUESAT-006": ("Hue 180: every colour turns to its opposite, red to cyan; the grey stays.",
                      case(hue=180), [0]),
    "FX-HUESAT-007": ("Saturation -100: every pixel turns grey at its own lightness, red, green, "
                      "blue and magenta all to mid grey; the grey stays.",
                      case(saturation=-100), [0]),
    "FX-HUESAT-008": ("Saturation +100: the line's saturation doubles, and the skin's, at 0.76, "
                      "is held at 1, fully vivid, at the same hue and lightness; the pure "
                      "primaries and magenta, already full, and the grey stay.",
                      case(saturation=100), [0]),
    "FX-HUESAT-009": ("Saturation -50: every colour half way to grey.",
                      case(saturation=-50), [0]),
    "FX-HUESAT-010": ("Lightness +100: every shown pixel turns white at its own covering, the "
                      "soft edge white at half; the empty pixels stay empty.",
                      case(lightness=100), [0]),
    "FX-HUESAT-011": ("Lightness -100: every shown pixel turns black at its own covering.",
                      case(lightness=-100), [0]),
    "FX-HUESAT-012": ("Lightness +50: red's lightness goes from 0.5 half way to 1, to #ff8080.",
                      case(lightness=50), [0]),
    "FX-HUESAT-013": ("Lightness -50: red's lightness goes from 0.5 half way to 0, to #800000.",
                      case(lightness=-50), [0]),
    "FX-HUESAT-014": ("Hue 60, saturation -50, lightness 20 together: red turns a pale, soft "
                      "yellow; the grey is lightened only.",
                      case(hue=60, saturation=-50, lightness=20), [0]),
    "FX-HUESAT-015": ("Hue keyed from -180 at frame 0 to 180 at frame 4, linear: frames 0 and 4 "
                      "are both FX-HUESAT-006, half a turn either way; frame 2, at 0, is the "
                      "drawing untouched.",
                      case(hue=keyed((0, -180), (4, 180))), [0, 2, 4]),
    "FX-HUESAT-016": ("Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is "
                      "FX-HUESAT-007, frame 2 the drawing, frame 4 FX-HUESAT-008.",
                      case(saturation=keyed((0, -100), (4, 100))), [0, 2, 4]),
    "FX-HUESAT-017": ("FX-HUESAT-002 moved three pixels right: the same, moved.",
                      case(hue=120, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-HUESAT-018": ("Hue 181, above 180.", case(hue=181)),
    "FX-HUESAT-019": ("Hue -181, below -180.", case(hue=-181)),
    "FX-HUESAT-020": ("Saturation 101, above 100.", case(saturation=101)),
    "FX-HUESAT-021": ("Lightness -101, below -100.", case(lightness=-101)),
    "FX-HUESAT-022": ("Hue keyed to 200 at frame 4.", case(hue=keyed((0, 0), (4, 200)))),
    "FX-HUESAT-023": ("Lightness written as the word \"20\", not a number.",
                      case(lightness="20")),
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
        "instance_id": "fx-0-0", "type_id": "core.hue_saturation", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in RANGES}}]
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

    (OUT / "expected_hue_saturation.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    red, green, blue, grey, skin, magenta, line, soft = (1, 4), (3, 4), (5, 4), (7, 4), (9, 4), \
        (11, 4), (13, 4), (15, 4)
    empty = [(0, 4), (4, 0), (4, 9)]

    # The rule's own pieces.
    assert to_hsl(1, 0, 0) == (0, 1, 0.5) and to_hsl(0.5, 0.5, 0.5) == (0, 0, 0.5)
    for q in [(246, 214, 190), (30, 26, 36), (255, 0, 255), (0, 177, 64), (255, 255, 0)]:
        h, s, l = to_hsl(*[v / 255 for v in q])
        assert abs(h - hue8(q)) < 1e-9
        assert near(from_hsl(h, s, l), [v / 255 for v in q])
    assert near(from_hsl(330, 1, 0.5), (1, 0, 0.5)) and near(from_hsl(30, 1, 0.5), (1, 0.5, 0))

    assert c["FX-HUESAT-001"]["0"] == drawn
    two = c["FX-HUESAT-002"]["0"]
    assert near(two[at(*red)], drawn[at(*green)]) and near(two[at(*green)], drawn[at(*blue)])
    assert near(two[at(*blue)], drawn[at(*red)]) and near(two[at(*magenta)], lin(255, 255, 0))
    assert near(two[at(*grey)], drawn[at(*grey)]) and two[at(*soft)][3] == 128 / 255
    three = c["FX-HUESAT-003"]["0"]
    assert near(three[at(*red)], drawn[at(*blue)]) and near(three[at(*green)], drawn[at(*red)])
    assert near(c["FX-HUESAT-004"]["0"][at(*red)], lin(255, 0, 127.5))
    assert near(c["FX-HUESAT-005"]["0"][at(*magenta)], lin(255, 127.5, 0))
    six = c["FX-HUESAT-006"]["0"]
    assert near(six[at(*red)], lin(0, 255, 255)) and near(six[at(*grey)], drawn[at(*grey)])
    seven = c["FX-HUESAT-007"]["0"]
    for p in seven:
        assert p[0] == p[1] == p[2]
    for xy in (red, green, blue, magenta):
        assert near(seven[at(*xy)], lin(127.5, 127.5, 127.5))
    eight = c["FX-HUESAT-008"]["0"]
    for xy in (red, green, blue, grey, magenta):
        assert near(eight[at(*xy)], drawn[at(*xy)])
    h, s, l = to_hsl(*[v / 255 for v in SKIN[:3]])
    assert 0.75 < s < 0.76 and near(eight[at(*skin)], lin(*[v * 255 for v in from_hsl(h, 1, l)]))
    h, s, l = to_hsl(*[v / 255 for v in LINE[:3]])
    assert s < 0.5 and near(eight[at(*line)], lin(*[v * 255 for v in from_hsl(h, 2 * s, l)]))
    assert near(c["FX-HUESAT-009"]["0"][at(*red)], lin(191.25, 63.75, 63.75))
    ten, eleven = c["FX-HUESAT-010"]["0"], c["FX-HUESAT-011"]["0"]
    for i in range(W * H):
        a = drawn[i][3]
        assert near(ten[i], [a, a, a, a]) and near(eleven[i], [0, 0, 0, a])
    assert near(c["FX-HUESAT-012"]["0"][at(*red)], lin(255, 127.5, 127.5))
    assert near(c["FX-HUESAT-013"]["0"][at(*red)], lin(127.5, 0, 0))
    fourteen = c["FX-HUESAT-014"]["0"]
    y = fourteen[at(*red)]
    assert abs(y[0] - y[1]) < 1e-9 and y[2] < y[0] and y[2] > 0
    g = fourteen[at(*grey)]
    assert g[0] == g[1] == g[2] > drawn[at(*grey)][0]
    fifteen = c["FX-HUESAT-015"]
    assert like(fifteen["0"], six) and like(fifteen["4"], six) and fifteen["2"] == drawn
    sixteen = c["FX-HUESAT-016"]
    assert like(sixteen["0"], seven) and sixteen["2"] == drawn and like(sixteen["4"], eight)
    moved = c["FX-HUESAT-017"]["0"]
    assert moved == c["FX-HUESAT-017"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    for name, frames in c.items():
        for px in frames.values():
            for xy in empty:
                assert px[at(*xy)] == [0.0] * 4 or name == "FX-HUESAT-017"
            assert px[at(*soft)][3] == 128 / 255 or name == "FX-HUESAT-017"
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
