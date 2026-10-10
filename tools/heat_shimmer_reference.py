"""D-411's Heat Shimmer, worked a second way.

PLUGINS.md's pick #17: the wavy air above a hot road or a fire, the background rippling as the
heat rises through it. PLUGINS.md says to merge it into `core.turbulent_displace` "as a drift
(direction and speed) setting, if anything": Turbulent Displace already gives the ripple, and
what a shimmer adds is the ripple travelling one way, usually up.

`core.turbulent_displace` gains two settings, both keyable:

- `drift_direction`, -3600 to 3600 degrees, absent 0: the way the ripple travels, 0 up, 90
  right, clockwise, as Directional Blur's and Lens Chromatic Aberration's angles are.
- `drift_speed`, 0 to 1000 pixels a frame, absent 0, a distance: how fast it travels.

At composition frame f, with the direction a and the speed v each held inside its range at that
frame, the field's point for pixel (X, Y) of the drawing's own space is D-127's point moved back
by the drift d = v f (sin a, -cos a):

    ((X + 0.5 - d_x) / size, (Y + 0.5 - d_y) / size, z)

so the warp the pixel at (X, Y) gets on frame f is the one the pixel at (X - d_x, Y - d_y) got on
frame 0, and the whole ripple slides along d. The push, the edges, the units (D-328) and the new
seed (D-410) are read as before. Speed times frame, as D-127's evolution speed is read: a keyed
speed gives the place it would be at that speed from frame 0. Speed 0, what a file without the
settings means, is D-127's and D-328's rule exactly, whatever the direction.

Every case is turbulent_displace_reference's 16 by 10 composition and stripes, in After
Effects' units (D-328) unless the case says. The projects go into `Fixtures/heat_shimmer`, the
expected frames into `Fixtures/heat_shimmer/expected_heat_shimmer.json`.

**This file never runs the build's code path.** It is turbulent_displace_reference's double
precision rule with its own drifted point, worked here from Fractal Noise's field F.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/heat_shimmer_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from fractal_noise_reference import F  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
import smooth_reference as S  # noqa: E402
import turbulent_displace_reference as T  # noqa: E402
import turbulent_ae_reference as A  # noqa: E402
import line_boil_reference as B  # noqa: E402

W, H = T.W, T.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "heat_shimmer"
TOLERANCE = T.TOLERANCE
RANGES = {"drift_direction": (-3600, 3600), "drift_speed": (0, 1000)}


def setting(c, k, frame_no):
    """The setting held inside its range at the frame; 0 when the file does not say."""
    if c[k] is None:
        return 0
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def drift(c, frame_no):
    a = math.radians(setting(c, "drift_direction", frame_no))
    v = setting(c, "drift_speed", frame_no) * frame_no
    return v * math.sin(a), -v * math.cos(a)


def shimmered(layer, amount, size, complexity, evolution, speed, seed, edges, frame_no, x, y, d):
    """D-127's output at pixel (x, y) of the drawing's own space, the field's point moved by -d."""
    if amount == 0:
        return layer["px"][y * layer["w"] + x] if 0 <= x < layer["w"] and 0 <= y < layer["h"] \
            else T.EMPTY
    g = T.growth(amount, edges)
    if not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return T.EMPTY
    p = ((x + 0.5 - d[0]) / size, (y + 0.5 - d[1]) / size, (evolution + speed * frame_no) / 360)
    dx, dy = amount * F(seed, 0, *p, complexity), amount * F(seed, 1, *p, complexity)
    return bilinear(layer, x + 0.5 + dx, y + 0.5 + dy, edges)


def case(drift_direction=0, drift_speed=1, units="after_effects", new_seed_every=None, **settings):
    """None: the file does not say."""
    c = A.case(units=units, **settings)
    c.update(drift_direction=drift_direction, drift_speed=drift_speed, new_seed_every=new_seed_every)
    return c


