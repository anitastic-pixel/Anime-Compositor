"""Polygon Shards, worked a second way.

D-675 adds Polygon Shards (`core.polygon_shards`), after CycoreFX's CC Pixel Polly (Simulation):
the layer cut along a grid into triangles or squares that are thrown away from a force centre
in 3D, falling and spinning, seen through the composition camera or the effect's own eye. The
CycoreFX manual names the controls and says what each is for (Force, Gravity, Spinning, Force
Center, Direction Randomness, Speed Randomness, Grid Spacing, Object, Enable Depth Sort, Start
Time). It publishes no formula, so the rule, the units, the ranges and the settings when added
are ours. The layer is replaced by its shards. No state is carried from frame to frame
(document 20): every shard's flight is closed form from the start time, and every setting is
read as it is at the frame drawn, so a keyed force changes the whole flight, not only what
follows the key. There is no seed control, as CycoreFX's has none.

At composition frame f, the layer's in point i0, r frames a second, on a drawing w by h, every
setting as it is at f:

1. The time t = (f - i0) / r - `start_time` seconds. With t <= 0 the layer is drawn as it is.
2. The grid: s = `grid_spacing` pixels, cells (x_0, y_0) = (s i, s j) for i < ceil(w / s), j <
   ceil(h / s), numbered n = j ceil(w / s) + i. With `object` `polygon` or `textured_polygon` a
   cell is two shards, numbered 2n (the upper right triangle, a >= b) and 2n + 1 (the lower left,
   b >= a), (a, b) a point's place in its cell from (x_0, y_0); with `square` or
   `textured_square` one, numbered n. A shard's centre c is its triangle's centroid, (x_0 + 2s/3,
   y_0 + s/3) and (x_0 + s/3, y_0 + 2s/3), or the square's middle.
3. Shard k's randoms u_q = (U(0, k, 0, 0, q) + 1) / 2, U Noise's hash (D-119) at seed 0. From the
   force centre F = (`force_center_x` w, `force_center_y` h) / 100, d = c - F: the way in the
   layer's plane theta = atan2(d_y, d_x) (2 pi u_0 where d = 0) + (u_1 - 1/2) pi D, the tilt
   towards the eye phi = 45 + (u_2 - 1/2) 90 D degrees, D = `direction_randomness` / 100; the
   way thrown e = (cos phi cos theta, cos phi sin theta, -sin phi), Z away from the eye. The speed
   v = (`force` / 100) (h / 4) (h / (h + |d|)) (1 - (`speed_randomness` / 100) u_3) pixels a
   second: 100 throws a shard at the force centre a quarter of the layer's height a second, slower
   further out.
   Its place P = (c_x, c_y, 0) + v e t + (0, G t^2 / 2, 0), G = `gravity` h pixels a second a
   second, downwards.
4. It turns about its centre, about the axis (sqrt(1 - z^2) cos psi, sqrt(1 - z^2) sin psi, z), z =
   2 u_4 - 1, psi = 2 pi u_5, by `spinning` (1/2 + u_6) t degrees (Rodrigues' rotation, M): a point
   (a, b) of the shard is at P + M ((x_0 + a, y_0 + b, 0) - (c_x, c_y, 0)).
5. The eye: with the composition camera (the file has one), its place (a_x, a_y) and its distance
   A_l from the layer's plane in the layer's own pixels, as compose works them out (D-610);
   otherwise the effect's own, (w / 2, h / 2) and A_l = (w / 2) / tan(19.8 degrees). The eye is
   at E = (a_x, a_y, -A_l) and the layer's plane is drawn 1:1. A shard with a corner within one
   pixel of the eye's plane or behind it (A_l + z <= 1) is not drawn.
6. At a pixel centre (X, Y) the ray from E through (X, Y, 0), R = (X - a_x, Y - a_y, A_l), meets
   shard k's plane (normal N = M (0, 0, 1)) at Q = E + lambda R, lambda = N.(P - E) / N.R, when
   |N.R| > 1e-9 |R| and lambda > 0. The point there of the drawing, (X_0, Y_0) = c + the first two
   of M^T (Q - P), at (a, b) in its cell. Its distance inside e = min over the shard's edges
   (a, b, s - a, s - b for a square; b, s - a and (a - b) / sqrt 2 for the upper right triangle;
   a, s - b and (b - a) / sqrt 2 for the lower left) and over the drawing's (X_0, Y_0, w - X_0,
   h - Y_0); it covers cov = clamp(e A_l / (A_l + Q_z) + 1/2, 0, 1).
7. Its colour there: `textured_*`, the drawing at (X_0, Y_0) by the bilinear read below; else the
   shard's own, the mean of the drawing's premultiplied pixels whose centres are inside its
   triangle or square and the drawing (on the diagonal, the lower left triangle's), clear when
   there is none.
8. Every pixel starts clear and takes each covering shard over it, colour times cov over (1 -
   alpha cov): far to near by the centre's Z (P_z larger first, then by number) with
   `depth_sort` on, by number with it off.

The bilinear read, premultiplied and linear: fx = clamp(X_0 - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge.

`force` 0 to 1000, 300 when added; `gravity` -100 to 100 times the height a second a second, 1;
`spinning` -3600 to 3600 degrees a second, 60; `force_center_x` and `_y` -1000 to 1000 per cent,
50 and 50; `direction_randomness` and `speed_randomness` 0 to 100, 25 and 25; `grid_spacing` 2
to 1000 pixels, 20, which a draft scales; `start_time` 0 to 3600 seconds after the in point, 0;
`object` `polygon`, `textured_polygon`, `square` or `textured_square`, textured_polygon;
`depth_sort` `off` or `on`, on. Every number keyable. The layer does not grow: shards thrown past
its edge are cut there.

**This file never runs the build's code path.** It tries every shard at every pixel, where the
build skips the shards out of reach, in double precision from the drawings' 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size: Bubbles' colour
field, or the field with its right half clear. The projects go into `Fixtures/polygon_shards`,
the expected frames into `Fixtures/polygon_shards/expected_polygon_shards.json`. Every case is
"not built yet" until B-554 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/polygon_shards_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
import smooth_reference as S  # noqa: E402
import bubbles_reference as BU  # noqa: E402

W, H, FPS = 48, 32, 24
assert (BU.W, BU.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "polygon_shards"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.polygon_shards"
SETTINGS = [("force", 0, 1000, 300), ("gravity", -100, 100, 1), ("spinning", -3600, 3600, 60),
            ("force_center_x", -1000, 1000, 50), ("force_center_y", -1000, 1000, 50),
            ("direction_randomness", 0, 100, 25), ("speed_randomness", 0, 100, 25),
            ("grid_spacing", 2, 1000, 20), ("start_time", 0, 3600, 0)]
WORDS = [("object", ("polygon", "textured_polygon", "square", "textured_square"),
          "textured_polygon"),
         ("depth_sort", ("off", "on"), "on")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
EYE = 1 / math.tan(math.radians(19.8))  # the own eye's distance over half the width


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def rotation(axis, deg):
    x, y, z = axis
    a = math.radians(deg)
    co, si = math.cos(a), math.sin(a)
    t = 1 - co
    return [[co + x * x * t, x * y * t - z * si, x * z * t + y * si],
            [y * x * t + z * si, co + y * y * t, y * z * t - x * si],
            [z * x * t - y * si, z * y * t + x * si, co + z * z * t]]


def shards(c, f, src, view, w=W, h=H):
    """Steps 1 to 5: the drawn shards in drawing order, each (k, P, M, (x_0, y_0), part, colour),
    part 0 the upper right triangle, 1 the lower left, 2 a square; colour None when textured.
    None when t <= 0."""
    v = settings(c, f)
    t = (f - c["in_frame"]) / c["rate"] - v["start_time"]
    if t <= 0:
        return None
    s = v["grid_spacing"]
    ax, ay, al = view
    cols, rows = math.ceil(w / s), math.ceil(h / s)
    tri = c["object"] in ("polygon", "textured_polygon")
    textured = c["object"].startswith("textured")
    fx, fy = v["force_center_x"] / 100 * w, v["force_center_y"] / 100 * h
    dr, sr = v["direction_randomness"] / 100, v["speed_randomness"] / 100
    out = []
    for n in range(cols * rows):
        i, j = n % cols, n // cols
        x0, y0 = s * i, s * j
        for part in ((0, 1) if tri else (2,)):
            k = 2 * n + part if tri else n
            cx, cy = ((x0 + 2 * s / 3, y0 + s / 3), (x0 + s / 3, y0 + 2 * s / 3),
                      (x0 + s / 2, y0 + s / 2))[part]
            corners = (((x0, y0), (x0 + s, y0), (x0 + s, y0 + s)),
                       ((x0, y0), (x0 + s, y0 + s), (x0, y0 + s)),
                       ((x0, y0), (x0 + s, y0), (x0 + s, y0 + s), (x0, y0 + s)))[part]
            uq = [(U(0, k, 0, 0, q) + 1) / 2 for q in range(7)]
            dx, dy = cx - fx, cy - fy
            dist = math.hypot(dx, dy)
            theta = (math.atan2(dy, dx) if dist > 0 else 2 * math.pi * uq[0]) \
                + (uq[1] - 0.5) * math.pi * dr
            phi = math.radians(45 + (uq[2] - 0.5) * 90 * dr)
            e = (math.cos(phi) * math.cos(theta), math.cos(phi) * math.sin(theta), -math.sin(phi))
            sp = v["force"] / 100 * (h / 4) * (h / (h + dist)) * (1 - sr * uq[3])
            g = v["gravity"] * h
            P = (cx + sp * e[0] * t, cy + sp * e[1] * t + g * t * t / 2, sp * e[2] * t)
            z = 2 * uq[4] - 1
            psi = 2 * math.pi * uq[5]
            rr = math.sqrt(max(0.0, 1 - z * z))
            M = rotation((rr * math.cos(psi), rr * math.sin(psi), z),
                         v["spinning"] * (0.5 + uq[6]) * t)
            ok = True
            for (px, py) in corners:
                q = (px - cx, py - cy)
                vz = P[2] + M[2][0] * q[0] + M[2][1] * q[1]
                if al + vz <= 1:
                    ok = False
            if not ok:
                continue
            colour = None
            if not textured:
                acc, cnt = [0.0] * 4, 0
                for yy in range(int(y0), min(h, int(y0 + s) + 1)):
                    for xx in range(int(x0), min(w, int(x0 + s) + 1)):
                        a, b = xx + 0.5 - x0, yy + 0.5 - y0
                        if not (0 <= a <= s and 0 <= b <= s):
                            continue
                        if part == 0 and not b < a:
                            continue
                        if part == 1 and not b >= a:
                            continue
                        p = src[yy * w + xx]
                        acc = [acc[q] + p[q] for q in range(4)]
                        cnt += 1
                colour = [q / cnt for q in acc] if cnt else [0.0] * 4
            out.append((k, P, M, (x0, y0), (cx, cy), part, colour))
    if c["depth_sort"] == "on":
        out.sort(key=lambda sh: (-sh[1][2], sh[0]))
    return out


def draw(src, live, s, view, w=W, h=H):
    """Steps 6 to 8."""
    ax, ay, al = view
    r2 = math.sqrt(2)
    out = []
    for py in range(h):
        for px in range(w):
            X, Y = px + 0.5, py + 0.5
            R = (X - ax, Y - ay, al)
            rl = math.sqrt(sum(q * q for q in R))
            acc = [0.0] * 4
            for (k, P, M, (x0, y0), (cx, cy), part, colour) in live:
                N = (M[0][2], M[1][2], M[2][2])
                den = N[0] * R[0] + N[1] * R[1] + N[2] * R[2]
                if not abs(den) > 1e-9 * rl:
                    continue
                pe = (P[0] - ax, P[1] - ay, P[2] + al)
                lam = (N[0] * pe[0] + N[1] * pe[1] + N[2] * pe[2]) / den
                if not lam > 0:
                    continue
                Q = (ax + lam * R[0], ay + lam * R[1], -al + lam * R[2])
                dq = (Q[0] - P[0], Q[1] - P[1], Q[2] - P[2])
                ls = M[0][0] * dq[0] + M[1][0] * dq[1] + M[2][0] * dq[2]
                lt = M[0][1] * dq[0] + M[1][1] * dq[1] + M[2][1] * dq[2]
                X0, Y0 = cx + ls, cy + lt
                a, b = X0 - x0, Y0 - y0
                if part == 0:
                    e = min(b, s - a, (a - b) / r2)
                elif part == 1:
                    e = min(a, s - b, (b - a) / r2)
                else:
                    e = min(a, b, s - a, s - b)
                e = min(e, X0, Y0, w - X0, h - Y0)
                cov = clamp(e * al / (al + Q[2]) + 0.5, 0, 1)
                if cov == 0:
                    continue
                col = colour if colour is not None else BU.bilinear(src, X0, Y0, w, h)
                acc = [col[q] * cov + acc[q] * (1 - col[3] * cov) for q in range(4)]
            out.append(acc)
    return out


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    return all(c[name] in words for name, words, _ in WORDS)


def view_of(c):
    """The eye in the layer's own pixels: the composition camera as compose works it out, or
    the effect's own."""
    if c["camera"] is None:
        return (W / 2, H / 2, W / 2 * EYE)
    x, y, depth, zoom = c["camera"]
    assert zoom == -depth, "only cameras that draw the layer's plane 1:1"
    return (x - c["shift"], y, 0 - depth)


