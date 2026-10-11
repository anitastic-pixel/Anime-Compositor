"""Remove Grain, worked a second way.

D-455 adds `core.remove_grain`, modelled on After Effects' Remove Grain (Noise & Grain), which
"differentiates fine image detail from grain and noise" and removes the grain while keeping the
detail; its Noise Reduction sets how much is removed, Passes "control the maximum noise radius
that can be detected", Multichannel "degrains all channels of a color image together" and Single
Channel "degrains each channel independently", and its Unsharp Mask can "return subtle edge
detail that the degraining removed" (Adobe's After Effects help, Remove Grain). Adobe publishes
no formula, ranges or starting values, so the rule, ranges and starting values below are this
program's own, and nothing is ported. It is three published pieces put together:

1. The measure. The layer's own picture as it reaches the effect, measured exactly as Match Grain
   measures its source (D-454, tools/match_grain_reference.py): per channel c of the straight
   colour through the sRGB curve, held in 0 to 1, Immerkaer's estimate sigma_c (J. Immerkaer,
   "Fast Noise Variance Estimation", CVIU 64(2):300-302, 1996).
2. The denoise: a bilateral filter (C. Tomasi and R. Manduchi, "Bilateral Filtering for Gray and
   Color Images", ICCV 1998) on the encoded straight colour, its range spread twice the measured
   noise, the ratio M. Zhang and B. Gunturk found best for Gaussian noise ("Multiresolution
   bilateral filtering for image denoising", IEEE Trans. Image Processing 17(12):2324-2333,
   2008), times Noise Reduction n:
       h_c = 2 n sigma_c.
   Pass k, for k = 1 to Passes, works on the last pass's output: each pixel that shows becomes
   the weighted mean of the taps within 2k pixels of it (dx^2 + dy^2 <= 4k^2) that lie on the
   layer and show, tap j weighed by its covering a_j, a bell on its distance of spread k,
   exp(-(dx^2 + dy^2) / (2 k^2)), and a bell on how far its colour is from the pixel's own:
   - Multichannel: one weight for the three channels, exp(-D^2 / (2 h^2)), D^2 the mean over the
     channels of the squared difference and h = 2 n sqrt(mean of sigma_c^2);
   - Single Channel: each channel its own, exp(-d_c^2 / (2 h_c^2)); a channel with h_c = 0 keeps
     its value.
   The result goes back through the sRGB curve, times the pixel's covering, which is kept. With n
   0 or no noise measured (h 0) the denoise is skipped and the pixels are untouched.
3. The Unsharp Mask: D-147's sharpen with D-317's threshold (tools/unsharp_threshold_reference.py),
   Amount, Radius and Threshold, on the denoised picture. Amount 0 changes nothing.

Settings: `noise_reduction` 0 to 3 (1), `passes` 1 to 4 (1, its whole part), `mode`
"multichannel" (the default) or "single_channel", `unsharp_amount` 0 to 500 (0), `unsharp_radius`
0 to 100 (1), `unsharp_threshold` 0 to 255 (0); the numbers keyable.

Known differences, each in the row: a draft measures and denoises the smaller picture with the
same disc (the Unsharp Mask's radius is scaled, as Sharpen's); values outside 0 to 1 are held to
it where the denoise runs. Not built, logged as gaps: View Mode and Preview Region (always the
final output), Channel Noise Reduction, Fine Tuning (Chroma Suppression, Texture, Noise Size Bias,
Clean Solid Areas), Temporal Filtering, the Sampling group and its sample boxes; Blend with
Original is the effect's own Mix.

**This file never runs the build's code path.** It works in double precision on lists with
numpy, from the drawings' exact 8-bit values.

Every case is a composition 16 by 10, five frames, in `Fixtures/remove_grain/`, holding one layer
the same size whose drawing the case names: `grainy`, a warm left half and a blue right half,
each with grain of a few levels per channel, a hard edge between them; `red_only`, the same with
grain in red alone; `holes`, `grainy` with one empty pixel and one half-covered column; `heavy`,
`grainy` with three times the grain; `flat`, one colour. The expected pixels are in
`Fixtures/remove_grain/expected_remove_grain.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/remove_grain_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import recolor_reference as R  # noqa: E402
import smooth_reference as S  # noqa: E402
import unsharp_threshold_reference as U  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "remove_grain"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = R.W, R.H, 5
MASK = np.array([[1, -2, 1], [-2, 4, -2], [1, -2, 1]], dtype=float)
RANGES = {"noise_reduction": (0, 3), "passes": (1, 4), "unsharp_amount": (0, 500),
          "unsharp_radius": (0, 100), "unsharp_threshold": (0, 255)}
MODES = ("multichannel", "single_channel")
NAMES = ("noise_reduction", "passes", "mode", "unsharp_amount", "unsharp_radius",
         "unsharp_threshold")
MARGIN = 0.05  # levels of 255 between a threshold and any difference it meets


# --- the drawings ---------------------------------------------------------------------------

def d(x, y):
    """A fixed grain step in -2..2 for pixel (x, y)."""
    return ((x * 7 + y * 13 + x * y * 3) % 5) - 2


def grainy(amount, holes=False):
    def at(x, y):
        if holes and (x, y) == (4, 3):
            return (0, 0, 0, 0)
        base = (215, 185, 150) if x < 8 else (35, 55, 115)
        return (base[0] + amount[0] * d(x, y), base[1] + amount[1] * d(x, y),
                base[2] + amount[2] * d(y, x), 128 if holes and x == 11 else 255)
    return [[at(x, y) for x in range(W)] for y in range(H)]


DRAWINGS = {
    "grainy": grainy((4, 3, 2)),
    "red_only": grainy((4, 0, 0)),
    "holes": grainy((4, 3, 2), holes=True),
    "heavy": grainy((12, 9, 6)),
    "flat": [[(90, 140, 60, 255)] * W for _ in range(H)],
}


def pixels(name):
    return [R.working(p) for row in DRAWINGS[name] for p in row]


# --- the rule -------------------------------------------------------------------------------

def encoded(px):
    """Each pixel's straight colour through the sRGB curve, held in 0 to 1, or None."""
    return [[S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]] if p[3] > 0
            else None for p in px]


