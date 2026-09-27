"""Sharpen, worked a second way.

D-147 adds `core.sharpen`. It crisps a drawing's edges, as an unsharp mask does to a soft scan
or a slightly blurred render: each colour is pushed away from the blurred colour around it, so a
light side of an edge grows lighter and the dark side darker, and a flat area stays as it is.
`amount` is 0 to 500 (100), per cent: how far the colour is pushed. `radius` is 0 to 100 (1), a
distance: the sigma of the blur the colour is compared with, so how wide the halo along an edge
is. It is this program's own method, modelled on After Effects' Sharpen and Unsharp Mask (the
unsharp mask of the darkroom and of every paint program); nothing is ported. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. Amount 0 or radius 0: the output is the input, exactly. Otherwise B is the input
through document 21's Gaussian blur at sigma `radius`, read at the input's own pixels (what the
blur grows past them is cut off). At a pixel with covering a > 0, with
e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its encoded straight colour and
eb = linear_to_srgb(clamp(B.rgb / B.a, 0, 1)) the blurred colour's (eb = e where B.a = 0):
per channel e'_c = e_c + amount / 100 (e_c - eb_c), and the output is
(srgb_to_linear(clamp(e'_c, 0, 1)) * a, a). A pixel with a = 0 stays as it is. The blurred colour
is divided by its own covering, so the empty pixels around a drawing do not darken it: a drawing's
outline against transparency, or a soft edge of one colour, is not an edge to Sharpen. The layer
does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers, and
it blurs with the two-dimensional kernel summed directly (`edges_reference.gaussian`, through
`light_wrap_reference.blurred`, which cuts the blur back to the picture's own size).

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a patch of skin alone on the left and a box of line with skin and a blue band in
it on the right, drawn below. The drawing goes into `Fixtures/sharpen/media`, the projects into
`Fixtures/sharpen`, and the expected frames into `Fixtures/sharpen/expected_sharpen.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/sharpen_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from light_wrap_reference import blurred  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "sharpen"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("amount", "radius")
RANGES = {"amount": (0, 500), "radius": (0, 100)}


# --- the rule -------------------------------------------------------------------------------

def enc(p):
    """The encoded straight colour of a premultiplied pixel that shows."""
    return [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]


def sharpen(px, amount, radius, size=(W, H)):
    """Working pixels of a picture `size` big, sharpened."""
    out = [list(p) for p in px]
    if amount == 0 or radius == 0:
        return out  # the build exits early
    B = blurred(px, radius, size)
    for o, b in zip(out, B):
        if o[3] > 0:
            e = enc(o)
            eb = enc(b) if b[3] > 0 else e
            o[:3] = [srgb_to_linear(min(1.0, max(0.0, v + amount / 100 * (v - vb)))) * o[3]
                     for v, vb in zip(e, eb)]
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE, SOFT, SKIN, NONE = R.LINE, R.SOFT, R.SKIN, R.NONE
HALF_SKIN = (246, 214, 190, 128)  # the skin at half covering, its soft edge
BAND = (58, 111, 216, 255)        # #3a6fd8, the ball's blue band


def picture(x, y):
    """Rows 0 and 9 are empty. On the left, skin in columns 0 to 2 and the skin at half covering
    in column 3, alone: nothing of another colour within three pixels. Columns 4 to 6 are empty.
    The line at half covering down column 7, rows 3 to 6. In columns 8 to 15 a
    box of line round skin, with a blue band across rows 4 and 5."""
    if y in (0, 9):
        return NONE
    if x <= 2:
        return SKIN
    if x == 3:
        return HALF_SKIN
    if x == 7:
        return SOFT if 3 <= y <= 6 else NONE
    if x < 8:
        return NONE
    if x in (8, 15) or y in (1, 8):
        return LINE
    return BAND if y in (4, 5) else SKIN


DRAWINGS = {"picture": [[picture(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(amount=100, radius=1, shift=0):
    return {"drawing": "picture", "amount": amount, "radius": radius, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(sharpen(pixels, *(held(c, k, frame_no) for k in NAMES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-SHARPEN-001": ("Amount 100 and radius 1, the settings as they start: each colour is "
                       "pushed away from the blurred colour around it, so the two sides of "
                       "every edge inside the box move apart. The line, dark beside the skin, "
                       "is pushed to black all round the box and down the soft column 7; the "
                       "skin beside the line grows lighter, its red held at 1; the skin beside "
                       "the band loses blue, and the band loses red and green and gains blue. "
                       "Every pixel of the box and of the soft line changes; the patch of skin "
                       "on the left, with its soft edge, has no other colour within reach of "
                       "the blur and is left as it is but for rounding, as its outline against "
                       "the empty pixels is not an edge. Every covering is kept, and the empty "
                       "pixels stay empty.",
                       case(), [0]),
    "FX-SHARPEN-002": ("Amount 0: the drawing, untouched.", case(amount=0), [0]),
    "FX-SHARPEN-003": ("Radius 0: the drawing, untouched.", case(radius=0), [0]),
    "FX-SHARPEN-004": ("Amount 50: each colour pushed half as far from its blur as in "
                       "FX-SHARPEN-001, wherever neither is held at 0 or 1.",
                       case(amount=50), [0]),
    "FX-SHARPEN-005": ("Amount 500: each colour pushed five times as far as in FX-SHARPEN-001, "
                       "held at 0 and 1, so the halos burn: the skin along the line turns white, "
                       "the skin beside the band a pale yellow, and the whole band pure blue "
                       "#0000ff; every pixel moves at least as far as in FX-SHARPEN-001, the "
                       "same way.",
                       case(amount=500), [0]),
    "FX-SHARPEN-006": ("Radius 0.5: a blur about one pixel wide, so each colour is compared "
                       "only with its nearest neighbours; the left patch is still left as it "
                       "is but for rounding.", case(radius=0.5), [0]),
    "FX-SHARPEN-007": ("Radius 3: a blur wide enough to reach from the left patch into the "
                       "box, so the skin of the patch, its blur now darkened by the line, "
                       "grows lighter too, every pixel of it, most at its soft edge; the "
                       "covering is kept.", case(radius=3), [0]),
    "FX-SHARPEN-008": ("Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 "
                       "untouched, frame 2 amount 100, FX-SHARPEN-001, and frame 4 amount 200.",
                       case(amount=keyed((0, 0), (4, 200))), [0, 2, 4]),
    "FX-SHARPEN-009": ("Radius keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 "
                       "untouched, frame 2 radius 1, FX-SHARPEN-001, and frame 4 radius 2, "
                       "whose blur reaches the left patch.",
                       case(radius=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-SHARPEN-010": ("Amount eased from 0 at frame 0 to 500 at frame 4 on a curve that "
                       "overshoots: at frame 2 it has gone past 500 and is held there, so "
                       "frames 2 and 4 are both FX-SHARPEN-005.",
                       case(amount=keyed((0, 0, OVERSHOOT), (4, 500))), [0, 2, 4]),
    "FX-SHARPEN-011": ("FX-SHARPEN-001 moved three pixels right: the same, moved.",
                       case(shift=3), [0, 3]),
    "FX-SHARPEN-012": ("Amount 250 and radius 1.5, a strong crisp: every pixel of the box "
                       "pushed at least as far as in FX-SHARPEN-001, and the wider blur just "
                       "reaches the patch's right side, faintly: its soft edge in column 3 "
                       "grows up to about two levels lighter and column 2 under a quarter of a "
                       "level, while columns 0 and 1 are left as they are but for rounding.",
                       case(amount=250, radius=1.5), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SHARPEN-013": ("Amount 501, above 500.", case(amount=501)),
    "FX-SHARPEN-014": ("Amount -1, below 0.", case(amount=-1)),
    "FX-SHARPEN-015": ("Radius 101, above 100.", case(radius=101)),
    "FX-SHARPEN-016": ("Radius -0.5, below 0.", case(radius=-0.5)),
    "FX-SHARPEN-017": ("Amount keyed to 600 at frame 4.", case(amount=keyed((0, 100), (4, 600)))),
    "FX-SHARPEN-018": ("Radius keyed to -2 at frame 4.", case(radius=keyed((0, 1), (4, -2)))),
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
        "instance_id": "fx-0-0", "type_id": "core.sharpen", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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

    (OUT / "expected_sharpen.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    shows = [p[3] > 0 for p in drawn]
    box = [at(x, y) for x in range(8, 16) for y in range(1, 9)]
    col7 = [at(7, y) for y in range(3, 7)]
    patch = [at(x, y) for x in range(4) for y in range(1, 9)]
    inside = lambda v: 1e-9 < v < 1 - 1e-9  # noqa: E731

    def push(f, i):
        """How far each encoded channel of pixel i moved from the drawing's."""
        return [u - v for u, v in zip(enc(f[i]), enc(drawn[i]))]

    # The rule's own pieces: a flat colour and its blur are the same colour whatever the
    # covering around it, so it is left alone; amount or radius 0 leave the input exactly.
    flat = [R.working(SKIN)] * 4 + [R.working(HALF_SKIN)] * 2 + [[0.0] * 4] * 2
    assert like(sharpen(flat, 300, 1, (4, 2)), flat)
    assert sharpen(drawn, 0, 5) == drawn and sharpen(drawn, 100, 0) == drawn
    B = blurred(drawn, 1)
    assert near(enc(B[at(1, 4)]), enc(drawn[at(1, 4)])) and B[at(1, 4)][3] < 1  # covering apart
    assert enc(B[at(9, 2)])[0] < enc(drawn[at(9, 2)])[0]  # the line darkens the skin's blur

    # Every case: the covering kept, the empty pixels empty, every channel within its covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-SHARPEN-011" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)

    one = c["FX-SHARPEN-001"]["0"]
    moved = [i for i in range(W * H) if not near(one[i], drawn[i])]
    assert sorted(moved) == sorted(box + col7)
    assert all(near(one[i], drawn[i]) for i in patch)
    assert all(u <= 1e-12 for u in push(one, at(8, 3)))  # the line beside the skin, darker
    assert all(u >= -1e-12 for u in push(one, at(9, 2)))  # the skin inside the corner, lighter
    assert abs(enc(one[at(9, 3)])[0] - 1) < 1e-12  # the skin's red held at 1
    lines = [at(x, y) for x in range(8, 16) for y in range(1, 9) if x in (8, 15) or y in (1, 8)]
    assert all(enc(one[i]) == [0.0] * 3 for i in lines + col7)  # the line, black
    s3, b4 = push(one, at(11, 3)), push(one, at(11, 4))  # the skin and band either side
    assert s3[0] > 0 and s3[1] > 0 and s3[2] < 0 and b4[0] < 0 and b4[1] < 0 and b4[2] > 0
    for n in ("002", "003"):
        assert c["FX-SHARPEN-" + n]["0"] == drawn
    half, five = c["FX-SHARPEN-004"]["0"], c["FX-SHARPEN-005"]["0"]
    for i in box + col7:
        e1, eh, e5 = enc(one[i]), enc(half[i]), enc(five[i])
        for ch, (d1, dh, d5) in enumerate(zip(push(one, i), push(half, i), push(five, i))):
            if inside(e1[ch]) and inside(eh[ch]):
                assert abs(dh - d1 / 2) < 1e-9, (i, ch)
            if inside(e5[ch]):
                assert abs(d5 - 5 * d1) < 1e-9, (i, ch)
            assert abs(d5) >= abs(d1) - 1e-12 and d5 * d1 >= -1e-24, (i, ch)
    rim = [at(x, y) for x in range(9, 15) for y in range(2, 8)
           if (x in (9, 14) or y in (2, 7)) and y not in (4, 5)]  # skin along the line
    assert all(near(enc(five[i]), [1.0] * 3) for i in rim)
    assert all(near(enc(five[at(x, y)])[:2], [1.0, 1.0]) and enc(five[at(x, y)])[2] < 0.8
               for x in range(10, 14) for y in (3, 6))  # the skin beside the band, pale yellow
    assert all(near(enc(five[at(x, y)]), [0.0, 0.0, 1.0]) for x in range(9, 15) for y in (4, 5))
    six = c["FX-SHARPEN-006"]["0"]
    assert all(near(six[i], drawn[i]) for i in patch)
    seven = c["FX-SHARPEN-007"]["0"]
    assert all(u > 0 for i in patch for u in push(seven, i))
    assert all(push(seven, at(3, y))[1] > push(seven, at(0, y))[1] for y in range(1, 9))
    eight = c["FX-SHARPEN-008"]
    assert eight["0"] == drawn and like(eight["2"], one)
    assert like(eight["4"], render(case(amount=200), 0))
    nine = c["FX-SHARPEN-009"]
    assert nine["0"] == drawn and like(nine["2"], one)
    assert like(nine["4"], render(case(radius=2), 0))
    assert any(not near(nine["4"][i], drawn[i]) for i in patch)
    ten = c["FX-SHARPEN-010"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(amount=keyed(
        (0, 0, OVERSHOOT), (4, 500)))["amount"], 2) > 500
    assert ten["0"] == drawn and ten["2"] == five and ten["4"] == five
    shifted = c["FX-SHARPEN-011"]["0"]
    assert shifted == c["FX-SHARPEN-011"]["3"]
    for y in range(H):
        assert shifted[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
        assert shifted[at(0, y):at(3, y)] == [[0.0] * 4] * 3
    twelve = c["FX-SHARPEN-012"]["0"]
    for i in box + col7:
        for d1, d12 in zip(push(one, i), push(twelve, i)):
            assert abs(d12) >= abs(d1) - 1e-12, i
    assert all(near(twelve[at(x, y)], drawn[at(x, y)]) for x in range(2) for y in range(1, 9))
    col2 = [u for y in range(1, 9) for u in push(twelve, at(2, y))]
    col3 = [u for y in range(1, 9) for u in push(twelve, at(3, y))]
    assert 0 < min(col2) and max(col2) < 0.25 / 255 and 0 < min(col3) and max(col3) < 2.5 / 255
    assert shows == [p[3] > 0 for p in twelve]
    print("checked")


if __name__ == "__main__":
    main()
