"""Particle Grid's points from a layer's shape, and strings, worked a second way.

D-712 (B-582) closes the buildable part of EFFECTS.md row 368's first gap on
`core.particle_grid` (D-616, `tools/particle_grid_reference.py`, here G): points from a layer's
shape, text and strings, after what Trapcode Form's layer-shaped and string base forms are for.
Red Giant publishes no formula and no settings when added, so every rule below is ours. No text
or code was copied, and no open-source code is used. The 3D model form stays a gap.

Added to G's settings:

- Two Base Form words, after G's two: "box_strings" and "layer".
- A fourth layer setting, `base_layer` (D-189's layer word, "" when added, stretched to the
  layer as G's three maps are). A file written before D-712 has none: it reads as "", and is
  saved without it while it is "".

The rules, with G's names (d0 a point's base place, a and b its spot on the form's front):

1. Box Strings: the points are Box Grid's, n_x along each of the n_y n_z strings. Each point j
   whose i is below n_x - 1 is drawn as a line from its own place to the place of point j + 1
   (the next on its string), each place moved, turned and seen as G's points are; the line
   D-610's Line covering, max(1, 2 R) wide with R the start point's radius in pixels (size p /
   (2 W), not held to 0.7), at the start point's opacity and colour (not drawn where its Color
   Map sample is clear), its depth the start point's. The last point of a string draws nothing,
   so one point a string draws nothing. Particle Type has no say: strings are lines.
2. Layer: the points are Box Grid's, and point j is kept only where the Base Layer's sample at
   its spot (G's bilinear sample at (a W, b H), stretched to the layer) has an opacity of 1/2 or
   more; the rest are left out (the kept keep their j, so their randomness is Box Grid's). With
   no Base Layer named, every point is kept: Layer draws as Box Grid. The sample is taken at the
   base place, before Disperse, Twist, Displace and the turn, so the shape moves with the form.
   Any layer of the composition serves: a drawing, a shape, or a text layer for points in the
   shape of its words.
3. Box Grid and Sphere Layered draw as G draws them, and ignore the Base Layer.

**This file never runs the build's code path.** It reuses G (and its own sources) in its own
process, adding the words, the setting and the drawings here only; G's fixtures are not written
again. The drawing is Broadcast Colors' 16 by 10; the map layers are G's three and three more 16
by 10 drawings, switched off, below the layer `art`. The projects go into
`Fixtures/particle_grid_shape`, the expected frames into
`Fixtures/particle_grid_shape/expected_particle_grid_shape.json`, each kept point's numbers for
some cases into `Fixtures/particle_grid_shape/particles_particle_grid_shape.json`. Every case is
"not built yet" until B-582 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_grid_shape_reference.py
"""

import json
import math
import sys
from pathlib import Path
from types import SimpleNamespace

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import particle_grid_reference as G  # noqa: E402
import particle_space_reference as P  # noqa: E402
import particle_emitter_reference as PE  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import effect_layer_reference as L  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_grid_shape"

# D-712's words, setting and drawings, in this process only.
G.WORDS = [("base_form", ("box_grid", "sphere_layered", "box_strings", "layer"), "box_grid")] \
    + G.WORDS[1:]
G.LAYERS = G.LAYERS + ["base_layer"]


def ring_drawing():
    """An opaque white ring round (8, 5), pixel centres 2.2 to 4.4 pixels out, clear elsewhere."""
    def px(x, y):
        e = math.hypot(x + 0.5 - 8, y + 0.5 - 5)
        return (255, 255, 255, 255) if 2.2 <= e <= 4.4 else (0, 0, 0, 0)
    return [[px(x, y) for x in range(W)] for y in range(H)]


LETTER = ["................",
          ".######.........",
          ".######.........",
          "...##.....#.....",
          "...##....###....",
          "...##...##.##...",
          "...##..#######..",
          "...##..##...##..",
          "...##..##...##..",
          "................"]


def letter_drawing():
    """Block letters "TA" in orange, as a text layer's words would stand, clear elsewhere."""
    return [[(255, 160, 40, 255) if ch == "#" else (0, 0, 0, 0) for ch in row] for row in LETTER]


