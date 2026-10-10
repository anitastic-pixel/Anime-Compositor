"""D-412's Map Chromatic Displacement, worked a second way.

PLUGINS.md's pick #19: light bent through water or glass parts into its colours. PLUGINS.md
says to merge Prism Displacement into `core.displacement_map` as a "spread by channel" setting,
and that Red Giant's Chromatic Displacement, which a Japanese anime compositor (Qiita,
median_ky) recommends for natural colour fringing on water, goes into the same merge. Red Giant's
manual names its edge: Spread Chroma brings "a rainbow spectrum of colors" to the displacement,
and Spread Quality sets how finely it is divided. Three channel copies moved apart give three
hard fringes; a spectrum gives the smooth rainbow.

`core.displacement_map` gains four settings, all keyable:

- `red_amount`, `green_amount`, `blue_amount`, -1000 to 1000 per cent, absent 100: how far each
  colour moves, as a share of D-193's displacement.
- `spectrum`, 3 to 32 samples, absent 3, its whole part counted: how many colours between red
  and blue are read.

At the holder's composition frame, with D-193's displacement (dx, dy) of a pixel (steps 1 to 4
of `tools/displacement_map_reference.py`, the map and the channel words read as before), the
amounts a_r, a_g, a_b as fractions (per cent / 100) and N the spectrum's whole part:

1. For i = 0 .. N - 1, t_i = i / (N - 1), from red (0) through green (1/2) to blue (1). The
   sample's amount a(t) runs straight from a_r to a_g over the first half and from a_g to a_b
   over the second; its weights are w_r(t) = max(0, 1 - 2t), w_g(t) = 1 - |2t - 1|,
   w_b(t) = max(0, 2t - 1).
2. Sample i is the picture read as D-193 reads it (bilinear, wrapped with `wrap`), at
   (x + 0.5 + a(t_i) dx, y + 0.5 + a(t_i) dy).
3. Each colour channel c of the output is the sum of w_c(t_i) times sample i's channel c, over
   the sum of w_c(t_i); the covering is the largest of the samples' coverings, so no colour is
   ever more than its covering.

With N = 3 the samples are red, green and blue themselves: three copies, each channel moved by its
own amount. More samples blend each channel over the colours beside it, the smooth rainbow. With
the three amounts equal, every sample is the same read, and the output is D-193's with its
displacement times that amount; at 100, what a file without the settings means, D-193's rule
exactly. The amounts are shares, not distances, so a draft leaves them (the maxima are already
scaled). With Expand Output (D-315) the layer grows by the larger maximum times the largest of
|a_r|, |a_g|, |a_b|, rounded up.

Every case is displacement_map_reference's 16 by 10 composition, its map layers and its
`holder`. The projects go into `Fixtures/map_chromatic`, the expected frames into
`Fixtures/map_chromatic/expected_map_chromatic.json`.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/map_chromatic_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import displacement_map_reference as D  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "map_chromatic"
TOLERANCE = D.TOLERANCE
W, H, FRAMES = D.W, D.H, D.FRAMES
AMOUNTS = ("red_amount", "green_amount", "blue_amount")
RANGES = {"red_amount": (-1000, 1000), "green_amount": (-1000, 1000),
          "blue_amount": (-1000, 1000), "spectrum": (3, 32)}
ABSENT = {"red_amount": 100, "green_amount": 100, "blue_amount": 100, "spectrum": 3}


# --- the rule -------------------------------------------------------------------------------

def setting(c, k, frame):
    if c[k] is None:
        return ABSENT[k]
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame)))


def samples(a, n):
    """(amount, (w_r, w_g, w_b)) for each of the n samples, a the three amounts as fractions."""
    out = []
    for i in range(n):
        t = i / (n - 1)
        amount = a[0] + (a[1] - a[0]) * 2 * t if t <= 0.5 else a[1] + (a[2] - a[1]) * (2 * t - 1)
        out.append((amount, (max(0.0, 1 - 2 * t), 1 - abs(2 * t - 1), max(0.0, 2 * t - 1))))
    return out


def chromatic(o, m, horizontal, max_h, vertical, max_v, wrap, a, n):
    if m is None:
        return o
    spec = samples(a, n)
    total = [sum(w[c] for _, w in spec) for c in range(3)]
    read = D.wrapped if wrap == "on" else L.bilinear
    out = []
    for y in range(o["h"]):
        for x in range(o["w"]):
            p = m["px"][y * o["w"] + x]
            dx = (2 * D.channel(horizontal, p) - 1) * max_h
            dy = (2 * D.channel(vertical, p) - 1) * max_v
            px = [0.0, 0.0, 0.0, 0.0]
            for amount, w in spec:
                s = read(o, x + 0.5 + amount * dx, y + 0.5 + amount * dy)
                for c in range(3):
                    px[c] += w[c] * s[c]
                px[3] = max(px[3], s[3])
            out.append([px[0] / total[0], px[1] / total[1], px[2] / total[2], px[3]])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="ramp", max_h=2, max_v=2, red=None, green=None, blue=None, spectrum=None,
         **kw):
    """None: the file does not say."""
    c = D.case(layer, max_h=max_h, max_v=max_v, **kw)
    c.update(red_amount=red, green_amount=green, blue_amount=blue, spectrum=spectrum)
    return c


def render(c, frame):
    a = [setting(c, k, frame) / 100 for k in AMOUNTS]
    n = math.floor(setting(c, "spectrum", frame))
    out = chromatic(D.decoded("holder"), D.the_map(c, frame), c["horizontal"],
                    value_at(c["max_horizontal"], frame), c["vertical"],
                    value_at(c["max_vertical"], frame), c["wrap"], a, n)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


SPLIT = {"red": 0, "green": 100, "blue": 200}  # red stays, green moves as D-193, blue twice as far
WATER = {"red": 50, "green": 100, "blue": 150}

CASES = {
    "FX-MAPCHROMA-001": ("The ramp at most 2, all three amounts 100 written and 3 samples: "
                         "FX-DMAP-002 exactly.", case(red=100, green=100, blue=100, spectrum=3),
                         (0,)),
    "FX-MAPCHROMA-002": ("A white solid at most 2, red 0, green 100, blue 200, 3 samples: red "
                         "stays put, green is read 2 right and 2 below, blue 4 right and 4 below; "
                         "the covering is the largest of the three.",
                         case("white", **SPLIT, spectrum=3), (0,)),
    "FX-MAPCHROMA-003": ("FX-MAPCHROMA-002 with 9 samples: each channel blended over the colours "
                         "beside it, so between red's and blue's places the fringes run smoothly.",
                         case("white", **SPLIT, spectrum=9), (0,)),
    "FX-MAPCHROMA-004": ("The ramp, red 50, green 100, blue 150, 3 samples: where the ramp "
                         "pushes hardest, at its ends, the colours part most.",
                         case(**WATER, spectrum=3), (0,)),
    "FX-MAPCHROMA-005": ("FX-MAPCHROMA-004 with 16 samples, an even count: the rainbow.",
                         case(**WATER, spectrum=16), (0,)),
    "FX-MAPCHROMA-006": ("The painted map, blue across at most 2.5 and luminance down at most "
                         "1.5 (FX-DMAP-008), red -100, green 0, blue 100, 5 samples: red moves "
                         "against the push, green stays, blue with it.",
                         case("paint", max_h=2.5, max_v=1.5, horizontal="blue",
                              vertical="luminance", red=-100, green=0, blue=100, spectrum=5),
                         (0,)),
    "FX-MAPCHROMA-007": ("The white solid, blue keyed from 100 at frame 0 to 300 at frame 4, the "
                         "others absent: frame 0 is FX-DMAP-003, then blue parts from the rest.",
                         case("white", blue=keyed((0, 100), (4, 300))), (0, 2, 4)),
    "FX-MAPCHROMA-008": ("FX-MAPCHROMA-002 with Wrap on: what each channel reads from beyond "
                         "the right or the bottom comes from the other side.",
                         case("white", **SPLIT, spectrum=3, wrap="on"), (0,)),
    "FX-MAPCHROMA-009": ("FX-MAPCHROMA-004 on the holder moved 2 right and 1 down: the same "
                         "picture, moved.", case(**WATER, spectrum=3, shift=(2, 1)), (0,)),
    "FX-MAPCHROMA-010": ("FX-MAPCHROMA-005 on an adjustment layer above the holder: the frame "
                         "is the holder's rectangle, so FX-MAPCHROMA-005.",
                         case(**WATER, spectrum=16, on="adjust"), (0,)),
    "FX-MAPCHROMA-011": ("Spectrum 4.5: its whole part, 4 samples.",
                         case(**WATER, spectrum=4.5), (0,)),
    "FX-MAPCHROMA-012": ("All three amounts 50 with 9 samples: no colour parts, the ramp's "
                         "displacement halved (FX-DMAP-002 at most 1).",
                         case(red=50, green=50, blue=50, spectrum=9), (0,)),
    "FX-MAPCHROMA-013": ("Spectrum keyed from 3 at frame 0 to 11 at frame 4 on FX-MAPCHROMA-002: "
                         "frame 0 is FX-MAPCHROMA-002, frame 3 9 samples, FX-MAPCHROMA-003.",
                         case("white", **SPLIT, spectrum=keyed((0, 3), (4, 11))), (0, 3)),
    "FX-MAPCHROMA-014": ("No layer named: nothing moves, whatever the amounts.",
                         case("", **SPLIT, spectrum=9), (0,)),
}

INVALID = {
    "FX-MAPCHROMA-015": ("Red amount 1001, above 1000.", case(red=1001)),
    "FX-MAPCHROMA-016": ("Blue amount -1001, below -1000.", case(blue=-1001)),
    "FX-MAPCHROMA-017": ("Spectrum 2, below 3.", case(spectrum=2)),
    "FX-MAPCHROMA-018": ("Spectrum 33, above 32.", case(spectrum=33)),
    "FX-MAPCHROMA-019": ("Green amount keyed from 100 at frame 0 to 2000 at frame 4.",
                         case(green=keyed((0, 100), (4, 2000)))),
}


def project_json(fx, c):
    p = D.project_json(fx, c)
    layers = p["compositions"][0]["layers"]
    params = next(l for l in layers if l.get("effects"))["effects"][0]["parameters"]
    for k in AMOUNTS + ("spectrum",):
        if c[k] is not None:
            params[k] = setting_json(c[k])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in D.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = D.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = D.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_map_chromatic.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((OUT.parent / "displacement_map" / "expected_displacement_map.json")
                     .read_text(encoding="utf-8"))["cases"]
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    o = D.decoded("holder")
    drawn = D.plain(case())

    # The weights: every channel's add to more than nothing, and 3 samples are the channels.
    for n in range(3, 33):
        assert all(sum(w[ch] for _, w in samples([1, 1, 1], n)) > 0 for ch in range(3)), n
    assert [w for _, w in samples([0, 1, 2], 3)] == [(1, 0, 0), (0, 1, 0), (0, 0, 1)]
    assert [a for a, _ in samples([0, 1, 2], 5)] == [0, 0.5, 1, 1.5, 2]

    # Equal amounts at 100 are D-193 itself; at 50, its displacement halved.
    assert near(c["FX-MAPCHROMA-001"]["0"], old["FX-DMAP-002"]["frames"]["0"])
    assert near(c["FX-MAPCHROMA-012"]["0"], D.render(D.case("ramp", max_h=1, max_v=1), 0))
    assert near(c["FX-MAPCHROMA-007"]["0"], old["FX-DMAP-003"]["frames"]["0"])
    assert c["FX-MAPCHROMA-014"]["0"] == drawn

    # Three samples on a white map: each channel the picture moved whole pixels.
    at = lambda dx, dy, x, y: L.at(o, x + dx, y + dy)  # noqa: E731
    two = c["FX-MAPCHROMA-002"]["0"]
    for y in range(H):
        for x in range(W):
            r, g, b = at(0, 0, x, y), at(2, 2, x, y), at(4, 4, x, y)
            want = [r[0], g[1], b[2], max(r[3], g[3], b[3])]
            assert near([two[y * W + x]], [want]), (x, y)
    # More samples: the same ends, smoother between, so a different picture.
    assert c["FX-MAPCHROMA-003"]["0"] != two
    assert c["FX-MAPCHROMA-005"]["0"] != c["FX-MAPCHROMA-004"]["0"]
    assert c["FX-MAPCHROMA-011"]["0"] == render(case(**WATER, spectrum=4), 0)
    assert c["FX-MAPCHROMA-013"]["0"] == two
    assert near(c["FX-MAPCHROMA-013"]["3"], c["FX-MAPCHROMA-003"]["0"])
    # Wrap changes only reads past an edge; the adjustment layer is the holder's own frame.
    assert c["FX-MAPCHROMA-008"]["0"] != two
    assert near(c["FX-MAPCHROMA-010"]["0"], c["FX-MAPCHROMA-005"]["0"])
    moved = c["FX-MAPCHROMA-009"]["0"]
    four = c["FX-MAPCHROMA-004"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == four[y * W + x] for y in range(H - 1)
               for x in range(W - 2))
    # Red against the push and blue with it: a picture none of the plain cases draw.
    assert c["FX-MAPCHROMA-006"]["0"] != old["FX-DMAP-008"]["frames"]["0"]
    # No colour ever exceeds its covering, and every covering is within 0 and 1.
    for v in c.values():
        for px in v.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(p[ch] <= p[3] + 1e-12 for ch in range(3))
    print("checked")


if __name__ == "__main__":
    main()
