"""Deband, worked a second way.

D-456 adds `core.deband`, an effect of this program's own named Deband, modelled on the control
list of CelFX's P_Deband (P SOFTHOUSE, docs.psoft.co.jp/cfx100w/en/latest/reference/
CFX_Deband.html), which "complements color bands that occur in subtle gradients by applying
smoothing". That page names the controls (View: Normal, Edge Only, Selected Edge; Edge Threshold;
Edge Softness; Blur Amount; Edge Blur Reduction; Diffusion Amount; Edge Diffusion Reduction;
Diffusion Reseed Interval; Noise Amount; Edge Noise Reduction; Noise Reseed Interval, "0: never
changes, 1: changes every frame") and gives no formula, ranges or starting values. The rule,
ranges and starting values below are this program's own, and nothing is ported:

1. The edges. Each pixel that shows has its straight colour through the sRGB curve, held in 0 to
   1, e. Its step g, in levels of 255, is the largest |e_n - e| over the three channels and the
   four neighbours n (left, right, up, down) that lie on the layer and show. Its edge weight E is
   0 where g <= Edge Threshold T, 1 where g >= T + Edge Softness S, and (g - T) / S between; with
   S 0 a hard step, 1 only where g > T.
2. The blur, only across flat ground and never through an edge: a normalised blur, across then
   down, with document 21's Gaussian taps k (sigma Blur Amount / 3, out to ceil(3 sigma)). Across,
   the pixel gathers from each tap t in its row a_t (1 - E_t) k(dx) times the product of (1 - E)
   over the pixels strictly between it and the tap, a the covering, so an edge lends nothing and
   stops what lies beyond it (in the spirit of Gastal and Oliveira's edge-stopping separable
   filters, "Domain Transform for Edge-Aware Image and Video Processing", SIGGRAPH 2011; the
   weights here are this program's own); the sums of e times those weights and of the weights.
   Down, the same sums from each tap q in its column, times k(dy) and the product of (1 - E)
   over the pixels strictly between. The blurred colour is the first sum over the second; where
   the second is 0, e itself. The pixel takes
       m = e + (blurred - e) (1 - E Edge Blur Reduction / 100).
3. The diffusion: each pixel reads m at a tap round(D s u_x), round(D s u_y) pixels away (round
   half away from 0), D the Diffusion Amount, s = 1 - E Edge Diffusion Reduction / 100, u_x and
   u_y numbers in -1..1 (D-119's hash, `noise_reference.u`, seed 0, at the pixel's place in the
   layer, depth the diffusion's step, channels 0 and 1); a tap off the layer or empty reads the
   pixel's own m; a tap on an edge lends its m only by (1 - E_tap), the rest the pixel's own:
       d = m_tap (1 - E_tap) + m E_tap.
4. The noise: shaped dither, triangular (the mean of two evenly spread numbers, as Lipshitz, Wannamaker and
   Vanderkooy, "Quantization and Dither: A Theoretical Survey", JAES 40(5), 1992, recommend for
   hiding a quantiser's steps), one number for the three channels:
       out = d + (u_2 + u_3) / 2 * Noise Amount / 255 * (1 - E Edge Noise Reduction / 100),
   u_2 and u_3 the hash's channels 2 and 3 at depth the noise's step; held in 0 to 1.
   The steps: with a reseed interval's whole part n of 1 or more, floor(frame / n) (P0-23's held
   step, as Turbulent Displace's New Seed Every); with 0, always 0, so it never changes.
5. Back through the curve, times the covering, which is kept. View Edge Only shows E as grey (out
   = E for all three); Selected Edge shows the result with the edges laid over it in red,
   out (1 - E) + (1, 0, 0) E.

Settings: `view` "normal" (the default), "edge_only" or "selected_edge"; `edge_threshold` 0 to
255 (4); `edge_softness` 0 to 255 (2); `blur_amount` 0 to 100 (8); `edge_blur_reduction` 0 to 100
(100); `diffusion_amount` 0 to 20 (1); `edge_diffusion_reduction` 0 to 100 (100);
`diffusion_reseed_interval` 0 to 100 (1, its whole part); `noise_amount` 0 to 32 (1);
`edge_noise_reduction` 0 to 100 (100); `noise_reseed_interval` 0 to 100 (1, its whole part); the
numbers keyable. A draft scales Blur Amount and Diffusion Amount; the thresholds are levels and
are not scaled.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawings' exact 8-bit values.

Every case is a composition 16 by 10, five frames, in `Fixtures/deband/`, holding one layer the
same size whose drawing the case names: `banded`, a gradient in bands (red up 3 levels every 3
columns, green up 3 every 4, blue up 2 every 2 rows) on its left twelve columns and a dark block
on its right four, a hard edge between; `holes`, `banded` with one empty pixel and one
half-covered column; `flat`, one colour. The expected pixels are in
`Fixtures/deband/expected_deband.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/deband_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from noise_reference import u as hashed  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import recolor_reference as R  # noqa: E402
import smooth_reference as S  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "deband"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = R.W, R.H, 5
RANGES = {"edge_threshold": (0, 255), "edge_softness": (0, 255), "blur_amount": (0, 100),
          "edge_blur_reduction": (0, 100), "diffusion_amount": (0, 20),
          "edge_diffusion_reduction": (0, 100), "diffusion_reseed_interval": (0, 100),
          "noise_amount": (0, 32), "edge_noise_reduction": (0, 100),
          "noise_reseed_interval": (0, 100)}
VIEWS = ("normal", "edge_only", "selected_edge")
NAMES = ("view",) + tuple(RANGES)
MARGIN = 0.05  # levels of 255 between a hard threshold and any step it meets
ROUNDING = 1e-3  # between a diffusion offset and a half pixel


# --- the drawings ---------------------------------------------------------------------------

def banded(holes=False):
    def at(x, y):
        if holes and (x, y) == (4, 3):
            return (0, 0, 0, 0)
        a = 128 if holes and x == 6 else 255
        if x >= 12:
            return (40, 30, 70, a)
        return (100 + 3 * (x // 3), 120 + 3 * (x // 4), 150 + 2 * (y // 2), a)
    return [[at(x, y) for x in range(W)] for y in range(H)]


DRAWINGS = {
    "banded": banded(),
    "holes": banded(holes=True),
    "flat": [[(90, 140, 60, 255)] * W for _ in range(H)],
}


def pixels(name):
    return [R.working(p) for row in DRAWINGS[name] for p in row]


# --- the rule -------------------------------------------------------------------------------

def encoded(px):
    return [[S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]] if p[3] > 0
            else None for p in px]


def steps(px):
    """Each pixel's step g in levels, None where it does not show."""
    e = encoded(px)
    out = []
    for i, own in enumerate(e):
        if own is None:
            out.append(None)
            continue
        x, y = i % W, i // W
        g = 0.0
        for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            tx, ty = x + dx, y + dy
            if 0 <= tx < W and 0 <= ty < H and e[ty * W + tx] is not None:
                t = e[ty * W + tx]
                g = max(g, max(abs(t[c] - own[c]) * 255 for c in range(3)))
        out.append(g)
    return out


