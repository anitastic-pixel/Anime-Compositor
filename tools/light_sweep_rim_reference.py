"""Light sweep's rim, worked a second way.

D-933 adds Light Sweep's `edge_look`, after the edge After Effects' CC Light Sweep draws (the
Liquid Glass tutorial, n0g4HXxx03s 8:46-9:14: Sweep Intensity 0, Edge Intensity 60, Edge
Thickness 3 to 4, Cutout, a soft thin rim all round the shape). The rule is ours; Adobe does not
publish CC Light Sweep's. `band`, D-199's edge, lit only inside the band, is what a file without
the word reads, so every file before D-933 draws as it did; `rim` is the new one, which a new
Light Sweep starts with.

The rim. With a the pixel's covering, k = floor(edge_thickness) and M(x, y) the mean covering
in the square of side 2 k + 1 centred on (x, y), the outside of the input counting as uncovered,
the edge is e = clamp(2 (a - M), 0, a): about a at the shape's border, falling evenly to 0 k
pixels in. The border's outward way is u = -g / |g| with g = (M(x+1, y) - M(x-1, y),
M(x, y+1) - M(x, y-1)), and with d the unit vector `direction` degrees clockwise from up, the
facing is f = (3 + u . d) / 4, from 1/2 on the side facing away to 1 on the side facing d; 3/4
where g is 0. The light at a pixel with a > 0 is L = p sweep_intensity / 100 + edge_intensity /
100 * f e / a, p the band's strength as D-199 has it: the band still carries the sweep, and the
rim is lit all round, whatever the band. The rest, the reception, the untouched pixels and
cutout's clearing, is D-199's.

**This file never runs the build's code path.** It works in double precision on lists.

The cases are `tools/light_sweep_reference.py`'s: the figure, 16 by 10. It writes
`Fixtures/light_sweep/fx_sweep_027.json` on and `expected_light_sweep_rim.json`;
`expected_light_sweep.json` is not touched.

    python tools/light_sweep_rim_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import light_sweep_reference as LS  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed  # noqa: E402
from rim_light_reference import toward  # noqa: E402

W, H, OUT = LS.W, LS.H, LS.OUT


def box_mean(layer, x, y, k):
    w, h, px = layer["w"], layer["h"], layer["px"]
    total = sum(px[v * w + u][3] for v in range(y - k, y + k + 1) for u in range(x - k, x + k + 1)
                if 0 <= u < w and 0 <= v < h)
    return total / (2 * k + 1) ** 2


def rim(layer, i, k, direction):
    """The pixel's edge e and its facing f."""
    x, y = i % layer["w"], i // layer["w"]
    a = layer["px"][i][3]
    e = min(a, max(0.0, 2 * (a - box_mean(layer, x, y, k))))
    gx = box_mean(layer, x + 1, y, k) - box_mean(layer, x - 1, y, k)
    gy = box_mean(layer, x, y + 1, k) - box_mean(layer, x, y - 1, k)
    g = math.hypot(gx, gy)
    d = toward(direction)
    f = 0.75 if g == 0 else (3 - (gx * d[0] + gy * d[1]) / g) / 4
    return e, f


def light_sweep(layer, center, direction, shape, width, sweep, edge_intensity, thickness,
                light_color, reception, dw, dh):
    cx, cy = center[0] / 100 * dw, center[1] / 100 * dh
    n = toward(direction + 90)
    r = width / 2
    k = math.floor(thickness)
    K = [srgb_to_linear(v / 255) for v in LS.RC.hex_color(light_color.lower())]
    out = []
    for i, o in enumerate(layer["px"]):
        a = o[3]
        if a == 0:
            out.append(list(o))
            continue
        px = layer["left"] + i % layer["w"] + 0.5
        py = layer["top"] + i // layer["w"] + 0.5
        p = LS.strength(shape, abs(n[0] * (px - cx) + n[1] * (py - cy)), r)
        L = p * sweep / 100
        if edge_intensity:
            e, f = rim(layer, i, k, direction)
            L += edge_intensity / 100 * f * e / a
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


def case(edge_look="rim", **kw):
    c = LS.case(**kw)
    c["edge_look"] = edge_look
    return c


def render(c, frame_no):
    if c["edge_look"] != "rim":
        return LS.render(c, frame_no)
    layer = LS.drawn_layer(c["drawing"])
    s = {k: LS.held(c, k, frame_no) for k in LS.NUMBERS}
    lit = light_sweep(layer, s["center"], s["direction"], c["shape"], s["width"],
                      s["sweep_intensity"], s["edge_intensity"], s["edge_thickness"],
                      c["light_color"], c["light_reception"], W, H)
    return LS.frame(lit, c["shift"])


EDGES = dict(width=10000, sweep_intensity=0)
TUTORIAL = dict(shape="smooth", width=100, sweep_intensity=0, edge_intensity=60,
                edge_thickness=3, light_reception="cutout")

