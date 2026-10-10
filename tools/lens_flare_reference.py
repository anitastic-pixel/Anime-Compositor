"""Lens Flare, worked a second way.

D-426 adds `core.lens_flare`, After Effects' Lens Flare (Generate): the flare a bright light
makes shining into a camera lens, laid over the layer. Adobe names the controls (Flare Center,
Flare Brightness, Lens Type with 50-300mm Zoom, 35mm Prime and 105mm Prime, Blend With Original)
and publishes no formula; the parts it is built from are the ones `docs/effects/PLUGINS.md` (pick
#9, Optical Flares) lists: glows, streaks, rings and ghosts along the flare's axis. The rule and
each lens's parts below are this program's own; nothing is ported, and After Effects' look is
not matched pixel for pixel.

With the holder's input O (linear, premultiplied), W by H its drawing, p a pixel's centre:

1. The flare's place F = (`flare_center_x` / 100 W, `flare_center_y` / 100 H); the layer's
   middle M = (W / 2, H / 2); the size D = sqrt(W^2 + H^2), the drawing's diagonal, so a draft
   (half the pixels) draws the same flare at half the size.
2. The parts. Each lens is a list of parts (kind, t, r, w, n, colour): a part sits at
   P = F + t (M - F), on the line from the flare through the middle (t 0 at the flare, 1 at the
   middle, 2 as far past it); d = |p - P|, R = r D. A **glow** gives exp(-(d / R)^2); a
   **ring** exp(-((d - R) / (w D))^2); a **disc** (a ghost) clamp((R - d) / (w D) + 1/2, 0, 1);
   a **star** exp(-d / R) ((1 + cos(n a)) / 2)^w, a = atan2(p_y - P_y, p_x - P_x) (0 where
   d is 0), n rays. Each times its colour (linear light).
3. The light L = B times the sum of the parts, B = `flare_brightness` / 100.
4. The flare laid on: F = (O.rgb + L, O.a + (1 - O.a) min(1, max(L.r, L.g, L.b))); the result
   O + (1 - `blend_with_original` / 100) (F - O). Brightness 0 or Blend 100 leaves the layer as
   it is. The layer does not grow; the flare is drawn on the layer's own drawing.

`flare_center` -1000 to 1000 per cent each way, 30, 30; `flare_brightness` 0 to 300, 100;
`lens_type` "zoom" (50-300mm Zoom), "35mm" or "105mm", "zoom"; `blend_with_original` 0 to 100, 0.
Every number keyable. The values when added are chosen here; Adobe publishes none.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values, where the build works in single precision on its buffers.

Every case is a project of one composition 16 by 10, eight frames at 24 a second, in
`Fixtures/lens_flare/`: Blobbylize's drawings, the holder at the top with the effect. The
expected pixels are in `Fixtures/lens_flare/expected_lens_flare.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lens_flare_reference.py
"""

import json
import sys
from math import atan2, cos, exp, hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import blobbylize_reference as B  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lens_flare"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 8
assert (B.W, B.H) == (W, H)
RANGES = {"flare_center": (-1000, 1000), "flare_brightness": (0, 300),
          "blend_with_original": (0, 100)}
NAMES = ("flare_center", "flare_brightness", "lens_type", "blend_with_original")
GLOW, RING, DISC, STAR = 0, 1, 2, 3
# Each lens's parts: (kind, t, r, w, n, red, green, blue). The same numbers are in
# src/layer_fx.rs (FLARE_LENSES).
LENSES = {
    "zoom": [
        (GLOW, 0.0, 0.012, 0.0, 0, 2.0, 2.0, 2.0),
        (GLOW, 0.0, 0.06, 0.0, 0, 1.0, 0.75, 0.45),
        (RING, 0.0, 0.2, 0.012, 0, 0.12, 0.18, 0.3),
        (STAR, 0.0, 0.1, 40.0, 6, 0.5, 0.45, 0.4),
        (DISC, 0.45, 0.02, 0.006, 0, 0.1, 0.18, 0.35),
        (DISC, 0.7, 0.045, 0.01, 0, 0.12, 0.3, 0.12),
        (DISC, 1.2, 0.03, 0.008, 0, 0.35, 0.2, 0.08),
        (DISC, 1.5, 0.08, 0.02, 0, 0.08, 0.08, 0.25),
        (DISC, 1.8, 0.015, 0.005, 0, 0.4, 0.1, 0.1),
        (RING, 2.0, 0.1, 0.008, 0, 0.1, 0.25, 0.15),
    ],
    "35mm": [
        (GLOW, 0.0, 0.01, 0.0, 0, 2.0, 2.0, 2.0),
        (GLOW, 0.0, 0.04, 0.0, 0, 1.0, 0.85, 0.65),
        (STAR, 0.0, 0.06, 60.0, 8, 0.4, 0.4, 0.4),
        (DISC, 0.5, 0.025, 0.006, 0, 0.1, 0.22, 0.4),
        (DISC, 1.3, 0.05, 0.012, 0, 0.2, 0.12, 0.35),
        (DISC, 1.7, 0.02, 0.005, 0, 0.4, 0.25, 0.1),
    ],
    "105mm": [
        (GLOW, 0.0, 0.015, 0.0, 0, 2.5, 2.5, 2.5),
        (GLOW, 0.0, 0.07, 0.0, 0, 1.2, 0.85, 0.5),
        (RING, 0.0, 0.11, 0.02, 0, 0.18, 0.1, 0.04),
        (STAR, 0.0, 0.14, 30.0, 12, 0.6, 0.55, 0.45),
        (DISC, 0.6, 0.03, 0.008, 0, 0.08, 0.18, 0.4),
        (DISC, 1.4, 0.06, 0.015, 0, 0.35, 0.15, 0.06),
    ],
}


