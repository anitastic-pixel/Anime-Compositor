"""Bend It, worked a second way.

D-377 adds `core.bend_it`, after CycoreFX's CC Bend It: the layer bent as a strong man bends a
steel bar, a real bend along a circle and not a slant, so that at a full turn the bar's two ends
meet. CycoreFX's manual says what each control does in a sentence and publishes no formula; the
rule below is this program's own reading of it, and nothing is ported.

The rule. S and E are `start` and `end`, per cent of the drawing's own width and height as Radial
Blur's centre is; L = |E - S|, t = (E - S) / L the bar's direction, n = t turned 90 degrees
clockwise on the screen (y down), so with the bar standing up, S at the bottom, n points right.
A point of the drawing is (u, v): u along t from S, v along n. Each pixel's centre P of the output
gathers every place of the drawing that the bend lays on it, and lays them one over another,
the place farthest along the bar (the largest u) on top; each is document 21's bilinear sample of
the drawing, transparent outside it.

- `bend` B degrees is the whole turn of the bar from S to E: theta = B pi / 180, R = L / theta
  (signed), the circle's centre C = S + R n. The bar's points 0 <= u <= L lie on the circle:
  (u, v) goes to C + (v - R) n(phi), phi = u theta / L, where t(phi) = t cos phi + n sin phi and
  n(phi) = n cos phi - t sin phi. Working back from P, with q = P - C, phi+ = atan2(-q.t, q.n):
  the places on P are phi = phi+ + j pi for every whole j, v = R + (-1)^j |q|, u = phi L / theta.
- `render_prestart`, the drawing before S (u < 0): `none` draws none of it; `static` draws it
  unbent, P itself wherever (P - S).t < 0; `bend` carries the same bend on back for L more, so the
  part from -L to 0 is the bar's mirror image in shape, and lays what lies past -L straight along
  the tangent there: P = C(-L) + a t(-theta) + b n(-theta) with a < 0 reads (-L + a, b);
  `mirror` bends the bar's own Start to End stretch back the same way from -L to 0, reading
  (-u, v) there, and draws nothing past -L.
- `distort`, the drawing past E (u > L): `legal` draws none of it; `extended` lays it straight
  along the tangent at E: P = C(L) + a t(theta) + b n(theta) with a > 0 reads (L + a, b).
- Bend 0 is the straight bar, and the prestart and distort still cut. S = E is no bar: the
  drawing untouched.

The layer does not grow, so a bend that carries the bar past its edges is cut there, as After
Effects cuts it. Nothing scales for a draft: the bend is degrees and the points shares.

`bend` -360 to 360, 45 when added; `start` and `end` -1000 to 1000 per cent each way, 50, 100
and 50, 0 when added: the bar standing up, the drawing's full height. `render_prestart` `none`,
`static`, `bend` or `mirror`, `none` when added; `distort` `legal` or `extended`, `legal`. The
numbers are keyable. The values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/bend_it/media`, the projects into
`Fixtures/bend_it`, and the expected frames into `Fixtures/bend_it/expected_bend_it.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bend_it_reference.py
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

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bend_it"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"bend": (-360, 360), "start": (-1000, 1000), "end": (-1000, 1000)}
NUMBERS = ("bend", "start", "end")
NAMES = ("bend", "start", "end", "render_prestart", "distort")
EMPTY = [0.0] * 4
NEAREST_CUT = [math.inf]  # how near any pixel came to a cut, in pixels; checked below


# --- the rule -------------------------------------------------------------------------------

def point(p):
    return p[0] / 100 * W, p[1] / 100 * H


def cut(d):
    NEAREST_CUT[0] = min(NEAREST_CUT[0], abs(d))


def places(bend, start, end, prestart, distort, px, py):
    """Every (u, source x, source y) laid on the output point (px, py), farthest along first."""
    sx, sy = point(start)
    ex, ey = point(end)
    length = math.hypot(ex - sx, ey - sy)
    if length == 0:
        return [(0.0, px, py)]
    tx, ty = (ex - sx) / length, (ey - sy) / length
    nx, ny = -ty, tx

    def source(u, v):
        return sx + u * tx + v * nx, sy + u * ty + v * ny

    a, b = (px - sx) * tx + (py - sy) * ty, (px - sx) * nx + (py - sy) * ny
    if bend == 0:
        cut(a)
        cut(a - length)
        if prestart == "mirror":
            cut(a + length)
        if 0 <= a <= length or (a > length and distort == "extended"):
            return [(a, px, py)]
        if a < 0 and prestart in ("static", "bend"):
            return [(a, px, py)]
        if -length <= a < 0 and prestart == "mirror":
            return [(a, *source(-a, b))]
        return []

    theta = math.radians(bend)
    r = length / theta
    cx, cy = sx + r * nx, sy + r * ny
    qx, qy = px - cx, py - cy
    qt, qn = qx * tx + qy * ty, qx * nx + qy * ny
    size = math.hypot(qx, qy)
    first = math.atan2(-qt, qn)
    lo = -length if prestart in ("bend", "mirror") else 0.0
    out = []

    def frame(phi):
        c, s = math.cos(phi), math.sin(phi)
        t = (tx * c + nx * s, ty * c + ny * s)
        n = (nx * c - tx * s, ny * c - ty * s)
        centre = (cx - r * n[0], cy - r * n[1])
        return centre, t, n

    if distort == "extended":
        (ox, oy), t, n = frame(theta)
        a2 = (px - ox) * t[0] + (py - oy) * t[1]
        b2 = (px - ox) * n[0] + (py - oy) * n[1]
        cut(a2)
        if a2 > 0:
            out.append((length + a2, *source(length + a2, b2)))

    k = theta / length
    lo_phi, hi_phi = sorted((k * lo, k * length))
    js = range(math.floor((lo_phi - first) / math.pi) - 1, math.ceil((hi_phi - first) / math.pi) + 2)
    arc = []
    for j in js:
        phi = first + j * math.pi
        u = phi / k
        v = r + (-1) ** j * size
        cut(u - lo)
        cut(u - length)
        if lo <= u <= length:
            arc.append((u, *(source(-u, v) if u < 0 and prestart == "mirror" else source(u, v))))
    out += sorted(arc, reverse=True)

    if prestart == "static":
        cut(a)
        if a < 0:
            out.append((a, px, py))
    elif prestart == "bend":
        (ox, oy), t, n = frame(-theta)
        a2 = (px - ox) * t[0] + (py - oy) * t[1]
        b2 = (px - ox) * n[0] + (py - oy) * n[1]
        cut(a2)
        if a2 < 0:
            out.append((-length + a2, *source(-length + a2, b2)))
    return out


def over(layers):
    acc = [0.0] * 4
    for p in layers:
        k = 1 - acc[3]
        acc = [acc[i] + k * p[i] for i in range(4)]
    return acc


def bent(layer, s, x, y):
    """The output at pixel (x, y) of the drawing's own space; empty outside the drawing."""
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    found = places(s["bend"], s["start"], s["end"], s["render_prestart"], s["distort"],
                   x + 0.5, y + 0.5)
    return over([bilinear(layer, fx, fy) for _, fx, fy in found])


