"""Shift Channels, worked a second way.

D-305 (B-190) added `core.shift_channels`, after After Effects' Shift Channels (Channel): each
of a drawing's alpha, red, green and blue is taken from a channel or a measure of the pixel's
colour, as P-26's Colorful Glitch tutorial takes a layer's alpha from its brightness so the dark
parts drop out. B-190 pinned it on hand-worked solids; D-553 (B-433) gives it this fixture folder.
It is this program's own method, modelled on After Effects' Shift Channels; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. `take_alpha`, `take_red`, `take_green` and `take_blue` are each one of "alpha", "red",
"green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", starting
at its own channel, none keyable. With each at its start the output is the input, exactly.
Otherwise, at every pixel, with a its covering and e = linear_to_srgb(clamp(p.rgb / a, 0, 1))
its encoded straight colour when a > 0, else e = (0, 0, 0), and M and m the largest and smallest
of e, L = (M + m) / 2, a word reads e_r, e_g, e_b, a, 0.2126 e_r + 0.7152 e_g + 0.0722 e_b,
D-141's HSV hue of e over 360 (0 for a grey), L, (M - m) / (1 - |2 L - 1|) (0 for a grey), 1,
0.5 or 0, in that order. a' = clamp(take_alpha, 0, 1), and the output is
(srgb_to_linear(clamp(take_c, 0, 1)) a' for each colour channel c, a'). So the covering is
whatever take_alpha reads: a pixel that did not show can show, black, and a soft edge taking its
alpha from its colour loses its softness. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Channel Mixer's swatches (tools/channel_mixer_reference.py). The drawing goes into
`Fixtures/shift_channels/media`, the projects into `Fixtures/shift_channels`, and the expected
frames into `Fixtures/shift_channels/expected_shift_channels.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/shift_channels_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import channel_mixer_reference as CM  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "shift_channels"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("take_alpha", "take_red", "take_green", "take_blue")
START = {"take_alpha": "alpha", "take_red": "red", "take_green": "green", "take_blue": "blue"}
FROM = ("alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full",
        "half", "off")


# --- the rule -------------------------------------------------------------------------------

def hue(e):
    """D-141's HSV hue of an encoded colour, in degrees, None for a grey."""
    r, g, b = e
    mx, mn = max(e), min(e)
    if mx == mn:
        return None
    d = mx - mn
    if r == mx:
        return (60 * (g - b) / d) % 360
    if g == mx:
        return 60 * ((b - r) / d + 2)
    return 60 * ((r - g) / d + 4)


def read(word, e, a):
    """What one word reads at a pixel of encoded straight colour `e` and covering `a`."""
    mx, mn = max(e), min(e)
    light = (mx + mn) / 2
    return {
        "red": e[0], "green": e[1], "blue": e[2], "alpha": a,
        "luminance": 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2],
        "hue": (hue(e) or 0.0) / 360,
        "lightness": light,
        "saturation": (mx - mn) / (1 - abs(2 * light - 1)) if mx > mn else 0.0,
        "full": 1.0, "half": 0.5, "off": 0.0,
    }[word]


def clamp01(v):
    return min(1.0, max(0.0, v))


def shift_channels(pixels, take_alpha, take_red, take_green, take_blue):
    """`pixels` are 8-bit straight RGBA."""
    out = [R.working(p) for p in pixels]
    if (take_alpha, take_red, take_green, take_blue) == ("alpha", "red", "green", "blue"):
        return out  # the build exits early
    for w in out:
        a = w[3]
        e = [S.linear_to_srgb(clamp01(v / a)) for v in w[:3]] if a > 0 else [0.0] * 3
        na = clamp01(read(take_alpha, e, a))
        w[:] = [srgb_to_linear(clamp01(read(t, e, a))) * na
                for t in (take_red, take_green, take_blue)] + [na]
    return out


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = CM.DRAWINGS  # the swatches


def case(shift=0, **take):
    return {"drawing": "swatches", "shift": shift, **START, **take}


def render(c, frame_no=0):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(shift_channels(pixels, *(c[k] for k in NAMES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]))


