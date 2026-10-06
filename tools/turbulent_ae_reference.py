"""D-328's Turbulent Displace in After Effects' units, worked a second way.

From P-26's tutorial 2, where After Effects' Turbulent Displace at Amount 80, Size 2 leaves a
glow smooth with a fine shimmer, and D-127's rule (push = amount pixels at any size) breaks it
into dust. The owner approved D-328 on 2026-10-05.

No source found says how After Effects scales Amount with Size. Adobe's manual
(https://helpx.adobe.com/after-effects/using/distort-effects.html) says only that Amount sets
the distortion and Size the area of it; an Adobe ideas thread
(https://community.adobe.com/t5/after-effects-ideas/change-turbulent-displace-s-quot-amount-quot-and-quot-size-quot-to-quot-amplitude-quot-and-quot/idi-p/14075919)
asks to rename them Amplitude and Wavelength; tutorials
(https://adobevideoworld.com/turbulent-displace-after-effects/,
https://www.premiumbeat.com/blog/how-to-create-heat-waves-in-after-effects/) use small sizes
with large amounts for a fine shimmer. All of that fits a push that shrinks with the wave, so
this program reads it, plainly as its own reading, as:

    push = amount * min(size, 100) / 100 pixels

times the field F, the same F as D-127. At Size 100 and above that is D-127 exactly; at Size 2,
Amount 80 pushes 1.6 pixels at most. With transparent edges the layer grows by ceil(push)
instead of ceil(amount); with repeat it does not grow, as before.

`core.turbulent_displace` gains `units`: "classic", what a file without it means, D-127's rule
exactly; or "after_effects", which a Turbulent Displace added from now on writes.

Every case is turbulent_displace_reference's 16 by 10 composition and stripes. The projects go
into `Fixtures/turbulent_ae`, the expected frames into
`Fixtures/turbulent_ae/expected_turbulent_ae.json`.

**This file never runs the build's code path.** It is turbulent_displace_reference's double
precision rule, given the push in place of the amount.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/turbulent_ae_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
import smooth_reference as S  # noqa: E402
import turbulent_displace_reference as T  # noqa: E402

W, H = T.W, T.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "turbulent_ae"
TOLERANCE = T.TOLERANCE


def strength(amount, size, units):
    """D-328: the push in pixels, before the field F."""
    return amount * min(size, 100) / 100 if units == "after_effects" else amount


def case(units="after_effects", **settings):
    """`units` None: the file does not say."""
    c = T.case(**settings)
    c["units"] = units
    return c


def render(c, frame_no):
    if c["units"] not in ("classic", "after_effects", None):
        return T.plain(c)
    layer = T.drawn_layer(c["drawing"])
    n = [T.held(c, k, frame_no) for k in T.NAMES[:-1]]
    n[0] = strength(n[0], n[1], c["units"])
    return [T.displaced(layer, *n, c["edges"], frame_no, x - c["shift"], y)
            for y in range(H) for x in range(W)]


TUTORIAL = {"amount": 80, "size": 2, "speed": 0}  # tutorial 2's setting, held still
WAVE = {"amount": 30, "size": 8, "speed": 0}  # 2.4 pixels at most, a wave every eight

CASES = {
    "FX-TURB-AE-001": ("Units after_effects, amount 80, size 2, speed 0, tutorial 2's setting: "
                       "the push is 80 x 2 / 100 = 1.6 pixels at most, so every pixel reads "
                       "from within two pixels of itself and the stripes stay stripes, "
                       "shimmering.", case(**TUTORIAL), [0]),
    "FX-TURB-AE-002": ("The same settings in a file without units: D-127's rule as before, a "
                       "push of up to 80 pixels at a wave every two, so each pixel reads from "
                       "far off, mostly past the drawing: it breaks into dust, and in this small frame only two "
                       "specks are left.",
                       case(units=None, **TUTORIAL), [0]),
    "FX-TURB-AE-003": ("Units written \"classic\": FX-TURB-AE-002 exactly.",
                       case(units="classic", **TUTORIAL), [0]),
    "FX-TURB-AE-004": ("Units after_effects with the settings as they start, amount 10, size "
                       "60, speed 20: the push is 6 pixels at most, D-127's amount 6, size 60, "
                       "on every frame.", case(), [0, 2, 4]),
    "FX-TURB-AE-005": ("Units after_effects, amount 3, size 1000, speed 0: past size 100 the "
                       "push is the amount, so this is FX-TURB-009 exactly.",
                       case(amount=3, size=1000, speed=0), [0]),
    "FX-TURB-AE-006": ("Units after_effects, amount 3, size 100, speed 0: at size 100 the push "
                       "is the amount, D-127's amount 3, size 100.",
                       case(amount=3, size=100, speed=0), [0]),
    "FX-TURB-AE-007": ("Units after_effects, amount 30, size 8, speed 0: a push of 2.4 pixels "
                       "at most, D-127's amount 2.4 at size 8; the layer grows by 3.",
                       case(**WAVE), [0]),
    "FX-TURB-AE-008": ("Units after_effects, size 8, speed 0, amount keyed from 0 at frame 0 "
                       "to 40 at frame 4, linear: frame 0 is the drawing, frame 2 amount 20 "
                       "(a push of 1.6), frame 4 amount 40 (3.2).",
                       case(amount=keyed((0, 0), (4, 40)), size=8, speed=0), [0, 2, 4]),
    "FX-TURB-AE-009": ("FX-TURB-AE-001 with edges repeat: nothing grows, and a push past the "
                       "drawing's edge reads the nearest edge pixel.",
                       case(edges="repeat", **TUTORIAL), [0]),
    "FX-TURB-AE-010": ("FX-TURB-AE-007 with seed 8, moved three pixels right: the layer grows "
                       "by ceil(2.4) = 3, so the three columns left of the drawing show the "
                       "stripes pushed into them.",
                       case(seed=8, shift=3, **WAVE), [0]),
}

INVALID = {
    "FX-TURB-AE-011": ("Units \"After_Effects\": the word is exact, so capitals are not it.",
                       case(units="After_Effects", **TUTORIAL)),
    "FX-TURB-AE-012": ("Units \"ae\", which is not one.", case(units="ae", **TUTORIAL)),
}


def project_json(fx, c):
    p = T.project_json(fx, c)
    if c["units"] is not None:
        p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["units"] = c["units"]
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
    (OUT / "expected_turbulent_ae.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def reach(c, frame_no=0):
    """The farthest any pixel of the drawing reads from itself, in pixels."""
    n = [T.held(c, k, frame_no) for k in T.NAMES[:-1]]
    n[0] = strength(n[0], n[1], c["units"])
    return max(max(abs(v) for v in T.push(*n, frame_no, x, y))
               for x in range(W) for y in range(H))


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    old = json.loads((OUT.parent / "turbulent_displace" / "expected_turbulent_displace.json")
                     .read_text(encoding="utf-8"))["cases"]
    drawn = T.plain(T.case())

    assert strength(80, 2, "after_effects") == 1.6 and strength(80, 2, "classic") == 80
    assert strength(3, 100, "after_effects") == 3 == strength(3, 1000, "after_effects")
    assert strength(1000, 1000, "after_effects") == 1000  # growth stays within 1000

    # Tutorial 2's setting: within two pixels in after_effects; dust without units.
    one, two = c["FX-TURB-AE-001"]["0"], c["FX-TURB-AE-002"]["0"]
    assert reach(case(**TUTORIAL)) < 1.6 + 1e-12 and reach(case(units=None, **TUTORIAL)) > 40
    assert one == render(T.case(amount=1.6, size=2, speed=0) | {"units": None}, 0)
    shown = lambda px: sum(p[3] > 0.5 for p in px)  # noqa: E731
    assert shown(one) > 0.8 * shown(drawn)  # the stripes stay
    assert shown(two) == 2  # broken into dust, two specks left
    assert c["FX-TURB-AE-003"]["0"] == two
    four = c["FX-TURB-AE-004"]
    for f in (0, 2, 4):
        assert four[str(f)] == render(T.case(amount=6) | {"units": None}, f)
    assert c["FX-TURB-AE-005"]["0"] == old["FX-TURB-009"]["frames"]["0"]
    assert c["FX-TURB-AE-006"]["0"] == render(case(units=None, amount=3, size=100, speed=0), 0)
    seven = c["FX-TURB-AE-007"]["0"]
    assert seven == render(T.case(amount=2.4, size=8, speed=0) | {"units": None}, 0)
    eight = c["FX-TURB-AE-008"]
    assert eight["0"] == drawn
    assert eight["2"] == render(case(amount=20, size=8, speed=0), 0) != eight["4"]
    # Repeat: the transparent warp wherever the sample lies inside the drawing.
    nine = c["FX-TURB-AE-009"]["0"]
    assert nine != one and sum(nine[i] == one[i] for i in range(W * H)) > W * H // 2
    ten = c["FX-TURB-AE-010"]["0"]
    assert any(ten[at(x, y)][3] > 0 for x in range(3) for y in range(H))  # grown, shown
    assert T.growth(strength(30, 8, "after_effects"), "transparent") == 3
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
