"""Glint Streaks, worked a second way.

D-546 is pick #15 in `docs/effects/EFFECTS.md` (row 262), "Glint Streaks": a colour per ray and
shimmer for Cross Glare. PLUGINS.md settles it as a merge into `core.cross_glare` (D-161), after
Glimmer's animated multi-ray glints ("up to six rays with their own colours, shimmer and
looping"). The plugin publishes no formula; the rule below is our own, built on Cross Glare's
(tools/cross_glare_reference.py) and Kira-kira's twinkle (D-186). No plugin code is used.

Four new settings on Cross Glare:

* `ray_colors`, "one" (as added), "alternate" or "gradient": "one" tints every ray with
  `color`, as before; "alternate" tints the even rays (0, 2, ...) with `color` and the odd ones
  with `color_2`; "gradient" runs ray j from `color` to `color_2`, f_j = j / (n - 1) (ray 0 is
  `color`, the last ray `color_2`; one ray is `color`).
* `color_2`, `#rrggbb`, default #80c0ff: the second colour; checked even when unused, as
  every colour is.
* `shimmer`, 0 to 100, default 0: how far each ray dims as it shimmers.
* `shimmer_period`, 1 to 1000 frames, default 24: how long a ray takes to dim and come back;
  the shimmer loops every period, exactly.

The two numbers are keyable, held inside their ranges at each frame. A file without the four
reads "one", #80c0ff, 0, 24, and they are written only when changed or in the file already, so
every earlier file draws the same numbers.

The rule. Cross Glare's light Lt, arms v_j, j = 0 to n - 1, and weights w_t / W, unchanged.
Ray j's colour is C_j = (1 - f_j) C + f_j C2 in linear light, f_j 0 for "one", j mod 2 for
"alternate", j / (n - 1) for "gradient" (0 when n is 1); C and C2 are `color` and `color_2`.
Ray j's strength at composition frame f is m_j = 1 - s (1 - g_j), s = shimmer / 100 and
g_j = 0.5 + 0.5 cos(2 pi (f / P + phi_j)), P = `shimmer_period` and phi_j = frac(j phi),
phi = (sqrt(5) - 1) / 2, the golden ratio's fraction, so neighbouring rays are out of step
(Kira-kira's twinkle, with a fixed phase per ray in place of a hashed one per star). Ray 0 is at
full strength on frame 0. The glare of ray j is G_j(P) = sum over t of (w_t / W)
bilinear(Lt, P - t v_j). The output is O + intensity sum_j m_j C_j G_j in red, green and blue,
and min(1, O.a + intensity sum_j m_j G_j.a) in covering. With "one" and shimmer 0 this is Cross
Glare's O + intensity C G exactly.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is Cross Glare's composition, 16 by 10, holding its `sparks` drawing, made nine frames
long (Cross Glare's is five) so the shimmer can be seen to frame 8. The drawing
goes into `Fixtures/glint_streaks/media`, the projects into `Fixtures/glint_streaks`, and the
expected frames into `Fixtures/glint_streaks/expected_glint_streaks.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glint_streaks_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cross_glare_reference as CG  # noqa: E402
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402

W, H = CG.W, CG.H
R, S = CG.R, CG.S
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glint_streaks"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {**CG.RANGES, "shimmer": (0, 100), "shimmer_period": (1, 1000)}
NEW = ("ray_colors", "color_2", "shimmer", "shimmer_period")
PHI = (math.sqrt(5) - 1) / 2
EMPTY = CG.EMPTY


# --- the rule -------------------------------------------------------------------------------

def linear(color):
    return [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]


def ray_colours(n, mode, color, color_2):
    c, c2 = linear(color), linear(color_2)
    def f(j):
        if mode == "alternate":
            return j % 2
        if mode == "gradient" and n > 1:
            return j / (n - 1)
        return 0
    return [[(1 - f(j)) * c[ch] + f(j) * c2[ch] for ch in range(3)] for j in range(n)]


def strengths(n, shimmer, period, frame_no):
    s = shimmer / 100
    g = [0.5 + 0.5 * math.cos(2 * math.pi * (frame_no / period + (j * PHI) % 1)) for j in range(n)]
    return [1 - s * (1 - gj) for gj in g]


def glint(pixels, threshold, length, points, angle, intensity, color, ray_colors, color_2,
          shimmer, period, frame_no, size=(W, H)):
    """Cross Glare's `cross_glare` with a colour and a strength per ray."""
    w, h = size
    working = [R.working(p) for p in pixels]
    inside = lambda x, y: 0 <= x < w and 0 <= y < h  # noqa: E731
    L = math.floor(length)
    if L == 0 or intensity == 0:
        return lambda x, y: working[y * w + x] if inside(x, y) else EMPTY
    light = CG.light_of(pixels, size, threshold)
    dirs, wts = CG.streaks(points, angle), CG.weights(L)
    cols = ray_colours(len(dirs), ray_colors, color, color_2)
    m = strengths(len(dirs), shimmer, period, frame_no)

    def at(x, y):
        if not (-L <= x < w + L and -L <= y < h + L):
            return EMPTY
        o = working[y * w + x] if inside(x, y) else EMPTY
        out = [0.0] * 4
        for j, d in enumerate(dirs):
            g = CG.glare(light, [d], wts, x + 0.5, y + 0.5)
            for ch in range(3):
                out[ch] += m[j] * cols[j][ch] * g[ch]
            out[3] += m[j] * g[3]
        return [o[ch] + intensity * out[ch] for ch in range(3)] + [min(1.0, o[3] + intensity * out[3])]
    return at


