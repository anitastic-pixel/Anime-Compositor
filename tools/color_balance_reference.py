"""Color Balance, worked a second way.

D-130 adds `core.color_balance`. It tints a drawing's shadows, midtones and highlights
separately, as a colourist warms the lights and cools the darks: each of `shadows`, `midtones`
and `highlights` is three numbers, red, green and blue, each -100 to 100, starting at 0, 0, 0,
and each pushes that channel of the pixels in its range up (above 0) or down (below 0), by at
most half the channel's whole scale at 100. Each pixel's brightness decides how much of each
triple it takes: a black pixel takes only the shadows, a white one only the highlights, a mid
grey only the midtones, and the tones between share the two nearest. The covering is kept, so a
soft edge stays soft, and a pixel that does not show stays as it is. With all nine at 0 it
changes nothing. It is this program's own method, modelled on After Effects' Color Balance;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour, L = 0.2126 e_r + 0.7152 e_g + 0.0722 e_b, w_s = clamp(1 - 2 L, 0, 1),
w_h = clamp(2 L - 1, 0, 1) and w_m = 1 - w_s - w_h: per channel
e'_c = clamp(e_c + (w_s S_c + w_m M_c + w_h H_c) / 200, 0, 1), and the output is
(srgb_to_linear(e'_c) * a, a). With all nine numbers 0 the output is the input, exactly. The
layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of tone from black to white, drawn below. The drawing goes into
`Fixtures/color_balance/media`, the projects into `Fixtures/color_balance`, and the expected
frames into `Fixtures/color_balance/expected_color_balance.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_balance_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_balance"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("shadows", "midtones", "highlights")
RANGE = (-100, 100)
ZERO = (0, 0, 0)


# --- the rule -------------------------------------------------------------------------------

def weights(e):
    """How much of the shadows, the midtones and the highlights an encoded colour takes."""
    L = 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]
    ws = min(1.0, max(0.0, 1 - 2 * L))
    wh = min(1.0, max(0.0, 2 * L - 1))
    return ws, 1 - ws - wh, wh


def grade(e, shadows, midtones, highlights):
    """One encoded straight colour, balanced."""
    ws, wm, wh = weights(e)
    return [min(1.0, max(0.0, e[c] + (ws * shadows[c] + wm * midtones[c]
                                      + wh * highlights[c]) / 200)) for c in range(3)]


def color_balance(pixels, shadows, midtones, highlights):
    out = [R.working(p) for p in pixels]
    if all(v == 0 for t in (shadows, midtones, highlights) for v in t):
        return out  # the build exits early
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            w[:3] = [srgb_to_linear(v) * a for v in grade(e, shadows, midtones, highlights)]
    return out


# --- the drawing ----------------------------------------------------------------------------

BLACK = (0, 0, 0, 255)
LINE = R.LINE                    # #1e1a24, the line: deep in the shadows
DARK = (64, 64, 64, 255)         # #404040: half shadows, half midtones
GREY = (128, 128, 128, 255)      # #808080: the midtones, a hair above half
LIGHT = (192, 192, 192, 255)     # #c0c0c0: half midtones, half highlights
SKIN = R.SKIN                    # #f6d6be: mostly highlights
WHITE = (255, 255, 255, 255)
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
NONE = S.NONE
BANDS = [NONE, BLACK, BLACK, LINE, LINE, DARK, DARK, GREY, GREY, LIGHT, LIGHT,
         SKIN, SKIN, WHITE, WHITE, SOFT]


def bands(x, y):
    """Rows 0 and 9 and column 0 are empty. Between, columns in pairs from dark to light: black
    1-2, line 3-4, dark grey 5-6, grey 7-8, light grey 9-10, skin 11-12, white 13-14, and the
    skin at half covering in 15."""
    return NONE if y in (0, 9) else BANDS[x]


DRAWINGS = {"tones": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(shadows=ZERO, midtones=ZERO, highlights=ZERO, shift=0):
    return {"drawing": "tones", "shadows": shadows, "midtones": midtones,
            "highlights": highlights, "shift": shift}


def held(c, k, frame_no):
    return [min(RANGE[1], max(RANGE[0], v)) for v in value_at(c[k], frame_no)]


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(color_balance(pixels, *(held(c, k, frame_no) for k in NAMES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


CASES = {
    "FX-BALANCE-001": ("All nine at 0, the settings as they start: the drawing, untouched.",
                       case(), [0]),
    "FX-BALANCE-002": ("Shadows 100, 0, 0: black turns a dark red, its red half way up the "
                       "scale, #800000 but for rounding (127.5); the line and the dark grey "
                       "redden too, by less, as they are less deep in the shadows, though the "
                       "dark grey's red lands on half the scale too, as any grey below half "
                       "does; the grey, the light grey, the skin and the white, all at or above "
                       "half brightness, take no shadows and stay as they are.",
                       case(shadows=(100, 0, 0)), [0]),
    "FX-BALANCE-003": ("Shadows -100, -100, -100: the line and the dark grey darken; black, "
                       "already at 0, is held there and stays black.",
                       case(shadows=(-100, -100, -100)), [0]),
    "FX-BALANCE-004": ("Midtones 0, 100, 0: the grey takes nearly all of it and turns a light "
                       "green; the dark and light greys take about half; black and white take "
                       "none and stay as they are.",
                       case(midtones=(0, 100, 0)), [0]),
    "FX-BALANCE-005": ("Midtones 0, -100, 0: the grey loses nearly half its green scale and "
                       "turns a dull purple; black and white stay.",
                       case(midtones=(0, -100, 0)), [0]),
    "FX-BALANCE-006": ("Highlights 0, 0, 100: the skin and the light grey turn bluer; the "
                       "white's blue is already full and is held there, so the white stays "
                       "white; black and the line stay.",
                       case(highlights=(0, 0, 100)), [0]),
    "FX-BALANCE-007": ("Highlights 0, 0, -100: the white turns a pale yellow, its blue half way "
                       "down, #ffff80 but for rounding (127.5); the skin turns more yellow; "
                       "black stays.",
                       case(highlights=(0, 0, -100)), [0]),
    "FX-BALANCE-008": ("All three 100, 100, 100: every shown pixel, whatever its tone, is "
                       "lifted half the scale on every channel, held at the top: black turns "
                       "mid grey (127.5) and white stays white.",
                       case(shadows=(100, 100, 100), midtones=(100, 100, 100),
                            highlights=(100, 100, 100)), [0]),
    "FX-BALANCE-009": ("Shadows 0, 0, 100 and highlights 100, 0, 0, cool shadows and warm "
                       "lights: the dark grey turns bluer, the light grey redder, and the grey, "
                       "a hair above half, takes a two-thousandth of the highlights' red and "
                       "no blue.",
                       case(shadows=(0, 0, 100), highlights=(100, 0, 0)), [0]),
    "FX-BALANCE-010": ("Shadows 100, 0, 0 and midtones 0, 0, 100: the dark grey, a quarter "
                       "bright, takes about half of each, red and blue up alike, and stays "
                       "neutral in green.",
                       case(shadows=(100, 0, 0), midtones=(0, 0, 100)), [0]),
    "FX-BALANCE-011": ("Shadows keyed from 0, 0, 0 at frame 0 to 100, 0, 0 at frame 4, linear, "
                       "one key holding all three numbers: frame 0 untouched, frame 2 shadows "
                       "50, 0, 0, frame 4 FX-BALANCE-002.",
                       case(shadows=keyed((0, ZERO), (4, (100, 0, 0)))), [0, 2, 4]),
    "FX-BALANCE-012": ("Midtones keyed from -100, -100, -100 at frame 0 to 100, 100, 100 at "
                       "frame 4: frame 0 darkens the middle tones, frame 2, all nine at 0, is "
                       "the drawing untouched, and frame 4 lightens them.",
                       case(midtones=keyed((0, (-100, -100, -100)), (4, (100, 100, 100)))),
                       [0, 2, 4]),
    "FX-BALANCE-013": ("Highlights eased from 0, 0, 0 at frame 0 to 0, 0, -100 at frame 4 on a "
                       "curve that overshoots: at frame 2 the blue has gone past -100 and is "
                       "held there, so frames 2 and 4 are both FX-BALANCE-007.",
                       case(highlights=keyed((0, ZERO, OVERSHOOT), (4, (0, 0, -100)))),
                       [0, 2, 4]),
    "FX-BALANCE-014": ("FX-BALANCE-009 moved three pixels right: the same, moved.",
                       case(shadows=(0, 0, 100), highlights=(100, 0, 0), shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BALANCE-015": ("Shadows 101, 0, 0: a red above 100.", case(shadows=(101, 0, 0))),
    "FX-BALANCE-016": ("Midtones 0, -101, 0: a green below -100.", case(midtones=(0, -101, 0))),
    "FX-BALANCE-017": ("Highlights 0, 0, 150: a blue above 100.",
                       case(highlights=(0, 0, 150))),
    "FX-BALANCE-018": ("Highlights keyed to 0, 0, 150 at frame 4.",
                       case(highlights=keyed((0, ZERO), (4, (0, 0, 150))))),
    "FX-BALANCE-019": ("Shadows written with two numbers, 100, 0: a tone is three.",
                       case(shadows=(100, 0))),
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
        "instance_id": "fx-0-0", "type_id": "core.color_balance", "enabled": True,
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

    (OUT / "expected_color_balance.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    same = lambda f, xy: near(f[at(*xy)], drawn[at(*xy)])  # noqa: E731
    black, line, dark, grey, light, skin, white, soft = (1, 4), (3, 4), (5, 4), (7, 4), \
        (9, 4), (11, 4), (13, 4), (15, 4)
    empty = [(0, 4), (4, 0), (4, 9)]
    enc = lambda q: [v / 255 for v in q[:3]]  # noqa: E731

    # The rule's own pieces: black is all shadows, white all highlights, and the dark and light
    # greys share with the midtones about half and half; the weights always add up to 1.
    assert weights((0, 0, 0)) == (1, 0, 0) and weights((1, 1, 1)) == (0, 0, 1)
    ws, wm, wh = weights(enc(DARK))
    assert abs(ws - 0.498) < 1e-3 and abs(wm - 0.502) < 1e-3 and wh == 0
    ws, wm, wh = weights(enc(LIGHT))
    assert ws == 0 and abs(wh - 0.506) < 1e-3
    ws, wm, wh = weights(enc(GREY))
    assert ws == 0 and 0 < wh < 0.004 and wm > 0.996
    for q in BANDS[1:]:
        assert abs(sum(weights(enc(q))) - 1) < 1e-15
    assert grade((0.2, 0.5, 0.9), ZERO, ZERO, ZERO) == [0.2, 0.5, 0.9]
    assert grade((0, 0, 0), (100, -100, 50), ZERO, ZERO) == [0.5, 0, 0.25]

    # A pixel that does not show, and the covering, are kept by every case; the soft skin is
    # graded as the skin, at half covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-BALANCE-014" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx != "FX-BALANCE-014":
                assert near([v / (128 / 255) for v in px[at(*soft)][:3]], px[at(*skin)][:3])

    assert c["FX-BALANCE-001"]["0"] == drawn
    two = c["FX-BALANCE-002"]["0"]
    assert near(two[at(*black)], lin(127.5, 0, 0))
    o = lambda xy: drawn[at(*xy)]  # noqa: E731
    assert two[at(*line)][0] > o(line)[0] and near(two[at(*line)][1:], o(line)[1:])
    push = lambda f, xy: S.linear_to_srgb(f[at(*xy)][0]) - BANDS[xy[0]][0] / 255  # noqa: E731
    assert abs(push(two, black) - 0.5) < 1e-9 and 0 < push(two, dark) < push(two, line) < 0.5
    assert abs(two[at(*dark)][0] - two[at(*black)][0]) < 1e-12
    assert all(same(two, xy) for xy in (grey, light, skin, white))
    three = c["FX-BALANCE-003"]["0"]
    assert three[at(*black)] == drawn[at(*black)]
    assert all(three[at(*xy)][k] < drawn[at(*xy)][k] for xy in (line, dark) for k in range(3))
    assert all(same(three, xy) for xy in (grey, light, skin, white))
    four = c["FX-BALANCE-004"]["0"]
    g = 128 / 255
    assert near(four[at(*grey)], lin(128, 255 * min(1, g + weights(enc(GREY))[1] / 2), 128))
    assert all(same(four, xy) for xy in (black, white))
    assert four[at(*dark)][1] > o(dark)[1] and four[at(*light)][1] > o(light)[1]
    five = c["FX-BALANCE-005"]["0"]
    assert five[at(*grey)][1] < o(grey)[1] / 10 and same(five, black) and same(five, white)
    six = c["FX-BALANCE-006"]["0"]
    assert six[at(*white)] == drawn[at(*white)] and same(six, black) and same(six, line)
    assert six[at(*skin)][2] > drawn[at(*skin)][2] and six[at(*light)][2] > drawn[at(*light)][2]
    seven = c["FX-BALANCE-007"]["0"]
    assert near(seven[at(*white)], lin(255, 255, 127.5)) and same(seven, black)
    assert seven[at(*skin)][2] < drawn[at(*skin)][2]
    eight = c["FX-BALANCE-008"]["0"]
    assert near(eight[at(*black)], lin(127.5, 127.5, 127.5)) and eight[at(*white)] == o(white)
    for xy in (line, dark, grey, light, skin):
        e = enc(BANDS[xy[0]])
        assert near(eight[at(*xy)], lin(*[255 * min(1, v + 0.5) for v in e]))
    nine = c["FX-BALANCE-009"]["0"]
    assert nine[at(*dark)][2] > o(dark)[2] and near(nine[at(*dark)][:2], o(dark)[:2])
    assert nine[at(*light)][0] > o(light)[0] and near(nine[at(*light)][1:], o(light)[1:])
    wh = weights(enc(GREY))[2]
    assert near(nine[at(*grey)], lin(255 * (g + wh / 2), 128, 128)) and wh / 2 < 0.002
    ten = c["FX-BALANCE-010"]["0"]
    ws, wm, _ = weights(enc(DARK))
    assert near(ten[at(*dark)], lin(255 * (64 / 255 + ws / 2), 64, 255 * (64 / 255 + wm / 2)))
    eleven = c["FX-BALANCE-011"]
    assert eleven["0"] == drawn and like(eleven["4"], two)
    assert like(eleven["2"], render(case(shadows=(50, 0, 0)), 0))
    twelve = c["FX-BALANCE-012"]
    assert twelve["2"] == drawn
    assert like(twelve["0"], render(case(midtones=(-100, -100, -100)), 0))
    assert like(twelve["4"], render(case(midtones=(100, 100, 100)), 0))
    assert twelve["0"][at(*grey)][0] < drawn[at(*grey)][0] < twelve["4"][at(*grey)][0]
    thirteen = c["FX-BALANCE-013"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(highlights=keyed(
        (0, ZERO, OVERSHOOT), (4, (0, 0, -100))))["highlights"], 2)[2] < -100
    assert thirteen["0"] == drawn and thirteen["2"] == seven and thirteen["4"] == seven
    moved = c["FX-BALANCE-014"]["0"]
    assert moved == c["FX-BALANCE-014"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == nine[at(0, y):at(W - 3, y)]
    for name, frames in c.items():
        if name != "FX-BALANCE-014":
            for px in frames.values():
                assert all(px[at(*xy)] == [0.0] * 4 for xy in empty)
    print("checked")


if __name__ == "__main__":
    main()
