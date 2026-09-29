"""Bevel Alpha and Bevel Edges, worked a second way.

D-213 adds two effects that make a flat drawing look raised, as a light from one side catches
its edges: `core.bevel_alpha` bevels the edges of what the drawing covers, its own outline, and
`core.bevel_edges` bevels the four sides of the layer's rectangle, as a raised tile or button.
They are After Effects' Bevel Alpha and Bevel Edges (Perspective) in purpose and names, and this
program's own rule. After Effects' CC Glass, a third bevelling effect, is left out. Nothing is
ported. Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

The rule, both. `light_angle` is the light's direction in degrees clockwise from up, and u(θ) =
(sin θ, -cos θ), exact at whole quarter turns, points from a pixel toward the light (Emboss's
and Drop Shadow's convention, D-145). Each pixel gets a slope s from -1 to 1, how much it faces
the light, and with I = `light_intensity` and L = `light_color`'s linear value, a pixel with
covering a > 0 and linear premultiplied colour p becomes:

    s > 0:  p + (L a - p) I s    (toward the light's colour)
    s < 0:  p (1 - I |s|)        (toward black)

its covering unchanged. A pixel with a = 0 is left as it is. The layer does not grow.

Bevel Alpha. H is the covering blurred by document 21's Gaussian at sigma T / 2, T =
`edge_thickness` in pixels, transparent outside the layer; at T / 2 under a sigma the kernel
reach is 0 and H is the covering itself. With the central differences gx = (H(x+1, y) -
H(x-1, y)) / 2 and gy = (H(x, y+1) - H(x, y-1)) / 2, H outside the blurred rectangle 0,

    s = clamp(-1.25 T (gx ux + gy uy), -1, 1)

so an edge facing the light, where the covering rises away from it, is lit, and the one facing
away is darkened; 1.25 T makes a long straight hard edge square to the light reach about 1. A
draft scales T, a distance in pixels.

Bevel Edges. T = `edge_thickness` times the smaller of the layer's width and height as it
reaches the effect. A pixel whose centre lies nearer than T to the rectangle's nearest side, the
first of left, top, right and bottom when two are as near, is on that side's face, whose
outward normal n is (-1, 0), (0, -1), (1, 0) or (0, 1), and s = n . u; every other pixel is on
the flat top, s = 0. A draft changes nothing, T being a share of the layer.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: for Bevel Alpha, Drop Shadow's card (`tools/drop_shadow_reference.py`); for Bevel
Edges, a slab covering the whole layer. The drawings go into `Fixtures/bevel/media`, the
projects into `Fixtures/bevel`, and the expected frames into `Fixtures/bevel/expected_bevel.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bevel_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from edges_reference import gaussian  # noqa: E402
import drop_shadow_reference as D  # noqa: E402
import emboss_reference as E  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bevel"
TOLERANCE = 2e-5  # document 25's default for a filter
ALPHA, EDGES = "core.bevel_alpha", "core.bevel_edges"
RANGES = {
    ALPHA: {"edge_thickness": (0, 200), "light_angle": (-3600, 3600), "light_intensity": (0, 1)},
    EDGES: {"edge_thickness": (0, 0.5), "light_angle": (-3600, 3600), "light_intensity": (0, 1)},
}
NAMES = ("edge_thickness", "light_angle", "light_color", "light_intensity")
CLIFF = 1e-5  # no pixel centre of Bevel Edges may sit closer than this to T


# --- the rule -------------------------------------------------------------------------------

def shade(p, s, light, i):
    if p[3] <= 0 or s == 0:
        return list(p)
    if s > 0:
        return [p[c] + (light[c] * p[3] - p[c]) * i * s for c in range(3)] + [p[3]]
    return [p[c] * (1 - i * -s) for c in range(3)] + [p[3]]


def alpha_slopes(layer, t, u):
    cover = dict(layer, px=[[0.0, 0.0, 0.0, p[3]] for p in layer["px"]])
    b = gaussian(cover, t / 2, "transparent")

    def h(x, y):
        bx, by = x - b["left"], y - b["top"]
        return b["px"][by * b["w"] + bx][3] if 0 <= bx < b["w"] and 0 <= by < b["h"] else 0.0

    out = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            gx = (h(x + 1, y) - h(x - 1, y)) / 2
            gy = (h(x, y + 1) - h(x, y - 1)) / 2
            out.append(min(1, max(-1, -1.25 * t * (gx * u[0] + gy * u[1]))))
    return out


def face(w, h, t, x, y):
    """Which side's face the pixel is on, as its outward normal, or None on the flat top."""
    d = [(x + 0.5, (-1, 0)), (y + 0.5, (0, -1)), (w - x - 0.5, (1, 0)), (h - y - 0.5, (0, 1))]
    near, n = min(d, key=lambda e: e[0])  # min keeps the first of equals
    return n if near < t else None


