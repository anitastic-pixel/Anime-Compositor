"""Soft Physical Glow's tint, worked a second way.

D-542 is pick #6 in `docs/effects/EFFECTS.md` (row 258), "Glow core-to-edge colour", built on
the shared soft-glow engine of D-353 (P0-21) as new settings of Soft Physical Glow, the
"physical" falloff of `core.glow`. PLUGINS.md section 2.3 (Deep Glow) settles that it is a
setting and not an effect of its own: Deep Glow's Tint Mode (Off, Monocolor, Duocolor), Color
Inner, Color Outer and Tint Strength are controls of the one glow, and the tint "colours the glow
evenly whatever the input's colour, even a grey input". The rule below is our own; no plugin code
or presets are used, and the docs never describe how the product mixes inner and outer.

The settings, all new, all left out of a file that does not use them:

* `tint`: "off" (the default: Soft Physical Glow exactly as D-353 draws it), "one_color" or
  "core_to_edge".
* `inner_color`, `#rrggbb`, default `#ffffff`: the colour of the core, or of the whole glow when
  `tint` is "one_color".
* `outer_color`, `#rrggbb`, default `#4080ff`: the colour of the tail's far end.
* `tint_amount`, 0 to 100 per cent, default 100, may be keyed.

The rule, inside D-353's (`tools/soft_glow_reference.py`), between the spread and Exposure
(PLUGINS.md 2.3, "Order of stages": tint after exposure is the same here, as both are
multiplications, and before the dirt matte):

1. Each level of the spread is read at the pixel as before (between its cells), and that value
   P, before its weight, is tinted: P' = (1 - a) P + a max(P) C, where a is Tint Amount as a
   fraction and max(P) the brightest of P's channels, so a white, a grey and a red light of the
   same brightness glow the same colour, and the colour as picked is what a white light glows.
2. The colour C of level k (k = 0 the widest) is the inner colour mixed towards the outer by
   t_k = clamp(1 - k / log2(2 Radius / 3), 0, 1): the widest level is the outer colour, a level
   of size sigma 0.5 (where levels start to count) would be the inner, and in between by octave,
   so the colour moves smoothly as Radius is animated. The light's own share (D-353's light laid
   on itself) is the core: it takes the inner colour. Mixing is in linear light.
3. "one_color": every level, and the light's own share, take the inner colour.

Then Exposure, Blend Mode, Unmult and Source Opacity as before.

**This file never runs the build's code path.** It works in double precision with numpy, on
D-353's padded planes of cells; the build works in single precision, pixel by pixel.

Every case is a composition 16 pixels by 10 holding one drawing the same size, the drawings of
`tools/soft_glow_reference.py`. The drawings go into `Fixtures/glow_tint/media`, the projects
into `Fixtures/glow_tint`, and the expected frames into
`Fixtures/glow_tint/expected_glow_tint.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glow_tint_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import soft_glow_reference as G  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glow_tint"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def linear(hexword):
    return np.array([srgb_to_linear(int(hexword[i:i + 2], 16) / 255) for i in (1, 3, 5)])


def place(radius, k):
    """How far out level k sits: 0 at the core, 1 at the widest level."""
    span = math.log2(2 * radius / 3)
    return min(1.0, max(0.0, 1 - k / span)) if span > 0 else 0.0


def tinted(p, colour, a):
    """P' = (1 - a) P + a max(P) C, over an array of pixels (... x 3)."""
    return (1 - a) * p + a * p.max(axis=-1, keepdims=True) * colour


def glow_layer(px, c, xs, ys):
    """D-353's finished layer with the tint: G.glow_layer with step 1 to 3 above added."""
    if c["tint"] == "off":
        return G.glow_layer(px, c, xs, ys)
    inner, outer = linear(c["inner_color"]), linear(c["outer_color"])
    a = c["tint_amount"] / 100
    lit = G.light(px, c)
    lvls, own = G.levels(c["radius"])
    gx = np.zeros((len(ys), len(xs), 3))
    X, Y = np.array(xs, float), np.array(ys, float)
    inside = lambda i, j: (0 <= i < W) and (0 <= j < H)  # noqa: E731
    if own > 0:
        for jy, y in enumerate(ys):
            for ix, x in enumerate(xs):
                if inside(x, y):
                    gx[jy, ix] += own * tinted(lit[y, x], inner, a)
    for k, (sigma, wt) in enumerate(lvls):
        if c["tint"] == "one_color":
            colour = inner
        else:
            colour = inner + place(c["radius"], k) * (outer - inner)
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
                gx[jy, ix] += wt * tinted(p, colour, a)
    g = gx * c["exposure"]
    screen = c["operation"] == "screen"
    if screen:
        g = np.minimum(g, 1.0)
    if c["unmult"] == "on":
        ga = np.minimum(1.0, g.max(axis=-1))
    else:
        ga = np.ones(g.shape[:2])
    o = c["source_opacity"] / 100
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


