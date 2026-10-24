"""Turbulent Noise's Overflow, Transform, Sub Settings and more blending modes, worked a second way.

D-647 adds to `core.turbulent_noise` (D-445, D-567; `tools/turbulent_noise_reference.py` and
`tools/turbulent_noise_types_reference.py`, whose rules this file reuses unchanged) the rest of
After Effects' Turbulent Noise controls this program had left out. Adobe publishes no formula for
any of them; every rule below is this program's own, after the controls' names, and nothing is
ported.

Overflow, on v = 0.5 + 0.5 n contrast / 100 + brightness / 100 before the colours:
- `allow_hdr` (when added): as before D-647, held to 0..1, and in Float only at black (D-319);
- `clip`: held to 0..1 in every depth;
- `soft_clamp`: 0.5 + 0.5 tanh(2 (v - 0.5)), unchanged in slope at the middle grey and eased
  toward black and white, never reaching them;
- `wrap_back`: t = v mod 2, then t up to 1 and 2 - t past it, so the greys fold back at black
  and white instead of flattening.

Transform. `rotation` in degrees turns the clouds clockwise round the drawing's middle c (its
width and height halved, in its own pixels); `uniform_scaling` `on` draws Size across and down,
leaving Scale Width and Scale Height out; `perspective_offset` `on` slides octave o by Offset
Turbulence times 0.5^o, so the finer octaves slide less, as if further away.

Sub Settings. `sub_influence`, 0 to 100 per cent, each octave's strength over the last's (50
when added, the 0.5 before D-647; Cloudy's 0.35 is scaled by it over 50); `sub_scaling`, 10 to
100 per cent, each octave's size over the last's (50 when added, twice as fine each octave);
`sub_rotation`, degrees, each octave turned that much more than the last; `sub_offset`, x then y
in pixels, each octave slid that much more than the last; `center_subscale` `on` makes each
octave finer round the drawing's middle rather than round its corner.

So octave o of n, with p the pixel's centre in the drawing's own space, sx and sy the size
across and down, w_o = 0.5^o with Perspective Offset on, else 1, and fine_o = f_0 (100 /
sub_scaling)^o (f_0 4 for small_bumps, else 1), is read at

    u = p - w_o offset - o sub_offset,
    q = c + R(-(rotation + o sub_rotation)) (u - c),  R(-t) (x, y) = (x cos t + y sin t, -x sin t + y cos t),
    (q.x / sx, q.y / sy) fine_o, or (c.x + fine_o (q.x - c.x)) / sx and the same down with
    Center Subscale on, and the depth z fine_o,

with D-567's moves of the type after it. With every one of these as added the clouds are D-445's
and D-567's to the bit: the build takes the old path.

Blending modes past normal, multiply, screen and add, on the straight linear colour b and the
noise's grey g: `soft_light` (the W3C's, D-185's), `hard_light` (2 b g with g up to 0.5, else
1 - 2 (1 - b)(1 - g)), `darken` (min), `lighten` (max) and `difference` (|b - g|).

D-731 (the owner's specification decision of 2026-10-13): FX-TURBNOISE-060 was regenerated from a
refused Hue to a drawn one: the W3C Compositing Level 1 Hue, SetLum(SetSat(g, Sat(b)), Lum(b)),
our basis as D-730's Saturation, Color and Luminosity (`tools/turbulent_blends_reference.py`), not
a match to After Effects. With the noise grey it gives the layer's own grey. No other case changed.

**This file never runs the build's code path.** Cases as in `tools/turbulent_noise_reference.py`:
a 16 by 10 composition holding Noise's card. The projects go into `Fixtures/turbulent_noise` as
`fx_turbnoise_033.json` on, the expected frames into
`Fixtures/turbulent_noise/expected_turbulent_more.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/turbulent_more_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402
import turbulent_noise_reference as T  # noqa: E402
import turbulent_noise_types_reference as TT  # noqa: E402

W, H = T.W, T.H
OUT = T.OUT
TOLERANCE = 2e-5
RANGES = {**T.RANGES, "rotation": (-100000, 100000), "sub_influence": (0, 100),
          "sub_scaling": (10, 100), "sub_rotation": (-100000, 100000)}
STARTS = {"overflow": "allow_hdr", "rotation": 0, "uniform_scaling": "off",
          "perspective_offset": "off", "sub_influence": 50, "sub_scaling": 50, "sub_rotation": 0,
          "sub_offset": [0, 0], "center_subscale": "off"}
MIXES = {**BLENDS,
         "soft_light": lambda b, g: (b - (1 - 2 * g) * b * (1 - b)) if g <= 0.5 else
         b + (2 * g - 1) * ((((16 * b - 12) * b + 4) * b if b <= 0.25 else math.sqrt(b)) - b),
         "hard_light": lambda b, g: 2 * b * g if g <= 0.5 else 1 - 2 * (1 - b) * (1 - g),
         "darken": min, "lighten": max, "difference": lambda b, g: abs(b - g)}


# --- the field ------------------------------------------------------------------------------

def moved(c, n):
    """Whether any octave is read anywhere but D-445's point."""
    return (n["rotation"] != 0 or c["perspective_offset"] == "on" or n["sub_influence"] != 50
            or n["sub_scaling"] != 50 or n["sub_rotation"] != 0 or list(c["sub_offset"]) != [0, 0]
            or c["center_subscale"] == "on")


