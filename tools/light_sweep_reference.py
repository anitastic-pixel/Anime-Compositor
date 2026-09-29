"""Light sweep, worked a second way.

D-199 adds `core.light_sweep`, After Effects' CC Light Sweep by a rule of our own: a band of
light lies across the drawing along a line, brightest on the line and fading to nothing at the
band's sides, and lights the drawing's edges more than its middle, the shine that runs across a
sword, a logo or a window pane when the band's centre is keyed to move. Only what is drawn is
lit; the empty parts stay empty. It is modelled on CC Light Sweep and not claimed to match it;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an
earlier effect may have grown; the input is not grown. The band's centre line runs through
C = (center_x / 100 W, center_y / 100 H) along the direction `direction` degrees clockwise from
up, and n is the unit vector across it, `direction + 90` degrees clockwise from up, both exact at
quarter turns. At a pixel whose centre is P in the drawing's own space, s = |n . (P - C)| and,
with r = width / 2, the band's strength p is: "linear", max(0, 1 - s / r); "smooth", 1 - t^2 (3 -
2 t) with t = s / r when t < 1, else 0; "sharp", clamp(r + 0.5 - s, 0, 1); and 0 everywhere
when the width is 0. With a the pixel's covering and k = floor(edge_thickness), the edge e is a
less the least covering in the square of side 2 k + 1 round the pixel, the input buffer's
outside counting as uncovered. The light at a pixel with a > 0 is L = p (sweep_intensity / 100
+ edge_intensity / 100 * e / a), and with K the light colour's linear value and O the pixel as it
is, premultiplied: "add" gives (O_rgb + L a K, a); "composite" gives (O_rgb + m (a K - O_rgb), a)
with m = min(L, 1); and "cutout" gives (m a K, m a), the drawing cut away and only the light
kept. A pixel with a = 0 is left as it is. A pixel whose light is 0 is left exactly as it is,
except under "cutout", where it is cleared.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Rim Light's figure, imported from `tools/rim_light_reference.py`. The drawing goes
into `Fixtures/light_sweep/media`, the projects into `Fixtures/light_sweep`, and the expected
frames into `Fixtures/light_sweep/expected_light_sweep.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/light_sweep_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as RC  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from rim_light_reference import DRAWINGS, toward  # noqa: E402

W, H = RC.W, RC.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "light_sweep"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "direction": (-3600, 3600), "width": (0, 10000),
          "sweep_intensity": (0, 100), "edge_intensity": (0, 100), "edge_thickness": (1, 50)}
NUMBERS = tuple(RANGES)
WORDS = ("shape", "light_color", "light_reception")
EMPTY = [0.0] * 4
ORANGE = "#ffb040"


# --- the rule -------------------------------------------------------------------------------

def strength(shape, s, r):
    if r == 0:
        return 0.0
    if shape == "linear":
        return max(0.0, 1 - s / r)
    if shape == "smooth":
        t = s / r
        return 1 - t * t * (3 - 2 * t) if t < 1 else 0.0
    return min(1.0, max(0.0, r + 0.5 - s))


def edge(layer, i, k):
    """The pixel's covering less the least covering in the square round it, outside uncovered."""
    w, h, px = layer["w"], layer["h"], layer["px"]
    x, y = i % w, i // w
    least = min((px[v * w + u][3] if 0 <= u < w and 0 <= v < h else 0.0)
                for v in range(y - k, y + k + 1) for u in range(x - k, x + k + 1))
    return px[i][3] - least