def soft_drawing():
    """White, its opacity rising left to right from 0 to 255 (x * 17)."""
    return [[(255, 255, 255, 17 * x) for x in range(W)] for _ in range(H)]


G.DRAWINGS = dict(G.DRAWINGS, ring=ring_drawing(), letter=letter_drawing(), soft=soft_drawing())

_base = G.base


def base(c, s, j):
    """Rules 1 and 2: Box Strings and Layer place their points as Box Grid."""
    if c["base_form"] in ("box_strings", "layer"):
        c = dict(c, base_form="box_grid")
    return _base(c, s, j)


G.base = base
_points = G.points


def spot_alpha(c, s, j):
    """Rule 2: the Base Layer's opacity at point j's spot on the form's front."""
    d0 = base(c, s, j)
    a, b = G.on_front(s, d0, "x"), G.on_front(s, d0, "y")
    return L.bilinear(G.map_of(c["base_layer"]), a * W, b * H)[3]


def points(c, n):
    """G's points; Layer keeps only those on the Base Layer's shape."""
    out = _points(c, n)
    if c["base_form"] != "layer" or c["base_layer"] == "":
        return out
    s = G.sample(c, n)
    return [p for p in out if spot_alpha(c, s, p[0]) >= 0.5]


G.points = points
_sprites = G.sprites


def sprites(c, n):
    """Rule 1 for Box Strings, else G's."""
    if c["base_form"] != "box_strings":
        return _sprites(c, n)
    see = P.camera(SimpleNamespace(now=None), PE.view_of(c))
    nx = G.counts(G.sample(c, n))[0]
    pts = points(c, n)
    out = []
    for j, x, size, alpha, col in pts:
        if j % nx == nx - 1 or col is None:
            continue
        v, v2 = see(x), see(pts[j + 1][1])
        if v is None or v2 is None:
            continue
        X, Y, ppu, depth = v
        rad = size * ppu / (2 * W)
        out.append((depth, j, {"kind": "streak", "X": X, "Y": Y, "X2": v2[0], "Y2": v2[1],
                               "w": max(1.0, 2 * rad), "alpha": alpha, "col": col}))
    out.sort(key=lambda e: (-e[0], e[1]))
    return out


G.sprites = sprites


def case(**given):
    c = G.case(**given)
    c.setdefault("base_layer", "")
    return c


def render(c, n):
    return G.render(c, n)


# A dense grid, one point at each pixel centre of the 16 by 10 layer.
DENSE = dict(form_size_x=16, form_size_y=10, particles_in_x=16, particles_in_y=10, size=1.2)
# Strings: 3 strings of 6 points over 15 by 6 pixels (x 1.75 to 14.25, y 2, 5, 8).
STRINGS = dict(base_form="box_strings", form_size_x=15, form_size_y=9, particles_in_x=6,
               particles_in_y=3, size=2)
RIPPLE = dict(displace=2, fractal_scale=6)