def F_more(seed, X, Y, z, octaves, kind, noise, g):
    """D-567's fractal, with each octave read at the point the Transform and Sub Settings say."""
    step = 0.35 * (g["influence"] / 50) if kind == "cloudy" else g["influence"] / 100
    fine = 4.0 if kind == "small_bumps" else 1.0
    (cx, cy), (sx, sy), off, sub = g["middle"], g["size"], g["offset"], g["sub_offset"]
    total = weight = 0.0
    amp, so_far, prev, prev_ridge, best = 1.0, 0.0, 0.0, 1.0, 0.0
    for o in range(octaves):
        w = 0.5 ** o if g["perspective"] else 1.0
        ux, uy = X - w * off[0] - o * sub[0], Y - w * off[1] - o * sub[1]
        t = math.radians(g["rotation"] + o * g["sub_rotation"])
        dx, dy = ux - cx, uy - cy
        qx, qy = cx + dx * math.cos(t) + dy * math.sin(t), cy - dx * math.sin(t) + dy * math.cos(t)
        if g["center"]:
            px, py = (cx + fine * (qx - cx)) / sx, (cy + fine * (qy - cy)) / sy
        else:
            px, py = fine * qx / sx, fine * qy / sy
        pz = z * fine
        if kind == "dynamic":
            px, py = px + 4 * so_far, py + 4 * prev
        elif kind == "dynamic_progressive":
            px, py = px + 2 * o * so_far, py + 2 * o * prev
        elif kind == "dynamic_twist" and o > 0:
            px, py = px + 2 * math.cos(2 * math.pi * so_far), py + 2 * math.sin(2 * math.pi * so_far)
        elif kind == "smeary" and o > 0:
            px = px / 4 + 2 * so_far
        v = TT.V(seed, 8 * o, px, py, pz, noise)
        ridge = (1 - abs(v)) ** 2
        gv = {"turbulent": abs(v), "turbulent_smooth": v * v, "turbulent_sharp": math.sqrt(abs(v)),
              "terrain": ridge * prev_ridge, "threads": 2 * max(0.0, 1 - 6 * abs(v)) - 1}.get(kind, v)
        if kind == "subscale" and o > 0:
            gv *= (prev + 1) / 2
        total += amp * gv
        weight += amp
        so_far = total / weight
        best = v if o == 0 else best + amp * max(0.0, v - best)
        prev, prev_ridge = v, ridge
        amp *= step
        fine *= 100 / g["sub_scaling"]
    b = total / weight
    if kind in ("turbulent", "turbulent_smooth", "turbulent_sharp", "terrain"):
        return 2 * b - 1
    if kind == "max":
        return best
    if kind == "swirly":
        return math.sin(3 * math.pi * b)
    if kind == "rocky":
        return min(1.0, max(-1.0, math.floor(2 * b + 0.5) / 2))
    if kind == "cloudy":
        return (b + 1) ** 2 / 2 - 1
    if kind == "strings":
        return 2 * max(0.0, 1 - 6 * abs(b)) - 1
    return b


def over(v, how):
    if how == "soft_clamp":
        return 0.5 + 0.5 * math.tanh(2 * (v - 0.5))
    if how == "wrap_back":
        t = v % 2
        return t if t <= 1 else 2 - t
    return min(1.0, max(0.0, v))  # allow_hdr in 8 bits, and clip


# --- the rule -------------------------------------------------------------------------------

