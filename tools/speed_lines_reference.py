"""Speed lines, worked a second way.

D-160 adds `core.speed_lines`, a generator: manga focus lines, thin wedges rushing in from the
edges toward a point, drawn inside a layer's covering, on a solid or on any drawing. `count`
lines are spread evenly round `center` (per cent of the drawing's own width and height, as Radial
Blur's centre is), each nudged off its even place by up to half a spacing by `angle_jitter`;
each is a wedge `thickness` degrees wide at its widest, one line thinner and the next wider by up
to half; and each starts `inner` pixels from the centre, nearer or further by up to
`inner_jitter` per cent. Inside the inner edge nothing is drawn, so the middle stays clear. The
wedges' edges are softened by half a pixel across, and a pixel takes the strongest line over it.
The lines are laid on in `color` at `opacity` per cent by Gradient's blend mix, normal (D-114).
They are fixed by `seed` and by the frame, and change every `hold` frames, as hand-drawn focus
lines are redrawn on twos. Every pixel keeps its own covering, and a pixel that does not show
stays as it is. It is this program's own method, modelled on the focus lines (shuchusen) of manga
and anime; nothing is ported. Document 21 is the rule in words; this file is the reference for
the numbers document 25 pins against it.

The rule. m = floor(frame / hold), frame the composition frame, a hidden value. For each line
k = 0..count-1, with U Noise's hash (D-119, `tools/noise_reference.py`):
theta_k = 360 / count (k + 0.5 angle_jitter / 100 U(seed, k, 0, m, 0)) degrees,
h_k = thickness / 2 (1 + 0.5 U(seed, k, 0, m, 1)) degrees and
r_k = inner (1 + inner_jitter / 100 U(seed, k, 0, m, 2)). At a pixel with a > 0, P its centre, c
the centre point, d = |P - c| and alpha the screen angle of P - c (0 straight up, 90 to the
right, atan2(x, -y) in degrees taken into [0, 360)); delta_k = min(D, 360 - D) with
D = (alpha - theta_k) taken into [0, 360);
q_k = clamp((h_k - delta_k) pi / 180 d + 0.5, 0, 1) clamp(d - r_k + 0.5, 0, 1), and q is the
largest q_k. With b the straight linear colour, G the colour's linear value and
op = q opacity / 100, the output is ((b + op (G - b)) a, a). No growth; a draft scales `inner`.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a white solid covering the frame, and for one case Noise's card
(`tools/noise_reference.py`). The drawings go into `Fixtures/speed_lines/media`, the projects
into `Fixtures/speed_lines`, and the expected frames into
`Fixtures/speed_lines/expected_speed_lines.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/speed_lines_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "speed_lines"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "count": (4, 1000), "thickness": (0, 30),
          "inner": (0, 100000), "inner_jitter": (0, 100), "angle_jitter": (0, 100),
          "seed": (0, 100000), "hold": (1, 100), "opacity": (0, 100)}
FLOORED = ("count", "seed", "hold")
NAMES = ("center", "color", "count", "thickness", "inner", "inner_jitter", "angle_jitter",
         "seed", "hold", "opacity")


# --- the rule -------------------------------------------------------------------------------

def screen_angle(vx, vy):
    """alpha(v): 0 straight up, 90 to the right, clockwise on the screen, in [0, 360)."""
    return math.degrees(math.atan2(vx, -vy)) % 360.0


def lines(count, thickness, inner, inner_jitter, angle_jitter, seed, hold, frame_no):
    """Each line's (theta_k, h_k, r_k) at a composition frame; the numbers already held and
    floored."""
    m = frame_no // hold
    return [(360 / count * (k + 0.5 * angle_jitter / 100 * U(seed, k, 0, m, 0)),
             thickness / 2 * (1 + 0.5 * U(seed, k, 0, m, 1)),
             inner * (1 + inner_jitter / 100 * U(seed, k, 0, m, 2))) for k in range(count)]


def strength(px, py, cx, cy, drawn):
    """q at the point (px, py): the strongest line over it."""
    vx, vy = px - cx, py - cy
    d = math.hypot(vx, vy)
    alpha = screen_angle(vx, vy)
    q = 0.0
    for theta, h, r in drawn:
        D = (alpha - theta) % 360.0
        delta = min(D, 360.0 - D)
        q = max(q, min(1.0, max(0.0, (h - delta) * math.pi / 180 * d + 0.5))
                * min(1.0, max(0.0, d - r + 0.5)))
    return q


def speed_lines(pixels, center, color, count, thickness, inner, inner_jitter, angle_jitter, seed,
                hold, opacity, frame_no, width=W, height=H):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, `width` wide, the drawing's
    top-left pixel at (0, 0). Numbers are already held; count, seed and hold already floored."""
    G = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    cx, cy = center[0] / 100 * width, center[1] / 100 * height
    drawn = lines(count, thickness, inner, inner_jitter, angle_jitter, seed, hold, frame_no)
    out = []
    for i, p in enumerate(pixels):
        w = R.working(p)
        a = w[3]
        if a == 0:
            out.append(w)
            continue
        op = strength(i % width + 0.5, i // width + 0.5, cx, cy, drawn) * opacity / 100
        b = [v / a for v in w[:3]]
        out.append([(b[c] + op * (G[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawings ---------------------------------------------------------------------------

WHITE = (255, 255, 255, 255)
DRAWINGS = {"solid": [[WHITE] * W for _ in range(H)],   # a white solid covering the frame
            "card": N.DRAWINGS["card"]}                 # grey, skin, white, black, a soft edge
BLUE = "#3a6fd8"


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), color="#000000", count=120, thickness=1.5, inner=150, inner_jitter=40,
         angle_jitter=50, seed=0, hold=2, opacity=100, shift=0, drawing="solid"):
    return {"drawing": drawing, "center": center, "color": color, "count": count,
            "thickness": thickness, "inner": inner, "inner_jitter": inner_jitter,
            "angle_jitter": angle_jitter, "seed": seed, "hold": hold, "opacity": opacity,
            "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, u)) for u in v]
    v = min(hi, max(lo, v))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    n = {k: held(c, k, frame_no) for k in RANGES}
    return R.frame(speed_lines(pixels, n["center"], c["color"], n["count"], n["thickness"],
                               n["inner"], n["inner_jitter"], n["angle_jitter"], n["seed"],
                               n["hold"], n["opacity"], frame_no), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(opacity=0, shift=c["shift"], drawing=c["drawing"]), 0)


BASE = {"count": 12, "thickness": 10, "inner": 2}   # twelve wide lines, the middle kept clear
EVEN = dict(BASE, inner_jitter=0, angle_jitter=0)   # every 30 degrees, straight up first

CASES = {
    "FX-SPEED-001": ("The settings as they start: about the middle, #000000, count 120, "
                     "thickness 1.5, inner 150, inner jitter 40, angle jitter 50, seed 0, hold "
                     "2, opacity 100. Every line starts at least 90 pixels from the centre, "
                     "further than any pixel of the small drawing: the solid, untouched, at "
                     "frame 0 and frame 4.",
                     case(), [0, 4]),
    "FX-SPEED-002": ("Inner 0, the rest as they start: 120 thin lines reach the centre, and "
                     "every pixel of the solid is greyed, none fully black. With hold 2, frame 1 "
                     "is frame 0; frame 2 draws new lines, and frame 4 new again.",
                     case(inner=0), [0, 1, 2, 4]),
    "FX-SPEED-003": ("Count 12, thickness 10, inner 2: twelve wide lines, jittered, rushing in "
                     "to 2 pixels or so from the centre; frame 1 is frame 0, frames 2 and 4 are "
                     "new lines.",
                     case(**BASE), [0, 1, 2, 4]),
    "FX-SPEED-004": ("FX-SPEED-003 with inner jitter 0 and angle jitter 0: the lines exactly "
                     "every 30 degrees, the first straight up, so columns 7 and 8, either side "
                     "of the lines straight up and straight down, are darkened alike in rows 0 "
                     "to 2 and 7 to 9, and the pixels nearest the centre, under 1.5 pixels "
                     "from it, untouched.",
                     case(**EVEN), [0]),
    "FX-SPEED-005": ("FX-SPEED-004 with inner 5 and inner jitter 100: each line starts "
                     "anywhere from the centre to 10 pixels out, so some pixels under 4.5 "
                     "pixels from the centre are reached, which inner jitter 0 leaves "
                     "untouched, and some further out are not.",
                     case(**dict(EVEN, inner=5, inner_jitter=100)), [0]),
    "FX-SPEED-006": ("FX-SPEED-004 with angle jitter 100: each line moved off its even place "
                     "by up to half a spacing, 15 degrees either way.",
                     case(**dict(EVEN, angle_jitter=100)), [0]),
    "FX-SPEED-007": ("FX-SPEED-003 with thickness 30: wedges three times as wide, darkening "
                     "more of the solid, none less.",
                     case(**dict(BASE, thickness=30)), [0]),
    "FX-SPEED-008": ("FX-SPEED-003 with thickness 0: each line a hairline, its softened edge "
                     "alone showing, so no pixel is more than half way to black.",
                     case(**dict(BASE, thickness=0)), [0]),
    "FX-SPEED-009": ("Count 4, thickness 30, inner 0, both jitters 0: four wedges, up, right, "
                     "down and left, a cross, black at its arms' ends; the diagonal pixels "
                     "(9, 6) to (11, 8) and (6, 3) to (4, 1), between the arms, untouched, and "
                     "the four pixels about the centre greyed, where the arms' softened edges "
                     "meet.",
                     case(count=4, thickness=30, inner=0, inner_jitter=0, angle_jitter=0), [0]),
    "FX-SPEED-010": ("Count 1000, thickness 30, inner 0: the lines overlap everywhere, so every "
                     "pixel 4 or more pixels from the centre is black.",
                     case(count=1000, thickness=30, inner=0), [0]),
    "FX-SPEED-011": ("FX-SPEED-003 with seed 7.9, which counts as 7: lines of their own, not "
                     "FX-SPEED-003's.",
                     case(**dict(BASE, seed=7.9)), [0]),
    "FX-SPEED-012": ("FX-SPEED-003 with hold 1: new lines every frame, so frame 1 is "
                     "FX-SPEED-003's frame 2 and frame 2 is its frame 4.",
                     case(**dict(BASE, hold=1)), [0, 1, 2]),
    "FX-SPEED-013": ("FX-SPEED-003 with hold 100: the same lines on every frame, frame 4 is "
                     "frame 0.",
                     case(**dict(BASE, hold=100)), [0, 4]),
    "FX-SPEED-014": ("FX-SPEED-003 with opacity 50: each pixel half as far toward black.",
                     case(**dict(BASE, opacity=50)), [0]),
    "FX-SPEED-015": (f"FX-SPEED-003 in {BLUE}, a blue: the same lines, blue.",
                     case(**dict(BASE, color=BLUE)), [0]),
    "FX-SPEED-016": ("FX-SPEED-015 with the colour written in capitals, #3A6FD8: the same.",
                     case(**dict(BASE, color=BLUE.upper())), [0]),
    "FX-SPEED-017": ("FX-SPEED-003 about the middle of the left edge, centre 0, 50: the lines "
                     "rush in toward the left edge, and the two pixels at its middle, (0, 4) and "
                     "(0, 5), inside every line's start, are untouched.",
                     case(**dict(BASE, center=(0, 50))), [0]),
    "FX-SPEED-018": ("FX-SPEED-003 with count keyed from 4 at frame 0 to 20 at frame 4, linear: "
                     "frame 0 has four lines, frame 1 eight, frame 2, at 12, is FX-SPEED-003's "
                     "frame 2, and frame 4 has twenty.",
                     case(**dict(BASE, count=keyed((0, 4), (4, 20)))), [0, 1, 2, 4]),
    "FX-SPEED-019": ("FX-SPEED-003 with opacity keyed from 0 at frame 0 to 100 at frame 4, "
                     "eased past its end: frame 0 is the solid; frame 2 is held at 100 and is "
                     "FX-SPEED-003's frame 2.",
                     case(**dict(BASE, opacity=keyed((0, 0, OVERSHOOT), (4, 100)))), [0, 2]),
    "FX-SPEED-020": ("FX-SPEED-003 moved three pixels right: the lines are worked in the "
                     "drawing's own space, so they move with it; the three columns left of it "
                     "are empty.",
                     case(**dict(BASE, shift=3)), [0]),
    "FX-SPEED-021": ("FX-SPEED-003 on Noise's card rather than the solid: the same lines, drawn "
                     "only inside the covering; the empty pixels stay empty, the soft right "
                     "edge keeps its half covering, and the black patch stays black.",
                     case(**dict(BASE, drawing="card")), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SPEED-022": ("Count 3, below 4.", case(count=3)),
    "FX-SPEED-023": ("Thickness 31, above 30.", case(thickness=31)),
    "FX-SPEED-024": ("Inner -1, below 0.", case(inner=-1)),
    "FX-SPEED-025": ("Inner jitter 101, above 100.", case(inner_jitter=101)),
    "FX-SPEED-026": ("Hold 0, below 1.", case(hold=0)),
    "FX-SPEED-027": ("Centre 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-SPEED-028": ("Opacity keyed to 150 at frame 4, above 100.",
                     case(opacity=keyed((0, 100), (4, 150)))),
    "FX-SPEED-029": ("Colour \"#12345\", one digit short.", case(color="#12345")),
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
        "instance_id": "fx-0-0", "type_id": "core.speed_lines", "enabled": True,
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

    (OUT / "expected_speed_lines.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    solid = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    dist = lambda i, cx=8, cy=5: math.hypot(i % W + 0.5 - cx, i // W + 0.5 - cy)  # noqa: E731
    q = lambda f, i: 1 - f[i][0]  # black on white: the red channel is 1 - q  # noqa: E731
    every = range(W * H)

    # The rule's own pieces: the screen angle, the wrap round 360, the jitters' reach.
    assert screen_angle(0, -1) == 0 and screen_angle(1, 0) == 90
    assert screen_angle(0, 1) == 180 and screen_angle(-1, 0) == 270 and screen_angle(0, 0) == 0
    assert abs(strength(8.5, 0.5, 8, 5, [(-0.5, 1, 0)])
               - strength(8.5, 0.5, 8, 5, [(359.5, 1, 0)])) < 1e-9
    assert strength(8, 0, 8, 5, [(0, 0, 0)]) == 0.5  # a hairline on its own axis: half
    assert strength(8, 0, 8, 5, [(0, 5, 5.5)]) == 0 and strength(8, 0, 8, 5, [(0, 10, 4.5)]) == 1
    for theta, h, r in lines(12, 10, 2, 40, 50, 0, 2, 0):
        assert 2.5 <= h <= 7.5 and 1.2 <= r <= 2.8
    assert all(abs(t - 30 * k) <= 7.5 for k, (t, _, _) in
               enumerate(lines(12, 10, 2, 40, 50, 0, 2, 0)))

    # Every case keeps every covering, leaves empty pixels empty and stays premultiplied.
    for fx, frames in c.items():
        s = CASES.get(fx) or INVALID[fx]
        base = plain(s[1])
        for px in frames.values():
            for i in every:
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
        if "warning" in expected["cases"][fx]:
            assert all(px == base for px in frames.values())

    one = c["FX-SPEED-001"]
    assert one["0"] == one["4"] == solid
    assert min(r for _, _, r in lines(120, 1.5, 150, 40, 50, 0, 2, 0)) > 90 > max(map(dist, every))
    two = c["FX-SPEED-002"]
    assert two["0"] == two["1"] and two["2"] != two["0"] and two["4"] not in (two["0"], two["2"])
    assert all(0 < q(two["0"], i) < 1 for i in every)
    three = c["FX-SPEED-003"]
    base = three["0"]
    assert three["0"] == three["1"] and three["2"] != three["0"] and three["4"] != three["2"]
    assert 0 < sum(q(base, i) == 1 for i in every) and 0 < sum(q(base, i) == 0 for i in every)
    four = c["FX-SPEED-004"]["0"]
    for i in every:
        x, y = i % W, i // W
        if x == 7 and not 3 <= y <= 6:  # the upright lines, straight up and straight down
            assert close(four[i], four[at(8, y)]) and four[i] != solid[i], y
        if dist(i) < 1.5:
            assert four[i] == solid[i]
    five = c["FX-SPEED-005"]["0"]
    fixed = render(case(**dict(EVEN, inner=5)), 0)
    assert all(fixed[i] == solid[i] for i in every if dist(i) < 4.5)
    assert any(five[i] != solid[i] for i in every if dist(i) < 4.5)
    assert any(five[i] == solid[i] != fixed[i] for i in every)
    six = c["FX-SPEED-006"]["0"]
    assert six != four
    assert all(abs(t - 30 * k) <= 15 for k, (t, _, _) in
               enumerate(lines(12, 10, 2, 0, 100, 0, 2, 0)))
    seven = c["FX-SPEED-007"]["0"]
    assert all(q(seven, i) >= q(base, i) for i in every)
    assert sum(q(seven, i) for i in every) > 1.5 * sum(q(base, i) for i in every)
    eight = c["FX-SPEED-008"]["0"]
    assert all(q(eight, i) <= 0.5 for i in every) and any(q(eight, i) > 0 for i in every)
    nine = c["FX-SPEED-009"]["0"]
    diagonal = [at(8 + k, 5 + k) for k in range(1, 4)] + [at(7 - k, 4 - k) for k in range(1, 4)]
    assert all(nine[i] == solid[i] for i in diagonal)
    assert all(0 < q(nine, at(x, y)) < 1 for x in (7, 8) for y in (4, 5))
    assert all(q(nine, at(x, 0)) == 1 for x in (7, 8)) and q(nine, at(0, 4)) == 1
    ten = c["FX-SPEED-010"]["0"]
    assert all(q(ten, i) == 1 for i in every if dist(i) >= 4)
    eleven = c["FX-SPEED-011"]["0"]
    assert eleven != base and eleven == render(case(**dict(BASE, seed=7)), 0)
    twelve = c["FX-SPEED-012"]
    assert twelve["1"] == three["2"] and twelve["2"] == three["4"] and twelve["0"] == base
    thirteen = c["FX-SPEED-013"]
    assert thirteen["0"] == thirteen["4"] == base
    fourteen = c["FX-SPEED-014"]["0"]
    assert all(abs(q(fourteen, i) - q(base, i) / 2) < 1e-12 for i in every)
    fifteen = c["FX-SPEED-015"]["0"]
    G = [srgb_to_linear(v / 255) for v in R.hex_color(BLUE)]
    assert all(close(fifteen[i], [1 + q(base, i) * (g - 1) for g in G] + [1.0]) for i in every)
    assert c["FX-SPEED-016"]["0"] == fifteen
    seventeen = c["FX-SPEED-017"]["0"]
    assert seventeen != base and all(seventeen[at(0, y)] == solid[at(0, y)] for y in (4, 5))
    eighteen = c["FX-SPEED-018"]
    assert eighteen["2"] == three["2"] and eighteen["0"] == render(case(**dict(BASE, count=4)), 0)
    assert eighteen["1"] == render(case(**dict(BASE, count=8)), 1)
    assert eighteen["4"] == render(case(**dict(BASE, count=20)), 4)
    nineteen = c["FX-SPEED-019"]
    assert ease(OVERSHOOT, 0.5) > 1 and nineteen["0"] == solid and nineteen["2"] == three["2"]
    moved = c["FX-SPEED-020"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == base[at(0, y):at(W - 3, y)]
        assert moved[at(0, y):at(3, y)] == [[0.0] * 4] * 3
    card, drawn = c["FX-SPEED-021"]["0"], plain(case(drawing="card"))
    for i in every:
        a = drawn[i][3]
        if a == 0:
            continue
        want = [(v / a + q(base, i) * (0 - v / a)) * a for v in drawn[i][:3]] + [a]
        assert close(card[i], want), i
    assert card[at(15, 4)][3] == 128 / 255 and card[at(15, 4)] != drawn[at(15, 4)]
    assert all(card[at(x, y)][:3] == [0.0] * 3 for x in (9, 10) for y in (3, 4))
    print("checked")


if __name__ == "__main__":
    main()
