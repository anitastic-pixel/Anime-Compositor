"""D-757's Turbulent Displace types and Offset, worked a second way.

Wave 1 and 2 of the 2026-10-22 effects audit (row: Turbulent Displace, missing controls) found
After Effects' Turbulent Displace offering nine Displacement types (Turbulent, Bulge, Twist,
Turbulent Smoother, Bulge Smoother, Twist Smoother, Vertical, Horizontal and Cross
Displacement) and an Offset (Turbulence) point, where this program had Turbulent, Horizontal and
Vertical (D-306) and no offset. Turbulent Displace is an inspired effect (D-127: "modelled on
After Effects' Turbulent Displace in spirit and not claimed to match it"), so D-757 only adds:
every existing setting keeps its meaning and every existing fixture its values.

`core.turbulent_displace` gains:

- `offset`, two numbers in pixels, keyable, absent [0, 0], a distance: the field slid by that
  much in the drawing's own space, as Fractal Noise's Offset (D-299) slides its clouds. After
  Effects' Offset starts at the layer's middle; this one starts at nothing, the slide from where
  the field was.
- six more `displacement` words: "cross", "bulge", "twist", "turbulent_smoother",
  "bulge_smoother" and "twist_smoother".

The rule, this program's own; nothing is ported. With p the field's point for the pixel, D-127's
point slid back by the drift (D-411) and the offset, ((X + 0.5 - d_x - o_x) / size,
(Y + 0.5 - d_y - o_y) / size, z), and A the push (D-328), n_c(p) is channel c of the fractal
field F (D-127, D-128), each octave half the last's strength, or a quarter of it for the three
Smoother words, so their finer octaves count for less and the warp is smoother. The pixel reads
from its centre pushed by D:

- turbulent, turbulent_smoother: A (n_0(p), n_1(p)), as before;
- horizontal, vertical: as before (D-306);
- cross: A (n_0(p_x / 8, p_y, z), n_1(p_x, p_y / 8, z)), the sideways push from a field
  stretched eight times across and the up-and-down push from one stretched eight times down,
  so straight streaks cross each other, as After Effects' Cross Displacement draws them;
- bulge, bulge_smoother: with h = 1/4, g = (n_0(p + (h, 0)) - n_0(p - (h, 0)),
  n_0(p + (0, h)) - n_0(p - (0, h))), each held to -1..1: D = A g, each pixel reading from
  uphill, so the field's hills swell like lenses;
- twist, twist_smoother: D = A (-g_y, g_x), g turned a quarter, so the pixels are carried round
  the hills, swirling.

Every push stays within A on each axis, so the layer grows by ceil(A) as before. Pinning (D-306)
fades D the same way for every word. Offset [0, 0] and the old words are D-127's, D-306's,
D-328's, D-410's and D-411's rules exactly.

Every case is turbulent_displace_reference's 16 by 10 composition and stripes, in After
Effects' units unless the case says. The projects go into `Fixtures/turbulent_types`, the
expected frames into `Fixtures/turbulent_types/expected_turbulent_types.json`.

**This file never runs the build's code path.** It works in double precision from Fractal
Noise's value noise, imported from `tools/fractal_noise_reference.py`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/turbulent_types_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from fractal_noise_reference import V, F  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
import smooth_reference as S  # noqa: E402
import turbulent_displace_reference as T  # noqa: E402
import turbulent_ae_reference as A  # noqa: E402

W, H = T.W, T.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "turbulent_types"
TOLERANCE = T.TOLERANCE
WORDS = ("turbulent", "horizontal", "vertical", "cross", "bulge", "twist", "turbulent_smoother",
         "bulge_smoother", "twist_smoother")
OFFSET = (-100000, 100000)
STRETCH = 8  # cross: how many times longer the streaks are than across
STEP = 0.25  # the Smoother words: each octave a quarter of the last's strength
HALF = 0.25  # bulge and twist: the field's slope read a quarter of a cell either side


# --- the rule -------------------------------------------------------------------------------

def Fs(seed, ch, x, y, z, n, step):
    """F with each octave `step` times the last's strength; step 0.5 is F exactly."""
    total = weight = 0.0
    for o in range(n):
        s = 2 ** o
        total += step ** o * V(seed, 8 * o + ch, x * s, y * s, z * s)
        weight += step ** o
    return total / weight


def clamp(v):
    return min(1.0, max(-1.0, v))


def push(word, amount, complexity, seed, p):
    """D for the field's point p = (x, y, z)."""
    x, y, z = p
    step = STEP if word.endswith("_smoother") else 0.5
    n = lambda ch, a, b: Fs(seed, ch, a, b, z, complexity, step)  # noqa: E731
    kind = word.removesuffix("_smoother")
    if kind == "turbulent":
        return amount * n(0, x, y), amount * n(1, x, y)
    if kind == "horizontal":
        return amount * F(seed, 0, x, y, z, complexity), 0.0
    if kind == "vertical":
        return 0.0, amount * F(seed, 1, x, y, z, complexity)
    if kind == "cross":
        return amount * n(0, x / STRETCH, y), amount * n(1, x, y / STRETCH)
    gx = clamp(n(0, x + HALF, y) - n(0, x - HALF, y))
    gy = clamp(n(0, x, y + HALF) - n(0, x, y - HALF))
    return (amount * gx, amount * gy) if kind == "bulge" else (-amount * gy, amount * gx)