def value(c, n, x, y):
    if c["uniform_scaling"] == "on":
        n = {**n, "scale_width": 100, "scale_height": 100}
    if moved(c, n):
        g = {"middle": (W / 2, H / 2), "offset": c["offset"], "sub_offset": c["sub_offset"],
             "size": (n["size"] / T.CELL * n["scale_width"] / 100,
                      n["size"] / T.CELL * n["scale_height"] / 100),
             "perspective": c["perspective_offset"] == "on", "rotation": n["rotation"],
             "sub_rotation": n["sub_rotation"], "influence": n["sub_influence"],
             "sub_scaling": n["sub_scaling"], "center": c["center_subscale"] == "on"}
        f = F_more(n["seed"], x + 0.5, y + 0.5, n["evolution"] / 360, n["complexity"],
                   c["fractal_type"], c["noise_type"], g)
    else:
        f = TT.F(n["seed"], *T.point(x, y, n, c["offset"]), n["complexity"], c["fractal_type"],
                 c["noise_type"])
    if c["invert"] == "on":
        f = -f
    return over(0.5 + 0.5 * f * T.CONTRAST * n["contrast"] / 100 + n["brightness"] / 100,
                c["overflow"])


def turbulent_noise(pixels, c, n, width=W):
    op = n["opacity"] / 100
    if c["blend"] == "hue":  # D-731: a whole-colour mode, as D-730's
        import turbulent_blends_reference as B
        whole = B.WHOLE["hue"]
    else:
        mix = MIXES[c["blend"]]
        whole = lambda b, s: [mix(v, s[0]) for v in b]  # noqa: E731
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        g = srgb_to_linear(value(c, n, i % width, i // width))
        a = p[3] / 255
        b = [srgb_to_linear(p[ch] / 255) for ch in range(3)]
        m = whole(b, [g, g, g])
        out.append([(b[ch] + op * (m[ch] - b[ch])) * a for ch in range(3)] + [a])
    return out


def held(c, k, frame_no):
    if k in T.RANGES:
        return T.held(c, k, frame_no)
    lo, hi = RANGES[k]
    return min(hi, max(lo, T.value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in T.DRAWINGS[c["drawing"]] for p in row]
    n = {k: held(c, k, frame_no) for k in RANGES}
    return R.frame(turbulent_noise(pixels, c, n), c["shift"])


# --- the cases ------------------------------------------------------------------------------

def case(**kw):
    more = {k: kw.pop(k) for k in list(kw) if k in STARTS}
    c = T.case(**kw)
    c.update(STARTS)
    c.update(more)
    c["sub_offset"] = list(c["sub_offset"])
    return c


FINE = T.FINE

CASES = {
    "FX-TURBNOISE-033": ("Size 4, contrast 300, Overflow clip: in an 8-bit composition the same "
                         "as FX-TURBNOISE-009, allow_hdr holding to 0..1 there too.",
                         case(contrast=300, overflow="clip", **FINE), [0]),
    "FX-TURBNOISE-034": ("Size 4, contrast 300, Overflow soft_clamp: the greys eased toward black "
                         "and white, none reaching them.",
                         case(contrast=300, overflow="soft_clamp", **FINE), [0]),
    "FX-TURBNOISE-035": ("Size 4, contrast 300, Overflow wrap_back: the greys past white and "
                         "black fold back, light bands inside the dark.",
                         case(contrast=300, overflow="wrap_back", **FINE), [0]),
    "FX-TURBNOISE-036": ("Size 4, Rotation 90: FX-TURBNOISE-002's clouds turned a quarter "
                         "clockwise round the card's middle.", case(rotation=90, **FINE), [0]),
    "FX-TURBNOISE-037": ("Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: "
                         "frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036.",
                         case(rotation=keyed((0, 0), (4, 90)), **FINE), [0, 2, 4]),
    "FX-TURBNOISE-038": ("Size 4, Scale Width 200, Scale Height 50, Uniform Scaling on: the two "
                         "scales left out, so it is FX-TURBNOISE-002.",
                         case(scale_width=200, scale_height=50, uniform_scaling="on", **FINE),
                         [0]),
    "FX-TURBNOISE-039": ("Size 4, Offset (6, -4), Perspective Offset on: the coarse clouds slide "
                         "6 right and 4 up, each finer octave half as far as the last.",
                         case(offset=(6, -4), perspective_offset="on", **FINE), [0]),
    "FX-TURBNOISE-040": ("Size 4, Perspective Offset on with no offset: nothing slides, so it is "
                         "FX-TURBNOISE-002 within the tolerance.",
                         case(perspective_offset="on", **FINE), [0]),
    "FX-TURBNOISE-041": ("Size 4, Sub Influence 100: every octave as strong as the first, busier "
                         "fine detail.", case(sub_influence=100, **FINE), [0]),
    "FX-TURBNOISE-042": ("Size 4, Sub Influence 0: only the first octave counts, so it is "
                         "FX-TURBNOISE-007 (complexity 1) within the tolerance.",
                         case(sub_influence=0, **FINE), [0]),
    "FX-TURBNOISE-043": ("Size 4, Sub Scaling 70: each octave only a little finer than the last, "
                         "softer clouds.", case(sub_scaling=70, **FINE), [0]),
    "FX-TURBNOISE-044": ("Size 4, Sub Rotation 45: each finer octave turned an eighth more.",
                         case(sub_rotation=45, **FINE), [0]),
    "FX-TURBNOISE-045": ("Size 4, Sub Offset (5, 3): each finer octave slid further.",
                         case(sub_offset=(5, 3), **FINE), [0]),
    "FX-TURBNOISE-046": ("Size 4, Center Subscale on: the octaves grow finer round the card's "
                         "middle, not its corner.", case(center_subscale="on", **FINE), [0]),
    "FX-TURBNOISE-047": ("Size 4, dynamic on spline, Rotation 30, Offset (3, 2) with Perspective "
                         "Offset, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset "
                         "(1, -2), Center Subscale on: everything together.",
                         case(fractal_type="dynamic", noise_type="spline", rotation=30,
                              offset=(3, 2), perspective_offset="on", sub_influence=70,
                              sub_scaling=60, sub_rotation=20, sub_offset=(1, -2),
                              center_subscale="on", **FINE), [0]),
    "FX-TURBNOISE-048": ("Size 4, cloudy, Sub Influence 80: cloudy's 0.35 a step scaled to 0.56.",
                         case(fractal_type="cloudy", sub_influence=80, **FINE), [0]),
    "FX-TURBNOISE-049": ("Size 4, blending mode soft_light: the card's shading lifted and "
                         "lowered gently by the clouds.", case(blend="soft_light", **FINE), [0]),
    "FX-TURBNOISE-050": ("Size 4, blending mode hard_light, opacity 70: as soft_light but "
                         "harder.", case(blend="hard_light", opacity=70, **FINE), [0]),
    "FX-TURBNOISE-051": ("Size 4, blending mode darken: each channel the darker of the card and "
                         "the clouds.", case(blend="darken", **FINE), [0]),
    "FX-TURBNOISE-052": ("Size 4, blending mode lighten: each channel the lighter.",
                         case(blend="lighten", **FINE), [0]),
    "FX-TURBNOISE-053": ("Size 4, blending mode difference: each channel how far apart; the "
                         "black patch shows the clouds themselves.",
                         case(blend="difference", **FINE), [0]),
    "FX-TURBNOISE-054": ("FX-TURBNOISE-036 moved three pixels right: turned round the drawing's "
                         "own middle, so the turned clouds move with it.",
                         case(rotation=90, shift=3, **FINE), [0]),
    "FX-TURBNOISE-055": ("FX-TURBNOISE-002 with every D-647 setting written as added: "
                         "FX-TURBNOISE-002 to the bit.", case(**FINE), [0]),
}

# D-46: the file is read, the effect is kept as written and left out of every frame, with
# EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TURBNOISE-056": ("Overflow \"hdr\", not one of the four.", case(overflow="hdr")),
    "FX-TURBNOISE-057": ("Sub Scaling 5, below 10.", case(sub_scaling=5)),
    "FX-TURBNOISE-058": ("Sub Influence 101, above 100.", case(sub_influence=101)),
    "FX-TURBNOISE-059": ("Uniform Scaling \"yes\", not off or on.", case(uniform_scaling="yes")),
}

# D-731: regenerated by the owner's decision of 2026-10-13, drawn where it was refused; kept last,
# where it stood, so the file's order is unchanged.
REGENERATED = {
    "FX-TURBNOISE-060": ("Size 4, blending mode hue, opacity 60: the noise is grey, so the card is "
                         "drained towards its own grey at its own lightness, 60 per cent of the way "
                         "(the W3C's Hue, our basis).", case(blend="hue", opacity=60, **FINE), [0]),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = T.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"].update(
        {k: setting_json(c[k]) for k in STARTS})
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = T.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = T.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    for fx, (says, c, frames) in REGENERATED.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        print(f"{fx}: regenerated (D-731)")
    (OUT / "expected_turbulent_more.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((OUT / "expected_turbulent_noise.json").read_text(encoding="utf-8"))["cases"]
    types_dir = OUT.parent / "turbulent_noise_types"
    old_types = json.loads((types_dir / "expected_turbulent_noise_types.json")
                           .read_text(encoding="utf-8"))["cases"]
    near = lambda a, b: all(abs(x - y) <= 1e-9 for p, q in zip(a, b) for x, y in zip(p, q))  # noqa: E731
    drawn = T.plain(case())
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]  # noqa: E731
    grey = lambda f, i: enc(f[i])[0]  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731

    # The defaults guard: every D-445 and D-567 case, with the new settings as added, is its
    # pinned frame to the bit.
    for fx, (_, oc, frames) in {**T.CASES, **TT.CASES}.items():
        pinned = (old if fx in old else old_types)[fx]["frames"]
        nc = {**oc, **STARTS}
        for f in frames:
            assert render(nc, f) == pinned[str(f)], fx
    # The general field with nothing moved is D-567's within rounding, every type and noise.
    g = {"middle": (8, 5), "offset": (0, 0), "sub_offset": (0, 0), "size": (4, 4),
         "perspective": False, "rotation": 0, "sub_rotation": 0, "influence": 50,
         "sub_scaling": 50, "center": False}
    for kind in TT.FRACTAL_TYPES:
        for noise in TT.NOISE_TYPES:
            for x, y in ((0.5, 0.5), (7.5, 3.5), (13.5, 8.5)):
                a = F_more(2, x, y, 0.3, 5, kind, noise, g)
                b = TT.F(2, x / 4, y / 4, 0.3, 5, kind, noise)
                assert abs(a - b) < 1e-9, (kind, noise)

    two = old["FX-TURBNOISE-002"]["frames"]["0"]
    assert c["FX-TURBNOISE-055"]["0"] == two
    assert c["FX-TURBNOISE-033"]["0"] == old["FX-TURBNOISE-009"]["frames"]["0"]
    v34 = [grey(c["FX-TURBNOISE-034"]["0"], i) for i in shown]
    assert all(0 < v < 1 for v in v34)
    nine = old["FX-TURBNOISE-009"]["frames"]["0"]
    assert c["FX-TURBNOISE-035"]["0"] != nine and c["FX-TURBNOISE-034"]["0"] != nine
    assert any(grey(nine, i) in (0.0, 1.0) and 0 < grey(c["FX-TURBNOISE-035"]["0"], i) < 1
               for i in shown)
    # Rotation 90 round (8, 5): the pixel at p reads the plain field at c + R(-90)(p - c).
    turned = c["FX-TURBNOISE-036"]["0"]
    assert turned != two
    for i in shown:
        x, y = i % W + 0.5, i // W + 0.5
        qx, qy = 8 + (y - 5), 5 - (x - 8)
        cell = 4 / T.CELL  # D-772
        want = over(0.5 + 0.5 * T.CONTRAST * TT.F(0, qx / cell, qy / cell, 0, 6, "basic",
                                                  "smooth"), "allow_hdr")
        assert abs(grey(turned, i) - want) < 1e-9
    thirty_seven = c["FX-TURBNOISE-037"]
    assert thirty_seven["0"] == two and near(thirty_seven["4"], turned)
    assert thirty_seven["2"] not in (two, turned)
    assert c["FX-TURBNOISE-038"]["0"] == two
    slid = render(case(offset=(6, -4), **FINE), 0)
    assert c["FX-TURBNOISE-039"]["0"] != slid
    assert near(render(case(offset=(6, -4), perspective_offset="on", complexity=1, **FINE), 0),
                render(case(offset=(6, -4), complexity=1, **FINE), 0))
    assert near(c["FX-TURBNOISE-040"]["0"], two)
    assert near(c["FX-TURBNOISE-042"]["0"], old["FX-TURBNOISE-007"]["frames"]["0"])
    looks = [str(c[f"FX-TURBNOISE-0{k}"]["0"]) for k in (36, 39, 41, 43, 44, 45, 46, 47, 48)]
    assert len(set(looks + [str(two)])) == 10
    assert c["FX-TURBNOISE-048"]["0"] != old_types["FX-TURBTYPE-010"]["frames"]["0"]
    # The blends: darken never lightens, lighten never darkens, difference on black is the grey.
    dk, lt, df = (c[f"FX-TURBNOISE-05{k}"]["0"] for k in (1, 2, 3))
    assert all(dk[i][ch] <= drawn[i][ch] + 1e-12 for i in shown for ch in range(3))
    assert all(lt[i][ch] >= drawn[i][ch] - 1e-12 for i in shown for ch in range(3))
    for i in [at(x, y) for x in (9, 10) for y in (3, 4)]:
        assert abs(df[i][0] - two[i][0]) < 1e-12
    assert len({str(c[f"FX-TURBNOISE-0{k}"]["0"]) for k in range(49, 54)}) == 5
    moved_ = c["FX-TURBNOISE-054"]["0"]
    for y in range(H):
        assert moved_[at(3, y):at(0, y + 1)] == turned[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        base = T.plain(case(shift=3 if fx == "FX-TURBNOISE-054" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(0 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
    print("checked")


if __name__ == "__main__":
    main()