def edge(g, t, s):
    if s > 0:
        return min(1.0, max(0.0, (g - t) / s))
    return 1.0 if g > t else 0.0


def weights(sigma):
    if sigma <= 0:
        return [1.0]
    r = math.ceil(3 * sigma)
    w = [math.exp(-((i - r) ** 2) / (2 * sigma * sigma)) for i in range(2 * r + 1)]
    return [v / sum(w) for v in w]


def gather(values, E, k, axis):
    """Each pixel's sums along `axis` (0 across, 1 down) of `values` (lists of numbers per
    pixel) from the taps out to the kernel's reach, each times its tap of `k` and the product of
    (1 - E) over the pixels strictly between."""
    r = len(k) // 2
    out = []
    for i in range(W * H):
        x, y = i % W, i // W
        total = [k[r] * v for v in values[i]]
        for sign in (-1, 1):
            through = 1.0
            for d in range(1, r + 1):
                tx, ty = (x + sign * d, y) if axis == 0 else (x, y + sign * d)
                if not (0 <= tx < W and 0 <= ty < H):
                    break
                j = ty * W + tx
                total = [t + k[r + d] * through * v for t, v in zip(total, values[j])]
                through *= 1 - E[j]
        out.append(total)
    return out


def step(interval, frame_no):
    n = math.floor(interval)
    return frame_no // n if n >= 1 else 0


def away(v):
    """Round half away from 0."""
    return int(math.copysign(math.floor(abs(v) + 0.5), v))