def edges_slopes(layer, t, u):
    w, h = layer["w"], layer["h"]
    T = t * min(w, h)
    out = []
    for y in range(h):
        for x in range(w):
            n = face(w, h, T, x, y)
            out.append(0.0 if n is None else n[0] * u[0] + n[1] * u[1])
    return out


def bevel(layer, n, c):
    u = D.unit(n["light_angle"])
    slopes = (alpha_slopes if c["effect"] == ALPHA else edges_slopes)(layer, n["edge_thickness"], u)
    light = [srgb_to_linear(v / 255) for v in R.hex_color(c["light_color"].lower())]
    return [shade(p, s, light, n["light_intensity"]) for p, s in zip(layer["px"], slopes)]


# --- the drawings ---------------------------------------------------------------------------

SKIN, BAND, SOFT_SKIN = R.SKIN, E.BAND, E.SOFT_SKIN


def slab(x, y):
    """The whole layer covered: skin, crossed by Emboss's blue band in rows 4 and 5, and column 12
    the skin at half covering."""
    if x == 12:
        return SOFT_SKIN
    return BAND if y in (4, 5) else SKIN


DRAWINGS = {"card": D.DRAWINGS["card"], "slab": [[slab(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

BLUE_HEX = "#2040a0"


def alpha(edge_thickness=2, light_angle=-60, light_color="#ffffff", light_intensity=0.4, shift=0):
    return {"effect": ALPHA, "drawing": "card", "edge_thickness": edge_thickness,
            "light_angle": light_angle, "light_color": light_color,
            "light_intensity": light_intensity, "shift": shift}


def edges(edge_thickness=0.1, light_angle=-60, light_color="#ffffff", light_intensity=0.4,
          shift=0):
    return dict(alpha(edge_thickness, light_angle, light_color, light_intensity, shift),
                effect=EDGES, drawing="slab")


def held(c, k, frame_no):
    lo, hi = RANGES[c["effect"]][k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def numbers(c, frame_no):
    return {k: held(c, k, frame_no) for k in RANGES[c["effect"]]}


def render(c, frame_no):
    return R.frame(bevel(drawn_layer(c["drawing"]), numbers(c, frame_no), c), c["shift"])


def plain(c):
    return R.frame(drawn_layer(c["drawing"])["px"], c["shift"])


CASES = {
    "FX-BEVEL-001": ("Bevel Alpha as it starts: edge thickness 2, the light from the upper left "
                     "at -60, white, intensity 0.4: the card's top and left edges lighter, its "
                     "bottom and right edges darker, its covering the same.", alpha(), [0]),
    "FX-BEVEL-002": ("Bevel Alpha, edge thickness 0: the frame is the drawing.",
                     alpha(edge_thickness=0), [0]),
    "FX-BEVEL-003": ("Bevel Alpha, intensity 0: the frame is the drawing.",
                     alpha(light_intensity=0), [0]),
    "FX-BEVEL-004": ("Bevel Alpha, the light straight above at 0: the top edge lighter, the "
                     "bottom darker.", alpha(light_angle=0), [0]),
    "FX-BEVEL-005": ("Bevel Alpha, the light from the right at 90: the right edge lighter, the "
                     "left darker.", alpha(light_angle=90), [0]),
    "FX-BEVEL-006": ("Bevel Alpha, edge thickness 6: a wider, gentler bevel reaching every pixel "
                     "of the card.", alpha(edge_thickness=6), [0]),
    "FX-BEVEL-007": ("Bevel Alpha, a blue light #2040a0 at intensity 1: the lit edges go toward "
                     "blue, the dark edges toward black.",
                     alpha(light_color=BLUE_HEX, light_intensity=1), [0]),
    "FX-BEVEL-008": ("Bevel Alpha, edge thickness keyed from 0 at frame 0 to 8 at frame 4, "
                     "linear: frame 0 is the drawing, frames 2 and 4 bevel at 4 and 8.",
                     alpha(edge_thickness=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-BEVEL-009": ("Bevel Alpha, intensity eased from 0 at frame 0 to 1 at frame 4 on a curve "
                     "that overshoots: at frame 2 it has gone above 1 and is held there, so "
                     "frames 2 and 4 are the same.",
                     alpha(light_intensity=keyed((0, 0, OVERSHOOT), (4, 1))), [0, 2, 4]),
    "FX-BEVEL-010": ("FX-BEVEL-001 moved three pixels right: the same, moved.",
                     alpha(shift=3), [0]),
    "FX-BEVEL-011": ("Bevel Edges as it starts: edge thickness 0.1 of the slab's 10 rows, one "
                     "pixel: the left column lit by 0.866 of 0.4 and the top row by 0.5 of it, "
                     "the right column darkened by 0.866 of it and the bottom row by 0.5; the "
                     "top-left corner pixel goes with the left side; the inside unchanged.",
                     edges(), [0]),
    "FX-BEVEL-012": ("Bevel Edges, edge thickness 0.3, three pixels: a frame three pixels "
                     "wide, its corners mitred.", edges(edge_thickness=0.3), [0]),
    "FX-BEVEL-013": ("Bevel Edges, edge thickness 0: the frame is the drawing.",
                     edges(edge_thickness=0), [0]),
    "FX-BEVEL-014": ("Bevel Edges, the light from below at 180: the bottom row lit and the top "
                     "row darkened by the whole 0.4, the left and right columns unchanged.",
                     edges(light_angle=180), [0]),
    "FX-BEVEL-015": ("Bevel Edges, edge thickness 0.5, five pixels: every pixel is on a face, "
                     "a four-sided pyramid.", edges(edge_thickness=0.5), [0]),
    "FX-BEVEL-016": ("Bevel Edges, a red light #ff0000 at intensity 1: the lit sides go toward "
                     "red.", edges(light_color="#ff0000", light_intensity=1), [0]),
    "FX-BEVEL-017": ("Bevel Edges, the light angle keyed from 0 at frame 0 to 360 at frame 4, "
                     "linear: at frame 2 it is 180, FX-BEVEL-014, and frame 4 is frame 0 again.",
                     edges(light_angle=keyed((0, 0), (4, 360))), [0, 2, 4]),
    "FX-BEVEL-018": ("FX-BEVEL-012 moved three pixels right: the same, moved.",
                     edges(edge_thickness=0.3, shift=3), [0]),
}

INVALID = {
    "FX-BEVEL-019": ("Bevel Alpha, edge thickness -1, below 0.", alpha(edge_thickness=-1)),
    "FX-BEVEL-020": ("Bevel Alpha, edge thickness 201, above 200.", alpha(edge_thickness=201)),
    "FX-BEVEL-021": ("Bevel Alpha, intensity 1.1, above 1.", alpha(light_intensity=1.1)),
    "FX-BEVEL-022": ("Bevel Alpha, light angle 3601, past ten turns.", alpha(light_angle=3601)),
    "FX-BEVEL-023": ("Bevel Alpha, light colour \"white\", not a colour written #rrggbb.",
                     alpha(light_color="white")),
    "FX-BEVEL-024": ("Bevel Edges, edge thickness 0.6, above 0.5.", edges(edge_thickness=0.6)),
    "FX-BEVEL-025": ("Bevel Edges, intensity -0.1, below 0.", edges(light_intensity=-0.1)),
    "FX-BEVEL-026": ("Bevel Edges, edge thickness keyed to 0.8 at frame 4.",
                     edges(edge_thickness=keyed((0, 0.1), (4, 0.8)))),
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
        "instance_id": "fx-0-0", "type_id": c["effect"], "enabled": True,
        "parameters": {k: (c[k] if k == "light_color" else setting_json(c[k])) for k in NAMES}}]
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
    (OUT / "expected_bevel.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    card, slab_ = plain(alpha()), plain(edges())

    def change(f, d, x, y):
        """How the pixel moved: +1 lighter, -1 darker, 0 the same; its covering never moves."""
        p, q = f[at(x, y)], d[at(x, y)]
        assert p[3] == q[3], (x, y)
        s = sum(p[:3]) - sum(q[:3])
        return 0 if abs(s) < 1e-12 else (1 if s > 0 else -1)

    # Every case keeps the empty pixels empty and every covering as it was; a Bevel Edges pixel
    # centre is never within 1e-5 of the thickness.
    for fx, frames in c.items():
        cc = (CASES.get(fx) or INVALID.get(fx))[1]
        base = plain(cc)
        for f, px in frames.items():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
            if fx in CASES and cc["effect"] == EDGES:
                T = numbers(cc, int(f))["edge_thickness"] * min(W, H)
                for y in range(H):
                    for x in range(W):
                        for d in (x + 0.5, y + 0.5, W - x - 0.5, H - y - 0.5):
                            assert abs(d - T) >= CLIFF, (fx, f, x, y)

    one = c["FX-BEVEL-001"]["0"]
    # the card is columns 5 to 9, rows 3 to 6: its top-left lit, its bottom-right darkened
    assert change(one, card, 5, 3) == 1 and change(one, card, 9, 6) == -1
    assert change(one, card, 7, 3) == 1 and change(one, card, 7, 6) == -1
    assert change(one, card, 5, 5) == 1 and change(one, card, 9, 4) == -1
    assert c["FX-BEVEL-002"]["0"] == card and c["FX-BEVEL-003"]["0"] == card
    four = c["FX-BEVEL-004"]["0"]
    assert change(four, card, 7, 3) == 1 and change(four, card, 7, 6) == -1
    five = c["FX-BEVEL-005"]["0"]
    assert change(five, card, 9, 5) == 1 and change(five, card, 5, 5) == -1
    six = c["FX-BEVEL-006"]["0"]
    assert all(change(six, card, x, y) != 0 for x in range(5, 10) for y in range(3, 7))
    seven = c["FX-BEVEL-007"]["0"]
    p, q = seven[at(5, 3)], card[at(5, 3)]
    assert p[2] - q[2] > p[0] - q[0]  # the lit corner goes more toward blue than red
    eight = c["FX-BEVEL-008"]
    assert eight["0"] == card and eight["2"] != eight["4"]
    nine = c["FX-BEVEL-009"]
    assert nine["0"] == card and nine["2"] == nine["4"]
    moved = c["FX-BEVEL-010"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))

    eleven = c["FX-BEVEL-011"]["0"]
    k = 0.4 * 0.8660254037844386
    for y in range(1, H - 1):
        p, q = eleven[at(0, y)], slab_[at(0, y)]
        assert all(abs(p[m] - (q[m] + (q[3] - q[m]) * k)) < 1e-12 for m in range(3))
        p, q = eleven[at(W - 1, y)], slab_[at(W - 1, y)]
        assert all(abs(p[m] - q[m] * (1 - k)) < 1e-12 for m in range(3))
    for x in range(1, W - 1):
        assert change(eleven, slab_, x, 0) == 1 and change(eleven, slab_, x, H - 1) == -1
        assert all(change(eleven, slab_, x, y) == 0 for y in range(1, H - 1))
    assert eleven[at(0, 0)] == eleven[at(0, 1)]  # the corner is the left side's
    twelve = c["FX-BEVEL-012"]["0"]
    assert change(twelve, slab_, 2, 5) == 1 and change(twelve, slab_, 3, 5) == 0
    assert change(twelve, slab_, 2, 1) == 1 and change(twelve, slab_, W - 2, H - 3) == -1
    assert c["FX-BEVEL-013"]["0"] == slab_
    fourteen = c["FX-BEVEL-014"]["0"]
    assert change(fourteen, slab_, 5, H - 1) == 1 and change(fourteen, slab_, 5, 0) == -1
    assert change(fourteen, slab_, 0, 5) == 0 and change(fourteen, slab_, W - 1, 5) == 0
    fifteen = c["FX-BEVEL-015"]["0"]
    assert all(change(fifteen, slab_, x, y) != 0 for x in range(W) for y in range(H))
    sixteen = c["FX-BEVEL-016"]["0"]
    p, q = sixteen[at(0, 5)], slab_[at(0, 5)]
    assert p[0] > q[0] and p[2] < q[2]
    seventeen = c["FX-BEVEL-017"]
    assert seventeen["2"] == fourteen and seventeen["0"] == seventeen["4"]
    moved = c["FX-BEVEL-018"]["0"]
    assert all(moved[at(x, y)] == twelve[at(x - 3, y)] for x in range(3, W) for y in range(H))
    for fx, (says, cc) in INVALID.items():
        assert c[fx]["0"] == c[fx]["4"] == plain(cc)
    print("checked")


if __name__ == "__main__":
    main()
