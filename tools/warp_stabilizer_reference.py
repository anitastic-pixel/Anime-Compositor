"""Warp Stabilizer, worked a second way.

D-653 proposes Warp Stabilizer (`core.warp_stabilizer`), after After Effects' Warp Stabilizer
(Distort): shaky footage steadied by the camera's own motion, found between neighbouring frames by
this program's own estimate (`tools/flow_reference.py`), smoothed over the frames around, and the
picture moved back by the difference. Adobe's page names the controls and says what each is for;
it publishes no formula, and the fit, the smoothing window, the crop and the scale are ours (P0-13,
D-600: "Blur + retiming only"). Nothing is analysed ahead or kept: every frame is worked from the
drawings around it.

At the holder's composition frame n, with n0 its in point, m1 its last frame (out point - 1) and
P(m) the holder at composition frame m with the effects before Warp Stabilizer, by the layer's own
timing, W by H, linear and premultiplied:

1. On an adjustment layer the output is the input, and the frame says
   TEMPORAL_SMOOTHING_SKIPPED: the frames beneath at other times are not to hand.
2. Points are worked in coordinates centred on the picture and divided by D = hypot(W, H) / 2:
   pixel (x, y)'s centre is ((x + 0.5 - W/2) / D, (y + 0.5 - H/2) / D).
3. The pair model M_k, from frame k to frame k + 1 (both n0 to m1): F = the estimate from P(k) to
   P(k + 1), Vector Detail 50, or 100 with Detailed Analysis `on` (finer than Pixel Motion
   Blur's 20, since the whole picture's move is wanted to a fraction of a pixel), the
   estimate's other settings as added. Every pixel p of P(k) is a point, its target p + F(p), its weight w0 = P(k)'s alpha
   there. Three weighted least-squares fits, the first with w0, the second and third with
   w0 / (1 + r^2), r the point's distance in pixels from where the fit before sent it:
   - `position`: a move, (tx, ty) the weighted mean of target - point;
   - `position_scale_rotation`: x' = a x - b y + tx, y' = b x + a y + ty, by the normal
     equations; with Preserve Scale `on`, (a, b) divided by its length after each fit;
   - `perspective`: x' = (a x + b y + c) / (g x + h y + 1) and y' likewise with d, e, f, from the
     linear form a x + b y + c - g x x' - h y x' = x' (and y'), by the normal equations.
   The normal equations are solved by Gaussian elimination with partial pivoting; a pivot below
   1e-12, or weights summing below 4, gives the unchanged map for that pair. Every model is a
   3 by 3 matrix whose corner is 1. Two frames that are the same (a drawing held on twos) give
   the unchanged map without an estimate.
4. T_k, from frame n to frame k: T_n is the unchanged map; T_(k+1) = M_k T_k above n and
   T_(k-1) = M_(k-1)^-1 T_k below, each product divided through by its corner.
5. The steadied map S:
   - Result `smooth_motion`: R = floor(fps Smoothness / 200 + 0.5), fps the composition's frames
     a second (24 in every case here), held to R_n = min(R, n - n0, m1 - n) so the window is always even about n. With
     R_n = 0, S is the unchanged map. Otherwise weights exp(-(k - n)^2 / (2 (R_n / 2)^2)) over k
     = n - R_n to n + R_n, and S = their weighted mean of T_k, number by number.
   - Result `no_motion`: S = T_n0, the map back to the layer's first frame.
6. Framing. c = (W/2, H/2) in pixels. s is the largest number from 0 to 1 for which the four
   corners of the rectangle about c, s W by s H, all land inside the picture (0 to W, 0 to H)
   when taken through S^-1: 60 halvings of 0 to 1, starting from 1 when 1 already fits.
   - `stabilize_only`: z = 1 and no rectangle.
   - `stabilize_crop`: z = 1, and outside that rectangle the picture is clear.
   - `stabilize_crop_auto_scale` (as added): the rectangle, and z = (1 - 2 m) / s held inside 1
     and Maximum Scale / 100 (Maximum Scale / 100 when s = 0), m = Action-Safe Margin / 100.
   Z = z Additional Scale / 100.
7. With S the unchanged map and Z = 1 the output is P(n) exactly. Otherwise, at pixel (x, y): q = c + ((x + 0.5, y + 0.5) - c) / Z; clear if a rectangle is used
   and q lies outside it (|q - c| more than s W / 2 or s H / 2); otherwise P(n) read bilinearly
   by document 21 (transparent outside) at S^-1(q), with points as step 2.

Settings: `result` `smooth_motion` or `no_motion`, `smooth_motion` when added; `smoothness` 0 to
1000 per cent, 50; `method` `position`, `position_scale_rotation` or `perspective`,
`position_scale_rotation`; `preserve_scale` `off` or `on`, `off`; `framing` `stabilize_only`,
`stabilize_crop` or `stabilize_crop_auto_scale`, `stabilize_crop_auto_scale`; `maximum_scale`
100 to 1000 per cent, 150; `action_safe_margin` 0 to 50 per cent, 0; `additional_scale` 1 to
1000 per cent, 100; `detailed_analysis` `off` or `on`, `off`. The numbers are keyable and read at
frame n, the words not.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 32 by 32, six frames, in `Fixtures/warp_stabilizer/`:
the drawing `holder`, a textured card shaken about a steady pan (`shake`), the same card also
turned a little each frame (`turn`), or Pixel Motion Blur's soft disc, with the effect. The
expected pixels are in `Fixtures/warp_stabilizer/expected_warp_stabilizer.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/warp_stabilizer_reference.py
"""