def deband(px, s, frame_no):
    """`s` the settings held at the frame. Returns the pixels and, for the margins, each pixel's
    step and diffusion offsets before rounding."""
    e = encoded(px)
    g = steps(px)
    E = [edge(v, s["edge_threshold"], s["edge_softness"]) if v is not None else 0.0 for v in g]
    sigma = s["blur_amount"] / 3
    k = weights(sigma)
    if len(k) > 1:
        wt = [p[3] * (1 - E[i]) if e[i] is not None else 0.0 for i, p in enumerate(px)]
        lent = [[wt[i] * v for v in e[i]] + [wt[i]] if e[i] is not None else [0.0] * 4
                for i in range(W * H)]
        b = gather(gather(lent, E, k, 0), E, k, 1)
    m = []
    for i, own in enumerate(e):
        if own is None:
            m.append(None)
            continue
        if len(k) > 1 and b[i][3] > 0:
            f = 1 - E[i] * s["edge_blur_reduction"] / 100
            m.append([own[c] + (b[i][c] / b[i][3] - own[c]) * f for c in range(3)])
        else:
            m.append(list(own))
    fd = step(s["diffusion_reseed_interval"], frame_no)
    fn = step(s["noise_reseed_interval"], frame_no)
    out, offsets = [], []
    for i, own in enumerate(m):
        if own is None:
            out.append([0.0, 0.0, 0.0, 0.0])
            continue
        x, y = i % W, i // W
        d = own
        if s["diffusion_amount"] > 0:
            k = s["diffusion_amount"] * (1 - E[i] * s["edge_diffusion_reduction"] / 100)
            vx, vy = k * hashed(0, x, y, fd, 0), k * hashed(0, x, y, fd, 1)
            offsets += [vx, vy]
            tx, ty = x + away(vx), y + away(vy)
            if 0 <= tx < W and 0 <= ty < H and m[ty * W + tx] is not None:
                j = ty * W + tx
                d = [m[j][c] * (1 - E[j]) + own[c] * E[j] for c in range(3)]
        t = (hashed(0, x, y, fn, 2) + hashed(0, x, y, fn, 3)) / 2
        n = t * s["noise_amount"] / 255 * (1 - E[i] * s["edge_noise_reduction"] / 100)
        o = [min(1.0, max(0.0, v + n)) for v in d]
        if s["view"] == "edge_only":
            o = [E[i]] * 3
        elif s["view"] == "selected_edge":
            o = [v * (1 - E[i]) + r * E[i] for v, r in zip(o, (1.0, 0.0, 0.0))]
        a = px[i][3]
        out.append([srgb_to_linear(v) * a for v in o] + [a])
    return out, g, offsets


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def settings(c, frame_no):
    s = {k: held(c, k, frame_no) for k in RANGES}
    s["view"] = c["view"]
    return s


def render(c, frame_no):
    return R.frame(deband(pixels(c["drawing"]), settings(c, frame_no), frame_no)[0], 0)


# --- the cases ------------------------------------------------------------------------------

OFF = {"blur_amount": 0, "diffusion_amount": 0, "noise_amount": 0}


def case(drawing="banded", view="normal", edge_threshold=4, edge_softness=2, blur_amount=8,
         edge_blur_reduction=100, diffusion_amount=1, edge_diffusion_reduction=100,
         diffusion_reseed_interval=1, noise_amount=1, edge_noise_reduction=100,
         noise_reseed_interval=1):
    return dict(drawing=drawing, view=view, edge_threshold=edge_threshold,
                edge_softness=edge_softness, blur_amount=blur_amount,
                edge_blur_reduction=edge_blur_reduction, diffusion_amount=diffusion_amount,
                edge_diffusion_reduction=edge_diffusion_reduction,
                diffusion_reseed_interval=diffusion_reseed_interval, noise_amount=noise_amount,
                edge_noise_reduction=edge_noise_reduction,
                noise_reseed_interval=noise_reseed_interval)


def only(**k):
    return case(**{**OFF, **k})


