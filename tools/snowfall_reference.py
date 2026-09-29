"""Snowfall, worked a second way.

D-204 adds `core.snowfall`, a generator: falling snow drawn inside a layer's covering, on a solid
or on any drawing, as Rain (D-163) draws rain. The snow lies in three planes, near, middle and
far. Each plane is a field of square cells laid on the drawing's own space; each cell holds a
flake or not, by `density` per cent and the seed, and a flake is a soft round dot at its own
place in its cell, its own width between a half and the whole of `size`, swaying from side to
side by up to `wiggle` pixels once every `period` frames, at its own point in the sway. The near
plane is at full scale; the others are nearer the far end by `depth` per cent: at depth 100 the
far plane is a third of the near one's scale. A plane's scale shrinks its cells, its flakes, its
fall, its drift and its sway alike, so far snow is finer, closer together and slower. Every plane
falls `speed` pixels a frame at its scale and drifts `wind` pixels a frame to the right (left
when negative). At each pixel that shows, the brightest flake of any plane mixes the snow's
`color` into the pixel, normal, at that brightness times `opacity` per cent. Every pixel keeps its
own covering, and a pixel that does not show stays as it is. The planes are fixed to the drawing,
so the snow moves with the layer. It is this program's own method, modelled on After Effects' CC
Snowfall; nothing is ported. Document 21 is the rule in words; this file is the reference for
the numbers document 25 pins against it.

The rule. For plane l = 0, 1, 2, its scale z = 1 - depth / 100 * l / 3, and s = spacing z,
rmax = size z / 2 and wz = wiggle z. At a pixel with a > 0, (X, Y) its centre in the drawing's own
coordinates and f the composition frame, a hidden value, the plane's field point is
(A, B) = (X - wind z f, Y - speed z f). Cell (i, j) holds a flake when
(U(seed, i, j, l, 0) + 1) / 2 < density / 100, U Noise's hash (D-119, `tools/noise_reference.py`),
the plane in its frame's place; with phi = (U(seed, i, j, l, 4) + 1) / 2, the flake's centre is
(s (i + (U(seed, i, j, l, 1) + 1) / 2) + wz sin(2 pi (f / period + phi)),
s (j + (U(seed, i, j, l, 2) + 1) / 2)) and its radius r = rmax (0.5 + 0.25 (U(seed, i, j, l, 3) + 1)).
With d the distance from (A, B) to its centre, its light is clamp(r + 0.5 - d, 0, 1) min(1, 2 r),
and q is the largest light of any flake of any plane (a cell whose flake cannot come within
rmax + 0.5, its sway counted, is skipped). With b the straight linear colour, G the colour's
linear value and op = q opacity / 100, the output is ((b + op (G - b)) a, a): the blend mix,
normal. The layer does not grow; a draft scales spacing, size, speed, wind and wiggle.

A flake's being there is a yes-or-no choice from a number: `check` asserts that no cell any case
reaches has (U + 1) / 2 within 1e-5 of density / 100.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build writes single precision to its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Rain's night solid, #1e1a24, covering the frame, or Noise's card, to show the snow
stays inside a drawing's covering. The drawings go into `Fixtures/snowfall/media`, the projects
into `Fixtures/snowfall`, and the expected frames into `Fixtures/snowfall/expected_snowfall.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/snowfall_reference.py
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
import rain_reference as RAIN  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "snowfall"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-5      # the spec's margin: no reached cell this close to the density's cliff
RANGES = {"density": (0, 100), "spacing": (2, 1000), "size": (0, 100), "depth": (0, 100),
          "speed": (0, 1000), "wind": (-1000, 1000), "wiggle": (0, 100), "period": (1, 1000),
          "seed": (0, 100000), "opacity": (0, 100)}
FLOORED = ("seed",)
NAMES = ("color",) + tuple(RANGES)
PLANES = 3


# --- the rule -------------------------------------------------------------------------------

def scale(depth, plane):
    """z: the near plane whole, each further one nearer a third by depth per cent."""
    return 1 - depth / 100 * plane / 3


def plane_cells(A, B, s, rmax, wz):
    """Every cell of a plane whose flake could reach its field point (A, B)."""
    r = rmax + 0.5
    for i in range(math.floor((A - r - wz) / s), math.floor((A + r + wz) / s) + 1):
        for j in range(math.floor((B - r) / s), math.floor((B + r) / s) + 1):
            yield i, j


def planes(n, X, Y, frame_no):
    """Each plane's number, scale, cell size, largest radius, sway and field point at (X, Y)."""
    for p in range(PLANES):
        z = scale(n["depth"], p)
        yield (p, z, n["spacing"] * z, n["size"] * z / 2, n["wiggle"] * z,
               X - n["wind"] * z * frame_no, Y - n["speed"] * z * frame_no)


