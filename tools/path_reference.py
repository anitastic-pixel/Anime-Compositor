"""The motion path through the position keys, worked a second way.

D-52 named three decisions and settled two of them. This file belongs to the third: an eased
position still travels a *straight line* between its keys, faster and slower, and a curve through
the keys in space is a different curve. D-53 names that curve, and this file is its reference.

The curve is a cubic Bezier in composition pixels. A segment from key A to key B has four control
points - A's value, A's value plus A's outgoing handle, B's value plus B's incoming handle, and
B's value - and the handles are stored as offsets in composition pixels from the key they belong
to, which is what makes a handle a thing a person can read off the canvas.

    B(t) = (1-t)^3 P0 + 3(1-t)^2 t P1 + 3(1-t) t^2 P2 + t^3 P3

**`t` is the temporal fraction, after the ease.** It is not arc length. That is the decision
D-53 records, and the one consequence worth knowing before reading the tables: on a curved
segment with uneven handles the layer does not travel at a constant speed even under linear
timing. What it buys is a path that is a polynomial rather than a numerical length solve, and
the straight line that falls out of it exactly.

**A segment with no handles is a straight line, and the handles that make it one are at the
thirds** - exactly as in time, where `[1/3, 1/3, 2/3, 2/3]` is the curve that is linear. Control
points at one third and two thirds of the way between two points give `B(t) = P0 + t(P3 - P0)`
identically, so an absent field and today's linear segment are the same segment and every
existing fixture holds. Handles of *zero* are a different thing and not the default: they would
give `P0 + (3t^2 - 2t^3)(P3 - P0)`, which is the straight line at an easy-ease speed.

**This file evaluates the curve by de Casteljau and the build evaluates it by the polynomial
above.** Two routes to the same point, in two languages, written from document 20 rather than
from each other. The ease, where a case has one, is solved by bisection here exactly as
`tools/ease_reference.py` does, and for the same reason.

It prints the expected values. They are pinned in document 25, beside the case each belongs to,
because the person who checks this project reads that document and does not read JSON. Fixtures
are read-only to implementation work: this file is run when the specification changes, and never
to make a build pass.

    python tools/path_reference.py
"""

from ease_reference import EASY_EASE, ease

# A quarter-turn through the corner: out of the first key along +x, into the second key along
# +y. Handles of 160 on a 240 square are close enough to the two thirds that approximates a
# circular arc for the shape to read as one, without this file pretending it is exact.
CORNER_OUT = [160.0, 0.0]
CORNER_IN = [0.0, -160.0]


def de_casteljau(p0, p1, p2, p3, t):
    """One point of the segment at parameter `t`, by repeated linear interpolation.

    This and the expanded polynomial agree to the last bits and are different arithmetic. The
    build may expand the cubic; a reference that expanded it too would be checking a
    transcription rather than an answer.
    """

    def lerp(a, b):
        return [a[i] + (b[i] - a[i]) * t for i in range(len(a))]

    a, b, c = lerp(p0, p1), lerp(p1, p2), lerp(p2, p3)
    d, e = lerp(a, b), lerp(b, c)
    return lerp(d, e)


def thirds(a_value, b_value, sign):
    """The handle that makes a segment the straight line, which is where an absent one sits."""
    return [sign * (b_value[i] - a_value[i]) / 3.0 for i in range(len(a_value))]


def segment(frame, a_frame, a_value, b_frame, b_value, out_handle, in_handle, curve=None):
    """Position at `frame` on one path segment from a to b.

    `out_handle` and `in_handle` are offsets from their own key, or None for the straight line.
    `curve` is the four numbers of an ease on the segment, or None for linear timing. The two
    are independent: the ease says when the layer is a given fraction of the way along, and the
    path says where that fraction is. This is the whole of how they compose.
    """
    u = (frame - a_frame) / (b_frame - a_frame)
    t = ease(curve, u) if curve else u
    p1 = [a_value[i] + (out_handle or thirds(a_value, b_value, 1.0))[i] for i in range(2)]
    p2 = [b_value[i] + (in_handle or thirds(a_value, b_value, -1.0))[i] for i in range(2)]
    return de_casteljau(a_value, p1, p2, b_value, t)