# --- the rule -------------------------------------------------------------------------------

def held(c, k, frame):
    v = value_at(c[k], frame)
    if k not in RANGES:
        return v
    lo, hi = RANGES[k]
    return [min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple)) else min(hi, max(lo, v))


def part(kind, d, dx, dy, big, w, n):
    """Step 2: one part's strength at distance d (dx, dy from its place), R = big pixels."""
    if kind == GLOW:
        return exp(-(d / big) ** 2)
    if kind == RING:
        return exp(-((d - big) / w) ** 2)
    if kind == DISC:
        return min(1.0, max(0.0, (big - d) / w + 0.5))
    a = atan2(dy, dx) if d > 0 else 0.0
    return exp(-d / big) * ((1 + cos(n * a)) / 2) ** w


def light(w, h, s, x, y):
    """Steps 1 to 3: the flare's light at pixel (x, y)."""
    fx, fy = s["flare_center"][0] / 100 * w, s["flare_center"][1] / 100 * h
    mx, my = w / 2, h / 2
    size = hypot(w, h)
    b = s["flare_brightness"] / 100
    out = [0.0, 0.0, 0.0]
    for kind, t, r, pw, n, *col in LENSES[s["lens_type"]]:
        px, py = fx + t * (mx - fx), fy + t * (my - fy)
        dx, dy = x + 0.5 - px, y + 0.5 - py
        v = part(kind, hypot(dx, dy), dx, dy, r * size, pw if kind == STAR else pw * size, n)
        for i in range(3):
            out[i] += b * col[i] * v
    return out


def lens_flare(o, s):
    w, h = o["w"], o["h"]
    keep = 1 - s["blend_with_original"] / 100
    out = []
    for y in range(h):
        for x in range(w):
            p = o["px"][y * w + x]
            lt = light(w, h, s, x, y)
            f = [p[i] + lt[i] for i in range(3)] + [p[3] + (1 - p[3]) * min(1.0, max(lt))]
            out.append([p[i] + keep * (f[i] - p[i]) for i in range(4)])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

def case(holder="photo", shift=(0, 0), **kw):
    c = {"flare_center": (30, 30), "flare_brightness": 100, "lens_type": "zoom",
         "blend_with_original": 0}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def settings(c, frame):
    return {k: held(c, k, frame) for k in NAMES}


def render(c, frame):
    out = lens_flare(B.decoded(c["holder"]), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = B.decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-FLARE-001": ("As added: the 50-300mm zoom flare at 30, 30 per cent, brightness 100: a "
                     "white-hot core and a warm glow round (4.8, 3), six faint rays, a blue "
                     "halo, and coloured ghosts along the line through the middle.", case(), (0,)),
    "FX-FLARE-002": ("The flare at the middle, 50, 50: every ghost sits on the flare itself, "
                     "so the picture is round about the middle.",
                     case(flare_center=(50, 50)), (0,)),
    "FX-FLARE-003": ("Brightness 0: no light; the photo untouched.",
                     case(flare_brightness=0), (0,)),
    "FX-FLARE-004": ("Brightness 300: three times FX-FLARE-001's light.",
                     case(flare_brightness=300), (0,)),
    "FX-FLARE-005": ("The 35mm prime: a smaller glow, eight rays, three ghosts.",
                     case(lens_type="35mm"), (0,)),
    "FX-FLARE-006": ("The 105mm prime: a bigger, warmer glow, twelve long rays, an orange "
                     "halo, two ghosts.", case(lens_type="105mm"), (0,)),
    "FX-FLARE-007": ("Blend With Original 100: the photo untouched.",
                     case(blend_with_original=100), (0,)),
    "FX-FLARE-008": ("Blend With Original 50: halfway between FX-FLARE-001 and the photo.",
                     case(blend_with_original=50), (0,)),
    "FX-FLARE-009": ("On the shapes drawing, clear between its blocks: the light shows in the "
                     "clear parts too, as covering as it is bright.",
                     case(holder="shapes"), (0,)),
    "FX-FLARE-010": ("FX-FLARE-001 on the holder moved 2 right and 1 down: the same, moved; the "
                     "flare is drawn on the drawing before it moves.", case(shift=(2, 1)), (0,)),
    "FX-FLARE-011": ("The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: "
                     "frame 2 is FX-FLARE-002.",
                     case(flare_center=keyed((0, (0, 0)), (4, (100, 100)))), (0, 2, 4)),
    "FX-FLARE-012": ("Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is.",
                     case(flare_brightness=keyed((0, 0, OVERSHOOT), (4, 300))), (0, 2, 4)),
    "FX-FLARE-013": ("The flare off the layer, at -50, 150: only its ghosts and the tail of its "
                     "glow and rays reach the drawing.", case(flare_center=(-50, 150)), (0,)),
    "FX-FLARE-014": ("The 105mm prime at 70, 20, brightness 150, blend 25, on the ramp: the "
                     "controls together.",
                     case(holder="ramp", lens_type="105mm", flare_center=(70, 20),
                          flare_brightness=150, blend_with_original=25), (0,)),
}

