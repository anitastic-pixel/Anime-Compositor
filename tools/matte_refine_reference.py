"""Matte refinement, worked a second way.

D-352 (EFFECTS.md P0-20) adds an edge-aware matte filter, the guided filter of He, Sun and Tang
("Guided Image Filtering", ECCV 2010 and TPAMI 2013, Algorithm 2 with a colour guide), in one
shared place for the keying effects to come, and three effects, this program's own, modelled on
After Effects' Matte Choker, Refine Hard Matte and Refine Soft Matte: Matte Choker
(`core.matte_choker`), Refine Hard Matte (`core.refine_hard_matte`) and Refine Soft Matte
(`core.refine_soft_matte`). Nothing is ported; Adobe does not publish its methods, so the rules
below are ours. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it. The guided filter here is written from the paper.

**Box means.** box_r(f) at a pixel is the sum of f over the square of side 2r + 1 about it, cut
to the layer's picture; mean_r(f) is that over the number of pixels in the cut square.

**The guided filter** of a matte p by a guide I (three channels) at radius r and epsilon e:
mean_I, mean_p, the 3 by 3 covariance S = mean_r(I I^T) - mean_I mean_I^T, cov = mean_r(I p) -
mean_I mean_p; a = (S + e U)^-1 cov, b = mean_p - a . mean_I; q = mean_r(a) . I + mean_r(b).
Here e = 1e-4 always.

**Matte Choker**: two stages, each with a geometric softness G (pixels), a choke k (-127 to
127) and a gray level softness s (per cent). A stage: the disc about each pixel, the whole
offsets within G (Simple Choker's, so G below 1 is the pixel alone); m = the covering summed over
the disc, pixels outside the layer counting as empty, over the disc's size; the new covering is
ramp(m, 0.5 + k / 255, s / 100). ramp(v, c, w): lo = c - w/2 and hi = c + w/2, each held inside 0
to 1; (v - lo) / (hi - lo) held inside 0 to 1 when hi > lo, else 1 where v > c and 0 elsewhere.
The colour is the pixel's own straight colour where it had covering, else the disc's
covering-weighted average. Stage 1, then stage 2, the pair repeated `iterations` times
(whole times, the number's floor). The layer does not grow.

**Refine Hard Matte and Refine Soft Matte.** r_e = floor(edge_radius) (Soft only; Hard has
none, r_e = 0), r_f = floor(feather), R = 2 (r_e + r_f). The fill colour at a pixel is the
covering-weighted average straight colour of box_R, where there is any. Two guides: I0 is each
pixel's straight colour through the sRGB curve, black where it has no covering, so a clear pixel
counts as a colour of its own; I1 is the same with the fill colour in place of black (black
where there is no fill colour either), so the guide runs on past the matte's edge and only a
colour edge inside the picture stops the filter. Then, on the covering a0:
1. Soft only: the edge region is the pixels whose r_e square holds a covered pixel and a pixel
   not fully covered; there a1 = the guided filter of a0 by I0 at r_e, held inside 0 to 1, which
   fits the edge's covering to its colours; elsewhere a0.
2. Feather, r_f at least 1: a2 = the guided filter of a1 by I1 at r_f, held inside 0 to 1, which
   softens the edge except across a colour edge.
3. a3 = ramp(a2, 0.5 - shift_edge / 200, 1 - contrast / 100).
4. The colour C: the pixel's own straight colour where a0 > 0; elsewhere the fill colour (a
   pixel with no fill colour stays empty).
5. Decontaminate on: r_d = R + 1 + floor(decontamination_radius); over the pixels with a0 > 0,
   weights a3 and 1 - a3: F' = box(a3 C) / box(a3), B' = box((1 - a3) C) / box(1 - a3) (F' = C
   and B' = 0 where a total is 0); F = F' + a3 (C - a3 F' - (1 - a3) B') held inside 0 to 1, in
   linear light (Germer and others, "Fast Multi-Level Foreground Estimation", 2020, one level);
   C becomes C + (amount / 100) (F - C).
6. View Edge Region (Soft): the layer opaque, white in the edge region and black elsewhere.
   View Decontamination Map: the layer opaque, grey (amount / 100) 4 a3 (1 - a3) where
   decontaminate is on, black where it is off. The edge region wins when both are on.
The output is C at covering a3.

**This file never runs the build's code path.** It works in double precision with numpy on the
drawing's exact 8-bit values, box sums by running totals and numpy's 3 by 3 solve, where the
build works from its single-precision buffers. It asserts that every case's numbers move by less
than a tenth of the tolerance when the inputs are rounded to single precision, and that no hard
threshold (gray level softness 0) has a disc mean within a hundred-thousandth of it.

Every case is a composition 24 by 16 holding one drawing the same size, unmoved unless the case
says, drawn below. The drawing goes into `Fixtures/matte_refine/media`, the projects into
`Fixtures/matte_refine`, and the expected frames into
`Fixtures/matte_refine/expected_matte_refine.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/matte_refine_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as RC  # noqa: E402
from outline_reference import steps  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "matte_refine"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 24, 16, 5
EPS = 1e-4


# --- the drawing ----------------------------------------------------------------------------

ORANGE = (235, 140, 50)


def background(x, y):
    return (40 + 3 * x, 100 + 2 * y, 130 + 2 * x)


def subject(x, y):
    """An orange disc about (11, 8), radius 4.47, on a teal ramp. The matte: a hard rectangle,
    columns 6 to 16 and rows 3 to 13, holding the disc and a ring of the teal round it, with a
    one-pixel hole at (11, 8); its right edge, column 17, at half covering; a speck of orange at
    (20, 2); and strands in columns 1 to 3, rows 4 to 11, orange and teal by turns down the rows,
    at coverings 64, 160 and 224. The rest is empty."""
    rgb = ORANGE if (x - 11) ** 2 + (y - 8) ** 2 <= 20 else background(x, y)
    if (x, y) == (11, 8):
        return (0, 0, 0, 0)
    if 6 <= x <= 16 and 3 <= y <= 13:
        return rgb + (255,)
    if x == 17 and 3 <= y <= 13:
        return rgb + (128,)
    if (x, y) == (20, 2):
        return ORANGE + (255,)
    if 1 <= x <= 3 and 4 <= y <= 11:
        return (ORANGE if y % 2 == 0 else background(x, y)) + ((64, 160, 224)[x - 1],)
    return (0, 0, 0, 0)


DRAWINGS = {"subject": [[subject(x, y) for x in range(W)] for y in range(H)]}


def drawn():
    """The drawing as the build reads it: linear, premultiplied, H by W by 4."""
    return np.array([[RC.working(p) for p in row] for row in DRAWINGS["subject"]])


# --- the shared pieces ----------------------------------------------------------------------

def box(f, r):
    """The sum of f over the square of side 2r + 1 about each pixel, cut to the picture, by
    running totals down the rows and then along them."""
    for axis in (0, 1):
        n = f.shape[axis]
        run = np.cumsum(f, axis=axis)
        run = np.concatenate([np.zeros_like(np.take(run, [0], axis=axis)), run], axis=axis)
        i = np.arange(n)
        f = np.take(run, np.minimum(i + r, n - 1) + 1, axis=axis) - \
            np.take(run, np.maximum(i - r, 0), axis=axis)
    return f


def mean(f, r):
    n = box(np.ones(f.shape[:2]), r)
    return box(f, r) / n.reshape(n.shape + (1,) * (f.ndim - 2))


def guided(I, p, r, eps=EPS):
    """He, Sun and Tang, Algorithm 2, with a colour guide I (H by W by 3) and a matte p."""
    mI, mp = mean(I, r), mean(p, r)
    S_ = mean(I[..., :, None] * I[..., None, :], r) - mI[..., :, None] * mI[..., None, :]
    cov = mean(I * p[..., None], r) - mI * mp[..., None]
    a = np.linalg.solve(S_ + eps * np.eye(3), cov[..., None])[..., 0]
    b = mp - (a * mI).sum(-1)
    return (mean(a, r) * I).sum(-1) + mean(b, r)


def ramp(v, c, w):
    lo, hi = min(1.0, max(0.0, c - w / 2)), min(1.0, max(0.0, c + w / 2))
    if hi > lo:
        return np.clip((v - lo) / (hi - lo), 0, 1)
    return (v > c).astype(float)


def to_srgb(c):
    c = np.clip(c, 0, 1)
    return np.where(c <= 0.0031308, 12.92 * c, 1.055 * np.power(c, 1 / 2.4) - 0.055)


def straight(P):
    a = P[..., 3:]
    return np.divide(P[..., :3], a, out=np.zeros_like(P[..., :3]), where=a > 0)


HARD = []  # (disc means, threshold) for every hard step taken, to check the margin


# --- Matte Choker ---------------------------------------------------------------------------

def choker_stage(P, g, k, s):
    near = steps(g)
    pad = math.ceil(g)
    Q = np.pad(P, ((pad, pad), (pad, pad), (0, 0)))
    total = sum(Q[pad + dy:pad + dy + H, pad + dx:pad + dx + W] for dx, dy in near)
    m = total[..., 3] / len(near)
    c, w = 0.5 + k / 255, s / 100
    if w == 0:
        HARD.append((m, c))
    a = ramp(m, c, w)
    colour = np.where(P[..., 3:] > 0, straight(P), straight(total))
    return np.concatenate([colour * a[..., None], a[..., None]], axis=-1)


def matte_choker(P, c, n):
    for _ in range(math.floor(value_at(c["iterations"], n))):
        for stage in ("1", "2"):
            P = choker_stage(P, value_at(c["geometric_softness_" + stage], n),
                             value_at(c["choke_" + stage], n),
                             value_at(c["gray_level_softness_" + stage], n))
    return P


# --- Refine Hard Matte and Refine Soft Matte ------------------------------------------------

def edge_region(a0, re):
    return (box((a0 > 0).astype(float), re) > 0) & (box((a0 < 1).astype(float), re) > 0)


def refine(P, c, n):
    soft = c["kind"] == "soft"
    a0 = P[..., 3]
    known = a0 > 0
    re = math.floor(value_at(c["edge_radius"], n)) if soft else 0
    rf = math.floor(value_at(c["feather"], n))
    R = 2 * (re + rf)
    C = straight(P)
    filled = np.zeros(a0.shape, bool)
    if R > 0:
        total = box(P, R)
        fill = straight(total)
        filled = ~known & (total[..., 3] > 0)
        C = np.where(known[..., None], C, fill)
    a = a0
    if soft and re > 0:
        a = np.where(edge_region(a0, re), np.clip(guided(to_srgb(straight(P)), a0, re), 0, 1), a0)
    if rf > 0:
        a = np.clip(guided(to_srgb(C), a, rf), 0, 1)
    a = ramp(a, 0.5 - value_at(c["shift_edge"], n) / 200, 1 - value_at(c["contrast"], n) / 100)
    a = np.where(known | filled, a, 0.0)
    amount = value_at(c["decontamination_amount"], n) / 100
    on = c["decontaminate"] == "on"
    if on:
        rd = R + 1 + math.floor(value_at(c["decontamination_radius"], n))
        k = known.astype(float)
        wf, wb = k * a, k * (1 - a)
        sf, sb = box(wf, rd), box(wb, rd)
        Fh = np.where(sf[..., None] > 0, box(wf[..., None] * C, rd) /
                      np.where(sf > 0, sf, 1)[..., None], C)
        Bh = np.where(sb[..., None] > 0, box(wb[..., None] * C, rd) /
                      np.where(sb > 0, sb, 1)[..., None], 0)
        A = a[..., None]
        F = np.clip(Fh + A * (C - A * Fh - (1 - A) * Bh), 0, 1)
        C = C + amount * (F - C)
    if soft and c["view_edge_region"] == "on":
        v = edge_region(a0, re).astype(float)
        return np.stack([v, v, v, np.ones_like(v)], axis=-1)
    if c["view_decontamination_map"] == "on":
        v = (amount if on else 0.0) * 4 * a * (1 - a)
        return np.stack([v, v, v, np.ones_like(v)], axis=-1)
    return np.concatenate([C * a[..., None], a[..., None]], axis=-1)


# --- the cases ------------------------------------------------------------------------------

def choker(geometric_softness_1=4, choke_1=75, gray_level_softness_1=10,
           geometric_softness_2=0, choke_2=0, gray_level_softness_2=100, iterations=1, shift=0):
    return {"kind": "choker", "geometric_softness_1": geometric_softness_1, "choke_1": choke_1,
            "gray_level_softness_1": gray_level_softness_1,
            "geometric_softness_2": geometric_softness_2, "choke_2": choke_2,
            "gray_level_softness_2": gray_level_softness_2, "iterations": iterations,
            "shift": shift}


def hard(feather=2, contrast=50, shift_edge=0, decontaminate="off", decontamination_amount=100,
         decontamination_radius=0, view_decontamination_map="off", shift=0):
    return {"kind": "hard", "feather": feather, "contrast": contrast, "shift_edge": shift_edge,
            "decontaminate": decontaminate, "decontamination_amount": decontamination_amount,
            "decontamination_radius": decontamination_radius,
            "view_decontamination_map": view_decontamination_map, "shift": shift}


def soft(edge_radius=10, view_edge_region="off", feather=0, contrast=0, shift_edge=0,
         decontaminate="on", decontamination_amount=100, decontamination_radius=0,
         view_decontamination_map="off", shift=0):
    c = hard(feather, contrast, shift_edge, decontaminate, decontamination_amount,
             decontamination_radius, view_decontamination_map, shift)
    c.update(kind="soft", edge_radius=edge_radius, view_edge_region=view_edge_region)
    return c


KEYS = {
    "choker": ("geometric_softness_1", "choke_1", "gray_level_softness_1", "geometric_softness_2",
               "choke_2", "gray_level_softness_2", "iterations"),
    "hard": ("feather", "contrast", "shift_edge", "decontaminate", "decontamination_amount",
             "decontamination_radius", "view_decontamination_map"),
    "soft": ("edge_radius", "view_edge_region", "feather", "contrast", "shift_edge",
             "decontaminate", "decontamination_amount", "decontamination_radius",
             "view_decontamination_map"),
}
TYPE = {"choker": "core.matte_choker", "hard": "core.refine_hard_matte",
        "soft": "core.refine_soft_matte"}


def effected(P, c, n):
    return matte_choker(P, c, n) if c["kind"] == "choker" else refine(P, c, n)


def moved(P, dx):
    out = np.zeros_like(P)
    out[:, dx:] = P[:, :W - dx] if dx else P
    return out


def render(c, n, P=None):
    P = drawn() if P is None else P
    return moved(effected(P, c, n), c["shift"])


def as_list(P):
    return [[float(v) for v in p] for row in P for p in row]


CASES = {
    "FX-MREF-001": ("Matte Choker as added: stage 1 geometric softness 4, choke 75, gray level "
                    "softness 10 per cent; stage 2 geometric softness 0, choke 0, gray level "
                    "softness 100 per cent; one iteration. The rectangle is choked in from its "
                    "edges and its corners rounded; the speck, the strands and the half-covered "
                    "edge go; the hole at (11, 8) fills, in the orange about it.",
                    choker(), (0,)),
    "FX-MREF-002": ("Matte Choker spreading then choking: stage 1 geometric softness 3, choke "
                    "-60, gray level softness 20; stage 2 geometric softness 3, choke 60, gray "
                    "level softness 20. The hole fills and the strands join, then the edges come "
                    "back in.",
                    choker(3, -60, 20, 3, 60, 20), (0,)),
    "FX-MREF-003": ("Matte Choker with both stages at geometric softness 0, choke 0, gray level "
                    "softness 100: each pixel's disc is itself and the ramp runs 0 to 1, so the "
                    "output is the drawing.",
                    choker(0, 0, 100, 0, 0, 100), (0,)),
    "FX-MREF-004": ("Matte Choker, stage 1 geometric softness 2, choke 10, gray level softness "
                    "0: a hard step, every pixel covered in full where more than 54 per cent of "
                    "its disc is covered and empty elsewhere.",
                    choker(2, 10, 0), (0,)),
    "FX-MREF-005": ("Matte Choker, stage 1 geometric softness 2, choke 0, gray level softness "
                    "100: the covering is the disc's average, a soft blur of the matte that "
                    "spreads into the empty pixels round it.",
                    choker(2, 0, 100), (0,)),
    "FX-MREF-006": ("Matte Choker, stage 1 geometric softness 1.5, choke -40, gray level "
                    "softness 50, three iterations: each pass spreads the matte further, though "
                    "a lone corner can thin.",
                    choker(1.5, -40, 50, iterations=3), (0,)),
    "FX-MREF-007": ("Matte Choker, stage 1 choke keyed from -100 at frame 0 to 100 at frame 4: "
                    "frame 2 is choke 0.",
                    choker(choke_1=keyed((0, -100), (4, 100))), (0, 2, 4)),
    "FX-MREF-008": ("FX-MREF-001 moved three pixels right: worked in the drawing's own space, "
                    "so the same, moved.", choker(shift=3), (0,)),
    "FX-MREF-013": ("Refine Hard Matte as added: feather 2 pixels, contrast 50 per cent. The "
                    "edge-aware filter softens the edges and contrast 50 steepens them again, so "
                    "the rectangle stays nearly hard while the strands, the half-covered right edge and "
                    "the speck change, and the hole at (11, 8) fills a little.",
                    hard(), (0,)),
    "FX-MREF-014": ("Refine Hard Matte, feather 4, contrast 0: a wider, softer edge.",
                    hard(feather=4, contrast=0), (0,)),
    "FX-MREF-015": ("FX-MREF-014 with shift edge 50: the edge moves out, more covering.",
                    hard(feather=4, contrast=0, shift_edge=50), (0,)),
    "FX-MREF-016": ("FX-MREF-014 with shift edge -50: the edge moves in, less covering.",
                    hard(feather=4, contrast=0, shift_edge=-50), (0,)),
    "FX-MREF-017": ("Refine Hard Matte, feather 0, contrast 80: no filter, only the soft "
                    "pixels steepened: the strands and the half-covered edge.",
                    hard(feather=0, contrast=80), (0,)),
    "FX-MREF-018": ("Refine Hard Matte, feather 3, contrast 0, decontaminate on, amount 100, "
                    "radius 2: each part-covered pixel's colour is pulled toward the colours of "
                    "the covered pixels about it and away from those of the less covered ones; "
                    "the covering is FX-MREF-014's at feather 3.",
                    hard(feather=3, contrast=0, decontaminate="on", decontamination_radius=2),
                    (0,)),
    "FX-MREF-019": ("FX-MREF-018 at amount 50: halfway, in linear light.",
                    hard(feather=3, contrast=0, decontaminate="on", decontamination_amount=50,
                         decontamination_radius=2), (0,)),
    "FX-MREF-020": ("FX-MREF-018 with View Decontamination Map: the layer opaque, grey where "
                    "the colour is decontaminated, brightest at half covering.",
                    hard(feather=3, contrast=0, decontaminate="on", decontamination_radius=2,
                         view_decontamination_map="on"), (0,)),
    "FX-MREF-021": ("Refine Hard Matte, feather keyed from 0 at frame 0 to 6 at frame 4: "
                    "feather is whole pixels, so frame 1 (1.5) is feather 1, frame 2 feather 3 "
                    "and frame 3 (4.5) feather 4.",
                    hard(feather=keyed((0, 0), (4, 6))), (0, 1, 2, 3)),
    "FX-MREF-022": ("FX-MREF-013 moved three pixels right.", hard(shift=3), (0,)),
    "FX-MREF-023": ("Refine Soft Matte as added: edge radius 10, decontaminate on. Within 10 "
                    "pixels of the matte's edge, on so small a drawing nearly all of it, the "
                    "covering is fitted to the colours: the strands' covering follows their "
                    "orange and teal rows, and the rectangle's teal ring and right side lose "
                    "some; the colours are decontaminated.",
                    soft(), (0,)),
    "FX-MREF-024": ("Refine Soft Matte, edge radius 4, View Edge Region: white where the edge "
                    "region is, black elsewhere, the layer opaque.",
                    soft(edge_radius=4, view_edge_region="on"), (0,)),
    "FX-MREF-025": ("Refine Soft Matte, edge radius 4, feather 2, contrast 30, decontaminate "
                    "off.", soft(edge_radius=4, feather=2, contrast=30, decontaminate="off"),
                    (0,)),
    "FX-MREF-026": ("Refine Soft Matte, edge radius 0, feather 0, contrast 0, decontaminate "
                    "off: nothing to do, the output is the drawing.",
                    soft(edge_radius=0, decontaminate="off"), (0,)),
    "FX-MREF-027": ("Refine Soft Matte, edge radius 4, shift edge 30, contrast 20, "
                    "decontaminate on with radius 1, amount 70.",
                    soft(edge_radius=4, shift_edge=30, contrast=20, decontamination_radius=1,
                         decontamination_amount=70), (0,)),
}

INVALID = {
    "FX-MREF-009": ("Matte Choker, choke 1 at 128, above 127.", choker(choke_1=128)),
    "FX-MREF-010": ("Matte Choker, gray level softness 2 at 101, above 100.",
                    choker(gray_level_softness_2=101)),
    "FX-MREF-011": ("Matte Choker, iterations 0, below 1.", choker(iterations=0)),
    "FX-MREF-012": ("Matte Choker, geometric softness 1 at -1, below 0.",
                    choker(geometric_softness_1=-1)),
    "FX-MREF-028": ("Refine Hard Matte, feather 101, above 100.", hard(feather=101)),
    "FX-MREF-029": ("Refine Hard Matte, contrast -1, below 0.", hard(contrast=-1)),
    "FX-MREF-030": ("Refine Hard Matte, decontaminate written \"yes\".", hard(decontaminate="yes")),
    "FX-MREF-031": ("Refine Hard Matte, shift edge 101, above 100.", hard(shift_edge=101)),
    "FX-MREF-032": ("Refine Hard Matte, decontamination radius -1, below 0.",
                    hard(decontamination_radius=-1)),
    "FX-MREF-033": ("Refine Soft Matte, edge radius 101, above 100.", soft(edge_radius=101)),
    "FX-MREF-034": ("Refine Soft Matte, view edge region written \"maybe\".",
                    soft(view_edge_region="maybe")),
    "FX-MREF-035": ("Refine Soft Matte, decontamination amount 101, above 100.",
                    soft(decontamination_amount=101)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-subject", "kind": "still", "name": "subject",
                    "path": "media/subject.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-subject",
                "enabled": True, "locked": False, "in_frame": 0, "out_frame": FRAMES,
                "source_offset_frames": 0,
                "transform": {
                    "anchor": prop([W / 2, H / 2]),
                    "position": prop([W / 2 + c["shift"], H / 2]),
                    "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
                },
                "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": TYPE[c["kind"]],
                             "enabled": True,
                             "parameters": {k: setting_json(c[k]) for k in KEYS[c["kind"]]}}],
            }],
        }],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def plain(c):
    return as_list(moved(drawn(), c["shift"]))


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    rounded = drawn().astype(np.float32).astype(np.float64)
    for fx, (says, c, frames) in sorted(CASES.items()):
        rendered = {}
        for f in frames:
            got = render(c, f)
            moved_by = float(np.abs(render(c, f, rounded) - got).max())
            assert moved_by < TOLERANCE / 10, (fx, f, moved_by)
            rendered[str(f)] = as_list(got)
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in sorted(INVALID.items()):
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    expected["cases"] = dict(sorted(expected["cases"].items()))

    (OUT / "expected_matte_refine.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    P = drawn()
    flat = as_list(P)
    at = lambda x, y: y * W + x  # noqa: E731
    alpha = lambda f: np.array([p[3] for p in f]).reshape(H, W)  # noqa: E731

    # The guided filter's own pieces: a constant matte comes back constant; with a guide that
    # does not vary it is the box mean of the matte, a little pulled toward it by e.
    I = to_srgb(straight(P))
    assert np.allclose(guided(I, np.full((H, W), 0.7), 3), 0.7, atol=1e-12)
    flatI = np.full((H, W, 3), 0.4)
    p = P[..., 3]
    assert np.allclose(guided(flatI, p, 2), mean(mean(p, 2), 2), atol=1e-12)
    # A matte that is exactly a linear function of the guide is kept, nearly (e is small).
    lin = 0.2 + 0.5 * I[..., 0] - 0.3 * I[..., 2]
    assert np.abs(guided(I, lin, 3) - lin).max() < 0.02
    # Every hard step clear of its threshold.
    for m, t in HARD:
        assert np.abs(m - t).min() > 1e-5

    # Matte Choker.
    one = alpha(c["FX-MREF-001"]["0"])
    assert one[2, 20] == 0 and one[4:12, 1:4].max() == 0 and one[:, 17].max() == 0
    assert one[8, 11] == 1  # the hole filled
    orange = RC.working(ORANGE + (255,))
    assert np.allclose(c["FX-MREF-001"]["0"][at(11, 8)][:3], orange[:3], atol=1e-12)
    assert one[3, 6] == 0 and one[8, 11] == 1 and one.sum() < p.sum()
    assert c["FX-MREF-003"]["0"] == flat
    four = alpha(c["FX-MREF-004"]["0"])
    assert set(np.unique(four)) <= {0.0, 1.0}
    five = alpha(c["FX-MREF-005"]["0"])
    assert five[8, 5] > 0 and p[8, 5] == 0 and 0 < five[3, 6] < 1  # spread, and softened
    six = alpha(c["FX-MREF-006"]["0"])
    once = render(choker(1.5, -40, 50), 0)[..., 3]
    assert six.sum() > once.sum() > p.sum() and (six > 0).sum() > (once > 0).sum()
    seven = c["FX-MREF-007"]
    assert np.allclose(seven["2"], as_list(render(choker(choke_1=0), 2)), atol=0)
    assert alpha(seven["0"]).sum() > alpha(seven["2"]).sum() > alpha(seven["4"]).sum()
    eight = c["FX-MREF-008"]["0"]
    for y in range(H):
        assert eight[at(3, y):at(W, y)] == c["FX-MREF-001"]["0"][at(0, y):at(W - 3, y)]

    # Refine Hard Matte: the teal ring loses covering against the disc, which keeps it.
    t13 = alpha(c["FX-MREF-013"]["0"])
    assert 0 < t13[8, 11] < 0.1 and (t13[3:14, 6:15] > 0.99).sum() == 11 * 9 - 1
    assert t13[8, 17] != p[8, 17] and t13[5, 1] != p[5, 1] and t13[2, 20] < 1
    f14 = alpha(c["FX-MREF-014"]["0"])
    assert f14[3, 6] < 1 and f14[3, 5] > 0  # softened both ways
    assert (alpha(c["FX-MREF-015"]["0"]) >= f14 - 1e-12).all()
    assert (alpha(c["FX-MREF-016"]["0"]) <= f14 + 1e-12).all()
    assert alpha(c["FX-MREF-015"]["0"]).sum() > f14.sum() > alpha(c["FX-MREF-016"]["0"]).sum()
    s17 = alpha(c["FX-MREF-017"]["0"])
    assert ((p == 0) | (p == 1)) .sum() == ((s17 == p) & ((p == 0) | (p == 1))).sum()
    assert s17[5, 1] < p[5, 1] and s17[5, 3] > p[5, 3]  # 64 down, 224 up
    d18, d19 = np.array(c["FX-MREF-018"]["0"]), np.array(c["FX-MREF-019"]["0"])
    n18 = np.array(as_list(render(hard(feather=3, contrast=0), 0)))
    assert np.allclose(d18[:, 3], n18[:, 3]) and np.abs(d18 - n18).max() > 1e-3
    assert np.allclose(d19, (d18 + n18) / 2, atol=1e-12)  # amount 50: halfway, premultiplied
    m20 = np.array(c["FX-MREF-020"]["0"])
    assert (m20[:, 3] == 1).all() and m20[:, 0].max() > 0.5 and m20[:, 0].min() == 0
    k21 = c["FX-MREF-021"]
    assert k21["2"] == as_list(render(hard(feather=3), 2))
    assert k21["1"] == as_list(render(hard(feather=1), 1))
    assert k21["3"] == as_list(render(hard(feather=4), 3))
    assert k21["0"] == as_list(render(hard(feather=0), 0))
    t22 = c["FX-MREF-022"]["0"]
    for y in range(H):
        assert t22[at(3, y):at(W, y)] == c["FX-MREF-013"]["0"][at(0, y):at(W - 3, y)]

    # Refine Soft Matte.
    v24 = np.array(c["FX-MREF-024"]["0"])
    region = v24[:, 0].reshape(H, W)
    assert (v24[:, 3] == 1).all() and set(np.unique(v24[:, 0])) == {0.0, 1.0}
    assert region[0, 0] == 1 and region[15, 23] == 0  # near the strands, far from all
    assert c["FX-MREF-026"]["0"] == flat
    s23 = alpha(c["FX-MREF-023"]["0"])
    assert abs(s23[4, 2] - s23[5, 2]) > abs(p[4, 2] - p[5, 2])  # follows the rows' colours
    assert s23[3, 6] < 0.9 and s23[8, 16] < 0.9 and s23[8, 12] > 0.9
    assert c["FX-MREF-023"]["0"] != c["FX-MREF-027"]["0"] != c["FX-MREF-025"]["0"]

    for name, frames in c.items():
        for px in frames.values():
            for q in px:
                assert -1e-12 <= q[3] <= 1 + 1e-12 and all(-1e-12 <= v <= q[3] + 1e-12
                                                          for v in q[:3]), (name, q)
    print("checked")


if __name__ == "__main__":
    main()
