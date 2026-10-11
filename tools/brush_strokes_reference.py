"""Brush Strokes, worked a second way.

D-447 adds `core.brush_strokes`, after After Effects' Brush Strokes (Stylize): the picture
repainted as short strokes of paint, each a straight dab of one colour picked up where it
starts and drawn on in the stroke's direction, so the picture looks smeared that way. After
Effects' controls are Stroke Angle, Brush Size, Stroke Length, Stroke Density, Stroke
Randomness, Paint Surface (on the original, on transparent, on white, on black) and Blend With
Original; Adobe says a Stroke Length of 0 paints dots, and its strokes change on every frame.
Adobe publishes no formula; the rule below is this program's own, and nothing is ported. Two
settings are this program's: `random_seed`, which picks a different set of strokes, and
`animate`, "on" (as After Effects: new strokes every frame) or "off" (the same strokes on every
frame, as After Effects' users get with Posterize Time).

The rule, for a pixel whose middle is at q in the drawing's own pixels (its top-left corner
(0, 0), however far an effect above grew the buffer):

1. The stroke direction d = (sin angle, -cos angle) (0 up, 90 right) and n = (-d_y, d_x) across
   it; u = q . d along the strokes and v = q . n across them.
2. With r the brush size, L the stroke length, D the density and R the randomness, m = min(R, 1):
   the strokes sit on cells s_u = max(1, max(L, 2 r) / D) long and s_v = max(1, 2 r / D)
   across. Cell (i, j) holds one stroke. Its six numbers h_0 to h_5 are 0.5 + 0.5 times
   P0-19's hash (`grade::unit`, D-119) of the seed's whole part, i, j, the frame (0 when
   `animate` is off) and the channel 0 to 5.
3. Its start c = ((i + 0.5 + (h_0 - 0.5) m) s_u, (j + 0.5 + (h_1 - 0.5) m) s_v) in (u, v); its
   slope t = (2 h_2 - 1) R tan 10 degrees, so it runs along e = (1, t) / sqrt(1 + t^2); its
   length l = L (1 - 0.5 m h_3); its radius rho = r (1 + 0.25 m (2 h_4 - 1)); and its place in
   the pile h_5, the highest on top.
4. Its colour is the buffer's pixel (premultiplied, linear) under c, that is at
   floor(c_u d + c_v n + the drawing's corner), held inside the buffer.
5. It covers the pixel by clamp(rho + 0.5 - the distance from (u, v) to the segment from c to
   c + l e, 0, 1).
6. Of the strokes covering the pixel (only cells i from floor((u - L - r') / s_u) - 1 to
   floor((u + r') / s_u) and j from floor((v - r' - L s) / s_v) - 1 to floor((v + r' + L s) /
   s_v) can, r' = 1.25 r + 0.5 and s = R tan 10 / sqrt(1 + (R tan 10)^2)), the two highest in
   the pile are laid on the paint surface, the lower first, each as colour times covering over
   what is below times 1 - covering times the colour's covering.
7. The surface: the pixel itself (original), nothing (transparent), white or black, each opaque.
8. The result is mixed back towards the pixel itself by Blend With Original / 100.

The layer never grows: strokes past its edge are cut off.

Settings: `stroke_angle` -100000 to 100000 degrees, 135 when added; `brush_size` 0.5 to 20
pixels, 2; `stroke_length` 0 to 100 pixels, 8; `stroke_density` 0.1 to 4, 1; `stroke_randomness`
0 to 2, 1; `paint_surface` `original` (when added), `transparent`, `white` or `black`;
`blend_with_original` 0 to 100, 0; `random_seed` 0 to 100000, 0, its whole part; `animate` `on`
(when added) or `off`. Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Noise's card, the same size, unmoved unless the case
says. The expected frames are in `Fixtures/brush_strokes/expected_brush_strokes.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/brush_strokes_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import DRAWINGS, u  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "brush_strokes"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"stroke_angle": (-100000, 100000), "brush_size": (0.5, 20), "stroke_length": (0, 100),
          "stroke_density": (0.1, 4), "stroke_randomness": (0, 2),
          "blend_with_original": (0, 100), "random_seed": (0, 100000)}
WORDS = ("paint_surface", "animate")
NAMES = ("stroke_angle", "brush_size", "stroke_length", "stroke_density", "stroke_randomness",
         "paint_surface", "blend_with_original", "random_seed", "animate")
SURFACES = {"original": None, "transparent": [0.0] * 4, "white": [1.0] * 4,
            "black": [0.0, 0.0, 0.0, 1.0]}
TAN10 = math.tan(math.radians(10))


# --- the rule -------------------------------------------------------------------------------

def brush_strokes(layer, n, w, frame_no):
    th = math.radians(n["stroke_angle"])
    d = (math.sin(th), -math.cos(th))
    nn = (-d[1], d[0])
    r, L, D, Rn = n["brush_size"], n["stroke_length"], n["stroke_density"], n["stroke_randomness"]
    m = min(Rn, 1.0)
    su, sv = max(1.0, max(L, 2 * r) / D), max(1.0, 2 * r / D)
    rr = 1.25 * r + 0.5
    slope = Rn * TAN10
    s = slope / math.sqrt(1 + slope * slope)
    seed = math.floor(n["random_seed"])
    f = frame_no if w["animate"] == "on" else 0
    blend = n["blend_with_original"] / 100
    surface = SURFACES[w["paint_surface"]]
    lw, lh = layer["w"], layer["h"]
    src = layer["px"]

    def stroke(i, j, pu, pv):
        h = [0.5 + 0.5 * u(seed, i, j, f, k) for k in range(6)]
        cu = (i + 0.5 + (h[0] - 0.5) * m) * su
        cv = (j + 0.5 + (h[1] - 0.5) * m) * sv
        t = (2 * h[2] - 1) * slope
        el = math.sqrt(1 + t * t)
        eu, ev = 1 / el, t / el
        length = L * (1 - 0.5 * m * h[3])
        rho = r * (1 + 0.25 * m * (2 * h[4] - 1))
        wu, wv = pu - cu, pv - cv
        a = min(length, max(0.0, wu * eu + wv * ev))
        dist = math.hypot(wu - a * eu, wv - a * ev)
        cov = min(1.0, max(0.0, rho + 0.5 - dist))
        if cov <= 0:
            return None
        sx = min(lw - 1, max(0, math.floor(cu * d[0] + cv * nn[0] - layer["left"])))
        sy = min(lh - 1, max(0, math.floor(cu * d[1] + cv * nn[1] - layer["top"])))
        return h[5], src[sy * lw + sx], cov

    out = []
    for y in range(lh):
        for x in range(lw):
            p = src[y * lw + x]
            qx, qy = layer["left"] + x, layer["top"] + y
            # Only the pixels a frame can show are worked (the tiled case's buffer is 300 by 300).
            if not (-16 <= qx < W + 16 and -16 <= qy < H + 16):
                out.append(p)
                continue
            qx, qy = qx + 0.5, qy + 0.5
            pu, pv = qx * d[0] + qy * d[1], qx * nn[0] + qy * nn[1]
            top = []
            for i in range(math.floor((pu - L - rr) / su) - 1, math.floor((pu + rr) / su) + 1):
                for j in range(math.floor((pv - rr - L * s) / sv) - 1,
                               math.floor((pv + rr + L * s) / sv) + 1):
                    st = stroke(i, j, pu, pv)
                    if st:
                        top = sorted(top + [st], key=lambda z: -z[0])[:2]
            o = list(p) if surface is None else list(surface)
            for _, col, cov in reversed(top):
                o = [col[c] * cov + o[c] * (1 - cov * col[3]) for c in range(4)]
            out.append([o[c] + blend * (p[c] - o[c]) for c in range(4)])
    return dict(layer, px=out)


# --- the cases ------------------------------------------------------------------------------

def case(stroke_angle=135, brush_size=2, stroke_length=8, stroke_density=1, stroke_randomness=1,
         paint_surface="original", blend_with_original=0, random_seed=0, animate="on", shift=0,
         tile=False):
    c = dict(locals())
    c["drawing"] = "card"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(brush_strokes(layer_of(c), n, {k: c[k] for k in WORDS}, frame_no), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


HELD = {"animate": "off"}

CASES = {
    "FX-BRUSH-001": ("The settings as they start: angle 135, brush 2, length 8, density 1, "
                     "randomness 1, on the original: the card smeared down and to the right in "
                     "strokes, new ones each frame.", case(), [0, 1]),
    "FX-BRUSH-002": ("Animate off: the same strokes on frames 0 and 3.", case(**HELD), [0, 3]),
    "FX-BRUSH-003": ("Random Seed 7, held: a different set of strokes from FX-BRUSH-002's.",
                     case(random_seed=7, **HELD), [0]),
    "FX-BRUSH-004": ("Stroke Length 0, held: dots, not strokes.", case(stroke_length=0, **HELD),
                     [0]),
    "FX-BRUSH-005": ("Stroke Angle 90, Randomness 0, on transparent, held: level strokes running "
                     "right, each the colour where it starts.",
                     case(stroke_angle=90, stroke_randomness=0, paint_surface="transparent",
                          **HELD), [0]),
    "FX-BRUSH-006": ("Stroke Angle 0, held: strokes running up.", case(stroke_angle=0, **HELD),
                     [0]),
    "FX-BRUSH-007": ("Randomness 0, held: every stroke the same length and width, on a regular "
                     "grid.", case(stroke_randomness=0, **HELD), [0]),
    "FX-BRUSH-008": ("Randomness 2, held: the strokes tilt further from the angle.",
                     case(stroke_randomness=2, **HELD), [0]),
    "FX-BRUSH-009": ("Brush Size 4, held: fatter strokes.", case(brush_size=4, **HELD), [0]),
    "FX-BRUSH-010": ("Density 3, held: more strokes, overlapping.", case(stroke_density=3, **HELD),
                     [0]),
    "FX-BRUSH-011": ("Density 0.4, held: fewer strokes, the card showing between them.",
                     case(stroke_density=0.4, **HELD), [0]),
    "FX-BRUSH-012": ("Paint on transparent, held: between the strokes nothing shows.",
                     case(paint_surface="transparent", stroke_density=0.6, **HELD), [0]),
    "FX-BRUSH-013": ("Paint on white, held: white between the strokes.",
                     case(paint_surface="white", stroke_density=0.6, **HELD), [0]),
    "FX-BRUSH-014": ("Paint on black, held: black between the strokes.",
                     case(paint_surface="black", stroke_density=0.6, **HELD), [0]),
    "FX-BRUSH-015": ("Blend With Original 100: the card as it was.",
                     case(blend_with_original=100), [0]),
    "FX-BRUSH-016": ("Blend With Original 50, held: halfway between FX-BRUSH-002 and the card.",
                     case(blend_with_original=50, **HELD), [0]),
    "FX-BRUSH-017": ("Brush Size keyed from 0.5 at frame 0 to 4 at frame 4, held.",
                     case(brush_size=keyed((0, 0.5), (4, 4)), **HELD), [0, 2, 4]),
    "FX-BRUSH-018": ("FX-BRUSH-002 moved three pixels right: the strokes are the drawing's own, "
                     "so they move with it.", case(shift=3, **HELD), [0]),
    "FX-BRUSH-019": ("After a Motion Tile that grows the layer: the strokes sit where they did, "
                     "and pick their colours from the grown buffer.", case(tile=True, **HELD),
                     [0]),
    "FX-BRUSH-020": ("Angle 200, brush 1.5, length 5.5, density 1.8, randomness 0.6, on black, "
                     "blend 20, seed 31: the controls together.",
                     case(stroke_angle=200, brush_size=1.5, stroke_length=5.5, stroke_density=1.8,
                          stroke_randomness=0.6, paint_surface="black", blend_with_original=20,
                          random_seed=31), [0, 3]),
}

INVALID = {
    "FX-BRUSH-021": ("Brush Size 0.4, below 0.5.", case(brush_size=0.4)),
    "FX-BRUSH-022": ("Stroke Length 101, above 100.", case(stroke_length=101)),
    "FX-BRUSH-023": ("Stroke Density 0.05, below 0.1.", case(stroke_density=0.05)),
    "FX-BRUSH-024": ("Stroke Randomness 2.5, above 2.", case(stroke_randomness=2.5)),
    "FX-BRUSH-025": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-BRUSH-026": ("Random Seed -1, below 0.", case(random_seed=-1)),
    "FX-BRUSH-027": ("Paint Surface \"White\", in capitals, kept as written and not the word.",
                     case(paint_surface="White")),
    "FX-BRUSH-028": ("Animate \"yes\", not on or off.", case(animate="yes")),
    "FX-BRUSH-029": ("Stroke Length keyed to 150 at frame 4, above 100.",
                     case(stroke_length=keyed((0, 8), (4, 150)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.brush_strokes",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
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
    (OUT / "expected_brush_strokes.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    colours = {tuple(p) for p in drawn}
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(f[i], g[i], e) for i in range(W * H))  # noqa: E731
    changed = lambda f, g: sum(not near(f[i], g[i]) for i in range(W * H))  # noqa: E731
    differ = lambda f, g: changed(f, g) > W * H // 5  # noqa: E731
    two = c["FX-BRUSH-002"]["0"]

    assert differ(c["FX-BRUSH-001"]["0"], drawn)
    assert differ(c["FX-BRUSH-001"]["0"], c["FX-BRUSH-001"]["1"])
    assert same(two, c["FX-BRUSH-002"]["3"]) and differ(two, drawn)
    assert differ(c["FX-BRUSH-003"]["0"], two)
    assert differ(c["FX-BRUSH-004"]["0"], two)
    # Level strokes running right with no randomness: every pixel painted fully is the colour of
    # a pixel at or left of it in its own row band, never one to its right.
    five = c["FX-BRUSH-005"]["0"]
    for y in range(H):
        for x in range(W):
            p = five[at(x, y)]
            if p[3] > 0 and tuple(p) in colours:
                assert any(near(p, drawn[at(x2, y2)]) for x2 in range(0, x + 3)
                           for y2 in range(max(0, y - 3), min(H, y + 4))), (x, y)
    assert differ(c["FX-BRUSH-006"]["0"], two)
    assert differ(c["FX-BRUSH-007"]["0"], two) and differ(c["FX-BRUSH-008"]["0"], two)
    assert differ(c["FX-BRUSH-009"]["0"], two)
    assert differ(c["FX-BRUSH-010"]["0"], c["FX-BRUSH-011"]["0"])
    # Between the strokes: the card on the original, nothing, white or black on the others.
    k12, k13, k14 = (c[f"FX-BRUSH-0{n}"]["0"] for n in (12, 13, 14))
    assert sum(p == [0.0] * 4 for p in k12) > 5
    assert sum(near(p, [1.0] * 4) for p in k13) > 5 and all(p[3] > 1 - 1e-9 for p in k13)
    assert sum(near(p, [0, 0, 0, 1]) for p in k14) > 5 and all(p[3] > 1 - 1e-9 for p in k14)
    assert same(c["FX-BRUSH-011"]["0"], drawn) is False
    assert same(c["FX-BRUSH-015"]["0"], drawn)
    half = c["FX-BRUSH-016"]["0"]
    for i in range(W * H):
        assert near(half[i], [(a + b) / 2 for a, b in zip(two[i], drawn[i])], 1e-9)
    k17 = c["FX-BRUSH-017"]
    assert differ(k17["0"], k17["4"]) and differ(k17["2"], k17["4"])
    moved = c["FX-BRUSH-018"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    tiled = c["FX-BRUSH-019"]["0"]
    assert changed(tiled, two) > 0 and differ(tiled, plain(case(tile=True)))
    k20 = c["FX-BRUSH-020"]
    assert differ(k20["0"], k20["3"])
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
