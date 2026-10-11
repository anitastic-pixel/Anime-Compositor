"""Solid Composite, worked a second way.

D-312 (B-195) added `core.solid_composite`, after After Effects' Solid Composite (Channel): the
layer is laid over a solid colour, as P-26's Advanced Electric tutorial lays its lightning on a
solid. B-195 pinned it on hand-worked solids; D-560 (B-440) gives it this fixture folder. It is
this program's own method, modelled on After Effects' Solid Composite; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. `source_opacity` and `opacity`, each 0..100 and keyable, each held inside 0..100 at
each frame; `color`, `#rrggbb`; `blend`, "normal", "add", "screen" or "multiply". Starting at
100, `#ffffff`, 100, normal. With k = source_opacity / 100, o = opacity / 100 and the solid
d = (srgb_to_linear(color) o, o), each premultiplied pixel p becomes the layer blend of
document 21 with s = k p as the source and d as the destination: normal is s + (1 - s.a) d;
otherwise, with straight colours cs = s.rgb / s.a and cd = d.rgb / d.a (0 where the alpha is
0), B(cs, cd) is cs cd for multiply, cs + cd - cs cd for screen and min(cs + cd, 1) for add, and
each colour channel is (1 - s.a) d + (1 - d.a) s + s.a d.a B, the alpha s.a + d.a - s.a d.a.
So the solid shows wherever the layer does not, and fills the layer's whole frame. The layer
does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Channel Mixer's swatches (tools/channel_mixer_reference.py). The drawing goes into
`Fixtures/solid_composite/media`, the projects into `Fixtures/solid_composite`, and the expected
frames into `Fixtures/solid_composite/expected_solid_composite.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/solid_composite_reference.py
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
import channel_mixer_reference as CM  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "solid_composite"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("source_opacity", "color", "opacity", "blend")
START = {"source_opacity": 100, "color": "#ffffff", "opacity": 100, "blend": "normal"}


# --- the rule -------------------------------------------------------------------------------

def straight(p):
    return [v / p[3] for v in p[:3]] if p[3] != 0 else [0.0] * 3


def blend(mode, s, d):
    """Document 21's layer blend, `s` the source over the destination `d`, both premultiplied."""
    if mode == "normal":
        return [s[c] + (1 - s[3]) * d[c] for c in range(4)]
    cs, cd = straight(s), straight(d)
    b = {"multiply": lambda x, y: x * y,
         "screen": lambda x, y: x + y - x * y,
         "add": lambda x, y: min(x + y, 1.0)}[mode]
    a_s, a_d = s[3], d[3]
    return [(1 - a_s) * d[c] + (1 - a_d) * s[c] + a_s * a_d * b(cs[c], cd[c])
            for c in range(3)] + [a_s + a_d - a_s * a_d]


