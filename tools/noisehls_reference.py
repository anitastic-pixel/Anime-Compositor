"""Noise HLS, worked a second way.

D-451 adds `core.noise_hls`, After Effects' Noise HLS (Noise & Grain): it "adds noise to the hue,
lightness, and saturation components of an image" (Adobe's After Effects user guide, Noise HLS
and Noise HLS Auto). Its settings are After Effects': Noise (Uniform, Squared, Grain), Hue,
Lightness, Saturation, Grain Size (for Grain only) and Noise Phase. Adobe describes each control
in words and publishes no formula, and its starting values were not to be read; the numbers below
are this program's own rule, built on D-113's HSL (`grade::to_hsl`, `from_hsl`) and P0-19's
value noise (`grade::value`, D-127, D-128, D-299). Nothing is ported. D-452's Noise HLS Auto
(`tools/noisehlsauto_reference.py`) uses this rule with the phase set by the frame.

The rule, at a pixel (x, y) of the drawing's own pixels (its top-left corner (0, 0), however far
an effect above grew the buffer), straight colour e through the sRGB curve:

1. The depth z is the Noise Phase in turns (phase / 360), so a full turn is a whole new noise,
   reached smoothly.
2. Three noises in -1..1, n_k for k = 0 (hue), 1 (lightness) and 2 (saturation), each P0-19's
   value noise for seed 0 and channel k. Uniform: in blocks of one pixel at (x, y, z), each pixel
   its own number. Squared: Uniform's number pushed out towards -1 and 1,
   sign(n) (1 - (1 - |n|)^2), higher in contrast. Grain: smooth, in cells of Grain Size pixels,
   at ((x + 0.5) / size, (y + 0.5) / size, z), so neighbouring pixels move together like a film's
   grain (a draft halves the size, so the grain keeps its size on the picture).
3. e as hue h (degrees), saturation s and lightness l (D-113's HSL). h becomes
   h + 180 n_0 hue / 100, round the wheel; l becomes l + n_1 lightness / 100 and s becomes
   s + n_2 saturation / 100, each held in 0 and 1. Back to a colour, held in 0 and 1, back
   through the curve, times the covering.

A pixel that does not show is left; the covering is kept. Hue, Lightness and Saturation all 0
change nothing. A grey has no hue, so Hue alone leaves it; with Saturation it takes red's (hue 0)
side of the wheel first, as D-113 reads a grey.

Settings: `noise` `uniform` (when added), `squared` or `grain`; `hue` 0 to 100 per cent, 0;
`lightness` 0 to 100 per cent, 10; `saturation` 0 to 100 per cent, 0; `grain_size` 0.5 to 100
pixels, 1; `noise_phase` -100000 to 100000 degrees, 0. Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Hue/Saturation's bands (`tools/hue_saturation_reference.py`:
red, green, blue, grey, skin, magenta, line and a soft edge), the same size, unmoved unless the
case says. The expected frames are in `Fixtures/noisehls/expected_noisehls.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/noisehls_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import u  # noqa: E402
from hue_saturation_reference import DRAWINGS, to_hsl, from_hsl  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "noisehls"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"hue": (0, 100), "lightness": (0, 100), "saturation": (0, 100),
          "grain_size": (0.5, 100), "noise_phase": (-100000, 100000)}
WORDS = ("noise",)
NAMES = ("noise", "hue", "lightness", "saturation", "grain_size", "noise_phase")


# --- the rule -------------------------------------------------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def value(ch, x, y, z, block):
    """grade::value for seed 0: the eight corners of the cell round (x, y, z), mixed by the
    smoothed place inside it; with `block` the cell is one number across and down. A corner that
    weighs 0 adds nothing and is skipped."""
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    s = [0.0, 0.0, fade(z - k)] if block else [fade(x - i), fade(y - j), fade(z - k)]
    v = 0.0
    for corner in range(8):
        d = [corner & 1, (corner >> 1) & 1, corner >> 2]
        w = 1.0
        for a in range(3):
            w *= s[a] if d[a] == 1 else 1 - s[a]
        if w != 0:
            v += w * u(0, i + d[0], j + d[1], k + d[2], ch)
    return v


def squared(n):
    return math.copysign(1 - (1 - abs(n)) ** 2, n)


def noises(kind, x, y, z, size):
    if kind == "grain":
        return [value(k, (x + 0.5) / size, (y + 0.5) / size, z, False) for k in range(3)]
    n = [value(k, x, y, z, True) for k in range(3)]
    return [squared(v) for v in n] if kind == "squared" else n


def shifted(e, n, hue, lightness, saturation):
    h, s, l = to_hsl(*e)
    h = (h + 180 * n[0] * hue / 100) % 360
    l = min(1.0, max(0.0, l + n[1] * lightness / 100))
    s = min(1.0, max(0.0, s + n[2] * saturation / 100))
    return from_hsl(h, s, l)


def noise_hls(layer, kind, hue, lightness, saturation, size, z):
    if hue == 0 and lightness == 0 and saturation == 0:
        return layer
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            a = p[3]
            if a <= 0:
                px.append(p)
                continue
            x, y = layer["left"] + i, layer["top"] + j
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in p[:3]]
            o = shifted(e, noises(kind, x, y, z, size), hue, lightness, saturation)
            px.append([srgb_to_linear(min(1.0, max(0.0, v))) * a for v in o] + [a])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(noise="uniform", hue=0, lightness=10, saturation=0, grain_size=1, noise_phase=0,
         shift=0, tile=False):
    c = dict(locals())
    c["drawing"] = "bands"
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
    out = noise_hls(layer_of(c), c["noise"], n["hue"], n["lightness"], n["saturation"],
                    n["grain_size"], n["noise_phase"] / 360)
    return frame(out, c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-NOISEHLS-001": ("The settings as they start: Uniform, Lightness 10: each shown pixel's "
                        "lightness moves up to 0.1 either way, its own amount, keeping its hue and "
                        "saturation and its covering; the empty pixels are left; the same on every "
                        "frame.", case(), [0, 2]),
    "FX-NOISEHLS-002": ("Hue 50, Lightness 0: each coloured pixel's hue turns up to 90 degrees "
                        "either way, its lightness and saturation kept; the grey has no hue and is "
                        "left.", case(hue=50, lightness=0), [0]),
    "FX-NOISEHLS-003": ("Saturation 50, Lightness 0: the saturation moves up to 0.5 either way, "
                        "held in 0 and 1, so the full primaries only lose saturation and the grey "
                        "only gains it.", case(saturation=50, lightness=0), [0]),
    "FX-NOISEHLS-004": ("Lightness 100: the lightness moves up to 1 either way, held, so some pixels "
                        "turn white or black.", case(lightness=100), [0]),
    "FX-NOISEHLS-005": ("Hue, Lightness and Saturation 30 together, each with its own noise.",
                        case(hue=30, lightness=30, saturation=30), [0]),
    "FX-NOISEHLS-006": ("Squared, Lightness 30: the noise pushed out towards its ends, so every "
                        "pixel's lightness moves at least as far as in Uniform, the same way.",
                        case(noise="squared", lightness=30), [0]),
    "FX-NOISEHLS-007": ("Grain, Grain Size 1, Lightness 30: smooth noise in cells of a pixel.",
                        case(noise="grain", lightness=30), [0]),
    "FX-NOISEHLS-008": ("Grain, Grain Size 4, Lightness 30: the cells four pixels across, so "
                        "neighbouring pixels move nearly together.",
                        case(noise="grain", grain_size=4, lightness=30), [0]),
    "FX-NOISEHLS-009": ("Grain, Grain Size 2.5, Hue 40, Saturation 40, Lightness 0.",
                        case(noise="grain", grain_size=2.5, hue=40, saturation=40, lightness=0),
                        [0]),
    "FX-NOISEHLS-010": ("Uniform at phase 180, Lightness 30: half way between the noise at depth 0 "
                        "and at depth 1.", case(noise_phase=180, lightness=30), [0]),
    "FX-NOISEHLS-011": ("Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Lightness 30: a new "
                        "noise each turn, reached smoothly; frame 1 is phase 180, FX-NOISEHLS-010's.",
                        case(noise_phase=keyed((0, 0), (4, 720)), lightness=30), [0, 1, 2, 4]),
    "FX-NOISEHLS-012": ("Phase -90, Hue 100: the phase runs below 0 too, a quarter of the way back "
                        "from depth 0 to depth -1.", case(noise_phase=-90, hue=100, lightness=0),
                        [0]),
    "FX-NOISEHLS-013": ("Hue, Lightness and Saturation 0: the drawing, untouched.",
                        case(lightness=0), [0, 2]),
    "FX-NOISEHLS-014": ("FX-NOISEHLS-005 moved three pixels right: the noise is the drawing's own, "
                        "so it moves with it.", case(hue=30, lightness=30, saturation=30, shift=3),
                        [0]),
    "FX-NOISEHLS-015": ("After a Motion Tile that grows the layer: the noise is worked in the "
                        "drawing's own pixels, so the frame is FX-NOISEHLS-005's.",
                        case(hue=30, lightness=30, saturation=30, tile=True), [0]),
    "FX-NOISEHLS-016": ("Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the "
                        "drawing.", case(lightness=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-NOISEHLS-017": ("Grain, Grain Size 0.5, Lightness 30: cells of half a pixel.",
                        case(noise="grain", grain_size=0.5, lightness=30), [0]),
    "FX-NOISEHLS-018": ("Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, phase keyed 0 to "
                        "500: the controls together.",
                        case(noise="grain", grain_size=3, hue=20, lightness=25, saturation=60,
                             noise_phase=keyed((0, 0), (4, 500))), [0, 1, 2, 3, 4]),
}

INVALID = {
    "FX-NOISEHLS-019": ("Hue 101, above 100.", case(hue=101)),
    "FX-NOISEHLS-020": ("Lightness -1, below 0.", case(lightness=-1)),
    "FX-NOISEHLS-021": ("Saturation 150, above 100.", case(saturation=150)),
    "FX-NOISEHLS-022": ("Grain Size 0.25, below 0.5.", case(grain_size=0.25)),
    "FX-NOISEHLS-023": ("Noise Phase 200000, above 100000.", case(noise_phase=200000)),
    "FX-NOISEHLS-024": ("Noise \"Uniform\", in capitals, kept as written and not the word.",
                        case(noise="Uniform")),
    "FX-NOISEHLS-025": ("Noise \"grainy\", not one of its three words.", case(noise="grainy")),
    "FX-NOISEHLS-026": ("Lightness keyed to 120 at frame 4, above 100.",
                        case(lightness=keyed((0, 10), (4, 120)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.noise_hls",
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
    (OUT / "expected_noisehls.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g: all(near(f[i], g[i]) for i in range(W * H))  # noqa: E731
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    empty = [i for i in range(W * H) if drawn[i][3] == 0]
    grey = [at(x, y) for x in (7, 8) for y in range(1, 9)]
    red = [at(x, y) for x in (1, 2) for y in range(1, 9)]
    assert len(shown) == 15 * 8 and len(empty) == 40

    def hsl(p):
        a = p[3]
        return to_hsl(*[S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in p[:3]])

    def hue_gap(a, b):
        d = abs(a - b) % 360
        return min(d, 360 - d)

    # The rule's pieces.
    assert value(1, 3, 4, 0.0, True) == u(0, 3, 4, 0, 1)
    assert value(1, 3, 4, 0.5, True) == 0.5 * u(0, 3, 4, 0, 1) + 0.5 * u(0, 3, 4, 1, 1)
    assert value(2, 3.0, 4.0, 0.0, False) == u(0, 3, 4, 0, 2)
    assert squared(0.5) == 0.75 and squared(-0.5) == -0.75
    assert near(shifted([1.0, 0.0, 0.0], [0.5, 0, 0], 100, 0, 0), from_hsl(90, 1, 0.5), 1e-12)

    # 8-bit storage moves HSL a hair, so 1e-6 for "kept".
    for fx in c:
        for px in c[fx].values():
            assert all(px[i] == drawn[i] for i in empty) or fx in ("FX-NOISEHLS-014",)
    one = c["FX-NOISEHLS-001"]
    assert one["0"] == one["2"]
    f = one["0"]
    moved = 0
    for i in shown:
        (h0, s0, l0), (h1, s1, l1) = hsl(drawn[i]), hsl(f[i])
        n = value(1, i % W, i // W, 0, True)
        assert abs(l1 - min(1, max(0, l0 + 0.1 * n))) < 1e-6, (i, l0, l1, n)
        assert f[i][3] == drawn[i][3]
        if 0 < l1 < 1 and s0 > 0:
            assert hue_gap(h0, h1) < 1e-4 and abs(s1 - s0) < 1e-4
        moved += abs(l1 - l0) > 1e-3
    assert moved > len(shown) // 2
    two = c["FX-NOISEHLS-002"]["0"]
    assert all(two[i] == drawn[i] for i in grey)
    turned = [hue_gap(hsl(drawn[i])[0], hsl(two[i])[0]) for i in red]
    assert max(turned) <= 90 + 1e-6 and max(turned) > 30
    three = c["FX-NOISEHLS-003"]["0"]
    assert all(hsl(three[i])[1] <= 1 + 1e-9 for i in shown)
    assert all(hsl(three[i])[1] >= -1e-9 for i in grey) and any(hsl(three[i])[1] > 0.05 for i in grey)
    assert all(hsl(three[i])[1] <= 1 + 1e-6 for i in red)
    four = c["FX-NOISEHLS-004"]["0"]
    assert any(hsl(four[i])[2] > 1 - 1e-9 for i in shown) and any(hsl(four[i])[2] < 1e-9 for i in shown)
    five = c["FX-NOISEHLS-005"]["0"]
    six, uni = c["FX-NOISEHLS-006"]["0"], render(case(lightness=30), 0)
    for i in shown:
        d6, du = hsl(six[i])[2] - hsl(drawn[i])[2], hsl(uni[i])[2] - hsl(drawn[i])[2]
        assert abs(d6) >= abs(du) - 1e-6 and d6 * du >= -1e-12
    seven, eight = c["FX-NOISEHLS-007"]["0"], c["FX-NOISEHLS-008"]["0"]
    assert not same(seven, uni) and not same(eight, seven)
    # In a grey's run of eight, Grain Size 4 moves neighbours nearly together.
    step = lambda f, i: hsl(f[i])[2] - hsl(drawn[i])[2]  # noqa: E731
    jump = lambda f: max(abs(step(f, at(7, y + 1)) - step(f, at(7, y))) for y in range(1, 8))  # noqa: E731
    assert jump(eight) < jump(c["FX-NOISEHLS-001"]["0"]) * 3 and jump(eight) < 0.12
    assert not same(c["FX-NOISEHLS-009"]["0"], drawn)
    ten = c["FX-NOISEHLS-010"]["0"]
    at1 = render(case(noise_phase=360, lightness=30), 0)
    for i in grey:  # the grey's lightness 0.5 never reaches 0 or 1 at Lightness 30
        assert abs(step(ten, i) - (step(uni, i) + step(at1, i)) / 2) < 1e-6
    k11 = c["FX-NOISEHLS-011"]
    assert same(k11["0"], uni) and same(k11["1"], ten) and same(k11["2"], at1)
    assert not same(k11["4"], uni) and not same(at1, uni)
    assert not same(c["FX-NOISEHLS-012"]["0"], drawn)
    assert c["FX-NOISEHLS-013"]["0"] == drawn == c["FX-NOISEHLS-013"]["2"]
    m14 = c["FX-NOISEHLS-014"]["0"]
    assert all(m14[at(x, y)] == five[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert same(c["FX-NOISEHLS-015"]["0"], five)
    k16 = c["FX-NOISEHLS-016"]
    assert k16["0"] == drawn
    for i in grey:
        assert abs(step(k16["4"], i) - 2 * step(k16["2"], i)) < 1e-6
    assert not same(c["FX-NOISEHLS-017"]["0"], drawn)
    k18 = c["FX-NOISEHLS-018"]
    assert not same(k18["0"], k18["4"]) and not same(k18["0"], drawn)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
