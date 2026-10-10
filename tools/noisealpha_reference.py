"""Noise Alpha, worked a second way.

D-450 adds `core.noise_alpha`, After Effects' Noise Alpha (Noise & Grain): it "adds noise to the
alpha channel" (Adobe's help page on the Noise & Grain effects). Its settings are After Effects':
Noise (Uniform Random, Squared Random, Uniform Animation, Squared Animation), Amount, Original
Alpha (Add, Clamp, Scale, Edges), Overflow (Clip, Wrap Back, Wrap), Random Seed, Noise Phase and
the Noise Options' Cycle Noise and Cycle. Adobe describes each control in words and publishes no
formula, and its starting values were not to be read; the numbers below are this program's own
rule, built on D-119's per-pixel number (`grade::unit`) and P0-19's value noise (`grade::value`,
D-127, D-128, D-299). Nothing is ported.

The rule, at a pixel (x, y) of the drawing's own pixels (its top-left corner (0, 0), however far
an effect above grew the buffer), covering a, straight colour b (the stored colour over a, or
black where a is 0):

1. The noise n in -1..1. Uniform Random: D-119's number for the seed's whole part at (x, y),
   depth 0, channel 0; it does not change from frame to frame (key the seed for that). Uniform
   Animation: the depth z is the Noise Phase in turns (phase / 360) and n is P0-19's value noise
   in blocks of one pixel at (x, y, z) for seed 0 (After Effects offers the seed for the Random
   kinds only), so a full turn of phase is a whole new field, reached smoothly. With Cycle Noise
   on, the depth's cells repeat after Cycle turns (its whole part), so the noise comes back to
   where it began.
2. Squared: n becomes sign(n) (1 - (1 - |n|)^2), pushed out towards -1 and 1, higher in
   contrast.
3. The step d = n amount / 100, in covering (a whole step is 0 to fully covered).
4. Original Alpha. Add: t = a + d, everywhere. Clamp: only where the pixel is fully covered
   (a = 1); elsewhere the pixel is left. Scale: t = a + d a, so in proportion to the covering
   and nothing where it is empty. Edges: only where the pixel is partly covered (0 < a < 1).
5. Overflow, when t leaves 0 to 1. Clip: held at 0 or 1. Wrap Back: reflected back in, 1.1
   to 0.9 and -0.1 to 0.1. Wrap: round the other end, 1.1 to 0.1 and -0.1 to 0.9.
6. The pixel becomes b t, t. A pixel whose covering comes out as it was is left as it was.

Amount 0 changes nothing.

Settings: `noise` `uniform_random` (when added), `squared_random`, `uniform_animation` or
`squared_animation`; `amount` 0 to 100 per cent, 20; `original_alpha` `clamp` (when added),
`add`, `scale` or `edges`; `overflow` `clip` (when added), `wrap_back` or `wrap`; `random_seed`
0 to 100000, 0, its whole part counted; `noise_phase` -100000 to 100000 degrees, 0;
`cycle_noise` `off` (when added) or `on`; `cycle` 1 to 1000 turns, 1, its whole part counted.
Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Noise's card, the same size, unmoved unless the case
says. The expected frames are in `Fixtures/noisealpha/expected_noisealpha.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/noisealpha_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import DRAWINGS, u  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "noisealpha"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 100), "random_seed": (0, 100000), "noise_phase": (-100000, 100000),
          "cycle": (1, 1000)}
WORDS = ("noise", "original_alpha", "overflow", "cycle_noise")
NAMES = ("noise", "amount", "original_alpha", "overflow", "random_seed", "noise_phase",
         "cycle_noise", "cycle")


# --- the rule -------------------------------------------------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def along(x, y, z, period):
    """grade::value in blocks of one pixel at (x, y, z), seed 0: the two depths round z, mixed by
    the smoothed place between them; a depth that weighs 0 is skipped. With a period the depth's
    cells repeat."""
    k = math.floor(z)
    s = fade(z - k)
    depth = (lambda k: k % period) if period > 0 else (lambda k: k)
    v = 0.0
    if 1 - s != 0:
        v += (1 - s) * u(0, x, y, depth(k), 0)
    if s != 0:
        v += s * u(0, x, y, depth(k + 1), 0)
    return v


def squared(n):
    return math.copysign(1 - (1 - abs(n)) ** 2, n)


def overflow(t, how):
    if 0 <= t <= 1:
        return t
    if how == "wrap_back":
        t = 2 - t if t > 1 else -t
    elif how == "wrap":
        t = t - 1 if t > 1 else t + 1
    return min(1.0, max(0.0, t))


def noise_alpha(layer, n, w):
    if n["amount"] == 0:
        return layer
    animated = w["noise"].endswith("_animation")
    seed = math.floor(n["random_seed"])
    z = n["noise_phase"] / 360
    period = math.floor(n["cycle"]) if w["cycle_noise"] == "on" else 0
    k = n["amount"] / 100
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            a = p[3]
            x, y = layer["left"] + i, layer["top"] + j
            how = w["original_alpha"]
            if (how == "clamp" and a != 1) or (how == "edges" and not 0 < a < 1) \
                    or (how == "scale" and a <= 0):
                px.append(p)
                continue
            v = along(x, y, z, period) if animated else u(seed, x, y, 0, 0)
            if w["noise"].startswith("squared"):
                v = squared(v)
            d = k * v
            t = overflow(a + d * a if how == "scale" else a + d, w["overflow"])
            if t == a:
                px.append(p)
                continue
            b = [c / a for c in p[:3]] if a > 0 else [0.0, 0.0, 0.0]
            px.append([c * t for c in b] + [t])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(noise="uniform_random", amount=20, original_alpha="clamp", overflow="clip",
         random_seed=0, noise_phase=0, cycle_noise="off", cycle=1, shift=0, tile=False):
    c = dict(locals())
    c["drawing"] = "card"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(noise_alpha(layer_of(c), n, {k: c[k] for k in WORDS}), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


ADD = {"original_alpha": "add"}
ANIM = {"noise": "uniform_animation", "original_alpha": "add"}

CASES = {
    "FX-NOISEALPHA-001": ("The settings as they start: Uniform Random, amount 20, Clamp, Clip; "
                          "only the fully covered pixels change, each losing up to 0.2 of its "
                          "covering or, where the noise would add, staying covered; the empty "
                          "pixels and the soft edge are left; the colour is kept; the same on "
                          "every frame.", case(), [0, 2]),
    "FX-NOISEALPHA-002": ("Add: the empty pixels gain up to 0.2 of covering, in black, the soft "
                          "edge moves up to 0.2 either way keeping its colour, and the covered "
                          "pixels lose up to 0.2.", case(**ADD), [0]),
    "FX-NOISEALPHA-003": ("Scale: the noise in proportion to the covering, so the empty pixels are "
                          "left and the soft edge moves half as far as the step.",
                          case(original_alpha="scale"), [0]),
    "FX-NOISEALPHA-004": ("Edges: only the soft edge, partly covered, changes.",
                          case(original_alpha="edges"), [0]),
    "FX-NOISEALPHA-005": ("Squared Random, Add: the noise pushed out towards its ends, so the soft "
                          "edge moves at least as far as in FX-NOISEALPHA-002, the same way.",
                          case(noise="squared_random", **ADD), [0]),
    "FX-NOISEALPHA-006": ("Amount 100, Add, Wrap Back: a covered pixel the noise would push past "
                          "full is reflected back as far, an empty one pushed below nothing comes "
                          "back up as far.", case(amount=100, overflow="wrap_back", **ADD), [0]),
    "FX-NOISEALPHA-007": ("Amount 100, Add, Wrap: past full comes round from nothing, below "
                          "nothing comes round from full.",
                          case(amount=100, overflow="wrap", **ADD), [0]),
    "FX-NOISEALPHA-008": ("Amount 100, Add, Clip: held at nothing and full.",
                          case(amount=100, **ADD), [0]),
    "FX-NOISEALPHA-009": ("Uniform Animation at phase 0, Add: the noise at depth 0 for seed 0, so "
                          "the frame is FX-NOISEALPHA-002's.", case(**ANIM), [0]),
    "FX-NOISEALPHA-010": ("Uniform Animation at phase 180, Add: half way between depth 0 and depth "
                          "1 in the noise.", case(noise_phase=180, **ANIM), [0]),
    "FX-NOISEALPHA-011": ("Uniform Animation, Noise Phase keyed from 0 at frame 0 to 720 at frame "
                          "4, Add: a new field each turn, reached smoothly; frame 1 is phase 180, "
                          "FX-NOISEALPHA-010's.", case(noise_phase=keyed((0, 0), (4, 720)), **ANIM),
                          [0, 1, 2, 4]),
    "FX-NOISEALPHA-012": ("FX-NOISEALPHA-011 with Cycle Noise on and Cycle 2: after two turns the "
                          "noise is where it began, so frame 4 is frame 0.",
                          case(noise_phase=keyed((0, 0), (4, 720)), cycle_noise="on", cycle=2,
                               **ANIM), [0, 2, 4]),
    "FX-NOISEALPHA-013": ("Cycle 1: every whole turn is the start, so phase 360 is phase 0.",
                          case(noise_phase=360, cycle_noise="on", cycle=1, **ANIM), [0]),
    "FX-NOISEALPHA-014": ("Squared Animation at phase 90, Add.",
                          case(noise="squared_animation", noise_phase=90, original_alpha="add"),
                          [0]),
    "FX-NOISEALPHA-015": ("Random Seed 7, Add: a different noise from FX-NOISEALPHA-002's.",
                          case(random_seed=7, **ADD), [0]),
    "FX-NOISEALPHA-016": ("Random Seed 7.6, Add: its whole part counts, so this is "
                          "FX-NOISEALPHA-015.", case(random_seed=7.6, **ADD), [0]),
    "FX-NOISEALPHA-017": ("Uniform Animation with Random Seed 7: the seed is for the Random kinds "
                          "only, so this is FX-NOISEALPHA-009.", case(random_seed=7, **ANIM), [0]),
    "FX-NOISEALPHA-018": ("Amount 0: the drawing, untouched.", case(amount=0, **ADD), [0, 2]),
    "FX-NOISEALPHA-019": ("FX-NOISEALPHA-002 moved three pixels right: the noise is the drawing's "
                          "own, so it moves with it.", case(shift=3, **ADD), [0]),
    "FX-NOISEALPHA-020": ("After a Motion Tile that grows the layer: the noise is worked in the "
                          "drawing's own pixels, so the frame is FX-NOISEALPHA-002's.",
                          case(tile=True, **ADD), [0]),
    "FX-NOISEALPHA-021": ("Amount keyed from 0 at frame 0 to 40 at frame 4, Add: frame 0 is the "
                          "drawing; on the soft edge frame 4's step is twice frame 2's.",
                          case(amount=keyed((0, 0), (4, 40)), **ADD), [0, 2, 4]),
    "FX-NOISEALPHA-022": ("Squared Animation, amount 60, Scale, Wrap Back, Cycle Noise on with "
                          "Cycle 3, phase keyed 0 to 400: the controls together.",
                          case(noise="squared_animation", amount=60, original_alpha="scale",
                               overflow="wrap_back", cycle_noise="on", cycle=3,
                               noise_phase=keyed((0, 0), (4, 400))), [0, 1, 2, 3, 4]),
}

INVALID = {
    "FX-NOISEALPHA-023": ("Amount 101, above 100.", case(amount=101)),
    "FX-NOISEALPHA-024": ("Random Seed 100001, above 100000.", case(random_seed=100001)),
    "FX-NOISEALPHA-025": ("Noise Phase 200000, above 100000.", case(noise_phase=200000)),
    "FX-NOISEALPHA-026": ("Cycle 0.5, below 1.", case(cycle=0.5)),
    "FX-NOISEALPHA-027": ("Noise \"uniform\", not one of its four words.", case(noise="uniform")),
    "FX-NOISEALPHA-028": ("Original Alpha \"Add\", in capitals, kept as written and not the word.",
                          case(original_alpha="Add")),
    "FX-NOISEALPHA-029": ("Overflow \"wrapback\", not one of its three words.",
                          case(overflow="wrapback")),
    "FX-NOISEALPHA-030": ("Cycle Noise \"yes\", not off or on.", case(cycle_noise="yes")),
    "FX-NOISEALPHA-031": ("Amount keyed to 150 at frame 4, above 100.",
                          case(amount=keyed((0, 20), (4, 150)))),
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
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.noise_alpha",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    comp["layers"][0]["effects"] = effects
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
    (OUT / "expected_noisealpha.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g: all(near(f[i], g[i]) for i in range(W * H))  # noqa: E731
    full = [i for i in range(W * H) if drawn[i][3] == 1]
    empty = [i for i in range(W * H) if drawn[i][3] == 0]
    edge = [at(15, y) for y in range(9)]
    colour = lambda p: [v / p[3] for v in p[:3]]  # noqa: E731
    step = lambda f, i: f[i][3] - drawn[i][3]  # noqa: E731
    assert len(full) == 14 * 9 and len(empty) == 25 and len(edge) == 9

    # The rule's pieces.
    assert along(3, 4, 0.0, 0) == u(0, 3, 4, 0, 0) and along(3, 4, 2.0, 2) == u(0, 3, 4, 0, 0)
    assert along(3, 4, 0.5, 0) == 0.5 * u(0, 3, 4, 0, 0) + 0.5 * u(0, 3, 4, 1, 0)
    assert squared(0.5) == 0.75 and squared(-0.5) == -0.75 and squared(0.0) == 0.0
    assert abs(overflow(1.1, "wrap_back") - 0.9) < 1e-12 and abs(overflow(-0.1, "wrap_back") - 0.1) < 1e-12
    assert abs(overflow(1.1, "wrap") - 0.1) < 1e-12 and abs(overflow(-0.1, "wrap") - 0.9) < 1e-12
    assert overflow(1.1, "clip") == 1 and overflow(-0.1, "clip") == 0

    one = c["FX-NOISEALPHA-001"]
    assert one["0"] == one["2"]
    f = one["0"]
    for i in range(W * H):
        if i in full:
            assert -0.2 - 1e-12 <= step(f, i) <= 0 and near(colour(f[i]), colour(drawn[i]), 1e-9)
        else:
            assert f[i] == drawn[i]
    assert sum(step(f, i) < 0 for i in full) > len(full) // 3
    two = c["FX-NOISEALPHA-002"]["0"]
    assert all(two[i][:3] == [0.0] * 3 and 0 <= two[i][3] <= 0.2 + 1e-12 for i in empty)
    assert sum(two[i][3] > 0 for i in empty) > 5
    assert all(abs(step(two, i)) <= 0.2 + 1e-12 and near(colour(two[i]), colour(drawn[i]), 1e-9)
               for i in edge)
    assert all(two[i] == f[i] for i in full)  # Add and Clamp agree where it is fully covered
    three = c["FX-NOISEALPHA-003"]["0"]
    assert all(three[i] == drawn[i] for i in empty)
    for i in edge:
        assert abs(step(three, i) - step(two, i) * drawn[i][3]) < 1e-12
    four = c["FX-NOISEALPHA-004"]["0"]
    assert all(four[i] == drawn[i] for i in range(W * H) if i not in edge)
    assert all(four[i] == two[i] for i in edge) and any(four[i] != drawn[i] for i in edge)
    five = c["FX-NOISEALPHA-005"]["0"]
    for i in edge:
        assert abs(step(five, i)) >= abs(step(two, i)) - 1e-12 and step(five, i) * step(two, i) >= 0
    six, seven, eight = (c[f"FX-NOISEALPHA-00{n}"]["0"] for n in (6, 7, 8))
    for i in full:
        d = u(0, i % W, i // W, 0, 0)
        assert abs(six[i][3] - (1 - abs(d))) < 1e-12
        assert abs(seven[i][3] - (d if d > 0 else 1 + d)) < 1e-12
        assert abs(eight[i][3] - min(1, 1 + d)) < 1e-12
    for i in empty:
        d = u(0, i % W, i // W, 0, 0)
        assert abs(six[i][3] - abs(d)) < 1e-12
        assert abs(seven[i][3] - (d if d >= 0 else 1 + d)) < 1e-12
    assert c["FX-NOISEALPHA-009"]["0"] == two
    ten = c["FX-NOISEALPHA-010"]["0"]
    at1 = render(case(noise_phase=360, **ANIM), 0)
    for i in edge:
        assert abs(step(ten, i) - (step(two, i) + step(at1, i)) / 2) < 1e-12
    k11 = c["FX-NOISEALPHA-011"]
    assert k11["0"] == two and k11["1"] == ten and same(k11["2"], at1) and not same(k11["4"], two)
    k12 = c["FX-NOISEALPHA-012"]
    assert same(k12["4"], k12["0"]) and not same(k12["2"], k12["0"])
    assert same(c["FX-NOISEALPHA-013"]["0"], two)
    assert not same(c["FX-NOISEALPHA-014"]["0"], two)
    fifteen = c["FX-NOISEALPHA-015"]["0"]
    assert not same(fifteen, two) and c["FX-NOISEALPHA-016"]["0"] == fifteen
    assert c["FX-NOISEALPHA-017"]["0"] == two
    assert c["FX-NOISEALPHA-018"]["0"] == drawn == c["FX-NOISEALPHA-018"]["2"]
    moved = c["FX-NOISEALPHA-019"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert same(c["FX-NOISEALPHA-020"]["0"], two)
    k21 = c["FX-NOISEALPHA-021"]
    assert k21["0"] == drawn
    for i in edge:
        assert abs(step(k21["4"], i) - 2 * step(k21["2"], i)) < 1e-12
    k22 = c["FX-NOISEALPHA-022"]
    assert not same(k22["0"], k22["4"]) and not same(k22["0"], drawn)
    assert all(k22[f][i] == drawn[i] for f in k22 for i in empty)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(0 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