def hex_color(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def solid_composite(pixels, source_opacity, color, opacity, mode):
    """`pixels` are 8-bit straight RGBA."""
    k, o = source_opacity / 100, opacity / 100
    d = [srgb_to_linear(v) * o for v in hex_color(color)] + [o]
    return [blend(mode, [v * k for v in R.working(p)], d) for p in pixels]


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = CM.DRAWINGS  # the swatches


def case(shift=0, **settings):
    return {"drawing": "swatches", "shift": shift, **START, **settings}


def held(c, k, frame_no):
    return min(100, max(0, value_at(c[k], frame_no)))


def render(c, frame_no=0):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(solid_composite(pixels, held(c, "source_opacity", frame_no), c["color"],
                                   held(c, "opacity", frame_no), c["blend"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame([R.working(p) for p in pixels], c["shift"])


BLUE = "#3366cc"
ADD = {"blend": "add", "color": "#404040"}

CASES = {
    "FX-SOLIDC-001": ("The settings as they start, source opacity 100, colour #ffffff, opacity "
                      "100, normal: the drawing over white: the empty border turns white, the "
                      "soft edges the line and the skin half over white, every solid pixel as "
                      "it was.",
                      case(), [0]),
    "FX-SOLIDC-002": ("Colour #3366cc: the drawing over a blue: the border turns #3366cc, every "
                      "solid pixel as it was.",
                      case(color=BLUE), [0]),
    "FX-SOLIDC-003": ("Colour #3366cc at opacity 50: the border is the blue at half covering, a "
                      "soft edge covers three quarters, every solid pixel as it was.",
                      case(color=BLUE, opacity=50), [0]),
    "FX-SOLIDC-004": ("Opacity 0: no solid at all: the drawing, untouched.",
                      case(color=BLUE, opacity=0), [0]),
    "FX-SOLIDC-005": ("Source opacity 50 over the blue: the drawing at half strength over a "
                      "solid blue, everything opaque.",
                      case(color=BLUE, source_opacity=50), [0]),
    "FX-SOLIDC-006": ("Source opacity 0: the blue alone, every pixel #3366cc.",
                      case(color=BLUE, source_opacity=0), [0]),
    "FX-SOLIDC-007": ("Add, colour #404040: every solid pixel raised by the grey (in linear "
                      "light, held at the top): the black turns #404040, the white stays white; "
                      "the border is the grey.",
                      case(**ADD), [0]),
    "FX-SOLIDC-008": ("Screen, colour #808080: every solid pixel lightened, the black turns "
                      "#808080, the white stays white; the border is the grey.",
                      case(blend="screen", color="#808080"), [0]),
    "FX-SOLIDC-009": ("Multiply, colour #ff8040: every solid pixel darkened by the orange: the "
                      "white turns #ff8040, the black stays black; the border is the orange.",
                      case(blend="multiply", color="#ff8040"), [0]),
    "FX-SOLIDC-010": ("Multiply, colour #ff8040 at opacity 60, source opacity 70: the drawing at "
                      "70 per cent multiplied with the orange at 60, where both show.",
                      case(blend="multiply", color="#ff8040", opacity=60, source_opacity=70),
                      [0]),
    "FX-SOLIDC-011": ("Colour #3366cc, opacity keyed from 0 at frame 0 to 100 at frame 4, "
                      "linear: frame 0 the drawing untouched, frame 2 FX-SOLIDC-003 (opacity "
                      "50), frame 4 FX-SOLIDC-002.",
                      case(color=BLUE, opacity=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-SOLIDC-012": ("Colour #3366cc, source opacity eased from 0 at frame 0 to 100 at frame 4 "
                      "on a curve that overshoots: frame 0 FX-SOLIDC-006; at frame 2 it has gone "
                      "past 100 and is held there, so frames 2 and 4 are both FX-SOLIDC-002.",
                      case(color=BLUE, source_opacity=keyed((0, 0, OVERSHOOT), (4, 100))),
                      [0, 2, 4]),
    "FX-SOLIDC-013": ("FX-SOLIDC-007 moved three pixels right: the same, moved; the grey fills "
                      "only the layer, so columns 0 to 2 of the frame stay empty.",
                      case(shift=3, **ADD), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SOLIDC-014": ("Opacity 101: above 100.", case(opacity=101)),
    "FX-SOLIDC-015": ("Source opacity -1: below 0.", case(source_opacity=-1)),
    "FX-SOLIDC-016": ("Blend \"difference\": this program's Solid Composite blends normal, add, "
                      "screen or multiply.", case(blend="difference")),
    "FX-SOLIDC-017": ("Colour \"white\": a colour is written #rrggbb.", case(color="white")),
    "FX-SOLIDC-018": ("Colour \"#12345\": five digits.", case(color="#12345")),
    "FX-SOLIDC-019": ("Opacity keyed to 150 at frame 4.",
                      case(opacity=keyed((0, 100), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.solid_composite", "enabled": True,
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

    (OUT / "expected_solid_composite.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    f = {fx: v["frames"] for fx, v in expected["cases"].items()}
    c = {fx: v["0"] for fx, v in f.items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda g, h: all(near(p, q) for p, q in zip(g, h))  # noqa: E731
    lin = lambda a, *e8: [srgb_to_linear(v / 255) * a for v in e8] + [a]  # noqa: E731
    is8 = lambda g, xy, *e8, a=1.0: near(g[at(*xy)], lin(a, *e8))  # noqa: E731
    same = lambda g, xy: near(g[at(*xy)], drawn[at(*xy)])  # noqa: E731
    line, skin, red, white, grey, black = (2, 4), (3, 4), (5, 4), (11, 4), (12, 4), (13, 4)
    solid = (line, skin, red, (7, 4), (9, 4), white, grey, black)
    soft_line = (1, 2)
    empty = [(0, 4), (15, 4), (4, 0), (4, 9)]
    blue = (0x33, 0x66, 0xcc)

    # The rule's own pieces.
    one, none = [0.2, 0.4, 0.6, 1.0], [0.0] * 4
    assert blend("normal", one, [1, 1, 1, 1]) == one and blend("normal", none, one) == one
    assert blend("multiply", one, [1, 1, 1, 1]) == one
    assert near(blend("screen", one, [0, 0, 0, 1]), one)
    assert blend("add", [0.8, 0.8, 0.8, 1], [0.5, 0.5, 0.5, 1])[:3] == [1, 1, 1]
    assert blend("add", none, [0.3, 0.3, 0.3, 0.5]) == [0.3, 0.3, 0.3, 0.5]
    assert hex_color("#ff8040") == [1, 128 / 255, 64 / 255]

    # Every case keeps colour inside its covering.
    for frames in f.values():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                      for v in p[:3])

    one = c["FX-SOLIDC-001"]
    assert all(same(one, xy) for xy in solid) and all(is8(one, xy, 255, 255, 255) for xy in empty)
    s = drawn[at(*soft_line)]
    assert near(one[at(*soft_line)], [v + (1 - s[3]) * 1 for v in s[:3]] + [1.0])
    two = c["FX-SOLIDC-002"]
    assert all(same(two, xy) for xy in solid) and all(is8(two, xy, *blue) for xy in empty)
    three = c["FX-SOLIDC-003"]
    assert all(same(three, xy) for xy in solid) and all(is8(three, xy, *blue, a=0.5)
                                                        for xy in empty)
    assert abs(three[at(*soft_line)][3] - (s[3] + (1 - s[3]) * 0.5)) < 1e-12
    assert c["FX-SOLIDC-004"] == drawn
    five = c["FX-SOLIDC-005"]
    assert all(p[3] == 1 for p in five)
    assert near(five[at(*white)], [0.5 + 0.5 * srgb_to_linear(v / 255) for v in blue] + [1])
    assert all(is8(c["FX-SOLIDC-006"], (x, y), *blue) for x in range(W) for y in range(H))
    seven = c["FX-SOLIDC-007"]
    assert is8(seven, black, 64, 64, 64) and same(seven, white)
    assert all(is8(seven, xy, 64, 64, 64) for xy in empty)
    g = srgb_to_linear(64 / 255)
    assert near(seven[at(*red)], [min(1, srgb_to_linear(v / 255) + g) for v in (200, 40, 40)]
                + [1])
    eight = c["FX-SOLIDC-008"]
    assert is8(eight, black, 128, 128, 128) and same(eight, white)
    assert all(eight[at(*xy)][0] >= drawn[at(*xy)][0] for xy in solid)
    nine = c["FX-SOLIDC-009"]
    assert is8(nine, white, 255, 128, 64) and same(nine, black)
    assert all(is8(nine, xy, 255, 128, 64) for xy in empty)
    assert all(nine[at(*xy)][2] <= drawn[at(*xy)][2] for xy in solid)
    ten = c["FX-SOLIDC-010"]
    assert abs(ten[at(*white)][3] - (0.7 + 0.6 - 0.42)) < 1e-12
    assert all(is8(ten, xy, 255, 128, 64, a=0.6) for xy in empty)
    eleven = f["FX-SOLIDC-011"]
    assert eleven["0"] == drawn and like(eleven["2"], three) and like(eleven["4"], two)
    twelve = f["FX-SOLIDC-012"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert value_at(case(source_opacity=keyed((0, 0, OVERSHOOT), (4, 100)))["source_opacity"],
                    2) > 100
    assert like(twelve["0"], c["FX-SOLIDC-006"]) and twelve["2"] == twelve["4"]
    assert like(twelve["4"], two)
    moved = f["FX-SOLIDC-013"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == seven[at(0, y):at(W - 3, y)]
        assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3))
    for fx in INVALID:
        assert c[fx] == drawn
    print("checked")


if __name__ == "__main__":
    main()