def light_sweep(layer, center, direction, shape, width, sweep, edge_intensity, thickness,
                light_color, reception, dw, dh):
    """The layer lit; its rectangle kept. The drawing's own space is the layer's, its corner at
    the layer's (0, 0) and the layer's `left` and `top` the growth of earlier effects, negated."""
    cx, cy = center[0] / 100 * dw, center[1] / 100 * dh
    n = toward(direction + 90)
    r = width / 2
    k = math.floor(thickness)
    K = [srgb_to_linear(v / 255) for v in RC.hex_color(light_color.lower())]
    out = []
    for i, o in enumerate(layer["px"]):
        a = o[3]
        if a == 0:
            out.append(list(o))
            continue
        px = layer["left"] + i % layer["w"] + 0.5
        py = layer["top"] + i // layer["w"] + 0.5
        p = strength(shape, abs(n[0] * (px - cx) + n[1] * (py - cy)), r)
        L = p * (sweep / 100 + (edge_intensity / 100 * edge(layer, i, k) / a
                                if p and edge_intensity else 0))
        if L == 0 and reception != "cutout":
            out.append(list(o))
            continue
        m = min(L, 1.0)
        if reception == "add":
            out.append([o[c] + L * a * K[c] for c in range(3)] + [a])
        elif reception == "composite":
            out.append([o[c] + m * (a * K[c] - o[c]) for c in range(3)] + [a])
        else:
            out.append([m * a * K[c] for c in range(3)] + [m * a])
    return dict(layer, px=out)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, tile=False, center=(50, 50), direction=-30, shape="smooth", width=50,
         sweep_intensity=50, edge_intensity=100, edge_thickness=1, light_color="#ffffff",
         light_reception="add"):
    return {"drawing": "figure", "shift": shift, "tile": tile, "center": center,
            "direction": direction, "shape": shape, "width": width,
            "sweep_intensity": sweep_intensity, "edge_intensity": edge_intensity,
            "edge_thickness": edge_thickness, "light_color": light_color,
            "light_reception": light_reception}


def band(**kw):
    """An upright band five pixels wide down the middle, the sweep alone at full strength."""
    return case(**{"direction": 0, "shape": "linear", "width": 5, "sweep_intensity": 100,
                   "edge_intensity": 0, **kw})


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    return tuple(min(hi, max(lo, e)) for e in v) if isinstance(v, (tuple, list)) else min(hi, max(lo, v))


