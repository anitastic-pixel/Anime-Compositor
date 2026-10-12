"""Hair, worked a second way.

D-673 adds Hair (`core.hair`), after CycoreFX's CC Hair (Simulation): hair grown out of the
layer, each strand rooted where the layer is opaque, standing out towards the viewer and bent
down by its weight, coloured by its own colour or the picture under its root, and lit as hair
is lit, brightest across the light. The CycoreFX manual names the controls and says what each is
for (Length; Thickness; Weight, how much the hair is pulled down, below 0 up; Constant Mass;
Density; the Hair Color group, its colour, how much it takes the picture's colour and its
opacity; and the Light and Shading groups). It publishes no formula, so the rule, the units, the
ranges and the settings when added are ours. The strands are 3D curves seen straight on, so a
strand standing towards the viewer looks short. Nothing moves by itself: a frame is the settings
as they are at that frame (document 20).

On a drawing w by h, with the settings held inside their ranges:

1. N = floor((`density` / 100) 4000 w / h) strands, j = 0 .. N - 1; u_k = (U(0, j, 0, 0, k) +
   1) / 2, U Noise's hash (D-119) at seed 0. Strand j's root is (u_0 w, u_1 h), and it grows
   only where u_2 < the layer's alpha at its root (the bilinear read below), so a half-clear
   picture grows half as much hair and a clear one none.
2. It leaves its root in the direction d = (sin t cos p, sin t sin p, cos t), p = 2 pi u_3, t =
   (pi / 3) u_4: towards the viewer, leaning up to 60 degrees. Its length is L_j = L (0.6 + 0.4
   u_5), L = (`length` / 100) h / 10; with L = 0 there is no hair. Its bend b_j = `weight` / 100,
   times L_j / L when `constant_mass` is on (a longer strand is heavier). Along it, at s from 0
   at the root to 1 at the tip, it is at root + L_j (d_x s, d_y s + b_j s^2, d_z s): it hangs
   down by b_j L_j at its tip, up for a negative weight. It is drawn as the 8 straight pieces
   between s = 0, 1/8, .. 1, seen straight on (x and y; z only lights it).
3. At a pixel centre (X, Y), each strand's coverage: on each piece, the nearest point to (X, Y),
   its distance e and its place s along the strand; the half-width there hw = (T / 2) (1 - 0.75
   s), T = `thickness` h / 1080, so a strand narrows to a quarter at its tip; the piece's
   coverage clamp(hw - e + 1/2, 0, 1). The strand's coverage c is the largest of its pieces',
   the first piece of the largest giving s.
4. Its colour: the hair colour (`hair_color`, linear) mixed towards the picture's straight colour
   at its root by `color_inheritance` / 100. Lit as hair is: the strand's direction at s, D = (d_x,
   d_y + 2 b_j s, d_z) / |.|, against the light V_L = (u_x, u_y, 1) / |.|, (u_x, u_y) the
   direction `light_direction` degrees points, and the eye V_E = (0, 0, 1): a = D.V_L, e = D_z,
   kd = sqrt(max(0, 1 - a^2)), the shine ks = (kd sqrt(max(0, 1 - e^2)) - a e)^(1 / `roughness`)
   where the bracket is above 0, else 0; each channel lit = col (ambient + diffuse I kd) +
   specular I (1 + (col - 1) metal) ks, I = `light_intensity` / 100, the shares over 100.
5. The strands are laid over the layer one after another in the order j, each at alpha c
   `opacity` / 100: out = lit alpha + out (1 - alpha), out's alpha = alpha + out's (1 - alpha).

The bilinear read, premultiplied and linear: fx = clamp(X' - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge. The roots read the layer as
it was before any hair.

`length` 0 to 1000, 30 when added; `thickness` 0 to 100, 2; `weight` -500 to 500, 50; `density`
0 to 1000, 100; `color_inheritance` 0 to 100, 50; `opacity` 0 to 100, 100; `light_intensity` 0
to 400, 100; `light_direction` -3600 to 3600, -45; `ambient` 25, `diffuse` 75, `specular` 50,
`metal` 0, each 0 to 100; `roughness` 0.001 to 1, 0.1; `constant_mass` `off` or `on`, off;
`hair_color` #rrggbb, #7a4e2d. Every number keyable, read as it is at the frame. Every length is a
share of the drawing's height, so a draft needs no scaling. The layer does not grow: hair
reaching past the layer's edge is cut there. The manual's Hairfall Map (a second layer steering
where and how the hair grows) is not offered.

**This file never runs the build's code path.** It tries every strand at every pixel inside the
strand's own box (its pieces' ends, widened by T / 2 + 1/2, past which no piece covers), where
the build bins the strands into tiles by a circle round the root, in double precision from the
drawings' 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size: Bubbles' colour
field, or the field with its right half clear. The projects go into `Fixtures/hair`, the expected
frames into `Fixtures/hair/expected_hair.json`. Every case is "not built yet" until B-552 lands
(D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/hair_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from vector_blur_reference import along  # noqa: E402
from noise_reference import u as U  # noqa: E402
import smooth_reference as S  # noqa: E402
import blobbylize_reference as BL  # noqa: E402
import bubbles_reference as BU  # noqa: E402

W, H, FPS = 48, 32, 24
assert (BU.W, BU.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "hair"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.hair"
SETTINGS = [("length", 0, 1000, 30), ("thickness", 0, 100, 2), ("weight", -500, 500, 50),
            ("density", 0, 1000, 100), ("color_inheritance", 0, 100, 50), ("opacity", 0, 100, 100),
            ("light_intensity", 0, 400, 100), ("light_direction", -3600, 3600, -45),
            ("ambient", 0, 100, 25), ("diffuse", 0, 100, 75), ("specular", 0, 100, 50),
            ("roughness", 0.001, 1, 0.1), ("metal", 0, 100, 0)]
WORDS = [("constant_mass", ("off", "on"), "off")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
PIECES = 8


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def strands(src, v, mass, w=W, h=H):
    """Steps 1 and 2: the strands that grow, in order, as (root, d, L_j, b_j, root colour)."""
    n = math.floor(v["density"] / 100 * 4000 * w / h)
    big = v["length"] / 100 * h / 10
    out = []
    if big == 0:
        return out
    for j in range(n):
        uk = [(U(0, j, 0, 0, k) + 1) / 2 for k in range(6)]
        x, y = uk[0] * w, uk[1] * h
        c = BU.bilinear(src, x, y, w, h)
        if not uk[2] < c[3]:
            continue
        p, t = 2 * math.pi * uk[3], math.pi / 3 * uk[4]
        d = (math.sin(t) * math.cos(p), math.sin(t) * math.sin(p), math.cos(t))
        length = big * (0.6 + 0.4 * uk[5])
        b = v["weight"] / 100 * (length / big if mass == "on" else 1)
        out.append(((x, y), d, length, b, [c[k] / c[3] for k in range(3)]))
    return out


def hair(src, v, mass, color, w=W, h=H):
    grown = strands(src, v, mass, w, h)
    T = v["thickness"] * h / 1080
    hc = BL.hex_linear(color)
    inh = v["color_inheritance"] / 100
    op = v["opacity"] / 100
    inten = v["light_intensity"] / 100
    amb, dif, spc, metal = (v[x] / 100 for x in ("ambient", "diffuse", "specular", "metal"))
    ux, uy = along(v["light_direction"])
    ll = math.sqrt(ux * ux + uy * uy + 1)
    lv = (ux / ll, uy / ll, 1 / ll)
    shapes = []
    for (rx, ry), d, length, b, rc in grown:
        pts = [(rx + length * d[0] * s, ry + length * (d[1] * s + b * s * s))
               for s in (i / PIECES for i in range(PIECES + 1))]
        col = [hc[k] * (1 - inh) + rc[k] * inh for k in range(3)]
        m = T / 2 + 0.5
        box = (min(p[0] for p in pts) - m, max(p[0] for p in pts) + m,
               min(p[1] for p in pts) - m, max(p[1] for p in pts) + m)
        shapes.append((pts, d, b, col, box))
    out = []
    for py in range(h):
        for px in range(w):
            X, Y = px + 0.5, py + 0.5
            o = list(src[py * w + px])
            for pts, d, b, col, box in shapes:
                if not (box[0] < X < box[1] and box[2] < Y < box[3]):
                    continue
                best, at = 0.0, 0.0
                for i in range(PIECES):
                    (ax, ay), (bx, by) = pts[i], pts[i + 1]
                    ex, ey = bx - ax, by - ay
                    ee = ex * ex + ey * ey
                    s = 0.0 if ee == 0 else clamp(((X - ax) * ex + (Y - ay) * ey) / ee, 0, 1)
                    e = math.hypot(X - (ax + s * ex), Y - (ay + s * ey))
                    along_s = (i + s) / PIECES
                    cov = clamp(T / 2 * (1 - 0.75 * along_s) - e + 0.5, 0, 1)
                    if cov > best:
                        best, at = cov, along_s
                if best == 0:
                    continue
                dv = (d[0], d[1] + 2 * b * at, d[2])
                dl = math.sqrt(sum(q * q for q in dv))
                dv = [q / dl for q in dv]
                a = sum(dv[k] * lv[k] for k in range(3))
                ez = dv[2]
                kd = math.sqrt(max(0.0, 1 - a * a))
                br = kd * math.sqrt(max(0.0, 1 - ez * ez)) - a * ez
                ks = br ** (1 / v["roughness"]) if br > 0 else 0.0
                lit = [col[k] * (amb + dif * inten * kd) + spc * inten * (1 + (col[k] - 1) * metal) * ks
                       for k in range(3)]
                al = best * op
                o = [lit[k] * al + o[k] * (1 - al) for k in range(3)] + [al + o[3] * (1 - al)]
            out.append(o)
    return out


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    if not all(c[name] in words for name, words, _ in WORDS):
        return False
    hc = c["hair_color"]
    return len(hc) == 7 and hc[0] == "#" and all(ch in "0123456789abcdefABCDEF" for ch in hc[1:])


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c) or f < c["in_frame"]:
        return BU.frame(src, c["shift"])
    out = hair(src, settings(c, f), c["constant_mass"], c["hair_color"])
    return BU.frame(out, c["shift"])


def plain(c):
    return BU.frame(BU.working(c["drawing"]), c["shift"])


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    c["hair_color"] = "#7a4e2d"
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift)
    return c


# A close look: thirty strands of about 8 pixels, a pixel thick.
BIG = dict(density=0.5, length=300, thickness=30)


def big(**kw):
    return case(**{**BIG, **kw})


CASES = {
    "FX-HAIR-001": ("As added: on a picture 32 pixels high the hair is a short fuzz a pixel long "
                    "and a twentieth of a pixel thick, 6000 strands, half the picture's colour "
                    "and half brown.", case(), (3,)),
    "FX-HAIR-002": ("A close look: about thirty strands of 6 to 10 pixels, a pixel thick, "
                    "hanging by half their length.", big(), (3,)),
    "FX-HAIR-003": ("Weight 0: the strands stand straight out, short where they face the "
                    "viewer.", big(weight=0), (3,)),
    "FX-HAIR-004": ("Weight -100: the strands curl upwards.", big(weight=-100), (3,)),
    "FX-HAIR-005": ("Weight 150 with Constant Mass: the longer strands hang further.",
                    big(weight=150, constant_mass="on"), (3,)),
    "FX-HAIR-006": ("Color Inheritance 0: every strand brown.", big(color_inheritance=0), (3,)),
    "FX-HAIR-007": ("Color Inheritance 100: every strand the picture's colour at its root.",
                    big(color_inheritance=100), (3,)),
    "FX-HAIR-008": ("Opacity 40: the picture shows through the strands.", big(opacity=40), (3,)),
    "FX-HAIR-009": ("Light Direction 135, from the bottom right.", big(light_direction=135),
                    (3,)),
    "FX-HAIR-010": ("Specular 100, Roughness 0.5, Metal 100: a broad shine in the strands' own "
                    "colour.", big(specular=100, roughness=0.5, metal=100), (3,)),
    "FX-HAIR-011": ("Unlit: Ambient 100, no diffuse or specular light; each strand one flat "
                    "colour.", big(ambient=100, diffuse=0, specular=0), (3,)),
    "FX-HAIR-012": ("Density 0: no hair, the picture untouched.", big(density=0), (3,)),
    "FX-HAIR-013": ("Length 0: no hair, the picture untouched.", big(length=0), (3,)),
    "FX-HAIR-014": ("The picture with its right half clear: hair roots only on the left half, "
                    "the strands hanging a little across.", big(drawing="half", density=1), (3,)),
    "FX-HAIR-015": ("Weight keyed from 0 at frame 0 to 100 at frame 12: at frame 0 as "
                    "FX-HAIR-003, then hanging lower.", big(weight=keyed((0, 0), (12, 100))),
                    (0, 6, 12)),
    "FX-HAIR-016": ("FX-HAIR-002 moved 3 pixels right: the same, moved.", big(shift=3), (3,)),
    "FX-HAIR-017": ("A blue hair colour, Color Inheritance 0.",
                    big(hair_color="#2050ff", color_inheritance=0), (3,)),
    "FX-HAIR-018": ("Thickness 100: strands three pixels thick at the root.", big(thickness=100),
                    (3,)),
    "FX-HAIR-019": ("A fur: density 3, length 150, so about 180 strands of 3 to 5 pixels.",
                    big(density=3, length=150), (3,)),
    "FX-HAIR-020": ("Roughness 0.01: a thin, tight shine.", big(roughness=0.01), (3,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-HAIR-021": ("Length -1, below 0.", big(length=-1)),
    "FX-HAIR-022": ("Length 1001, above 1000.", big(length=1001)),
    "FX-HAIR-023": ("Thickness 101, above 100.", big(thickness=101)),
    "FX-HAIR-024": ("Weight 501, above 500.", big(weight=501)),
    "FX-HAIR-025": ("Density -1, below 0.", big(density=-1)),
    "FX-HAIR-026": ("Color Inheritance 101, above 100.", big(color_inheritance=101)),
    "FX-HAIR-027": ("Opacity 101, above 100.", big(opacity=101)),
    "FX-HAIR-028": ("Roughness 0, below 0.001.", big(roughness=0)),
    "FX-HAIR-029": ("Constant Mass written \"yes\".", big(constant_mass="yes")),
    "FX-HAIR-030": ("A hair colour written \"brown\".", big(hair_color="brown")),
    "FX-HAIR-031": ("Light Direction 3601, above 3600.", big(light_direction=3601)),
    "FX-HAIR-032": ("Weight keyed to 600 at frame 4, above 500.",
                    big(weight=keyed((0, 0), (4, 600)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    frames = 30
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    params["hair_color"] = c["hair_color"]
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": c["rate"], "denominator": 1},
            "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-" + d,
                "enabled": True, "locked": False, "in_frame": c["in_frame"], "out_frame": frames,
                "source_offset_frames": 0,
                "transform": {
                    "anchor": prop([0, 0]), "position": prop([c["shift"], 0]),
                    "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
                },
                "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": TYPE, "enabled": True,
                             "parameters": params}],
            }],
        }],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in BU.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        assert valid(c), fx
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        src = BU.working(c["drawing"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {len(strands(src, settings(c, int(f)), c['constant_mass']))} strands, "
            f"{sum(max(abs(a - b) for a, b in zip(px[i], before[i])) > 1e-9 for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        assert not valid(c), fx
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_hair.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    field = plain(case())
    src = BU.working("field")
    assert len(strands(src, settings(case(), 0), "off")) == 6000  # the field is opaque
    assert 25 <= len(strands(src, settings(big(), 0), "off")) <= 35
    half = strands(BU.working("half"), settings(big(density=1), 0), "off")
    assert half and all(r[0] < W / 2 + 1 for r, *_ in half)
    for fx in ("FX-HAIR-012", "FX-HAIR-013"):
        assert c[fx]["3"] == field, fx
    two = c["FX-HAIR-002"]["3"]
    assert two != field
    for n in (3, 4, 5, 6, 7, 8, 9, 10, 11, 17, 18, 20):
        assert c["FX-HAIR-%03d" % n]["3"] != two, n
    moved = c["FX-HAIR-016"]["3"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two[y * W:y * W + W - 3]
    k = c["FX-HAIR-015"]
    assert k["0"] == c["FX-HAIR-003"]["3"] and k["0"] != k["6"] != k["12"]
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
