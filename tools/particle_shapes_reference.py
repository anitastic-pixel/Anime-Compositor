"""Particle Space's Tri, Quad and Motion Polygon, worked a second way.

D-1031 (B-910) gives `core.particle_space` (D-610, `tools/particle_space_reference.py`) three
more Particle Types, after CC Particle World's flat shapes the wave 12 tutorials pick: "tri",
"quad" and "motion_polygon". Cycore publishes no maths, so every rule below is ours (document
21, D-1031). No text or code was copied.

At a sprite of centre (X, Y) and radius R (after the small sizes' rule: R < 0.7 drawn at 0.7
with the opacity times (R / 0.7)^2, as for a Disc), its opacity A and its Color Map colour
(fogged with Depth Cue Fog, as D-610's), never lit and never textured:

1. Quad: D-611's Textured Square with no texture (`tools/particle_texture_reference.py`): the
   turn theta = Initial Rotation (b) r(k, 10) + Rotation Speed (b) age / fps, and the covering
   clamp(R - |a| + 1/2, 0, 1) clamp(R - |c| + 1/2, 0, 1). A Texture Layer is never read by it.
2. Tri: an equilateral triangle of side 2R about (X, Y), turned by the same theta, its first
   corner straight up at theta 0. In the turned frame (a, c) its corners are (0, -2R / sqrt 3)
   and (+-R, R / sqrt 3); on screen x = X + a cos theta - c sin theta, y = Y + a sin theta +
   c cos theta.
3. Motion Polygon: a triangle pointing the way the particle moves, its tip at (X, Y) and its
   base at the point a Line's tail is drawn to (D-610: the particle's place size / 0.15 / fps
   frames ago), the base 2R wide across the way it moves. A particle that does not move (tip and
   base at one point) draws nothing; one whose base is not seen (behind the camera) neither.
4. A triangle's covering, as D-612's Star's: with e the pixel centre's distance to the nearest
   of the three edges (as segments), clamp(1/2 + e, 0, 1) inside and clamp(1/2 - e, 0, 1)
   outside. (An edge's distance as a whole line would reach out along a thin triangle's
   point.)

Cube (a lit, turning box) is not here: D-1032, PROPOSED.

**This file never runs the build's code path.** It reuses `tools/particle_texture_reference.py`
(and so D-610's) for the particles, adding the three words in its own process only (neither
file's fixtures are written again).

Every case is a project in `Fixtures/particle_shapes/`. The expected frames are in
`Fixtures/particle_shapes/expected_particle_shapes.json`. Every case is "not built yet" until
B-910 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_shapes_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import particle_texture_reference as T  # noqa: E402
from fxkey_reference import keyed  # noqa: E402

P = T.P
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_shapes"
W, H, FPS = P.W, P.H, P.FPS
SHAPES = ("tri", "quad", "motion_polygon")

P.WORDS = [(n, words + SHAPES, first) if n == "particle_type" else (n, words, first)
           for n, words, first in P.WORDS]

textured_sprites = T.sprites


def sprites(run, px, view):
    """D-611's sprites, with each shape's turn or base."""
    out = textured_sprites(run, px, view)
    see = P.camera(run, view)
    kept = []
    for depth, k, sp, p in out:
        kind = sp["kind"]
        b = p["b"]
        if kind in ("tri", "quad"):
            theta = math.radians(run.at("initial_rotation", b) * run.r(k, 10)
                                 + run.at("rotation_speed", b) * (run.u - b) / FPS)
            sp.update(cos=math.cos(theta), sin=math.sin(theta))
        elif kind == "motion_polygon":
            age, life = run.u - b, run.at("longevity", b) * FPS
            bs, ds = p["sizes"]
            size = (bs + (ds - bs) * age / life) * p["size_factor"]
            tail = see(P.sub(p["x"], P.mul(p["v"], size / 0.15 / FPS)))
            if tail is None:
                continue
            sp.update(X2=tail[0], Y2=tail[1])
        kept.append((depth, k, sp, p))
    return kept


def corners(sp):
    """A Tri's or Motion Polygon's three corners on screen, or None."""
    X, Y, rad = sp["X"], sp["Y"], sp["R"]
    if sp["kind"] == "tri":
        co, si = sp["cos"], sp["sin"]
        q = rad / math.sqrt(3)
        return [(X + a * co - c * si, Y + a * si + c * co)
                for a, c in ((0.0, -2 * q), (rad, q), (-rad, q))]
    dx, dy = sp["X2"] - X, sp["Y2"] - Y
    ll = math.hypot(dx, dy)
    if ll == 0:
        return None
    nx, ny = -dy / ll * rad, dx / ll * rad
    return [(X, Y), (sp["X2"] + nx, sp["Y2"] + ny), (sp["X2"] - nx, sp["Y2"] - ny)]


def triangle(v, x, y):
    """Rule 4."""
    (ax, ay), (bx, by), (cx, cy) = v
    sign = 1.0 if (bx - ax) * (cy - ay) - (by - ay) * (cx - ax) > 0 else -1.0
    e, inside = math.inf, True
    for (px, py), (qx, qy) in ((v[0], v[1]), (v[1], v[2]), (v[2], v[0])):
        ex, ey = qx - px, qy - py
        inside &= sign * (ex * (y - py) - ey * (x - px)) >= 0
        t = P.clamp(((x - px) * ex + (y - py) * ey) / (ex * ex + ey * ey))
        e = min(e, math.hypot(x - px - t * ex, y - py - t * ey))
    return P.clamp(0.5 + e if inside else 0.5 - e)


def cover(sp, x, y, vs, lt, tex):
    kind = sp["kind"]
    if kind not in SHAPES:
        return T.cover(sp, x, y, vs, lt, tex)
    if kind == "quad":
        rad = sp["R"]
        dx, dy = x - sp["X"], y - sp["Y"]
        a = dx * sp["cos"] + dy * sp["sin"]
        c = -dx * sp["sin"] + dy * sp["cos"]
        m = P.clamp(rad - abs(a) + 0.5) * P.clamp(rad - abs(c) + 0.5)
    else:
        v = corners(sp)
        if v is None:
            return None
        m = triangle(v, x, y)
    if m <= 0:
        return None
    aa = sp["alpha"] * m
    return [sp["col"][i] * aa for i in range(3)] + [aa]


def particle_space(px, c, n, view, tex):
    """D-611's drawing, the shapes by `cover` above."""
    run = P.Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    sps = sprites(run, px, view)
    out = [list(p) if c["composite_on_original"] == "on" else [0.0] * 4 for p in px]
    vs = run.now["volume_shade"] / 100
    lt = P.light(run.now)
    mode = c["transfer_mode"]
    for _, _, sp, _ in sps:
        kind = sp["kind"]
        if kind == "line":
            r = sp["w"] / 2 + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        elif kind == "motion_polygon":
            r = sp["R"] + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        else:
            r = sp["R"] * (math.sqrt(2) if kind in ("textured_square", "tri", "quad") else 1) + 1
            xs, ys = (sp["X"] - r, sp["X"] + r), (sp["Y"] - r, sp["Y"] + r)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = cover(sp, gx + 0.5, gy + 0.5, vs, lt, tex)
                if s is None:
                    continue
                d = out[gy * W + gx]
                if mode == "composite":
                    d = [s[i] + (1 - s[3]) * d[i] for i in range(4)]
                elif mode == "screen":
                    d = [s[i] + d[i] - s[i] * d[i] for i in range(4)]
                else:
                    d = [s[0] + d[0], s[1] + d[1], s[2] + d[2], min(1.0, s[3] + d[3])]
                out[gy * W + gx] = d
    return out


