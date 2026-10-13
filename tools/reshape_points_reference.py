"""Reshape's Correspondence Points and Interpolation, worked a second way.

D-702 closes D-606's gaps on `core.reshape` (`tools/reshape_reference.py`, whose rule this
file imports and does not change). After Effects names Correspondence Points and an
Interpolation choice and publishes no formula; the rule below is this program's own, and nothing
is ported.

The settings, beside D-606's.

`correspondence_points`, none (when added), or pairs of numbers, at most 32 pairs: each pair a
place on the source outline and the place on the destination outline that answers it, each per
cent of its outline's length from its first point, the way the outline is drawn, 0 to 100
(100 the first point again). Keyable as one list.

`interpolation`, `discrete` (when added), `linear` or `smooth`; a word, not keyed.

The rule, after D-606's steps 1 and 2.

1. Correspondence Points. With none, D-606's points: K = 64 along each outline from its first
   point. With pairs (a_i, b_i), each taken as a fraction (per cent / 100, 1 counted as 0):
   where the destination was turned round to wind the source's way (D-606 step 2), each b_i
   becomes 1 - b_i (1 counted as 0), its place on the turned outline. The pairs are put in
   order of a_i (two at the same a in the order written); a_0 is the first. Source point k
   (0 to K - 1) lies at r = k / K past a_0, at the fraction (a_0 + r) mod 1 along the source.
   With o_i = a_i - a_0, pair i is the last with o_i <= r; the next is pair i + 1 with o_(i+1),
   or, after the last, pair 0 with o = 1. f = (r - o_i) / (o_next - o_i); the gap
   g = (b_next - b_i) mod 1, and g = 1 for one pair alone; destination point k lies at the
   fraction (b_i + f g) mod 1 along the destination. A place at the fraction s along an outline
   is D-606's step 1 with s L for k L / K. One pair (0, 0) is D-606's points.
2. Interpolation. `discrete`: each frame is worked from Percent at that frame, D-606. `linear`
   and `smooth`, when Percent has two keys or more and the frame lies strictly between two of
   them, key A at frame a and key B at frame b: the frame is not worked from Percent at the
   frame; the field (D-606's steps at the grid's corners) is worked at Percent's value at the
   keys only, held inside 0 to 100, each with the masks as they are at this frame, and blended:
   u = (frame - a) / (b - a); `linear` (1 - u) F_A + u F_B; `smooth` the uniform Catmull-Rom
   through the key before A (A itself when there is none), A, B and the key after B (B itself
   when there is none), with weights (-u^3 + 2u^2 - u) / 2, (3u^3 - 5u^2 + 2) / 2,
   (-3u^3 + 4u^2 + u) / 2 and (u^3 - u^2) / 2. The keys' eases are not read. On a key, before
   the first or after the last, or with fewer than two keys, it is `discrete`. When every
   Percent blended is 0 the drawing is drawn as it is.

**This file never runs the build's code path.** It works in double precision on lists.

Every case is D-606's composition, 16 pixels by 10, Bulge's striped drawing, masks of mode
None. The drawing goes into `Fixtures/reshape_points/media`, the projects into
`Fixtures/reshape_points`, the expected frames into
`Fixtures/reshape_points/expected_reshape_points.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/reshape_points_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, OVERSHOOT  # noqa: E402
import reshape_reference as RS  # noqa: E402
import mesh_reference as M  # noqa: E402
import smooth_reference as S  # noqa: E402
import bulge_reference as B  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from mask_reference import points  # noqa: E402

W, H = RS.W, RS.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "reshape_points"
TOLERANCE = RS.TOLERANCE
K = RS.K
NAMES = RS.NAMES + ("correspondence_points", "interpolation")
WORDS = ("elasticity", "interpolation")


# --- the rule -------------------------------------------------------------------------------

def along(outline, s):
    """The place at the fraction `s` of `outline`'s length, D-606's step 1 with s L."""
    n = len(outline)
    pieces = [(outline[i], outline[(i + 1) % n]) for i in range(n)]
    lengths = [math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in pieces]
    total = 0.0
    for l in lengths:
        total += l
    at = s * total
    i, c = 0, 0.0
    while i < n - 1 and c + lengths[i] <= at:
        c += lengths[i]
        i += 1
    a, b = pieces[i]
    u = (at - c) / lengths[i] if lengths[i] > 0 else 0.0
    return (a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u)


def frac(p):
    return (p / 100) % 1.0


def matched(source, destination, pairs):
    """Step 1: the source and destination points, matched by the pairs."""
    if not pairs:
        return RS.reshape_points(source, destination)
    flip = RS.winding(source) * RS.winding(destination) < 0
    if flip:
        destination = RS.turned(destination)
    ps = [(frac(a), (1.0 - frac(b)) % 1.0 if flip else frac(b)) for a, b in pairs]
    ps = sorted(ps, key=lambda p: p[0])  # stable: equal places keep their order
    a0 = ps[0][0]
    o = [a - a0 for a, _ in ps] + [1.0]
    src, dst = [], []
    for k in range(K):
        r = k / K
        i = max(j for j in range(len(ps)) if o[j] <= r)
        nb = ps[(i + 1) % len(ps)][1]
        f = (r - o[i]) / (o[i + 1] - o[i])
        g = 1.0 if len(ps) == 1 else (nb - ps[i][1]) % 1.0
        src.append(along(source, (a0 + r) % 1.0))
        dst.append(along(destination, (ps[i][1] + f * g) % 1.0))
    return src, dst


def blend(c, frame_no):
    """Step 2: the Percents to work and their weights."""
    p = c["percent"]
    now = [(RS.held(c, "percent", frame_no), 1.0)]
    if c["interpolation"] == "discrete" or not isinstance(p, dict) or len(p["keys"]) < 2:
        return now
    keys = p["keys"]
    if frame_no <= keys[0][0] or frame_no >= keys[-1][0] or any(k[0] == frame_no for k in keys):
        return now
    j = max(i for i, k in enumerate(keys) if k[0] < frame_no)
    val = lambda i: min(100, max(0, keys[max(0, min(len(keys) - 1, i))][1]))  # noqa: E731
    u = (frame_no - keys[j][0]) / (keys[j + 1][0] - keys[j][0])
    if c["interpolation"] == "linear":
        return [(val(j), 1 - u), (val(j + 1), u)]
    w = [(-u ** 3 + 2 * u ** 2 - u) / 2, (3 * u ** 3 - 5 * u ** 2 + 2) / 2,
         (-3 * u ** 3 + 4 * u ** 2 + u) / 2, (u ** 3 - u ** 2) / 2]
    return [(val(j - 1), w[0]), (val(j), w[1]), (val(j + 1), w[2]), (val(j + 2), w[3])]


def reshape(layer, c, frame_no):
    src, dst, bound = (RS.chosen(c, k, frame_no) for k in
                       ("source_mask", "destination_mask", "boundary_mask"))
    if RS.MISSING_PATH in (src, dst, bound):
        return layer, True
    parts = blend(c, frame_no)
    if src is None or dst is None or all(p == 0 for p, _ in parts):
        return layer, False
    pairs = value_at(c["correspondence_points"], frame_no)
    pairs = [(pairs[i], pairs[i + 1]) for i in range(0, len(pairs), 2)]
    a, b = matched(src, dst, pairs)
    e = RS.ELASTICITY[c["elasticity"]]
    total, gx, gy = None, 0, 0
    for p, wt in parts:
        steps, gx, gy = RS.field(a, b, bound, p / 100, e, W, H)
        if total is None:
            total = [(wt * x, wt * y) for x, y in steps]
        else:
            total = [(tx + wt * x, ty + wt * y) for (tx, ty), (x, y) in zip(total, steps)]
    return M.draw_field(layer, total, gx, gy, W, H), False


def render(c, frame_no):
    return frame(reshape(B.drawn_layer(c["drawing"]), c, frame_no)[0], c["shift"])


# --- the cases ------------------------------------------------------------------------------

SQUARE, RIGHT, RIGHT_BACK = RS.SQUARE, RS.RIGHT, RS.RIGHT_BACK
none = RS.none


def case(correspondence_points=(), interpolation="discrete", **more):
    return {**RS.case(**more), "correspondence_points": correspondence_points if isinstance(correspondence_points, dict) else list(correspondence_points),
            "interpolation": interpolation}


RAMP = keyed((0, 0), (4, 100))
THREE = keyed((0, 0), (2, 100), (4, 40))

CASES = {
    "FX-RESHP-001": ("D-606's FX-RESHAPE-002 (the square pulled three pixels right) with one "
                     "correspondence pair, (0, 0): the first points answer each other, so "
                     "FX-RESHAPE-002's frame.", case([0, 0]), [0]),
    "FX-RESHP-002": ("One pair, (0, 25): the source's first corner answers the destination's "
                     "second, so the square is pulled right and turned a quarter round.",
                     case([0, 25]), [0]),
    "FX-RESHP-003": ("Two pairs, (0, 0) and (50, 25): the source's first half is squeezed into "
                     "the destination's first quarter.", case([0, 0, 50, 25]), [0]),
    "FX-RESHP-004": ("The destination drawn the other way round with pairs (0, 0) and (25, 75): "
                     "75 per cent along it as drawn is 25 per cent along it turned, so "
                     "FX-RESHAPE-002's frame.",
                     case([0, 0, 25, 75], masks=[none(SQUARE), none(RIGHT_BACK)]), [0]),
    "FX-RESHP-005": ("Percent keyed 0 at frame 0 to 100 at frame 4, Interpolation Linear: frame "
                     "2 is half of frame 4's field, not D-606's Percent 50.",
                     case(interpolation="linear", percent=RAMP), [0, 1, 2, 4]),
    "FX-RESHP-006": ("The same keys eased past their end, Linear: the ease is not read, so "
                     "FX-RESHP-005's frames.",
                     case(interpolation="linear", percent=keyed((0, 0, OVERSHOOT), (4, 100))),
                     [0, 1, 2, 4]),
    "FX-RESHP-007": ("Percent keyed 0, 100, 40 at frames 0, 2, 4, Interpolation Smooth: frames "
                     "1 and 3 blended through all three keys.",
                     case(interpolation="smooth", percent=THREE), [0, 1, 2, 3, 4]),
    "FX-RESHP-008": ("The same keys, Linear: frames 1 and 3 differ from Smooth's.",
                     case(interpolation="linear", percent=THREE), [1, 3]),
    "FX-RESHP-009": ("Discrete written out, Percent keyed 0 to 100: D-606's FX-RESHAPE-010.",
                     case(interpolation="discrete", percent=RAMP), [0, 2, 4]),
    "FX-RESHP-010": ("Smooth with Percent not keyed (100): FX-RESHAPE-002's frame.",
                     case(interpolation="smooth"), [0]),
    "FX-RESHP-011": ("The pairs keyed from (0, 0) at frame 0 to (0, 25) at frame 4: frame 0 is "
                     "FX-RESHAPE-002's, frame 4 FX-RESHP-002's.",
                     case(keyed((0, [0, 0]), (4, [0, 25]))), [0, 4]),
}

INVALID = {
    "FX-RESHP-012": ("Correspondence points 0, 0, 50: three numbers, not pairs.",
                     case([0, 0, 50])),
    "FX-RESHP-013": ("A correspondence point at 101, above 100.", case([0, 101])),
    "FX-RESHP-014": ("33 pairs, more than 32.", case([0, 0] * 33)),
    "FX-RESHP-015": ("Interpolation \"cubic\", not one of the three.",
                     case(interpolation="cubic")),
}


def project_json(fx, c):
    return RS.project_json(fx, c, names=NAMES, words=WORDS)


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
        before = RS.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = RS.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_reshape_points.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    drawn = RS.plain(RS.case())
    d606 = lambda **k: RS.render(RS.case(**k), 0)  # noqa: E731
    pulled = d606()

    # Step 1 on the numbers: one pair (0, 0) is D-606's points; (0, 25) a quarter round.
    sq, rt = (RS.flatten(RS.points_at(none(m), 0)) for m in (SQUARE, RIGHT))
    a, b = matched(sq, rt, [(0, 0)])
    a0, b0 = RS.reshape_points(sq, rt)
    assert all(near(p, q) for p, q in zip(a + b, a0 + b0))
    _, b = matched(sq, rt, [(0, 25)])
    assert near(b[0], b0[16]) and near(b[48], b0[0])
    a, b = matched(sq, rt, [(0, 0), (50, 25)])
    assert near(b[32], b0[16]) and near(b[16], b0[8]) and near(a[32], a0[32])

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert same(c["FX-RESHP-001"]["0"], pulled)
    assert same(c["FX-RESHP-004"]["0"], pulled)
    looks = [c["FX-RESHP-002"]["0"], c["FX-RESHP-003"]["0"], pulled]
    for i, x in enumerate(looks):
        for y in looks[i + 1:]:
            assert not same(x, y)
    lin = c["FX-RESHP-005"]
    assert lin["0"] == drawn and same(lin["4"], pulled)
    assert not same(lin["2"], d606(percent=50)) and not same(lin["2"], drawn)
    assert all(same(lin[f], c["FX-RESHP-006"][f]) for f in lin)
    sm, ln = c["FX-RESHP-007"], c["FX-RESHP-008"]
    assert same(sm["2"], pulled) and same(sm["4"], d606(percent=40)) and sm["0"] == drawn
    assert not same(sm["1"], ln["1"]) and not same(sm["3"], ln["3"])
    assert same(c["FX-RESHP-009"]["2"], d606(percent=50)) and same(c["FX-RESHP-009"]["4"], pulled)
    assert same(c["FX-RESHP-010"]["0"], pulled)
    assert same(c["FX-RESHP-011"]["0"], pulled) and same(c["FX-RESHP-011"]["4"], c["FX-RESHP-002"]["0"])
    print("checked")


if __name__ == "__main__":
    main()
