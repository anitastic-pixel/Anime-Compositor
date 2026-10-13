"""Reshape, worked a second way.

D-606 adds `core.reshape`, after After Effects' Reshape (Distort): the drawing inside one of the
layer's masks is pulled into the shape of another, the rest of the drawing following, held still
at a third mask or at the drawing's own edge. After Effects names its controls and publishes no
formula; the rule below is this program's own, and nothing is ported. D-607's Mask Smear
(`tools/mask_smear_reference.py`) uses the same field with its destination worked from the source.

The settings. `source_mask`, `destination_mask` and `boundary_mask`, 0 to 1000, the mask
numbered so (1 the first, its floor taken), 0 for none; 0, 0 and 0 when added. `percent`, how
far along the way from the source shape to the destination shape the drawing is pulled, 0 to
100, 100 when added. `elasticity`, how far each edge's pull reaches into the drawing: `stiff`,
`less_stiff`, `below_normal`, `normal` (when added), `above_normal`, `super_fluid` or `liquid`.
Every number is keyable.

The masks are chosen as Fill chooses them (D-418): the layer's masks at the frame, each as
document 19 flattens it, in the drawing's own space, a mask counting when it is switched on and
has two points or more, whatever its own mode. A source or destination of 0 draws the drawing
as it is and says nothing; a boundary of 0 is the drawing's own rectangle, (0, 0), (W, 0),
(W, H), (0, H). A mask named and not found draws the drawing as it is and says
EFFECT_PATH_MISSING every frame.

The rule.

1. Each outline is resampled to K = 64 points equally spaced along it, closed, starting at its
   first point and going its own way: the outline's pieces (point i to point i + 1, the last to
   the first) have lengths l_i and the whole length L (summed in order); point k (0 to K - 1)
   lies at k L / K along it, on the piece where the lengths before it, c, satisfy
   c <= k L / K < c + l_i (a piece of no length skipped; the last piece takes what is left), at
   a + (b - a) u, u = (k L / K - c) / l_i (0 on a piece of no length). An outline of no length
   is K copies of its first point.
2. Where the source and destination outlines wind opposite ways (their shoelace sums, x_i
   y_(i+1) - x_(i+1) y_i summed in order, of opposite signs), the destination is turned round
   first, keeping its first point: 0, n - 1, n - 2, ... 1. Then point k of the one answers point
   k of the other.
3. With t = `percent` / 100, source point S_k is pulled to T_k = S_k + t (D_k - S_k), and the
   drawing there is read from S_k: a step v_k = S_k - T_k. The boundary's K points B_k are held:
   a step of (0, 0).
4. The step at the place p is 0 outside the boundary (ADR-016's even-odd rule at p itself) and
   inside it the inverse-distance blend of every T_k's and B_k's step, in that order: with
   d^2 = (p - q)_x^2 + (p - q)_y^2, w = (d^2)^(-e / 2), the step sum(w v) / sum(w); a place on
   a point exactly (d^2 = 0) takes that point's step, the first such. e is 1, 1.5, 2, 2.5, 3,
   4 or 5 from `stiff` to `liquid`: a low e spreads each pull broadly, a high one keeps it near
   its edge.
5. The steps are worked at each corner of a grid of gx by gy cells over the drawing,
   gx = min(256, ceil(W / 2)), gy = min(256, ceil(H / 2)), and the frame drawn as
   `tools/mesh_reference.py` (D-601) draws a backward field, the whole share, no offset.

With `percent` 0 the drawing is untouched.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says, with masks of mode None (which leave the drawing as it is). The
drawing goes into `Fixtures/reshape/media`, the projects into `Fixtures/reshape`, and the
expected frames into `Fixtures/reshape/expected_reshape.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/reshape_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import bulge_reference as B  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402
from mask_reference import flatten, points_at, points, mask_json, point_inside  # noqa: E402
from mask_reference import keyed as moving_path  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "reshape"
TOLERANCE = 2e-5  # document 25's default for a filter
K = 64
ELASTICITY = {"stiff": 1.0, "less_stiff": 1.5, "below_normal": 2.0, "normal": 2.5,
              "above_normal": 3.0, "super_fluid": 4.0, "liquid": 5.0}
NAMES = ("source_mask", "destination_mask", "boundary_mask", "percent", "elasticity")
RANGES = {"source_mask": (0, 1000), "destination_mask": (0, 1000), "boundary_mask": (0, 1000),
          "percent": (0, 100)}


# --- the rule (D-607's Smear imports it) ----------------------------------------------------

def resample(outline, k=K):
    n = len(outline)
    pieces = [(outline[i], outline[(i + 1) % n]) for i in range(n)]
    lengths = [math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in pieces]
    total = 0.0
    for l in lengths:
        total += l
    out = []
    i, c = 0, 0.0
    for m in range(k):
        s = m * total / k
        while i < n - 1 and c + lengths[i] <= s:
            c += lengths[i]
            i += 1
        a, b = pieces[i]
        u = (s - c) / lengths[i] if lengths[i] > 0 else 0.0
        out.append((a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u))
    return out


def winding(outline):
    n = len(outline)
    s = 0.0
    for i in range(n):
        (x0, y0), (x1, y1) = outline[i], outline[(i + 1) % n]
        s += x0 * y1 - x1 * y0
    return s


def turned(outline):
    return [outline[0]] + outline[:0:-1]


def field(src, dst, bound, t, e, w, h):
    """The steps at the grid's corners, from the resampled source and destination points and
    the boundary outline (None for the drawing's rectangle)."""
    gx, gy = min(256, math.ceil(w / 2)), min(256, math.ceil(h / 2))
    held = resample(bound if bound is not None else [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)])
    anchors = []
    for s, d in zip(src, dst):
        q = (s[0] + t * (d[0] - s[0]), s[1] + t * (d[1] - s[1]))
        anchors.append((q, (s[0] - q[0], s[1] - q[1])))
    anchors += [(b, (0.0, 0.0)) for b in held]
    steps = []
    for j in range(gy + 1):
        for i in range(gx + 1):
            p = (i * w / gx, j * h / gy)
            if bound is not None and not point_inside(bound, p[0], p[1]):
                steps.append((0.0, 0.0))
                continue
            nx = ny = den = 0.0
            hit = None
            for q, v in anchors:
                dx, dy = p[0] - q[0], p[1] - q[1]
                d2 = dx * dx + dy * dy
                if d2 == 0.0:
                    hit = v
                    break
                wt = d2 ** (-e / 2)
                nx += wt * v[0]
                ny += wt * v[1]
                den += wt
            steps.append(hit if hit is not None else (nx / den, ny / den))
    return steps, gx, gy


