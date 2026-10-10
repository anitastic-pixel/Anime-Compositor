"""Noise HLS Auto, worked a second way.

D-452 adds `core.noise_hls_auto`, After Effects' Noise HLS Auto (Noise & Grain): Noise HLS whose
noise "is animated automatically" (Adobe's After Effects user guide, Noise HLS and Noise HLS
Auto). Its settings are After Effects': Noise (Uniform, Squared, Grain), Hue, Lightness,
Saturation, Grain Size (for Grain only) and Noise Animation Speed, in place of Noise HLS's Noise
Phase. Adobe describes each control in words and publishes no formula, and its starting values
were not to be read; the numbers below are this program's own rule. Nothing is ported.

The rule is D-451's Noise HLS (`tools/noisehls_reference.py`, whose functions this file calls),
with the depth z set by the frame instead of by a phase: z = frame animation_speed, the frame
being the composition's, so at speed 1 every frame is a whole new noise, at 0.5 every other frame
is, gliding between, and at 0 the noise holds still (Noise HLS at phase 0), as D-443's Add Grain
reads its Animation Speed. Everything else is Noise HLS's: three noises from P0-19's value noise
under seed 0 laid on D-113's HSL, Uniform, Squared towards its ends, Grain smooth in cells of
Grain Size pixels; the covering kept; Hue, Lightness and Saturation all 0 change nothing.

Settings: `noise` `uniform` (when added), `squared` or `grain`; `hue` 0 to 100 per cent, 0;
`lightness` 0 to 100 per cent, 10; `saturation` 0 to 100 per cent, 0; `grain_size` 0.5 to 100
pixels, 1; `animation_speed` 0 to 10, 1. Every number can be keyed; a keyed speed is read at the
frame and multiplied by it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Hue/Saturation's bands, the same size, unmoved
unless the case says. The expected frames are in `Fixtures/noisehlsauto/expected_noisehlsauto.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/noisehlsauto_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import noisehls_reference as N  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from hue_saturation_reference import DRAWINGS  # noqa: E402

W, H = N.W, N.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "noisehlsauto"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"hue": (0, 100), "lightness": (0, 100), "saturation": (0, 100),
          "grain_size": (0.5, 100), "animation_speed": (0, 10)}
WORDS = ("noise",)
NAMES = ("noise", "hue", "lightness", "saturation", "grain_size", "animation_speed")


# --- the cases ------------------------------------------------------------------------------

def case(noise="uniform", hue=0, lightness=10, saturation=0, grain_size=1, animation_speed=1,
         shift=0, tile=False):
    c = dict(locals())
    c["drawing"] = "bands"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    out = N.noise_hls(N.layer_of(c), c["noise"], n["hue"], n["lightness"], n["saturation"],
                      n["grain_size"], frame_no * n["animation_speed"])
    return frame(out, c["shift"])


def plain(c):
    return frame(N.layer_of(c), c["shift"])


CASES = {
    "FX-NOISEHLSAUTO-001": ("The settings as they start: Uniform, Lightness 10, speed 1: each shown "
                            "pixel's lightness moves up to 0.1 either way, a whole new noise every "
                            "frame; the covering kept and the empty pixels left.", case(), [0, 1, 2]),
    "FX-NOISEHLSAUTO-002": ("Speed 0: the noise holds still, every frame Noise HLS's at phase 0 "
                            "(FX-NOISEHLS-001).", case(animation_speed=0), [0, 2]),
    "FX-NOISEHLSAUTO-003": ("Speed 0.5, Lightness 30: frame 1 is half way from the first noise to "
                            "the next, Noise HLS's phase 180 (FX-NOISEHLS-010); frame 2 is the next "
                            "noise.", case(animation_speed=0.5, lightness=30), [1, 2]),
    "FX-NOISEHLSAUTO-004": ("Hue 50, Lightness 0: each coloured pixel's hue turns up to 90 degrees "
                            "either way, differently each frame; the grey has no hue and is left.",
                            case(hue=50, lightness=0), [0, 3]),
    "FX-NOISEHLSAUTO-005": ("Saturation 50, Lightness 0, at frame 1.",
                            case(saturation=50, lightness=0), [1]),
    "FX-NOISEHLSAUTO-006": ("Squared, Lightness 30: the noise pushed out towards its ends.",
                            case(noise="squared", lightness=30), [2]),
    "FX-NOISEHLSAUTO-007": ("Grain, Grain Size 4, Lightness 30, speed 0.25: soft cells four pixels "
                            "across, a new grain every four frames, gliding between.",
                            case(noise="grain", grain_size=4, lightness=30, animation_speed=0.25),
                            [0, 1, 4]),
    "FX-NOISEHLSAUTO-008": ("Grain, Grain Size 2.5, Hue 40, Saturation 40, Lightness 0, speed 2: two "
                            "new grains a frame.",
                            case(noise="grain", grain_size=2.5, hue=40, saturation=40, lightness=0,
                                 animation_speed=2), [1, 2]),
    "FX-NOISEHLSAUTO-009": ("Hue, Lightness and Saturation 0: the drawing, untouched, on every frame.",
                            case(lightness=0), [0, 3]),
    "FX-NOISEHLSAUTO-010": ("Hue, Lightness and Saturation 30 moved three pixels right: the noise is "
                            "the drawing's own, so it moves with it.",
                            case(hue=30, lightness=30, saturation=30, shift=3), [2]),
    "FX-NOISEHLSAUTO-011": ("Hue, Lightness and Saturation 30 after a Motion Tile that grows the "
                            "layer: the noise is worked in the drawing's own pixels.",
                            case(hue=30, lightness=30, saturation=30, tile=True), [2]),
    "FX-NOISEHLSAUTO-012": ("Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the "
                            "drawing.", case(lightness=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-NOISEHLSAUTO-013": ("Speed keyed from 0 at frame 0 to 2 at frame 4, Lightness 30: the "
                            "speed at the frame times the frame, so frame 2 is speed 1's frame 2 "
                            "and frame 4 is depth 8.",
                            case(animation_speed=keyed((0, 0), (4, 2)), lightness=30), [0, 2, 4]),
    "FX-NOISEHLSAUTO-014": ("Grain, Grain Size 0.5, Lightness 30: cells of half a pixel.",
                            case(noise="grain", grain_size=0.5, lightness=30), [1]),
    "FX-NOISEHLSAUTO-015": ("Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, speed 0.75: "
                            "the controls together.",
                            case(noise="grain", grain_size=3, hue=20, lightness=25, saturation=60,
                                 animation_speed=0.75), [0, 1, 2, 3, 4]),
}

INVALID = {
    "FX-NOISEHLSAUTO-016": ("Hue 101, above 100.", case(hue=101)),
    "FX-NOISEHLSAUTO-017": ("Lightness -1, below 0.", case(lightness=-1)),
    "FX-NOISEHLSAUTO-018": ("Saturation 150, above 100.", case(saturation=150)),
    "FX-NOISEHLSAUTO-019": ("Grain Size 0.25, below 0.5.", case(grain_size=0.25)),
    "FX-NOISEHLSAUTO-020": ("Speed 11, above 10.", case(animation_speed=11)),
    "FX-NOISEHLSAUTO-021": ("Speed -1, below 0.", case(animation_speed=-1)),
    "FX-NOISEHLSAUTO-022": ("Noise \"Squared\", in capitals, kept as written and not the word.",
                            case(noise="Squared")),
    "FX-NOISEHLSAUTO-023": ("Noise \"film\", not one of its three words.", case(noise="film")),
    "FX-NOISEHLSAUTO-024": ("Saturation keyed to 120 at frame 4, above 100.",
                            case(saturation=keyed((0, 0), (4, 120)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = N.project_json(fx, dict(c, noise_phase=0))
    effect = p["compositions"][0]["layers"][0]["effects"][-1]
    effect["type_id"] = "core.noise_hls_auto"
    effect["parameters"] = {k: (c[k] if k in WORDS else setting_json(c[k])) for k in NAMES}
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
    (OUT / "expected_noisehlsauto.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked, and against D-451's own
    expected frames."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    hls = json.loads((OUT.parent / "noisehls" / "expected_noisehls.json").read_text())["cases"]
    h = {fx: v["frames"] for fx, v in hls.items()}
    drawn = plain(case())
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g: all(near(f[i], g[i]) for i in range(W * H))  # noqa: E731
    empty = [i for i in range(W * H) if drawn[i][3] == 0]
    grey = [y * W + x for x in (7, 8) for y in range(1, 9)]

    for fx in c:
        for px in c[fx].values():
            assert all(px[i] == drawn[i] for i in empty) or fx == "FX-NOISEHLSAUTO-010"
    one = c["FX-NOISEHLSAUTO-001"]
    assert same(one["0"], h["FX-NOISEHLS-001"]["0"])
    assert not same(one["1"], one["0"]) and not same(one["2"], one["1"])
    # Speed 1's frame 1 is Noise HLS a whole turn on.
    assert same(one["1"], N.render(N.case(noise_phase=360), 0))
    two = c["FX-NOISEHLSAUTO-002"]
    assert same(two["0"], h["FX-NOISEHLS-001"]["0"]) and two["0"] == two["2"]
    three = c["FX-NOISEHLSAUTO-003"]
    assert same(three["1"], h["FX-NOISEHLS-010"]["0"])
    assert same(three["2"], h["FX-NOISEHLS-011"]["2"])
    four = c["FX-NOISEHLSAUTO-004"]
    assert all(four["0"][i] == drawn[i] for i in grey) and not same(four["0"], four["3"])
    assert same(four["0"], h["FX-NOISEHLS-002"]["0"])
    assert not same(c["FX-NOISEHLSAUTO-005"]["1"], drawn)
    assert same(c["FX-NOISEHLSAUTO-006"]["2"], N.render(N.case(noise="squared", lightness=30,
                                                                noise_phase=720), 0))
    seven = c["FX-NOISEHLSAUTO-007"]
    assert same(seven["0"], h["FX-NOISEHLS-008"]["0"])
    assert not same(seven["1"], seven["0"]) and not same(seven["4"], seven["0"])
    assert same(seven["4"], N.render(N.case(noise="grain", grain_size=4, lightness=30,
                                            noise_phase=360), 0))
    assert not same(c["FX-NOISEHLSAUTO-008"]["1"], c["FX-NOISEHLSAUTO-008"]["2"])
    nine = c["FX-NOISEHLSAUTO-009"]
    assert nine["0"] == drawn == nine["3"]
    ten, eleven = c["FX-NOISEHLSAUTO-010"]["2"], c["FX-NOISEHLSAUTO-011"]["2"]
    assert all(ten[y * W + x] == eleven[y * W + x - 3] for x in range(3, W) for y in range(H))
    assert same(eleven, N.render(N.case(hue=30, lightness=30, saturation=30, noise_phase=720), 0))
    k12 = c["FX-NOISEHLSAUTO-012"]
    assert k12["0"] == drawn and not same(k12["2"], k12["4"])
    k13 = c["FX-NOISEHLSAUTO-013"]
    assert same(k13["0"], render(case(lightness=30), 0))
    assert same(k13["2"], render(case(lightness=30), 2))
    assert same(k13["4"], N.render(N.case(lightness=30, noise_phase=8 * 360), 0))
    assert not same(c["FX-NOISEHLSAUTO-014"]["1"], drawn)
    k15 = c["FX-NOISEHLSAUTO-015"]
    assert all(not same(k15[str(f)], k15[str(f + 1)]) for f in range(4))
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
