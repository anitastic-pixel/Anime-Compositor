"""Drizzle, worked a second way.

D-672 adds Drizzle (`core.drizzle`), after CycoreFX's CC Drizzle (Simulation): raindrops falling
on a pond, each a ring of ripples that is born, spreads and dies away, bending and lighting the
picture as the water's surface would. The CycoreFX manual names the controls and says what each
is for (Drip Rate, how many drops are born; Longevity, how long a ripple takes to spread to its
full radius, "cannot be animated"; Rippling, "each rotation around the dial adds another ring";
Displacement; Ripple Height, which "affects the displacement as well as the shaded
appearance"; Spreading, the radius the ripples reach; and the Light and Shading groups shared
with CC Blobbylize and CC Glue Gun). It publishes no formula, so the rule, the units, the ranges
and the settings when added are ours. No state is carried from frame to frame (document 20):
everything is worked from the layer's in point. There is no seed control, as CycoreFX's has none.

At composition frame f, the layer's in point i0, r frames a second, u = f - i0 frames since the
in point, on a drawing w by h, with the settings held inside their ranges:

1. The drops born so far. C(m), m = 0 .. u, is the trapezoid sum of `drip_rate` (drops a
   second) over the frames i0 .. i0 + m, divided by r: C(0) = 0, C(m + 1) = C(m) + (rate(m) +
   rate(m + 1)) / (2 r). Between whole frames C is joined by straight lines; before the in point
   it is 0. Drop j (j = 1, 2, ...) is born at b_j, the first t with C(t) = j.
2. L = `longevity` in seconds, as it is at the in point (CycoreFX's "cannot be animated": a keyed
   longevity is read at the in point only). Drop j is alive at u when C(u - L r) < j <= C(u):
   its age is a = (u - b_j) / r seconds and s = a / L, 0 <= s < 1. With L = 0 no drop lives.
3. Drop j lands at (u_0 w, u_1 h), u_k = (U(0, j, 0, 0, k) + 1) / 2, U Noise's hash (D-119) at
   seed 0. Its ripple's radius is R = s R_max, R_max = (`spreading` / 100) h / 4; its height A =
   (`ripple_height` / 100) (h / 50) 4 s (1 - s), rising and falling over its life; its rings n =
   1 + `rippling` / 360.
4. Within R of the drop, q = d / R, the ripple's surface is z = A (1 - q^2)^2 cos(2 pi n q):
   smooth at the drop and flat at the ripple's edge. Its slope is G (dx, dy) / R^2, where
   G = A (-4 (1 - q^2) cos(2 pi n q) - (1 - q^2)^2 2 pi n sin(2 pi n q) / q), the last fraction
   (2 pi n)^2 at q = 0. The water's slope (Zx, Zy) is the sum over the live drops.
5. At a pixel centre (X, Y) the picture is read at (X, Y) - k (Zx, Zy), k = (`displacement` / 100)
   (h / 50), by the bilinear read below: premultiplied c, its straight colour col = c / alpha
   (black where clear).
6. The water's normal N = (-Zx, -Zy, 1) / |.| is lit by Blobbylize's rule (D-379): the light L =
   (100 ux, 100 uy, `light_height`) / |.|, (ux, uy) the direction `light_direction` degrees
   points, or for a point light (lx - X, ly - Y, `light_height`) / |.|, (lx, ly) =
   (`light_position_x` w, `light_position_y` h) / 100; nl = N.L, rz = 2 nl N_z - L_z, shine =
   rz^(1 / `roughness`) where nl > 0 and rz > 0, else 0; each channel lit = col (ambient +
   diffuse I Lc max(nl, 0)) + specular I (Lc + (col - Lc) metal) shine, I = `light_intensity` /
   100, Lc the light's colour linear, the shares over 100. The pixel is (lit alpha, alpha). Flat
   water is lit too, as Blobbylize's flat parts are.

The bilinear read, premultiplied and linear: fx = clamp(X' - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge.

`drip_rate` 0 to 1000, 10 when added; `longevity` 0 to 100 seconds, 1; `rippling` 0 to 3600
degrees, 360; `displacement` -1000 to 1000, 50; `ripple_height` -1000 to 1000, 50; `spreading`
0 to 1000, 50; and Glue Gun's light and shading: `light_intensity` 0 to 400, 100; `light_color`
#ffffff; `light_type` `distant` or `point`, distant; `light_height` -1000 to 1000, 100;
`light_position_x` and `_y` -1000 to 1000 per cent, 30 and 30; `light_direction` -3600 to 3600,
-45; `ambient` 25, `diffuse` 75, `specular` 50, `metal` 100, each 0 to 100; `roughness` 0.001
to 1, 0.05. Every number keyable; all but `drip_rate` (summed from the in point) and
`longevity` (read at the in point) are read as they are now, every live ripple taking them. The
layer does not grow. Every length is a share of the drawing's height, but for a point light's
height, in pixels, which a draft scales. After Effects' lights (the manual's Using: AE Lights)
are not offered.

**This file never runs the build's code path.** It sums every live drop at every pixel, where
the build skips the drops out of reach, in double precision from the drawings' 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size: Bubbles' colour
field, or the field with its right half clear. The projects go into `Fixtures/drizzle`, the
expected frames into `Fixtures/drizzle/expected_drizzle.json`. Every case is "not built yet"
until B-551 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/drizzle_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "drizzle"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.drizzle"
SETTINGS = [("drip_rate", 0, 1000, 10), ("longevity", 0, 100, 1), ("rippling", 0, 3600, 360),
            ("displacement", -1000, 1000, 50), ("ripple_height", -1000, 1000, 50),
            ("spreading", 0, 1000, 50), ("light_intensity", 0, 400, 100),
            ("light_height", -1000, 1000, 100), ("light_position_x", -1000, 1000, 30),
            ("light_position_y", -1000, 1000, 30), ("light_direction", -3600, 3600, -45),
            ("ambient", 0, 100, 25), ("diffuse", 0, 100, 75), ("specular", 0, 100, 50),
            ("roughness", 0.001, 1, 0.05), ("metal", 0, 100, 100)]
WORDS = [("light_type", ("distant", "point"), "distant")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def counts(c, u):
    """Step 1: C(0) .. C(u)."""
    i0, r = c["in_frame"], c["rate"]
    rate = [settings(c, i0 + m)["drip_rate"] for m in range(u + 1)]
    out = [0.0]
    for m in range(u):
        out.append(out[-1] + (rate[m] + rate[m + 1]) / (2 * r))
    return out


def count_at(C, t):
    if t <= 0:
        return 0.0
    m = math.floor(t)
    if m >= len(C) - 1:
        return C[-1]
    return C[m] + (C[m + 1] - C[m]) * (t - m)


def born(C, j):
    """The first t with C(t) = j, for 0 < j <= C[-1]."""
    m = 0
    while C[m + 1] < j:
        m += 1
    return m + (j - C[m]) / (C[m + 1] - C[m])


def ripples(c, f, w=W, h=H):
    """Steps 1 to 3: the live drops at frame f as (x, y, R, A)."""
    u = f - c["in_frame"]
    v = settings(c, f)
    life = clamp(value_at(c["longevity"], c["in_frame"]), *RANGES["longevity"])
    if u < 0 or life == 0:
        return []
    C = counts(c, u)
    lo, hi = count_at(C, u - life * c["rate"]), C[-1]
    out = []
    for j in range(math.floor(lo) + 1, math.floor(hi) + 1):
        if j <= lo:
            continue
        s = (u - born(C, j)) / c["rate"] / life
        rr = s * v["spreading"] / 100 * h / 4
        if rr <= 0:
            continue
        a = v["ripple_height"] / 100 * (h / 50) * 4 * s * (1 - s)
        x, y = ((U(0, j, 0, 0, k) + 1) / 2 for k in (0, 1))
        out.append((x * w, y * h, rr, a))
    return out


def drizzle(src, v, light_type, light_color, drops, w=W, h=H):
    n = 1 + v["rippling"] / 360
    k = v["displacement"] / 100 * (h / 50)
    lc = BL.hex_linear(light_color)
    inten = v["light_intensity"] / 100
    amb, dif, spc, metal = (v[x] / 100 for x in ("ambient", "diffuse", "specular", "metal"))
    ux, uy = along(v["light_direction"])
    out = []
    for py in range(h):
        for px in range(w):
            X, Y = px + 0.5, py + 0.5
            zx = zy = 0.0
            for (x, y, rr, a) in drops:
                dx, dy = X - x, Y - y
                d2 = dx * dx + dy * dy
                if d2 >= rr * rr:
                    continue
                q = math.sqrt(d2) / rr
                one = 1 - q * q
                ph = 2 * math.pi * n * q
                sq = (2 * math.pi * n) ** 2 if q == 0 else 2 * math.pi * n * math.sin(ph) / q
                g = a * (-4 * one * math.cos(ph) - one * one * sq)
                zx += g * dx / (rr * rr)
                zy += g * dy / (rr * rr)
            cc = BU.bilinear(src, X - k * zx, Y - k * zy, w, h)
            al = cc[3]
            col = [cc[i] / al for i in range(3)] if al > 0 else [0.0] * 3
            nn = [-zx, -zy, 1.0]
            ln = math.sqrt(sum(t * t for t in nn))
            nn = [t / ln for t in nn]
            if light_type == "point":
                lv = [v["light_position_x"] / 100 * w - X, v["light_position_y"] / 100 * h - Y,
                      v["light_height"]]
            else:
                lv = [100 * ux, 100 * uy, v["light_height"]]
            ll = math.sqrt(sum(t * t for t in lv))
            lv = [t / ll for t in lv] if ll > 0 else [0.0] * 3
            nl = sum(nn[i] * lv[i] for i in range(3))
            rz = 2 * nl * nn[2] - lv[2]
            shine = rz ** (1 / v["roughness"]) if nl > 0 and rz > 0 else 0.0
            lit = [col[i] * (amb + dif * inten * lc[i] * max(nl, 0.0))
                   + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * shine for i in range(3)]
            out.append([lit[i] * al for i in range(3)] + [al])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    c["light_color"] = "#ffffff"
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift)
    return c


# "Unlit": Ambient 100 and no diffuse or specular light, so the water shows the picture bent
# and nothing else; where there is no ripple the picture is untouched.
FLAT = dict(ambient=100, diffuse=0, specular=0)
# Four drops a second spreading to 24 pixels, Displacement 200: a close look.
BIG = dict(drip_rate=4, spreading=300, displacement=200)


def big(**kw):
    return case(**{**BIG, **kw})


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    if not all(c[name] in words for name, words, _ in WORDS):
        return False
    lc = c["light_color"]
    return len(lc) == 7 and lc[0] == "#" and all(ch in "0123456789abcdefABCDEF" for ch in lc[1:])


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c) or f < c["in_frame"]:
        return BU.frame(src, c["shift"])
    out = drizzle(src, settings(c, f), c["light_type"], c["light_color"], ripples(c, f))
    return BU.frame(out, c["shift"])


def plain(c):
    return BU.frame(BU.working(c["drawing"]), c["shift"])


CASES = {
    "FX-DRIZZLE-001": ("As added: ten drops a second, each ripple spreading for a second to a "
                       "radius of an eighth of the picture's height (4 pixels here), two rings, "
                       "lit from the top left: at frame 0 no drop yet, the still water lit "
                       "evenly; at frames 13 and 25 small rings over the picture.",
                       case(), (0, 13, 25)),
    "FX-DRIZZLE-002": ("Four drops a second spreading to 24 pixels, Displacement 200, unlit: the "
                       "picture bent by wide rings and untouched between them; frame 8 has one "
                       "young ripple, frame 20 three.", big(**FLAT), (8, 14, 20)),
    "FX-DRIZZLE-003": ("FX-DRIZZLE-002 lit as added: the rings stand out, lighter on the sides "
                       "facing the top-left light.", big(), (14,)),
    "FX-DRIZZLE-004": ("Rippling 0: one ring a ripple.", big(rippling=0, **FLAT), (14,)),
    "FX-DRIZZLE-005": ("Rippling 1080: four rings a ripple, closer together.",
                       big(rippling=1080, **FLAT), (14,)),
    "FX-DRIZZLE-006": ("Displacement 0: the picture is not bent, only lit by the rings.",
                       big(displacement=0), (14,)),
    "FX-DRIZZLE-007": ("Displacement -200: bent the other way.", big(displacement=-200, **FLAT),
                       (14,)),
    "FX-DRIZZLE-008": ("Ripple Height 0: still water, so unlit the picture is untouched.",
                       big(ripple_height=0, **FLAT), (14,)),
    "FX-DRIZZLE-009": ("Ripple Height -100: the rings dip where they rose, twice as deep.",
                       big(ripple_height=-100), (14,)),
    "FX-DRIZZLE-010": ("Longevity 0.5: each ripple spreads in half a second and is gone, so "
                       "fewer live at once.", big(longevity=0.5, **FLAT), (14, 20)),
    "FX-DRIZZLE-011": ("Longevity 0: no ripple lives; unlit, the picture untouched.",
                       big(longevity=0, **FLAT), (14,)),
    "FX-DRIZZLE-012": ("Drip Rate 0: no drops; unlit, the picture untouched.",
                       big(drip_rate=0, **FLAT), (14,)),
    "FX-DRIZZLE-013": ("Drip Rate keyed from 0 at frame 0 to 24 at frame 12: by frame 11 five "
                       "drops have been born, the rate summed from the in point.",
                       big(drip_rate=keyed((0, 0), (12, 24)), **FLAT), (6, 11)),
    "FX-DRIZZLE-014": ("Longevity keyed from 1 at frame 0 to 0.25 at frame 12: it is read at "
                       "the in point only, so the same as FX-DRIZZLE-002.",
                       big(longevity=keyed((0, 1), (12, 0.25)), **FLAT), (14, 20)),
    "FX-DRIZZLE-015": ("FX-DRIZZLE-002 with the layer starting at frame 6: frame 14 is 002's "
                       "frame 8 and frame 26 its frame 20, because the drops start at the in "
                       "point.", big(in_frame=6, **FLAT), (14, 26)),
    "FX-DRIZZLE-016": ("FX-DRIZZLE-003 moved 3 pixels right: the same, moved.",
                       big(shift=3), (14,)),
    "FX-DRIZZLE-017": ("FX-DRIZZLE-002 at 30 frames a second: frame 14 is seven fifteenths "
                       "of a second in, not seven twelfths.", big(rate=30, **FLAT), (14,)),
    "FX-DRIZZLE-018": ("A point light at the middle, 10 pixels up: the water brightest under "
                       "it and darker towards the corners.",
                       big(light_type="point", light_position_x=50, light_position_y=50,
                           light_height=10), (14,)),
    "FX-DRIZZLE-019": ("An orange light, Specular 100, Roughness 0.5, Metal 0, Diffuse 0: "
                       "broad highlights in the light's own orange on the rings.",
                       big(light_color="#ff8000", specular=100, roughness=0.5, metal=0,
                           diffuse=0, light_height=40), (14,)),
    "FX-DRIZZLE-020": ("The picture with its right half clear: the clear half stays clear "
                       "except where a ring bends the picture across the edge.",
                       big(drawing="half"), (14,)),
    "FX-DRIZZLE-021": ("Light Direction 135, from the bottom right: the other sides of the "
                       "rings light.", big(light_direction=135), (14,)),
    "FX-DRIZZLE-022": ("Spreading 0: no ripple has a size; lit as added, the still water.",
                       big(spreading=0), (14,)),
    "FX-DRIZZLE-023": ("Spreading keyed from 300 at frame 0 to 100 at frame 12: every live "
                       "ripple takes the spreading as it is now, so at frame 14 all are a third "
                       "the size of FX-DRIZZLE-002's.",
                       big(spreading=keyed((0, 300), (12, 100)), **FLAT), (14,)),
    "FX-DRIZZLE-024": ("Thirty drops a second, Rippling 720, lit as added: a downpour.",
                       big(drip_rate=30, rippling=720, spreading=150), (25,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-DRIZZLE-025": ("Drip Rate -1, below 0.", big(drip_rate=-1)),
    "FX-DRIZZLE-026": ("Drip Rate 1001, above 1000.", big(drip_rate=1001)),
    "FX-DRIZZLE-027": ("Longevity 101, above 100 seconds.", big(longevity=101)),
    "FX-DRIZZLE-028": ("Rippling 3601, above 3600.", big(rippling=3601)),
    "FX-DRIZZLE-029": ("Displacement 1001, above 1000.", big(displacement=1001)),
    "FX-DRIZZLE-030": ("Ripple Height -1001, below -1000.", big(ripple_height=-1001)),
    "FX-DRIZZLE-031": ("Spreading 1001, above 1000.", big(spreading=1001)),
    "FX-DRIZZLE-032": ("Roughness 0, below 0.001.", big(roughness=0)),
    "FX-DRIZZLE-033": ("A light type written \"spot\".", big(light_type="spot")),
    "FX-DRIZZLE-034": ("A light colour written \"white\".", big(light_color="white")),
    "FX-DRIZZLE-035": ("Light position across 1001, past ten widths.",
                       big(light_position_x=1001)),
    "FX-DRIZZLE-036": ("Drip Rate keyed to 2000 at frame 4, above 1000.",
                       big(drip_rate=keyed((0, 4), (4, 2000)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    frames = 30
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    params["light_color"] = c["light_color"]
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
        print(f"{fx}: " + ", ".join(
            f"frame {f} {len(ripples(c, int(f)))} drops, "
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
    (OUT / "expected_drizzle.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for x, y in zip(p, q) for a, b in zip(x, y))  # noqa: E731
    field = plain(case())
    # the drops: births and lives
    assert ripples(case(), 0) == [] and len(ripples(case(), 13)) == 5
    assert len(ripples(case(), 25)) == 10  # drop 1, born at frame 2.4, lives to 26.4
    assert [len(ripples(big(), f)) for f in (8, 14, 20)] == [1, 2, 3]
    assert len(ripples(big(drip_rate=keyed((0, 0), (12, 24))), 11)) == 5  # C(11) = 121 / 24
    assert len(ripples(big(longevity=0.5), 20)) == 2
    first = c["FX-DRIZZLE-001"]["0"]
    assert first != field and all(abs(p[0] / q[0] - first[0][0] / field[0][0]) < 1e-9
                                  for p, q in zip(first, field))  # still water: one shading
    for fx in ("FX-DRIZZLE-008", "FX-DRIZZLE-011", "FX-DRIZZLE-012"):
        assert near(c[fx]["14"], field), fx
    two = c["FX-DRIZZLE-002"]
    changed = lambda fr: sum(fr[i] != field[i] for i in range(W * H))  # noqa: E731
    assert 0 < changed(two["8"]) < changed(two["20"]) and two["14"] != two["20"]
    assert c["FX-DRIZZLE-014"]["14"] == two["14"] and c["FX-DRIZZLE-014"]["20"] == two["20"]
    assert c["FX-DRIZZLE-015"]["14"] == two["8"] and c["FX-DRIZZLE-015"]["26"] == two["20"]
    moved, base = c["FX-DRIZZLE-016"]["14"], c["FX-DRIZZLE-003"]["14"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert c["FX-DRIZZLE-017"]["14"] != two["14"]
    assert c["FX-DRIZZLE-022"]["14"] == render(big(drip_rate=0), 14)
    for n in (4, 5, 7, 10, 23):
        assert c["FX-DRIZZLE-%03d" % n]["14"] != two["14"], n
    assert c["FX-DRIZZLE-006"]["14"] != c["FX-DRIZZLE-003"]["14"]
    assert c["FX-DRIZZLE-009"]["14"] != c["FX-DRIZZLE-003"]["14"]
    # the clear half: clear at the far right, where no ring reaches across
    half = c["FX-DRIZZLE-020"]["14"]
    assert half[5 * W + 47][3] == 0 or any(abs(r[0] - 47.5) < r[2] for r in ripples(big(), 14))
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