def reshape_points(source, destination):
    """The source and destination outlines resampled, the destination turned to wind the
    source's way."""
    if winding(source) * winding(destination) < 0:
        destination = turned(destination)
    return resample(source), resample(destination)


# --- the cases ------------------------------------------------------------------------------

def none(pts, **more):
    return {"points": pts, "mode": "none", **more}


SQUARE = points([4, 3], [8, 3], [8, 7], [4, 7])
RIGHT = points([7, 3], [11, 3], [11, 7], [7, 7])          # SQUARE three pixels right
RIGHT_BACK = points([7, 3], [7, 7], [11, 7], [11, 3])     # the same, drawn the other way round
TALL = points([5, 1], [7, 1], [7, 9], [5, 9])
C4 = 0.5522847498307936
RING = points([8, 2.5], [10.5, 5], [8, 7.5], [5.5, 5], handles=[  # a circle 5 across, curved
    ([-2.5 * C4, 0], [2.5 * C4, 0]), ([0, -2.5 * C4], [0, 2.5 * C4]),
    ([2.5 * C4, 0], [-2.5 * C4, 0]), ([0, 2.5 * C4], [0, -2.5 * C4])])
FENCE = points([3, 1], [13, 1], [13, 9], [3, 9])          # the boundary, clear of columns 0-2, 13-15
LOW = points([4, 5], [8, 5], [8, 9], [4, 9])              # SQUARE two rows lower
MASKS = [none(SQUARE), none(RIGHT), none(FENCE)]


def case(masks=None, source_mask=1, destination_mask=2, boundary_mask=0, percent=100,
         elasticity="normal", shift=0):
    return {"drawing": "stripes", "shift": shift, "masks": MASKS if masks is None else masks,
            "source_mask": source_mask, "destination_mask": destination_mask,
            "boundary_mask": boundary_mask, "percent": percent, "elasticity": elasticity}