import json
import sys
from math import cos, exp, floor, hypot, pi, sin
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import flow_reference as flow  # noqa: E402
import pixel_motion_blur_reference as P  # noqa: E402
from pixel_motion_blur_reference import W, H, FRAMES, ONES, TWOS, clamp8  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "warp_stabilizer"
TOLERANCE = 2e-5  # document 25's default for a filter
FPS = 24
D = hypot(W, H) / 2
IDENTITY = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]

# The shake: a steady pan of 0.8 a frame to the right, with a jolt each frame, and for `turn` a
# small turn about the middle.
JX = (0.0, 1.0, -0.6, 0.7, -0.8, 0.4)
JY = (0.0, -0.7, 0.5, -0.4, 0.9, -0.3)
TURN = (0.0, 0.03, -0.02, 0.025, -0.03, 0.01)


def texture(u, v):
    t = 0.5 + 0.3 * sin(2 * pi * u / 11) + 0.15 * cos(2 * pi * v / 9 + u / 5)
    return (clamp8(255 * t), clamp8(90 + 100 * t), clamp8(230 - 150 * t), 255)


def shaken(k, turned):
    """Drawing k + 1: the scene point s shows at p = Rot(theta_k) (s - c) + c + o_k."""
    ox, oy = 0.8 * k + JX[k], JY[k]
    th = TURN[k] if turned else 0.0
    co, si = cos(th), sin(th)
    rows = []
    for y in range(H):
        row = []
        for x in range(W):
            px, py = x + 0.5 - W / 2 - ox, y + 0.5 - H / 2 - oy
            row.append(texture(co * px + si * py + W / 2, -si * px + co * py + H / 2))
        rows.append(row)
    return rows


DRAWINGS = {**{f"shake_{k + 1}": shaken(k, False) for k in range(FRAMES)},
            **{f"turn_{k + 1}": shaken(k, True) for k in range(FRAMES)},
            **{f"disc_{k + 1}": P.disc(k) for k in range(FRAMES)}}


def drawn(c, m):
    """The holder at composition frame m with the effects before Warp Stabilizer."""
    local = m - c["in"]
    number = next(s["drawing_number"] for s in c["spans"]
                  if s["start_frame"] <= local < s["end_frame_exclusive"])
    name = f"{c['asset']}_{number}"
    L.DRAWINGS[name] = DRAWINGS[name]
    p = L.decoded(name)
    return L.exposure(p, c["before"]) if c["before"] is not None else p


# --- the rule -------------------------------------------------------------------------------

def solve(a, b):
    """Gaussian elimination with partial pivoting; None when a pivot is below 1e-12."""
    n = len(b)
    m = [list(r) + [v] for r, v in zip(a, b)]
    for i in range(n):
        p = max(range(i, n), key=lambda r: abs(m[r][i]))
        if abs(m[p][i]) < 1e-12:
            return None
        m[i], m[p] = m[p], m[i]
        for r in range(i + 1, n):
            f = m[r][i] / m[i][i]
            for j in range(i, n + 1):
                m[r][j] -= f * m[i][j]
    out = [0.0] * n
    for i in range(n - 1, -1, -1):
        out[i] = (m[i][n] - sum(m[i][j] * out[j] for j in range(i + 1, n))) / m[i][i]
    return out