# --- the cases ------------------------------------------------------------------------------

def case(threshold=80, length=4, points=4, angle=0, intensity=1, color="#ff8000", shift=0,
         **new):
    """Cross Glare's settings, length 4 and angle 0 (a plus on whole pixels) unless the case
    says; `new` holds the D-546 settings the file writes, the rest absent."""
    return {"drawing": "sparks", "threshold": threshold, "length": length, "points": points,
            "angle": angle, "intensity": intensity, "color": color, "shift": shift, "new": new}


START = {"ray_colors": "one", "color_2": "#80c0ff", "shimmer": 0, "shimmer_period": 24}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = c[k] if k in CG.NUMBERS else {**START, **c["new"]}[k]
    return min(hi, max(lo, value_at(v, frame_no)))


def render(c, frame_no):
    pixels = [p for row in CG.DRAWINGS[c["drawing"]] for p in row]
    n = {**START, **c["new"]}
    out = glint(pixels, *[held(c, k, frame_no) for k in CG.NUMBERS], c["color"], n["ray_colors"],
                n["color_2"], held(c, "shimmer", frame_no), held(c, "shimmer_period", frame_no),
                frame_no)
    return [out(x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    return render(case(intensity=0, shift=c["shift"]), 0)


TWO = {"color_2": "#0080ff"}  # orange and blue: ray colours that tell apart in every channel
ALT = {"ray_colors": "alternate", **TWO}
GRAD = {"ray_colors": "gradient", "color_2": "#0000ff"}
SHIM = {"shimmer": 100, "shimmer_period": 8}

CASES = {
    "FX-GLINT-001": ("The four new settings written at their start values (one colour, #80c0ff, "
                     "shimmer 0, period 24) on Cross Glare's FX-GLARE-005 (angle 45, white): "
                     "FX-GLARE-005's numbers, so a glare drawn before Glint Streaks draws the "
                     "same.",
                     case(angle=45, color="#ffffff", **START), [0]),
    "FX-GLINT-002": ("Alternate, orange #ff8000 and blue #0080ff, a plus (angle 0): rays 0 and "
                     "2, up and down, are orange; rays 1 and 3, right and left, are blue. The "
                     "pixel right of the yellow point takes 0.8 squared over 1.2 of its light "
                     "in blue.",
                     case(**ALT), [0]),
    "FX-GLINT-003": ("Gradient from red #ff0000 to blue #0000ff: up is red, right two thirds "
                     "red and one third blue, down one third red and two thirds blue, left "
                     "blue, mixed in linear light.",
                     case(color="#ff0000", **GRAD), [0]),
    "FX-GLINT-004": ("Alternate with one ray: the ray is ray 0, so it is orange; Cross Glare "
                     "with points 1, orange.",
                     case(points=1, **ALT), [0]),
    "FX-GLINT-005": ("Gradient with one ray: the ray is the first colour; FX-GLINT-004 "
                     "exactly.",
                     case(points=1, ray_colors="gradient", **TWO), [0]),
    "FX-GLINT-006": ("Alternate on eight rays from angle 0: up, right, down and left orange, the "
                     "four diagonals blue.",
                     case(points=8, **ALT), [0]),
    "FX-GLINT-007": ("Alternate with the second colour the same as the first: one colour, Cross "
                     "Glare's own glare.",
                     case(ray_colors="alternate", color_2="#ff8000"), [0]),
    "FX-GLINT-008": ("FX-GLINT-002 with the second colour written in capitals, #0080FF: the "
                     "same.",
                     case(ray_colors="alternate", color_2="#0080FF"), [0]),
    "FX-GLINT-009": ("Shimmer 100, period 8, one colour: on frame 0 ray 0 (up) is at full and "
                     "the others are dimmed by their own phases; on frame 4, half a period on, "
                     "ray 0 is gone; frame 8, a whole period on, is frame 0 again.",
                     case(**SHIM), [0, 2, 4, 8]),
    "FX-GLINT-010": ("Shimmer 50, period 8, alternate: each ray breathes between half and full "
                     "strength in its own colour, never gone.",
                     case(**ALT, shimmer=50, shimmer_period=8), [0, 3]),
    "FX-GLINT-011": ("Shimmer keyed from 0 at frame 0 to 100 at frame 4, linear, period 8: "
                     "frame 0 is the plain glare, frame 4 is FX-GLINT-009's frame 4.",
                     case(shimmer=keyed((0, 0), (4, 100)), shimmer_period=8), [0, 2, 4]),
    "FX-GLINT-012": ("Shimmer 100, period 16: frame 4 is a quarter period on, as frame 2 is at "
                     "period 8, so it is FX-GLINT-009's frame 2.",
                     case(shimmer=100, shimmer_period=16), [4]),
    "FX-GLINT-013": ("Shimmer 100, period 1: every whole frame is a whole period on, so the "
                     "rays hold still, each dimmed by its own phase.",
                     case(shimmer=100, shimmer_period=1), [0, 1, 5]),
    "FX-GLINT-014": ("FX-GLINT-002 moved three pixels right: the layer grew by 4, and the "
                     "yellow's blue left ray shows in the two grown columns left of the "
                     "drawing.",
                     case(shift=3, **ALT), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GLINT-015": ("Shimmer 101, above 100.", case(shimmer=101)),
    "FX-GLINT-016": ("Shimmer period 0.5, below 1.", case(shimmer_period=0.5)),
    "FX-GLINT-017": ("Shimmer period 1001, above 1000.", case(shimmer_period=1001)),
    "FX-GLINT-018": ("Ray colours \"rainbow\", not one of \"one\", \"alternate\" or "
                     "\"gradient\".", case(ray_colors="rainbow")),
    "FX-GLINT-019": ("A second colour written \"#12345\", one digit short.", case(color_2="#12345")),
    "FX-GLINT-020": ("Shimmer keyed to 120 at frame 4.", case(shimmer=keyed((0, 0), (4, 120)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = CG.project_json(fx, c)
    # The shimmer's frames run to 8, so the shot and the layer are nine frames long.
    comp = p["compositions"][0]
    comp["duration_frames"] = comp["work_area"]["end_frame_exclusive"] = 9
    comp["layers"][0]["out_frame"] = 9
    params = comp["layers"][0]["effects"][0]["parameters"]
    for k in NEW:
        if k in c["new"]:
            params[k] = setting_json(c["new"][k])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in CG.DRAWINGS.items():
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

    (OUT / "expected_glint_streaks.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    lit = lambda px, x, y: px[at(x, y)][3] > drawn[at(x, y)][3] + 1e-9  # noqa: E731
    yx, yy = CG.YELLOW
    yellow = R.working(CG.BRIGHT)
    orange, blue = linear("#ff8000"), linear("#0080ff")
    cross = lambda **k: CG.render(CG.case(**k), 0)  # noqa: E731

    # The rule's pieces: one colour at no shimmer is Cross Glare's tint; the phases differ.
    assert ray_colours(4, "alternate", "#ff8000", "#0080ff") == [orange, blue, orange, blue]
    assert ray_colours(1, "gradient", "#ff8000", "#0080ff") == [orange]
    assert strengths(6, 0, 24, 7) == [1.0] * 6 and strengths(3, 100, 8, 0)[0] == 1.0
    assert len({round(p, 9) for p in strengths(8, 100, 8, 0)}) == 8
    # Everywhere: nothing is taken away, the covering stays inside 0 to 1.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-GLINT-014" else 0))
        for px in frames.values():
            for p, q in zip(px, base):
                assert 0 <= p[3] <= 1 and p[3] >= q[3] - 1e-15, fx
                assert all(p[ch] >= q[ch] - 1e-15 for ch in range(3)), fx

    # 001: Cross Glare's FX-GLARE-005.
    assert near(c["FX-GLINT-001"]["0"], cross(length=4))
    # 002: up and down orange, right and left blue, with Cross Glare's weights.
    two = c["FX-GLINT-002"]["0"]
    k = 0.64 / 1.2
    assert near([two[at(yx + 1, yy)]], [[k * blue[ch] * yellow[ch] for ch in range(3)] + [k * yellow[3]]])
    assert near([two[at(yx, yy - 1)]], [[k * orange[ch] * yellow[ch] for ch in range(3)] + [k * yellow[3]]])
    assert near([two[at(yx - 1, yy)]], [two[at(yx + 1, yy)]])
    assert two[at(yx + 1, yy)][2] > two[at(yx, yy - 1)][2] and two[at(yx, yy - 1)][0] > two[at(yx + 1, yy)][0]
    # 003: the gradient's thirds.
    three = c["FX-GLINT-003"]["0"]
    red, deep = linear("#ff0000"), linear("#0000ff")
    right = [(2 / 3) * red[ch] + (1 / 3) * deep[ch] for ch in range(3)]
    assert near([three[at(yx + 1, yy)][:3]], [[k * right[ch] * yellow[ch] for ch in range(3)]])
    assert three[at(yx, yy - 1)][2] == 0 and three[at(yx - 1, yy)][0] == 0
    # 004 and 005: one ray is the first colour.
    assert near(c["FX-GLINT-004"]["0"], cross(length=4, angle=0, points=1, color="#ff8000"))
    assert c["FX-GLINT-005"]["0"] == c["FX-GLINT-004"]["0"]
    # 006: diagonals blue, straight rays orange.
    six = c["FX-GLINT-006"]["0"]
    assert six[at(yx + 1, yy - 1)][2] > six[at(yx + 1, yy - 1)][0] and six[at(yx + 1, yy)][0] > six[at(yx + 1, yy)][2]
    # 007: the same colour twice is Cross Glare's glare; 008: capitals read the same.
    assert near(c["FX-GLINT-007"]["0"], cross(length=4, angle=0, color="#ff8000"))
    assert c["FX-GLINT-008"]["0"] == two
    # 009: ray 0 full on frame 0, gone half a period on; the loop closes.
    nine = c["FX-GLINT-009"]
    plus = cross(length=4, angle=0, color="#ff8000")
    assert near([nine["0"][at(yx, yy - 1)]], [plus[at(yx, yy - 1)]])
    assert not lit(nine["4"], yx, yy - 1) and lit(nine["4"], yx + 1, yy)
    assert near(nine["8"], nine["0"], 1e-9) and nine["2"] != nine["0"]
    # 010: half shimmer never puts a ray out.
    for px in c["FX-GLINT-010"].values():
        assert all(lit(px, x, y) for x, y in ((yx + 1, yy), (yx, yy - 1), (yx - 1, yy), (yx, yy + 1)))
    # 011: the keyed shimmer meets the plain glare and FX-GLINT-009.
    k11 = c["FX-GLINT-011"]
    assert near(k11["0"], plus) and near(k11["4"], nine["4"])
    # 012 and 013: the period scales time; a period of one frame holds still.
    assert near(c["FX-GLINT-012"]["4"], nine["2"])
    t = c["FX-GLINT-013"]
    assert near(t["1"], t["0"], 1e-9) and near(t["5"], t["0"], 1e-9) and not near(t["0"], plus)
    # 014: FX-GLINT-002 moved, the blue left ray on the grown pixels.
    moved = c["FX-GLINT-014"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert moved[at(2, yy)][3] > 0 and moved[at(2, yy)][2] > moved[at(2, yy)][0]
    print("checked")


if __name__ == "__main__":
    main()
