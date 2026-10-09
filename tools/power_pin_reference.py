"""Power Pin, worked a second way.

D-389 adds `core.power_pin`, after CycoreFX's CC Power Pin: Corner Pin (D-198) with three more
controls, a Perspective that can be turned down so that pulling corners together squeezes the
drawing evenly instead of in perspective, an Unstretch that runs the pinning backwards (the
four corners' shape is stretched out to fill the layer), and four Expansions that grow or shrink
the pinned shape past the pins. CycoreFX's manual says what each control does in a sentence and
publishes no formula; the rule below is this program's own reading of it, and nothing is ported.

The rule. W and H are the drawing's own size. The pins are per cent of it as Corner Pin's are,
the ring A = top left, B = top right, C = bottom right, D = bottom left. With the four pins at
their starting places and every expansion 0 the output is the input and nothing grows. Unless
the ring is convex (Corner Pin's test) the output is the input's size and clear.

1. p = `perspective` / 100. M is Heckbert's square-to-quad map onto A, B, C, D (Corner Pin's)
   and Bl the bilinear one, Bl(u, v) = A + u (B - A) + v (D - A) + u v (A - B + C - D).
2. The expanded square: l, t, r, b = `expansion_left`, `_top`, `_right`, `_bottom` / 100; its
   corners (-l, -t), (1 + r, -t), (1 + r, 1 + b), (-l, 1 + b). Each is carried to
   p M(u, v) + (1 - p) Bl(u, v), the target ring Q; a corner not expanded (its u and v each 0 or
   1) is its pin, exactly, as both maps put it. When p > 0, a corner M carries to or past its
   horizon (M's third row, g u + h v + 1, at most 0) leaves the output clear; so does a Q that is
   not convex.
3. Unstretch `off`: the layer grows as Corner Pin's does, so Q's corners are inside it. Each
   output pixel centre X (in the drawing's space) is taken back to the square two ways: by the
   inverse of Q's square-to-quad map, (U, V, T) = adj(M_Q) X, which needs T det(M_Q) > 0, giving
   (U / T, V / T); and by the inverse of Q's bilinear map (`unbilinear` below), which needs a
   real root. At p = 1 only the first is needed, at p = 0 only the second; between, both, and the
   point is p times the first plus 1 - p times the second. Missing one, the pixel is clear;
   otherwise it is document 21's bilinear sample of the input at (u W, v H).
4. Unstretch `on`: nothing grows. Each output pixel centre X is the square's (u, v) = (X_x / W,
   X_y / H), carried to p M_Q(u, v) + (1 - p) Bl_Q(u, v), which needs M_Q's third row above 0
   when p > 0, and sampled there: Q's shape stretched out to fill the drawing.

`top_left` 0, 0; `top_right` 100, 0; `bottom_left` 0, 100; `bottom_right` 100, 100, each -400
to 500 per cent each way, Corner Pin's range; `perspective` 0 to 100, 100; `unstretch` `off` or
`on`, `off`; `expansion_top`, `expansion_left`, `expansion_right`, `expansion_bottom` -40 to 100,
0. The numbers are keyable. Everything is a share of the drawing, so a draft needs no change.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/power_pin/media`, the projects into
`Fixtures/power_pin`, and the expected frames into `Fixtures/power_pin/expected_power_pin.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/power_pin_reference.py
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
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from corner_pin_reference import convex, square_to_quad, adjugate, det, growth  # noqa: E402
import corner_pin_reference as CP  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "power_pin"
TOLERANCE = 2e-5  # document 25's default for a filter
PINS = ("top_left", "top_right", "bottom_left", "bottom_right")
EXPANSIONS = ("expansion_top", "expansion_left", "expansion_right", "expansion_bottom")
NAMES = PINS + ("perspective", "unstretch") + EXPANSIONS
START = {"top_left": (0, 0), "top_right": (100, 0), "bottom_left": (0, 100),
         "bottom_right": (100, 100)}
RANGES = {**{k: (-400, 500) for k in PINS}, "perspective": (0, 100),
          **{k: (-40, 100) for k in EXPANSIONS}}
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def ring_of(s_):
    """A, B, C, D: top left, top right, bottom right, bottom left, in the drawing's space."""
    return [(s_[k][0] / 100 * W, s_[k][1] / 100 * H)
            for k in ("top_left", "top_right", "bottom_right", "bottom_left")]