def settings_at(c, frame_no):
    out = G.settings_at(c, frame_no)
    out["tint_amount"] = min(100, max(0, float(value_at(c["tint_amount"], frame_no))))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(name="patches", tint="core_to_edge", inner_color="#ffffff", outer_color="#4080ff", tint_amount=100,
         **rest):
    c = G.case(name, **rest)
    c.update(tint=tint, inner_color=inner_color, outer_color=outer_color,
             tint_amount=tint_amount)
    return c


TINT = ("tint", "inner_color", "outer_color", "tint_amount")
ORANGE = "#ff8020"


def render(c, frame_no):
    s = settings_at(c, frame_no)
    out = glow_layer(G.pixels(c["drawing"]), s, [x - c["shift"] for x in range(W)],
                     list(range(H)))
    return [[float(v) for v in out[y, x]] for y in range(H) for x in range(W)]


CASES = {
    "FX-GTINT-001": ("Tint One Colour, orange, radius 12, everything else at Soft Physical "
                     "Glow's defaults: the yellow, brown and purple patches all glow orange.",
                     case(tint="one_color", inner_color=ORANGE), [0]),
    "FX-GTINT-002": ("Tint Core to Edge at its defaults, white core and blue edge, radius 12: "
                     "white close to each patch, turning blue as the glow spreads.",
                     case(), [0]),
    "FX-GTINT-003": ("Core to Edge, radius 100: seven levels, so the change from white to blue "
                     "runs over more steps.",
                     case(radius=100), [0]),
    "FX-GTINT-004": ("Core to Edge, Tint Amount 50: halfway between the glow's own colours and "
                     "the tint.",
                     case(tint_amount=50), [0]),
    "FX-GTINT-005": ("Core to Edge, Tint Amount 0: the glow's own colours, the same as Soft "
                     "Physical Glow with no tint (FX-SGLOW-001).",
                     case(tint_amount=0), [0]),
    "FX-GTINT-006": ("Tint Off written in the file, with colours set: Soft Physical Glow as "
                     "before (FX-SGLOW-001).",
                     case(tint="off", inner_color=ORANGE, outer_color="#00ff00"), [0]),
    "FX-GTINT-007": ("One Colour, orange, on the drawing with a grey patch: the grey glows "
                     "orange too, as brightly as a colour of its brightness would.",
                     case("grey", tint="one_color", inner_color=ORANGE), [0]),
    "FX-GTINT-008": ("Core to Edge, orange core and purple edge, Threshold 30 in Luminance: "
                     "only the yellow glows, orange near it and purple further out.",
                     case(inner_color=ORANGE, outer_color="#8020ff", threshold=30,
                          threshold_mode="luminance"), [0]),
    "FX-GTINT-009": ("Core to Edge, radius 0: nothing spreads; the light laid on itself takes "
                     "the core colour, white, so each patch brightens towards grey-white.",
                     case(radius=0), [0]),
    "FX-GTINT-010": ("Core to Edge, radius 2.25: the one level, half in, is the widest, so it "
                     "is blue, and the half laid on itself is white.",
                     case(radius=2.25), [0]),
    "FX-GTINT-011": ("Tint Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                     "FX-GTINT-005, frame 2 is FX-GTINT-004, frame 4 is FX-GTINT-002.",
                     case(tint_amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-GTINT-012": ("Core to Edge, Blend Mode Add, exposure 3: the tinted light adds up past "
                     "white.",
                     case(operation="add", exposure=3), [0]),
    "FX-GTINT-013": ("Core to Edge, Unmult off, Source Opacity 0: the tinted glow alone, on "
                     "black.",
                     case(unmult="off", source_opacity=0), [0]),
    "FX-GTINT-014": ("Core to Edge, Aspect Ratio 1.5 at Angle 30, moved three pixels right: "
                     "the leaning glow tinted, and the layer grown to hold it.",
                     case(aspect_ratio=1.5, aspect_angle=30, shift=3), [0]),
    "FX-GTINT-015": ("Core to Edge, the patches half covering: the light counts at half, so the "
                     "tint is half as bright.",
                     case("faint"), [0]),
    "FX-GTINT-016": ("Core to Edge, radius 2,000: on a drawing this small the wide blue "
                     "levels spread their light far past the frame, so what stays is mostly the "
                     "small, near-white levels: a faint wash, white touched with blue.",
                     case(radius=2000), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GTINT-017": ("Tint \"duocolor\", which is not off, one_color or core_to_edge.",
                     case(tint="duocolor")),
    "FX-GTINT-018": ("Inner Colour \"#12345\", not six hexadecimal digits.",
                     case(inner_color="#12345")),
    "FX-GTINT-019": ("Outer Colour \"blue\", not a colour word the file takes.",
                     case(outer_color="blue")),
    "FX-GTINT-020": ("Tint Amount 101, above 100.", case(tint_amount=101)),
    "FX-GTINT-021": ("Tint Amount -1, below 0.", case(tint_amount=-1)),
    "FX-GTINT-022": ("Tint Amount keyed to 150 at frame 4.",
                     case(tint_amount=keyed((0, 0), (4, 150)))),
}


# --- the project files ----------------------------------------------------------------------

def write(fx, c):
    p = G.project_json(fx, c)
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"].update(
        {k: setting_json(c[k]) for k in TINT})
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

    (OUT / "expected_glow_tint.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) <= e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-12: all(close(p, q, e) for p, q in zip(f, g))  # noqa: E731
    plain_glow = G.render(G.case(), 0)

    # The places: widest level outer, and by octave inward; continuous as Radius moves.
    assert place(12, 0) == 1.0 and abs(place(12, 1) - (1 - 1 / 3)) < 1e-12
    assert place(1.6, 0) == 1.0 and place(1.0, 0) == 0.0
    # 005 and 006 are D-353's picture, to the last bit of the double-precision sum.
    assert same(c["FX-GTINT-005"]["0"], plain_glow, 1e-12)
    assert same(c["FX-GTINT-006"]["0"], plain_glow, 0)
    # 001: one colour: away from the patches the glow is orange's proportions.
    orange = linear(ORANGE)
    for x, y in ((0, 4), (5, 4), (10, 4), (15, 0)):
        p = c["FX-GTINT-001"]["0"][at(x, y)]
        assert abs(p[1] / p[0] - orange[1] / orange[0]) < 1e-9 and abs(p[2] / p[0] - orange[2] / orange[0]) < 1e-9
    # 002: blue grows outward: the far corner is bluer than the pixel beside a patch.
    two = c["FX-GTINT-002"]["0"]
    ratio = lambda p: p[2] / p[0]  # noqa: E731
    assert ratio(two[at(15, 0)]) > ratio(two[at(5, 4)]) > 1.0
    assert ratio(c["FX-GTINT-003"]["0"][at(15, 0)]) > 1.0
    # 004 between 005 and 002, channel by channel, off the patches.
    for x, y in ((0, 4), (15, 0)):
        for ch in range(3):
            lo, hi = sorted((c["FX-GTINT-005"]["0"][at(x, y)][ch], two[at(x, y)][ch]))
            assert lo - 1e-12 <= c["FX-GTINT-004"]["0"][at(x, y)][ch] <= hi + 1e-12
    # 007: the grey patch's own glow, beside it in the gap rows, is orange's proportions too.
    grey = c["FX-GTINT-007"]["0"]
    p = grey[at(7, 1)]
    assert abs(p[1] / p[0] - orange[1] / orange[0]) < 1e-9
    # 009: radius 0: each pixel plus its brightest channel in white, held at 1.
    drawn = G.plain("patches")
    for i in range(W * H):
        m = max(drawn[i][:3])
        assert close(c["FX-GTINT-009"]["0"][i], [min(1, v + m) for v in drawn[i][:3]] + [drawn[i][3]])
    # 011's frames.
    assert same(c["FX-GTINT-011"]["0"], c["FX-GTINT-005"]["0"])
    assert same(c["FX-GTINT-011"]["2"], c["FX-GTINT-004"]["0"])
    assert same(c["FX-GTINT-011"]["4"], two)
    assert any(v > 1 for p in c["FX-GTINT-012"]["0"] for v in p[:3])
    assert all(p[3] == 1 for p in c["FX-GTINT-013"]["0"])
    assert 1.0 < ratio(c["FX-GTINT-016"]["0"][at(15, 0)]) < ratio(two[at(15, 0)])
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= -1e-12 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