def levels(px):
    """Immerkaer's sigma per channel of a W by H picture."""
    e = encoded(px)
    out = []
    for ch in range(3):
        total, n = 0.0, 0
        for y in range(H - 2):
            for x in range(W - 2):
                win = [e[(y + j) * W + x + i] for j in range(3) for i in range(3)]
                if all(v is not None for v in win):
                    img = np.array([v[ch] for v in win]).reshape(3, 3)
                    total += abs(float((img * MASK).sum()))
                    n += 1
        out.append(math.sqrt(math.pi / 2) / (6 * n) * total if n else 0.0)
    return out


def denoise(px, n, passes, mode):
    sig = levels(px)
    if mode == "multichannel":
        hs = [2 * n * math.sqrt(sum(s * s for s in sig) / 3)] * 3
    else:
        hs = [2 * n * s for s in sig]
    if max(hs) == 0:
        return [list(p) for p in px]
    e = encoded(px)
    for k in range(1, passes + 1):
        r = 2 * k
        taps = [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
                if dx * dx + dy * dy <= r * r]
        nxt = []
        for i, own in enumerate(e):
            if own is None:
                nxt.append(None)
                continue
            x, y = i % W, i // W
            total, weight = [0.0] * 3, [0.0] * 3
            for dx, dy in taps:
                tx, ty = x + dx, y + dy
                if not (0 <= tx < W and 0 <= ty < H) or e[ty * W + tx] is None:
                    continue
                t = e[ty * W + tx]
                g = math.exp(-(dx * dx + dy * dy) / (2 * k * k)) * px[ty * W + tx][3]
                if mode == "multichannel":
                    dd = sum((t[c] - own[c]) ** 2 for c in range(3)) / 3
                    wt = [g * math.exp(-dd / (2 * hs[0] ** 2))] * 3
                else:
                    wt = [g * math.exp(-(t[c] - own[c]) ** 2 / (2 * hs[c] ** 2)) if hs[c] > 0
                          else 0.0 for c in range(3)]
                for c in range(3):
                    total[c] += wt[c] * t[c]
                    weight[c] += wt[c]
            nxt.append([total[c] / weight[c] if hs[c] > 0 else own[c] for c in range(3)])
        e = nxt
    return [[srgb_to_linear(v) * p[3] for v in q] + [p[3]] if q is not None else list(p)
            for p, q in zip(px, e)]


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def denoised(c, frame_no):
    return denoise(pixels(c["drawing"]), held(c, "noise_reduction", frame_no),
                   math.floor(held(c, "passes", frame_no)), c["mode"])


def render(c, frame_no):
    out = U.unsharp(denoised(c, frame_no), held(c, "unsharp_amount", frame_no),
                    held(c, "unsharp_radius", frame_no), held(c, "unsharp_threshold", frame_no))
    return R.frame(out, 0)


# --- the cases ------------------------------------------------------------------------------