INVALID = {
    "FX-FLARE-015": ("Brightness 301, above 300.", case(flare_brightness=301)),
    "FX-FLARE-016": ("Brightness -1, below 0.", case(flare_brightness=-1)),
    "FX-FLARE-017": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-FLARE-018": ("A lens type written \"200mm\".", case(lens_type="200mm")),
    "FX-FLARE-019": ("A centre 1001, 50, past ten widths.", case(flare_center=(1001, 50))),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.lens_flare", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-" + c["holder"], position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect(c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in ("photo", "shapes", "ramp")],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [holder])]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("photo", "shapes", "ramp"):
        (OUT / "media" / f"{name}.png").write_bytes(png(B.DRAWINGS[name]))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_lens_flare.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    photo, shapes = plain(case()), plain(case(holder="shapes"))
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for u, v in zip(p, q)  # noqa: E731
                                    for a, b in zip(u, v))
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)

    # The parts: a glow is 1 at its place, a disc half at its edge, a ring full on its radius.
    assert part(GLOW, 0, 0, 0, 2, 0, 0) == 1 and part(DISC, 3, 3, 0, 3, 1, 0) == 0.5
    assert part(RING, 4, 4, 0, 4, 1, 0) == 1 and part(STAR, 0, 0, 0, 2, 40, 6) == 1
    # A star is dark between its rays: six rays, the gap at 30 degrees.
    assert part(STAR, 1, cos(0.5236), 0.5, 2, 40, 6) < 1e-12

    one = c["FX-FLARE-001"]["0"]
    # Brighter at the flare than far from it; never darker anywhere.
    assert one[at(4, 3)][0] - photo[at(4, 3)][0] > one[at(15, 9)][0] - photo[at(15, 9)][0]
    assert all(p[i] >= q[i] - 1e-15 for p, q in zip(one, photo) for i in range(4))
    two = c["FX-FLARE-002"]["0"]
    light2 = [[p[i] - q[i] for i in range(3)] for p, q in zip(two, photo)]
    # Round about the middle: the light at (x, y) is the light at (15 - x, 9 - y).
    assert all(abs(light2[at(x, y)][i] - light2[at(W - 1 - x, H - 1 - y)][i]) < 1e-12
               for x in range(W) for y in range(H) for i in range(3))
    assert c["FX-FLARE-003"]["0"] == photo and c["FX-FLARE-007"]["0"] == photo
    four = c["FX-FLARE-004"]["0"]
    assert all(abs((four[i][k] - photo[i][k]) - 3 * (one[i][k] - photo[i][k])) < 1e-12
               for i in range(W * H) for k in range(3))
    five, six = c["FX-FLARE-005"]["0"], c["FX-FLARE-006"]["0"]
    assert five != one and six != one and five != six
    eight = c["FX-FLARE-008"]["0"]
    assert all(abs(eight[i][k] - (one[i][k] + photo[i][k]) / 2) < 1e-12
               for i in range(W * H) for k in range(4))
    nine = c["FX-FLARE-009"]["0"]
    assert any(nine[i][3] > 0 for i in range(W * H) if shapes[i][3] == 0)
    moved = c["FX-FLARE-010"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    k = c["FX-FLARE-011"]
    assert near(k["2"], two) and k["0"] != k["4"]
    k = c["FX-FLARE-012"]
    assert k["0"] == photo and k["2"] == k["4"] == four
    assert c["FX-FLARE-013"]["0"] != photo
    print("checked")


if __name__ == "__main__":
    main()