CASES = {
    "FX-PGSHP-001": ("Base Form Layer with no Base Layer named: drawn as Box Grid, FX-PGRID-003's "
                     "fine grid.", case(**dict(G.FINE, base_form="layer")), [0]),
    "FX-PGSHP-002": ("Layer, the ring: a dot at each pixel centre on the ring, none inside or "
                     "outside it.", case(**dict(DENSE, base_form="layer", base_layer="ring")),
                     [0]),
    "FX-PGSHP-003": ("Layer, the block letters TA: dots in the letters' shape.",
                     case(**dict(DENSE, base_form="layer", base_layer="letter")), [0]),
    "FX-PGSHP-004": ("Layer, half clear: only the left half's dots, white.",
                     case(**dict(DENSE, base_form="layer", base_layer="half")), [0]),
    "FX-PGSHP-005": ("Layer, opacity rising left to right, on a grid between pixel centres: the "
                     "dots where it is half or more.",
                     case(**dict(G.FINE, form_size_x=13, particles_in_x=4, base_form="layer", base_layer="soft",
                                 size=2)), [0]),
    "FX-PGSHP-006": ("The ring coloured by the ramp: Color Map and Base Layer together.",
                     case(**dict(DENSE, base_form="layer", base_layer="ring", color_map="ramp")),
                     [0]),
    "FX-PGSHP-007": ("The letters turned 40 degrees about Y and dispersed: the shape moves with "
                     "the form.", case(**dict(DENSE, base_form="layer", base_layer="letter",
                                              form_rotation_y=40, disperse=0.6)), [0]),
    "FX-PGSHP-008": ("The ring rippling and drifting, Flow X 12, at frames 0 and 12.",
                     case(**dict(DENSE, **RIPPLE, base_form="layer", base_layer="ring",
                                 flow_x=12)), [0, 12]),
    "FX-PGSHP-009": ("The ring two deep, 10 pixels front to back: the far ring smaller.",
                     case(**dict(DENSE, base_form="layer", base_layer="ring", particles_in_z=2,
                                 form_size_z=10)), [0]),
    "FX-PGSHP-010": ("The letters as glowing spheres, added.",
                     case(**dict(DENSE, base_form="layer", base_layer="letter", size=2,
                                 particle_type="glow_sphere", transfer_mode="add")), [0]),
    "FX-PGSHP-011": ("The ring at Opacity Random 60, Random Seed 3: the same points, some "
                     "fainter.", case(**dict(DENSE, base_form="layer", base_layer="ring",
                                             opacity_random=60, random_seed=3)), [0]),
    "FX-PGSHP-012": ("Sphere Layered ignores the Base Layer: FX-PGRID-021's ball.",
                     case(**dict(G.BALL, base_layer="ring")), [0]),
    "FX-PGSHP-013": ("Box Strings: 3 strings, each a line through its 6 points.",
                     case(**STRINGS), [0]),
    "FX-PGSHP-014": ("Box Strings rippling, Flow X 12, at frames 0 and 12: wavy lines.",
                     case(**dict(STRINGS, **RIPPLE, flow_x=12)), [0, 12]),
    "FX-PGSHP-015": ("Box Strings twisted 120 degrees, 2 deep and 12 pixels front to back.",
                     case(**dict(STRINGS, twist=120, particles_in_z=2, form_size_z=12)), [0]),
    "FX-PGSHP-016": ("One point a string: nothing drawn.",
                     case(**dict(STRINGS, particles_in_x=1)), [0]),
    "FX-PGSHP-017": ("Box Strings coloured by the ramp.",
                     case(**dict(STRINGS, color_map="ramp")), [0]),
    "FX-PGSHP-018": ("Box Strings at Opacity 50, Size 3, added.",
                     case(**dict(STRINGS, size=3, opacity=50, transfer_mode="add")), [0]),
    "FX-PGSHP-019": ("Box Strings turned 30 degrees about Z, through a composition camera 12 "
                     "pixels in front.", case(camera=(8, 5, -12, 12),
                                              **dict(STRINGS, form_rotation_z=30)), [0]),
    "FX-PGSHP-020": ("Box Strings with Color Map half clear: the lines starting in the clear "
                     "half are not drawn.", case(**dict(STRINGS, color_map="half")),
                     [0]),
    "FX-PGSHP-021": ("Box Strings as Stars: Particle Type has no say, FX-PGSHP-013's lines.",
                     case(**dict(STRINGS, particle_type="star")), [0]),
    "FX-PGSHP-022": ("Box Strings at Size 0.2: lines one pixel wide, at full opacity.",
                     case(**dict(STRINGS, size=0.2)), [0]),
    "FX-PGSHP-023": ("Box Strings with Size keyed from 1 at frame 0 to 4 at frame 24, at frame "
                     "12.", case(**dict(STRINGS, size=keyed((0, 1), (24, 4)))), [12]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
G.FRAMES_OF = FRAMES_OF
assert all(G.in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PGSHP-024": ("Base Form \"text\": not a word (a text layer is named as the Base Layer "
                     "instead).", case(base_form="text")),
    "FX-PGSHP-025": ("Base Form \"Box_Strings\": the word is exact.",
                     case(base_form="Box_Strings")),
    "FX-PGSHP-026": ("Base Layer 5: not a layer's name.", case(base_form="layer", base_layer=5)),
    "FX-PGSHP-027": ("Particle Type \"streak\" on Box Strings: still not a Particle Grid kind.",
                     case(**dict(STRINGS, particle_type="streak"))),
}

LISTED = [("FX-PGSHP-002", 0), ("FX-PGSHP-003", 0), ("FX-PGSHP-005", 0), ("FX-PGSHP-007", 0),
          ("FX-PGSHP-008", 12), ("FX-PGSHP-009", 0)]


def listed():
    """Each kept point's j, place (engine units), size in pixels and opacity."""
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        rows = [[j] + x + [size, alpha] for j, x, size, alpha, _ in points(CASES[fx][1], f)]
        out["cases"][fx] = {"frame": f, "particles": rows}
        print(f"{fx} frame {f}: {len(rows)} points listed")
    (OUT / "particles_particle_grid_shape.json").write_text(json.dumps(out, indent=1) + "\n",
                                                            encoding="utf-8")


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side:
    a point count near a whole number, a Base Layer opacity near a half, a depth near the
    camera's cut, or two overlapping sprites at nearly but not quite the same depth."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            s = G.sample(c, f)
            for a in "xyz":
                v = s["particles_in_" + a]
                assert v == math.floor(v) or abs(v - round(v)) > 1e-6, (fx, f, a)
            if c["base_form"] == "layer" and c["base_layer"]:
                nx, ny, nz = G.counts(s)
                for j in range(nx * ny * nz):
                    assert abs(spot_alpha(c, s, j) - 0.5) > 1e-9, (fx, f, j)
            sps = sprites(c, f)
            for i in range(len(sps)):
                assert abs(sps[i][0] - 0.01) > 1e-6, (fx, f)
                for k in range(i + 1, len(sps)):
                    (ax, ay), (bx, by) = PE.reach(sps[i][2]), PE.reach(sps[k][2])
                    near = ax[0] < bx[1] and bx[0] < ax[1] and ay[0] < by[1] and by[0] < ay[1]
                    gap = abs(sps[i][0] - sps[k][0])
                    assert not (near and 0 < gap < 1e-7), (fx, f, i, k, gap)
    print("clear")


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    g = json.loads((G.OUT / "expected_particle_grid.json").read_text(encoding="utf-8"))
    pgrid = {fx: v["frames"] for fx, v in g["cases"].items()}
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    s = lambda n, f=0: c["FX-PGSHP-%03d" % n][str(f)]  # noqa: E731
    # Layer with no layer is Box Grid; Sphere Layered ignores the Base Layer
    assert s(1) == pgrid["FX-PGRID-003"]["0"]
    assert s(12) == pgrid["FX-PGRID-021"]["0"]
    # the ring: dots on it, none in its middle or the corners
    assert s(2)[5 * W + 8][3] == 0 and s(2)[0][3] == 0 and total(s(2)) > 0
    ring = len(points(CASES["FX-PGSHP-002"][1], 0))
    assert 0 < ring < 160
    # the letters keep the letters' pixel count of points
    assert len(points(CASES["FX-PGSHP-003"][1], 0)) == sum(r.count("#") for r in LETTER)
    # half: nothing right of the middle
    assert all(s(4)[y * W + x][3] == 0 for y in range(H) for x in range(9, W))
    # soft: the kept points are the right-hand ones
    kept = points(CASES["FX-PGSHP-005"][1], 0)
    assert kept and all(x[0] > 0 for _, x, *_ in kept)
    assert s(6) != s(2) and s(7) != s(3) and s(8, 0) != s(8, 12) and s(9) != s(2)
    assert s(11) != s(2) and [p[3] > 0 for p in s(11)] == [p[3] > 0 for p in s(2)]
    # strings: lines, not dots; one point a string draws nothing; Particle Type has no say
    assert total(s(13)) > total(render(case(**dict(STRINGS, base_form="box_grid")), 0))
    assert s(14, 0) != s(14, 12) and s(15) != s(13)
    assert s(16) == clear_frame
    assert s(21) == s(13)
    assert all(s(20)[y * W + x][3] == 0 for y in range(H) for x in range(11, W))
    assert s(22) != s(13) and s(23, 12) != s(13)
    print("checked")


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, rows in G.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    clear()
    expected = B.write_cases(OUT, "particle_grid_shape", CASES, INVALID, render, G.plain,
                             G.file_json)
    listed()
    check(expected)


if __name__ == "__main__":
    main()
