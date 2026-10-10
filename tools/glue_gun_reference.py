"""Glue Gun, worked a second way.

D-424 adds `core.glue_gun`, our name for CycoreFX's CC Glue Gun: a glossy, blobby stroke laid
along the path the brush has taken, toothpaste squeezed from a tube. The stroke is the brush's
own history, so the picture at a frame depends on where the brush was at the frames before it.
CycoreFX's manual says what each control does in a sentence and publishes no formula; the rule
below is this program's own reading of it, and nothing is ported.

At composition frame f, with fps the composition's frames per second, the holder's input O
(linear, premultiplied) and W by H its drawing:

1. The history. n frames are read: with `time_span` 0 (the stroke kept for ever) n = f - the
   layer's in point, none before it; otherwise n = floor(`time_span` fps + 1/2). P_j, j = 0 to n,
   is `brush_position` as it is at frame f - j, read as the layer's keys are and held to its
   range, then in pixels: x per cent of W, y per cent of H.
2. The blobs. With d = `density` blobs a frame, blob k (each whole number k) is born at frame
   k / d, and lives while f - n <= k / d <= f: k from ceil((f - n) d) to floor(f d). Its age in
   frames is u = (f d - k) / d; with j = floor(u) and t = u - j, it lies at P_j + (P_j+1 - P_j) t,
   where the brush was when it was born, the brush's path joined by straight lines between
   frames. So a brush that moves slowly lays its blobs close together and one that rests piles
   them up, as the manual says the keys' timing changes the density. With density 0 there are no
   blobs.
3. Wobbly (`paint_style` `wobbly`): blob k moves by (`wobble_width` sin(2 pi (s T + a_k)),
   `wobble_height` sin(2 pi (s T + b_k))), T = f / fps seconds, s = `wobble_speed` turns a second,
   a_k and b_k the fractional parts of k times 0.6180339887498949 and 0.7548776662466927, so
   neighbouring blobs swing out of step. Plain does not move them.
4. The field. r = `stroke_width` / 2 is a lone blob's radius and R = r (1 + `strength` / 100) the
   reach of its pull. At a pixel's centre p, F = the sum over the blobs of (1 - |p - c|^2 / R^2)^2
   where |p - c| < R, and G the sum of -4 (1 - |p - c|^2 / R^2) (p - c) / R^2, F's slope. The
   stroke is where F passes T0 = (1 - (r / R)^2)^2, a lone blob's value at r: Strength 0 keeps the
   blobs as round discs that only touch; higher, they swell into one another. s = (F - T0) / |G|
   is about how far inside the edge p lies, in pixels (where |G| is 0: deep inside if F > T0,
   outside otherwise), and the covering a = s + 1/2 held to 0..1, about a pixel of soft edge.
   With stroke width 0 there is no stroke.
5. The surface is a tube of radius r laid round the edge: m = 1 - s / r held to 0..1 and
   N = (-m Gx / |G|, -m Gy / |G|, sqrt(1 - m^2)), facing straight out sideways at the edge and
   straight up from r inside it on (and where |G| is 0). Blobs piled on one spot make one dome,
   no steeper than a lone blob.
6. The colour, the layer seen in the paint: O read at p - (`reflection` / 100) 2 r (Nx, Ny), by
   document 21's bilinear sample (transparent outside), straight (black where it is clear). At
   Reflection 0 it is the layer's own colour under the stroke; higher, each edge shows the
   layer from across the stroke, mirrored.
7. The light and its shading are Blobbylize's (D-379, steps 6 and 7): the effect's own distant
   light from `light_direction` at `light_height` over 100, or a point light at `light_position`
   (per cent of W and H) `light_height` pixels up; each channel = C (ambient + diffuse I Lc
   max(N.L, 0)) + specular I (Lc + (C - Lc) metal) (R.z)^(1 / roughness).
8. The stroke, lit colour times a and covering a, lies over O. The layer does not grow. For a
   draft the stroke width, the wobble's width and height and a point light's height are
   distances.

`brush_position` -1000 to 1000 per cent each way, 50, 50; `stroke_width` 0 to 500 pixels, 20;
`density` 0 to 100 blobs a frame, 5; `time_span` 0 to 100 seconds, 1; `reflection` 0 to 100, 50;
`strength` 0 to 100, 50; `paint_style` `plain` or `wobbly`, `plain`; `wobble_width` and
`wobble_height` 0 to 1000 pixels, 10; `wobble_speed` 0 to 100, 1; and Blobbylize's light and
shading: `light_intensity` 0 to 400, 100; `light_color`, white; `light_type` `distant` or
`point`, `distant`; `light_height` -1000 to 1000, 100; `light_position` per cent, 30, 30;
`light_direction` -3600 to 3600, -45; `ambient` 25, `diffuse` 75, `specular` 50, `metal` 100;
`roughness` 0.001 to 1, 0.05. Every number keyable; only `brush_position` is read at the frames
before, every other setting as it is now. The values when added are chosen here; the manual
gives none. After Effects' lights (the manual's Using: AE Lights) are not offered.

**This file never runs the build's code path.** It sums every blob at every pixel, where the
build skips the blobs out of reach, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, eight frames at 24 a second, in
`Fixtures/glue_gun/`: Blobbylize's drawings, the holder at the top with the effect. The expected
pixels are in `Fixtures/glue_gun/expected_glue_gun.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glue_gun_reference.py
"""