def homog(M, u, v):
    """M(u, v) and the third row t."""
    x, y, t = (r[0] * u + r[1] * v + r[2] for r in M)
    return x / t if t else math.inf, y / t if t else math.inf, t


def bilin(ring, u, v):
    A, B_, C, D = ring
    return tuple((1 - u) * (1 - v) * A[i] + u * (1 - v) * B_[i] + u * v * C[i] + (1 - u) * v * D[i]
                 for i in range(2))


def cross(a, b):
    return a[0] * b[1] - a[1] * b[0]


def unbilinear(ring, X):
    """(u, v) with Bl(u, v) = X, the root nearer the square's middle, or None."""
    A, B_, C, D = ring
    e = (B_[0] - A[0], B_[1] - A[1])
    f = (D[0] - A[0], D[1] - A[1])
    g = (A[0] - B_[0] + C[0] - D[0], A[1] - B_[1] + C[1] - D[1])
    h = (X[0] - A[0], X[1] - A[1])
    k2 = cross(g, f)
    k1 = cross(e, f) + cross(h, g)
    k0 = cross(h, e)
    disc = k1 * k1 - 4 * k2 * k0
    if disc < 0:
        return None
    q = -0.5 * (k1 + math.copysign(math.sqrt(disc), k1 if k1 != 0 else 1.0))
    best = None
    for v in ((q / k2) if k2 != 0 else math.inf, (k0 / q) if q != 0 else math.inf):
        if not math.isfinite(v):
            continue
        ex, ey = e[0] + v * g[0], e[1] + v * g[1]
        den = ex * ex + ey * ey
        if den == 0:
            continue
        u = ((h[0] - v * f[0]) * ex + (h[1] - v * f[1]) * ey) / den
        far = max(abs(u - 0.5), abs(v - 0.5))
        if best is None or far < best[0]:
            best = (far, u, v)
    return None if best is None else (best[1], best[2])


def target(s_):
    """Q, the expanded square's corners carried by the blend, or None when there is none."""
    ring = ring_of(s_)
    if not convex(ring):
        return None
    p = s_["perspective"] / 100
    M = square_to_quad(*ring)
    l, t, r, b = (s_[k] / 100 for k in ("expansion_left", "expansion_top", "expansion_right",
                                         "expansion_bottom"))
    Q = []
    for corner, (u, v) in zip(ring, ((-l, -t), (1 + r, -t), (1 + r, 1 + b), (-l, 1 + b))):
        if u in (0, 1) and v in (0, 1):
            Q.append(corner)  # both maps carry the square's own corner to its pin, exactly
            continue
        x, y, w = homog(M, u, v)
        if p > 0 and w <= 0:
            return None
        bx, by = bilin(ring, u, v)
        Q.append((p * x + (1 - p) * bx, p * y + (1 - p) * by) if p > 0 else (bx, by))
    return Q if convex(Q) else None


def stretched_at(layer, Q, MQ, adj, dm, p, X):
    """Unstretch off: the output at X."""
    uv = []
    if p > 0:
        U, V, T = (r[0] * X[0] + r[1] * X[1] + r[2] for r in adj)
        if T * dm <= 0:
            return EMPTY
        uv.append((p, U / T, V / T))
    if p < 1:
        b = unbilinear(Q, X)
        if b is None:
            return EMPTY
        uv.append((1 - p, b[0], b[1]))
    u = sum(w * a for w, a, _ in uv)
    v = sum(w * c for w, _, c in uv)
    sx, sy = u * W, v * H
    if not (math.isfinite(sx) and math.isfinite(sy)):
        return EMPTY
    return bilinear(layer, sx, sy)


def unstretched_at(layer, Q, MQ, p, X):
    """Unstretch on: the output at X."""
    u, v = X[0] / W, X[1] / H
    x, y = bilin(Q, u, v)
    if p > 0:
        hx, hy, t = homog(MQ, u, v)
        if t <= 0:
            return EMPTY
        x, y = p * hx + (1 - p) * x, p * hy + (1 - p) * y
    return bilinear(layer, x, y)