CASES = [
    {
        "id": "FX-PATH-001",
        "purpose": "the handles that make a straight line make a straight line",
        "from": {"frame": 0, "value": [0.0, 0.0], "out": [80.0, 40.0]},
        "to": {"frame": 24, "value": [240.0, 120.0], "in": [-80.0, -40.0]},
        "ease": None,
        "frames": [0, 1, 6, 12, 18, 23, 24],
        "tolerance": 1e-9,
    },
    {
        "id": "FX-PATH-002",
        "purpose": "one curved segment under linear timing",
        "from": {"frame": 0, "value": [0.0, 0.0], "out": CORNER_OUT},
        "to": {"frame": 24, "value": [240.0, 240.0], "in": CORNER_IN},
        "ease": None,
        "frames": [0, 6, 12, 18, 24],
        "tolerance": 1e-6,
    },
    {
        "id": "FX-PATH-003",
        "purpose": "the same curved segment with an ease on it, to show the two compose",
        "from": {"frame": 0, "value": [0.0, 0.0], "out": CORNER_OUT},
        "to": {"frame": 24, "value": [240.0, 240.0], "in": CORNER_IN},
        "ease": list(EASY_EASE),
        "frames": [0, 6, 12, 18, 24],
        "tolerance": 1e-6,
    },
    {
        "id": "FX-PATH-004",
        "purpose": "a handle longer than its own segment, which is legal in space",
        "from": {"frame": 0, "value": [0.0, 0.0], "out": [-150.0, 90.0]},
        "to": {"frame": 20, "value": [200.0, 0.0], "in": [150.0, 90.0]},
        "ease": None,
        "frames": [0, 5, 10, 15, 20],
        "tolerance": 1e-6,
    },
]


def build():
    out = []
    for case in CASES:
        a, b = case["from"], case["to"]
        entry = dict(case)
        entry["expected"] = [
            segment(
                f,
                a["frame"],
                a["value"],
                b["frame"],
                b["value"],
                a.get("out"),
                b.get("in"),
                case["ease"],
            )
            for f in case["frames"]
        ]
        out.append(entry)
    return out


def main():
    for c in build():
        print(c["id"], "-", c["purpose"])
        print("  tolerance", c["tolerance"], "ease", c["ease"])
        for f, v in zip(c["frames"], c["expected"]):
            print("  frame %-4d %s" % (f, v))
        print()


if __name__ == "__main__":
    # The thirds are the straight line, exactly. If this fails there is no motion path
    # specification worth reading, because the one thing it must not do is move a fixture that
    # already passes.
    for n in (1, 7, 23):
        p = segment(n, 0, [0.0, 0.0], 24, [240.0, 120.0], None, None)
        # Identically in real arithmetic; to about 6e-14 in f64, because de Casteljau reaches
        # the point by five roundings rather than none. That gap is what FX-PATH-001's 1e-9
        # tolerance is for, and it is the same gap FX-EASE-001 records in time.
        assert abs(p[0] - 240.0 * n / 24.0) < 1e-12, "absent handles are the straight line in x"
        assert abs(p[1] - 120.0 * n / 24.0) < 1e-12, "absent handles are the straight line in y"
    # Zero handles are the other thing, and that difference is why absence is not zero.
    mid = segment(12, 0, [0.0, 0.0], 24, [240.0, 120.0], [0.0, 0.0], [0.0, 0.0])
    assert mid == [120.0, 60.0], "a symmetric speed profile is still half way at half way"
    quarter = segment(6, 0, [0.0, 0.0], 24, [240.0, 120.0], [0.0, 0.0], [0.0, 0.0])
    assert quarter[0] < 60.0, "zero handles are a speed profile, not the straight line"
    main()