import json
import sys
from math import ceil, floor, pi, sin, sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import along  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import blobbylize_reference as B  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glue_gun"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 16, 10, 8, 24
assert (B.W, B.H) == (W, H)
GOLD, PLASTIC = 0.6180339887498949, 0.7548776662466927
RANGES = {"brush_position": (-1000, 1000), "stroke_width": (0, 500), "density": (0, 100),
          "time_span": (0, 100), "reflection": (0, 100), "strength": (0, 100),
          "wobble_width": (0, 1000), "wobble_height": (0, 1000), "wobble_speed": (0, 100),
          "light_intensity": (0, 400), "light_height": (-1000, 1000),
          "light_position": (-1000, 1000), "light_direction": (-3600, 3600),
          "ambient": (0, 100), "diffuse": (0, 100), "specular": (0, 100),
          "roughness": (0.001, 1), "metal": (0, 100)}
NAMES = ("brush_position", "stroke_width", "density", "time_span", "reflection", "strength",
         "paint_style", "wobble_width", "wobble_height", "wobble_speed", "light_intensity",
         "light_color", "light_type", "light_height", "light_position", "light_direction",
         "ambient", "diffuse", "specular", "roughness", "metal")


# --- the rule -------------------------------------------------------------------------------

def held(c, k, frame):
    v = value_at(c[k], frame)
    if k not in RANGES:
        return v
    lo, hi = RANGES[k]
    return [min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple)) else min(hi, max(lo, v))


def frac(x):
    return x - floor(x)


def blobs(c, f, in_frame=0):
    """Steps 1 to 3: the blobs' centres at frame f, oldest first."""
    d, width = held(c, "density", f), held(c, "stroke_width", f)
    if d == 0 or width == 0:
        return []
    span = held(c, "time_span", f)
    n = max(f - in_frame, 0) if span == 0 else floor(span * FPS + 0.5)
    p = []
    for j in range(n + 1):
        x, y = held(c, "brush_position", f - j)
        p.append((x / 100 * W, y / 100 * H))
    out = []
    wobbly = c["paint_style"] == "wobbly"
    ww, wh, s = (held(c, k, f) for k in ("wobble_width", "wobble_height", "wobble_speed"))
    t_now = f / FPS
    for k in range(ceil((f - n) * d), floor(f * d) + 1):
        u = (f * d - k) / d
        j = floor(u)
        t = u - j
        if j >= n:
            x, y = p[n]
        else:
            x = p[j][0] + (p[j + 1][0] - p[j][0]) * t
            y = p[j][1] + (p[j + 1][1] - p[j][1]) * t
        if wobbly:
            x += ww * sin(2 * pi * (s * t_now + frac(k * GOLD)))
            y += wh * sin(2 * pi * (s * t_now + frac(k * PLASTIC)))
        out.append((x, y))
    return out