def drawn_layer(name):
    return {"px": [RC.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    if c["tile"]:
        layer = motion_tile(layer, 300, 300, "off")
    s = {k: held(c, k, frame_no) for k in NUMBERS}
    lit = light_sweep(layer, s["center"], s["direction"], c["shape"], s["width"],
                      s["sweep_intensity"], s["edge_intensity"], s["edge_thickness"],
                      c["light_color"], c["light_reception"], W, H)
    return frame(lit, c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-SWEEP-001": ("The settings as they start: centre (50, 50), direction -30, smooth, width "
                     "50, sweep intensity 50, edge intensity 100, edge thickness 1, white, add. "
                     "The band, wider than the frame, lights the whole figure, most along a line "
                     "through the middle leaning left, and its edges most of all: the block's "
                     "border, the soft edge and the lone red dot.",
                     case(), [0]),
    "FX-SWEEP-002": ("Width 0: no band, the drawing untouched.", case(width=0), [0]),
    "FX-SWEEP-003": ("Sweep intensity 0 and edge intensity 0: no light, the drawing untouched.",
                     case(sweep_intensity=0, edge_intensity=0), [0]),
    "FX-SWEEP-004": ("Direction 0, linear, width 5, sweep intensity 100, edge intensity 0: an "
                     "upright band down the middle of the frame, x = 8; columns 7 and 8 gain 0.8 "
                     "of white, columns 6 and 9 0.4, and the rest is untouched.",
                     band(), [0]),
    "FX-SWEEP-005": ("As FX-SWEEP-004, smooth: columns 7 and 8 gain 0.896 of white and columns 6 "
                     "and 9 0.352.",
                     band(shape="smooth"), [0]),
    "FX-SWEEP-006": ("As FX-SWEEP-004, sharp: columns 6 to 9 gain all of white and columns 5 and "
                     "10, the band's soft sides, half.",
                     band(shape="sharp"), [0]),
    "FX-SWEEP-007": ("As FX-SWEEP-004, direction 90, width 3: a band lying across rows 4 and 5, "
                     "which gain two thirds of white; the red dot, in row 5, is lit too.",
                     band(direction=90, width=3), [0]),
    "FX-SWEEP-008": ("As FX-SWEEP-004, direction 45, width 6: the band leans right, from the "
                     "lower left to the upper right through the middle.",
                     band(direction=45, width=6), [0]),
    "FX-SWEEP-009": ("The edges alone: width 10000, sweep intensity 0, edge intensity 100. The "
                     "block's outer ring of pixels, the soft edge and the red dot are lit about "
                     "wholly white; the column beside the soft edge, half covered on its left, "
                     "half; the block's middle is untouched.",
                     case(width=10000, sweep_intensity=0), [0]),
    "FX-SWEEP-010": ("As FX-SWEEP-009, edge thickness 2.7, counted as 2: the ring two pixels "
                     "deep.",
                     case(width=10000, sweep_intensity=0, edge_thickness=2.7), [0]),
    "FX-SWEEP-011": ("As FX-SWEEP-004 in orange #ffb040: columns 7 and 8 gain 0.8 of orange.",
                     band(light_color=ORANGE), [0]),
    "FX-SWEEP-012": ("As FX-SWEEP-011, composite: the band paints the drawing orange, columns 7 "
                     "and 8 four fifths of the way and columns 6 and 9 two fifths, never "
                     "brighter than the orange.",
                     band(light_color=ORANGE, light_reception="composite"), [0]),
    "FX-SWEEP-013": ("As FX-SWEEP-004, cutout: only the band's light is left, white in columns 7 "
                     "and 8 at 0.8 covering and in columns 6 and 9 at 0.4, the rest clear.",
                     band(light_reception="cutout"), [0]),
    "FX-SWEEP-014": ("As FX-SWEEP-013, edge intensity 100: the band's cut-out light is whole "
                     "where it crosses the block's top and foot, the edges adding to it, "
                     "held at full; inside, 0.8 as before.",
                     band(light_reception="cutout", edge_intensity=100), [0]),
    "FX-SWEEP-015": ("As FX-SWEEP-004, the centre keyed from (20, 50) at frame 0 to (80, 50) at "
                     "frame 4, linear: the band sweeps across left to right, over the soft edge "
                     "at frame 0, down the middle at frame 2 (FX-SWEEP-004) and over the red "
                     "dot at frame 4.",
                     band(center=keyed((0, (20, 50)), (4, (80, 50)))), [0, 2, 4]),
    "FX-SWEEP-016": ("As FX-SWEEP-004, the sweep intensity eased from 0 at frame 0 to 100 at "
                     "frame 4 past its end: frame 0 is the drawing, and frame 2 is held at 100, "
                     "FX-SWEEP-004.",
                     band(sweep_intensity=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-SWEEP-017": ("As FX-SWEEP-007, direction 450, a turn and a quarter: FX-SWEEP-007 "
                     "exactly.",
                     band(direction=450, width=3), [0]),
    "FX-SWEEP-018": ("As FX-SWEEP-004, the layer moved three pixels right: the band moves with "
                     "it, down x = 11.",
                     band(shift=3), [0]),
    "FX-SWEEP-019": ("Motion Tile at 300% by 300% before it, width 7, the band's centre at (0, 50), the "
                     "drawing's left edge, the layer moved eight pixels right: the band crosses "
                     "from the drawing into the tile on its left, lighting the soft edge in "
                     "column 10 and the tile's red dot in column 6, and nowhere else.",
                     band(shift=8, tile=True, center=(0, 50), width=7), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SWEEP-020": ("Width -1, below 0.", case(width=-1)),
    "FX-SWEEP-021": ("Edge thickness 51, above 50.", case(edge_thickness=51)),
    "FX-SWEEP-022": ("Edge thickness 0.5, below 1.", case(edge_thickness=0.5)),
    "FX-SWEEP-023": ("Sweep intensity keyed to 150 at frame 4.",
                     case(sweep_intensity=keyed((0, 50), (4, 150)))),
    "FX-SWEEP-024": ("Light reception \"glow\", which is not one.",
                     case(light_reception="glow")),
    "FX-SWEEP-025": ("Shape \"round\", which is not one.", case(shape="round")),
    "FX-SWEEP-026": ("Light colour \"#fff\", written in three digits, not six.",
                     case(light_color="#fff")),
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
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "off"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.light_sweep",
                    "enabled": True,
                    "parameters": {k: setting_json(c[k]) for k in NUMBERS + WORDS}})
    comp["layers"][0]["effects"] = effects
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

    (OUT / "expected_light_sweep.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    gain = lambda f, x, y: f[at(x, y)][2] - drawn[at(x, y)][2]  # noqa: E731  blue, per covering
    shown = [(x, y) for y in range(H) for x in range(W) if drawn[at(x, y)][3] > 0]
    empty = [(x, y) for y in range(H) for x in range(W) if drawn[at(x, y)][3] == 0]
    orange = [srgb_to_linear(v / 255) for v in RC.hex_color(ORANGE)]

    # The rule's own pieces.
    assert strength("linear", 0.5, 2.5) == 0.8 and strength("linear", 2.5, 2.5) == 0
    assert abs(strength("smooth", 0.5, 2.5) - 0.896) < 1e-12
    assert abs(strength("smooth", 1.5, 2.5) - 0.352) < 1e-12
    assert strength("sharp", 1.5, 2.5) == 1 and strength("sharp", 2.5, 2.5) == 0.5
    assert all(strength(s, 0, 0) == 0 for s in ("linear", "smooth", "sharp"))
    assert toward(90) == (1.0, 0.0) and toward(180) == (0.0, 1.0) and toward(540) == (0.0, 1.0)

    # Only what is drawn is lit; nothing but cutout changes a covering.
    for name, frames in c.items():
        if name in ("FX-SWEEP-018", "FX-SWEEP-019"):
            continue
        for f in frames.values():
            assert all(f[at(x, y)] == drawn[at(x, y)] for x, y in empty), name
            if name not in ("FX-SWEEP-013", "FX-SWEEP-014"):
                assert all(f[i][3] == drawn[i][3] for i in range(W * H)), name

    one = c["FX-SWEEP-001"]["0"]
    assert all(gain(one, x, y) > 0 for x, y in shown)
    assert gain(one, 8, 2) > gain(one, 8, 4) and gain(one, 12, 5) > gain(one, 11, 5)  # rims
    assert c["FX-SWEEP-002"]["0"] == drawn and c["FX-SWEEP-003"]["0"] == drawn

    four = c["FX-SWEEP-004"]["0"]
    for x, y in shown:
        want = {7: 0.8, 8: 0.8, 6: 0.4, 9: 0.4}.get(x, 0.0)
        a = drawn[at(x, y)][3]
        assert close(four[at(x, y)], [v + want * a for v in drawn[at(x, y)][:3]] + [a]), (x, y)
    five, six = c["FX-SWEEP-005"]["0"], c["FX-SWEEP-006"]["0"]
    for y in range(2, 8):
        assert abs(gain(five, 7, y) - 0.896) < 1e-12 and abs(gain(five, 9, y) - 0.352) < 1e-12
        assert all(abs(gain(six, x, y) - 1) < 1e-12 for x in range(6, 10))
        assert abs(gain(six, 5, y) - 0.5) < 1e-12 and abs(gain(six, 10, y) - 0.5) < 1e-12
        assert gain(six, 4, y) == 0 and gain(six, 11, y) == 0
    seven = c["FX-SWEEP-007"]["0"]
    for x, y in shown:
        want = 2 / 3 if y in (4, 5) else 0.0
        assert abs(gain(seven, x, y) - want * drawn[at(x, y)][3]) < 1e-12, (x, y)
    assert gain(seven, 14, 5) > 0.6
    eight = c["FX-SWEEP-008"]["0"]
    assert gain(eight, 8, 5) > 0.6 and gain(eight, 11, 2) > 0.6 and gain(eight, 4, 7) > 0.2
    assert gain(eight, 4, 2) == 0 and gain(eight, 12, 7) == 0

    nine = c["FX-SWEEP-009"]["0"]
    ring = [(x, y) for x, y in shown if x in (3, 12) or y in (2, 7)]
    middle = [(x, y) for x in range(4, 12) for y in range(3, 7)]
    assert all(gain(nine, x, y) > 0.99 * drawn[at(x, y)][3] for x, y in ring
               if x != 3 or y in (2, 7))
    assert all(0.49 < gain(nine, 3, y) < 0.5 for y in range(3, 7))  # beside the soft edge
    assert all(gain(nine, 2, y) > 0.49 for y in range(2, 8)) and gain(nine, 14, 5) > 0.99
    assert all(nine[at(x, y)] == drawn[at(x, y)] for x, y in middle)
    ten = c["FX-SWEEP-010"]["0"]
    assert all(gain(ten, x, 3) > 0.99 for x in range(5, 11))
    assert all(ten[at(x, y)] == drawn[at(x, y)] for x in range(5, 11) for y in (4, 5))

    eleven = c["FX-SWEEP-011"]["0"]
    for y in range(2, 8):
        a = drawn[at(8, y)][3]
        assert close(eleven[at(8, y)], [drawn[at(8, y)][i] + 0.8 * a * orange[i]
                                        for i in range(3)] + [a])
    twelve = c["FX-SWEEP-012"]["0"]
    for y in range(2, 8):
        for x, m in ((7, 0.8), (6, 0.4)):
            o = drawn[at(x, y)]
            assert close(twelve[at(x, y)], [o[i] + m * (o[3] * orange[i] - o[i])
                                            for i in range(3)] + [o[3]])
    assert all(v <= max(orange[i], 1) for p in twelve for i, v in enumerate(p[:3]))
    thirteen = c["FX-SWEEP-013"]["0"]
    for x, y in shown:
        m = {7: 0.8, 8: 0.8, 6: 0.4, 9: 0.4}.get(x, 0.0) * drawn[at(x, y)][3]
        assert close(thirteen[at(x, y)], [m] * 4), (x, y)
    fourteen = c["FX-SWEEP-014"]["0"]
    assert close(fourteen[at(7, 2)], [1.0] * 4) and close(fourteen[at(7, 7)], [1.0] * 4)
    assert close(fourteen[at(7, 4)], [0.8] * 4) and fourteen[at(4, 4)] == EMPTY

    fifteen = c["FX-SWEEP-015"]
    assert fifteen["2"] == four and fifteen["0"] == render(band(center=(20, 50)), 0)
    assert fifteen["4"] == render(band(center=(80, 50)), 0)
    assert gain(fifteen["0"], 2, 4) > 0 and gain(fifteen["0"], 6, 4) == 0
    assert gain(fifteen["4"], 14, 5) > 0 and gain(fifteen["4"], 9, 4) == 0
    sixteen = c["FX-SWEEP-016"]
    assert ease(OVERSHOOT, 0.5) > 1 and sixteen["0"] == drawn and sixteen["2"] == four
    assert c["FX-SWEEP-017"]["0"] == seven
    moved = c["FX-SWEEP-018"]["0"]
    assert all(moved[at(x, y)] == four[at(x - 3, y)] for x in range(3, W) for y in range(H))
    tiled = c["FX-SWEEP-019"]["0"]
    lit_px = [(x, y) for y in range(H) for x in range(W)
              if tiled[at(x, y)] != frame(motion_tile(drawn_layer("figure"), 300, 300, "off"),
                                          8)[at(x, y)]]
    assert {x for x, _ in lit_px} == {6, 10}, lit_px
    assert (6, 5) in lit_px and all((10, y) in lit_px for y in range(2, 8))
    print("checked")


if __name__ == "__main__":
    main()