def render(c, n):
    tex = T.texture_at(c, n)
    if tex == "gone":
        return P.plain(c)
    out = particle_space(P.working(), c, n, P.view_of(c), tex)
    move = c["shift"]
    if c["camera"] is not None:
        move += int(W / 2 - c["camera"][0])
    return T.R.frame(out, move)


# --- the cases ------------------------------------------------------------------------------

case = T.case
ONE = T.ONE
QUAD = dict(ONE, particle_type="quad")
TRI = dict(ONE, particle_type="tri")
# One particle drifting: born at frame 9.2, thrown out slowly, so its base trails behind it.
DRIFT = dict(ONE, velocity=0.05, particle_type="motion_polygon")
FLOWP = dict(P.FLOW, particle_type="motion_polygon")

CASES = {
    "FX-PSHAPE-001": ("Quad: a square about 9 pixels across, upright, in the Color Map's colour, "
                      "unlit: FX-PTEX-001's Textured Square with no texture.",
                      case(**QUAD), [20]),
    "FX-PSHAPE-002": ("Quad with the dot texture named: the texture is not used, FX-PSHAPE-001.",
                      case(**dict(QUAD, texture_layer="dot")), [20]),
    "FX-PSHAPE-003": ("Tri: a triangle about 9 pixels a side, its point straight up.",
                      case(**TRI), [20]),
    "FX-PSHAPE-004": ("Tri, Initial Rotation 360: turned by its own random amount.",
                      case(**dict(TRI, initial_rotation=360)), [20]),
    "FX-PSHAPE-005": ("Tri, Rotation Speed 90 a second: at 10.8 frames old, turned about 40 "
                      "degrees clockwise.", case(**dict(TRI, rotation_speed=90)), [20]),
    "FX-PSHAPE-006": ("Tri, Rotation Speed keyed -180 at frame 0 to 180 at frame 24: read at its "
                      "birth, so it turns anticlockwise.",
                      case(**dict(TRI, rotation_speed=keyed((0, -180), (24, 180)))), [20]),
    "FX-PSHAPE-007": ("Motion Polygon, one drifting particle: a thin triangle, its point where "
                      "the particle is and its base trailing behind.", case(**DRIFT), [20]),
    "FX-PSHAPE-008": ("Motion Polygon, a particle that does not move: nothing drawn.",
                      case(**dict(ONE, particle_type="motion_polygon")), [20]),
    "FX-PSHAPE-009": ("A burst of Motion Polygons at frames 12 and 24: each a streak pointing "
                      "outwards, as a Line's, but wide at its tail.", case(**FLOWP), [12, 24]),
    "FX-PSHAPE-010": ("A burst of turning Quads, Rotation Speed 180 and Initial Rotation 360, at "
                      "frames 12 and 24.",
                      case(**dict(P.FLOW, particle_type="quad", rotation_speed=180,
                                  initial_rotation=360)), [12, 24]),
    "FX-PSHAPE-011": ("A spread of Tris at many depths, each its own turn.",
                      case(**dict(T.CAM, particle_type="tri", texture_layer="")), [20]),
    "FX-PSHAPE-012": ("Two Tris, Screen.",
                      case(**dict(T.TWO, particle_type="tri", texture_layer="",
                                  transfer_mode="screen")), [20]),
    "FX-PSHAPE-013": ("The Motion Polygon burst with Depth Cue Fog towards blue, distance 3.",
                      case(**dict(FLOWP, depth_cue="fog", depth_cue_distance=3,
                                  depth_cue_color="#3366ff")), [12]),
    "FX-PSHAPE-014": ("FX-PSHAPE-003 with the layer moved 3 pixels right.",
                      case(shift=3, **TRI), [20]),
    "FX-PSHAPE-015": ("A tiny Tri, size 1: drawn as one 0.7 radius, fainter.",
                      case(**dict(TRI, birth_size=1, death_size=1)), [20]),
    "FX-PSHAPE-016": ("Tris over the drawing (Composite With Original on), Max Opacity 70.",
                      case(**dict(TRI, composite_on_original="on", max_opacity=70)), [20]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
P.FRAMES_OF = FRAMES_OF
assert all(P.in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PSHAPE-017": ("Particle Type \"cube\": not built (D-1032, PROPOSED), so not one of the "
                      "eleven.", case(particle_type="cube")),
    "FX-PSHAPE-018": ("Particle Type \"triangle\": the word is \"tri\".",
                      case(particle_type="triangle")),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(T.file_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "colours.png").write_bytes(T.B.S.png(T.B.DRAWINGS["colours"]))
    for name, rows in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(T.png(rows))
    clear()
    expected = {"tolerance": T.B.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = P.plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = P.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_particle_shapes.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def clear():
    """As D-610's: no birth near the frame drawn, no age near its life, no depth near the
    camera's cut-off, no two overlapping sprites at nearly the same depth."""
    px = P.working()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            run = P.Run(c, f)
            for k, b in P.births(run):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
                assert abs((run.u - b) - run.at("longevity", b) * FPS) > 1e-6, (fx, f, k)
            sps = sprites(run, px, P.view_of(c))
            for i in range(len(sps)):
                assert abs(sps[i][0] - 0.01) > 1e-6, fx
                for j in range(i + 1, len(sps)):
                    a, b = sps[i][2], sps[j][2]
                    near = math.hypot(a["X"] - b["X"], a["Y"] - b["Y"]) < (a["R"] + b["R"]) * 1.5 + 1
                    assert not (near and abs(sps[i][0] - sps[j][0]) < 1e-7), (fx, f, i, j)
    print("clear")


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    blank = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    # Quad is D-611's Textured Square with no texture, and a texture named is not read.
    assert c["FX-PSHAPE-001"]["20"] == T.render(T.CASES["FX-PTEX-001"][1], 20)
    assert c["FX-PSHAPE-002"]["20"] == c["FX-PSHAPE-001"]["20"]
    # A triangle of side 2R covers sqrt(3) / 4 of the square of side 2R.
    sq, tri = total(c["FX-PSHAPE-001"]["20"]), total(c["FX-PSHAPE-003"]["20"])
    assert abs(tri / sq - math.sqrt(3) / 4) < 0.02, tri / sq
    # Point up: the triangle is widest at its base, so the bottom half holds more.
    up = c["FX-PSHAPE-003"]["20"]
    cy = H // 2
    assert sum(p[3] for p in up[cy * W:]) > sum(p[3] for p in up[:cy * W])
    for n in (4, 5, 6):
        assert c["FX-PSHAPE-%03d" % n]["20"] != up, n
        assert abs(total(c["FX-PSHAPE-%03d" % n]["20"]) - tri) < 0.05 * tri, n
    assert c["FX-PSHAPE-007"]["20"] != blank
    assert c["FX-PSHAPE-008"]["20"] == blank
    assert c["FX-PSHAPE-009"]["12"] != c["FX-PSHAPE-009"]["24"]
    assert c["FX-PSHAPE-013"]["12"] != c["FX-PSHAPE-009"]["12"]
    mv = c["FX-PSHAPE-014"]["20"]
    for y in range(H):
        assert mv[y * W + 3:(y + 1) * W] == up[y * W:y * W + W - 3]
    assert 0 < total(c["FX-PSHAPE-015"]["20"]) < tri
    print("checked")


if __name__ == "__main__":
    main()
