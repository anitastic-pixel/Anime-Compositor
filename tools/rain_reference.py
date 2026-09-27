"""Rain, worked a second way.

D-163 adds `core.rain`, a generator: falling rain drawn inside a layer's covering, on a solid or
on any drawing. The drawing's own space is laid with a field of square cells `spacing` pixels
wide, turned so that its rows run across the fall; each cell holds a drop or not, by `density`
per cent and the seed, and a drop is a short straight streak `length` pixels long and `width`
pixels thick, lying along the fall, at its own place in its cell and its own strength between a
half and the whole. At each pixel that shows, the strongest streak reaching it mixes the rain's
`color` into the pixel, normal, at that strength times `opacity` per cent. `direction` is the
way the rain falls, 180 straight down, 170 as it starts, drifting a little right as it falls; the
whole field slides `speed` pixels a frame along that way, so the rain falls. Every pixel keeps
its own covering, and a pixel that does not show stays as it is. The field is fixed to the
drawing, so it moves with the layer. It is this program's own method, modelled on the rain of
After Effects' CC Rainfall and anime compositing; nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

The rule. t = u(direction) = (sin direction, -cos direction), exact at whole multiples of 90,
and n = (-t.y, t.x). At a pixel with a > 0, (X, Y) its centre in the drawing's own coordinates,
its field point is (A, B) = (n . (X, Y), t . (X, Y) - speed * frame), frame the composition
frame, a hidden value. Cell (i, j) holds a drop when (U(seed, i, j, 0, 0) + 1) / 2 < density /
100, U Noise's hash (D-119, `tools/noise_reference.py`); the drop's centre is (spacing (i +
(U(seed, i, j, 0, 1) + 1) / 2), spacing (j + (U(seed, i, j, 0, 2) + 1) / 2)) and its strength
beta = 0.5 + 0.25 (U(seed, i, j, 0, 3) + 1). The drop is the segment from its centre - length /
2 to + length / 2 along B; delta is the distance from (A, B) to it, and q is the largest of
clamp(width / 2 - delta + 0.5, 0, 1) beta over the drops (a cell further than width / 2 + 0.5
across or length / 2 + width / 2 + 0.5 along cannot reach, and is skipped). With b the straight
linear colour, G the colour's linear value and op = q opacity / 100, the output is ((b + op (G -
b)) a, a): the blend mix, normal. The layer does not grow; a draft scales spacing, length, width
and speed.

A drop's being there is a yes-or-no choice from a number: `check` asserts that no cell any case
reaches has (U + 1) / 2 within 1e-5 of density / 100.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a solid night of the line's colour #1e1a24 covering the frame, or Noise's card to
show the rain stays inside a drawing's covering. The drawings go into `Fixtures/rain/media`, the
projects into `Fixtures/rain`, and the expected frames into `Fixtures/rain/expected_rain.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/rain_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from noise_reference import u as U  # noqa: E402
import noise_reference as N  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "rain"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-5      # the spec's margin: no reached cell this close to the density's cliff
RANGES = {"density": (0, 100), "spacing": (2, 1000), "length": (0, 1000), "width": (0, 20),
          "direction": (-3600, 3600), "speed": (0, 1000), "seed": (0, 100000),
          "opacity": (0, 100)}
FLOORED = ("seed",)
NAMES = ("color", "density", "spacing", "length", "width", "direction", "speed", "seed",
         "opacity")


# --- the rule -------------------------------------------------------------------------------

def unit(deg):
    """u(theta): 0 up, 90 right, exact at whole multiples of 90."""
    if deg % 90 == 0:
        return [(0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)][int(deg // 90) % 4]
    r = math.radians(deg)
    return math.sin(r), -math.cos(r)


def field_point(X, Y, direction, speed, frame_no):
    t = unit(direction)
    n = (-t[1], t[0])
    return n[0] * X + n[1] * Y, t[0] * X + t[1] * Y - speed * frame_no


def cells(A, B, spacing, length, width):
    """Every cell whose drop could reach the field point (A, B)."""
    r = width / 2 + 0.5
    for i in range(math.floor((A - r) / spacing), math.floor((A + r) / spacing) + 1):
        for j in range(math.floor((B - length / 2 - r) / spacing),
                       math.floor((B + length / 2 + r) / spacing) + 1):
            yield i, j


def strength(A, B, density, spacing, length, width, seed):
    """q at the field point (A, B): the strongest drop's reach, times its strength."""
    q = 0.0
    for i, j in cells(A, B, spacing, length, width):
        if (U(seed, i, j, 0, 0) + 1) / 2 >= density / 100:
            continue
        cx = spacing * (i + (U(seed, i, j, 0, 1) + 1) / 2)
        cy = spacing * (j + (U(seed, i, j, 0, 2) + 1) / 2)
        beta = 0.5 + 0.25 * (U(seed, i, j, 0, 3) + 1)
        delta = math.hypot(A - cx, max(0.0, abs(B - cy) - length / 2))
        q = max(q, min(1.0, max(0.0, width / 2 - delta + 0.5)) * beta)
    return q