def glue_gun(o, centres, s):
    w, h = o["w"], o["h"]
    r = s["stroke_width"] / 2
    big = r * (1 + s["strength"] / 100)
    t0 = (1 - (r / big) ** 2) ** 2 if big > 0 else 0.0
    look = s["reflection"] / 100 * 2 * r
    lc = B.hex_linear(s["light_color"])
    inten = s["light_intensity"] / 100
    amb, dif, spc, metal = (s[n] / 100 for n in ("ambient", "diffuse", "specular", "metal"))
    out = []
    for y in range(h):
        for x in range(w):
            px, py = x + 0.5, y + 0.5
            f, gx, gy = 0.0, 0.0, 0.0
            for cx, cy in centres:
                dx, dy = px - cx, py - cy
                q = (dx * dx + dy * dy) / (big * big)
                if q < 1:
                    f += (1 - q) * (1 - q)
                    gx += -4 * (1 - q) * dx / (big * big)
                    gy += -4 * (1 - q) * dy / (big * big)
            g = sqrt(gx * gx + gy * gy)
            under = o["px"][y * w + x]
            if g > 0:
                s_in = (f - t0) / g
                a = min(max(s_in + 0.5, 0.0), 1.0)
                m = min(max(1 - s_in / r, 0.0), 1.0)
                nn = [-m * gx / g, -m * gy / g, sqrt(1 - m * m)]
            else:
                a = 1.0 if f > t0 else 0.0
                nn = [0.0, 0.0, 1.0]
            if a == 0:
                out.append(list(under))
                continue
            seen = L.bilinear(o, px - look * nn[0], py - look * nn[1])
            col = [seen[i] / seen[3] for i in range(3)] if seen[3] > 0 else [0.0] * 3
            if s["light_type"] == "point":
                lx, ly = s["light_position"][0] / 100 * w, s["light_position"][1] / 100 * h
                lv = [lx - px, ly - py, s["light_height"]]
            else:
                ux, uy = along(s["light_direction"])
                lv = [100 * ux, 100 * uy, s["light_height"]]
            ll = sqrt(sum(v * v for v in lv))
            lv = [v / ll for v in lv] if ll > 0 else [0.0] * 3
            nl = sum(nn[i] * lv[i] for i in range(3))
            rz = 2 * nl * nn[2] - lv[2]
            hi = rz ** (1 / s["roughness"]) if nl > 0 and rz > 0 else 0.0
            lit = [col[i] * (amb + dif * inten * lc[i] * max(nl, 0.0))
                   + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * hi for i in range(3)]
            out.append([lit[i] * a + (1 - a) * under[i] for i in range(3)]
                       + [a + (1 - a) * under[3]])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

# "Unlit" below: Ambient 50 and no diffuse or specular light, so the stroke is the colour it
# shows at half brightness, its shape plain against the photo.
FLAT = {"ambient": 50, "diffuse": 0, "specular": 0}


def case(holder="photo", shift=(0, 0), **kw):
    c = {"brush_position": (50, 50), "stroke_width": 20, "density": 5, "time_span": 1,
         "reflection": 50, "strength": 50, "paint_style": "plain", "wobble_width": 10,
         "wobble_height": 10, "wobble_speed": 1, "light_intensity": 100,
         "light_color": "#ffffff", "light_type": "distant", "light_height": 100,
         "light_position": (30, 30), "light_direction": -45, "ambient": 25, "diffuse": 75,
         "specular": 50, "roughness": 0.05, "metal": 100}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def settings(c, frame):
    return {k: held(c, k, frame) for k in NAMES}