def power_pin(layer, s_):
    if all(tuple(s_[k]) == START[k] for k in PINS) and all(s_[k] == 0 for k in EXPANSIONS):
        return layer
    Q = target(s_)
    if Q is None:
        return dict(layer, px=[EMPTY] * (layer["w"] * layer["h"]))
    p = s_["perspective"] / 100
    MQ = square_to_quad(*Q)
    if s_["unstretch"] == "on":
        px = [unstretched_at(layer, Q, MQ, p, (layer["left"] + i + 0.5, layer["top"] + j + 0.5))
              for j in range(layer["h"]) for i in range(layer["w"])]
        return dict(layer, px=px)
    adj, dm = adjugate(MQ), det(MQ)
    gx, gy = growth(layer, Q)
    left, top = layer["left"] - gx, layer["top"] - gy
    w, h = layer["w"] + 2 * gx, layer["h"] + 2 * gy
    px = [stretched_at(layer, Q, MQ, adj, dm, p, (left + i + 0.5, top + j + 0.5))
          for j in range(h) for i in range(w)]
    return {"px": px, "left": left, "top": top, "w": w, "h": h}


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, perspective=100, unstretch="off", **more):
    c = {"drawing": "stripes", "shift": shift, "perspective": perspective,
         "unstretch": unstretch, **START, **{k: 0 for k in EXPANSIONS}}
    c.update(more)
    return c


