"""D-1011: Color Grade's Hue vs Saturation curve you can draw, the reference for the page's arithmetic.

Writes `Fixtures/hue_curve_drag/expected_hue_curve_drags.json`: where a press on the graph puts a
hue and saturation, the curve's saturation at a hue (D-398's rule, `grade::hue_curve`), what a
press adds and where a dragged point goes. Run before the page code; B-891's page check runs
every case through the page's own `hueCurveAt`, `hueCurveValue`, `hueCurveAdd` and
`hueCurveMove`.

The graph: hue 0 at the left to 360 at the right, saturation 200 at the top to 0 at the bottom;
u and v run 0 to 1 from the top left. A point (u, v): the hue is 360 u rounded, held to 0..359;
the saturation is 200 (1 - v) rounded, held to 0..200. With no points the curve is 100 at every
hue. A press away from every point adds one at its hue, on the line (the curve's saturation
there, rounded), so the picture does not change until it is dragged; no more than 16, and none
on a hue a point already has. A dragged point keeps between its neighbours' hues (at least 1
apart, 0 to 359) and its saturation 0 to 200.

  python -I tools/hue_curve_drag_reference.py
"""
import json
import math
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "hue_curve_drag"


def rnd(x):
    return math.floor(x + 0.5)


def hc_at(u, v):
    return {"hue": min(359, max(0, rnd(360 * u))), "saturation": min(200, max(0, rnd(200 * (1 - v))))}


def hc_value(points, hue):
    """grade::hue_curve, 100 when there are no points."""
    n = len(points)
    if n == 0:
        return 100.0
    for i in range(n):
        h0, s0, s1 = points[i][0], points[i][1], points[(i + 1) % n][1]
        h1 = points[0][0] + 360 if i + 1 == n else points[i + 1][0]
        x = hue if hue >= h0 else hue + 360
        if n > 1 and h0 <= x < h1:
            u = (x - h0) / (h1 - h0)
            return s0 + (s1 - s0) * u * u * (3 - 2 * u)
    return float(points[0][1])


def hc_add(points, hue):
    if len(points) >= 16 or any(p[0] == hue for p in points):
        return None
    new = sorted(points + [[hue, rnd(hc_value(points, hue))]])
    return {"points": new, "index": new.index([hue, rnd(hc_value(points, hue))])}


def hc_move(points, i, hue, sat):
    lo = points[i - 1][0] + 1 if i > 0 else 0
    hi = points[i + 1][0] - 1 if i + 1 < len(points) else 359
    new = [list(p) for p in points]
    new[i] = [min(hi, max(lo, hue)), min(200, max(0, sat))]
    return new


AT = [
    ("The top left: hue 0, saturation 200.", 0, 0),
    ("The middle: hue 180, saturation 100.", 0.5, 0.5),
    ("The bottom right: held to hue 359, saturation 0.", 1, 1),
    ("A third across, a quarter down: hue 120, saturation 150.", 1 / 3, 0.25),
    ("Outside the graph: held to 0 and 200.", -0.2, -0.3),
    ("Hue 0.4 of a step past 90 rounds down.", 90.4 / 360, 0.5),
]
VALUE = [
    ("No points: 100 at every hue.", [], 77),
    ("One point: its saturation at every hue.", [[120, 40]], 300),
    ("Two points, on the first.", [[0, 100], [180, 0]], 0),
    ("Two points, half way: the easing's middle, 50.", [[0, 100], [180, 0]], 90),
    ("Two points, a quarter of the way: 100 - 100 x 0.15625 = 84.375.", [[0, 100], [180, 0]], 45),
    ("Past the last point, round the circle to the first: half way from 180 to 360, 50.", [[0, 100], [180, 0]], 270),
    ("Before the first point, through 360: half way from 300 to 420 (60).", [[60, 200], [300, 0]], 0),
]
ADD = [
    ("Into no points: on the flat line, 100.", [], 200),
    ("Between two: on the line at 90, 50.", [[0, 100], [180, 0]], 90),
    ("After the last, round the circle: at 270, 50.", [[0, 100], [180, 0]], 270),
    ("On a hue a point already has: nothing added.", [[0, 100], [180, 0]], 180),
    ("A 17th point: refused.", [[h * 20, 100] for h in range(16)], 330),
]
MOVE = [
    ("A lone point dragged anywhere.", [[120, 100]], 0, 300, 20),
    ("Dragged past its right neighbour: held one below it.", [[0, 100], [100, 50], [200, 80]], 1, 250, 60),
    ("Dragged past its left neighbour: held one above it.", [[0, 100], [100, 50], [200, 80]], 1, -40, 60),
    ("The last point dragged past 359: held to 359.", [[0, 100], [200, 80]], 1, 400, 80),
    ("Dragged above 200 and below 0: held.", [[0, 100], [200, 80]], 0, 0, 260),
    ("Dragged below 0.", [[0, 100], [200, 80]], 1, 200, -5),
]


def main():
    cases = {
        "rule": "D-1011",
        "at": [{"says": s, "u": u, "v": v, "to": hc_at(u, v)} for s, u, v in AT],
        "value": [{"says": s, "points": p, "hue": h, "to": round(hc_value(p, h), 6)} for s, p, h in VALUE],
        "add": [{"says": s, "points": p, "hue": h, "to": hc_add(p, h)} for s, p, h in ADD],
        "move": [{"says": s, "points": p, "index": i, "hue": h, "saturation": v, "to": hc_move(p, i, h, v)}
                 for s, p, i, h, v in MOVE],
    }
    # The claims the cases make.
    a = [c["to"] for c in cases["at"]]
    assert a[0] == {"hue": 0, "saturation": 200} and a[1] == {"hue": 180, "saturation": 100}
    assert a[2] == {"hue": 359, "saturation": 0} and a[3] == {"hue": 120, "saturation": 150}
    assert a[4] == {"hue": 0, "saturation": 200} and a[5]["hue"] == 90
    v = [c["to"] for c in cases["value"]]
    assert v == [100, 40, 100, 50, 84.375, 50, 100]
    d = [c["to"] for c in cases["add"]]
    assert d[0] == {"points": [[200, 100]], "index": 0}
    assert d[1] == {"points": [[0, 100], [90, 50], [180, 0]], "index": 1}
    assert d[2] == {"points": [[0, 100], [180, 0], [270, 50]], "index": 2}
    assert d[3] is None and d[4] is None
    m = [c["to"] for c in cases["move"]]
    assert m[0] == [[300, 20]] and m[1][1] == [199, 60] and m[2][1] == [1, 60]
    assert m[3][1] == [359, 80] and m[4][0] == [0, 200] and m[5][1] == [200, 0]
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "expected_hue_curve_drags.json").write_bytes((json.dumps(cases, indent=1) + "\n").encode("utf-8"))
    print("checked")


if __name__ == "__main__":
    main()
