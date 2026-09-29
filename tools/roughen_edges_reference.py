"""Roughen Edges, worked a second way.

D-206 adds `core.roughen_edges`, After Effects' Roughen Edges by a rule of our own: the layer's
edges are eaten into by a rough, noise-driven amount, up to the border deep, and under Roughen
Color a band just inside the new edge takes the edge colour. It is modelled on After Effects'
Roughen Edges and not claimed to match it; nothing is ported. Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it.

The rule. The noise is Fractal Noise's field (D-128, `tools/fractal_noise_reference.py`): at an
input pixel whose centre is P, in the drawing's own space, F = F(seed, 0, P / size,
(evolution + speed frame) / 360) with floor(complexity) octaves and the seed floored, in -1..1.
The depth there is e = border clamp(0.5 + F, 0, 1). With a(Q) the covering of document 21's
bilinear sample of the input at Q, transparent outside the input, m = min(a(P), a(P +- e x),
a(P +- e y)), the least covering here and e either way across and down. Under Roughen the output
pixel is the input's times m / a(P); under Roughen Color, with m2 the same least covering at 2e
and m, t = 1 - m2 / m, and the output is (1 - t) times the input's times m / a(P) plus t times
the edge colour's linear value times m. A pixel with a(P) = 0 stays empty. Nothing grows: every
pixel keeps its place and loses covering, and a covering the same all round is kept as it is.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding Wave Warp's stripes, the same size, unmoved
unless the case says. The drawing goes into `Fixtures/roughen_edges/media`, the projects into
`Fixtures/roughen_edges`, and the expected frames into
`Fixtures/roughen_edges/expected_roughen_edges.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/roughen_edges_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from wave_warp_reference import DRAWINGS  # noqa: E402
from fractal_noise_reference import F, point  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "roughen_edges"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"border": (0, 500), "size": (1, 1000), "complexity": (1, 10),
          "evolution": (-100000, 100000), "speed": (-360, 360), "seed": (0, 100000)}
RUST = "#8a3c14"
NAMES = ("edge_type", "edge_color") + tuple(RANGES)


# --- the rule -------------------------------------------------------------------------------

def depth(n, x, y, frame_no):
    """e at the drawing-space pixel (x, y), and the noise value it came from."""
    f = F(int(n["seed"]), 0, *point(x, y, n["size"], n["evolution"], n["speed"], frame_no),
          int(n["complexity"]))
    return n["border"] * min(1.0, max(0.0, 0.5 + f)), f


def least(layer, X, Y, e):
    a = lambda x, y: bilinear(layer, x, y)[3]  # noqa: E731
    return min(a(X, Y), a(X + e, Y), a(X - e, Y), a(X, Y + e), a(X, Y - e))


def roughen_edges(layer, n, edge_type, color, frame_no):
    E = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            if p[3] == 0:
                px.append(list(p))
                continue
            x, y = layer["left"] + i, layer["top"] + j
            e = depth(n, x, y, frame_no)[0]
            m = least(layer, x + 0.5, y + 0.5, e)
            k = m / p[3]
            out = [v * k for v in p[:3]] + [m]
            if edge_type == "roughen_color" and m > 0:
                t = 1 - min(m, least(layer, x + 0.5, y + 0.5, 2 * e)) / m
                out = [(1 - t) * out[c] + t * E[c] * m for c in range(3)] + [m]
            px.append(out)
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(edge_type="roughen", edge_color=RUST, border=2, size=4, complexity=3, evolution=0,
         speed=0, seed=0, shift=0, tile=False):
    return {"drawing": "stripes", "edge_type": edge_type, "edge_color": edge_color,
            "border": border, "size": size, "complexity": complexity, "evolution": evolution,
            "speed": speed, "seed": seed, "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(roughen_edges(layer_of(c), n, c["edge_type"], c["edge_color"], frame_no),
                 c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-ROUGH-001": ("Border 2, scale 4, the rest as they start: Roughen, complexity 3, evolution "
                     "0, speed 0, seed 0: the stripes are eaten into along the drawing's top and "
                     "left edges and round the empty column and row, up to two pixels deep; more "
                     "than two pixels in, nothing changes.",
                     case(), [0]),
    "FX-ROUGH-002": ("Border 0: nothing changes.", case(border=0), [0]),
    "FX-ROUGH-003": ("Border 4: deeper bites than FX-ROUGH-001.", case(border=4), [0]),
    "FX-ROUGH-004": ("Roughen Color with the rust edge colour as it starts: FX-ROUGH-001's "
                     "covering, with a band of rust just inside the new edge.",
                     case(edge_type="roughen_color"), [0]),
    "FX-ROUGH-005": ("Roughen Color with a blue edge colour, #2060ff.",
                     case(edge_type="roughen_color", edge_color="#2060FF"), [0]),
    "FX-ROUGH-006": ("Complexity 6: finer detail over the same broad bites.", case(complexity=6),
                     [0]),
    "FX-ROUGH-007": ("Complexity 3.9, which counts as 3: FX-ROUGH-001.", case(complexity=3.9),
                     [0]),
    "FX-ROUGH-008": ("Seed 7: other bites.", case(seed=7), [0]),
    "FX-ROUGH-009": ("Evolution 180: the noise moved half a step in depth, other bites.",
                     case(evolution=180), [0]),
    "FX-ROUGH-010": ("Speed 90 degrees a frame: frame 0 is FX-ROUGH-001 and frame 2, evolution "
                     "180 by then, is FX-ROUGH-009.", case(speed=90), [0, 2]),
    "FX-ROUGH-011": ("Scale 12: broader bites.", case(size=12), [0]),
    "FX-ROUGH-012": ("Border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the "
                     "drawing, frame 2 FX-ROUGH-001 and frame 4 FX-ROUGH-003.",
                     case(border=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-ROUGH-013": ("Border keyed from 2 at frame 0 to 500 at frame 4, eased past its end: frame "
                     "2 would pass 500, is held at 500, and is border 500.",
                     case(border=keyed((0, 2, OVERSHOOT), (4, 500))), [0, 2]),
    "FX-ROUGH-014": ("FX-ROUGH-001 moved three pixels right: the bites move with the drawing and "
                     "the three columns left bare stay empty.", case(shift=3), [0]),
    "FX-ROUGH-015": ("After a Motion Tile that grows the layer, its tiles mirrored: the noise is "
                     "the drawing's own, and the drawing's top and left edges, which now meet "
                     "their own mirror images, are not eaten into; the empty column and row "
                     "still are.", case(tile=True), [0]),
}

INVALID = {
    "FX-ROUGH-016": ("Border 501, above 500.", case(border=501)),
    "FX-ROUGH-017": ("Scale 0, below 1.", case(size=0)),
    "FX-ROUGH-018": ("Complexity 0, below 1.", case(complexity=0)),
    "FX-ROUGH-019": ("Complexity 11, above 10.", case(complexity=11)),
    "FX-ROUGH-020": ("Speed 361, above 360.", case(speed=361)),
    "FX-ROUGH-021": ("Seed 100001, above 100000.", case(seed=100001)),
    "FX-ROUGH-022": ("An edge type \"spiky\", which is not one.", case(edge_type="spiky")),
    "FX-ROUGH-023": ("An edge colour \"#12345\", not six hex digits.", case(edge_color="#12345")),
    "FX-ROUGH-024": ("Border keyed to 501 at frame 4, above 500.",
                     case(border=keyed((0, 2), (4, 501)))),
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
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.roughen_edges",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in ("edge_type", "edge_color")
                                       else setting_json(c[k])) for k in NAMES}})
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
    (OUT / "expected_roughen_edges.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q: all(abs(a - b) < 1e-9 for a, b in zip(p, q))  # noqa: E731
    art = plain(case())

    # Every covering in 0..1, every colour inside its covering, and no covering ever grows.
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx
            if fx != "FX-ROUGH-014":
                assert all(p[3] <= q[3] + 1e-12 for p, q in zip(px, art)), fx

    # The depth reaches both of its ends somewhere in the cases, so the clamp is exercised.
    n0 = {k: held(case(), k, 0) for k in RANGES}
    fs = [depth(n0, x, y, 0)[1] for y in range(H) for x in range(W)]
    print(f"noise at FX-ROUGH-001's pixels: {min(fs):.3f} to {max(fs):.3f}")
    assert min(fs) < -0.5 and max(fs) > 0.5

    one = c["FX-ROUGH-001"]["0"]
    assert one != art
    # Deep inside, more than two pixels from any edge, nothing changes.
    for y in range(3, 7):
        for x in range(3, 12):
            assert near(one[at(x, y)], art[at(x, y)]), (x, y)
    lost = sum(one[at(x, y)][3] < art[at(x, y)][3] - 1e-9 for x in range(W) for y in range(H))
    print(f"FX-ROUGH-001: {lost} pixels lose covering")
    assert c["FX-ROUGH-002"]["0"] == art
    three = c["FX-ROUGH-003"]["0"]
    assert sum(p[3] for p in three) < sum(p[3] for p in one)
    four = c["FX-ROUGH-004"]["0"]
    assert all(abs(p[3] - q[3]) < 1e-12 for p, q in zip(four, one)) and four != one
    five = c["FX-ROUGH-005"]["0"]
    assert all(abs(p[3] - q[3]) < 1e-12 for p, q in zip(five, one)) and five != four
    assert c["FX-ROUGH-006"]["0"] != one
    assert c["FX-ROUGH-007"]["0"] == one
    assert c["FX-ROUGH-008"]["0"] != one and c["FX-ROUGH-009"]["0"] != one
    ten = c["FX-ROUGH-010"]
    assert ten["0"] == one and ten["2"] == c["FX-ROUGH-009"]["0"]
    assert c["FX-ROUGH-011"]["0"] != one
    twelve = c["FX-ROUGH-012"]
    assert twelve["0"] == art and twelve["2"] == one and twelve["4"] == three
    thirteen = c["FX-ROUGH-013"]
    assert ease(OVERSHOOT, 0.5) > 1 and thirteen["0"] == one
    assert thirteen["2"] == render(case(border=500), 2)
    moved = c["FX-ROUGH-014"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    tiled = c["FX-ROUGH-015"]["0"]
    assert tiled != one and near(tiled[at(0, 0)], art[at(0, 0)])
    assert sum(p[3] for p in tiled) > sum(p[3] for p in one)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
