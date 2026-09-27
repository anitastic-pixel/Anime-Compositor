"""Diffusion, worked a second way.

D-148 adds `core.diffusion`. It is anime compositing's diffusion: a soft glow of the picture laid
over itself, so the lights bloom gently into the darks next to them and the whole cel takes on a
hazy, filmic softness while its lines stay where they are. `radius` is 0 to 500 (10), a
distance: how far the glow spreads. `amount` is 0 to 100 (50), per cent: how much of the glow is
laid on. `blend` is "screen", "lighten" or "normal" (screen): how the glow is laid on. The
covering is kept, so a soft edge stays soft, the glow never spills onto a pixel that does not
show, and a pixel that does not show stays as it is. It is this program's own method, modelled
in spirit on the diffusion filter of anime compositing (the picture blurred and screened over
itself) and on After Effects' soft-glow recipes; nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

The rule. Amount 0 or radius 0: the output is the input, exactly. B is the input through
document 21's Gaussian blur at sigma radius / 3, read at the input's own pixels, anything grown
past them cut off. At a pixel with covering a > 0, with b = p.rgb / a its straight linear colour
and g = B.rgb / B.a when B.a > 0, else b, the glow's own colour (so the transparency round the
drawing does not darken it): f = 1 - (1 - b)(1 - g) (screen), max(b, g) per channel (lighten)
or g (normal); the output is ((b + amount / 100 (f - b)) a, a), not clamped. A pixel with a = 0
is left as it is. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers, and
it blurs with the two-dimensional kernel summed directly (`light_wrap_reference.blurred`, which
is `edges_reference.gaussian` cut back to the picture).

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a cel with a dark shadow holding a small light, a line, skin with a hole in it, a
blue band, and the skin at half covering down its right edge, drawn below. The drawing goes into
`Fixtures/diffusion/media`, the projects into `Fixtures/diffusion`, and the expected frames into
`Fixtures/diffusion/expected_diffusion.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/diffusion_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from light_wrap_reference import blurred  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "diffusion"
TOLERANCE = 2e-5  # document 25's default for a filter
RADIUS, AMOUNT, BLENDS = (0, 500), (0, 100), ("screen", "lighten", "normal")


# --- the rule -------------------------------------------------------------------------------

def laid(b, g, blend):
    """The glow g laid on the straight colour b, per channel, before the amount."""
    if blend == "screen":
        return [1 - (1 - u) * (1 - v) for u, v in zip(b, g)]
    if blend == "lighten":
        return [max(u, v) for u, v in zip(b, g)]
    return list(g)


def glow(px, radius, size=(W, H)):
    """B: the picture through the Gaussian at radius / 3, read at its own pixels."""
    return blurred(px, radius / 3, size)


def diffusion(px, radius, amount, blend, size=(W, H)):
    """Working pixels (linear premultiplied) with the soft glow of themselves laid over."""
    out = [list(p) for p in px]
    if amount == 0 or radius == 0:
        return out  # the build exits early
    for o, q in zip(out, glow(px, radius, size)):
        a = o[3]
        if a <= 0:
            continue
        b = [v / a for v in o[:3]]
        g = [v / q[3] for v in q[:3]] if q[3] > 0 else b
        f = laid(b, g, blend)
        o[:3] = [(u + amount / 100 * (v - u)) * a for u, v in zip(b, f)]
    return out


# --- the drawing ----------------------------------------------------------------------------

SHADOW = (40, 30, 60, 255)       # #281e3c, a deep shadow
LIGHT = (255, 245, 234, 255)     # #fff5ea, the ball's highlight
LINE, SKIN, NONE = R.LINE, R.SKIN, S.NONE
BAND = (58, 111, 216, 255)       # #3a6fd8, the ball's blue band
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge


def cel(x, y):
    """Rows 0 and 9 and column 0 are empty. Between: the shadow in columns 1 to 6 holding the
    light in columns 3 and 4, rows 3 to 5; the line down column 7; skin in columns 8 to 11 with
    a hole at (10, 6); the blue band in columns 12 to 14; the soft skin down column 15."""
    if y in (0, 9) or x == 0 or (x, y) == (10, 6):
        return NONE
    if x <= 6:
        return LIGHT if x in (3, 4) and 3 <= y <= 5 else SHADOW
    return LINE if x == 7 else SKIN if x <= 11 else BAND if x <= 14 else SOFT


DRAWINGS = {"cel": [[cel(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(radius=10, amount=50, blend="screen", shift=0):
    return {"drawing": "cel", "radius": radius, "amount": amount, "blend": blend,
            "shift": shift}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(diffusion(pixels, held(value_at(c["radius"], frame_no), RADIUS),
                             held(value_at(c["amount"], frame_no), AMOUNT), c["blend"]),
                   c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-DIFFUSE-001": ("The settings as they start, radius 10, amount 50, screen: every pixel "
                       "that shows is lightened, never darkened, by half its screen with the "
                       "glow; every pixel of the dark shadow and the line lifts more than any of "
                       "the pale skin or the light, whose red, already full, stays at 1; the "
                       "coverings are kept, and the empty pixels, the hole in the skin among "
                       "them, stay empty.", case(), [0]),
    "FX-DIFFUSE-002": ("Amount 100: the whole screen laid on, every pixel lifted further than "
                       "in FX-DIFFUSE-001, which is exactly halfway between the drawing and "
                       "this.", case(amount=100), [0]),
    "FX-DIFFUSE-003": ("Amount 0: the drawing, untouched.", case(amount=0), [0]),
    "FX-DIFFUSE-004": ("Radius 0: the drawing, untouched.", case(radius=0), [0]),
    "FX-DIFFUSE-005": ("Radius 3, sigma 1: a tight glow. The shadow just left of the light, at "
                       "(2, 4), lifts more than at radius 10, and the shadow in the far corner, "
                       "at (1, 1), less than a fifth as much.", case(radius=3), [0]),
    "FX-DIFFUSE-006": ("Radius 20, sigma 6.67: a glow as wide as the cel, nearly the same "
                       "colour everywhere, so the most-lifted shadow pixel lifts less than 1.3 "
                       "times the least (at radius 10, more than twice).",
                       case(radius=20), [0]),
    "FX-DIFFUSE-007": ("Blend lighten, amount 100: each channel takes the larger of its own "
                       "and the glow's, so the shadow and the line lift, and the light, "
                       "brighter in every channel than the glow round it, stays exactly as it "
                       "is.", case(blend="lighten", amount=100), [0]),
    "FX-DIFFUSE-008": ("Blend normal, amount 100: every pixel that shows takes the glow's own "
                       "colour at its own covering, the picture blurred. The soft edge, where "
                       "the blur covers less than 0.4 as nothing lies to its right, takes the "
                       "colour of what is near it, divided by that covering and so not "
                       "darkened by the emptiness, and the hole stays empty.",
                       case(blend="normal", amount=100), [0]),
    "FX-DIFFUSE-009": ("Blend normal, amount 50: halfway between the drawing and "
                       "FX-DIFFUSE-008, so the light darkens as the shadow lightens.",
                       case(blend="normal"), [0]),
    "FX-DIFFUSE-010": ("Radius keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 is "
                       "the drawing, frame 2 is FX-DIFFUSE-001 and frame 4 is FX-DIFFUSE-006.",
                       case(radius=keyed((0, 0), (4, 20))), [0, 2, 4]),
    "FX-DIFFUSE-011": ("Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: "
                       "frame 0 is the drawing, and at frame 2 it has gone past 100 and is "
                       "held there, so frames 2 and 4 are both FX-DIFFUSE-002.",
                       case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-DIFFUSE-012": ("FX-DIFFUSE-001 moved three pixels right: the same, moved, the glow "
                       "worked on the drawing before it is placed.", case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-DIFFUSE-013": ("Radius 501, above 500.", case(radius=501)),
    "FX-DIFFUSE-014": ("Radius -1, below 0.", case(radius=-1)),
    "FX-DIFFUSE-015": ("Amount 101, above 100.", case(amount=101)),
    "FX-DIFFUSE-016": ("Amount -1, below 0.", case(amount=-1)),
    "FX-DIFFUSE-017": ("Blend \"add\", which is not screen, lighten or normal.",
                       case(blend="add")),
    "FX-DIFFUSE-018": ("Radius keyed to 600 at frame 4.", case(radius=keyed((0, 10), (4, 600)))),
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
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.diffusion", "enabled": True,
        "parameters": {"radius": setting_json(c["radius"]), "amount": setting_json(c["amount"]),
                       "blend": c["blend"]}}]
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

    (OUT / "expected_diffusion.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = [R.working(p) for row in DRAWINGS["cel"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    kind = lambda i: DRAWINGS["cel"][i // W][i % W]  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3] > 0]
    lift = lambda f, i: sum(u - v for u, v in zip(f[i][:3], drawn[i][:3]))  # noqa: E731
    halfway = lambda f, g: like(f, [[(u + v) / 2 for u, v in zip(p, q)]  # noqa: E731
                                    for p, q in zip(drawn, g)])
    straight = lambda f, i: [v / f[i][3] for v in f[i][:3]]  # noqa: E731

    # The rule's own pieces: the three ways of laying the glow on, and a picture of one colour,
    # its edges against nothing, whose glow is its own colour, so normal leaves it as it is.
    assert laid([0.2, 1.0, 0.5], [0.5, 0.3, 0.0], "screen") == [0.6, 1.0, 0.5]
    assert laid([0.2, 1.0, 0.5], [0.5, 0.3, 0.0], "lighten") == [0.5, 1.0, 0.5]
    assert laid([0.2, 1.0, 0.5], [0.5, 0.3, 0.0], "normal") == [0.5, 0.3, 0.0]
    flat = [R.working(SOFT if 3 <= x <= 9 and 2 <= y <= 6 else NONE)
            for y in range(H) for x in range(W)]
    assert like(diffusion(flat, 10, 100, "normal"), flat)
    assert diffusion(px, 0, 100, "screen") == px and diffusion(px, 10, 0, "screen") == px
    q = glow(px, 10)
    assert all(q[i][3] > 0 for i in shows) and q[at(3, 4)] != px[at(3, 4)]

    # Every case keeps every covering and leaves the empty pixels empty; screen and lighten
    # never darken a channel.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-DIFFUSE-012" else 0))
        blend = (CASES.get(fx) or INVALID.get(fx))[1]["blend"]
        for f in frames.values():
            for p, b in zip(f, base):
                assert p[3] == b[3], fx
                if b[3] == 0:
                    assert p == [0.0] * 4, fx
                if blend in ("screen", "lighten"):
                    assert all(u >= v - 1e-15 for u, v in zip(p, b)), fx

    one = c["FX-DIFFUSE-001"]["0"]
    assert [i for i in range(W * H) if one[i] != drawn[i]] == shows
    assert kind(at(10, 6)) == NONE and one[at(10, 6)] == [0.0] * 4
    dark = [lift(one, i) for i in shows if kind(i) in (SHADOW, LINE)]
    pale = [lift(one, i) for i in shows if kind(i) in (SKIN, LIGHT)]
    assert min(dark) > max(pale) > 0
    assert all(one[i][0] == 1.0 for i in shows if kind(i) == LIGHT)
    b, g = px[at(5, 4)][:3], [v / q[at(5, 4)][3] for v in q[at(5, 4)][:3]]
    assert near(one[at(5, 4)][:3], [u + 0.5 * (1 - (1 - u) * (1 - v) - u) for u, v in zip(b, g)])
    two = c["FX-DIFFUSE-002"]["0"]
    assert all(all(u > v for u, v in zip(two[i][:3], one[i][:3])) for i in shows
               if kind(i) != LIGHT)
    assert all(two[i][0] == 1.0 and all(u > v for u, v in zip(two[i][1:3], one[i][1:3]))
               for i in shows if kind(i) == LIGHT)
    assert halfway(one, two)
    assert c["FX-DIFFUSE-003"]["0"] == drawn and c["FX-DIFFUSE-004"]["0"] == drawn
    five = c["FX-DIFFUSE-005"]["0"]
    assert lift(five, at(2, 4)) > lift(one, at(2, 4))
    assert 0 < lift(five, at(1, 1)) < lift(one, at(1, 1)) / 5
    six = c["FX-DIFFUSE-006"]["0"]
    spread = lambda f: max(f) / min(f)  # noqa: E731
    shadow = [i for i in shows if kind(i) == SHADOW]
    assert spread([lift(six, i) for i in shadow]) < 1.3
    assert spread([lift(one, i) for i in shadow]) > 2
    seven = c["FX-DIFFUSE-007"]["0"]
    assert all(seven[i] == drawn[i] for i in shows if kind(i) == LIGHT)
    assert all(seven[i] != drawn[i] for i in shows if kind(i) in (SHADOW, LINE))
    for i in shows:
        g = [v / q[i][3] for v in q[i][:3]]
        assert near(seven[i][:3], [max(u, v) * px[i][3] for u, v in zip(straight(px, i), g)])
    eight = c["FX-DIFFUSE-008"]["0"]
    for i in shows:
        assert near(straight(eight, i), [v / q[i][3] for v in q[i][:3]])
    soft = [at(15, y) for y in range(1, 9)]
    assert all(q[i][3] < 0.4 and kind(i) == SOFT for i in soft)
    assert all(straight(eight, i)[k] > q[i][k] * 2 for i in soft for k in range(3))
    nine = c["FX-DIFFUSE-009"]["0"]
    assert halfway(nine, eight)
    assert all(all(u < v for u, v in zip(nine[i][:3], drawn[i][:3])) for i in shows
               if kind(i) == LIGHT)
    assert all(all(u > v for u, v in zip(nine[i][:3], drawn[i][:3])) for i in shadow)
    ten = c["FX-DIFFUSE-010"]
    assert ten["0"] == drawn and like(ten["2"], one) and like(ten["4"], six)
    eleven = c["FX-DIFFUSE-011"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(amount=keyed(
        (0, 0, OVERSHOOT), (4, 100)))["amount"], 2) > 100
    assert eleven["0"] == drawn and eleven["2"] == two and eleven["4"] == two
    moved = c["FX-DIFFUSE-012"]["0"]
    assert moved == c["FX-DIFFUSE-012"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
        assert moved[at(0, y):at(3, y)] == [[0.0] * 4] * 3
    print("checked")


if __name__ == "__main__":
    main()
