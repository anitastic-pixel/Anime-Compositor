"""Lens chromatic aberration, worked a second way.

D-409 extends `core.chromatic_aberration` (D-120) as PLUGINS.md's pick #3, "Lens Chromatic
Aberration", and folds in its RGB Separation row as a second mode. A file whose effect carries
`mode` is the new form; a file without it (every file written before D-409) is D-120's form and
draws by D-120's rule unchanged, which `tools/chromatic_aberration_reference.py` and
FX-CHROMA-001 to 016 keep pinning (D-383's and D-401's way).

The new form's settings:

- `mode`: "radial" (when added) or "offset". Not keyable.
- `amount`: 0 to 100 (3), pixels: how far a channel at scale 100 moves, at the drawing's corner
  (radial, half its diagonal from the centre) or everywhere (offset).
- `center`: per cent of the drawing's width and height ([50, 50]), radial only.
- `angle`: -3600 to 3600 degrees (90), offset only: the direction a channel at a positive scale
  moves, 0 up, 90 right (the shared `along`).
- `falloff`: 0 to 100 (0), radial only: 0 grows the shift in step with the distance from the
  centre, as D-120 does; 100 grows it with the distance cubed (Brown's first radial term), weak
  in the middle and strong at the edges. The corner's shift is the amount whatever the falloff.
- `red_scale`, `green_scale`, `blue_scale`: -200 to 200 per cent (100, 0, -100): each channel's
  share of the amount; positive moves it outward (radial) or along the angle (offset). The
  starting values are D-120's split: red outward, blue inward, green kept.
- `fringe_blur`: 0 to 100 (0), per cent of each channel's own shift: the channel is smeared
  along its shift over that length, centred on its shifted place.

The numbers are keyable.

The rule. With the drawing W0 by H0, c = (center_x / 100 W0, center_y / 100 H0), Rc =
sqrt(W0^2 + H0^2) / 2, k = amount / Rc, b = fringe_blur / 100 and, for each channel, s = its
scale / 100 and n = 1 when b s is 0, else ceil(amount |s| b) + 1, with t_j = j / (n - 1) - 1/2
for j = 0 .. n - 1 (t_0 = 0 when n is 1). At the pixel whose centre is P:

- radial: d = P - c, rho = |d| / Rc, g = rho^(2 falloff / 100) (1 when the power is 0, 0 at
  rho 0 otherwise); the channel's samples are at c + d (1 - s k g + t_j b |s| k g);
- offset: u = along(angle); the samples are at P + u amount (t_j b |s| - s).

Each sample is document 21's bilinear sample of the layer (premultiplied, transparent outside).
The channel's own number is the mean of its samples' numbers for that channel, summed in order
j = 0 .. n - 1 and divided once, and the channel's covering is the mean of the samples'
coverings likewise. The pixel is (red's red, green's green, blue's blue) and its covering is the
largest of the three channels' coverings. The layer does not grow. At the starting values
(radial, falloff 0, scales 100, 0, -100, no blur) this is D-120's picture: the green channel's
one sample is at P itself, which is the pixel's own.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Chromatic Aberration's figure. The drawing goes into
`Fixtures/lens_chromatic_aberration/media`, the projects into `Fixtures/lens_chromatic_aberration`,
and the expected frames into
`Fixtures/lens_chromatic_aberration/expected_lens_chromatic_aberration.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lens_chromatic_aberration_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from bloom_reference import along  # noqa: E402
import chromatic_aberration_reference as CA  # noqa: E402
from adjust_reference import prop  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lens_chromatic_aberration"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 100), "angle": (-3600, 3600), "falloff": (0, 100),
          "red_scale": (-200, 200), "green_scale": (-200, 200), "blue_scale": (-200, 200),
          "fringe_blur": (0, 100)}
CENTER = (-1000, 1000)
NUMBERS = ("amount", "center", "angle", "falloff", "red_scale", "green_scale", "blue_scale",
           "fringe_blur")


# --- the rule -------------------------------------------------------------------------------

def count(amount, s, b):
    """How many samples a channel takes: one with no smear, else one a pixel of its longest."""
    return 1 if b * s == 0 else math.ceil(amount * abs(s) * b) + 1


def lens(layer, v, size=(W, H)):
    """The layer's pixels, split. `v` holds the settings at the frame; `size` is W0 by H0."""
    w0, h0 = size
    cx, cy = v["center"][0] / 100 * w0, v["center"][1] / 100 * h0
    rc = math.sqrt(w0 * w0 + h0 * h0) / 2
    k = v["amount"] / rc
    e = 2 * v["falloff"] / 100
    b = v["fringe_blur"] / 100
    ux, uy = along(v["angle"])
    scales = [v[n] / 100 for n in ("red_scale", "green_scale", "blue_scale")]
    counts = [count(v["amount"], s, b) for s in scales]
    out = []
    for i in range(len(layer["px"])):
        px = layer["left"] + i % layer["w"] + 0.5
        py = layer["top"] + i // layer["w"] + 0.5
        dx, dy = px - cx, py - cy
        if e == 0:
            g = 1.0
        else:
            rho = math.sqrt(dx * dx + dy * dy) / rc
            g = rho ** e if rho > 0 else 0.0
        chans = []
        for s, n in zip(scales, counts):
            total = [0.0] * 4
            for j in range(n):
                t = j / (n - 1) - 0.5 if n > 1 else 0.0
                if v["mode"] == "radial":
                    f = 1 - s * k * g + t * b * abs(s) * k * g
                    q = bilinear(layer, cx + dx * f, cy + dy * f)
                else:
                    m = v["amount"] * (t * b * abs(s) - s)
                    q = bilinear(layer, px + ux * m, py + uy * m)
                total = [a + c for a, c in zip(total, q)]
            chans.append([a / n for a in total])
        out.append([chans[0][0], chans[1][1], chans[2][2],
                    max(chans[0][3], chans[1][3], chans[2][3])])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(mode="radial", amount=3, center=(50, 50), angle=90, falloff=0, red_scale=100,
         green_scale=0, blue_scale=-100, fringe_blur=0, shift=0):
    return {"drawing": "figure", "mode": mode, "amount": amount, "center": center,
            "angle": angle, "falloff": falloff, "red_scale": red_scale,
            "green_scale": green_scale, "blue_scale": blue_scale, "fringe_blur": fringe_blur,
            "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def settings(c, frame_no):
    v = {"mode": c["mode"]}
    for n, r in RANGES.items():
        v[n] = clamp(value_at(c[n], frame_no), r)
    v["center"] = [clamp(x, CENTER) for x in value_at(c["center"], frame_no)]
    return v


def layer_of(c):
    return {"px": [R.working(p) for row in CA.DRAWINGS[c["drawing"]] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def render(c, frame_no):
    return R.frame(lens(layer_of(c), settings(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-LENSCA-001": ("The new form's settings as they start (radial, amount 3, falloff 0, "
                      "red 100, green 0, blue -100, no blur): the same picture as D-120's "
                      "FX-CHROMA-001, red fringing outward, blue inward.",
                      case(), [0]),
    "FX-LENSCA-002": ("Amount 0: the drawing, untouched.",
                      case(amount=0), [0]),
    "FX-LENSCA-003": ("Falloff 100 at amount 6: the shift grows with the distance cubed, so "
                      "the skin block, near the middle, barely fringes, while the white block's "
                      "far corners still split.",
                      case(amount=6, falloff=100), [0]),
    "FX-LENSCA-004": ("Falloff 50 at amount 6: between FX-LENSCA-003 and no falloff.",
                      case(amount=6, falloff=50), [0]),
    "FX-LENSCA-005": ("Scales red 0, green 100, blue 0: only green moves outward; red and blue "
                      "stay, so magenta fringes inside the blocks' outer edges and green ones "
                      "outside.",
                      case(red_scale=0, green_scale=100, blue_scale=0), [0]),
    "FX-LENSCA-006": ("Red 200, blue -50: red twice as far out as at the start, blue half as "
                      "far in.",
                      case(red_scale=200, blue_scale=-50), [0]),
    "FX-LENSCA-007": ("Fringe blur 50 at amount 6: each fringe smeared along its own shift "
                      "over half its length, softer than FX-CHROMA-003's.",
                      case(amount=6, fringe_blur=50), [0]),
    "FX-LENSCA-008": ("Fringe blur 100 at amount 6 with falloff 100: the smear over the whole "
                      "shift, strongest at the edges.",
                      case(amount=6, fringe_blur=100, falloff=100), [0]),
    "FX-LENSCA-009": ("Offset mode, amount 2, angle 90: red moved exactly two pixels right, "
                      "blue two left, green kept, the same at every pixel.",
                      case(mode="offset", amount=2), [0]),
    "FX-LENSCA-010": ("Offset mode, amount 1.5, angle 0: red moved a pixel and a half up, "
                      "blue as far down, each a blend of two rows.",
                      case(mode="offset", amount=1.5, angle=0), [0]),
    "FX-LENSCA-011": ("Offset mode, amount 3, angle 45, fringe blur 100: red and blue smeared "
                      "along the diagonal over their whole shift.",
                      case(mode="offset", amount=3, angle=45, fringe_blur=100), [0]),
    "FX-LENSCA-012": ("Offset mode, angle keyed from 0 at frame 0 to 360 at frame 4, linear: "
                      "frames 0 and 4 the same, frame 2 the split turned over (red down).",
                      case(mode="offset", amount=2, angle=keyed((0, 0), (4, 360))), [0, 2, 4]),
    "FX-LENSCA-013": ("Falloff keyed from 0 at frame 0 to 100 at frame 4 at amount 6: frame 0 "
                      "D-120's split at 6, frame 4 FX-LENSCA-003.",
                      case(amount=6, falloff=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-LENSCA-014": ("FX-LENSCA-009 moved three pixels right: the same, moved; the three "
                      "columns left of the drawing stay empty, as the layer does not grow.",
                      case(mode="offset", amount=2, shift=3), [0, 3]),
    "FX-LENSCA-015": ("Radial about 0, 50 with falloff 100 at amount 6: weak by the left "
                      "edge's middle, strong at the right edge.",
                      case(amount=6, center=(0, 50), falloff=100), [0]),
    "FX-LENSCA-016": ("Offset mode with centre 0, 0 and falloff 100: the centre and the "
                      "falloff are radial's alone, so the picture is FX-LENSCA-009's.",
                      case(mode="offset", amount=2, center=(0, 0), falloff=100), [0]),
    "FX-LENSCA-017": ("Radial mode with angle 0: the angle is offset's alone, so the picture "
                      "is FX-LENSCA-001's.",
                      case(angle=0), [0]),
    "FX-LENSCA-018": ("Green 100, red and blue 100 too, offset mode amount 1, angle 270: the "
                      "whole drawing moved a pixel left, as one.",
                      case(mode="offset", amount=1, angle=270, green_scale=100,
                           blue_scale=100), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LENSCA-019": ("Mode \"spiral\", not a mode.", case(mode="spiral")),
    "FX-LENSCA-020": ("Falloff 101, above 100.", case(falloff=101)),
    "FX-LENSCA-021": ("Red scale 201, above 200.", case(red_scale=201)),
    "FX-LENSCA-022": ("Blue scale -201, below -200.", case(blue_scale=-201)),
    "FX-LENSCA-023": ("Fringe blur -1, below 0.", case(fringe_blur=-1)),
    "FX-LENSCA-024": ("Angle 3601, past ten turns.", case(angle=3601)),
    "FX-LENSCA-025": ("Amount 101 in the new form, above 100.", case(amount=101)),
    "FX-LENSCA-026": ("Green scale keyed to 300 at frame 4.",
                      case(green_scale=keyed((0, 0), (4, 300)))),
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
    params = {"mode": c["mode"]}
    params.update({k: setting_json(c[k]) for k in NUMBERS})
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.chromatic_aberration", "enabled": True,
        "parameters": params}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in CA.DRAWINGS.items():
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

    (OUT / "expected_lens_chromatic_aberration.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, eps=1e-12: all(abs(p - q) < eps for u, v in zip(a, b)  # noqa: E731
                                       for p, q in zip(u, v))
    spread = lambda f: sum(abs(f[i][0] - drawn[i][0]) + abs(f[i][2] - drawn[i][2])  # noqa: E731
                           for i in range(W * H))
    layer = layer_of(case())

    for fx, frames in c.items():
        for px in frames.values():
            for i, p in enumerate(px):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (fx, i)
    # The starting values are D-120's picture, at 3 and at 6, and at any centre.
    old = lambda amount, center=(50, 50): R.frame(  # noqa: E731
        CA.aberration(layer, amount, center), 0)
    assert near(c["FX-LENSCA-001"]["0"], old(3))
    assert near(c["FX-LENSCA-013"]["0"], old(6))
    assert near(render(case(center=(0, 50)), 0), old(3, (0, 50)))
    assert c["FX-LENSCA-002"]["0"] == drawn
    # Falloff: the corner's shift is the amount whatever the falloff; the middle's is less.
    # About the top left corner, the bottom right pixel's centre is nearly the diagonal away
    # (rho near 2), so a falloff makes its shift larger there; next to the centre, smaller.
    rc = math.sqrt(W * W + H * H) / 2
    assert (math.hypot(W - 0.5, H - 0.5) / rc) ** 2 > 1 > (math.hypot(0.5, 0.5) / rc) ** 2
    three, four, six0 = c["FX-LENSCA-003"]["0"], c["FX-LENSCA-004"]["0"], old(6)
    skin = [at(x, y) for x in range(8, 15) for y in range(2, 8)]
    near_skin = lambda f: sum(abs(f[i][0] - drawn[i][0]) + abs(f[i][2] - drawn[i][2])  # noqa
                              for i in skin)
    assert near_skin(three) < near_skin(four) < near_skin(six0)
    assert near_skin(three) > 0
    # Per-channel scales: green alone moves; red and blue are the pixel's own.
    five = c["FX-LENSCA-005"]["0"]
    assert all(five[i][0] == drawn[i][0] and five[i][2] == drawn[i][2] for i in range(W * H))
    assert any(five[i][1] != drawn[i][1] for i in range(W * H))
    assert drawn[at(2, 1)][3] == 0 and five[at(2, 1)][1] > 0 and five[at(2, 1)][0] == 0
    six = c["FX-LENSCA-006"]["0"]
    assert near(six, R.frame(lens(layer, settings(case(red_scale=200, blue_scale=-50), 0)), 0))
    assert spread(six) != spread(c["FX-LENSCA-001"]["0"])
    # Fringe blur: softer than the sharp split, sums the same light along the line.
    seven = c["FX-LENSCA-007"]["0"]
    assert seven != six0 and count(6, 1, 0.5) == 4 and count(6, 0, 0.5) == 1
    assert all(seven[i][1] == drawn[i][1] for i in range(W * H))  # green unscaled, unsmeared
    eight = c["FX-LENSCA-008"]["0"]
    assert eight != three and count(6, 1, 1) == 7
    # Offset: red exactly two pixels right, blue two left, green kept.
    nine = c["FX-LENSCA-009"]["0"]
    for y in range(H):
        for x in range(W):
            p = nine[at(x, y)]
            red = drawn[at(x - 2, y)][0] if x >= 2 else 0.0
            blue = drawn[at(x + 2, y)][2] if x + 2 < W else 0.0
            assert p[0] == red and p[2] == blue and p[1] == drawn[at(x, y)][1], (x, y)
    ten = c["FX-LENSCA-010"]["0"]
    # Row 0's red is sampled a pixel and a half below its centre, between rows 1 and 2: half of
    # the white block's red; row 1's, between rows 2 and 3, all of it.
    assert abs(ten[at(2, 0)][0] - drawn[at(2, 2)][0] / 2) < 1e-12
    assert abs(ten[at(2, 1)][0] - drawn[at(2, 2)][0]) < 1e-12
    eleven = c["FX-LENSCA-011"]["0"]
    assert eleven != render(case(mode="offset", amount=3, angle=45), 0)
    twelve = c["FX-LENSCA-012"]
    assert near(twelve["0"], twelve["4"])
    assert near(twelve["2"], render(case(mode="offset", amount=2, angle=180), 0))
    assert near(twelve["0"], render(case(mode="offset", amount=2, angle=0), 0))
    thirteen = c["FX-LENSCA-013"]
    assert near(thirteen["4"], three)
    assert near(thirteen["2"], four)
    moved = c["FX-LENSCA-014"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == nine[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    fifteen = c["FX-LENSCA-015"]["0"]
    by_centre = [at(x, y) for x in range(0, 7) for y in range(2, 8)]
    edge = [at(x, y) for x in range(9, 15) for y in range(3, 7)]
    moved_by = lambda f, ids: sum(abs(f[i][0] - drawn[i][0]) + abs(f[i][2] - drawn[i][2])  # noqa
                                  for i in ids)
    flat = old(6, (0, 50))
    assert moved_by(fifteen, by_centre) < moved_by(flat, by_centre)
    assert moved_by(fifteen, edge) > 0
    assert c["FX-LENSCA-016"]["0"] == nine
    assert c["FX-LENSCA-017"]["0"] == c["FX-LENSCA-001"]["0"]
    eighteen = c["FX-LENSCA-018"]["0"]
    for y in range(H):
        for x in range(W):
            want = drawn[at(x + 1, y)] if x + 1 < W else [0.0] * 4
            assert eighteen[at(x, y)] == want, (x, y)
    print("checked")


if __name__ == "__main__":
    main()