def move_of(c):
    move = c["shift"]
    if c["camera"] is not None:
        assert c["camera"][1] == H / 2 and c["camera"][0] == int(c["camera"][0])
        move += int(W / 2 - c["camera"][0])
    return move


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c) or f < c["in_frame"]:
        return BU.frame(src, move_of(c))
    view = view_of(c)
    live = shards(c, f, src, view)
    if live is None:
        return BU.frame(src, move_of(c))
    return BU.frame(draw(src, live, settings(c, f)["grid_spacing"], view), move_of(c))


def plain(c):
    return BU.frame(BU.working(c["drawing"]), move_of(c))


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, camera=None, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift, camera=camera)
    return c


# A close look: a grid of 8 pixels, six cells by four, 48 triangles.
SMALL = dict(grid_spacing=8)


def small(**kw):
    return case(**{**SMALL, **kw})


CASES = {
    "FX-POLYGON-SHARDS-001": ("As added: a grid of 20 pixels, three cells by two, each cut into "
                              "two triangles thrown out from the middle and towards the eye, "
                              "falling and spinning; frame 0 the layer as it is.",
                              case(), (0, 6, 12, 24)),
    "FX-POLYGON-SHARDS-002": ("A close look: a grid of 8 pixels, 48 triangles; frame 3 just "
                              "cracked apart, frame 12 flying.", small(), (3, 12)),
    "FX-POLYGON-SHARDS-003": ("Object Polygon: each triangle its own flat colour.",
                              small(object="polygon"), (0, 6)),
    "FX-POLYGON-SHARDS-004": ("Object Textured Square: squares carrying the picture.",
                              small(object="textured_square"), (6,)),
    "FX-POLYGON-SHARDS-005": ("Object Square: squares, each its own flat colour.",
                              small(object="square"), (6,)),
    "FX-POLYGON-SHARDS-006": ("Force 0: the shards only fall and spin.", small(force=0), (12,)),
    "FX-POLYGON-SHARDS-007": ("Gravity 0, Force 100: thrown out slowly, not falling.",
                              small(gravity=0, force=100), (12,)),
    "FX-POLYGON-SHARDS-008": ("Gravity -1: the shards float up.", small(gravity=-1), (12,)),
    "FX-POLYGON-SHARDS-009": ("Spinning 0: the shards fly flat, facing the eye.",
                              small(spinning=0), (12,)),
    "FX-POLYGON-SHARDS-010": ("Spinning 720: the shards tumble fast.", small(spinning=720),
                              (6,)),
    "FX-POLYGON-SHARDS-011": ("The force centre at the top left corner: thrown down and right.",
                              small(force_center_x=0, force_center_y=0), (6,)),
    "FX-POLYGON-SHARDS-012": ("Direction Randomness 0: each thrown straight out from the "
                              "centre, 45 degrees towards the eye.",
                              small(direction_randomness=0), (6,)),
    "FX-POLYGON-SHARDS-013": ("Direction Randomness 100: thrown every which way, from flat "
                              "across the layer to straight at the eye.",
                              small(direction_randomness=100), (6,)),
    "FX-POLYGON-SHARDS-014": ("Speed Randomness 100: speeds from none to full.",
                              small(speed_randomness=100), (12,)),
    "FX-POLYGON-SHARDS-015": ("Depth Sort off: drawn in grid order, later shards over earlier "
                              "ones whatever their depth.", small(depth_sort="off"), (12,)),
    "FX-POLYGON-SHARDS-016": ("Start Time 0.5 seconds: frame 6 the layer as it is, frame 18 "
                              "FX-POLYGON-SHARDS-002's frame 6.", small(start_time=0.5),
                              (6, 18)),
    "FX-POLYGON-SHARDS-017": ("The picture with its right half clear: the right half's shards "
                              "are clear.", small(drawing="half"), (6,)),
    "FX-POLYGON-SHARDS-018": ("The half picture, Object Polygon: the right half's triangles "
                              "clear, the left half's flat.",
                              small(drawing="half", object="polygon"), (6,)),
    "FX-POLYGON-SHARDS-019": ("A grid of 7 pixels, which fits neither side: the last column "
                              "and row of cells cut at the layer's edge.",
                              case(grid_spacing=7), (6,)),
    "FX-POLYGON-SHARDS-020": ("Force keyed from 0 at frame 0 to 600 at frame 24: frame 12 is "
                              "Force 300 at frame 12, the whole flight from the key's value "
                              "then.", small(force=keyed((0, 0), (24, 600))), (12,)),
    "FX-POLYGON-SHARDS-021": ("FX-POLYGON-SHARDS-002 moved 3 pixels right: the same, moved.",
                              small(shift=3), (12,)),
    "FX-POLYGON-SHARDS-022": ("FX-POLYGON-SHARDS-002 with the layer starting at frame 6: frame "
                              "9 is 002's frame 3 and frame 18 its 12.", small(in_frame=6),
                              (9, 18)),
    "FX-POLYGON-SHARDS-023": ("Through a composition camera 100 pixels away over the middle: "
                              "the shards coming at the eye grow less than through the "
                              "effect's own eye 67 pixels away.",
                              small(camera=(24, 16, -100, 100)), (12,)),
    "FX-POLYGON-SHARDS-024": ("The composition camera 6 pixels to the right: the frame moved 6 "
                              "pixels left, the shards seen from the side.",
                              small(camera=(30, 16, -100, 100)), (12,)),
    "FX-POLYGON-SHARDS-025": ("FX-POLYGON-SHARDS-002 at 30 frames a second: frame 12 is two "
                              "fifths of a second in, not half.", small(rate=30), (12,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-POLYGON-SHARDS-026": ("Force -1, below 0.", small(force=-1)),
    "FX-POLYGON-SHARDS-027": ("Gravity 101, above 100.", small(gravity=101)),
    "FX-POLYGON-SHARDS-028": ("Spinning 3601, above 3600.", small(spinning=3601)),
    "FX-POLYGON-SHARDS-029": ("Force Center X 1001, above 1000.", small(force_center_x=1001)),
    "FX-POLYGON-SHARDS-030": ("Direction Randomness 101, above 100.",
                              small(direction_randomness=101)),
    "FX-POLYGON-SHARDS-031": ("Speed Randomness -1, below 0.", small(speed_randomness=-1)),
    "FX-POLYGON-SHARDS-032": ("Grid Spacing 1, below 2.", small(grid_spacing=1)),
    "FX-POLYGON-SHARDS-033": ("Start Time -1, below 0.", small(start_time=-1)),
    "FX-POLYGON-SHARDS-034": ("An object written \"triangle\".", small(object="triangle")),
    "FX-POLYGON-SHARDS-035": ("A depth sort written \"yes\".", small(depth_sort="yes")),
    "FX-POLYGON-SHARDS-036": ("Force keyed to 2000 at frame 4, above 1000.",
                              small(force=keyed((0, 300), (4, 2000)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    frames = 36
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    comp = {
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
    }
    if c["camera"] is not None:
        x, y, depth, zoom = c["camera"]
        comp["camera"] = {"position": prop([x, y]), "depth": prop(depth), "zoom": prop(zoom)}
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [comp],
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
        src = BU.working(c["drawing"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {len(shards(c, int(f), src, view_of(c)) or [])} shards, "
            f"{sum(p[3] > 0 for p in px)} covered"
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
    (OUT / "expected_polygon_shards.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx.rsplit("-", 1)[1]: v["frames"] for fx, v in expected["cases"].items()}
    src = BU.working("field")
    layer = BU.frame(src, 0)
    assert c["001"]["0"] == layer and c["003"]["0"] == layer
    assert c["016"]["6"] == layer and c["016"]["18"] == render(small(), 6)
    two = c["002"]
    assert two["3"] != layer and two["12"] != two["3"]
    assert c["020"]["12"] == render(small(force=300), 12)
    assert c["022"]["9"] == two["3"] and c["022"]["18"] == two["12"]
    moved = c["021"]["12"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two["12"][y * W:y * W + W - 3]
    for n in (6, 7, 8, 9, 14, 15, 23, 24, 25):
        assert c["%03d" % n]["12"] != two["12"], n
    for n in (3, 4, 5, 10, 11, 12, 13, 17, 18):
        assert c["%03d" % n]["6"] != render(small(), 6), n
    assert c["017"]["6"] != c["018"]["6"]
    # Depth sort matters: some pixel covered by two shards whose order flips.
    live = shards(small(), 12, src, view_of(small()))
    assert [sh[0] for sh in live] != sorted(sh[0] for sh in live)
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