MISSING_PATH = "missing"


def chosen(c, k, frame_no):
    """The outline of the mask `k` names; None for 0; MISSING_PATH when it is not there."""
    i = math.floor(held(c, k, frame_no)) - 1
    if i < 0:
        return None
    if i >= len(c["masks"]):
        return MISSING_PATH
    m = c["masks"][i]
    pts = points_at(m, frame_no)
    if not m.get("enabled", True) or len(pts) < 2:
        return MISSING_PATH
    return flatten(pts)


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def reshape(layer, c, frame_no):
    src, dst, bound = (chosen(c, k, frame_no) for k in
                       ("source_mask", "destination_mask", "boundary_mask"))
    if MISSING_PATH in (src, dst, bound):
        return layer, True
    t = held(c, "percent", frame_no) / 100
    if src is None or dst is None or t == 0:
        return layer, False
    a, b = reshape_points(src, dst)
    steps, gx, gy = field(a, b, bound, t, ELASTICITY[c["elasticity"]], W, H)
    return M.draw_field(layer, steps, gx, gy, W, H), False


def render(c, frame_no):
    return frame(reshape(B.drawn_layer(c["drawing"]), c, frame_no)[0], c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-RESHAPE-001": ("The settings as they start: no source and no destination mask: the "
                       "drawing, untouched.",
                       case(source_mask=0, destination_mask=0), [0]),
    "FX-RESHAPE-002": ("Source mask 1, a square from (4, 3) to (8, 7); destination mask 2, the "
                       "same square three pixels right; Percent 100, Elasticity Normal: the "
                       "stripes in the square pulled right, the drawing's edges held.",
                       case(), [0]),
    "FX-RESHAPE-003": ("Destination a tall strip from (5, 1) to (7, 9): the square squeezed "
                       "across and stretched down.",
                       case(masks=[none(SQUARE), none(TALL)]), [0]),
    "FX-RESHAPE-004": ("FX-RESHAPE-002 at Percent 50: half way.", case(percent=50), [0]),
    "FX-RESHAPE-005": ("FX-RESHAPE-002 with boundary mask 3, (3, 1) to (13, 9): nothing outside "
                       "it moves, so columns 0 to 1 and 14 to 15 are the drawing's own.",
                       case(boundary_mask=3), [0]),
    "FX-RESHAPE-006": ("FX-RESHAPE-002 at Elasticity Stiff: the pull spread wide.",
                       case(elasticity="stiff"), [0]),
    "FX-RESHAPE-007": ("FX-RESHAPE-002 at Elasticity Liquid: the pull kept near the edges.",
                       case(elasticity="liquid"), [0]),
    "FX-RESHAPE-008": ("The destination drawn the other way round: turned to wind the source's "
                       "way first, so FX-RESHAPE-002's frame.",
                       case(masks=[none(SQUARE), none(RIGHT_BACK)]), [0]),
    "FX-RESHAPE-009": ("Destination a curved circle 5 across about (8, 5): the square rounded.",
                       case(masks=[none(SQUARE), none(RING)]), [0]),
    "FX-RESHAPE-010": ("Percent keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the "
                       "drawing, frame 2 FX-RESHAPE-004's, frame 4 FX-RESHAPE-002's.",
                       case(percent=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-RESHAPE-011": ("FX-RESHAPE-002 with the layer moved three pixels right: the masks are "
                       "the layer's, so all of it moves with the layer.", case(shift=3), [0]),
    "FX-RESHAPE-012": ("The destination's path keyed from three pixels right at frame 0 to two "
                       "rows lower at frame 4: the pull turns downward.",
                       case(masks=[none(SQUARE), {**moving_path((0, RIGHT), (4, LOW)),
                                                  "mode": "none"}]), [0, 2, 4]),
    "FX-RESHAPE-013": ("Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that "
                       "passes its end: held at 100 at frame 2, the same as frame 4.",
                       case(percent=keyed((0, 20, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-RESHAPE-014": ("Source and destination the same mask: nothing to pull, the drawing as "
                       "it is.", case(destination_mask=1), [0]),
}

MISSING = {
    "FX-RESHAPE-015": ("Source mask 4, of three.", case(source_mask=4)),
    "FX-RESHAPE-016": ("Destination mask 2, switched off.",
                       case(masks=[none(SQUARE), none(RIGHT, enabled=False)])),
    "FX-RESHAPE-017": ("Boundary mask 3 with two masks.",
                       case(masks=[none(SQUARE), none(RIGHT)], boundary_mask=3)),
}

INVALID = {
    "FX-RESHAPE-018": ("Elasticity \"rubber\", not one of the seven.", case(elasticity="rubber")),
    "FX-RESHAPE-019": ("Percent 101, above 100.", case(percent=101)),
    "FX-RESHAPE-020": ("Source mask -1, below 0.", case(source_mask=-1)),
    "FX-RESHAPE-021": ("Boundary mask 1001, above 1000.", case(boundary_mask=1001)),
    "FX-RESHAPE-022": ("Percent keyed to 101 at frame 4, above 100.",
                       case(percent=keyed((0, 100), (4, 101)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c, type_id="core.reshape", names=NAMES, words=("elasticity",)):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    layer = comp["layers"][0]
    t = layer["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    # In the order the build writes a layer, `masks` where `mask` was.
    layer = {k: v for k, v in layer.items() if k != "mask"}
    items = list(layer.items())
    items.insert(list(layer).index("matte"),
                 ("masks", [mask_json(m, i + 1) for i, m in enumerate(c["masks"])]))
    layer = dict(items)
    layer["effects"] = [{
        "instance_id": "fx-0-0", "type_id": type_id, "enabled": True,
        "parameters": {k: (c[k] if k in words else setting_json(c[k])) for k in names}}]
    comp["layers"][0] = layer
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_bytes((json.dumps(project_json(fx, c), indent=2) + "\n").encode("utf-8"))
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
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
        says += (" Nothing to reshape: the layer is drawn without the effect, which is kept as "
                 "written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert reshape(B.drawn_layer(c["drawing"]), c, 0)[1] and render(c, 0) == before
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

    (OUT / "expected_reshape.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    at = lambda px, x, y: px[y * W + x]  # noqa: E731

    # Resampling: a 4 by 4 square's 64 points a quarter apart, starting at its first corner.
    r = resample([(0, 0), (4, 0), (4, 4), (0, 4)])
    assert len(r) == 64 and r[0] == (0, 0) and near(r[1], (0.25, 0)) and near(r[16], (4, 0))
    assert resample([(2, 2), (2, 2)]) == [(2.0, 2.0)] * 64
    assert winding([(0, 0), (4, 0), (4, 4)]) > 0 > winding(turned([(0, 0), (4, 0), (4, 4)]))

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx] or "frame_warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    pulled = c["FX-RESHAPE-002"]["0"]
    assert c["FX-RESHAPE-001"]["0"] == drawn and c["FX-RESHAPE-014"]["0"] == drawn
    looks = [c[f"FX-RESHAPE-{n:03}"]["0"] for n in (2, 3, 4, 5, 6, 7, 9)]
    for i, a in enumerate(looks):
        assert not same(a, drawn), i
        for b in looks[i + 1:]:
            assert not same(a, b), i
    fenced = c["FX-RESHAPE-005"]["0"]
    for y in range(H):
        for x in (0, 1, 14, 15):
            assert at(fenced, x, y) == at(drawn, x, y), (x, y)
    assert same(c["FX-RESHAPE-008"]["0"], pulled)
    keyed_on = c["FX-RESHAPE-010"]
    assert keyed_on["0"] == drawn and same(keyed_on["2"], c["FX-RESHAPE-004"]["0"])
    assert same(keyed_on["4"], pulled)
    assert same(c["FX-RESHAPE-011"]["0"], frame(reshape(B.drawn_layer("stripes"), case(), 0)[0], 3))
    moving = c["FX-RESHAPE-012"]
    assert same(moving["0"], pulled) and not same(moving["4"], pulled)
    eased = c["FX-RESHAPE-013"]
    assert value_at(keyed((0, 20, OVERSHOOT), (4, 100)), 2) > 100 and eased["2"] == eased["4"]
    assert same(eased["4"], pulled)
    print("checked")


if __name__ == "__main__":
    main()