def render(c, frame_no):
    if c["units"] not in ("classic", "after_effects", None):
        return T.plain(c)
    layer = T.drawn_layer(c["drawing"])
    n = [T.held(c, k, frame_no) for k in T.NAMES[:-1]]
    n[0] = A.strength(n[0], n[1], c["units"])
    n[5] = B.seed_at(n[5], B.every(c, frame_no), frame_no)
    d = drift(c, frame_no)
    return [shimmered(layer, *n, c["edges"], frame_no, x - c["shift"], y, d)
            for y in range(H) for x in range(W)]


WAVE = {"amount": 30, "size": 8, "speed": 0}  # FX-TURB-AE-007's wave, held still

CASES = {
    "FX-SHIMMER-001": ("Drift up (direction 0) at 1 pixel a frame, amount 30, size 8, speed 0: "
                       "frame 0 is FX-TURB-AE-007; on frame f each pixel gets the warp the pixel "
                       "f rows below it got on frame 0, so the ripple rises a pixel a frame.",
                       case(**WAVE), [0, 1, 2, 3]),
    "FX-SHIMMER-002": ("Drift speed 0 written, direction 45: no drift, every frame "
                       "FX-TURB-AE-007.", case(drift_direction=45, drift_speed=0, **WAVE),
                       [0, 2, 4]),
    "FX-SHIMMER-003": ("Drift right (direction 90) at 2 pixels a frame: the ripple slides two "
                       "columns right each frame.", case(drift_direction=90, drift_speed=2, **WAVE),
                       [0, 1, 2]),
    "FX-SHIMMER-004": ("Drift down (direction 180) at half a pixel a frame: on frame 1 the field "
                       "is read half a row up, between whole rows.",
                       case(drift_direction=180, drift_speed=0.5, **WAVE), [1, 2, 3]),
    "FX-SHIMMER-005": ("Drift at 30 degrees, 3 pixels a frame: up and to the right, 1.5 pixels "
                       "right and about 2.6 up each frame.",
                       case(drift_direction=30, drift_speed=3, **WAVE), [1, 2]),
    "FX-SHIMMER-006": ("Drift up at 1 pixel a frame with speed 20: the ripple rises and changes "
                       "as it goes, the usual heat shimmer.", case(amount=30, size=8, speed=20),
                       [0, 1, 2, 3]),
    "FX-SHIMMER-007": ("Drift speed keyed from 0 at frame 0 to 4 at frame 4, linear, up: frame f "
                       "drifts f times f pixels (frame 2: 2 a frame, 4 pixels; frame 3: 9 "
                       "pixels).", case(drift_speed=keyed((0, 0), (4, 4)), **WAVE), [1, 2, 3, 4]),
    "FX-SHIMMER-008": ("Drift direction keyed from 0 at frame 0 to 90 at frame 4, 2 pixels a "
                       "frame: frame 2 drifts 4 pixels at 45 degrees, frame 4 8 pixels right.",
                       case(drift_direction=keyed((0, 0), (4, 90)), drift_speed=2, **WAVE),
                       [2, 4]),
    "FX-SHIMMER-009": ("Drift up at 1 pixel a frame in a file without units (D-127's classic "
                       "push), amount 3, size 8, speed 0: frame 0 is FX-TURB-004.",
                       case(units=None, amount=3, size=8, speed=0), [0, 2]),
    "FX-SHIMMER-010": ("FX-SHIMMER-001 moved three pixels right: the drift is in the drawing's "
                       "own space, so the picture is the same, moved.", case(shift=3, **WAVE),
                       [0, 2]),
    "FX-SHIMMER-011": ("FX-SHIMMER-001 with edges repeat: a push past the edge reads the nearest "
                       "edge pixel; the ripple drifts as before.", case(edges="repeat", **WAVE),
                       [0, 2]),
    "FX-SHIMMER-012": ("Drift up at 1 pixel a frame with a new seed every 2 frames (D-410): the "
                       "seed steps on frame 2 and the drift goes on.",
                       case(new_seed_every=2, **WAVE), [1, 2, 3]),
}

INVALID = {
    "FX-SHIMMER-013": ("Drift speed -1, below 0.", case(drift_speed=-1, **WAVE)),
    "FX-SHIMMER-014": ("Drift speed 1001, above 1000.", case(drift_speed=1001, **WAVE)),
    "FX-SHIMMER-015": ("Drift direction 3601, above 3600.", case(drift_direction=3601, **WAVE)),
    "FX-SHIMMER-016": ("Drift speed keyed from 1 at frame 0 to 2000 at frame 4.",
                       case(drift_speed=keyed((0, 1), (4, 2000)), **WAVE)),
}


