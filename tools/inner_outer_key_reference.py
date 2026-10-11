"""Inner/Outer Key, worked a second way.

D-502 adds `core.inner_outer_key`, after After Effects' Inner/Outer Key (Keying, EFFECTS.md row
310): a subject is cut from its background by two masks, one drawn just inside the subject's
edge and one just outside it, and the colours of the soft edge between them are worked out
again without the background's spill. Adobe's help describes the settings (Foreground (Inside),
Background (Outside), Single Mask Highlight Radius, Edge Thin, Edge Feather, Edge Threshold,
Invert Extraction, Blend With Original) but publishes no formula, so the rule below, the ranges
and the settings when added are ours, the "rule ours" precedent. Additional Foreground,
Additional Background and the Cleanup brushes are not built (document 21 names them as gaps).

The masks are chosen as Fill chooses its one (D-418): the layer's masks at the frame, each as
document 19 flattens it, in the drawing's own space (the top-left corner (0, 0), however far an
effect above grew the buffer), whatever its own mode, inversion, feather, expansion or opacity.
`foreground` and `background` number them (1 the first, the floor taken). `foreground` 0 is none:
nothing is keyed and the layer is left as it is. `background` 0 is none: the single-mask way.
A number that names no mask switched on with two points or more leaves the layer as it is, and
EFFECT_PATH_MISSING is said every frame.

At each pixel of the buffer, whose drawing-space top-left corner is (X, Y):

1. Regions. cin is the foreground's covering, ADR-016's 4 by 4 grid of even-odd samples over
   sixteen; cout the background's. With two masks the pixel is kept (K) where cin >= 0.5, gone
   (G) where it is not kept and cout < 0.5, and unknown (U) otherwise. With one mask, d is the
   distance from the pixel's centre (X + 0.5, Y + 0.5) to the foreground's closed outline, r the
   Single Mask Highlight Radius: K where cin >= 0.5 and d > r, G where cin < 0.5 and d > r, U
   where d <= r. Invert Extraction swaps K and G.
2. Estimates. For a set S of pixels (K or G), D_S is the Chebyshev distance max(|dx|, |dy|) from
   the pixel to the nearest pixel of S, and E_S the colour of the S pixels in the square of
   half-side D_S about the pixel: the sum of their premultiplied colours over the sum of their
   alphas, 0 where that sum is 0. F = E_K (0 with K empty), B = E_G (0 with G empty), straight
   and linear.
3. The extraction a. K: 1. G: 0. U: 0 with K empty; else 1 with G empty; else, C the pixel's
   straight colour (rgb / alpha, 0 where alpha is 0) and v = F - B,
   a = clamp((C - B) . v / |v|^2, 0, 1) when |v|^2 > 1e-12, else D_G / (D_K + D_G).
4. The kept colour Fc: K, C; U, clamp((C - (1 - a) B) / a, 0, 1) each channel when a > 0, else F;
   G, F.
5. The matte m = a, then in turn: Edge Thin t pixels, above 0 the least m within the disc
   dx^2 + dy^2 <= t^2 (Simple Choker's disc), below 0 the greatest within |t|'s disc, in either
   case only pixels inside the buffer; Edge Feather f, document 21's Gaussian at sigma f / 2, cut
   at ceil(3 sigma), across then down, with the buffer's edges held; Edge Threshold h, with
   q = h / 100, m = clamp((m - q) / (1 - q), 0, 1) for q < 1, and at q = 1 m is 1 where it is 1
   and 0 elsewhere.
6. The pixel P (premultiplied, alpha A) becomes A m Fc with alpha A m, then
   P' (1 - b) + P b with b = Blend With Original / 100.

`foreground` and `background` 0 to 1000, keyable, 0 when added; `single_mask_highlight_radius`
0 to 1000 pixels, keyable, 10 when added; `edge_thin` -100 to 100 pixels, keyable, 0 when added;
`edge_feather` 0 to 1000 pixels, keyable, 0 when added; `edge_threshold` 0 to 100, keyable, 0
when added; `invert_extraction` `off` (when added) or `on`; `blend_with_original` 0 to 100,
keyable, 0 when added. The radius, thin and feather are distances: a draft halves them; the masks
are scaled as they always are. The layer never grows.

**This file never runs the build's code path.** It samples every pixel's sixteen points, finds
every nearest pixel and adds every square by hand, in double precision on lists, straight from
the drawing's 8-bit values.

Every case is a composition 16 by 10 holding the drawing `subject`, the same size, unmoved unless
the case says: a blue ground, a yellow block (5, 3) to (10, 6) whose edge pixels are mixtures of
the two in linear light, the left column half see-through and the right column empty. The masks
are of mode None (which leave the drawing as it is). The expected frames are in
`Fixtures/inner_outer_key/expected_inner_outer_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/inner_outer_key_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from mask_reference import flatten, points_at, points, mask_json  # noqa: E402
from mask_reference import keyed as moving_path  # noqa: E402
from fill_reference import sampled, weights, none  # noqa: E402

W, H = 16, 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "inner_outer_key"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"foreground": (0, 1000), "background": (0, 1000),
          "single_mask_highlight_radius": (0, 1000), "edge_thin": (-100, 100),
          "edge_feather": (0, 1000), "edge_threshold": (0, 100), "blend_with_original": (0, 100)}
WORDS = ("invert_extraction",)
NAMES = ("foreground", "background", "single_mask_highlight_radius", "edge_thin", "edge_feather",
         "edge_threshold", "invert_extraction", "blend_with_original")

BLUE = (40, 80, 200)
YELLOW = (240, 210, 40)


def mixed(t):
    """t of the yellow and 1 - t of the blue, mixed in linear light, written in 8 bits."""
    return tuple(round(255 * S.linear_to_srgb(t * srgb_to_linear(y / 255)
                                              + (1 - t) * srgb_to_linear(b / 255)))
                 for y, b in zip(YELLOW, BLUE))


def subject(x, y):
    if x == 15:
        return (0, 0, 0, 0)
    inx, iny = 5 <= x <= 10, 3 <= y <= 6
    side, row = x in (4, 11), y in (2, 7)
    t = 1.0 if inx and iny else 0.5 if side and iny else 0.25 if inx and row else \
        0.125 if side and row else 0.0
    return mixed(t) + (128 if x == 0 else 255,)


DRAWINGS = {"subject": [[subject(x, y) for x in range(W)] for y in range(H)]}

INNER = points([5, 3], [11, 3], [11, 7], [5, 7])       # just the solid yellow
OUTER = points([3, 1], [13, 1], [13, 9], [3, 9])       # two pixels round it
LOW = points([5, 5], [11, 5], [11, 9], [5, 9])         # INNER two rows lower
C4 = 0.5522847498307936
RING = points([8, 1], [12, 5], [8, 9], [4, 5], handles=[  # a circle 8 across, curved
    ([-4 * C4, 0], [4 * C4, 0]), ([0, -4 * C4], [0, 4 * C4]),
    ([4 * C4, 0], [-4 * C4, 0]), ([0, 4 * C4], [0, -4 * C4])])
OFF = points([20, 2], [24, 2], [24, 6], [20, 6])       # right of the picture: no pixel inside
WIDE = points([-30, -30], [46, -30], [46, 40], [-30, 40])  # the whole picture and more


# --- the rule -------------------------------------------------------------------------------

def segment_distance(a, b, x, y):
    dx, dy = b[0] - a[0], b[1] - a[1]
    ll = dx * dx + dy * dy
    t = 0.0 if ll == 0 else max(0.0, min(1.0, ((x - a[0]) * dx + (y - a[1]) * dy) / ll))
    return math.hypot(x - (a[0] + t * dx), y - (a[1] + t * dy))


def path_distance(outline, x, y):
    return min(segment_distance(outline[i], outline[(i + 1) % len(outline)], x, y)
               for i in range(len(outline)))


def clamp(v, lo=0.0, hi=1.0):
    return min(hi, max(lo, v))


def estimate(layer, members, x, y):
    """(D_S, E_S) at pixel (x, y) of the buffer, every member looked at; None with S empty."""
    w = layer["w"]
    if not members:
        return None
    d = min(max(abs(i - x), abs(j - y)) for i, j in members)
    rgb, alpha = [0.0, 0.0, 0.0], 0.0
    for i, j in members:
        if max(abs(i - x), abs(j - y)) <= d:
            p = layer["px"][j * w + i]
            rgb = [rgb[c] + p[c] for c in range(3)]
            alpha += p[3]
    return d, [v / alpha for v in rgb] if alpha > 0 else [0.0, 0.0, 0.0]


def blurred(m, w, h, feather):
    taps, r = weights(feather)
    if r == 0:
        return m
    across = [sum(taps[k] * m[y * w + min(w - 1, max(0, x + k - r))] for k in range(len(taps)))
              for y in range(h) for x in range(w)]
    return [sum(taps[k] * across[min(h - 1, max(0, y + k - r)) * w + x] for k in range(len(taps)))
            for y in range(h) for x in range(w)]


def key(layer, fg, bg, n, invert):
    w, h, left, top = layer["w"], layer["h"], layer["left"], layer["top"]
    region = []
    for j in range(h):
        for i in range(w):
            X, Y = left + i, top + j
            inside = sampled(fg, X, Y) >= 0.5
            if bg is None:
                far = path_distance(fg, X + 0.5, Y + 0.5) > n["single_mask_highlight_radius"]
                k, g = inside and far, (not inside) and far
            else:
                k = inside
                g = not k and sampled(bg, X, Y) < 0.5
            if invert == "on":
                k, g = g, k
            region.append("K" if k else "G" if g else "U")
    kept = [(i % w, i // w) for i, s in enumerate(region) if s == "K"]
    gone = [(i % w, i // w) for i, s in enumerate(region) if s == "G"]
    a, fc = [], []
    for idx, p in enumerate(layer["px"]):
        x, y = idx % w, idx // w
        C = [v / p[3] for v in p[:3]] if p[3] > 0 else [0.0, 0.0, 0.0]
        ek, eg = estimate(layer, kept, x, y), estimate(layer, gone, x, y)
        F = ek[1] if ek else [0.0, 0.0, 0.0]
        B = eg[1] if eg else [0.0, 0.0, 0.0]
        s = region[idx]
        if s == "K":
            al = 1.0
        elif s == "G":
            al = 0.0
        elif ek is None:
            al = 0.0
        elif eg is None:
            al = 1.0
        else:
            v = [F[c] - B[c] for c in range(3)]
            vv = sum(t * t for t in v)
            if vv > 1e-12:
                al = clamp(sum((C[c] - B[c]) * v[c] for c in range(3)) / vv)
            else:
                al = eg[0] / (ek[0] + eg[0])
        if s == "K":
            f = C
        elif s == "U" and al > 0:
            f = [clamp((C[c] - (1 - al) * B[c]) / al) for c in range(3)]
        else:
            f = F
        a.append(al)
        fc.append(f)
    m = a
    t = n["edge_thin"]
    if t != 0:
        rr, R0 = t * t, math.floor(abs(t))
        disc = [(dx, dy) for dy in range(-R0, R0 + 1) for dx in range(-R0, R0 + 1)
                if dx * dx + dy * dy <= rr]
        pick = min if t > 0 else max
        m = [pick(m[(y + dy) * w + x + dx] for dx, dy in disc
                  if 0 <= x + dx < w and 0 <= y + dy < h)
             for y in range(h) for x in range(w)]
    m = blurred(m, w, h, n["edge_feather"])
    q = n["edge_threshold"] / 100
    m = [clamp((v - q) / (1 - q)) if q < 1 else (1.0 if v >= 1 else 0.0) for v in m]
    b = n["blend_with_original"] / 100
    px = []
    for p, f, v in zip(layer["px"], fc, m):
        out = [p[3] * v * f[c] for c in range(3)] + [p[3] * v]
        px.append([out[c] * (1 - b) + p[c] * b for c in range(4)])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(masks=None, foreground=1, background=2, single_mask_highlight_radius=10, edge_thin=0,
         edge_feather=0, edge_threshold=0, invert_extraction="off", blend_with_original=0,
         shift=0, tile=False):
    return {"drawing": "subject", "masks": [none(INNER), none(OUTER)] if masks is None else masks,
            "foreground": foreground, "background": background,
            "single_mask_highlight_radius": single_mask_highlight_radius, "edge_thin": edge_thin,
            "edge_feather": edge_feather, "edge_threshold": edge_threshold,
            "invert_extraction": invert_extraction, "blend_with_original": blend_with_original,
            "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


MISSING_PATH = "missing"


def chosen(c, k, frame_no):
    """The numbered mask's outline; None for 0; MISSING_PATH when it names none usable."""
    i = math.floor(held(c, k, frame_no)) - 1
    if i < 0:
        return None
    if i >= len(c["masks"]):
        return MISSING_PATH
    m = c["masks"][i]
    pts = points_at(m, frame_no)
    return flatten(pts) if m.get("enabled", True) and len(pts) >= 2 else MISSING_PATH


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    fg, bg = chosen(c, "foreground", frame_no), chosen(c, "background", frame_no)
    if fg is None or MISSING_PATH in (fg, bg):
        return plain(c)
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(key(layer_of(c), fg, bg, n, c["invert_extraction"]), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


ONE = [none(INNER)]

CASES = {
    "FX-IOK-001": ("The settings as they start: Foreground 0 (none): nothing is keyed, the "
                   "picture as it was.", case(foreground=0, background=0), [0]),
    "FX-IOK-002": ("Foreground mask 1 just inside the yellow block, Background mask 2 two pixels "
                   "outside it: the block kept, the blue outside the outer mask gone, the edge "
                   "mixtures between them see-through as much as they are blue, their colour "
                   "the yellow without the blue.", case(), [0]),
    "FX-IOK-003": ("One mask, the inner one, Single Mask Highlight Radius 1.2: the pixels "
                   "whose centres lie within 1.2 of its outline are worked out, the rest kept "
                   "inside it and gone outside.",
                   case(masks=ONE, background=0, single_mask_highlight_radius=1.2), [0]),
    "FX-IOK-004": ("One mask with the radius as added, 10: every pixel of this small picture "
                   "lies within 10 of the outline, so nothing is surely kept and every pixel goes.",
                   case(masks=ONE, background=0), [0]),
    "FX-IOK-005": ("FX-IOK-002 with Invert Extraction: the block is gone and the blue kept.",
                   case(invert_extraction="on"), [0]),
    "FX-IOK-006": ("Edge Thin 1: the matte shrinks a pixel all round.", case(edge_thin=1), [0]),
    "FX-IOK-007": ("Edge Thin -1: the matte grows a pixel all round.", case(edge_thin=-1), [0]),
    "FX-IOK-008": ("Edge Feather 2: the matte's edge softens.", case(edge_feather=2), [0]),
    "FX-IOK-009": ("Edge Threshold 50: the half-kept pixels and less go, the rest harden.",
                   case(edge_threshold=50), [0]),
    "FX-IOK-010": ("Blend With Original 50: half way between FX-IOK-002 and the picture.",
                   case(blend_with_original=50), [0]),
    "FX-IOK-011": ("Edge Thin 1.5, Edge Feather 3 and Edge Threshold 20 together, in that order.",
                   case(edge_thin=1.5, edge_feather=3, edge_threshold=20), [0]),
    "FX-IOK-012": ("One mask, the radius keyed from 0 at frame 0 to 1.2 at frame 4: a hard cut "
                   "along the outline at frame 0; at frame 2, 0.6, the pixels either side of its "
                   "straight sides are worked out; at frame 4 its corners too.",
                   case(masks=ONE, background=0,
                        single_mask_highlight_radius=keyed((0, 0), (4, 1.2))), [0, 2, 4]),
    "FX-IOK-013": ("Blend With Original keyed from 0 at frame 0 to 100 at frame 4: FX-IOK-002, "
                   "half way, then the picture.", case(blend_with_original=keyed((0, 0), (4, 100))),
                   [0, 2, 4]),
    "FX-IOK-014": ("The inner mask's path keyed two rows lower by frame 4: the kept part "
                   "follows it, frames 0, 2 and 4.",
                   case(masks=[{**moving_path((0, INNER), (4, LOW)), "mode": "none"},
                               none(OUTER)]), [0, 2, 4]),
    "FX-IOK-015": ("FX-IOK-002 moved three pixels right: the key moves with the layer.",
                   case(shift=3), [0]),
    "FX-IOK-016": ("After a Motion Tile that grows the layer: the masks are the drawing's own, "
                   "so they key the drawing's place in the grown buffer, whose copies lie "
                   "outside the outer mask and are gone.", case(tile=True), [0]),
    "FX-IOK-017": ("Foreground 1.5 and Background 2.7: their floors, FX-IOK-002's frame.",
                   case(foreground=1.5, background=2.7), [0]),
    "FX-IOK-018": ("Foreground and Background the same mask: nothing is unknown, a hard cut "
                   "along it.", case(background=1), [0]),
    "FX-IOK-019": ("A foreground mask right of the picture: nothing is kept, so every pixel "
                   "goes.", case(masks=[none(OFF), none(OUTER)]), [0]),
    "FX-IOK-020": ("A background mask round the whole picture: nothing is gone, so every pixel "
                   "is kept as it was.", case(masks=[none(INNER), none(WIDE)]), [0]),
    "FX-IOK-021": ("One curved mask, a circle 8 across about (8, 5), radius 1.2.",
                   case(masks=[none(RING)], background=0, single_mask_highlight_radius=1.2), [0]),
    "FX-IOK-022": ("Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease "
                   "that passes its end: held at 100 at frame 2, where it would pass it.",
                   case(blend_with_original=keyed((0, 40, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-IOK-023": ("Foreground 0 with a background named: still none, the picture as it was.",
                   case(foreground=0), [0]),
    "FX-IOK-024": ("Edge Threshold 100: only the wholly kept pixels stay, whole.",
                   case(edge_threshold=100), [0]),
}

MISSING = {
    "FX-IOK-025": ("Foreground 1 with no masks at all.", case(masks=[])),
    "FX-IOK-026": ("Foreground 3, of two.", case(foreground=3)),
    "FX-IOK-027": ("Background 3, of two.", case(background=3)),
    "FX-IOK-028": ("Foreground 1, switched off.",
                   case(masks=[none(INNER, enabled=False), none(OUTER)])),
    "FX-IOK-029": ("Background 2, switched off.",
                   case(masks=[none(INNER), none(OUTER, enabled=False)])),
}

INVALID = {
    "FX-IOK-030": ("Foreground -1, below 0.", case(foreground=-1)),
    "FX-IOK-031": ("Background 1001, above 1000.", case(background=1001)),
    "FX-IOK-032": ("Single Mask Highlight Radius -1, below 0.",
                   case(single_mask_highlight_radius=-1)),
    "FX-IOK-033": ("Edge Thin -101, below -100.", case(edge_thin=-101)),
    "FX-IOK-034": ("Edge Thin 101, above 100.", case(edge_thin=101)),
    "FX-IOK-035": ("Edge Feather 1001, above 1000.", case(edge_feather=1001)),
    "FX-IOK-036": ("Edge Threshold 101, above 100.", case(edge_threshold=101)),
    "FX-IOK-037": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-IOK-038": ("Invert Extraction \"yes\", not off or on.", case(invert_extraction="yes")),
    "FX-IOK-039": ("Blend With Original keyed to 101 at frame 4, above 100.",
                   case(blend_with_original=keyed((0, 0), (4, 101)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    layer = comp["layers"][0]
    t = layer["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    if c["masks"]:
        # In the order the build writes a layer, `masks` where `mask` was.
        layer = {k: v for k, v in layer.items() if k != "mask"}
        at = list(layer).index("matte")
        items = list(layer.items())
        items.insert(at, ("masks", [mask_json(m, i + 1) for i, m in enumerate(c["masks"])]))
        layer = dict(items)
        comp["layers"][0] = layer
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.inner_outer_key",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    layer["effects"] = effects
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
    for fx, (says, c) in MISSING.items():
        says += (" Nothing to key by: the layer is drawn without the effect, which is kept as "
                 "written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert render(c, 0) == before
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_inner_outer_key.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    clear = [0.0] * 4
    yellow = [srgb_to_linear(v / 255) for v in YELLOW]
    block = lambda x, y: 5 <= x <= 10 and 3 <= y <= 6  # noqa: E731
    ring = lambda x, y: 3 <= x <= 12 and 1 <= y <= 8  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            assert all(all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]) for p in px), fx
    assert c["FX-IOK-001"]["0"] == art and c["FX-IOK-023"]["0"] == art
    two = c["FX-IOK-002"]["0"]
    for y in range(H):
        for x in range(W):
            if block(x, y):
                assert near(two[at(x, y)], art[at(x, y)]), (x, y)
            elif not ring(x, y):
                assert two[at(x, y)] == clear, (x, y)
    # The pure blue inside the ring goes; the half mixtures keep about half, in yellow.
    assert two[at(3, 4)][3] < 0.01 and two[at(12, 4)][3] < 0.01
    edge = two[at(4, 4)]
    assert abs(edge[3] - 0.5) < 0.02, edge
    assert near([v / edge[3] for v in edge[:3]], yellow, 0.03), edge
    assert abs(two[at(7, 2)][3] - 0.25) < 0.02 and abs(two[at(4, 2)][3] - 0.125) < 0.02
    inv = c["FX-IOK-005"]["0"]
    assert all(inv[at(x, y)] == clear for x in range(5, 11) for y in range(3, 7))
    assert all(near(inv[at(x, y)], art[at(x, y)]) for x in range(W) for y in range(H)
               if not ring(x, y))
    one = c["FX-IOK-003"]["0"]
    assert near(one[at(7, 5)], art[at(7, 5)]) and one[at(2, 5)] == clear
    assert all(p == clear for p in c["FX-IOK-004"]["0"])
    thin, grow = c["FX-IOK-006"]["0"], c["FX-IOK-007"]["0"]
    assert thin[at(5, 4)][3] < two[at(5, 4)][3] and thin[at(7, 5)][3] == two[at(7, 5)][3]
    assert grow[at(4, 4)][3] > two[at(4, 4)][3] and grow[at(1, 4)] == clear
    assert c["FX-IOK-008"]["0"] != two and c["FX-IOK-009"]["0"] != two
    half = c["FX-IOK-010"]["0"]
    assert all(near(half[i], [(a + b) / 2 for a, b in zip(two[i], art[i])], 1e-12)
               for i in range(W * H))
    assert c["FX-IOK-011"]["0"] not in (two, thin, c["FX-IOK-008"]["0"])
    radius = c["FX-IOK-012"]
    assert radius["0"] != radius["2"] and radius["4"] not in (radius["2"], radius["0"])
    assert radius["4"] == one
    rising = c["FX-IOK-013"]
    assert rising["0"] == two and rising["4"] == art and near(rising["2"][at(4, 4)],
                                                              half[at(4, 4)], 1e-12)
    moving = c["FX-IOK-014"]
    assert moving["0"] == two and moving["4"] != two and moving["2"] not in (two, moving["4"])
    moved = c["FX-IOK-015"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-IOK-016"]["0"] != art
    assert c["FX-IOK-017"]["0"] == two
    hard = c["FX-IOK-018"]["0"]
    assert all(hard[at(x, y)] == (art[at(x, y)] if block(x, y) else clear) or
               near(hard[at(x, y)], art[at(x, y)]) for x in range(W) for y in range(H))
    assert all(p == clear for p in c["FX-IOK-019"]["0"])
    assert all(near(p, q, 1e-12) for p, q in zip(c["FX-IOK-020"]["0"], art))
    assert c["FX-IOK-021"]["0"] not in (one, art)
    eased = c["FX-IOK-022"]
    assert eased["0"] != art and eased["2"] == art and eased["4"] == art
    full = c["FX-IOK-024"]["0"]
    assert all(p[3] in (0.0, art[i][3]) or near(p, art[i]) for i, p in enumerate(full))
    for fx in list(MISSING) + list(INVALID):
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
