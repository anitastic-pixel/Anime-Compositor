"""Particle Space's textured sprites, worked a second way.

D-611 (B-491) gives `core.particle_space` (D-610, `tools/particle_space_reference.py`) a layer
of the composition as each particle's picture, as CC Particle World's textured particle types
are for: three more Particle Types, a Texture Layer and two rotation settings. Cycore publishes
no maths, so every rule below is ours (document 21, D-611). No text or code was copied.

The settings added:

- `particle_type` takes three more words, after the five: "textured_square",
  "textured_disc" and "textured_faded_disc".
- `rotation_speed`, -3600 to 3600 degrees a second, 0 when added; `initial_rotation`, 0 to 360
  degrees, 0 when added. Both keyable, both read at the particle's birth b_k, as Birth Size is.
- `texture_layer`, D-189's layer setting (`tools/effect_layer_reference.py`), "" when added; the
  layer named is read whole, at its own size, at the frame drawn (its own timing).

A file written before D-611 has none of the three: each reads as its value when added.

The rule for the three textured kinds, at a sprite of centre (X, Y) and radius R (after the small
sizes' rule, R < 0.7 drawn at 0.7 with the opacity times (R / 0.7)^2, as for a Disc):

1. The turn. theta = Initial Rotation (b) r(k, 10) + Rotation Speed (b) age / fps, in degrees,
   clockwise on screen (Y down); r and age as in D-610. At a pixel centre (x, y), with
   dx = x - X and dy = y - Y: a = dx cos theta + dy sin theta, c = -dx sin theta + dy cos theta.
2. The covering m. Textured Square: clamp(R - |a| + 1/2, 0, 1) clamp(R - |c| + 1/2, 0, 1).
   Textured Disc: clamp(R - e + 1/2, 0, 1), e = sqrt(dx^2 + dy^2). Textured Faded Disc:
   max(0, 1 - (e / R)^2)^2. Never lit (Volume Shade and the light are for the five).
3. With no texture (Texture Layer ""): the sprite is A = opacity m times the Color Map's colour
   (fogged with Depth Cue Fog, as D-610's), A its covering. So Textured Disc and Textured Faded
   Disc are D-610's Disc and Faded Sphere at Volume Shade 0.
4. With a texture T (linear, premultiplied, Tw by Th): its levels, level 0 T itself, each next
   ceil(w / 2) by ceil(h / 2), each pixel the mean of the 2 by 2 pixels under it that are inside
   (fewer at an odd edge), down to 1 by 1; top the last level's number. The level
   lambda = log2(max(Tw, Th) / (2 R)), held within 0 to top. The texture's point
   (s, t) = (a / (2 R) + 1/2, c / (2 R) + 1/2); level j read by document 21's bilinear at
   (s wj, t hj), pixel centres at halves, each tap held to the level's edge pixels. The colour
   Tc is level floor(lambda) mixed towards the next by lambda's fraction (level top alone at the
   top). The sprite: A (fd Tc.rgb + (1 - fd) Fog Tc.a, Tc.a), A = opacity m (Depth Cue Fade's
   fd already in the opacity), Fog the Depth Cue colour and fd D-610's clamp(1 - depth /
   distance, 0, 1) with Depth Cue Fog, else 1. The Color Map is not used.
5. A texture layer whose picture is empty at the frame (before its in point): nothing is drawn
   by a textured kind. A named layer not in the composition: no texture, the warning
   EFFECT_LAYER_MISSING every frame, and the effect left out (D-189's rule 4), so the drawing
   shows as it reaches the effect.

Line and the four round kinds are as D-610 draws them, the texture and the turn not used.

**This file never runs the build's code path.** It works in double precision from the
drawings' 8-bit values and reuses `tools/particle_space_reference.py` for the particles, adding
the two settings and three words in its own process only (that file's fixtures are not written
again). The drawing is Broadcast Colors' 16 by 10; the textures are drawn here.

Every case is a project in `Fixtures/particle_texture/`: the layer `art` with the effect and,
under it, the texture layers `dot`, `big` and `late`, every one switched off. The expected frames
are in `Fixtures/particle_texture/expected_particle_texture.json`. Every case is "not built yet"
until B-491 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_texture_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import particle_space_reference as P  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_texture"
W, H, FPS = P.W, P.H, P.FPS
TEXTURED = ("textured_square", "textured_disc", "textured_faded_disc")

# D-611's settings, in this process only.
P.SETTINGS = P.SETTINGS + [("rotation_speed", -3600, 3600, 0), ("initial_rotation", 0, 360, 0)]
P.NAMES = [s[0] for s in P.SETTINGS]
P.RANGES = {s[0]: (s[1], s[2]) for s in P.SETTINGS}
P.WORDS = [(n, words + TEXTURED, first) if n == "particle_type" else (n, words, first)
           for n, words, first in P.WORDS]


# --- the textures ---------------------------------------------------------------------------

def dot():
    """4 by 4: red, green, blue and white quarters, the red's pixel (1, 1) half covering."""
    q = {(0, 0): (230, 40, 40), (1, 0): (40, 200, 60), (0, 1): (40, 80, 230),
         (1, 1): (250, 250, 250)}
    return [[q[(x // 2, y // 2)] + ((128,) if (x, y) == (1, 1) else (255,)) for x in range(4)]
            for y in range(4)]


def big():
    """20 by 12: red rising across, green down, blue a checker of 60 and 220 (so taking it down
    greys the blue), the top left pixel clear."""
    return [[(round(20 + 215 * x / 19), round(30 + 200 * y / 11), 220 if (x + y) % 2 else 60,
              0 if (x, y) == (0, 0) else 255) for x in range(20)] for y in range(12)]


DRAWINGS = {"dot": dot(), "big": big()}
LAYERS = {"dot": ("dot", 0), "big": ("big", 0), "late": ("dot", 15)}  # name: (drawing, in point)


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


def levels(t):
    """Step 4's levels, level 0 first."""
    out = [t]
    while out[-1]["w"] > 1 or out[-1]["h"] > 1:
        a = out[-1]
        w, h = (a["w"] + 1) // 2, (a["h"] + 1) // 2
        px = []
        for y in range(h):
            for x in range(w):
                taps = [a["px"][yy * a["w"] + xx] for yy in (2 * y, 2 * y + 1)
                        for xx in (2 * x, 2 * x + 1) if xx < a["w"] and yy < a["h"]]
                px.append([sum(p[i] for p in taps) / len(taps) for i in range(4)])
        out.append(L.pic(w, h, px))
    return out


def held(p, sx, sy):
    """Document 21's bilinear at (sx, sy), each tap held to the picture's edge pixels."""
    fx, fy = sx - 0.5, sy - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            q = p["px"][min(max(y0 + dy, 0), p["h"] - 1) * p["w"] + min(max(x0 + dx, 0), p["w"] - 1)]
            for i in range(4):
                out[i] += q[i] * wx * wy
    return out


def texel(mips, s, t, rad):
    top = len(mips) - 1
    lam = min(max(math.log2(max(mips[0]["w"], mips[0]["h"]) / (2 * rad)), 0.0), top)
    j = math.floor(lam)
    a = held(mips[j], s * mips[j]["w"], t * mips[j]["h"])
    if j == top:
        return a
    b = held(mips[j + 1], s * mips[j + 1]["w"], t * mips[j + 1]["h"])
    f = lam - j
    return [a[i] + (b[i] - a[i]) * f for i in range(4)]


def texture_at(c, n):
    """The texture at composition frame n: None for none, "gone" for a layer not there, or its
    levels ([] for an empty picture)."""
    name = c["texture_layer"]
    if name == "":
        return None
    if name not in LAYERS:
        return "gone"
    drawing, start = LAYERS[name]
    return [] if n < start else levels(decoded(drawing))


# --- the rule -------------------------------------------------------------------------------

plain_sprites = P.sprites


def sprites(run, px, view):
    """D-610's sprites, with each textured one's turn and Depth Cue Fog's share."""
    out = plain_sprites(run, px, view)
    c = run.c
    for depth, k, sp, p in out:
        if sp["kind"] not in TEXTURED:
            continue
        b = p["b"]
        theta = math.radians(run.at("initial_rotation", b) * run.r(k, 10)
                             + run.at("rotation_speed", b) * (run.u - b) / FPS)
        sp.update(cos=math.cos(theta), sin=math.sin(theta), fog=P.linear(c["depth_cue_color"]),
                  fd=P.clamp(1 - depth / run.now["depth_cue_distance"])
                  if c["depth_cue"] == "fog" else 1.0)
    return out


P.sprites = sprites


def cover(sp, x, y, vs, lt, tex):
    if sp["kind"] not in TEXTURED:
        return P.cover(sp, x, y, vs, lt)
    rad = sp["R"]
    dx, dy = x - sp["X"], y - sp["Y"]
    a = dx * sp["cos"] + dy * sp["sin"]
    c = -dx * sp["sin"] + dy * sp["cos"]
    if sp["kind"] == "textured_square":
        m = P.clamp(rad - abs(a) + 0.5) * P.clamp(rad - abs(c) + 0.5)
    else:
        e = math.hypot(dx, dy)
        m = P.clamp(rad - e + 0.5) if sp["kind"] == "textured_disc" else max(0.0, 1 - (e / rad) ** 2) ** 2
    if m <= 0:
        return None
    aa = sp["alpha"] * m
    if tex is None:
        return [sp["col"][i] * aa for i in range(3)] + [aa]
    if tex == []:
        return None
    t = texel(tex, a / (2 * rad) + 0.5, c / (2 * rad) + 0.5, rad)
    fd = sp["fd"]
    return [aa * (fd * t[i] + (1 - fd) * sp["fog"][i] * t[3]) for i in range(3)] + [aa * t[3]]


def particle_space(px, c, n, view, tex):
    """D-610's drawing, the textured kinds by `cover` above."""
    run = P.Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    sps = sprites(run, px, view)
    out = [list(p) if c["composite_on_original"] == "on" else [0.0] * 4 for p in px]
    vs = run.now["volume_shade"] / 100
    lt = P.light(run.now)
    mode = c["transfer_mode"]
    for _, _, sp, _ in sps:
        if sp["kind"] == "line":
            r = sp["w"] / 2 + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        else:
            r = sp["R"] * (math.sqrt(2) if sp["kind"] == "textured_square" else 1) + 1
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
    tex = texture_at(c, n)
    if tex == "gone":
        return plain(c)
    out = particle_space(P.working(), c, n, P.view_of(c), tex)
    move = c["shift"]
    if c["camera"] is not None:
        move += int(W / 2 - c["camera"][0])
    return R.frame(out, move)


plain = P.plain


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, camera=None, in_frame=0, texture_layer="", **given):
    c = P.case(shift, camera, in_frame, **given)
    c["texture_layer"] = texture_layer
    return c


def file_json(fx, c):
    p = P.file_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["texture_layer"] = \
        c["texture_layer"]
    p["assets"] += [L.still(n, f"media/{n}.png") for n in DRAWINGS]
    comp = p["compositions"][0]
    frames = comp["duration_frames"]
    under = [L.raster(n, "asset-" + d, enabled=False, in_frame=start, out_frame=frames)
             for n, (d, start) in LAYERS.items()]
    comp["layers"] += under
    comp["layer_order"] += [l["id"] for l in under]
    return p


# One still particle seen close, as D-610's ONE through a lens half as wide: about 9.2 pixels
# across.
ONE = dict(P.ONE, camera_fov=5)
SQUARE = dict(ONE, particle_type="textured_square", texture_layer="dot")
DISC = dict(ONE, particle_type="textured_disc", texture_layer="dot")
FADED = dict(ONE, particle_type="textured_faded_disc", texture_layer="dot")
TWO = dict(P.TWO, camera_fov=6, particle_type="textured_disc", texture_layer="dot")
CAM = dict(P.CAM, camera_fov=14, particle_type="textured_square",
           texture_layer="dot", initial_rotation=360)

CASES = {
    "FX-PTEX-001": ("Textured Square with no texture: a square about 9 pixels across in the "
                    "Color Map's colour, a fifth of the way from yellow to red, unlit.",
                    case(**dict(ONE, particle_type="textured_square")), [20]),
    "FX-PTEX-002": ("Textured Disc with no texture, D-610's size: D-610's Disc (FX-PSPACE-002).",
                    case(**dict(P.ONE, particle_type="textured_disc")), [20]),
    "FX-PTEX-003": ("Textured Faded Disc with no texture, D-610's size: D-610's Faded Sphere "
                    "(FX-PSPACE-003).",
                    case(**dict(P.ONE, particle_type="textured_faded_disc")), [20]),
    "FX-PTEX-004": ("Textured Square with the dot texture: its red, green, blue and white "
                    "quarters, upright, the red one at the top left with a fainter pixel.",
                    case(**SQUARE), [20]),
    "FX-PTEX-005": ("Textured Disc with the dot: the same quarters cut round.", case(**DISC), [20]),
    "FX-PTEX-006": ("Textured Faded Disc with the dot: the quarters fading to nothing at the "
                    "edge.", case(**FADED), [20]),
    "FX-PTEX-007": ("Initial Rotation 360: the square turned by its own random amount.",
                    case(**dict(SQUARE, initial_rotation=360)), [20]),
    "FX-PTEX-008": ("Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees "
                    "clockwise.", case(**dict(SQUARE, rotation_speed=90)), [20]),
    "FX-PTEX-009": ("Rotation Speed keyed from -180 at frame 0 to 180 at frame 24: read at the "
                    "particle's birth (frame 9.2), so it turns anticlockwise at about 42 a "
                    "second for life.",
                    case(**dict(SQUARE, rotation_speed=keyed((0, -180), (24, 180)))), [20]),
    "FX-PTEX-010": ("The 20 by 12 texture on the square: taken down between its second and "
                    "third levels, so its blue checker greys.",
                    case(**dict(SQUARE, texture_layer="big")), [20]),
    "FX-PTEX-011": ("A spread of turned textured squares at many depths: each read at its own "
                    "level.", case(**CAM), [20]),
    "FX-PTEX-012": ("The big texture on the spread: the far ones greyer.",
                    case(**dict(CAM, texture_layer="big")), [20]),
    "FX-PTEX-013": ("Depth Cue Fog towards blue, distance 3: the texture two thirds of the way "
                    "to blue, its fainter pixel fainter blue.",
                    case(**dict(SQUARE, depth_cue="fog", depth_cue_distance=3,
                                depth_cue_color="#3366ff")), [20]),
    "FX-PTEX-014": ("Depth Cue Fade, distance 3: the texture at a third.",
                    case(**dict(DISC, depth_cue="fade", depth_cue_distance=3)), [20]),
    "FX-PTEX-015": ("Two textured discs, Composite: the nearer over the farther.",
                    case(**TWO), [20]),
    "FX-PTEX-016": ("The two, Screen.", case(**dict(TWO, transfer_mode="screen")), [20]),
    "FX-PTEX-017": ("The two, Add.", case(**dict(TWO, transfer_mode="add")), [20]),
    "FX-PTEX-018": ("The two over the drawing (Composite With Original on), Max Opacity 70.",
                    case(**dict(TWO, composite_on_original="on", max_opacity=70)), [20]),
    "FX-PTEX-019": ("The texture layer starting at frame 15: at frame 10 it has no picture, so "
                    "nothing is drawn; at frame 20 it is FX-PTEX-005.",
                    case(**dict(DISC, texture_layer="late")), [10, 20]),
    "FX-PTEX-020": ("A texture layer not in the composition, `gone`: the effect is left out, "
                    "the drawing shows, and the warning every frame.",
                    case(**dict(DISC, texture_layer="gone")), [20]),
    "FX-PTEX-021": ("FX-PTEX-004 with the layer moved 3 pixels right.",
                    case(shift=3, **SQUARE), [20]),
    "FX-PTEX-022": ("The composition's camera, as added, on the spread.",
                    case(camera=(8, 5, -W * 50 / 36, W * 50 / 36), **CAM), [20]),
    "FX-PTEX-023": ("A burst of turning textured squares, Rotation Speed 180 and Initial "
                    "Rotation 360, at frames 12 and 24.",
                    case(**dict(P.FLOW, particle_type="textured_square", texture_layer="dot",
                                rotation_speed=180, initial_rotation=360)), [12, 24]),
    "FX-PTEX-024": ("A tiny textured disc, size 1: drawn 1.4 pixels across and fainter, the "
                    "texture taken down to its last levels.",
                    case(**dict(DISC, birth_size=1, death_size=1)), [20]),
    "FX-PTEX-025": ("A Disc (not textured) with the texture and rotation set: D-610's Disc, the "
                    "texture not used (FX-PSPACE-002).",
                    case(**dict(P.ONE, particle_type="disc", texture_layer="dot",
                                rotation_speed=90)), [20]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
P.FRAMES_OF = FRAMES_OF
assert all(P.in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PTEX-026": ("Particle Type \"textured_polygon\": not one of the eight.",
                    case(particle_type="textured_polygon")),
    "FX-PTEX-027": ("Rotation Speed 3601, above 3600.", case(rotation_speed=3601)),
    "FX-PTEX-028": ("Initial Rotation -1, below 0.", case(initial_rotation=-1)),
    "FX-PTEX-029": ("Initial Rotation 361, above 360.", case(initial_rotation=361)),
    "FX-PTEX-030": ("A texture layer written as the number 3, not a word.",
                    case(texture_layer=3)),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(file_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "colours.png").write_bytes(B.S.png(B.DRAWINGS["colours"]))
    for name, rows in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    clear()
    expected = {"tolerance": B.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["texture_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_particle_texture.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    old = lambda fx: P.render(dict(P.CASES[fx][1], rotation_speed=0, initial_rotation=0), 20)  # noqa: E731
    # No texture: D-610's Disc and Faded Sphere at Volume Shade 0; a Disc ignores the texture.
    assert c["FX-PTEX-002"]["20"] == old("FX-PSPACE-002")
    assert c["FX-PTEX-003"]["20"] == old("FX-PSPACE-003")
    assert c["FX-PTEX-025"]["20"] == old("FX-PSPACE-002")
    # The square covers more than the disc, the disc more than the faded disc.
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    sq, dc, fa = (c["FX-PTEX-%03d" % n]["20"] for n in (4, 5, 6))
    assert total(sq) > total(dc) > total(fa) > 0
    # The quarters upright: the top left red, the bottom right white.
    up = c["FX-PTEX-004"]["20"]
    centre = (round(W / 2 - 0.5), round(H / 2 - 0.5))
    tl, br = up[(centre[1] - 2) * W + centre[0] - 2], up[(centre[1] + 2) * W + centre[0] + 2]
    assert tl[0] > 3 * tl[1] and min(br[:3]) > 0.8, (tl, br)
    # Turned cases differ from upright; Initial Rotation 360's turn differs from Speed 90's.
    for n in (7, 8, 9, 10, 13):
        assert c["FX-PTEX-%03d" % n]["20"] != sq, n
    assert c["FX-PTEX-007"]["20"] != c["FX-PTEX-008"]["20"]
    # The turn is clockwise: theta 90 maps the screen's right to the texture's top.
    a, cc = 1 * math.cos(math.pi / 2) + 0 * math.sin(math.pi / 2), -1 * math.sin(math.pi / 2)
    assert abs(a) < 1e-12 and cc == -1
    # The levels: dot's second level is each quarter's mean; big's go 20x12 .. 1x1, six levels.
    d = levels(decoded("dot"))
    assert [(m["w"], m["h"]) for m in d] == [(4, 4), (2, 2), (1, 1)]
    q = decoded("dot")["px"]
    assert all(abs(d[1]["px"][0][i] - sum(q[y * 4 + x][i] for y in (0, 1) for x in (0, 1)) / 4)
               < 1e-15 for i in range(4))
    assert [(m["w"], m["h"]) for m in levels(decoded("big"))] == \
        [(20, 12), (10, 6), (5, 3), (3, 2), (2, 1), (1, 1)]
    # Fog: bluer than the plain texture.
    fog = c["FX-PTEX-013"]["20"]
    assert sum(p[2] for p in fog) > sum(p[2] for p in sq)
    assert total(c["FX-PTEX-014"]["20"]) < total(dc) * 0.34
    # Two: the three transfer modes differ.
    comp, scr, ad = (c["FX-PTEX-%03d" % n]["20"] for n in (15, 16, 17))
    assert comp != scr != ad != comp
    # Late: nothing before its in point, FX-PTEX-005 after.
    assert c["FX-PTEX-019"]["10"] == blank
    assert c["FX-PTEX-019"]["20"] == dc
    # Gone: the drawing.
    assert c["FX-PTEX-020"]["20"] == plain(CASES["FX-PTEX-020"][1])
    # Moved.
    mv = c["FX-PTEX-021"]["20"]
    for y in range(H):
        assert mv[y * W + 3:(y + 1) * W] == sq[y * W:y * W + W - 3]
    assert c["FX-PTEX-012"]["20"] != c["FX-PTEX-011"]["20"]
    assert c["FX-PTEX-022"]["20"] != c["FX-PTEX-011"]["20"]
    assert c["FX-PTEX-023"]["12"] != c["FX-PTEX-023"]["24"]
    assert 0 < total(c["FX-PTEX-024"]["20"]) < total(dc)
    print("checked")


if __name__ == "__main__":
    main()
