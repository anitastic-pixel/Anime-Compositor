"""Layer Style Gradient Overlay, worked a second way.

D-1005 adds After Effects' Gradient Overlay layer style, `core.style_gradient_overlay`. It joins
D-922's Inner Shadow and Drop Shadow: a style always runs after every effect, and the Gradient
Overlay is drawn first of the styles, then the Inner Shadows, then the Drop Shadows, each kind in
its listed order (Photoshop's stacking: the shadows lie over the overlay). It is this program's
own method, modelled on After Effects' and Photoshop's styles; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The settings, After Effects' rows: `blend_mode`, one of document 21's layer modes but the two
stencils, Normal when added; `opacity`, 0 to 100; `color_stops`, 2 to 16 of [location 0 to 100,
"#rrggbb"], locations never falling; `opacity_stops`, 2 to 16 of [location 0 to 100, opacity 0 to
100], likewise; `angle`, -3600 to 3600 degrees counter-clockwise from the right; `style`, one of
"linear", "radial", "angle", "reflected", "diamond"; `reverse`, "off" or "on"; `scale`, 10 to 150
per cent; `offset`, [x, y] pixels. Added: Normal, 100, white at 0 to black at 100, both opacity
stops 100, 90, linear, off, 100, [0, 0], After Effects' own.

The rule. The box is the layer's buffer as the style receives it, w by h. For the pixel centre P,
d = P - (w / 2, h / 2) - offset, y down; u = (cos a, -sin a), v = (sin a, cos a), and
L = (|w cos a| + |h sin a|) * scale / 100, the box's length along u.
  linear     t = 0.5 + d.u / L
  reflected  t = |d.u| / (L / 2)
  radial     t = |d| / (L / 2)
  diamond    t = (|d.u| + |d.v|) / (L / 2)
  angle      t = ((atan2(-dy, dx) in degrees - a) mod 360) / 360
With reverse on t is 1 - t; then t is held to 0..1. At location 100 t the colour C and the opacity
q are each read straight between the two stops round it, the first stop's below it and the last's
above, stops at the same location a hard step (the later one wins at it). Colours are mixed as
the working values (the linear values here; in an 8 bpc project the display values, as D-922's).
k = q / 100 * opacity / 100, and the pixel S = (C * k, k) is laid on the layer with document 21's
mode keeping the layer's alpha, D-828's "atop", as an Inner Shadow is. The layer never grows.

Left out, said in document 14: Gradient Smoothness (After Effects' 100, a smooth step the rule's
straight one only approximates), Align with Layer off, the stops' midpoints, and Dither.

**This file never runs the build's code path.** It works in double precision on lists.

The projects reuse D-922's plate (`Fixtures/layer_styles/media/plate.png`), go into
`Fixtures/layer_styles/fx_gover_NNN.json`, and the expected frames into
`Fixtures/layer_styles/expected_gradient_overlay.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/gradient_overlay_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import layer_styles_reference as L  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402

W, H = L.W, L.H
OUT = L.OUT
TOLERANCE = L.TOLERANCE
RANGES = {"opacity": (0, 100), "angle": (-3600, 3600), "scale": (10, 150)}
WHITE_BLACK = [[0, "#ffffff"], [100, "#000000"]]
SOLID = [[0, 100], [100, 100]]


# --- the rule -------------------------------------------------------------------------------

def linear_of(hex_):
    return [srgb_to_linear(v / 255) for v in L.R.hex_color(hex_.lower())]


def along(stops, t):
    """Straight between the stops round t, held at the ends; the later of two at one place."""
    at = 100 * t
    if at < stops[0][0]:
        return stops[0][1]
    for a, b in zip(stops, stops[1:]):
        if a[0] <= at < b[0]:
            f = (at - a[0]) / (b[0] - a[0])
            if isinstance(a[1], list):
                return [a[1][c] + (b[1][c] - a[1][c]) * f for c in range(3)]
            return a[1] + (b[1] - a[1]) * f
    return stops[-1][1]


def place(g, w, h, x, y):
    a = math.radians(g["angle"])
    ca, sa = math.cos(a), math.sin(a)
    dx = x + 0.5 - w / 2 - g["offset"][0]
    dy = y + 0.5 - h / 2 - g["offset"][1]
    du = dx * ca - dy * sa
    dv = dx * sa + dy * ca
    length = (abs(w * ca) + abs(h * sa)) * g["scale"] / 100
    s = g["style"]
    if s == "linear":
        t = 0.5 + du / length
    elif s == "reflected":
        t = abs(du) / (length / 2)
    elif s == "radial":
        t = math.hypot(dx, dy) / (length / 2)
    elif s == "diamond":
        t = (abs(du) + abs(dv)) / (length / 2)
    else:
        t = ((math.degrees(math.atan2(-dy, dx)) - g["angle"]) % 360) / 360
    if g["reverse"] == "on":
        t = 1 - t
    return min(1.0, max(0.0, t))


def overlay_mix(b, s):
    """Document 21's Overlay on encoded straight colours, the colour beneath first."""
    return 2 * b * s if b <= 0.5 else 1 - 2 * (1 - b) * (1 - s)


