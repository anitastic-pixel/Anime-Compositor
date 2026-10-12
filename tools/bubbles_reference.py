"""Bubbles, worked a second way.

D-671 adds Bubbles (`core.bubbles`), after CycoreFX's CC Bubbles (Simulation): the layer turned
into bubbles that rise (or fall) and wobble, each showing the picture through itself. The
CycoreFX manual names the controls and says what each is for (Bubble Amount, Bubble Speed,
rising when positive, Wobble Amplitude and Frequency, Bubble Size, Reflection Type Inverse,
"self-contained, like refraction", or World, "reflects the source layer", and the five Shading
Types); it publishes no formula, so the rule, the units, the ranges and the settings when added
are ours. No state is carried from frame to frame (document 20): everything is worked from the
layer's in point.

At composition frame n, the layer's in point i0, r frames a second, u = max(0, n - i0) frames
since the in point, on a drawing w by h (its own size, before anything an earlier effect grew),
with the settings held inside their ranges:

1. Two running sums from the in point, in seconds: S, the integral of `speed`, and F, the
   integral of `wobble_frequency`. Each is the trapezoid sum over the frames i0 .. i0 + u of
   the setting at each frame (exact for a setting that is not keyed, and for linear keys).
2. The bubbles are i = 0 .. floor(`amount`) - 1, at n's amount. Each takes six numbers u_k =
   (U(seed, i, 0, 0, k) + 1) / 2, k = 0 .. 5, U Noise's hash (D-119), seed
   floor(`random_seed`).
3. Its radius rho_i = `size` (1/2 + u_2) h / 40 pixels; none is drawn whose radius is 0. Its
   pace p_i = 1/2 + u_3. Its rise R_i = h + 2 rho_i: from wholly below the picture to wholly
   above it. q = u_1 + p_i S (h / 4) / R_i (so speed 1 lifts a bubble of pace 1 a quarter of
   the picture a second), and its centre's y = h + rho_i - frac(q) R_i: rising for a positive
   speed, falling for a negative one, round again when it leaves.
4. Across: x = u_0 (w + 2 rho_i) - rho_i + (`wobble_amplitude` / 10) rho_i sin(2 pi phi), phi
   = u_4 + (3/4 + u_5 / 2) F. Wobble 10 swings a bubble its own radius either side.
5. At a pixel centre (X, Y), a bubble at (x, y) radius rho: d its distance, the cover clamp(rho
   - d + 1/2, 0, 1). The unit offset e = (X - x, Y - y) / rho, cut to length 1 outside the
   bubble; t = |e|, z = sqrt(1 - t^2). The picture is read at (x, y) - 2 rho e / (1 + z) for
   `reflection` `inverse` (the picture under the bubble turned over and spread, as a glass ball
   shows it) or (x, y) + 2 rho e / (1 + z) for `world` (the picture round it, upright), with
   the bilinear read below.
6. f = t^2. `shading` `none` keeps that colour c; `lighten` is c (1 - f) + white f, opaque;
   `darken` is c (1 - f) + black f, opaque; `fade_inwards` is c f (clear at the middle);
   `fade_outwards` is c (1 - f) (clear at the rim).
7. The bubbles are laid in order of i, each at its cover, normal, onto clear: the layer becomes
   the bubbles alone.

The bilinear read, premultiplied and linear: fx = clamp(X' - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge.

`amount` 0 to 10000, 100 when added; `speed` -100 to 100, 0.5; `wobble_amplitude` 0 to 1000,
5; `wobble_frequency` 0 to 100 a second, 1; `size` 0 to 100, 1; `random_seed` 0 to 10000, 0;
`reflection` `inverse` or `world`, inverse; `shading` one of `none`, `lighten`, `darken`,
`fade_inwards`, `fade_outwards`, fade_inwards. Every number is keyable. The layer does not grow.
Every length is a share of the drawing's height, so a draft is the same picture smaller.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 48 pixels by 32 holding one drawing the same size: a colour field, or
the field with its right half clear. The drawings go into `Fixtures/bubbles/media`, the projects
into `Fixtures/bubbles`, and the expected frames into `Fixtures/bubbles/expected_bubbles.json`.
Every case is "not built yet" until B-550 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bubbles_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
from noise_reference import u as U  # noqa: E402

W, H, FPS = 48, 32, 24
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bubbles"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.bubbles"
SETTINGS = [("amount", 0, 10000, 100), ("speed", -100, 100, 0.5),
            ("wobble_amplitude", 0, 1000, 5), ("wobble_frequency", 0, 100, 1),
            ("size", 0, 100, 1), ("random_seed", 0, 10000, 0)]
WORDS = [("reflection", ("inverse", "world"), "inverse"),
         ("shading", ("none", "lighten", "darken", "fade_inwards", "fade_outwards"),
          "fade_inwards")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
NEAR = []  # how near a wrap each drawn bubble comes, for the printout


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def sums(c, n):
    """Step 1: the integrals of speed and wobble frequency from the in point, in seconds."""
    i0 = c["in_frame"]
    u = max(0, n - i0)
    s = [settings(c, i0 + j) for j in range(u + 1)]
    S_ = sum((s[j]["speed"] + s[j + 1]["speed"]) / 2 for j in range(u)) / c["rate"]
    F = sum((s[j]["wobble_frequency"] + s[j + 1]["wobble_frequency"]) / 2
            for j in range(u)) / c["rate"]
    return S_, F


def bubbles_of(v, S_, F, w, h):
    seed = math.floor(v["random_seed"])
    out = []
    for i in range(math.floor(v["amount"])):
        u = [(U(seed, i, 0, 0, k) + 1) / 2 for k in range(6)]
        rho = v["size"] * (0.5 + u[2]) * h / 40
        if rho == 0:
            continue
        p = 0.5 + u[3]
        R = h + 2 * rho
        q = u[1] + p * S_ * (h / 4) / R
        NEAR.append(abs(q - round(q)))
        y = h + rho - (q - math.floor(q)) * R
        phi = u[4] + (0.75 + u[5] / 2) * F
        x = u[0] * (w + 2 * rho) - rho + v["wobble_amplitude"] / 10 * rho * math.sin(2 * math.pi * phi)
        out.append((x, y, rho))
    return out


def bilinear(src, X, Y, w, h):
    fx, fy = clamp(X - 0.5, 0, w - 1), clamp(Y - 0.5, 0, h - 1)
    x0, y0 = math.floor(fx), math.floor(fy)
    x1, y1 = min(x0 + 1, w - 1), min(y0 + 1, h - 1)
    tx, ty = fx - x0, fy - y0
    a, b, c_, d = src[y0 * w + x0], src[y0 * w + x1], src[y1 * w + x0], src[y1 * w + x1]
    return [(a[k] * (1 - tx) + b[k] * tx) * (1 - ty) + (c_[k] * (1 - tx) + d[k] * tx) * ty
            for k in range(4)]


def bubbles(src, v, words, S_, F, w=W, h=H):
    """`src` the drawing premultiplied linear; the layer after the effect."""
    sign = -1 if words["reflection"] == "inverse" else 1
    shading = words["shading"]
    out = [[0.0] * 4 for _ in range(w * h)]
    for (x, y, rho) in bubbles_of(v, S_, F, w, h):
        for py in range(max(0, math.floor(y - rho - 1)), min(h, math.ceil(y + rho + 1) + 1)):
            for px in range(max(0, math.floor(x - rho - 1)), min(w, math.ceil(x + rho + 1) + 1)):
                X, Y = px + 0.5, py + 0.5
                dx, dy = X - x, Y - y
                d = math.hypot(dx, dy)
                cover = clamp(rho - d + 0.5, 0.0, 1.0)
                if cover == 0:
                    continue
                if d > rho:
                    ex, ey = dx / d, dy / d
                else:
                    ex, ey = dx / rho, dy / rho
                t2 = min(1.0, ex * ex + ey * ey)
                k = 2 * rho / (1 + math.sqrt(1 - t2))
                c = bilinear(src, x + sign * k * ex, y + sign * k * ey, w, h)
                f = t2
                if shading == "lighten":
                    b = [c[j] * (1 - f) + f for j in range(4)]
                elif shading == "darken":
                    b = [c[j] * (1 - f) for j in range(3)] + [c[3] * (1 - f) + f]
                elif shading == "fade_inwards":
                    b = [c[j] * f for j in range(4)]
                elif shading == "fade_outwards":
                    b = [c[j] * (1 - f) for j in range(4)]
                else:
                    b = c
                o = out[py * w + px]
                out[py * w + px] = [b[j] * cover + o[j] * (1 - b[3] * cover) for j in range(4)]
    return out


# --- the drawings ---------------------------------------------------------------------------

def field(x, y):
    return (40 + 4 * x, 30 + 6 * y, 220 - 3 * x - 2 * y, 255)


DRAWINGS = {"field": [[field(x, y) for x in range(W)] for y in range(H)],
            "half": [[field(x, y) if x < W // 2 else S.NONE for x in range(W)] for y in range(H)]}


def working(drawing):
    out = []
    for row in DRAWINGS[drawing]:
        for p in row:
            a = p[3] / 255
            out.append([srgb_to_linear(p[k] / 255) * a for k in range(3)] + [a])
    return out


def frame(layer, shift):
    return [layer[y * W + x - shift] if 0 <= x - shift < W else [0.0] * 4
            for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    for k, v in given.items():
        assert k in c, k
        c[k] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift)
    return c


# Six bubbles about 3 to 10 pixels across the radius: a close look.
FEW = dict(amount=6, size=8, random_seed=4)


def few(**kw):
    return case(**{**FEW, **kw})


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    return all(c[name] in words for name, words, _ in WORDS)


def render(c, n):
    src = working(c["drawing"])
    if not valid(c) or n < c["in_frame"]:
        return frame(src, c["shift"])
    S_, F = sums(c, n)
    out = bubbles(src, settings(c, n), {w[0]: c[w[0]] for w in WORDS}, S_, F)
    return frame(out, c["shift"])


def plain(c):
    return frame(working(c["drawing"]), c["shift"])


CASES = {
    "FX-BUBBLE-001": ("As added: 100 bubbles of size 1 (on a picture 32 high, between half a "
                      "pixel and a pixel and a half in radius), speed 0.5, wobble 5 at once a "
                      "second, Inverse, Fade Inwards: a scatter of tiny faint dots on clear, the "
                      "picture itself gone.", case(), (0, 24)),
    "FX-BUBBLE-002": ("Six bubbles of size 8, 3 to 10 pixels in radius, each the picture under "
                      "it turned over and spread, clear in the middle, rising: frame 12 is half "
                      "a second later.", few(), (0, 12)),
    "FX-BUBBLE-003": ("World reflection: each bubble the picture round it, upright.",
                      few(reflection="world"), (0,)),
    "FX-BUBBLE-004": ("Shading None: fully filled bubbles.", few(shading="none"), (0,)),
    "FX-BUBBLE-005": ("Lighten: white towards the rim.", few(shading="lighten"), (0,)),
    "FX-BUBBLE-006": ("Darken: black towards the rim.", few(shading="darken"), (0,)),
    "FX-BUBBLE-007": ("Fade Outwards: clear at the rim, solid in the middle.",
                      few(shading="fade_outwards"), (0,)),
    "FX-BUBBLE-008": ("Speed -2: falling, half the picture a second at pace 1.",
                      few(speed=-2), (0, 6)),
    "FX-BUBBLE-009": ("Speed 0 and wobble 0: still, so frame 12 is frame 0.",
                      few(speed=0, wobble_amplitude=0), (0, 12)),
    "FX-BUBBLE-010": ("Wobble 20 at twice a second: each swings twice its radius either side.",
                      few(wobble_amplitude=20, wobble_frequency=2), (0, 3)),
    "FX-BUBBLE-011": ("Speed 0, wobble frequency 0: the wobble holds where it started, so frame "
                      "6 is frame 0.", few(speed=0, wobble_frequency=0, wobble_amplitude=20),
                      (0, 6)),
    "FX-BUBBLE-012": ("Size 0: no bubble, clear.", few(size=0), (3,)),
    "FX-BUBBLE-013": ("Amount 0: no bubble, clear.", few(amount=0), (3,)),
    "FX-BUBBLE-014": ("Seed 7: bubbles of their own.", few(random_seed=7), (3,)),
    "FX-BUBBLE-015": ("Seed 7.9 counts as 7: FX-BUBBLE-014.", few(random_seed=7.9), (3,)),
    "FX-BUBBLE-016": ("The picture with its right half clear: a bubble over it shows clear.",
                      few(drawing="half", shading="none", amount=12), (0,)),
    "FX-BUBBLE-017": ("Speed keyed from 0 at frame 0 to 4 at frame 8: by frame 8 the bubbles "
                      "have risen as far as at a steady 2 for a third of a second.",
                      few(speed=keyed((0, 0), (8, 4))), (4, 8)),
    "FX-BUBBLE-018": ("Wobble frequency keyed from 0 to 3 over 8 frames, wobble 20, speed 0.",
                      few(speed=0, wobble_amplitude=20, wobble_frequency=keyed((0, 0), (8, 3))),
                      (4, 8)),
    "FX-BUBBLE-019": ("FX-BUBBLE-002 with the layer starting at frame 6: frame 6 is 002's frame "
                      "0 and frame 18 its frame 12, because the bubbles start at the in point.",
                      few(in_frame=6), (6, 18)),
    "FX-BUBBLE-020": ("FX-BUBBLE-002 moved 3 pixels right: the same bubbles, moved, cut at the "
                      "layer's edge.", few(shift=3), (12,)),
    "FX-BUBBLE-021": ("FX-BUBBLE-002 at 30 frames a second: frame 12 is two fifths of a second "
                      "in, not a half.", few(rate=30), (12,)),
    "FX-BUBBLE-022": ("Forty bubbles with everything: size 5, speed 1.5, wobble 12 at 1.5, "
                      "World, Lighten, seed 3.",
                      few(amount=40, size=5, speed=1.5, wobble_amplitude=12, wobble_frequency=1.5,
                          reflection="world", shading="lighten", random_seed=3), (0, 10)),
    "FX-BUBBLE-023": ("Amount keyed from 0 at frame 0 to 12 at frame 4: six at frame 2, "
                      "FX-BUBBLE-002's six.", few(amount=keyed((0, 0), (4, 12))), (0, 2)),
    "FX-BUBBLE-024": ("Size 40: bubbles bigger than the picture, each a fisheye of it.",
                      few(amount=2, size=40, shading="none"), (0,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-BUBBLE-025": ("Amount -1, below 0.", few(amount=-1)),
    "FX-BUBBLE-026": ("Amount 10001, above 10000.", few(amount=10001)),
    "FX-BUBBLE-027": ("Speed 101, above 100.", few(speed=101)),
    "FX-BUBBLE-028": ("Wobble amplitude -1, below 0.", few(wobble_amplitude=-1)),
    "FX-BUBBLE-029": ("Wobble frequency 101, above 100.", few(wobble_frequency=101)),
    "FX-BUBBLE-030": ("Size 101, above 100.", few(size=101)),
    "FX-BUBBLE-031": ("Random seed -1, below 0.", few(random_seed=-1)),
    "FX-BUBBLE-032": ("Reflection \"mirror\".", few(reflection="mirror")),
    "FX-BUBBLE-033": ("Shading \"Lighten\": the word is exact, so capitals are not it.",
                      few(shading="Lighten")),
    "FX-BUBBLE-034": ("Speed keyed to 200 at frame 4, above 100.",
                      few(speed=keyed((0, 1), (4, 200)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    frames = 30
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": c["rate"], "denominator": 1},
            "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-" + d,
                "enabled": True, "locked": False, "in_frame": c["in_frame"], "out_frame": frames,
                "source_offset_frames": 0,
                "transform": {
                    "anchor": prop([0, 0]), "position": prop([c["shift"], 0]),
                    "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
                },
                "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": TYPE, "enabled": True,
                             "parameters": params}],
            }],
        }],
    }


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
        assert valid(c), fx
        NEAR.clear()
        rendered = {str(f): render(c, f) for f in frames}
        assert not NEAR or min(NEAR) > 1e-9, fx
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        assert not valid(c), fx
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_bubbles.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    assert 0 < total(c["FX-BUBBLE-001"]["0"]) < W * H * 0.2
    assert c["FX-BUBBLE-001"]["0"] != c["FX-BUBBLE-001"]["24"]
    f0 = c["FX-BUBBLE-002"]["0"]
    assert total(f0) > 0 and f0 != c["FX-BUBBLE-002"]["12"]
    for n in (3, 4, 5, 6, 7):
        assert c["FX-BUBBLE-%03d" % n]["0"] != f0, n
    assert total(c["FX-BUBBLE-004"]["0"]) > total(f0)
    assert c["FX-BUBBLE-009"]["0"] == c["FX-BUBBLE-009"]["12"]
    assert c["FX-BUBBLE-011"]["0"] == c["FX-BUBBLE-011"]["6"]
    assert c["FX-BUBBLE-012"]["3"] == clear and c["FX-BUBBLE-013"]["3"] == clear
    assert c["FX-BUBBLE-014"]["3"] == c["FX-BUBBLE-015"]["3"]
    assert c["FX-BUBBLE-014"]["3"] != render(few(), 3)
    half = c["FX-BUBBLE-016"]["0"]
    assert total(half) < total(render(few(shading="none", amount=12), 0))
    assert c["FX-BUBBLE-019"]["6"] == f0 and c["FX-BUBBLE-019"]["18"] == c["FX-BUBBLE-002"]["12"]
    moved = c["FX-BUBBLE-020"]["12"]
    base = c["FX-BUBBLE-002"]["12"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert c["FX-BUBBLE-021"]["12"] != base
    assert c["FX-BUBBLE-023"]["0"] == clear and c["FX-BUBBLE-023"]["2"] == render(few(), 2)
    # the keyed speed: at frame 8 the sum is the steady 2 for 8 frames
    S_, F = sums(few(speed=keyed((0, 0), (8, 4))), 8)
    assert abs(S_ - 2 * 8 / FPS) < 1e-12 and abs(F - 8 / FPS) < 1e-12
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