CASES = {
    "FX-SWEEP-027": ("Edge look rim, the edges alone as FX-SWEEP-009 has them (width 10000, sweep "
                     "intensity 0, edge intensity 100, thickness 1, direction -30): the block's "
                     "border is lit all round, softly, the side facing up and left most.",
                     case(**EDGES), [0]),
    "FX-SWEEP-028": ("As FX-SWEEP-027, width 0: no band, and the rim is lit just the same.",
                     case(width=0, sweep_intensity=0), [0]),
    "FX-SWEEP-029": ("As FX-SWEEP-027, direction 0, thickness 3: the rim three pixels deep, "
                     "fading inward; the top border brighter than the foot.",
                     case(direction=0, edge_thickness=3, **EDGES), [0]),
    "FX-SWEEP-030": ("As FX-SWEEP-029, direction 180: the foot brighter than the top, the "
                     "mirror of FX-SWEEP-029's.",
                     case(direction=180, edge_thickness=3, **EDGES), [0]),
    "FX-SWEEP-031": ("The tutorial's settings: smooth, width 100, sweep intensity 0, edge "
                     "intensity 60, thickness 3, cutout, white, direction -30: only a soft rim "
                     "of white light is left, faint in the middle of a block too small to have one clear.",
                     case(**TUTORIAL), [0]),
    "FX-SWEEP-032": ("Edge look band, written: FX-SWEEP-009 exactly.",
                     case(edge_look="band", **EDGES), [0]),
    "FX-SWEEP-033": ("Edge look rim with edge intensity 0: the band alone, FX-SWEEP-004 "
                     "exactly.",
                     case(direction=0, shape="linear", width=5, sweep_intensity=100,
                          edge_intensity=0), [0]),
    "FX-SWEEP-034": ("As FX-SWEEP-031, the direction keyed from -30 at frame 0 to 330 at frame "
                     "4, a turn: frames 0 and 4 the same, frame 2 lit most at the foot.",
                     case(**{**TUTORIAL, "direction": keyed((0, -30), (4, 330))}), [0, 2, 4]),
    "FX-SWEEP-035": ("As FX-SWEEP-031, the layer moved three pixels right: the rim moves with "
                     "it.", case(shift=3, **TUTORIAL), [0]),
}

INVALID = {
    "FX-SWEEP-036": ("Edge look \"glow\", which is not one.", case(edge_look="glow")),
}


def write(fx, c):
    p = LS.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][-1]["parameters"]["edge_look"] = c["edge_look"]
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": LS.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = LS.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
    (OUT / "expected_light_sweep_rim.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((OUT / "expected_light_sweep.json").read_text())["cases"]
    drawn = LS.plain(LS.case())
    at = lambda x, y: y * W + x  # noqa: E731
    gain = lambda f, x, y: f[at(x, y)][2] - drawn[at(x, y)][2]  # noqa: E731

    e27 = c["FX-SWEEP-027"]["0"]
    # Lit all round the block, its middle untouched, nothing empty touched.
    assert all(gain(e27, x, y) > 0 for x in range(4, 12) for y in (2, 7))
    assert all(gain(e27, 12, y) > 0 for y in range(3, 7))
    assert all(e27[at(x, y)] == drawn[at(x, y)] for x in range(5, 11) for y in range(4, 6))
    assert all(e27[i] == drawn[i] for i in range(W * H) if drawn[i][3] == 0)
    # Facing up-left: the top brighter than the foot.
    assert gain(e27, 8, 2) > gain(e27, 8, 7) > 0
    assert c["FX-SWEEP-028"]["0"] == e27
    e29, e30 = c["FX-SWEEP-029"]["0"], c["FX-SWEEP-030"]["0"]
    assert gain(e29, 8, 2) > gain(e29, 8, 3) > gain(e29, 8, 4) >= 0  # fades inward
    assert gain(e29, 8, 2) > gain(e29, 8, 7) and gain(e30, 8, 7) > gain(e30, 8, 2)
    assert abs(gain(e29, 8, 2) - gain(e30, 8, 7)) < 1e-12  # the mirror
    e31 = c["FX-SWEEP-031"]["0"]
    assert e31[at(8, 2)][3] > 0.4 > 0.1 > e31[at(8, 5)][3] > 0 and e31[at(0, 0)] == [0.0] * 4
    assert c["FX-SWEEP-032"]["0"] == old["FX-SWEEP-009"]["frames"]["0"]
    assert c["FX-SWEEP-033"]["0"] == old["FX-SWEEP-004"]["frames"]["0"]
    k34 = c["FX-SWEEP-034"]
    assert k34["0"] == e31 and k34["4"] == render(case(**{**TUTORIAL, "direction": 330}), 0)
    assert all(abs(a - b) < 1e-12 for p, q in zip(k34["0"], k34["4"]) for a, b in zip(p, q))
    assert k34["2"][at(8, 7)][3] > k34["2"][at(8, 2)][3]
    moved = c["FX-SWEEP-035"]["0"]
    assert all(moved[at(x, y)] == e31[at(x - 3, y)] for x in range(3, W) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
