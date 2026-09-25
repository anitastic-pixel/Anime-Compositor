"""Bloom, worked a second way.

D-96 adds `core.bloom`. It makes the brightest parts of a drawing flood with light, the way
strong light blooms on film: a soft halo, tight and bright near the source and wide and faint
further out, and, if asked, streaks of light crossing it like a star filter on a lens. It is
this program's own method, built from D-89's glow test, document 21's Gaussian blur and D-92's
line samples; nothing is ported. Document 21 is the rule in words; this file is the reference
for the numbers document 25 pins against it.

The rule. (1) The light: each pixel that shows, whose brightest channel in 8-bit steps is at
least the threshold's share of 255, exactly as D-89's glow tests it; every other pixel gives
nothing. (2) The halo H: the plain average of four of document 21's Gaussian blurs of the
light, at sigma radius / 3 times 1, 1/2, 1/4 and 1/8. (3) The streaks T, when `streaks` is
`cross` or `star`: two lines, at `angle` and `angle` + 90 degrees, or four, at 45 degree steps,
each direction measured clockwise from up as D-92's is. Along one line, with m = ceil(length),
the light is sampled (document 21's bilinear sample, nothing outside the drawing) at
t_k = (k - m) * length / m for k = 0 to 2m and weighted m - |k - m|, a tent, the weights
summing to m * m; with m = 0 it is the light itself. T is the plain average of the lines.
(4) The output is O + intensity * (H + T), T being nothing with streaks `none`; the covering
stops at full and the colour is not cut off at white. The layer grows by ceil(radius), or by
ceil(length) with streaks on if that is more.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, and sums each blur in two dimensions at once, where the build
works in single precision on its buffers and blurs across and then down.

Every case is a composition 16 pixels by 10 holding glow's drawing of three patches the same
size, unmoved unless the case says. The projects go into `Fixtures/bloom`, the drawings into
`Fixtures/bloom/media`, and the expected frames into `Fixtures/bloom/expected_bloom.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bloom_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import glow_reference as G  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bloom"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_THRESHOLD, MAX_RADIUS, MAX_INTENSITY, MAX_LENGTH, MAX_ANGLE = 100, 500, 10, 500, 3600
SCALES = (1, 0.5, 0.25, 0.125)
LINES = {"none": 0, "cross": 2, "star": 4}


# --- the rule -------------------------------------------------------------------------------

def bilinear(layer, x, y):
    """Document 21's sample from pixel centres, transparent outside the drawing."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx, sy = x0 + dx, y0 + dy
            if 0 <= sx < W and 0 <= sy < H and wx * wy:
                p = layer[sy * W + sx]
                for i in range(4):
                    out[i] += p[i] * wx * wy
    return out


# At a whole quarter turn the step is exact, as D-92's is.
QUARTERS = {0: (0.0, -1.0), 90: (1.0, 0.0), 180: (0.0, 1.0), 270: (-1.0, 0.0)}


def along(direction):
    u = QUARTERS.get(direction % 360)
    if u is None:
        a = math.radians(direction)
        u = (math.sin(a), -math.cos(a))
    return u


def gaussian(sigma):
    """Document 21's one-dimensional weights, normalised after cutting at three sigma."""
    reach = math.ceil(3 * sigma) if sigma > 0 else 0
    if not reach:
        return 0, [1.0]
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-reach, reach + 1)]
    total = sum(one)
    return reach, [v / total for v in one]