def apply(m, x, y):
    t = m[2][0] * x + m[2][1] * y + m[2][2]
    return (m[0][0] * x + m[0][1] * y + m[0][2]) / t, (m[1][0] * x + m[1][1] * y + m[1][2]) / t


def mul(a, b):
    m = [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    return [[v / m[2][2] for v in r] for r in m]


def inverse(m):
    (a, b, c), (d, e, f), (g, h, i) = m
    adj = [[e * i - f * h, c * h - b * i, b * f - c * e],
           [f * g - d * i, a * i - c * g, c * d - a * f],
           [d * h - e * g, b * g - a * h, a * e - b * d]]
    return [[v / adj[2][2] for v in r] for r in adj]


def fitted(method, preserve, points, weights):
    """One weighted fit of step 3, or None."""
    if sum(weights) < 4:
        return None
    if method == "position":
        sw = sum(weights)
        tx = sum(w * (X - x) for w, (x, y, X, Y) in zip(weights, points)) / sw
        ty = sum(w * (Y - y) for w, (x, y, X, Y) in zip(weights, points)) / sw
        return [[1.0, 0.0, tx], [0.0, 1.0, ty], [0.0, 0.0, 1.0]]
    rows = []
    for w, (x, y, X, Y) in zip(weights, points):
        if method == "position_scale_rotation":
            rows.append((w, (x, -y, 1.0, 0.0), X))
            rows.append((w, (y, x, 0.0, 1.0), Y))
        else:
            rows.append((w, (x, y, 1.0, 0.0, 0.0, 0.0, -x * X, -y * X), X))
            rows.append((w, (0.0, 0.0, 0.0, x, y, 1.0, -x * Y, -y * Y), Y))
    n = len(rows[0][1])
    a = [[sum(w * r[i] * r[j] for w, r, _ in rows) for j in range(n)] for i in range(n)]
    b = [sum(w * r[i] * v for w, r, v in rows) for i in range(n)]
    s = solve(a, b)
    if s is None:
        return None
    if method == "perspective":
        return [[s[0], s[1], s[2]], [s[3], s[4], s[5]], [s[6], s[7], 1.0]]
    ra, rb = s[0], s[1]
    if preserve:
        length = hypot(ra, rb)
        ra, rb = ra / length, rb / length
    return [[ra, -rb, s[2]], [rb, ra, s[3]], [0.0, 0.0, 1.0]]


PAIRS = {}


def pair(c, n, k):
    """Step 3's M_k."""
    vd = 100 if c["detailed_analysis"] == "on" else 50
    key = (c["asset"], c["before"], c["in"], str(c["spans"]), k, vd, c["method"],
           c["preserve_scale"])
    if key in PAIRS:
        return PAIRS[key]
    a, b = drawn(c, k), drawn(c, k + 1)
    if a["px"] == b["px"]:
        PAIRS[key] = IDENTITY
        return IDENTITY
    u = flow.estimate(a, b, {"vector_detail": vd})
    points, base = [], []
    for y in range(H):
        for x in range(W):
            i = y * W + x
            px, py = (x + 0.5 - W / 2) / D, (y + 0.5 - H / 2) / D
            points.append((px, py, px + u[0][i] / D, py + u[1][i] / D))
            base.append(a["px"][i][3])
    m, weights = None, base
    for _ in range(3):
        m = fitted(c["method"], c["preserve_scale"] == "on", points, weights)
        if m is None:
            break
        weights = []
        for w, (x, y, X, Y) in zip(base, points):
            fx, fy = apply(m, x, y)
            r = hypot(fx - X, fy - Y) * D
            weights.append(w / (1 + r * r))
    PAIRS[key] = m if m is not None else IDENTITY
    return PAIRS[key]


def towards(c, n, k):
    """Step 4's T_k."""
    t = IDENTITY
    for j in range(n, k):
        t = mul(pair(c, n, j), t)
    for j in range(n, k, -1):
        t = mul(inverse(pair(c, n, j - 1)), t)
    return t


def steadied(c, n):
    """Step 5's S."""
    n0, m1 = c["in"], FRAMES - 1
    if c["result"] == "no_motion":
        return towards(c, n, n0)
    r = floor(FPS * value_at(c["smoothness"], n) / 200 + 0.5)
    r = min(r, n - n0, m1 - n)
    if r == 0:
        return IDENTITY
    total, s = 0.0, [[0.0] * 3 for _ in range(3)]
    for k in range(n - r, n + r + 1):
        w = exp(-(k - n) ** 2 / (2 * (r / 2) ** 2))
        t = towards(c, n, k)
        total += w
        for i in range(3):
            for j in range(3):
                s[i][j] += w * t[i][j]
    return [[v / total for v in row] for row in s]


def fits(back, s):
    for cx, cy in ((-1, -1), (1, -1), (-1, 1), (1, 1)):
        x, y = apply(back, cx * s * W / 2 / D, cy * s * H / 2 / D)
        x, y = x * D + W / 2, y * D + H / 2
        if not (0 <= x <= W and 0 <= y <= H):
            return False
    return True


def stabilized(c, n):
    """Steps 5 to 7 at frame n."""
    steady = steadied(c, n)
    back = inverse(steady)
    framing = c["framing"]
    s = 1.0
    if not fits(back, 1.0):
        lo, hi = 0.0, 1.0
        for _ in range(60):
            mid = (lo + hi) / 2
            if fits(back, mid):
                lo = mid
            else:
                hi = mid
        s = lo
    z = 1.0
    if framing == "stabilize_crop_auto_scale":
        top = value_at(c["maximum_scale"], n) / 100
        m = value_at(c["action_safe_margin"], n) / 100
        z = top if s == 0 else min(max((1 - 2 * m) / s, 1.0), top)
    zz = z * value_at(c["additional_scale"], n) / 100
    o = drawn(c, n)
    if steady == IDENTITY and zz == 1:
        return o
    out = []
    for y in range(H):
        for x in range(W):
            qx = W / 2 + (x + 0.5 - W / 2) / zz
            qy = H / 2 + (y + 0.5 - H / 2) / zz
            if framing != "stabilize_only" and (abs(qx - W / 2) > s * W / 2
                                                or abs(qy - H / 2) > s * H / 2):
                out.append([0.0] * 4)
                continue
            sx, sy = apply(back, (qx - W / 2) / D, (qy - H / 2) / D)
            out.append(list(L.bilinear(o, sx * D + W / 2, sy * D + H / 2)))
    return L.pic(W, H, out)


# --- the cases ------------------------------------------------------------------------------

ADDED = {"result": "smooth_motion", "smoothness": 50, "method": "position_scale_rotation",
         "preserve_scale": "off", "framing": "stabilize_crop_auto_scale", "maximum_scale": 150,
         "action_safe_margin": 0, "additional_scale": 100, "detailed_analysis": "off"}
KEYS = tuple(ADDED)


def case(asset="shake", spans=ONES, start=0, shift=(0, 0), before=None, after=None,
         on="holder", **settings):
    c = dict(ADDED)
    c.update(settings)
    c.update({"asset": asset, "spans": spans, "in": start, "shift": shift, "before": before,
              "after": after, "on": on})
    return c


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        out = drawn(c, frame)
    else:
        out = stabilized(c, frame)
        if c["after"] is not None:
            out = L.exposure(out, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    dx, dy = c["shift"]
    p = drawn(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


ONLY = {"framing": "stabilize_only"}
CASES = {
    "FX-WS-001": ("As added: Smooth Motion at Smoothness 50, Position, Scale, Rotation, Stabilize, "
                  "Crop, Auto-scale, on the card shaken about a steady pan. Frames 0 and 5 are the "
                  "layer's ends, where the window closes to nothing, so they are as drawn; frames "
                  "2 and 3 are steadied and scaled up past their moving edges.", case(),
                  (0, 2, 3, 5)),
    "FX-WS-002": ("Stabilize Only: steadied, the edges the shake uncovers left clear.",
                  case(**ONLY), (2, 3)),
    "FX-WS-003": ("Stabilize, Crop: steadied, cut to the largest middle rectangle the shake never "
                  "uncovers, not scaled.", case(framing="stabilize_crop"), (2,)),
    "FX-WS-004": ("Method Position, Stabilize Only: moves only.",
                  case(method="position", **ONLY), (2,)),
    "FX-WS-005": ("The card also turned a little each frame, Position, Scale, Rotation, Stabilize "
                  "Only.", case(asset="turn", **ONLY), (2,)),
    "FX-WS-006": ("The turned card, Perspective, Stabilize Only.",
                  case(asset="turn", method="perspective", **ONLY), (2,)),
    "FX-WS-007": ("The turned card, Preserve Scale on, Stabilize Only.",
                  case(asset="turn", preserve_scale="on", **ONLY), (2,)),
    "FX-WS-008": ("No Motion, Stabilize Only: every frame held to the layer's first, the pan "
                  "taken out too.", case(result="no_motion", **ONLY), (1, 3)),
    "FX-WS-009": ("Smoothness 0: the window is one frame, the layer exactly as it is.",
                  case(smoothness=0), (2,)),
    "FX-WS-010": ("Smoothness 8: one frame either side, where 50 reaches as far as the layer "
                  "allows, two at frame 2.", case(smoothness=8, **ONLY), (2,)),
    "FX-WS-011": ("Maximum Scale 101: the scale stops short, and the cut edges show clear.",
                  case(maximum_scale=101), (2,)),
    "FX-WS-012": ("Action-Safe Margin 10: scaled less, since a tenth of each side may stay cut.",
                  case(action_safe_margin=10), (2,)),
    "FX-WS-013": ("Additional Scale 120 on Stabilize Only.", case(additional_scale=120, **ONLY),
                  (2,)),
    "FX-WS-014": ("Additional Scale keyed from 100 at frame 0 to 140 at frame 4: 120 at frame 2.",
                  case(additional_scale=keyed((0, 100), (4, 140)), **ONLY), (2,)),
    "FX-WS-015": ("Detailed Analysis on: the motion found with a vector for every pixel.",
                  case(detailed_analysis="on", **ONLY), (2,)),
    "FX-WS-016": ("Pixel Motion Blur's disc running steadily over nothing, Stabilize Only: only "
                  "its own pixels count, and a steady run is left nearly as it is.",
                  case(asset="disc", **ONLY), (2,)),
    "FX-WS-017": ("An Exposure of +1 before Warp Stabilizer: the frames it reads are brightened "
                  "too.", case(before=1), (2,)),
    "FX-WS-018": ("An Exposure of -1 after Warp Stabilizer darkens its picture.", case(after=-1),
                  (2,)),
    "FX-WS-019": ("The holder moved 3 right and 2 down: FX-WS-002 moved.",
                  case(shift=(3, 2), **ONLY), (2,)),
    "FX-WS-020": ("The layer's in point at frame 2: frame 2 is its first, as drawn; frame 3 has "
                  "one frame either side.", case(start=2, **ONLY), (2, 3)),
    "FX-WS-021": ("The effect on an adjustment layer above the holder: nothing changes, and the "
                  "frame says why.", case(on="adjust"), (2,)),
    "FX-WS-022": ("The shaken card on twos: the frames that repeat a drawing add no motion.",
                  case(spans=TWOS, **ONLY), (2, 3)),
}

INVALID = {
    "FX-WS-023": ("Smoothness 1001, above 1000.", case(smoothness=1001)),
    "FX-WS-024": ("Method written \"subspace_warp\", which this program leaves out.",
                  case(method="subspace_warp")),
    "FX-WS-025": ("Framing written \"stabilize_synthesize_edges\", which this program leaves out.",
                  case(framing="stabilize_synthesize_edges")),
    "FX-WS-026": ("Maximum Scale 99, below 100.", case(maximum_scale=99)),
    "FX-WS-027": ("Action-Safe Margin 51, above 50.", case(action_safe_margin=51)),
    "FX-WS-028": ("Additional Scale 0, below 1.", case(additional_scale=0)),
    "FX-WS-029": ("Result written \"maybe\".", case(result="maybe")),
    "FX-WS-030": ("Maximum Scale keyed to 2000 at frame 4.",
                  case(maximum_scale=keyed((0, 150), (4, 2000)))),
}


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.warp_stabilizer", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def project_json(fx, c):
    asset = c["asset"]
    holder = L.raster("holder", "asset-" + asset, spans=c["spans"], position=c["shift"],
                      in_frame=c["in"], out_frame=FRAMES)
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    comp = L.composition("comp-main", W, H, FRAMES, layers)
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-" + asset, "kind": "image_sequence", "name": asset,
                        "pattern": asset + "_####.png",
                        "frames": {str(k): f"media/{asset}_{k}.png" for k in range(1, FRAMES + 1)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [comp]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)), "frames": rendered}
        if c["on"] == "adjust":
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": plain(c, 0), "4": plain(c, 4)}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_warp_stabilizer.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    d = lambda k, **kw: plain(case(**kw), k)  # noqa: E731
    # The fit finds the shake: frame 1 to 2 moves by the pan plus the jolts' difference.
    m = pair(case(), 1, 1)
    tx, ty = m[0][2] * D, m[1][2] * D
    assert abs(tx - (0.8 + JX[2] - JX[1])) < 0.35 and abs(ty - (JY[2] - JY[1])) < 0.35, (tx, ty)
    m = pair(case(asset="turn"), 1, 1)
    assert abs(m[1][0] - sin(TURN[2] - TURN[1])) < 0.02, m
    # The ends of the layer, Smoothness 0 and the adjustment layer are exactly as drawn.
    assert c["FX-WS-001"]["0"] == d(0) and c["FX-WS-001"]["5"] == d(5)
    assert c["FX-WS-009"]["2"] == d(2)
    assert c["FX-WS-020"]["2"] == plain(case(start=2), 2)
    assert c["FX-WS-021"]["2"] == d(2)
    # Steadied frames differ from the drawings, and the framings from each other.
    for fx in ("FX-WS-001", "FX-WS-002", "FX-WS-003"):
        assert c[fx]["2"] != d(2), fx
    assert c["FX-WS-001"]["2"] != c["FX-WS-002"]["2"] != c["FX-WS-003"]["2"]
    clear = lambda px: sum(p[3] < 1e-9 for p in px)  # noqa: E731
    uncovered = lambda px: sum(p[3] < 0.999 for p in px)  # noqa: E731
    assert uncovered(c["FX-WS-002"]["2"]) > 0 and clear(c["FX-WS-003"]["2"]) > 0
    assert clear(c["FX-WS-001"]["2"]) == 0
    assert uncovered(c["FX-WS-011"]["2"]) > uncovered(c["FX-WS-001"]["2"])
    assert c["FX-WS-012"]["2"] != c["FX-WS-001"]["2"]
    # No Motion holds frame 3 close to frame 0 where both are drawn.
    held, first = c["FX-WS-008"]["3"], d(0)
    inner = [y * W + x for y in range(8, 24) for x in range(8, 24)]
    assert sum(abs(held[i][1] - first[i][1]) for i in inner) / len(inner) < 0.03
    # Methods and options each change the picture.
    assert c["FX-WS-004"]["2"] != c["FX-WS-002"]["2"]
    assert c["FX-WS-005"]["2"] != c["FX-WS-006"]["2"] != c["FX-WS-007"]["2"]
    assert c["FX-WS-010"]["2"] != c["FX-WS-002"]["2"]
    assert near(c["FX-WS-014"]["2"], c["FX-WS-013"]["2"])
    assert c["FX-WS-015"]["2"] != c["FX-WS-002"]["2"]
    assert c["FX-WS-017"]["2"] != c["FX-WS-001"]["2"]
    assert near(c["FX-WS-018"]["2"], [[p[0] / 2, p[1] / 2, p[2] / 2, p[3]]
                                      for p in c["FX-WS-001"]["2"]])
    moved = c["FX-WS-019"]["2"]
    assert all(moved[(y + 2) * W + x + 3] == c["FX-WS-002"]["2"][y * W + x]
               for y in range(H - 2) for x in range(W - 3))
    # A steady run is nearly left alone: the disc's middle moves under a pixel.
    mid = lambda px: sum((i % W) * p[3] for i, p in enumerate(px)) / sum(p[3] for p in px)  # noqa: E731,E501
    assert abs(mid(c["FX-WS-016"]["2"]) - mid(d(2, asset="disc"))) < 1.0
    assert c["FX-WS-022"]["2"] != c["FX-WS-002"]["2"]
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