def case(drawing="grainy", noise_reduction=1, passes=1, mode="multichannel", unsharp_amount=0,
         unsharp_radius=1, unsharp_threshold=0):
    return {"drawing": drawing, "noise_reduction": noise_reduction, "passes": passes,
            "mode": mode, "unsharp_amount": unsharp_amount, "unsharp_radius": unsharp_radius,
            "unsharp_threshold": unsharp_threshold}


CASES = {
    "FX-REMOVEGRAIN-001": ("The settings as added (Noise Reduction 1, one pass, Multichannel, no "
                           "Unsharp Mask) on `grainy`: the grain in each half is smoothed, the "
                           "edge between the halves stays hard.", case(), (0,)),
    "FX-REMOVEGRAIN-002": ("Noise Reduction 0: the drawing as it is.",
                           case(noise_reduction=0), (0,)),
    "FX-REMOVEGRAIN-003": ("Noise Reduction 2: smoother than FX-REMOVEGRAIN-001.",
                           case(noise_reduction=2), (0,)),
    "FX-REMOVEGRAIN-004": ("Noise Reduction 3: smoother still, the edge still hard.",
                           case(noise_reduction=3), (0,)),
    "FX-REMOVEGRAIN-005": ("Passes 2: a second, wider pass, smoother than FX-REMOVEGRAIN-001.",
                           case(passes=2), (0,)),
    "FX-REMOVEGRAIN-006": ("Passes 4: four passes, the widest within 8 pixels.",
                           case(passes=4), (0,)),
    "FX-REMOVEGRAIN-007": ("Single Channel: each channel smoothed by its own noise.",
                           case(mode="single_channel"), (0,)),
    "FX-REMOVEGRAIN-008": ("`red_only`, Single Channel: red smoothed, green and blue, which have "
                           "no grain, kept exactly.", case("red_only", mode="single_channel"),
                           (0,)),
    "FX-REMOVEGRAIN-009": ("Unsharp Mask amount 100, radius 1: FX-REMOVEGRAIN-001 sharpened, "
                           "the edge crisper.", case(unsharp_amount=100), (0,)),
    "FX-REMOVEGRAIN-010": ("Unsharp Mask amount 200, radius 2, threshold 20: only the edge is "
                           "sharpened; the smoothed halves away from it keep "
                           "FX-REMOVEGRAIN-001's values.",
                           case(unsharp_amount=200, unsharp_radius=2, unsharp_threshold=20),
                           (0,)),
    "FX-REMOVEGRAIN-011": ("`holes`, with one empty pixel and a half-covered column: the empty "
                           "pixel stays empty and the column keeps its covering.",
                           case("holes"), (0,)),
    "FX-REMOVEGRAIN-012": ("`flat`, one colour: no noise measured, the drawing as it is.",
                           case("flat", noise_reduction=3, passes=4), (0, 2)),
    "FX-REMOVEGRAIN-013": ("Noise Reduction keyed from 0 at frame 0 to 3 at frame 4: frame 0 the "
                           "drawing, frame 4 FX-REMOVEGRAIN-004.",
                           case(noise_reduction=keyed((0, 0), (4, 3))), (0, 2, 4)),
    "FX-REMOVEGRAIN-014": ("Passes 2.6: its whole part, FX-REMOVEGRAIN-005.",
                           case(passes=2.6), (0,)),
    "FX-REMOVEGRAIN-015": ("`heavy`, three times the grain, as added: the noise measured is "
                           "larger, so the same setting smooths harder.", case("heavy"), (0,)),
    "FX-REMOVEGRAIN-016": ("Noise Reduction 0 with Unsharp Mask amount 100: no denoise, the "
                           "drawing sharpened as Sharpen would.",
                           case(noise_reduction=0, unsharp_amount=100), (0,)),
}