CASES = {
    "FX-DEBAND-001": ("The settings as added (threshold 4, softness 2, blur 8, diffusion 1, noise "
                      "1, every reduction 100, reseeding every frame) on `banded`: the bands "
                      "smoothed, a little dither, the edge to the dark block kept; frame 1 has "
                      "fresh dither.", case(), (0, 1)),
    "FX-DEBAND-002": ("Blur, diffusion and noise all 0: the drawing as it is.", only(), (0,)),
    "FX-DEBAND-003": ("Blur 8 alone: the bands smoothed, the edge pixels exactly as drawn.",
                      only(blur_amount=8), (0,)),
    "FX-DEBAND-004": ("Blur 8 with Edge Blur Reduction 0: the edge pixels take the flat ground's "
                      "blur too.", only(blur_amount=8, edge_blur_reduction=0), (0,)),
    "FX-DEBAND-005": ("Threshold 0, softness 0, blur 8: every pixel with any step is an edge, and "
                      "a flat pixel meets only its own colour before an edge stops the blur: the "
                      "drawing as it is.",
                      only(blur_amount=8, edge_threshold=0, edge_softness=0), (0,)),
    "FX-DEBAND-006": ("Threshold 255, blur 8: nothing is an edge, the whole layer blurred, the "
                      "dark block's edge too.", only(blur_amount=8, edge_threshold=255), (0,)),
    "FX-DEBAND-007": ("Threshold 2, softness 2, blur 8: the band steps of 3 levels are half edges, "
                      "half held back.", only(blur_amount=8, edge_threshold=2), (0,)),
    "FX-DEBAND-008": ("Diffusion 3 alone: pixels read from up to 3 away inside the flat ground; "
                      "the edge pixels kept.", only(diffusion_amount=3), (0,)),
    "FX-DEBAND-009": ("Diffusion 3, Edge Diffusion Reduction 0: the edge pixels read from flat "
                      "ground too.", only(diffusion_amount=3, edge_diffusion_reduction=0), (0,)),
    "FX-DEBAND-010": ("Noise 4 alone: dither on the flat ground, none on the edge pixels.",
                      only(noise_amount=4), (0,)),
    "FX-DEBAND-011": ("Noise 4, Edge Noise Reduction 0: dither everywhere.",
                      only(noise_amount=4, edge_noise_reduction=0), (0,)),
    "FX-DEBAND-012": ("Both reseed intervals 0: the dither and diffusion never change, frame 3 "
                      "as frame 0.", case(diffusion_reseed_interval=0, noise_reseed_interval=0),
                      (0, 3)),
    "FX-DEBAND-013": ("Both reseed intervals 2: frames 0 and 1 the same, frame 2 changed.",
                      case(diffusion_reseed_interval=2, noise_reseed_interval=2), (0, 1, 2)),
    "FX-DEBAND-014": ("View Edge Only, threshold 2: the edge weight as grey, white on the edge to "
                      "the block, mid grey on the 3-level band steps, black on the flat ground.",
                      case(view="edge_only", edge_threshold=2), (0,)),
    "FX-DEBAND-015": ("View Selected Edge: FX-DEBAND-001 with the edges laid over it in red.",
                      case(view="selected_edge"), (0,)),
    "FX-DEBAND-016": ("`holes`, with one empty pixel and a half-covered column: the empty pixel "
                      "stays empty and the column keeps its covering.", case("holes"), (0,)),
    "FX-DEBAND-017": ("`flat`, one colour, as added: no edges, the blur changes nothing, only the "
                      "dither shows.", case("flat"), (0,)),
    "FX-DEBAND-018": ("Blur keyed from 0 at frame 0 to 16 at frame 4, nothing else: frame 0 the "
                      "drawing, then smoother.", only(blur_amount=keyed((0, 0), (4, 16))),
                      (0, 2, 4)),
    "FX-DEBAND-019": ("Both reseed intervals 2.6: their whole part, FX-DEBAND-013.",
                      case(diffusion_reseed_interval=2.6, noise_reseed_interval=2.6), (0, 1, 2)),
}

