"""Soft Physical Glow's Spectral setting, worked a second way.

D-543 is pick #10 in `docs/effects/EFFECTS.md` (row 259), "Spectral Glow": a separate glow size
for each colour channel, so the halo's edge fringes in colour. PLUGINS.md settles that it is a
setting of Soft Physical Glow (D-353, the "physical" falloff of `core.glow`) and not an effect of
its own: section 2.3 reads Deep Glow's Glow Aberration (Multiply Red / Green / Blue, 80% / 100% /
120%) as "item 10 (Spectral Glow) as part of the same engine", and the ranked list says "Merge
into Soft Physical Glow (per-channel radius)". The rule below is our own; no plugin code or
presets are used.

The settings, all new, all left out of a file that does not use them:

* `spectral`: "off" (the default: Soft Physical Glow exactly as before) or "on".
* `spectral_red`, `spectral_green`, `spectral_blue`: 0 to 200 per cent of Radius, defaults 80,
  100 and 120 (Deep Glow's), each may be keyed.

The rule, inside D-353's and D-542's (`tools/glow_tint_reference.py`): with Spectral on, the glow
before Exposure is worked three times, at radius Radius x red / 100, Radius x green / 100 and
Radius x blue / 100, each the whole of D-353's levels and D-542's tint for that radius; the red
channel is taken from the first, the green from the second and the blue from the third. Exposure,
Blend Mode, Unmult (the brightest of the three channels) and Source Opacity then act once, on the
three channels together, as before. So a white light glows white at its core, where all three
sizes give much the same, and fringes red inside and blue outside (with the defaults) where the
sizes part. The layer grows by the largest of the three.

**This file never runs the build's code path.** It works in double precision with numpy, on
D-353's padded planes of cells; the build works in single precision, pixel by pixel.

Every case is a composition 16 pixels by 10 holding one drawing the same size, the drawings of
`tools/soft_glow_reference.py`. The drawings go into `Fixtures/spectral_glow/media`, the projects
into `Fixtures/spectral_glow`, and the expected frames into
`Fixtures/spectral_glow/expected_spectral_glow.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/spectral_glow_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import soft_glow_reference as G  # noqa: E402
import glow_tint_reference as T  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "spectral_glow"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_SCALE = 200


# --- the rule -------------------------------------------------------------------------------

def glow_sum(px, c, xs, ys, radius):
    """D-353's glow before Exposure at `radius`, with D-542's tint (amount 0 when off)."""
    tint = c["tint"] != "off"
    inner, outer = T.linear(c["inner_color"]), T.linear(c["outer_color"])
    a = c["tint_amount"] / 100 if tint else 0.0
    lit = G.light(px, c)
    lvls, own = G.levels(radius)
    gx = np.zeros((len(ys), len(xs), 3))
    X, Y = np.array(xs, float), np.array(ys, float)
    inside = lambda i, j: (0 <= i < W) and (0 <= j < H)  # noqa: E731
    if own > 0:
        for jy, y in enumerate(ys):
            for ix, x in enumerate(xs):
                if inside(x, y):
                    gx[jy, ix] += own * T.tinted(lit[y, x], inner, a)
    for k, (sigma, wt) in enumerate(lvls):
        if c["tint"] == "core_to_edge":
            colour = inner + T.place(radius, k) * (outer - inner)
        else:
            colour = inner
        d = G.down_factor(sigma)
        nw, nh = -(-W // d), -(-H // d)
        pad = 4 * math.ceil(3 * sigma * 2 / d) + 64
        cells = np.zeros((nh + 2 * pad, nw + 2 * pad, 3))
        for j in range(nh):
            for i in range(nw):
                block = lit[j * d:(j + 1) * d, i * d:(i + 1) * d]
                cells[pad + j, pad + i] = block.sum(axis=(0, 1)) / (d * d)
        blurred = G.spread_level(cells, sigma, d, c["aspect_ratio"], c["aspect_angle"])
        u, v = (X + 0.5) / d - 0.5, (Y + 0.5) / d - 0.5
        u0, v0 = np.floor(u).astype(int), np.floor(v).astype(int)
        fu, fv = u - u0, v - v0
        for jy in range(len(ys)):
            for ix in range(len(xs)):
                p = np.zeros(3)
                for dj, wj in ((0, 1 - fv[jy]), (1, fv[jy])):
                    for di, wi in ((0, 1 - fu[ix]), (1, fu[ix])):
                        p += wj * wi * blurred[pad + v0[jy] + dj, pad + u0[ix] + di]
                gx[jy, ix] += wt * T.tinted(p, colour, a)
    return gx


def glow_layer(px, c, xs, ys):
    """The finished layer: D-542's when Spectral is off, else each channel at its own radius."""
    if c["spectral"] == "off":
        return T.glow_layer(px, c, xs, ys)
    gx = np.zeros((len(ys), len(xs), 3))
    for ch, key in enumerate(("spectral_red", "spectral_green", "spectral_blue")):
        gx[..., ch] = glow_sum(px, c, xs, ys, c["radius"] * c[key] / 100)[..., ch]
    g = gx * c["exposure"]
    screen = c["operation"] == "screen"
    if screen:
        g = np.minimum(g, 1.0)
    ga = np.minimum(1.0, g.max(axis=-1)) if c["unmult"] == "on" else np.ones(g.shape[:2])
    o = c["source_opacity"] / 100
    inside = lambda i, j: (0 <= i < W) and (0 <= j < H)  # noqa: E731
    out = np.zeros((len(ys), len(xs), 4))
    for jy, y in enumerate(ys):
        for ix, x in enumerate(xs):
            s = px[y, x] if inside(x, y) else np.zeros(4)
            rgb = g[jy, ix] + o * s[:3]
            if screen:
                rgb = np.minimum(rgb, 1.0)
            out[jy, ix, :3] = rgb
            out[jy, ix, 3] = min(1.0, ga[jy, ix] + o * s[3])
    return out