def project_json(fx, c):
    p = A.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    for k in ("drift_direction", "drift_speed", "new_seed_every"):
        if c[k] is not None:
            params[k] = setting_json(c[k])
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
    (OUT / "expected_heat_shimmer.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    ae = json.loads((OUT.parent / "turbulent_ae" / "expected_turbulent_ae.json")
                    .read_text(encoding="utf-8"))["cases"]
    old = json.loads((OUT.parent / "turbulent_displace" / "expected_turbulent_displace.json")
                     .read_text(encoding="utf-8"))["cases"]
    boil = json.loads((OUT.parent / "line_boil" / "expected_line_boil.json")
                      .read_text(encoding="utf-8"))["cases"]
    still = ae["FX-TURB-AE-007"]["frames"]["0"]

    # The drift itself: the field the pixel (X, Y) reads on frame f is the one (X - dx, Y - dy)
    # read on frame 0; with whole-pixel drifts the points are the very same numbers.
    def point(cc, f, x, y):
        d = drift(cc, f)
        return (x + 0.5 - d[0]) / 8, (y + 0.5 - d[1]) / 8
    one = case(**WAVE)
    assert all(point(one, f, 3, 4) == point(one, 0, 3, 4 + f) for f in range(4))
    right = case(drift_direction=90, drift_speed=2, **WAVE)
    assert all(abs(point(right, f, 5, 4)[0] - point(right, 0, 5 - 2 * f, 4)[0]) < 1e-15
               for f in range(3))
    assert abs(drift(case(drift_direction=30, drift_speed=3), 1)[0] - 1.5) < 1e-12
    assert drift(case(drift_speed=keyed((0, 0), (4, 4))), 3) == (0.0, -9.0)
    d8 = drift(case(drift_direction=keyed((0, 0), (4, 90)), drift_speed=2), 2)
    assert abs(d8[0] - 4 * math.sqrt(0.5)) < 1e-12 and abs(d8[1] + 4 * math.sqrt(0.5)) < 1e-12

    s1 = c["FX-SHIMMER-001"]
    assert s1["0"] == still and len({json.dumps(v) for v in s1.values()}) == 4
    assert all(v == still for v in c["FX-SHIMMER-002"].values())
    assert c["FX-SHIMMER-003"]["0"] == still != c["FX-SHIMMER-003"]["1"]
    assert c["FX-SHIMMER-004"]["1"] != still
    assert c["FX-SHIMMER-006"]["0"] == still
    assert c["FX-SHIMMER-006"]["2"] != render(case(drift_speed=0, amount=30, size=8, speed=20), 2)
    s7 = c["FX-SHIMMER-007"]
    assert s7["3"] == render(case(drift_speed=3, **WAVE), 3)  # 3 a frame on frame 3: 9 pixels
    assert c["FX-SHIMMER-008"]["4"] == render(case(drift_direction=90, drift_speed=2, **WAVE), 4)
    assert c["FX-SHIMMER-009"]["0"] == old["FX-TURB-004"]["frames"]["0"]
    assert c["FX-SHIMMER-009"]["2"] != c["FX-SHIMMER-009"]["0"]
    assert c["FX-SHIMMER-010"]["0"] == render(case(drift_speed=0, shift=3, **WAVE), 0)
    assert c["FX-SHIMMER-010"]["2"] != c["FX-SHIMMER-010"]["0"]
    assert c["FX-SHIMMER-011"]["0"] == render(case(drift_speed=0, edges="repeat", **WAVE), 0)
    s12 = c["FX-SHIMMER-012"]
    assert s12["2"] != render(case(**WAVE), 2) and s12["1"] == render(case(**WAVE), 1)
    assert render(case(drift_speed=0, new_seed_every=2, **WAVE), 2) == boil["FX-BOIL-001"]["frames"]["2"]
    print("checked")


if __name__ == "__main__":
    main()