LUMA = {"take_red": "luminance", "take_green": "luminance", "take_blue": "luminance"}
GLITCH = {"take_alpha": "luminance"}

CASES = {
    "FX-SHIFT-001": ("The settings as they start, alpha from alpha, red from red, green from "
                     "green, blue from blue: the drawing, untouched.",
                     case(), [0]),
    "FX-SHIFT-002": ("Red from blue and blue from red, the two swapped: the red swatch turns "
                     "blue, #2828c8, the blue turns #ff6040, the skin a pale blue, #bed6f6; every "
                     "grey stays as it is, and every covering is kept.",
                     case(take_red="blue", take_blue="red"), [0]),
    "FX-SHIFT-003": ("Red, green and blue all from green: each pixel turns the grey of its own "
                     "green: the red #282828, the green #b4b4b4, the skin #d6d6d6.",
                     case(take_red="green", take_blue="green"), [0]),
    "FX-SHIFT-004": ("Red, green and blue from luminance: each pixel turns the grey of its "
                     "encoded luma, 0.2126 red + 0.7152 green + 0.0722 blue: the red 74.016 of "
                     "255, the green 147.268; white, grey and black stay as they are.",
                     case(**LUMA), [0]),
    "FX-SHIFT-005": ("Red, green and blue from hue: each pixel turns the grey of its HSV hue, a "
                     "full turn the whole scale: the red, at 0 degrees, turns black, the green "
                     "(130 degrees) 92.08 of 255, the blue (229.95 degrees) 162.88 and the skin "
                     "(25.71 degrees) 18.21; a grey has no hue and turns black.",
                     case(take_red="hue", take_green="hue", take_blue="hue"), [0]),
    "FX-SHIFT-006": ("Red, green and blue from lightness, the middle of the largest and the "
                     "smallest channel: the red 120 of 255, the skin 218, the blue 159.5; every "
                     "grey stays as it is.",
                     case(take_red="lightness", take_green="lightness", take_blue="lightness"),
                     [0]),
    "FX-SHIFT-007": ("Red, green and blue from saturation, HSL's: the red two thirds of the "
                     "scale (170), the green half (127.5), the blue the whole; every grey has "
                     "none and turns black.",
                     case(take_red="saturation", take_green="saturation",
                          take_blue="saturation"), [0]),
    "FX-SHIFT-008": ("Red full, green half and blue off: every pixel that shows turns orange, "
                     "full red and green at half of the encoded scale (127.5), its covering "
                     "kept, so the soft edges are orange at half covering.",
                     case(take_red="full", take_green="half", take_blue="off"), [0]),
    "FX-SHIFT-009": ("Alpha from luminance, the Colorful Glitch tutorial's use: the dark parts "
                     "drop out: the black is gone, the white stays, the grey shows at 128 of "
                     "255, the red at 74.016, the line at 27.57; a soft edge's covering is its "
                     "colour's luma, so the soft line is as faint as the line and the soft skin "
                     "as strong as the skin.",
                     case(**GLITCH), [0]),
    "FX-SHIFT-010": ("Alpha full: everything shows, at full covering: the empty border turns "
                     "black, and the soft edges turn solid line and solid skin.",
                     case(take_alpha="full"), [0]),
    "FX-SHIFT-011": ("Alpha off: nothing shows.",
                     case(take_alpha="off"), [0]),
    "FX-SHIFT-012": ("Alpha half: everything shows at half covering, the empty border black at "
                     "half, the soft edges no fainter than the rest.",
                     case(take_alpha="half"), [0]),
    "FX-SHIFT-013": ("Red, green and blue from alpha: the covering as grey: the drawing white, "
                     "its soft edges a half grey (128 of 255) at half covering, the border still "
                     "empty.",
                     case(take_red="alpha", take_green="alpha", take_blue="alpha"), [0]),
    "FX-SHIFT-014": ("Alpha from saturation: every grey drops out, the blue stays whole, the "
                     "red shows at two thirds and the green at half.",
                     case(take_alpha="saturation"), [0]),
    "FX-SHIFT-015": ("All four at once: red from hue, green from saturation, blue from lightness "
                     "and alpha from luminance, the swatches seen as HSL over their brightness.",
                     case(take_red="hue", take_green="saturation", take_blue="lightness", **GLITCH),
                     [0]),
    "FX-SHIFT-016": ("FX-SHIFT-009 moved three pixels right: the same, moved.",
                     case(shift=3, **GLITCH), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SHIFT-017": ("Red from \"Full\": the words are lower case.", case(take_red="Full")),
    "FX-SHIFT-018": ("Alpha from \"yes\", not a word Shift Channels takes.",
                     case(take_alpha="yes")),
    "FX-SHIFT-019": ("Green from \"brightness\": this program's word is \"luminance\".",
                     case(take_green="brightness")),
    "FX-SHIFT-020": ("Blue from \"\", no word at all.", case(take_blue="")),
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
        "instance_id": "fx-0-0", "type_id": "core.shift_channels", "enabled": True,
        "parameters": {k: c[k] for k in NAMES}}]
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

    (OUT / "expected_shift_channels.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    lin = lambda a, *e8: [srgb_to_linear(v / 255) * a for v in e8] + [a]  # noqa: E731
    is8 = lambda f, xy, *e8, a=1.0: near(f[at(*xy)], lin(a, *e8))  # noqa: E731
    same = lambda f, xy: near(f[at(*xy)], drawn[at(*xy)])  # noqa: E731
    line, skin, red, green, blue, white, grey, black = (2, 4), (3, 4), (5, 4), (7, 4), (9, 4), \
        (11, 4), (12, 4), (13, 4)
    greys = (white, grey, black)
    soft_line, soft_skin = (1, 2), (1, 6)
    empty = [(0, 4), (15, 4), (4, 0), (4, 9)]
    half = 128 / 255
    clear = [0.0] * 4

    # The rule's own pieces.
    assert hue((1, 0, 0)) == 0 and hue((0, 1, 0)) == 120 and hue((0, 0, 1)) == 240
    assert hue((1, 0, 0.5)) == 330 and hue((0.5, 0.5, 0.5)) is None
    assert read("saturation", (0.5, 0.5, 0.5), 1) == 0 and read("hue", (0.2, 0.2, 0.2), 1) == 0
    assert read("saturation", (1, 0, 0), 1) == 1 and read("lightness", (1, 0, 0), 1) == 0.5
    assert abs(read("luminance", (1, 1, 1), 1) - 1) < 1e-12
    assert all(w in FROM for w in START.values()) and len(set(FROM)) == 11

    # Every case keeps its colour channels inside the covering; and every case whose alpha comes
    # from alpha keeps every covering and leaves the empty border empty.
    for fx, frames in ((fx, v["frames"]) for fx, v in expected["cases"].items()):
        cc = CASES.get(fx, INVALID.get(fx))[1]
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
            if cc["take_alpha"] == "alpha" or fx in INVALID:
                base = plain(cc)
                assert all(px[i][3] == base[i][3] for i in range(W * H)), fx
                assert all(px[i] == clear for i in range(W * H) if base[i][3] == 0), fx

    assert c["FX-SHIFT-001"] == drawn
    two = c["FX-SHIFT-002"]
    assert is8(two, red, 40, 40, 200) and is8(two, blue, 255, 96, 64)
    assert is8(two, skin, 190, 214, 246) and all(same(two, xy) for xy in greys)
    assert is8(two, soft_skin, 190, 214, 246, a=half)
    three = c["FX-SHIFT-003"]
    assert is8(three, red, 40, 40, 40) and is8(three, green, 180, 180, 180)
    assert is8(three, skin, 214, 214, 214) and all(same(three, xy) for xy in greys)
    four = c["FX-SHIFT-004"]
    assert is8(four, red, *[74.016] * 3) and is8(four, green, *[147.268] * 3)
    assert all(same(four, xy) for xy in greys)
    five = c["FX-SHIFT-005"]
    g, b, s = 130 / 360 * 255, (60 * (4 - 32 / 191)) / 360 * 255, (60 * 24 / 56) / 360 * 255
    assert is8(five, red, 0, 0, 0) and is8(five, green, g, g, g) and is8(five, blue, b, b, b)
    assert is8(five, skin, s, s, s) and all(is8(five, xy, 0, 0, 0) for xy in greys)
    assert abs(g - 92.083) < 1e-3 and abs(b - 162.88) < 1e-2 and abs(s - 18.21) < 1e-2
    six = c["FX-SHIFT-006"]
    assert is8(six, red, 120, 120, 120) and is8(six, skin, 218, 218, 218)
    assert is8(six, blue, 159.5, 159.5, 159.5) and all(same(six, xy) for xy in greys)
    seven = c["FX-SHIFT-007"]
    assert is8(seven, red, 170, 170, 170) and is8(seven, green, 127.5, 127.5, 127.5)
    assert is8(seven, blue, 255, 255, 255) and all(is8(seven, xy, 0, 0, 0) for xy in greys)
    eight = c["FX-SHIFT-008"]
    for xy in (line, skin, red, green, blue, white, grey, black):
        assert is8(eight, xy, 255, 127.5, 0)
    assert is8(eight, soft_line, 255, 127.5, 0, a=half)
    nine = c["FX-SHIFT-009"]
    luma8 = lambda *e8: 0.2126 * e8[0] + 0.7152 * e8[1] + 0.0722 * e8[2]  # noqa: E731
    assert nine[at(*black)] == clear and same(nine, white)
    assert is8(nine, grey, 128, 128, 128, a=half)
    assert is8(nine, red, 200, 40, 40, a=74.016 / 255)
    assert abs(luma8(30, 26, 36) - 27.5724) < 1e-9
    assert is8(nine, line, 30, 26, 36, a=luma8(30, 26, 36) / 255)
    assert near(nine[at(*soft_line)], nine[at(*line)]) and near(nine[at(*soft_skin)],
                                                                nine[at(*skin)])
    assert all(nine[at(*xy)] == clear for xy in empty)
    ten = c["FX-SHIFT-010"]
    assert all(ten[i][3] == 1 for i in range(W * H))
    assert all(ten[at(*xy)] == [0.0, 0.0, 0.0, 1.0] for xy in empty)
    assert near(ten[at(*soft_line)], drawn[at(*line)]) and near(ten[at(*soft_skin)],
                                                                drawn[at(*skin)])
    assert all(same(ten, xy) for xy in (line, skin, red, white, black))
    assert all(p == clear for p in c["FX-SHIFT-011"])
    twelve = c["FX-SHIFT-012"]
    assert all(p[3] == 0.5 for p in twelve)
    assert all(twelve[at(*xy)] == [0.0, 0.0, 0.0, 0.5] for xy in empty)
    assert near(twelve[at(*soft_line)], twelve[at(*line)])
    assert near(twelve[at(*red)], [v / 2 for v in drawn[at(*red)]])
    thirteen = c["FX-SHIFT-013"]
    assert all(is8(thirteen, xy, 255, 255, 255) for xy in (line, skin, red, black))
    assert is8(thirteen, soft_line, 128, 128, 128, a=half)
    assert all(thirteen[at(*xy)] == clear for xy in empty)
    fourteen = c["FX-SHIFT-014"]
    assert all(fourteen[at(*xy)] == clear for xy in greys) and same(fourteen, blue)
    assert is8(fourteen, red, 200, 40, 40, a=2 / 3) and is8(fourteen, green, 60, 180, 80, a=0.5)
    fifteen = c["FX-SHIFT-015"]
    assert is8(fifteen, blue, b, 255, 159.5, a=luma8(64, 96, 255) / 255)
    assert is8(fifteen, white, 0, 0, 255) and fifteen[at(*black)] == clear
    moved = expected["cases"]["FX-SHIFT-016"]["frames"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == nine[at(0, y):at(W - 3, y)]
    for fx in INVALID:
        assert c[fx] == drawn
    print("checked")


if __name__ == "__main__":
    main()