def light(n, X, Y, frame_no):
    """q at the pixel centre (X, Y): the brightest flake of any plane reaching it."""
    q = 0.0
    for p, z, s, rmax, wz, A, B in planes(n, X, Y, frame_no):
        for i, j in plane_cells(A, B, s, rmax, wz):
            def u(ch):
                return U(n["seed"], i, j, p, ch)
            if (u(0) + 1) / 2 >= n["density"] / 100:
                continue
            phi = (u(4) + 1) / 2
            cx = s * (i + (u(1) + 1) / 2) + wz * math.sin(2 * math.pi * (frame_no / n["period"] + phi))
            cy = s * (j + (u(2) + 1) / 2)
            r = rmax * (0.5 + 0.25 * (u(3) + 1))
            d = math.hypot(A - cx, B - cy)
            q = max(q, min(1.0, max(0.0, r + 0.5 - d)) * min(1.0, 2 * r))
    return q


def snow(pixels, color, n, frame_no, width_px=W):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, `width_px` wide, the drawing's
    top-left pixel at (0, 0). `n` holds the numbers, already held; seed already floored."""
    G = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        op = light(n, i % width_px + 0.5, i // width_px + 0.5, frame_no) * n["opacity"] / 100
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) for c in range(3)]
        out.append([(b[c] + op * (G[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = RAIN.DRAWINGS  # the night solid and Noise's card
SNOW = "#ffffff"


def case(drawing="night", color=SNOW, density=50, spacing=32, size=6, depth=50, speed=2,
         wind=0.5, wiggle=3, period=48, seed=0, opacity=100, shift=0):
    return {"drawing": drawing, "color": color, "density": density, "spacing": spacing,
            "size": size, "depth": depth, "speed": speed, "wind": wind, "wiggle": wiggle,
            "period": period, "seed": seed, "opacity": opacity, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    n = {k: held(c, k, frame_no) for k in RANGES}
    return R.frame(snow(pixels, c["color"], n, frame_no), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(drawing=c["drawing"], opacity=0, shift=c["shift"]), 0)


FINE = {"spacing": 6, "size": 3}         # small cells, so a few flakes of each plane show
FLAT = dict(FINE, depth=0, wiggle=0, wind=0, speed=0)  # three planes alike, and still

CASES = {
    "FX-SNOW-001": ("The settings as they start: colour #ffffff, density 50, spacing 32, size 6, "
                    "scene depth 50, speed 2, wind 0.5, wiggle 3 every 48 frames, seed 0, "
                    "opacity 100, on the night solid. The frame is smaller than one near cell, "
                    "so it sees a few flakes, falling and drifting a little right between frame "
                    "0 and frame 4.",
                    case(), [0, 4]),
    "FX-SNOW-002": ("Spacing 6, size 3: flakes of all three planes across the frame, the far "
                    "ones smaller, falling two pixels a frame, the far ones slower.",
                    case(**FINE), [0, 2]),
    "FX-SNOW-003": ("Spacing 6, size 3, density 0: no cell holds a flake, so the solid is "
                    "untouched at every frame.",
                    case(density=0, **FINE), [0, 4]),
    "FX-SNOW-004": ("Spacing 6, size 3, density 100: every cell holds a flake, FX-SNOW-002's "
                    "among them in the same places, so every pixel is at least as snowy as "
                    "there.",
                    case(density=100, **FINE), [0]),
    "FX-SNOW-005": ("Spacing 6, size 3, opacity 50: FX-SNOW-002's flakes at half their "
                    "strength.",
                    case(opacity=50, **FINE), [0]),
    "FX-SNOW-006": ("Spacing 6, size 3, opacity 0: the solid, untouched.",
                    case(opacity=0, **FINE), [0]),
    "FX-SNOW-007": ("Spacing 6, size 0: flakes of no size give no light; the solid, untouched "
                    "at every frame.",
                    case(spacing=6, size=0), [0, 4]),
    "FX-SNOW-008": ("Spacing 6, size 3, scene depth 0: the three planes all at the near scale, "
                    "each its own flakes.",
                    case(depth=0, **FINE), [0]),
    "FX-SNOW-009": ("Spacing 6, size 3, scene depth 100: the far plane at a third of the near "
                    "scale, its flakes a pixel wide at most and close together.",
                    case(depth=100, **FINE), [0]),
    "FX-SNOW-010": ("Spacing 6, size 3, scene depth 0, no wind and no wiggle, speed 1: every "
                    "plane falls straight down one pixel a frame, so frame 1 is frame 0 moved one "
                    "pixel down, and frame 4 four.",
                    case(**dict(FLAT, speed=1)), [0, 1, 4]),
    "FX-SNOW-011": ("Spacing 6, size 3, scene depth 0, no wiggle, speed 0, wind -1: the snow "
                    "blown left one pixel a frame, so frame 2 is frame 0 moved two pixels left.",
                    case(**dict(FLAT, wind=-1)), [0, 2]),
    "FX-SNOW-012": ("Spacing 6, size 3, scene depth 0, speed 0, no wind, wiggle 2 every 4 "
                    "frames: the flakes sway where they hang, so frame 2 differs and frame 4, "
                    "one whole sway later, is frame 0.",
                    case(**dict(FLAT, wiggle=2, period=4)), [0, 2, 4]),
    "FX-SNOW-013": ("Spacing 6, size 3, still: no fall, no wind, no wiggle, so frame 4 is "
                    "frame 0.",
                    case(**FLAT), [0, 4]),
    "FX-SNOW-014": ("Spacing 6, size 3, seed 7: flakes of their own.",
                    case(seed=7, **FINE), [0]),
    "FX-SNOW-015": ("Spacing 6, size 3, seed 7.9, which counts as 7: FX-SNOW-014.",
                    case(seed=7.9, **FINE), [0]),
    "FX-SNOW-016": ("Spacing 6, size 3, colour a pale blue #a0c8ff written in capitals, "
                    "#A0C8FF: FX-SNOW-002's flakes at the same strengths, in the blue.",
                    case(color="#A0C8FF", **FINE), [0]),
    "FX-SNOW-017": ("Spacing 6, size 3, opacity keyed from 0 at frame 0 to 100 at frame 4, "
                    "linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100.",
                    case(opacity=keyed((0, 0), (4, 100)), **FINE), [0, 2, 4]),
    "FX-SNOW-018": ("Spacing 6, size 3, density keyed from 0 at frame 0 to 100 at frame 4, "
                    "eased past its end: frame 0 is the solid; frame 2 would pass 100, is held "
                    "at 100, and is density 100 at frame 2.",
                    case(density=keyed((0, 0, OVERSHOOT), (4, 100)), **FINE), [0, 2]),
    "FX-SNOW-019": ("FX-SNOW-002 moved three pixels right: the planes are the drawing's own, "
                    "so the snow moves with it, and the three columns left bare stay empty.",
                    case(shift=3, **FINE), [0]),
    "FX-SNOW-020": ("FX-SNOW-002 on Noise's card: the same flakes, each pixel that shows mixed "
                    "toward the snow as much as on the solid, the soft edge at its own half "
                    "covering, and the empty column and row stay empty.",
                    case(drawing="card", **FINE), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SNOW-021": ("Density 101, above 100.", case(density=101)),
    "FX-SNOW-022": ("Spacing 1, below 2.", case(spacing=1)),
    "FX-SNOW-023": ("Size 101, above 100.", case(size=101)),
    "FX-SNOW-024": ("Scene depth -1, below 0.", case(depth=-1)),
    "FX-SNOW-025": ("Wind -1001, past -1000.", case(wind=-1001)),
    "FX-SNOW-026": ("Wiggle 101, above 100.", case(wiggle=101)),
    "FX-SNOW-027": ("Period 0.5, below 1.", case(period=0.5)),
    "FX-SNOW-028": ("A colour written \"#fffff\", one digit short.", case(color="#fffff")),
    "FX-SNOW-029": ("Opacity keyed to 150 at frame 4, above 100.",
                    case(opacity=keyed((0, 100), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.snowfall", "enabled": True,
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

    (OUT / "expected_snowfall.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    night = plain(case())

    def ops(f, base, color=SNOW):
        """The mix op at each pixel that shows, read back from a frame; None where empty."""
        G = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
        out = []
        for p, d in zip(f, base):
            if d[3] == 0:
                out.append(None)
                continue
            b = [v / d[3] for v in d[:3]]
            got = [(p[k] / d[3] - b[k]) / (G[k] - b[k]) for k in range(3)
                   if abs(G[k] - b[k]) > 1e-6]
            if not got:  # the pixel is the snow's own colour already
                out.append(None)
                continue
            assert max(got) - min(got) < 1e-9 and p[3] == d[3], (got, p, d)
            out.append(got[0])
        return out

    # The rule's own pieces.
    assert scale(50, 0) == 1 and scale(0, 2) == 1 and abs(scale(100, 2) - 1 / 3) < 1e-15
    lone = {k: held(case(spacing=400, size=10, depth=0, wiggle=0, speed=0, wind=0), k, 0)
            for k in RANGES}
    i, j = next((i, j) for i in range(50) for j in range(50)
                if U(0, i, j, 0, 0) < 0 and all(U(0, i, j, p, 0) > 0 for p in (1, 2)))
    # Where the near plane's flake is, and every plane at the same scale: its light is whole at
    # its centre and a quarter a quarter pixel inside its edge.
    cx, cy = 400 * (i + (U(0, i, j, 0, 1) + 1) / 2), 400 * (j + (U(0, i, j, 0, 2) + 1) / 2)
    r = 5 * (0.5 + 0.25 * (U(0, i, j, 0, 3) + 1))
    assert light(lone, cx, cy, 0) == 1
    assert abs(light(lone, cx + r + 0.25, cy, 0) - 0.25) < 1e-9

    # The cliff: no cell any case reaches holds its flake by a hair.
    margin = 1.0
    for fx, (says, cs, frames) in CASES.items():
        for f in frames:
            n = {k: held(cs, k, f) for k in RANGES}
            for i in range(W * H):
                for p, z, s, rmax, wz, A, B in planes(n, i % W + 0.5, i // W + 0.5, f):
                    for ci, cj in plane_cells(A, B, s, rmax, wz):
                        margin = min(margin, abs((U(n["seed"], ci, cj, p, 0) + 1) / 2
                                                 - n["density"] / 100))
    assert margin > CLIFF, margin
    print(f"closest to the cliff: {margin:.3g}")

    # Every case keeps every covering and every empty pixel, and moves each pixel toward the
    # snow by at most opacity / 100.
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
                           for o in ops(px, base, cs["color"])), fx
        if "warning" in expected["cases"][fx]:
            assert all(px == base for px in frames.values())

    q = lambda fx, f="0", op=1.0, color=SNOW: [o / op for o in ops(c[fx][f], night, color)]  # noqa: E731
    snowy = lambda qs: sum(v > 1e-12 for v in qs if v is not None)  # noqa: E731
    at_least = lambda hi, lo: all(h >= l - 1e-12 for h, l in zip(hi, lo))  # noqa: E731
    near = lambda a, b: all(abs(u - v) < 1e-12 for p, r in zip(a, b) for u, v in zip(p, r))  # noqa: E731

    one = c["FX-SNOW-001"]
    assert 0 < snowy(q("FX-SNOW-001")) and one["0"] != one["4"]
    two = c["FX-SNOW-002"]["0"]
    q2 = q("FX-SNOW-002")
    assert snowy(q2) >= 20 and max(q2) <= 1 and c["FX-SNOW-002"]["2"] != two
    assert c["FX-SNOW-003"]["0"] == c["FX-SNOW-003"]["4"] == night
    q4 = q("FX-SNOW-004")
    assert at_least(q4, q2) and snowy(q4) > snowy(q2)
    assert all(abs(a - b) < 1e-9 for a, b in zip(q("FX-SNOW-005", op=0.5), q2))
    assert c["FX-SNOW-006"]["0"] == night
    assert c["FX-SNOW-007"]["0"] == c["FX-SNOW-007"]["4"] == night
    assert c["FX-SNOW-008"]["0"] not in (two, night) and c["FX-SNOW-009"]["0"] not in (two, night)
    # Straight down, one pixel a frame; blown left, one pixel a frame.
    ten = {f: q("FX-SNOW-010", f) for f in "014"}
    for f in (1, 4):
        for y in range(f, H):
            for x in range(W):
                assert abs(ten[str(f)][at(x, y)] - ten["0"][at(x, y - f)]) < 1e-9, (f, x, y)
    eleven = {f: q("FX-SNOW-011", f) for f in "02"}
    for y in range(H):
        for x in range(W - 2):
            assert abs(eleven["2"][at(x, y)] - eleven["0"][at(x + 2, y)]) < 1e-9, (x, y)
    twelve = c["FX-SNOW-012"]
    assert near(twelve["4"], twelve["0"]) and not near(twelve["2"], twelve["0"])
    assert c["FX-SNOW-013"]["0"] == c["FX-SNOW-013"]["4"]
    fourteen = c["FX-SNOW-014"]["0"]
    assert fourteen != two and c["FX-SNOW-015"]["0"] == fourteen
    assert all(abs(a - b) < 1e-9 for a, b in zip(q("FX-SNOW-016", color="#a0c8ff"), q2))
    seventeen = c["FX-SNOW-017"]
    assert seventeen["0"] == night
    assert seventeen["2"] == render(case(opacity=50, **FINE), 2)
    assert seventeen["4"] == render(case(opacity=100, **FINE), 4)
    eighteen = c["FX-SNOW-018"]
    assert ease(OVERSHOOT, 0.5) > 1 and eighteen["0"] == night
    assert eighteen["2"] == render(case(density=100, **FINE), 2)
    moved = c["FX-SNOW-019"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    card = plain(case(drawing="card"))
    qc = ops(c["FX-SNOW-020"]["0"], card)
    shown = [i for i in range(W * H) if card[i][3] > 0]
    assert all(qc[i] is None or abs(qc[i] - q2[i]) < 1e-9 for i in shown)
    assert snowy([qc[i] for i in shown])
    print("checked")


if __name__ == "__main__":
    main()