def settings(c, frame_no):
    held = {"unstretch": c["unstretch"]}
    for k in RANGES:
        v = value_at(c[k], frame_no)
        lo, hi = RANGES[k]
        held[k] = (tuple(min(hi, max(lo, u)) for u in v) if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    return frame(power_pin(B.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


SHRINK = dict(top_left=(25, 25), top_right=(75, 25), bottom_left=(25, 75), bottom_right=(75, 75))
KEYSTONE = dict(top_left=(25, 0), top_right=(75, 0))
FAR_LEFT = dict(top_left=(0, 45), bottom_left=(0, 55))
ALL50 = {k: 50 for k in EXPANSIONS}

CASES = {
    "FX-POWERPIN-001": ("The settings as they start: the drawing, untouched, and nothing grows.",
                        case(), [0]),
    "FX-POWERPIN-002": ("Shrunk to the middle, the pins at (25, 25), (75, 25), (25, 75) and (75, "
                        "75): the drawing at half its size in the middle, as Corner Pin draws it.",
                        case(**SHRINK), [0]),
    "FX-POWERPIN-003": ("A keystone, Top Left at (25, 0) and Top Right at (75, 0), Perspective "
                        "100: the drawing leans back as Corner Pin leans it, its far upper half "
                        "drawn smaller, so the blue band lands in rows 2 and 3.",
                        case(**KEYSTONE), [0]),
    "FX-POWERPIN-004": ("The same keystone at Perspective 0: squeezed evenly instead, every row "
                        "as tall as before, so the band stays in rows 4 and 5.",
                        case(perspective=0, **KEYSTONE), [0]),
    "FX-POWERPIN-005": ("The same keystone at Perspective 50: half way between.",
                        case(perspective=50, **KEYSTONE), [0]),
    "FX-POWERPIN-006": ("Unstretch on, the pins shrunk to the middle: the other way round, the "
                        "middle of the drawing stretched out to fill it, twice the size.",
                        case(unstretch="on", **SHRINK), [0]),
    "FX-POWERPIN-007": ("Unstretch on with the keystone: the keystone's shape stretched out to "
                        "fill the drawing, its top widened.",
                        case(unstretch="on", **KEYSTONE), [0]),
    "FX-POWERPIN-008": ("The pins shrunk to the middle and every expansion 50: the pinned shape "
                        "grown back out by half of it on each side, which is the whole drawing: "
                        "the drawing, untouched.", case(**SHRINK, **ALL50), [0]),
    "FX-POWERPIN-009": ("Expansion Left -25 with the pins where they start: the left side "
                        "pulled in a quarter of the way, the drawing squeezed into columns 4 to "
                        "15.", case(expansion_left=-25), [0]),
    "FX-POWERPIN-010": ("Expansion Right 50 and Bottom 50 with the pins where they start: the "
                        "drawing grown half as much again to the right and down from its top "
                        "left corner; the layer grows to hold it.",
                        case(expansion_right=50, expansion_bottom=50), [0]),
    "FX-POWERPIN-011": ("Top Right at (100, 100) and Bottom Right at (100, 0), crossed like a "
                        "bow tie: no drawable shape, so the frame is clear.",
                        case(top_right=(100, 100), bottom_right=(100, 0)), [0]),
    "FX-POWERPIN-012": ("The left side squeezed small and far away, Top Left at (0, 45) and "
                        "Bottom Left at (0, 55), so the right side is the near one, and Expansion "
                        "Right 100 at Perspective 100: the near side grown by the drawing's whole "
                        "width would run past the vanishing line, so the frame is clear.",
                        case(expansion_right=100, **FAR_LEFT), [0]),
    "FX-POWERPIN-013": ("The same at Perspective 0: squeezed evenly there is no vanishing line, "
                        "and the grown shape is drawn.",
                        case(perspective=0, expansion_right=100, **FAR_LEFT), [0]),
    "FX-POWERPIN-014": ("The keystone with Perspective keyed from 0 at frame 0 to 100 at frame "
                        "4, linear: frame 0 FX-POWERPIN-004, frame 2 FX-POWERPIN-005 and frame 4 "
                        "FX-POWERPIN-003.",
                        case(perspective=keyed((0, 0), (4, 100)), **KEYSTONE), [0, 2, 4]),
    "FX-POWERPIN-015": ("Top Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4 at "
                        "Perspective 0: frame 0 the drawing, then its right side ever shorter, "
                        "squeezed evenly.",
                        case(perspective=0, top_right=keyed((0, (100, 0)), (4, (100, 50)))),
                        [0, 2, 4]),
    "FX-POWERPIN-016": ("Top Left pulled out to (-25, 0), the layer moved three pixels right, "
                        "Perspective 100: the layer grows four pixels each side as Corner Pin's "
                        "FX-PIN-006 does, and the frame is Corner Pin's.",
                        case(3, top_left=(-25, 0)), [0]),
    "FX-POWERPIN-017": ("Expansion Top keyed from 0 at frame 0 to 100 at frame 4, eased past its "
                        "end: frame 2 would pass 100 and is held there, the same as frame 4.",
                        case(expansion_top=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-POWERPIN-018": ("Unstretch on with the keystone at Perspective 0, the layer moved two "
                        "pixels right: the evenly squeezed keystone stretched back out; nothing "
                        "grows.", case(2, perspective=0, unstretch="on", **KEYSTONE), [0]),
}

INVALID = {
    "FX-POWERPIN-019": ("Perspective 101, above 100.", case(perspective=101)),
    "FX-POWERPIN-020": ("Perspective -1, below 0.", case(perspective=-1)),
    "FX-POWERPIN-021": ("Expansion Top 101, above 100.", case(expansion_top=101)),
    "FX-POWERPIN-022": ("Expansion Left -41, below -40.", case(expansion_left=-41)),
    "FX-POWERPIN-023": ("Unstretch \"yes\", not a word it takes.", case(unstretch="yes")),
    "FX-POWERPIN-024": ("Top Left at (-401, 0), its x below -400.", case(top_left=(-401, 0))),
    "FX-POWERPIN-025": ("Bottom Right keyed to (100, 600) at frame 4.",
                        case(bottom_right=keyed((0, (100, 100)), (4, (100, 600))))),
}

# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.power_pin", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
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
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_power_pin.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    blue = lambda p: p[2] > p[0] + 0.1  # noqa: E731
    clear = lambda p: p == EMPTY  # noqa: E731

    # The inverse bilinear map undoes the bilinear one, on a skewed ring and a parallelogram.
    for ring in (ring_of(settings(case(**KEYSTONE), 0)), [(1, 1), (9, 2), (12, 9), (0, 7)],
                 [(0, 0), (10, 2), (12, 9), (2, 7)]):
        for u, v in ((0.1, 0.2), (0.5, 0.5), (0.9, 0.7), (1.3, -0.2)):
            back = unbilinear(ring, bilin(ring, u, v))
            assert back and abs(back[0] - u) < 1e-9 and abs(back[1] - v) < 1e-9, (ring, u, v)

    # At Perspective 100 with no expansion, Power Pin is Corner Pin.
    for pins in (SHRINK, KEYSTONE, dict(top_left=(-25, 0)), FAR_LEFT,
                 dict(bottom_right=(60, 60))):
        cp = {"upper_left": "top_left", "upper_right": "top_right", "lower_left": "bottom_left",
              "lower_right": "bottom_right"}
        s_ = settings(case(**pins), 0)
        ours = power_pin(B.drawn_layer("stripes"), s_)
        theirs = CP.corner_pin(B.drawn_layer("stripes"), {k: s_[v] for k, v in cp.items()}, W, H)
        assert (ours["left"], ours["top"], ours["w"], ours["h"]) == (
            theirs["left"], theirs["top"], theirs["w"], theirs["h"])
        assert same(ours["px"], theirs["px"]), pins

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert c["FX-POWERPIN-001"]["0"] == drawn
    two = c["FX-POWERPIN-002"]["0"]
    assert all(clear(two[at(x, y)]) for x in range(W) for y in range(H)
               if not (4 <= x <= 11 and 2 <= y <= 6))
    assert all(not clear(two[at(x, 4)]) for x in range(4, 11))
    three, four, five = (c[f"FX-POWERPIN-00{i}"]["0"] for i in (3, 4, 5))
    assert blue(three[at(8, 2)]) and blue(three[at(8, 3)]) and not blue(three[at(8, 5)])
    assert blue(four[at(8, 4)]) and blue(four[at(8, 5)]) and not blue(four[at(8, 3)])
    assert not same(five, three) and not same(five, four)
    six = c["FX-POWERPIN-006"]["0"]
    layer = B.drawn_layer("stripes")
    assert all(near(six[at(x, y)], bilinear(layer, (x + 0.5) / 2 + W / 4, (y + 0.5) / 2 + H / 4))
               for x in range(W) for y in range(H))
    assert not same(c["FX-POWERPIN-007"]["0"], drawn)
    assert same(c["FX-POWERPIN-008"]["0"], drawn)
    nine = c["FX-POWERPIN-009"]["0"]
    assert all(clear(nine[at(x, y)]) for x in range(4) for y in range(H))
    assert not clear(nine[at(4, 1)])
    ten = c["FX-POWERPIN-010"]["0"]
    assert near(ten[at(0, 2)], bilinear(layer, 0.5 / 1.5, 2.5 / 1.5))
    assert all(clear(p) for p in c["FX-POWERPIN-011"]["0"])
    assert target(settings(case(expansion_right=100, **FAR_LEFT), 0)) is None
    assert target(settings(case(expansion_right=10, **FAR_LEFT), 0)) is not None
    assert all(clear(p) for p in c["FX-POWERPIN-012"]["0"])
    assert any(not clear(p) for p in c["FX-POWERPIN-013"]["0"])
    fourteen = c["FX-POWERPIN-014"]
    assert fourteen["0"] == four and fourteen["2"] == five and fourteen["4"] == three
    fifteen = c["FX-POWERPIN-015"]
    assert fifteen["0"] == drawn and fifteen["2"] != drawn and fifteen["4"] != fifteen["2"]
    sixteen = c["FX-POWERPIN-016"]["0"]
    assert same(sixteen, frame(CP.corner_pin(
        B.drawn_layer("stripes"), {**CP.START, "upper_left": (-25, 0)}, W, H), 3))
    assert all(any(sixteen[at(x, y)][3] > 0.2 for x in range(3)) for y in range(2, 8))
    seventeen = c["FX-POWERPIN-017"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 100)), 2) > 100
    assert seventeen["2"] == seventeen["4"] != drawn
    eighteen = c["FX-POWERPIN-018"]["0"]
    assert all(clear(eighteen[at(x, y)]) for x in range(2) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