def offset_at(c, frame_no):
    if c["offset"] is None:
        return (0.0, 0.0)
    return tuple(min(OFFSET[1], max(OFFSET[0], v)) for v in value_at(c["offset"], frame_no))


def typed(layer, word, amount, size, complexity, evolution, speed, seed, edges, frame_no, x, y,
          o):
    if amount == 0:
        return layer["px"][y * layer["w"] + x] if 0 <= x < layer["w"] and 0 <= y < layer["h"] \
            else T.EMPTY
    g = T.growth(amount, edges)
    if not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return T.EMPTY
    p = ((x + 0.5 - o[0]) / size, (y + 0.5 - o[1]) / size, (evolution + speed * frame_no) / 360)
    dx, dy = push(word, amount, complexity, seed, p)
    return bilinear(layer, x + 0.5 + dx, y + 0.5 + dy, edges)


def case(displacement="turbulent", offset=None, units="after_effects", **settings):
    """`displacement` or `offset` None: the file does not say."""
    c = A.case(units=units, **{"speed": 0, **settings})
    c.update(displacement=displacement, offset=offset)
    return c


def valid(c):
    return (c["units"] in ("classic", "after_effects", None)
            and c["displacement"] in WORDS + (None,))


def render(c, frame_no):
    if not valid(c):
        return T.plain(c)
    layer = T.drawn_layer(c["drawing"])
    n = [T.held(c, k, frame_no) for k in T.NAMES[:-1]]
    n[0] = A.strength(n[0], n[1], c["units"])
    o = offset_at(c, frame_no)
    word = c["displacement"] or "turbulent"
    return [typed(layer, word, *n, c["edges"], frame_no, x - c["shift"], y, o)
            for y in range(H) for x in range(W)]


WAVE = {"amount": 30, "size": 8}  # FX-TURB-AE-007's wave: 2.4 pixels at most, a wave every 8

CASES = {
    "FX-TURB-TY-001": ("Offset (3, 0), amount 30, size 8, speed 0: the field slides three "
                       "pixels right, so each pixel gets the push the pixel three to its left "
                       "had in FX-TURB-AE-007.",
                       case(offset=(3, 0), **WAVE), [0]),
    "FX-TURB-TY-002": ("Offset (0, 0) written: FX-TURB-AE-007 exactly.",
                       case(offset=(0, 0), **WAVE), [0]),
    "FX-TURB-TY-003": ("Offset keyed from (0, 0) at frame 0 to (8, -4) at frame 4, linear: "
                       "frame 0 is FX-TURB-AE-007, frame 2 offset (4, -2), frame 4 (8, -4), one "
                       "whole wave across.",
                       case(offset=keyed((0, (0, 0)), (4, (8, -4))), **WAVE), [0, 2, 4]),
    "FX-TURB-TY-004": ("Displacement cross: the sideways push from a field stretched eight "
                       "times across, the up-and-down push from one stretched eight times down.",
                       case(displacement="cross", **WAVE), [0]),
    "FX-TURB-TY-005": ("Displacement bulge: each pixel reads from uphill on the field, so its "
                       "hills swell.", case(displacement="bulge", **WAVE), [0]),
    "FX-TURB-TY-006": ("Displacement twist: the bulge's push turned a quarter, carrying the "
                       "pixels round the hills.", case(displacement="twist", **WAVE), [0]),
    "FX-TURB-TY-007": ("Displacement turbulent_smoother, complexity 4: FX-TURB-AE-007's push "
                       "with each finer octave a quarter of the last's strength, not a half.",
                       case(displacement="turbulent_smoother", complexity=4, **WAVE), [0]),
    "FX-TURB-TY-008": ("Displacement bulge_smoother, complexity 4.",
                       case(displacement="bulge_smoother", complexity=4, **WAVE), [0]),
    "FX-TURB-TY-009": ("Displacement twist_smoother, complexity 4.",
                       case(displacement="twist_smoother", complexity=4, **WAVE), [0]),
    "FX-TURB-TY-010": ("Displacement turbulent_smoother, complexity 1: one octave has nothing "
                       "finer to weaken, so this is FX-TURB-AE-007 with complexity 1.",
                       case(displacement="turbulent_smoother", complexity=1, **WAVE), [0]),
    "FX-TURB-TY-011": ("Bulge with edges repeat: nothing grows, and a push past the drawing's "
                       "edge reads the nearest edge pixel.",
                       case(displacement="bulge", edges="repeat", **WAVE), [0]),
    "FX-TURB-TY-012": ("Cross moved three pixels right: worked in the drawing's own space, so "
                       "it moves with it.", case(displacement="cross", shift=3, **WAVE), [0]),
    "FX-TURB-TY-013": ("Twist with offset (2.5, -1.5), seed 8 and speed 90: the offset slides "
                       "the twist's field as it slides the turbulent one, and the field moves "
                       "from frame 0 to 2.",
                       case(displacement="twist", offset=(2.5, -1.5), seed=8, amount=30, size=8,
                            speed=90), [0, 2]),
    "FX-TURB-TY-014": ("Bulge in a file without units, D-127's classic push: amount 3, size 8.",
                       case(displacement="bulge", units=None, amount=3, size=8), [0]),
}