# --- the cases ------------------------------------------------------------------------------

def case(bend=45, start=(50, 100), end=(50, 0), prestart="none", distort="legal", shift=0):
    return {"drawing": "stripes", "bend": bend, "start": start, "end": end,
            "render_prestart": prestart, "distort": distort, "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        v = value_at(c[k], frame_no)
        if k in RANGES:
            lo, hi = RANGES[k]
            v = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                 else min(hi, max(lo, v)))
        held[k] = v
    return held


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    s = settings(c, frame_no)
    return [bent(layer, s, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


SHORT = {"start": (50, 80), "end": (50, 20)}  # a bar from row 8 up to row 2, rows 0, 1, 8, 9 off it

CASES = {
    "FX-BENDIT-001": ("The settings as they start: Bend 45, the bar standing up from the middle "
                      "of the bottom edge to the middle of the top, None, Legal. The drawing "
                      "leans and curves to the right as it rises, its top an eighth of a turn "
                      "over; the left part of the top rows, carried past the edge, is cut.",
                      case(), [0]),
    "FX-BENDIT-002": ("Bend 0: the straight bar, the whole drawing on it: untouched.",
                      case(bend=0), [0]),
    "FX-BENDIT-003": ("Bend 90: a quarter turn, the top of the bar lying flat to the right, the "
                      "circle's centre 6.37 pixels right of the bottom middle.",
                      case(bend=90), [0]),
    "FX-BENDIT-004": ("Bend -90: the same quarter turn to the left, the picture FX-BENDIT-003's "
                      "turned over left to right about the bar, as far as the drawing, "
                      "not quite symmetric about its middle, allows.",
                      case(bend=-90), [0]),
    "FX-BENDIT-005": ("Bend 360: the bar bent into a whole circle 1.59 pixels round its "
                      "centre, so the stripes' two ends meet; pixels the circle's sheets cross "
                      "twice show the one farther along the bar over the other.",
                      case(bend=360), [0]),
    "FX-BENDIT-006": ("Start 50, 80 and End 50, 20, a bar from row 8 to row 2, Bend 60, None, "
                      "Legal: what lies before the Start (rows 8 and 9) and past the End (rows "
                      "0 and 1) is not drawn; the six rows between bend.",
                      case(bend=60, **SHORT), [0]),
    "FX-BENDIT-007": ("The same with Render Prestart Static: rows 8 and 9, before the Start, "
                      "drawn unbent, under the bent part where it swings over them.",
                      case(bend=60, prestart="static", **SHORT), [0]),
    "FX-BENDIT-008": ("The same with Render Prestart Bend: the bend carried on back past the "
                      "Start the other way round the same circle, so rows 8 and 9 curve away "
                      "to the left below it.",
                      case(bend=60, prestart="bend", **SHORT), [0]),
    "FX-BENDIT-009": ("The same with Render Prestart Mirror: the bar's own Start to End stretch "
                      "bent back from the Start as Bend bends it, mirrored, so the rows just "
                      "above the Start reappear below it.",
                      case(bend=60, prestart="mirror", **SHORT), [0]),
    "FX-BENDIT-010": ("FX-BENDIT-006 with Distort Extended: rows 0 and 1, past the End, laid "
                      "straight on along the bent bar's last direction, 60 degrees over.",
                      case(bend=60, distort="extended", **SHORT), [0]),
    "FX-BENDIT-011": ("Start 0, 50 and End 100, 50, the bar lying across, Bend 90: its right "
                      "half curls down, n pointing down from a bar that runs to the right; "
                      "rows above and below the bar bend with it.",
                      case(bend=90, start=(0, 50), end=(100, 50)), [0]),
    "FX-BENDIT-012": ("Start and End the same point, 50, 50: no bar, the drawing untouched.",
                      case(bend=90, start=(50, 50), end=(50, 50)), [0]),
    "FX-BENDIT-013": ("Bend keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 the "
                      "drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003.",
                      case(bend=keyed((0, 0), (4, 90))), [0, 2, 4]),
    "FX-BENDIT-014": ("Bend 60, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar "
                      "shortens, the same turn in less length, a tighter curve; with Legal the "
                      "rows past the End go.",
                      case(bend=60, end=keyed((0, (50, 0)), (4, (50, 50)))), [0, 2, 4]),
    "FX-BENDIT-015": ("FX-BENDIT-003 moved three pixels right: the same, moved; the bend is "
                      "worked in the drawing's own space and moves with it, nothing grows, and "
                      "the three columns left of the drawing stay empty.",
                      case(bend=90, shift=3), [0]),
    "FX-BENDIT-016": ("Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that "
                      "overshoots: at frame 2 it would pass 360, is held at 360, and is "
                      "FX-BENDIT-005, as frame 4 is.",
                      case(bend=keyed((0, 0, OVERSHOOT), (4, 360))), [0, 2, 4]),
    "FX-BENDIT-017": ("Bend -150 on the short bar with Static and Extended: the bar curls back "
                      "on itself to the left, past the Start row, over the static rows, and "
                      "runs on straight from the End.",
                      case(bend=-150, prestart="static", distort="extended", **SHORT), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BENDIT-018": ("Bend 361, above 360.", case(bend=361)),
    "FX-BENDIT-019": ("Bend -361, below -360.", case(bend=-361)),
    "FX-BENDIT-020": ("A Render Prestart written \"Bend\", with a capital.", case(prestart="Bend")),
    "FX-BENDIT-021": ("A Distort written \"extend\".", case(distort="extend")),
    "FX-BENDIT-022": ("Start 50, 1001, past ten heights.", case(start=(50, 1001))),
    "FX-BENDIT-023": ("Bend keyed to 400 at frame 4.", case(bend=keyed((0, 0), (4, 400)))),
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
        "instance_id": "fx-0-0", "type_id": "core.bend_it", "enabled": True,
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

    (OUT / "expected_bend_it.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    # No pixel's centre sits within a millionth of a pixel of a cut, where the card's rounding
    # could put it on the other side.
    assert NEAREST_CUT[0] > 1e-6, NEAREST_CUT

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    # The forward bend and the backward search agree: a point of the bar, bent, is found again.
    s = case(bend=90)
    sx, sy = point(s["start"])
    length, theta = 10.0, math.pi / 2
    rr = length / theta
    for u, v in ((3.0, 1.0), (7.5, -2.0), (9.0, 4.0)):
        phi = u / rr
        # t = (0, -1), n = (1, 0): n(phi) = n cos - t sin
        nphi = (math.cos(phi), math.sin(phi))
        p = (sx + rr + (v - rr) * nphi[0], sy + (v - rr) * nphi[1])
        found = places(90, s["start"], s["end"], "none", "legal", *p)
        assert any(abs(fx - (sx + v)) < 1e-9 and abs(fy - (sy - u)) < 1e-9 for _, fx, fy in found)

    one = c["FX-BENDIT-001"]["0"]
    assert sum(one[i] != drawn[i] for i in range(W * H)) > 60
    assert c["FX-BENDIT-002"]["0"] == drawn and c["FX-BENDIT-012"]["0"] == drawn
    three, four = c["FX-BENDIT-003"]["0"], c["FX-BENDIT-004"]["0"]
    assert three != drawn and four != drawn and three != four
    # The bottom row barely bends: its middle pixels stay close to as drawn.
    assert near(three[at(8, 9)], drawn[at(8, 9)], 0.2)
    assert c["FX-BENDIT-005"]["0"] != three
    six, seven = c["FX-BENDIT-006"]["0"], c["FX-BENDIT-007"]["0"]
    eight, nine, ten = (c[f"FX-BENDIT-0{n:02d}"]["0"] for n in (8, 9, 10))
    # Row 9, before the Start: off with None; with Static, as drawn wherever nothing bent lies.
    assert sum(six[at(x, 9)] == EMPTY for x in range(W)) > 8
    assert all(seven[at(x, 9)] == drawn[at(x, 9)] for x in range(W) if six[at(x, 9)] == EMPTY)
    assert eight != six and nine != six and eight != nine and ten != six
    assert sum(ten[i][3] for i in range(W * H)) > sum(six[i][3] for i in range(W * H))
    assert c["FX-BENDIT-011"]["0"] != drawn
    thirteen = c["FX-BENDIT-013"]
    assert thirteen["0"] == drawn and thirteen["2"] == one and thirteen["4"] == three
    fourteen = c["FX-BENDIT-014"]
    assert fourteen["0"] == render(case(bend=60), 0)
    assert fourteen["4"] == render(case(bend=60, end=(50, 50)), 0) != fourteen["0"]
    fifteen = c["FX-BENDIT-015"]["0"]
    assert all(fifteen[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(fifteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    sixteen = c["FX-BENDIT-016"]
    assert sixteen["0"] == drawn and sixteen["2"] == c["FX-BENDIT-005"]["0"] == sixteen["4"]
    assert c["FX-BENDIT-017"]["0"] not in (six, seven, ten)
    print("checked")


if __name__ == "__main__":
    main()