def kept(mode, src, dst):
    """D-828's mode keeping the layer's alpha; Overlay added here, the rest D-922's."""
    if mode != "overlay":
        return L.kept(mode, src, dst)
    cd, cs = L.unpremultiply(dst), L.unpremultiply(src)
    b = [L.lin(overlay_mix(L.enc(min(1.0, cd[c])), L.enc(min(1.0, cs[c])))) for c in range(3)]
    return [(1 - src[3]) * dst[c] + src[3] * dst[3] * b[c] for c in range(3)] + [dst[3]]


def gradient_overlay(layer, g):
    colors = [[s[0], linear_of(s[1])] for s in g["color_stops"]]
    out = []
    w, h = layer["w"], layer["h"]
    for y in range(h):
        for x in range(w):
            t = place(g, w, h, x, y)
            c = along(colors, t)
            k = along(g["opacity_stops"], t) / 100 * g["opacity"] / 100
            out.append(kept(g["blend_mode"], [c[0] * k, c[1] * k, c[2] * k, k],
                            layer["px"][y * w + x]))
    return dict(layer, px=out)


# --- the cases ------------------------------------------------------------------------------

def overlay(blend_mode="normal", opacity=100, color_stops=None, opacity_stops=None, angle=90,
            style="linear", reverse="off", scale=100, offset=(0, 0)):
    return {"kind": "gover", "blend_mode": blend_mode, "opacity": opacity,
            "color_stops": color_stops or WHITE_BLACK, "opacity_stops": opacity_stops or SOLID,
            "angle": angle, "style": style, "reverse": reverse, "scale": scale,
            "offset": list(offset)}


def render(styles, frame_no):
    layer = L.drawn_layer()
    order = [s for s in styles if s["kind"] == "gover"] + \
            [s for s in styles if s["kind"] == "inner"] + \
            [s for s in styles if s["kind"] == "drop"]
    for s in order:
        if s["kind"] == "gover":
            now = dict(s, **{k: L.held(value_at(s[k], frame_no), r) for k, r in RANGES.items()})
            layer = gradient_overlay(layer, now)
        else:
            now = dict(s, **{k: L.held(value_at(s[k], frame_no), r) for k, r in L.RANGES.items()})
            now["angle"] = 120 if now["use_global_light"] == "on" else now["angle"]
            layer = (L.inner_shadow if s["kind"] == "inner" else L.drop_shadow)(layer, now)
    return L.frame(layer)