INVALID = {
    "FX-TURB-TY-015": ("Displacement \"Bulge\": the word is exact, so a capital is not it.",
                       case(displacement="Bulge", **WAVE)),
    "FX-TURB-TY-016": ("Displacement \"cross_displacement\", After Effects' name, which is not "
                       "one of the words.", case(displacement="cross_displacement", **WAVE)),
    "FX-TURB-TY-017": ("Offset (100001, 0), above 100000 across.",
                       case(offset=(100001, 0), **WAVE)),
    "FX-TURB-TY-018": ("Offset keyed to (0, -200000) at frame 4, below -100000 down.",
                       case(offset=keyed((0, (0, 0)), (4, (0, -200000))), **WAVE)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = A.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    if c["displacement"] is not None:
        params["displacement"] = c["displacement"]
    if c["offset"] is not None:
        params["offset"] = setting_json(c["offset"])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = T.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = T.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_turbulent_types.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    ae = json.loads((OUT.parent / "turbulent_ae" / "expected_turbulent_ae.json")
                    .read_text(encoding="utf-8"))["cases"]
    still = ae["FX-TURB-AE-007"]["frames"]["0"]
    p = (5.5 / 8, 4.5 / 8, 0.0)

    # The old words are the old rule, the Smoother step 0.5 is F, and every push stays in A.
    assert push("turbulent", 2.4, 2, 0, p) == (2.4 * F(0, 0, *p, 2), 2.4 * F(0, 1, *p, 2))
    assert Fs(3, 1, *p, 5, 0.5) == F(3, 1, *p, 5)
    assert Fs(3, 1, *p, 1, STEP) == F(3, 1, *p, 1)
    for word in WORDS:
        for x in range(-4, 20):
            for y in range(-4, 14):
                q = ((x + 0.5) / 8, (y + 0.5) / 8, 0.3)
                assert all(abs(v) <= 2.4 for v in push(word, 2.4, 4, 7, q)), word
    # Twist is bulge turned a quarter.
    bx, by = push("bulge", 1, 2, 0, p)
    tx, ty = push("twist", 1, 2, 0, p)
    assert (tx, ty) == (-by, bx) and (bx, by) != (0, 0)
    # The offset is a slide of the field: whole-pixel offsets give the very same points.
    assert all(abs(((x + 0.5 - 3) / 8) - ((x - 3 + 0.5) / 8)) < 1e-15 for x in range(W))

    assert c["FX-TURB-TY-002"]["0"] == still
    one = c["FX-TURB-TY-001"]["0"]
    assert one != still
    three = c["FX-TURB-TY-003"]
    assert three["0"] == still and three["2"] == render(case(offset=(4, -2), **WAVE), 0)
    assert three["4"] == render(case(offset=(8, -4), **WAVE), 0) != three["2"]
    looks = {fx: c[fx]["0"] for fx in ("FX-TURB-TY-004", "FX-TURB-TY-005", "FX-TURB-TY-006",
                                       "FX-TURB-TY-007", "FX-TURB-TY-008", "FX-TURB-TY-009")}
    assert len({json.dumps(v) for v in looks.values()}) == 6 and still not in looks.values()
    assert render(case(complexity=4, **WAVE), 0) != looks["FX-TURB-TY-007"]
    assert c["FX-TURB-TY-010"]["0"] == render(case(complexity=1, **WAVE), 0)
    assert c["FX-TURB-TY-011"]["0"] != looks["FX-TURB-TY-005"]
    twelve = c["FX-TURB-TY-012"]["0"]
    at = lambda x, y: y * W + x  # noqa: E731
    assert all(twelve[at(x, y)] == looks["FX-TURB-TY-004"][at(x - 3, y)]
               for x in range(3, W) for y in range(H))
    assert c["FX-TURB-TY-013"]["0"] != c["FX-TURB-TY-013"]["2"]
    assert c["FX-TURB-TY-014"]["0"] != looks["FX-TURB-TY-005"]
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == T.plain(case())
    for name, frames in c.items():
        for px in frames.values():
            for q in px:
                assert -1e-12 <= q[3] <= 1 + 1e-12 and all(-1e-12 <= v <= q[3] + 1e-12
                                                          for v in q[:3]), (name, q)
    print("checked")


if __name__ == "__main__":
    main()