def render(c, frame):
    out = glue_gun(B.decoded(c["holder"]), blobs(c, frame), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = B.decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


# A stroke left to right along the middle, 20 to 80 per cent over frames 0 to 4 (x 3.2 to 12.8,
# y 5), 4 pixels wide, two blobs a frame, kept for ever.
SWEEP = dict(brush_position=keyed((0, (20, 50)), (4, (80, 50))), stroke_width=4, density=2,
             time_span=0, reflection=0)
CASES = {
    "FX-GLUE-001": ("As added, the brush never keyed at the middle: Stroke Width 20, Density 5, "
                    "Time Span 1, Reflection 50, Strength 50, lit as Blobbylize is: one round "
                    "glossy blob over the whole drawing, the same at frame 4 (more blobs on the "
                    "same spot pile up but reach no further), on the grey ramp.", case(holder="ramp"),
                    (0, 4)),
    "FX-GLUE-002": ("The sweep, unlit, Reflection 0: at frame 0 a single blob at the left, at "
                    "frame 2 a stroke half way across, at frame 4 the whole stroke, 4 pixels "
                    "wide, the photo beneath it at half brightness.", case(**SWEEP, **FLAT),
                    (0, 2, 4)),
    "FX-GLUE-003": ("The sweep lit as added: the stroke stands up from the photo, lighter on "
                    "its upper edge facing the top-left light, on the grey ramp.",
                    case(holder="ramp", **SWEEP), (2, 4)),
    "FX-GLUE-004": ("The sweep with Time Span 0.1 seconds, 2 frames at 24 a second: at frame 4 "
                    "only the last two frames of the stroke are left, the tail gone.",
                    case(**dict(SWEEP, time_span=0.1)), (4, 6)),
    "FX-GLUE-005": ("The sweep with Density 0.5, one blob every second frame, and Strength 0: "
                    "separate round discs 2 pixels across, not touching.",
                    case(**dict(SWEEP, density=0.5, strength=0), **FLAT), (4,)),
    "FX-GLUE-006": ("FX-GLUE-005 with Strength 100: the same blobs swell into one another and "
                    "join.", case(**dict(SWEEP, density=0.5, strength=100), **FLAT), (4,)),
    "FX-GLUE-007": ("The sweep, Reflection 100, Stroke Width 6, unlit: the stroke's edges show "
                    "the photo from across the stroke, mirrored, its middle the photo beneath.",
                    case(**dict(SWEEP, reflection=100, stroke_width=6), **FLAT), (4,)),
    "FX-GLUE-008": ("The sweep, Wobbly, Wobble Width and Height 1, Speed 2: the blobs swing a "
                    "pixel out of place, out of step with one another, and on at frame 6 though "
                    "the brush has stopped.",
                    case(**dict(SWEEP, paint_style="wobbly", wobble_width=1, wobble_height=1,
                                wobble_speed=2)), (4, 6)),
    "FX-GLUE-009": ("The sweep lit by a point light at 50, 20 per cent, 6 pixels up: brightest "
                    "near the stroke's middle top.",
                    case(**dict(SWEEP, light_type="point", light_position=(50, 20),
                                light_height=6)), (4,)),
    "FX-GLUE-010": ("The sweep, an orange light, Specular 100, Roughness 0.5, Metal 0, Diffuse "
                    "0, Stroke Width 6: a broad highlight in the light's own orange.",
                    case(**dict(SWEEP, light_color="#ff8000", specular=100, roughness=0.5,
                                metal=0, diffuse=0, light_height=40, stroke_width=6)), (4,)),
    "FX-GLUE-011": ("FX-GLUE-003 on the holder moved 2 right and 1 down: the same, moved.",
                    case(holder="ramp", shift=(2, 1), **SWEEP), (4,)),
    "FX-GLUE-012": ("Stroke Width 0: no stroke, the photo untouched.",
                    case(**dict(SWEEP, stroke_width=0)), (4,)),
    "FX-GLUE-013": ("Density 0: no blobs, the photo untouched.", case(**dict(SWEEP, density=0)),
                    (4,)),
    "FX-GLUE-014": ("The sweep over the shapes drawing, clear between its blocks, Stroke Width "
                    "6: over the blocks the stroke takes their colour; over the clear gap there "
                    "is nothing to reflect, so the paint is black, lit only by its highlight.",
                    case(holder="shapes", **dict(SWEEP, stroke_width=6)), (4,)),
    "FX-GLUE-015": ("The brush held at 20 per cent until frame 2, then at 80 from frame 3: at "
                    "frame 4 a pile of blobs at the left, then a line of them across the jump, "
                    "laid between frames 2 and 3, unlit.",
                    case(**dict(SWEEP, brush_position=keyed((0, (20, 50), "hold"), (3, (80, 50)))),
                         **FLAT), (2, 4)),
    "FX-GLUE-016": ("The sweep with the light's Direction 135, from the bottom right: the lower "
                    "edge lights instead.", case(holder="ramp", **dict(SWEEP, light_direction=135)),
                    (4,)),
    "FX-GLUE-017": ("The sweep with Stroke Width keyed from 2 at frame 0 to 6 at frame 4: every "
                    "blob takes the width as it is now, so the whole stroke thickens.",
                    case(**dict(SWEEP, stroke_width=keyed((0, 2), (4, 6))), **FLAT), (0, 4)),
    "FX-GLUE-018": ("The brush eased from 20 to 80 per cent across on a curve that overshoots, "
                    "Stroke Width 4, unlit: the history is read as the keys are, past the last "
                    "key's place and back.",
                    case(**dict(SWEEP, brush_position=keyed((0, (20, 50), OVERSHOOT),
                                                            (4, (80, 50)))), **FLAT), (2, 4)),
}

INVALID = {
    "FX-GLUE-019": ("Stroke Width 501, above 500.", case(stroke_width=501)),
    "FX-GLUE-020": ("Density -1, below 0.", case(density=-1)),
    "FX-GLUE-021": ("Time Span 101, above 100 seconds.", case(time_span=101)),
    "FX-GLUE-022": ("A paint style written \"drippy\".", case(paint_style="drippy")),
    "FX-GLUE-023": ("A light type written \"spot\".", case(light_type="spot")),
    "FX-GLUE-024": ("Roughness 0, below 0.001.", case(roughness=0)),
    "FX-GLUE-025": ("A brush position 50, 1001, past ten heights.",
                    case(brush_position=(50, 1001))),
    "FX-GLUE-026": ("A light colour written \"orange\".", case(light_color="orange")),
    "FX-GLUE-027": ("Wobble Speed keyed to 150 at frame 4.",
                    case(wobble_speed=keyed((0, 1), (4, 150)))),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.glue_gun", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-" + c["holder"], position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect(c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in ("photo", "shapes", "ramp")],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [holder])]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("photo", "shapes", "ramp"):
        (OUT / "media" / f"{name}.png").write_bytes(png(B.DRAWINGS[name]))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
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
    (OUT / "expected_glue_gun.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    photo, ramp = plain(case()), plain(case(holder="ramp"))
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)
        if fx in INVALID:
            assert all(px == plain(INVALID[fx][1]) for px in frames.values())

    # The blobs: the sweep's history and births.
    sw = case(**SWEEP)
    assert blobs(sw, 0) == [(3.2, 5.0)]
    four = blobs(sw, 4)
    assert len(four) == 9 and near(four[-1], (12.8, 5.0)) and near(four[0], (3.2, 5.0))
    assert near(four[1], (4.4, 5.0))  # half a frame after frame 0, half way to 5.6
    assert len(blobs(case(**dict(SWEEP, time_span=0.1)), 4)) == 5  # frames 2 to 4
    assert len(blobs(case(**dict(SWEEP, density=0.5)), 4)) == 3  # frames 0, 2, 4
    assert blobs(case(**dict(SWEEP, density=0)), 4) == []
    jump = blobs(case(**dict(SWEEP, brush_position=keyed((0, (20, 50), "hold"), (3, (80, 50))))), 4)
    assert sum(1 for b in jump if near(b, (3.2, 5.0))) == 5 and near(jump[5], (8.0, 5.0))

    one = c["FX-GLUE-001"]
    assert one["0"] == one["4"] and one["0"] != photo
    two = c["FX-GLUE-002"]
    # Unlit at Reflection 0, the stroke shows the photo itself: within it, the photo; at frame
    # 0 only near the first blob.
    half = [photo[at(8, 4)][i] / 2 for i in range(3)] + [photo[at(8, 4)][3]]
    assert near(two["4"][at(8, 4)], half) and two["4"][at(8, 1)] == photo[at(8, 1)]
    assert two["0"][at(3, 4)] != photo[at(3, 4)] or two["0"][at(3, 5)] != photo[at(3, 5)]
    assert two["0"][at(10, 4)] == photo[at(10, 4)]
    three = c["FX-GLUE-003"]["4"]
    lum = lambda p: sum(p[:3]) / p[3] if p[3] > 0 else 0  # noqa: E731
    assert three != ramp and three[at(8, 0)] == ramp[at(8, 0)]
    assert c["FX-GLUE-004"]["4"][at(3, 5)] == photo[at(3, 5)] != two["4"][at(3, 5)]
    five, six = c["FX-GLUE-005"]["4"], c["FX-GLUE-006"]["4"]
    # Discs at x 3.2, 8, 12.8 apart at Strength 0, joined at 100 between 3.2 and 8.
    assert five[at(5, 4)] == photo[at(5, 4)] and six[at(5, 4)] != photo[at(5, 4)]
    assert c["FX-GLUE-007"]["4"] != two["4"]
    eight = c["FX-GLUE-008"]
    assert eight["4"] != three and eight["6"] != eight["4"]
    assert c["FX-GLUE-009"]["4"] not in (three, photo)
    ten = c["FX-GLUE-010"]["4"]
    assert ten != three
    eleven = c["FX-GLUE-011"]["4"]
    assert all(eleven[at(x, y)] == three[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    assert c["FX-GLUE-012"]["4"] == photo and c["FX-GLUE-013"]["4"] == photo
    fourteen = c["FX-GLUE-014"]["4"]
    gap = fourteen[at(7, 5)]
    assert gap[3] > 0.5 and gap[0] == gap[1] == gap[2]  # black paint, white highlight or none
    fifteen = c["FX-GLUE-015"]
    assert fifteen["2"][at(10, 5)] == photo[at(10, 5)] and fifteen["4"][at(10, 5)] != photo[at(10, 5)]
    sixteen = c["FX-GLUE-016"]["4"]
    assert lum(three[at(8, 3)]) > lum(sixteen[at(8, 3)]) and lum(three[at(8, 6)]) < lum(sixteen[at(8, 6)])
    seventeen = c["FX-GLUE-017"]
    assert seventeen["0"] != seventeen["4"]
    assert c["FX-GLUE-018"]["2"] != two["2"]
    assert value_at(case(**dict(SWEEP, brush_position=keyed((0, (20, 50), OVERSHOOT),
                                                            (4, (80, 50)))))["brush_position"],
                    3)[0] > 80
    print("checked")


if __name__ == "__main__":
    main()
