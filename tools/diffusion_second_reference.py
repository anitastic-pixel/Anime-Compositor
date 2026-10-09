"""Diffusion's second pass, worked a second way.

D-364 adds a second pass to `core.diffusion` (D-148), so anime compositing's usual two-layer
diffusion is one effect: a blurred copy of the picture laid over it in Lighten or Screen (the
first pass, as before), then a blurred copy laid over that in Soft Light or Overlay. Two
settings are new. `second_blend` is "soft_light" or "overlay" (soft_light): how the second copy
is laid on. `second_amount` is 0 to 100 (0), per cent: how much of it is laid on. At 0 the
effect is exactly D-148's, so every older file draws as before.

The sources (PLUGINS.md, section 3.2): tomoex lays Lighten at 50% under Soft Light at 50%, each
a blurred layer, sizes not given; taka2composite calls Lighten the essential mode for diffusion;
the RETAS lesson's second layer is the sharp drawing at 30% in Normal, to bring the lines back,
which the first pass's amount already does. None gives the second copy a blur of its own, so
both copies share one: the picture blurred once, at Radius.

The rule. Radius 0, or amount 0 and second amount 0: the output is the input, exactly. B is the
input through document 21's Gaussian blur at sigma radius / 3, read at the input's own pixels.
At a pixel with covering a > 0, b = p.rgb / a its straight linear colour and g = B.rgb / B.a
when B.a > 0, else b. First, as D-148: r = b + amount / 100 (f - b), f the screen, lighten or
normal of b and g. Then, as document 21's layer modes (D-301) lay a layer in overlay or soft
light, on the encoded colours held to 0 to 1 and decoded again, with r beneath and g on top:
s = to_linear(m(to_srgb(r), to_srgb(g))), m the W3C's overlay or soft light. The output is
((r + second_amount / 100 (s - r)) a, a), not clamped. A pixel with a = 0 is left as it is.

**This file never runs the build's code path.** It reuses `tools/diffusion_reference.py` for
the drawing, the blur and the first pass (itself independent of the build), and works the
second pass here, in double precision on lists, from the W3C's Compositing and Blending
definitions of overlay and soft light, not from the build's `grade::mixer`.

The cases use FX-DIFFUSE-001's cel, 16 by 10. The projects go into `Fixtures/diffusion` as
`fx_diffuse_019.json` on, and the expected frames into
`Fixtures/diffusion/expected_diffusion_second.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/diffusion_second_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import diffusion_reference as D  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = D.W, D.H
OUT = D.OUT
TOLERANCE = D.TOLERANCE
SECOND = (0, 100)
SECOND_BLENDS = ("soft_light", "overlay")


# --- the rule -------------------------------------------------------------------------------

def to_srgb(c):
    c = min(1.0, max(0.0, c))
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def to_linear(e):
    return e / 12.92 if e <= 0.04045 else ((e + 0.055) / 1.055) ** 2.4


def overlay(cb, cs):
    """W3C overlay: hard light with the layers swapped; cb the backdrop, cs the source."""
    return 2 * cb * cs if cb <= 0.5 else 1 - 2 * (1 - cb) * (1 - cs)


def soft_light(cb, cs):
    """W3C soft light."""
    if cs <= 0.5:
        return cb - (1 - 2 * cs) * cb * (1 - cb)
    d = ((16 * cb - 12) * cb + 4) * cb if cb <= 0.25 else math.sqrt(cb)
    return cb + (2 * cs - 1) * (d - cb)


MODES = {"soft_light": soft_light, "overlay": overlay}


def second(r, g, blend):
    """The second copy g laid on r, per channel, on encoded colours, decoded again."""
    m = MODES[blend]
    return [to_linear(m(to_srgb(u), to_srgb(v))) for u, v in zip(r, g)]


def diffusion(px, radius, amount, blend, second_amount, second_blend, size=(W, H)):
    out = [list(p) for p in px]
    if radius == 0 or (amount == 0 and second_amount == 0):
        return out  # the build exits early
    for o, q in zip(out, D.glow(px, radius, size)):
        a = o[3]
        if a <= 0:
            continue
        b = [v / a for v in o[:3]]
        g = [v / q[3] for v in q[:3]] if q[3] > 0 else b
        f = D.laid(b, g, blend)
        r = [u + amount / 100 * (v - u) for u, v in zip(b, f)]
        s = second(r, g, second_blend)
        o[:3] = [(u + second_amount / 100 * (v - u)) * a for u, v in zip(r, s)]
    return out


# --- the cases ------------------------------------------------------------------------------

def case(radius=10, amount=50, blend="lighten", second_amount=50, second_blend="soft_light",
         shift=0):
    return {"drawing": "cel", "radius": radius, "amount": amount, "blend": blend,
            "second_amount": second_amount, "second_blend": second_blend, "shift": shift}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [R.working(p) for row in D.DRAWINGS[c["drawing"]] for p in row]
    v = lambda k, r: held(value_at(c[k], frame_no), r)  # noqa: E731
    return R.frame(diffusion(pixels, v("radius", D.RADIUS), v("amount", D.AMOUNT), c["blend"],
                             v("second_amount", SECOND), c["second_blend"]), c["shift"])


def plain(c):
    return render(case(amount=0, second_amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-DIFFUSE-019": ("The two-layer diffusion as tomoex lays it: radius 10, lighten at 50, "
                       "then soft light at 50. The shadow and the line lift as in lighten "
                       "alone, then the soft light deepens the contrast a little: every dark "
                       "channel (encoded below a half) of the glow pulls its pixel down, every "
                       "light one lifts it.", case(), [0]),
    "FX-DIFFUSE-020": ("Screen at 50 then soft light at 50: the settings as they start (FX-"
                       "DIFFUSE-001) with the second pass laid on.", case(blend="screen"), [0]),
    "FX-DIFFUSE-021": ("Lighten at 50 then overlay at 30.",
                       case(second_amount=30, second_blend="overlay"), [0]),
    "FX-DIFFUSE-022": ("Screen at 50 with the second pass written at 0: exactly FX-DIFFUSE-001, "
                       "sample for sample.", case(blend="screen", second_amount=0), [0]),
    "FX-DIFFUSE-023": ("Radius 3, amount 0, soft light at 100: only the second pass, the "
                       "blurred picture in soft light over the drawing. Each channel moves toward "
                       "the glow's side of an encoded half, darker where the glow is dark, "
                       "lighter where it is light: the shadow's far corner (1, 1) darkens in "
                       "every channel, and every skin pixel lightens in every channel.",
                       case(radius=3, amount=0, second_amount=100), [0]),
    "FX-DIFFUSE-024": ("Radius 3, amount 0, overlay at 100: only the second pass, in overlay; "
                       "the same directions as FX-DIFFUSE-023.",
                       case(radius=3, amount=0, second_amount=100, second_blend="overlay"),
                       [0]),
    "FX-DIFFUSE-025": ("Radius 0 with the second pass at 50: the drawing, untouched.",
                       case(radius=0), [0]),
    "FX-DIFFUSE-026": ("Second amount keyed from 0 at frame 0 to 100 at frame 4, linear, over "
                       "lighten at 50: frame 0 is lighten alone, frame 2 is FX-DIFFUSE-019, "
                       "frame 4 the soft light laid fully on.",
                       case(second_amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-DIFFUSE-027": ("Second amount keyed from 0 to 100, eased past its end: held at 100 from "
                       "frame 2, so frames 2 and 4 are FX-DIFFUSE-026's frame 4.",
                       case(second_amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-DIFFUSE-028": ("FX-DIFFUSE-019 moved three pixels right: the same, moved.",
                       case(shift=3), [0, 3]),
}

INVALID = {
    "FX-DIFFUSE-029": ("Second amount 101, above 100.", case(second_amount=101)),
    "FX-DIFFUSE-030": ("Second amount -1, below 0.", case(second_amount=-1)),
    "FX-DIFFUSE-031": ("Second blend \"screen\", which is not soft_light or overlay.",
                       case(second_blend="screen")),
    "FX-DIFFUSE-032": ("Second amount keyed to 150 at frame 4.",
                       case(second_amount=keyed((0, 50), (4, 150)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = D.project_json(fx, c)
    prm = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    prm["second_blend"] = c["second_blend"]
    prm["second_amount"] = setting_json(c["second_amount"])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
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
    (OUT / "expected_diffusion_second.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((OUT / "expected_diffusion.json").read_text(encoding="utf-8"))["cases"]
    drawn = plain(case())
    px = [R.working(p) for row in D.DRAWINGS["cel"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    kind = lambda i: D.DRAWINGS["cel"][i // W][i % W]  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3] > 0]
    q = D.glow(px, 10)

    # The W3C's pieces: a 50% encoded grey on top leaves the backdrop as it is in soft light,
    # and black and white beneath stay put in overlay.
    for v in (0.0, 0.1, 0.3, 0.5, 0.8, 1.0):
        assert abs(soft_light(v, 0.5) - v) < 1e-15
    for v in (0.0, 0.2, 0.7, 1.0):
        assert overlay(0.0, v) == 0.0 and overlay(1.0, v) == 1.0
    assert abs(to_linear(to_srgb(0.25)) - 0.25) < 1e-15
    # Second amount 0 is D-148's effect exactly, for every blend.
    for blend in D.BLENDS:
        assert diffusion(px, 10, 50, blend, 0, "overlay") == D.diffusion(px, 10, 50, blend)

    # Coverings kept, empty pixels empty, in every case.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-DIFFUSE-028" else 0))
        for f in frames.values():
            for p, b in zip(f, base):
                assert p[3] == b[3], fx
                if b[3] == 0:
                    assert p == [0.0] * 4, fx

    lighten50 = render(case(second_amount=0), 0)
    nineteen = c["FX-DIFFUSE-019"]["0"]
    assert like(lighten50, D.diffusion(px, 10, 50, "lighten"))
    for i in shows:
        g = [v / q[i][3] for v in q[i][:3]]
        for k in range(3):
            d = nineteen[i][k] - lighten50[i][k]
            # soft light moves toward the glow's side of an encoded half, never past
            if to_srgb(g[k]) < 0.5 - 1e-9:
                assert d <= 1e-15, (i, k)
            elif to_srgb(g[k]) > 0.5 + 1e-9:
                assert d >= -1e-15, (i, k)
    twenty = c["FX-DIFFUSE-020"]["0"]
    assert like(c["FX-DIFFUSE-022"]["0"], old["FX-DIFFUSE-001"]["frames"]["0"])
    assert c["FX-DIFFUSE-022"]["0"] == old["FX-DIFFUSE-001"]["frames"]["0"]
    assert twenty != old["FX-DIFFUSE-001"]["frames"]["0"]
    assert c["FX-DIFFUSE-025"]["0"] == drawn
    t23, t24 = c["FX-DIFFUSE-023"]["0"], c["FX-DIFFUSE-024"]["0"]
    light = [i for i in shows if kind(i) == D.LIGHT]
    shadow = [i for i in shows if kind(i) == D.SHADOW]
    q3 = D.glow(px, 3)
    for t in (t23, t24):
        for i in shows:
            g = [to_srgb(v / q3[i][3]) for v in q3[i][:3]]
            for k in range(3):
                d = t[i][k] - drawn[i][k]
                assert (d <= 1e-15) if g[k] < 0.5 else (d >= -1e-15), (i, k)
        assert all(u < v for u, v in zip(t[at(1, 1)][:3], drawn[at(1, 1)][:3]))
        assert all(all(u > v for u, v in zip(t[i][:3], drawn[i][:3]))
                   for i in shows if kind(i) == D.SKIN)
    t26 = c["FX-DIFFUSE-026"]
    assert like(t26["0"], lighten50) and like(t26["2"], nineteen)
    assert like(t26["2"], [[(u + v) / 2 for u, v in zip(p, r)] for p, r in zip(lighten50, t26["4"])])
    t27 = c["FX-DIFFUSE-027"]
    assert value_at(case(second_amount=keyed((0, 0, OVERSHOOT), (4, 100)))["second_amount"], 2) > 100
    assert like(t27["0"], lighten50) and t27["2"] == t26["4"] and t27["4"] == t26["4"]
    moved = c["FX-DIFFUSE-028"]["0"]
    assert moved == c["FX-DIFFUSE-028"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == nineteen[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