SCALES = ("spectral_red", "spectral_green", "spectral_blue")


def settings_at(c, frame_no):
    out = T.settings_at(c, frame_no)
    for k in SCALES:
        out[k] = min(MAX_SCALE, max(0, float(value_at(c[k], frame_no))))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(name="patches", spectral="on", spectral_red=80, spectral_green=100, spectral_blue=120,
         tint="off", inner_color="#ffffff", outer_color="#4080ff", tint_amount=100, **rest):
    c = T.case(name, tint=tint, inner_color=inner_color, outer_color=outer_color,
               tint_amount=tint_amount, **rest)
    c.update(spectral=spectral, spectral_red=spectral_red, spectral_green=spectral_green,
             spectral_blue=spectral_blue)
    return c


SPECTRAL = ("spectral",) + SCALES


def render(c, frame_no):
    s = settings_at(c, frame_no)
    out = glow_layer(G.pixels(c["drawing"]), s, [x - c["shift"] for x in range(W)],
                     list(range(H)))
    return [[float(v) for v in out[y, x]] for y in range(H) for x in range(W)]


CASES = {
    "FX-SPGLOW-001": ("Spectral on at its defaults (red 80, green 100, blue 120 per cent), radius "
                      "12, everything else at Soft Physical Glow's defaults: red stays closer to "
                      "each patch and blue reaches further, so the tail turns bluer outward.",
                      case(), [0]),
    "FX-SPGLOW-002": ("Spectral on, all three at 100: the same as Soft Physical Glow with "
                      "Spectral off (FX-SGLOW-001).",
                      case(spectral_red=100, spectral_blue=100), [0]),
    "FX-SPGLOW-003": ("Spectral off written in the file, with sizes set: Soft Physical Glow as "
                      "before (FX-SGLOW-001).",
                      case(spectral="off", spectral_red=30, spectral_blue=200), [0]),
    "FX-SPGLOW-004": ("Spectral on, radius 100, red 50, green 100, blue 200: a strong fringe, "
                      "the blue tail far wider than the red.",
                      case(radius=100, spectral_red=50, spectral_blue=200), [0]),
    "FX-SPGLOW-005": ("Spectral on, red 200, green 100, blue 50, the other way round: a red "
                      "tail outside a blue core.",
                      case(spectral_red=200, spectral_blue=50), [0]),
    "FX-SPGLOW-006": ("Spectral on, red 0: red does not spread at all, the light laid on itself "
                      "only; green and blue glow as before.",
                      case(spectral_red=0), [0]),
    "FX-SPGLOW-007": ("Spectral on with the grey drawing: the grey patch, white light, glows "
                      "white close in and fringes in colour further out.",
                      case("grey"), [0]),
    "FX-SPGLOW-008": ("Spectral on with Tint Core to Edge: each channel's levels tinted at its "
                      "own radius, then the channels taken one from each.",
                      case(tint="core_to_edge"), [0]),
    "FX-SPGLOW-009": ("Spectral on with Tint One Colour orange at 60 per cent, Threshold 30 and "
                      "Smooth 50 in Luminance.",
                      case(tint="one_color", inner_color=T.ORANGE, tint_amount=60, threshold=30,
                           threshold_smooth=50, threshold_mode="luminance"), [0]),
    "FX-SPGLOW-010": ("Blue keyed from 100 at frame 0 to 200 at frame 4, linear, red and green "
                      "at 100: frame 0 is FX-SPGLOW-002, frame 2 has blue at 150.",
                      case(spectral_red=100, spectral_blue=keyed((0, 100), (4, 200))), [0, 2, 4]),
    "FX-SPGLOW-011": ("Spectral on, Blend Mode Add, exposure 3: the fringed light adds up past "
                      "white.",
                      case(operation="add", exposure=3), [0]),
    "FX-SPGLOW-012": ("Spectral on, Unmult off, Source Opacity 0: the fringed glow alone, on "
                      "black.",
                      case(unmult="off", source_opacity=0), [0]),
    "FX-SPGLOW-013": ("Spectral on, Aspect Ratio 1.5 at Angle 30, moved three pixels right.",
                      case(aspect_ratio=1.5, aspect_angle=30, shift=3), [0]),
    "FX-SPGLOW-014": ("Spectral on, radius 2.25, red 50, blue 200: red has no level (radius "
                      "1.125), green one half-in level, blue two.",
                      case(radius=2.25, spectral_red=50, spectral_blue=200), [0]),
    "FX-SPGLOW-015": ("Spectral on, radius 2,000, red 50, blue 200: blue's radius 4,000, past "
                      "Radius's own top, is worked as it is.",
                      case(radius=2000, spectral_red=50, spectral_blue=200), [0]),
    "FX-SPGLOW-016": ("Spectral on, the patches half covering: the light counts at half.",
                      case("faint"), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SPGLOW-017": ("Spectral \"yes\", which is not on or off.", case(spectral="yes")),
    "FX-SPGLOW-018": ("Spectral Red 201, above 200.", case(spectral_red=201)),
    "FX-SPGLOW-019": ("Spectral Blue -1, below 0.", case(spectral_blue=-1)),
    "FX-SPGLOW-020": ("Spectral Green keyed to 250 at frame 4.",
                      case(spectral_green=keyed((0, 100), (4, 250)))),
}


