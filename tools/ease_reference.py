"""The eased keyframe segment, worked a second way.

Document 20 says a keyframe carries the interpolation mode used from it to the next keyframe,
and D-52 names the third mode: a cubic Bezier of value against time, written as the two inner
control points of a curve whose ends are fixed at (0,0) and (1,1). That is the same shape CSS
writes as `cubic-bezier(x1, y1, x2, y2)` and the same shape After Effects draws as two handles
in its value graph, which is why it was chosen over anything invented here.

Evaluating it means two steps, and only the first is interesting:

    x(u) = 3(1-u)^2 u x1 + 3(1-u) u^2 x2 + u^3
    y(u) = 3(1-u)^2 u y1 + 3(1-u) u^2 y2 + u^3

`u` is the curve's own parameter and is **not** time. Given a fraction of the way through the
segment, `p`, the value wanted is `y` at the `u` where `x(u) = p`. There is no useful closed
form, so it is solved.

**This file solves it by bisection and the build solves it by Newton's method with a bisection
fallback.** That is the point of this file existing: the same number reached two ways, in two
languages, from this document rather than from each other. The build's solver is fast and
converges in a handful of steps; this one is slow, obvious, and cannot converge on the wrong
answer, because `x` is non-decreasing on a curve whose control points are inside the unit square
and bisection on a monotonic function has nowhere else to go. Two hundred halvings of an
interval one unit wide is far past what f64 can distinguish, so what comes out is the f64 nearest
the true answer.

It prints the expected values. They are pinned in document 25, beside the case each belongs
to, because the person who checks this project reads that document and does not read JSON.
Fixtures are read-only to implementation work: this file is run when the specification
changes, and never to make a build pass.

    python tools/ease_reference.py
"""

# After Effects' easy ease is 33.33% influence with no speed either side. As the two inner
# control points of the unit curve that is exactly (1/3, 0) and (2/3, 1), and these are the f64
# nearest a third and two thirds, written out in full so the file and the build hold the same
# bits rather than the same fraction.
EASY_EASE = (0.3333333333333333, 0.0, 0.6666666666666666, 1.0)

# The curve that is linear. Present as a fixture rather than as a remark: an implementation that
# quietly rounds, clamps or shortcuts the solve will fail this one first.
LINEAR_AS_BEZIER = (0.3333333333333333, 0.3333333333333333, 0.6666666666666666, 0.6666666666666666)

# Slow out of the first key and stop dead into the second - deliberately not symmetric, so that
# a solver that assumes symmetry has somewhere to break.
EASE_OUT_ONLY = (0.0, 0.0, 0.58, 1.0)

# The web's `ease-in-out`. It is here because of an accident in the other two: with the inner
# control points a third and two thirds of the way along, x(u) works out to exactly u, the solve
# has nothing to do, and easy ease reduces to 3u^2 - 2u^3, which is smoothstep. That is a true and
# useful fact - it is why FX-EASE-002 can be checked by hand - but a fixture set made only of
# curves where the solve is free would not notice a solver that never worked. This one's x
# handles are 0.42 and 0.58, so the solve is real.
EASE_IN_OUT = (0.42, 0.0, 0.58, 1.0)


def _cubic(a, b, u):
    """One coordinate of the curve at parameter `u`, ends fixed at 0 and 1."""
    v = 1.0 - u
    return 3.0 * v * v * u * a + 3.0 * v * u * u * b + u * u * u


def ease(curve, p):
    """The eased fraction at progress `p`, by bisection on x."""
    x1, y1, x2, y2 = curve
    if p <= 0.0:
        return 0.0
    if p >= 1.0:
        return 1.0
    lo, hi = 0.0, 1.0
    for _ in range(200):
        mid = (lo + hi) / 2.0
        if _cubic(x1, x2, mid) < p:
            lo = mid
        else:
            hi = mid
    return _cubic(y1, y2, (lo + hi) / 2.0)


def segment(frame, a_frame, a_value, b_frame, b_value, curve):
    """One property's value at `frame`, on an eased segment from a to b.

    Scalars and pairs take the same curve: this is an ease in time, so both components of a
    position are the same fraction of the way along. A curve through the keys in space is a
    different decision and is not this one.
    """
    p = (frame - a_frame) / (b_frame - a_frame)
    f = ease(curve, p)
    if isinstance(a_value, list):
        return [a + (b - a) * f for a, b in zip(a_value, b_value)]
    return a_value + (b_value - a_value) * f


CASES = [
    {
        "id": "FX-EASE-001",
        "purpose": "the curve that is linear evaluates as linear",
        "curve": list(LINEAR_AS_BEZIER),
        "from": {"frame": 0, "value": 0.0},
        "to": {"frame": 24, "value": 120.0},
        "frames": [0, 1, 6, 12, 18, 23, 24],
        "tolerance": 1e-9,
    },
    {
        "id": "FX-EASE-002",
        "purpose": "easy ease on a scalar, and its exact midpoint",
        "curve": list(EASY_EASE),
        "from": {"frame": 0, "value": 0.0},
        "to": {"frame": 24, "value": 100.0},
        "frames": [0, 3, 6, 12, 18, 21, 24],
        "tolerance": 1e-6,
    },
    {
        "id": "FX-EASE-003",
        "purpose": "one timing curve drives both components of a pair",
        "curve": list(EASY_EASE),
        "from": {"frame": 0, "value": [0.0, 200.0]},
        "to": {"frame": 12, "value": [300.0, -40.0]},
        "frames": [0, 3, 6, 9, 12],
        "tolerance": 1e-6,
    },
    {
        "id": "FX-EASE-004",
        "purpose": "an asymmetric curve is not evaluated as a symmetric one",
        "curve": list(EASE_OUT_ONLY),
        "from": {"frame": 10, "value": 0.0},
        "to": {"frame": 34, "value": 1.0},
        "frames": [10, 16, 22, 28, 34],
        "tolerance": 1e-6,
    },
    {
        "id": "FX-EASE-005",
        "purpose": "a curve whose solve is not free, because x(u) is not u",
        "curve": list(EASE_IN_OUT),
        "from": {"frame": 0, "value": 0.0},
        "to": {"frame": 20, "value": 1.0},
        "frames": [0, 4, 8, 10, 12, 16, 20],
        "tolerance": 1e-6,
    },
]


def build():
    out = []
    for case in CASES:
        a, b = case["from"], case["to"]
        expected = [
            segment(f, a["frame"], a["value"], b["frame"], b["value"], case["curve"])
            for f in case["frames"]
        ]
        entry = dict(case)
        entry["expected"] = expected
        out.append(entry)
    return out


def main():
    for c in build():
        print(c["id"], "-", c["purpose"])
        print("  curve", c["curve"], "tolerance", c["tolerance"])
        for f, v in zip(c["frames"], c["expected"]):
            print("  frame %-4d %s" % (f, v))
        print()


if __name__ == "__main__":
    # The midpoint of a symmetric curve is the midpoint of the values, exactly, and a solver
    # that is merely close will not produce 50.0 from 0 and 100. Checked here so that running
    # this file at all says whether the reference itself still holds.
    assert ease(EASY_EASE, 0.5) == 0.5, "easy ease is symmetric about its middle"
    assert ease(LINEAR_AS_BEZIER, 0.25) - 0.25 < 1e-12, "the linear curve is linear"
    assert ease(EASY_EASE, 0.0) == 0.0 and ease(EASY_EASE, 1.0) == 1.0, "ends are pinned"
    main()