RED, BLUE, ORANGE, NAVY = "#ff0000", "#0000ff", "#ff8000", "#232444"
CASES = {
    "FX-GOVER-001": ("Gradient Overlay as added: Normal, 100, white at 0 to black at 100, angle "
                     "90, linear, scale 100. The plate turns from black along its top row to "
                     "white along its bottom row (location 0 is where the angle points away "
                     "from); its alpha never changes, so the empty corners stay empty.",
                     [overlay()], [0]),
    "FX-GOVER-002": ("Angle 0, red at 0 to blue at 100: red down the plate's left edge, blue "
                     "down its right, the same in every row.",
                     [overlay(color_stops=[[0, RED], [100, BLUE]], angle=0)], [0]),
    "FX-GOVER-003": ("The tutorial's (PVUPTcfJ5HY 4:00-6:40, pv_400): Overlay, 100, #232444 at "
                     "both colour stops, opacity 0 at 65.6 and 100 at 100, angle 90, Radial, "
                     "scale 150. Only the plate's two left-hand corners, the pixels "
                     "farthest from the box's centre, darken, like a vignette's edge; the rest is "
                     "untouched.",
                     [overlay(blend_mode="overlay", color_stops=[[0, NAVY], [100, NAVY]],
                              opacity_stops=[[65.6, 0], [100, 100]], style="radial",
                              scale=150)], [0]),
    "FX-GOVER-004": ("Reflected, angle 0, red at 0 to blue at 100: red down the middle, blue at "
                     "both side edges, left and right the same.",
                     [overlay(color_stops=[[0, RED], [100, BLUE]], angle=0,
                              style="reflected")], [0]),
    "FX-GOVER-005": ("Angle style at 0, red to blue: the colour goes round the box's centre "
                     "counter-clockwise from the right, red just above the right-hand middle, "
                     "blue just below it.",
                     [overlay(color_stops=[[0, RED], [100, BLUE]], angle=0, style="angle")],
                     [0]),
    "FX-GOVER-006": ("Diamond, angle 0, red to blue: red in the middle, a diamond of blue "
                     "growing out to the plate's corners.",
                     [overlay(color_stops=[[0, RED], [100, BLUE]], angle=0,
                              style="diamond")], [0]),
    "FX-GOVER-007": ("FX-GOVER-001 with Reverse on: white along the top, black along the "
                     "bottom.",
                     [overlay(reverse="on")], [0]),
    "FX-GOVER-008": ("Radial, red to blue, offset 3 pixels right: the red middle moves three "
                     "columns right of the box's centre.",
                     [overlay(color_stops=[[0, RED], [100, BLUE]], style="radial",
                              offset=(3, 0))], [0]),
    "FX-GOVER-009": ("Angle keyed from 0 at frame 0 to 180 at frame 4, linear, red to blue: at "
                     "frame 0 red on the left, at frame 2 (90) red on the bottom, at frame 4 "
                     "red on the right.",
                     [overlay(color_stops=[[0, RED], [100, BLUE]],
                              angle=keyed((0, 0), (4, 180)))], [0, 2, 4]),
    "FX-GOVER-010": ("Three colour stops, red at 0, #ff8000 at 50 and blue at 100, angle 0, "
                     "scale 50: red over the left quarter, through orange at the middle, blue "
                     "past the right quarter.",
                     [overlay(color_stops=[[0, RED], [50, ORANGE], [100, BLUE]], angle=0,
                              scale=50)], [0]),
    "FX-GOVER-011": ("Opacity 0: the plate untouched.", [overlay(opacity=0)], [0]),
    "FX-GOVER-012": ("Multiply, #ff8000 to blue, angle 0: the skin multiplied by orange on the "
                     "left, by blue on the right.",
                     [overlay(blend_mode="multiply", color_stops=[[0, ORANGE], [100, BLUE]],
                              angle=0)], [0]),
    "FX-GOVER-013": ("An Inner Shadow listed first (Normal, #ff8000, 100, Global Light off, "
                     "angle 0, distance 2, size 0), then the Gradient Overlay as added: the "
                     "overlay is drawn first, so the plate's two right-hand columns are orange "
                     "over the black-to-white ramp.",
                     [L.style(blend_mode="normal", color=ORANGE, opacity=100,
                              use_global_light="off", angle=0, distance=2, size=0),
                      overlay()], [0]),
    "FX-GOVER-014": ("Opacity stops 100 at 0 and 0 at 100, angle 0, red to red: solid red on "
                     "the left fading to the plate's own skin on the right.",
                     [overlay(color_stops=[[0, RED], [100, RED]],
                              opacity_stops=[[0, 100], [100, 0]], angle=0)], [0]),
    "FX-GOVER-015": ("Two stops at the same place, red at 0 and 50, blue at 50 and 100, angle 0: "
                     "a hard step from red to blue down the middle, no blend between.",
                     [overlay(color_stops=[[0, RED], [50, RED], [50, BLUE], [100, BLUE]],
                              angle=0)], [0]),
}

# Outside the contract in a file. D-46: the file is read, the style is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GOVER-016": ("Colour stops at 60 then 40, falling.",
                     overlay(color_stops=[[60, RED], [40, BLUE]])),
    "FX-GOVER-017": ("A colour stop at 101, past 100.",
                     overlay(color_stops=[[0, RED], [101, BLUE]])),
    "FX-GOVER-018": ("An opacity stop of 101 per cent.",
                     overlay(opacity_stops=[[0, 100], [100, 101]])),
    "FX-GOVER-019": ("A colour written \"black\".",
                     overlay(color_stops=[[0, "#ffffff"], [100, "black"]])),
    "FX-GOVER-020": ("One colour stop only.", overlay(color_stops=[[0, RED]])),
    "FX-GOVER-021": ("Style \"spiral\".", overlay(style="spiral")),
    "FX-GOVER-022": ("Scale 151, above 150.", overlay(scale=151)),
    "FX-GOVER-023": ("Blend mode \"stencil_luma\", a stencil, not a style's mode.",
                     overlay(blend_mode="stencil_luma")),
    "FX-GOVER-024": ("Reverse written \"yes\".", overlay(reverse="yes")),
    "FX-GOVER-025": ("An opacity stop of three numbers.",
                     overlay(opacity_stops=[[0, 100, 5], [100, 100]])),
}