# --- the project files ----------------------------------------------------------------------

def write(fx, c):
    p = G.project_json(fx, c)
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    if c["tint"] != "off":
        params.update({k: setting_json(c[k]) for k in T.TINT})
    params.update({k: setting_json(c[k]) for k in SPECTRAL})
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, px in G.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(px))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        base = G.plain(c["drawing"], c["shift"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(max(abs(a - b) for a, b in zip(px[i], base[i])) > 1e-9 for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        base = G.plain(c["drawing"])
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": base, "4": base},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_spectral_glow.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) <= e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-12: all(close(p, q, e) for p, q in zip(f, g))  # noqa: E731
    plain_glow = G.render(G.case(), 0)

    # 002 and 003 are D-353's picture.
    assert same(c["FX-SPGLOW-002"]["0"], plain_glow, 1e-12)
    assert same(c["FX-SPGLOW-003"]["0"], plain_glow, 0)
    # 001: blue over red grows outward, against the untinted glow's own proportions.
    one = c["FX-SPGLOW-001"]["0"]
    ratio = lambda p: p[2] / p[0]  # noqa: E731
    for x, y in ((15, 0), (0, 0)):
        assert ratio(one[at(x, y)]) > ratio(plain_glow[at(x, y)])
    # 005, the other way round: redder far out than the plain glow.
    assert ratio(c["FX-SPGLOW-005"]["0"][at(15, 0)]) < ratio(plain_glow[at(15, 0)])
    # 006: red alone does not spread: off the patches red is 0.
    drawn = G.plain("patches")
    for i in range(W * H):
        if drawn[i][3] == 0:
            assert c["FX-SPGLOW-006"]["0"][i][0] == 0
    # 007: the grey light fringes: at the far corner blue beats red.
    p = c["FX-SPGLOW-007"]["0"][at(15, 0)]
    assert p[2] > p[0]
    # 010's frames.
    assert same(c["FX-SPGLOW-010"]["0"], c["FX-SPGLOW-002"]["0"])
    assert not same(c["FX-SPGLOW-010"]["2"], c["FX-SPGLOW-010"]["4"], 1e-9)
    assert any(v > 1 for p in c["FX-SPGLOW-011"]["0"] for v in p[:3])
    assert all(p[3] == 1 for p in c["FX-SPGLOW-012"]["0"])
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= -1e-12 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