INVALID = {
    "FX-REMOVEGRAIN-017": ("Noise Reduction 3.5, above 3.", case(noise_reduction=3.5)),
    "FX-REMOVEGRAIN-018": ("Passes 0, below 1.", case(passes=0)),
    "FX-REMOVEGRAIN-019": ("Mode \"both\", not one of its two words.", case(mode="both")),
    "FX-REMOVEGRAIN-020": ("Unsharp Mask amount 600, above 500.", case(unsharp_amount=600)),
    "FX-REMOVEGRAIN-021": ("Unsharp Mask threshold keyed to 300 at frame 4.",
                           case(unsharp_threshold=keyed((0, 10), (4, 300)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    layer = L.raster("holder", f"asset-{c['drawing']}", out_frame=FRAMES)
    params = {k: (c[k] if k == "mode" else setting_json(c[k])) for k in NAMES}
    layer["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.remove_grain",
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
    expected["levels"] = {k: levels(pixels(k)) for k in DRAWINGS}
    (OUT / "expected_remove_grain.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    lv = expected["levels"]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    plain = {k: R.frame(pixels(k), 0) for k in DRAWINGS}
    # The grain left: the summed squared step between neighbours inside each half, edge apart.
    inside = [(x, y) for y in range(H) for x in range(W - 1) if x != 7]

    def rough(f, ch=None):
        chans = range(3) if ch is None else (ch,)
        return sum((enc(f[at(x + 1, y)])[k] - enc(f[at(x, y)])[k]) ** 2
                   for x, y in inside for k in chans)

    def edge(f):
        return sum(abs(enc(f[at(8, y)])[0] - enc(f[at(7, y)])[0]) for y in range(H))

    # The measure: Gaussian grain of spread 0.05 on a flat grey reads within 5 per cent.
    rng = np.random.default_rng(1)
    noisy = np.clip(0.5 + rng.normal(0, 0.05, (H, W)), 0, 1)
    lin = [[srgb_to_linear(v)] * 3 + [1.0] for v in noisy.flatten()]
    assert all(0.03 < s < 0.07 for s in levels(lin)), levels(lin)
    assert lv["grainy"][0] > lv["grainy"][1] > lv["grainy"][2] > 0
    assert lv["flat"] == [0.0, 0.0, 0.0]
    assert lv["heavy"][0] > 2 * lv["grainy"][0]

    g = plain["grainy"]
    one = c["FX-REMOVEGRAIN-001"]["0"]
    assert rough(one) < 0.5 * rough(g) and edge(one) > 0.95 * edge(g)
    assert same(c["FX-REMOVEGRAIN-002"]["0"], g, 1e-12)
    three, four = c["FX-REMOVEGRAIN-003"]["0"], c["FX-REMOVEGRAIN-004"]["0"]
    assert rough(four) < rough(three) < rough(one) and edge(four) > 0.95 * edge(g)
    five, six = c["FX-REMOVEGRAIN-005"]["0"], c["FX-REMOVEGRAIN-006"]["0"]
    assert rough(six) < rough(five) < rough(one)
    assert not same(c["FX-REMOVEGRAIN-007"]["0"], one, 1e-6)
    eight, ro = c["FX-REMOVEGRAIN-008"]["0"], plain["red_only"]
    assert all(abs(eight[i][ch] - ro[i][ch]) < 1e-12 for i in range(W * H) for ch in (1, 2, 3))
    assert rough(eight, 0) < 0.5 * rough(ro, 0)
    nine = c["FX-REMOVEGRAIN-009"]["0"]
    assert edge(nine) > edge(one)
    ten = c["FX-REMOVEGRAIN-010"]["0"]
    far = [at(x, y) for x in (0, 1, 2, 3, 12, 13, 14, 15) for y in range(H)]
    assert all(ten[i] == one[i] for i in far) and edge(ten) > edge(one)
    # No threshold lies within MARGIN levels of a difference it is compared with.
    for fx, (_, cc, frames) in CASES.items():
        for f in frames:
            t = held(cc, "unsharp_threshold", f)
            if t == 0 or held(cc, "unsharp_amount", f) == 0:
                continue
            for dd in U.differences(denoised(cc, f), held(cc, "unsharp_radius", f)):
                if dd:
                    for u, v in zip(*dd):
                        assert abs(abs(u - v) * 255 - t) >= MARGIN, (fx, f, u, v)
    ho, hp = c["FX-REMOVEGRAIN-011"]["0"], plain["holes"]
    assert ho[at(4, 3)] == [0.0] * 4
    assert all(ho[at(11, y)][3] == hp[at(11, y)][3] for y in range(H))
    assert not same(ho, hp, 1e-6)
    assert all(same(f, plain["flat"], 1e-12) for f in c["FX-REMOVEGRAIN-012"].values())
    k13 = c["FX-REMOVEGRAIN-013"]
    assert same(k13["0"], g, 1e-12) and k13["4"] == four
    assert rough(four) < rough(k13["2"]) < rough(g)
    assert c["FX-REMOVEGRAIN-014"]["0"] == five
    hv = c["FX-REMOVEGRAIN-015"]["0"]
    assert rough(hv) < 0.5 * rough(plain["heavy"])
    sharp = R.frame(U.unsharp(pixels("grainy"), 100, 1, 0), 0)
    assert c["FX-REMOVEGRAIN-016"]["0"] == [[float(v) for v in p] for p in sharp]
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), name
    print("checked")


if __name__ == "__main__":
    main()