INVALID = {
    "FX-DEBAND-020": ("View \"edges\", not one of its three words.", case(view="edges")),
    "FX-DEBAND-021": ("Edge Threshold 300, above 255.", case(edge_threshold=300)),
    "FX-DEBAND-022": ("Blur Amount -1, below 0.", case(blur_amount=-1)),
    "FX-DEBAND-023": ("Noise Reseed Interval 101, above 100.", case(noise_reseed_interval=101)),
    "FX-DEBAND-024": ("Noise Amount keyed to 40 at frame 4, above 32.",
                      case(noise_amount=keyed((0, 1), (4, 40)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    layer = L.raster("holder", f"asset-{c['drawing']}", out_frame=FRAMES)
    params = {k: (c[k] if k == "view" else setting_json(c[k])) for k in NAMES}
    layer["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.deband",
                         "enabled": True, "parameters": params}]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(c["drawing"], f"media/{c['drawing']}.png")],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [layer])]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, rows in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in render(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = R.frame(pixels(c["drawing"]), 0)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = R.frame(pixels(c["drawing"]), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_deband.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    plain = {k: R.frame(pixels(k), 0) for k in DRAWINGS}
    b = plain["banded"]
    g = steps(pixels("banded"))
    flat = [at(x, y) for y in range(H) for x in range(12) if g[at(x, y)] <= 4]
    edges = [at(x, y) for y in range(H) for x in (11, 12)]

    def banding(f):
        """The largest step between neighbours across the gradient, in levels."""
        return max(abs(enc(f[at(x + 1, y)])[k] - enc(f[at(x, y)])[k]) * 255
                   for y in range(H) for x in range(10) for k in range(3))

    def hard(f):
        return sum(abs(enc(f[at(12, y)])[0] - enc(f[at(11, y)])[0]) for y in range(H))

    # The margins: no hard threshold meets a step within MARGIN levels, and no diffusion offset
    # lies within ROUNDING of a half pixel.
    for fx, (_, cc, frames) in CASES.items():
        for fr in frames:
            s = settings(cc, fr)
            _, gs, offs = deband(pixels(cc["drawing"]), s, fr)
            if s["edge_softness"] == 0:
                assert all(v is None or v == 0 or abs(v - s["edge_threshold"]) >= MARGIN
                           for v in gs), fx
            assert all(abs(abs(v) % 1 - 0.5) >= ROUNDING for v in offs), fx

    one = c["FX-DEBAND-001"]
    # As added: the edge kept, and within two levels of the blur alone (the dither and the
    # diffusion on top of it).
    assert hard(one["0"]) > 0.95 * hard(b)
    assert max(abs(enc(p)[k] - enc(q)[k]) * 255 for p, q in zip(one["0"], c["FX-DEBAND-003"]["0"])
               for k in range(3)) < 2
    assert not same(one["0"], one["1"], 1e-6)
    assert same(c["FX-DEBAND-002"]["0"], b, 1e-12)
    three = c["FX-DEBAND-003"]["0"]
    assert banding(three) < 0.5 * banding(b)
    assert all(near(three[i], b[i], 1e-12) for i in edges)
    four = c["FX-DEBAND-004"]["0"]
    assert hard(four) < hard(three) and not same(four, three, 1e-6)
    five = c["FX-DEBAND-005"]["0"]
    assert all(near(five[i], b[i], 1e-12) for i in range(W * H) if g[i] > 0)
    assert same(five, b, 1e-12)
    six = c["FX-DEBAND-006"]["0"]
    assert hard(six) < 0.8 * hard(b)
    seven = c["FX-DEBAND-007"]["0"]
    assert not same(seven, three, 1e-6)
    eight = c["FX-DEBAND-008"]["0"]
    assert all(near(eight[i], b[i], 1e-12) for i in edges) and not same(eight, b, 1e-6)
    nine = c["FX-DEBAND-009"]["0"]
    assert not all(near(nine[i], b[i], 1e-6) for i in edges)
    ten = c["FX-DEBAND-010"]["0"]
    assert all(near(ten[i], b[i], 1e-12) for i in edges) and not same(ten, b, 1e-6)
    eleven = c["FX-DEBAND-011"]["0"]
    assert not all(near(eleven[i], b[i], 1e-6) for i in edges)
    t12 = c["FX-DEBAND-012"]
    assert t12["0"] == t12["3"]
    t13 = c["FX-DEBAND-013"]
    assert t13["0"] == t13["1"] and not same(t13["0"], t13["2"], 1e-6)
    eo = c["FX-DEBAND-014"]["0"]
    assert all(near(eo[i][:3], [1.0] * 3, 1e-12) for i in edges)
    assert all(eo[i][:3] == [0.0] * 3 for i in range(W * H) if g[i] <= 2)
    assert any(near(enc(eo[i]), [0.5] * 3, 1e-6) for i in range(W * H))
    se = c["FX-DEBAND-015"]["0"]
    assert all(near(se[i], [1.0, 0.0, 0.0, 1.0], 1e-12) for i in edges)
    assert all(se[i] == one["0"][i] for i in flat)
    ho, hp = c["FX-DEBAND-016"]["0"], plain["holes"]
    assert ho[at(4, 3)] == [0.0] * 4
    assert all(ho[at(6, y)][3] == hp[at(6, y)][3] for y in range(H))
    assert not same(ho, hp, 1e-6)
    fl = c["FX-DEBAND-017"]["0"]
    dev = [abs(enc(p)[k] - enc(q)[k]) * 255 for p, q in zip(fl, plain["flat"]) for k in range(3)]
    assert 0 < max(dev) <= 1 + 1e-9
    k18 = c["FX-DEBAND-018"]
    assert same(k18["0"], b, 1e-12)
    assert banding(k18["4"]) < banding(k18["2"]) < banding(b)
    assert c["FX-DEBAND-019"] == t13
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), name
    print("checked")


if __name__ == "__main__":
    main()