# --- the project files ----------------------------------------------------------------------

def style_json(i, s):
    if s["kind"] != "gover":
        return L.style_json(i, s)
    p = {"blend_mode": s["blend_mode"], "opacity": setting_json(s["opacity"]),
         "color_stops": s["color_stops"], "opacity_stops": s["opacity_stops"],
         "angle": setting_json(s["angle"]), "style": s["style"], "reverse": s["reverse"],
         "scale": setting_json(s["scale"]), "offset": s["offset"]}
    return {"instance_id": f"fx-0-{i}", "type_id": "core.style_gradient_overlay",
            "enabled": True, "parameters": p}


def write(fx, styles):
    p = L.project_json(fx, L.case())
    p["compositions"][0]["layers"][0]["effects"] = [style_json(i, s) for i, s in enumerate(styles)]
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    drawn = L.plain(None)
    for fx, (says, styles, frames) in CASES.items():
        rendered = {str(f): render(styles, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, styles), "frames": rendered}
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != drawn[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, s) in INVALID.items():
        says += (" The file is read, the style is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, [s]),
                                 "frames": {"0": drawn, "4": drawn},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_gradient_overlay.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = L.plain(None)
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-9: all(abs(p - q) < e for p, q in zip(a, b))  # noqa: E731
    red, blue = linear_of(RED), linear_of(BLUE)

    for name, frames in c.items():
        for f in frames.values():
            assert all(p[3] == drawn[i][3] for i, p in enumerate(f)), name  # alpha kept
    one = c["FX-GOVER-001"]["0"]
    assert one[at(5, 2)][0] < one[at(5, 4)][0] < one[at(5, 7)][0]
    assert near(one[at(3, 2)], one[at(11, 2)])  # the same along a row
    assert one[at(0, 0)] == drawn[at(0, 0)]
    two = c["FX-GOVER-002"]["0"]
    assert two[at(3, 2)][0] > two[at(3, 2)][2] and two[at(11, 2)][2] > two[at(11, 2)][0]
    three = c["FX-GOVER-003"]["0"]
    assert near(three[at(7, 4)], drawn[at(7, 4)]) and near(three[at(8, 5)], drawn[at(8, 5)])
    assert three[at(3, 2)][1] < drawn[at(3, 2)][1] - 0.01  # the far corner darkened
    four = c["FX-GOVER-004"]["0"]
    assert near(four[at(4, 4)], four[at(11, 4)]) and four[at(7, 4)][0] > four[at(4, 4)][0]
    five = c["FX-GOVER-005"]["0"]
    assert five[at(11, 4)][0] > 0.9 and five[at(11, 5)][2] > 0.9
    six = c["FX-GOVER-006"]["0"]
    assert six[at(7, 4)][0] > six[at(3, 2)][0] and six[at(3, 2)][2] > six[at(7, 4)][2]
    seven = c["FX-GOVER-007"]["0"]
    assert seven[at(5, 2)][0] > seven[at(5, 7)][0]
    eight = c["FX-GOVER-008"]["0"]
    assert eight[at(10, 4)][0] > eight[at(7, 4)][0]
    nine = c["FX-GOVER-009"]
    assert nine["0"][at(3, 4)][0] > nine["0"][at(11, 4)][0]
    assert nine["2"][at(5, 7)][0] > nine["2"][at(5, 2)][0]
    assert nine["4"][at(11, 4)][0] > nine["4"][at(3, 4)][0]
    ten = c["FX-GOVER-010"]["0"]
    assert near(ten[at(3, 4)], red + [1.0]) and near(ten[at(12, 4)], [v * 128 / 255 for v in blue] + [128 / 255])
    assert c["FX-GOVER-011"]["0"] == drawn
    twelve = c["FX-GOVER-012"]["0"]
    assert twelve[at(11, 4)][0] < twelve[at(3, 4)][0] / 2 and twelve[at(3, 4)][2] < drawn[at(3, 4)][2]
    thirteen = c["FX-GOVER-013"]["0"]
    orange = linear_of(ORANGE)
    assert near(thirteen[at(11, 4)], orange + [1.0]) and thirteen[at(5, 4)] == one[at(5, 4)]
    fourteen = c["FX-GOVER-014"]["0"]
    assert fourteen[at(3, 4)][1] < fourteen[at(11, 4)][1] < drawn[at(11, 4)][1] + 1e-12
    fifteen = c["FX-GOVER-015"]["0"]
    assert near(fifteen[at(7, 4)], red + [1.0]) and near(fifteen[at(8, 4)], blue + [1.0])
    print("checked")


if __name__ == "__main__":
    main()