def rain(pixels, color, density, spacing, length, width, direction, speed, seed, opacity,
         frame_no, width_px=W):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, `width_px` wide, the drawing's
    top-left pixel at (0, 0). Numbers are already held; seed already floored."""
    G = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        A, B = field_point(i % width_px + 0.5, i // width_px + 0.5, direction, speed, frame_no)
        op = strength(A, B, density, spacing, length, width, seed) * opacity / 100
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) for c in range(3)]
        out.append([(b[c] + op * (G[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawings ---------------------------------------------------------------------------

NIGHT = R.LINE  # #1e1a24, the line's colour, a night sky
DRAWINGS = {"night": [[NIGHT] * W for _ in range(H)],
            "card": N.DRAWINGS["card"]}  # Noise's card: grey, skin, white, black, a soft edge


# --- the cases ------------------------------------------------------------------------------

RAIN = "#c8d8ff"


def case(drawing="night", color=RAIN, density=30, spacing=24, length=20, width=1,
         direction=170, speed=30, seed=0, opacity=60, shift=0):
    return {"drawing": drawing, "color": color, "density": density, "spacing": spacing,
            "length": length, "width": width, "direction": direction, "speed": speed,
            "seed": seed, "opacity": opacity, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    n = {k: held(c, k, frame_no) for k in RANGES}
    return R.frame(rain(pixels, c["color"], n["density"], n["spacing"], n["length"], n["width"],
                        n["direction"], n["speed"], n["seed"], n["opacity"], frame_no),
                   c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(drawing=c["drawing"], opacity=0, shift=c["shift"]), 0)


FINE = {"spacing": 4, "length": 5}  # small cells and short drops, so many fall in the frame

CASES = {
    "FX-RAIN-001": ("The settings as they start: colour #c8d8ff, density 30, spacing 24, "
                    "length 20, width 1, direction 170, speed 30, seed 0, opacity 60, on the "
                    "night solid. The frame is smaller than one cell of the field, so it sees "
                    "few drops: none at frame 0, one long pale streak drifting a little right, at "
                    "frame 3, and none again at frame 4, the rain having fallen 30 pixels a "
                    "frame past it.",
                    case(), [0, 3, 4]),
    "FX-RAIN-002": ("Spacing 4, length 5: many short drops across the frame.",
                    case(**FINE), [0]),
    "FX-RAIN-003": ("Spacing 4, length 5, density 0: no cell holds a drop, so the solid is "
                    "untouched at every frame.",
                    case(density=0, **FINE), [0, 4]),
    "FX-RAIN-004": ("Spacing 4, length 5, density 100: every cell holds a drop, FX-RAIN-002's "
                    "among them in the same places, so every pixel is at least as rainy as "
                    "there.",
                    case(density=100, **FINE), [0]),
    "FX-RAIN-005": ("Spacing 4, length 5, opacity 100: FX-RAIN-002's drops at five thirds of "
                    "their strength.",
                    case(opacity=100, **FINE), [0]),
    "FX-RAIN-006": ("Spacing 4, length 5, opacity 0: the solid, untouched.",
                    case(opacity=0, **FINE), [0]),
    "FX-RAIN-007": ("Spacing 4, length 5, width 3: FX-RAIN-002's drops thicker, every pixel at "
                    "least as rainy.",
                    case(width=3, **FINE), [0]),
    "FX-RAIN-008": ("Spacing 4, length 5, width 0: hairline drops, never more than half their "
                    "strength, every pixel at most as rainy as FX-RAIN-002.",
                    case(width=0, **FINE), [0]),
    "FX-RAIN-009": ("Spacing 4, length 0: each drop a dot, every pixel at most as rainy as "
                    "FX-RAIN-002.",
                    case(spacing=4, length=0), [0]),
    "FX-RAIN-010": ("Spacing 4, length 5, direction 180, speed 1: falling straight down one "
                    "pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 "
                    "four; the streaks run down the columns.",
                    case(direction=180, speed=1, **FINE), [0, 1, 4]),
    "FX-RAIN-011": ("FX-RAIN-010 with direction 540, a whole turn past 180: the same.",
                    case(direction=540, speed=1, **FINE), [0, 1, 4]),
    "FX-RAIN-012": ("Spacing 4, length 5, direction 90, speed 0: falling to the right, the "
                    "streaks run along the rows; with speed 0 frame 4 is frame 0.",
                    case(direction=90, speed=0, **FINE), [0, 4]),
    "FX-RAIN-013": ("Spacing 4, length 5, seed 7: drops of their own.",
                    case(seed=7, **FINE), [0]),
    "FX-RAIN-014": ("Spacing 4, length 5, seed 7.9, which counts as 7: FX-RAIN-013.",
                    case(seed=7.9, **FINE), [0]),
    "FX-RAIN-015": ("FX-RAIN-002 with its colour written in capitals, #C8D8FF: the same.",
                    case(color=RAIN.upper(), **FINE), [0]),
    "FX-RAIN-016": ("Spacing 4, length 5, opacity keyed from 0 at frame 0 to 100 at frame 4, "
                    "linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100.",
                    case(opacity=keyed((0, 0), (4, 100)), **FINE), [0, 2, 4]),
    "FX-RAIN-017": ("Spacing 4, length 5, density keyed from 0 at frame 0 to 100 at frame 4, "
                    "eased past its end: frame 0 is the solid; frame 2 would pass 100, is held "
                    "at 100, and is density 100 at frame 2.",
                    case(density=keyed((0, 0, OVERSHOOT), (4, 100)), **FINE), [0, 2]),
    "FX-RAIN-018": ("FX-RAIN-002 moved three pixels right: the field is the drawing's own, so "
                    "the rain moves with it, and the three columns left bare stay empty.",
                    case(shift=3, **FINE), [0]),
    "FX-RAIN-019": ("FX-RAIN-002 on Noise's card: the same drops, each pixel that shows mixed "
                    "toward the rain as much as on the solid, the soft edge at its own half "
                    "covering, and the empty column and row stay empty.",
                    case(drawing="card", **FINE), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RAIN-020": ("Density 101, above 100.", case(density=101)),
    "FX-RAIN-021": ("Spacing 1, below 2.", case(spacing=1)),
    "FX-RAIN-022": ("Length -1, below 0.", case(length=-1)),
    "FX-RAIN-023": ("Width 21, above 20.", case(width=21)),
    "FX-RAIN-024": ("Direction 3601, past ten turns.", case(direction=3601)),
    "FX-RAIN-025": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-RAIN-026": ("A colour written \"#c8d8f\", one digit short.", case(color="#c8d8f")),
    "FX-RAIN-027": ("Speed keyed to 1001 at frame 4, above 1000.",
                    case(speed=keyed((0, 30), (4, 1001)))),
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
        "instance_id": "fx-0-0", "type_id": "core.rain", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_rain.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    G = [srgb_to_linear(v / 255) for v in R.hex_color(RAIN)]
    night = plain(case())

    def ops(f, base):
        """The mix op at each pixel that shows, read back from a frame; None where empty."""
        out = []
        for p, d in zip(f, base):
            if d[3] == 0:
                out.append(None)
                continue
            b = [v / d[3] for v in d[:3]]
            got = [(p[k] / d[3] - b[k]) / (G[k] - b[k]) for k in range(3)
                   if abs(G[k] - b[k]) > 1e-6]
            assert max(got) - min(got) < 1e-9 and p[3] == d[3], (got, p, d)
            out.append(got[0])
        return out

    # The rule's own pieces.
    assert unit(0) == (0.0, -1.0) and unit(90) == (1.0, 0.0) and unit(180) == (0.0, 1.0)
    assert unit(-90) == (-1.0, 0.0) and unit(540) == unit(180)
    assert field_point(2.5, 3.5, 180, 1, 2) == (-2.5, 1.5)
    assert strength(1.0, 5.0, 0, 4, 5, 1, 0) == 0
    # A drop alone: at its centre, q is its strength; past its end by the half width and a half
    # pixel, nothing.
    i, j = next((i, j) for i in range(50) for j in range(50) if U(0, i, j, 0, 0) < -0.5)
    cx, cy = 400 * (i + (U(0, i, j, 0, 1) + 1) / 2), 400 * (j + (U(0, i, j, 0, 2) + 1) / 2)
    beta = 0.5 + 0.25 * (U(0, i, j, 0, 3) + 1)
    assert 0.5 <= beta <= 1 and strength(cx, cy, 30, 400, 0, 1, 0) == beta
    assert strength(cx, cy + 3, 30, 400, 6, 1, 0) == beta  # along the streak, still whole
    assert strength(cx, cy + 4, 30, 400, 6, 1, 0) == 0     # 3 + 0.5 + 0.5 past the centre
    assert strength(cx + 0.75, cy, 30, 400, 6, 1, 0) == 0.25 * beta  # a quarter pixel's reach

    # The cliff: no cell any case reaches holds its drop by a hair.
    margin = 1.0
    for fx, (says, cs, frames) in CASES.items():
        for f in frames:
            n = {k: held(cs, k, f) for k in RANGES}
            for i in range(W * H):
                A, B = field_point(i % W + 0.5, i // W + 0.5, n["direction"], n["speed"], f)
                for ci, cj in cells(A, B, n["spacing"], n["length"], n["width"]):
                    margin = min(margin, abs((U(n["seed"], ci, cj, 0, 0) + 1) / 2
                                             - n["density"] / 100))
    assert margin > CLIFF, margin
    print(f"closest to the cliff: {margin:.3g}")

    # Every case keeps every covering and every empty pixel, and moves each pixel toward the
    # rain by at most opacity / 100.
    for fx, frames in c.items():
        cs = CASES[fx][1] if fx in CASES else INVALID[fx][1]
        base = plain(cs)
        for f, px in frames.items():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx in CASES:
                assert all(o is None or -1e-12 <= o <= held(cs, "opacity", int(f)) / 100 + 1e-12
                           for o in ops(px, base)), fx
        if "warning" in expected["cases"][fx]:
            assert all(px == base for px in frames.values())

    q = lambda fx, f="0", op=0.6: [o / op for o in ops(c[fx][f], night)]  # noqa: E731
    rainy = lambda qs: sum(v > 1e-12 for v in qs if v is not None)  # noqa: E731
    at_least = lambda hi, lo: all(h >= l - 1e-12 for h, l in zip(hi, lo))  # noqa: E731

    one = c["FX-RAIN-001"]
    assert one["0"] == one["4"] == night and 10 <= rainy(q("FX-RAIN-001", "3")) <= 20
    two = c["FX-RAIN-002"]["0"]
    q2 = q("FX-RAIN-002")
    assert rainy(q2) >= 20 and max(q2) <= 1
    assert c["FX-RAIN-003"]["0"] == c["FX-RAIN-003"]["4"] == night
    q4 = q("FX-RAIN-004")
    assert at_least(q4, q2) and rainy(q4) > rainy(q2)
    assert all(abs(a - b) < 1e-9 for a, b in zip(q("FX-RAIN-005", op=1.0), q2))
    assert c["FX-RAIN-006"]["0"] == night
    q7, q8, q9 = q("FX-RAIN-007"), q("FX-RAIN-008"), q("FX-RAIN-009")
    assert at_least(q7, q2) and rainy(q7) > rainy(q2)
    assert at_least(q2, q8) and max(q8) <= 0.5 + 1e-12 and rainy(q8) > 0
    assert at_least(q2, q9) and rainy(q9) < rainy(q2)
    # Straight down, one pixel a frame.
    ten = {f: q("FX-RAIN-010", f) for f in "014"}
    for f in (1, 4):
        for y in range(f, H):
            for x in range(W):
                assert ten[str(f)][at(x, y)] == ten["0"][at(x, y - f)], (f, x, y)
    assert c["FX-RAIN-011"] == c["FX-RAIN-010"]

    def runs(qs):
        """Neighbouring rainy pairs along the rows and down the columns."""
        r = lambda i: qs[i] > 0.05  # noqa: E731
        across = sum(r(at(x, y)) and r(at(x + 1, y)) for x in range(W - 1) for y in range(H))
        down = sum(r(at(x, y)) and r(at(x, y + 1)) for x in range(W) for y in range(H - 1))
        return across, down
    a10, d10 = runs(ten["0"])
    twelve = c["FX-RAIN-012"]
    a12, d12 = runs(q("FX-RAIN-012"))
    assert d10 > a10 and a12 > d12 and twelve["0"] == twelve["4"]
    thirteen = c["FX-RAIN-013"]["0"]
    assert thirteen != two and c["FX-RAIN-014"]["0"] == thirteen
    assert c["FX-RAIN-015"]["0"] == two
    sixteen = c["FX-RAIN-016"]
    assert sixteen["0"] == night
    assert sixteen["2"] == render(case(opacity=50, **FINE), 2)
    assert sixteen["4"] == render(case(opacity=100, **FINE), 4)
    seventeen = c["FX-RAIN-017"]
    assert ease(OVERSHOOT, 0.5) > 1 and seventeen["0"] == night
    assert seventeen["2"] == render(case(density=100, **FINE), 2)
    moved = c["FX-RAIN-018"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    card = plain(case(drawing="card"))
    qc = [None if o is None else o / 0.6 for o in ops(c["FX-RAIN-019"]["0"], card)]
    shown = [i for i in range(W * H) if card[i][3] > 0]
    assert all(abs(qc[i] - q2[i]) < 1e-9 for i in shown) and rainy([qc[i] for i in shown])
    assert c["FX-RAIN-019"]["0"][at(15, 4)][3] == 128 / 255
    assert rainy([qc[at(15, y)] for y in range(9)]) > 0  # the soft edge takes rain too
    print("checked")


if __name__ == "__main__":
    main()