def bloom(pixels, c, frame_no, shift):
    """The bloomed drawing, as the composition's W by H frame. The composition's column x shows
    the drawing's column x - shift; outside the drawing is the space the bloom grows into."""
    threshold = min(MAX_THRESHOLD, max(0.0, value_at(c["threshold"], frame_no)))
    radius = min(MAX_RADIUS, max(0.0, value_at(c["radius"], frame_no)))
    intensity = min(MAX_INTENSITY, max(0.0, value_at(c["intensity"], frame_no)))
    length = min(MAX_LENGTH, max(0.0, value_at(c["length"], frame_no)))
    angle = min(MAX_ANGLE, max(-MAX_ANGLE, value_at(c["angle"], frame_no)))
    lines = LINES[c["streaks"]]

    # 1. The light: glow's bright test, the pixel as it is.
    bright = {"based_on": "bright"}
    light = [G.working(p) if G.glows(p, bright, threshold, 0) else [0.0] * 4 for p in pixels]
    lit = [(i % W, i // W) for i in range(W * H) if light[i][3] > 0]
    blurs = [gaussian(radius / 3 * s) for s in SCALES]

    m = math.ceil(length)
    ts = [(k - m) * length / m for k in range(2 * m + 1)] if m else [0.0]
    weights = [m - abs(k - m) for k in range(2 * m + 1)] if m else [1]
    total = sum(weights)
    dirs = [along(angle + j * 180 / lines) for j in range(lines)]

    out = []
    for y in range(H):
        for x in range(W):
            dx = x - shift
            # 2. The halo: the average of the four blurs, summed in two dimensions at once.
            halo = [0.0] * 4
            for reach, one in blurs:
                for lx, ly in lit:
                    i, j = lx - dx, ly - y
                    if abs(i) <= reach and abs(j) <= reach:
                        k = one[i + reach] * one[j + reach] / len(SCALES)
                        for ch in range(4):
                            halo[ch] += light[ly * W + lx][ch] * k
            # 3. The streaks: tent-weighted samples along each line, the lines averaged.
            streak = [0.0] * 4
            for u in dirs:
                line = [0.0] * 4
                for t, wt in zip(ts, weights):
                    s = bilinear(light, dx + 0.5 + t * u[0], y + 0.5 + t * u[1])
                    for ch in range(4):
                        line[ch] += s[ch] * wt
                for ch in range(4):
                    streak[ch] += line[ch] / total / lines
            # 4. Added on top of the drawing; the covering stops at full.
            g = [intensity * (halo[ch] + streak[ch]) for ch in range(4)]
            o = G.working(pixels[y * W + dx]) if 0 <= dx < W else [0.0] * 4
            out.append([o[ch] + g[ch] for ch in range(3)] + [min(1.0, o[3] + g[3])])
    return out


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = {"patches": G.DRAWINGS["patches"], "faint": G.DRAWINGS["faint"]}


def case(name="patches", threshold=80, radius=4, intensity=1, streaks="none", length=6,
         angle=0, shift=0):
    return {"drawing": name, "threshold": threshold, "radius": radius, "intensity": intensity,
            "streaks": streaks, "length": length, "angle": angle, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return bloom(pixels, c, frame_no, c["shift"])


def plain(c):
    """The case's drawing, moved as the case moves it, with no bloom."""
    return render(case(c["drawing"], intensity=0, shift=c["shift"]), 0)


CASES = {
    "FX-BLOOM-001": ("Threshold 80, radius 4, no streaks: only the yellow patch, whose brightest "
                     "channel is 98 %, blooms; the brown at 60 % and the purple do not, though "
                     "the halo reaches the brown. The halo is brightest close to the yellow and "
                     "fades into the empty space round it.",
                     case(), [0]),
    "FX-BLOOM-002": ("Intensity 0: the drawing, untouched.",
                     case(intensity=0), [0]),
    "FX-BLOOM-003": ("Radius 0: nothing spreads, and each yellow pixel is added onto itself, "
                     "twice as bright.",
                     case(radius=0), [0]),
    "FX-BLOOM-004": ("Threshold 100: nothing is that bright, so the drawing is untouched.",
                     case(threshold=100), [0]),
    "FX-BLOOM-005": ("Threshold 60: the brown, at exactly 60 %, blooms too.",
                     case(threshold=60), [0]),
    "FX-BLOOM-006": ("Radius 0 with a cross of streaks, length 6, angle 0: the yellow's light "
                     "runs straight up and down and straight left and right, and nowhere else: "
                     "above the patch it is lit, off its corners it is not.",
                     case(radius=0, streaks="cross"), [0]),
    "FX-BLOOM-007": ("The same with a star: four lines, so the diagonals off the patch's corners "
                     "are lit too, and the straight lines are half as strong, as the light is "
                     "shared among four lines, not two.",
                     case(radius=0, streaks="star"), [0]),
    "FX-BLOOM-008": ("A cross at angle 45: the streaks run along the diagonals only.",
                     case(radius=0, streaks="cross", angle=45), [0]),
    "FX-BLOOM-009": ("A cross at angle 90: the same two lines as angle 0, each walked the "
                     "other way, so this is FX-BLOOM-006 to rounding.",
                     case(radius=0, streaks="cross", angle=90), [0]),
    "FX-BLOOM-010": ("The defaults, threshold 80, radius 20, intensity 1, no streaks: a halo "
                     "wider than the whole drawing.",
                     case(radius=20, length=60), [0]),
    "FX-BLOOM-011": ("The defaults with a star of streaks, length 60.",
                     case(radius=20, length=60, streaks="star"), [0]),
    "FX-BLOOM-012": ("Radius 4, a cross of length 6, intensity 2.5: two and a half times as "
                     "strong, and where it adds past white it is not cut off.",
                     case(streaks="cross", intensity=2.5), [0]),
    "FX-BLOOM-013": ("The patches half covering: the yellow blooms as in FX-BLOOM-001, at half "
                     "the strength.",
                     case("faint"), [0]),
    "FX-BLOOM-014": ("Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is "
                     "FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8.",
                     case(radius=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-BLOOM-015": ("Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame "
                     "0 is each yellow pixel added onto itself twice over, frame 2 is "
                     "FX-BLOOM-006.",
                     case(radius=0, streaks="cross", length=keyed((0, 0), (4, 12))), [0, 2, 4]),
    "FX-BLOOM-016": ("Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame "
                     "0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 "
                     "again.",
                     case(radius=0, streaks="cross", angle=keyed((0, 0), (4, 90))), [0, 2, 4]),
    "FX-BLOOM-017": ("Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is "
                     "the drawing, frame 4 has the purple blooming too.",
                     case(threshold=keyed((0, 100), (4, 20))), [0, 2, 4]),
    "FX-BLOOM-018": ("Radius 4 and a cross of length 6, moved three pixels right: the bloom is "
                     "done on the drawing before it is moved, and the streak that ran past the "
                     "drawing's left edge now shows in columns 0 to 2.",
                     case(streaks="cross", shift=3), [0, 3]),
    "FX-BLOOM-019": ("Radius 2.5 and length 2.5: neither is rounded.",
                     case(radius=2.5, length=2.5, streaks="cross"), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BLOOM-020": ("Threshold 101, above 100.", case(threshold=101)),
    "FX-BLOOM-021": ("Radius 501, above 500.", case(radius=501)),
    "FX-BLOOM-022": ("Radius -1, below 0.", case(radius=-1)),
    "FX-BLOOM-023": ("Intensity 11, above 10.", case(intensity=11)),
    "FX-BLOOM-024": ("Length 501, above 500.", case(length=501)),
    "FX-BLOOM-025": ("Angle 3601, above 3600.", case(angle=3601)),
    "FX-BLOOM-026": ("Radius keyed to 600 at frame 4.", case(radius=keyed((0, 0), (4, 600)))),
    "FX-BLOOM-027": ("Streaks \"rays\", which is not none, cross or star.", case(streaks="rays")),
    "FX-BLOOM-028": ("Streaks \"Cross\", with a capital: words are matched exactly.",
                     case(streaks="Cross")),
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
        "instance_id": "fx-0-0", "type_id": "core.bloom", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in (
            "threshold", "radius", "intensity", "streaks", "length", "angle")}}]
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
        plain = render(case(c["drawing"], intensity=0, shift=c["shift"]), 0)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        plain = render(case(c["drawing"], intensity=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_bloom.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    plain = {name: [G.working(p) for row in DRAWINGS[name] for p in row] for name in DRAWINGS}
    drawn = plain["patches"]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p[ch] - q[ch]) < 1e-12  # noqa: E731
                            for p, q in zip(a, b) for ch in range(4))
    one = c["FX-BLOOM-001"]["0"]

    # 001: the yellow and the space round it brighten, more close by than further out; the brown
    # is reached by the halo, the purple, 7 columns off, is not; the far corner stays empty.
    assert all(one[at(3, 4)][ch] > drawn[at(3, 4)][ch] for ch in range(3))
    assert one[at(5, 4)][3] > one[at(6, 4)][3] > 0
    assert one[at(7, 4)] != drawn[at(7, 4)] and one[at(11, 4)] == drawn[at(11, 4)]
    assert one[at(15, 0)] == [0.0] * 4
    # The halo is tighter than glow's at the same radius: more of the light stays on the yellow
    # itself, and less reaches the space round it.
    glow = G.render(G.case(threshold=80, radius=4), 0)
    assert one[at(3, 4)][0] > glow[at(3, 4)][0]
    assert all(one[at(x, y)][3] < glow[at(x, y)][3] for x, y in ((5, 4), (0, 0), (3, 1)))

    assert c["FX-BLOOM-002"]["0"] == drawn
    three = c["FX-BLOOM-003"]["0"]
    for i in range(W * H):
        g = drawn[i] if drawn[i] == G.working(G.BRIGHT) else [0.0] * 4
        assert three[i] == [drawn[i][ch] + g[ch] for ch in range(3)] + [drawn[i][3]]
    assert c["FX-BLOOM-004"]["0"] == drawn
    assert c["FX-BLOOM-005"]["0"][at(9, 4)] != one[at(9, 4)]  # the brown's own halo

    six, seven, eight = (c[f"FX-BLOOM-00{n}"]["0"] for n in (6, 7, 8))
    assert six[at(3, 0)][3] > 0 and six[at(0, 4)][3] > 0  # above and beside the yellow
    assert six[at(0, 0)] == [0.0] * 4 and six[at(6, 1)] == [0.0] * 4  # off its corners
    assert seven[at(0, 0)][3] > 0 and seven[at(6, 1)][3] > 0  # the star's diagonals
    assert eight[at(3, 0)] == [0.0] * 4 and eight[at(6, 1)][3] > 0
    # The same two lines, each walked the other way, so equal to rounding.
    assert near(c["FX-BLOOM-009"]["0"], six)
    # A star is the average of the cross at 0 and the cross at 45.
    assert all(abs(seven[i][ch] - (six[i][ch] + eight[i][ch]) / 2) < 1e-12
               for i in range(W * H) for ch in range(3))

    assert all(p[3] > 0 for p in c["FX-BLOOM-010"]["0"])  # radius 20 floods the drawing
    assert c["FX-BLOOM-011"]["0"] != c["FX-BLOOM-010"]["0"]
    assert any(v > 1 for p in c["FX-BLOOM-012"]["0"] for v in p[:3])  # past white
    thirteen = c["FX-BLOOM-013"]["0"]
    assert thirteen[at(3, 4)][3] > plain["faint"][at(3, 4)][3]
    # In the empty space the halo is the full drawing's, scaled by the covering, 128 / 255.
    assert near([thirteen[at(5, 4)]], [[v * 128 / 255 for v in one[at(5, 4)]]])
    assert c["FX-BLOOM-014"]["0"] == three and c["FX-BLOOM-014"]["2"] == one
    assert c["FX-BLOOM-014"]["4"] == render(case(radius=8), 0)
    assert c["FX-BLOOM-015"]["2"] == six
    zero = c["FX-BLOOM-015"]["0"]
    for i in range(W * H):
        g = drawn[i] if drawn[i] == G.working(G.BRIGHT) else [0.0] * 4
        assert zero[i] == [drawn[i][ch] + 2 * g[ch] for ch in range(3)] + [drawn[i][3]]
    assert c["FX-BLOOM-016"]["0"] == six and near(c["FX-BLOOM-016"]["4"], six)
    assert c["FX-BLOOM-016"]["2"] == eight
    assert c["FX-BLOOM-017"]["0"] == drawn
    assert c["FX-BLOOM-017"]["4"][at(14, 4)] != drawn[at(14, 4)]
    moved = c["FX-BLOOM-018"]["0"]
    assert moved == c["FX-BLOOM-018"]["3"]
    unmoved = render(case(streaks="cross"), 0)
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == unmoved[at(0, y):at(W - 3, y)]
    assert all(moved[at(x, 4)][3] > 0 for x in (0, 1, 2))  # grown past the drawing's edge
    nineteen = c["FX-BLOOM-019"]["0"]
    assert nineteen not in (render(case(radius=2, length=2, streaks="cross"), 0),
                            render(case(radius=3, length=3, streaks="cross"), 0))

    # Everywhere: covering inside 0 to 1, and no colour below nothing.
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
