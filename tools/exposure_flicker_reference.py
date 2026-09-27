"""Exposure flicker, worked a second way.

D-125 adds `core.exposure_flicker`. It makes a drawing's brightness jitter from frame to frame,
as a film's exposure flickers: on each frame the whole drawing is brightened or darkened by a
number of stops, the same for every pixel, different from frame to frame. The number is not
random: it is Noise's fixed hash (D-119) of the seed and the frame, so the same file always
flickers the same way. `amount` is 0 to 4 stops (0.25), the most it may move either way;
`hold` is 1 to 100 frames (1), how many frames each brightness lasts, of which only the whole
part counts; `seed` is 0 to 100000 (0), of which only the whole part counts.

The rule. With `frame` the composition frame (a hidden value, as Noise's is),
`m = floor(frame / hold)`, `u = U(seed, m, 0, 0, 3)` (Noise's hash, in -1..1) and
`stops = amount * u`, the output is Exposure's rule: every premultiplied colour channel times
`2^stops`, not clamped, and the covering unchanged. So a pixel that does not show stays as it
is, a soft edge keeps its covering, and at amount 0 nothing changes. The layer does not grow.
It is this program's own method, modelled on After Effects' Exposure with a seeded wiggle of its
stops; nothing is ported. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Noise's card (`tools/noise_reference.py`), imported, not copied. The drawing goes
into `Fixtures/exposure_flicker/media`, the projects into `Fixtures/exposure_flicker`, and the
expected frames into `Fixtures/exposure_flicker/expected_exposure_flicker.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/exposure_flicker_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import noise_reference as N  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "exposure_flicker"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 4), "hold": (1, 100), "seed": (0, 100000)}
ALL = [0, 1, 2, 3, 4]


# --- the rule -------------------------------------------------------------------------------

def stops(amount, hold, seed, frame_no):
    """The frame's stops. `hold` and `seed` are held values, floored here."""
    return amount * N.u(math.floor(seed), frame_no // math.floor(hold), 0, 0, 3)


def flicker(working, amount, hold, seed, frame_no):
    """`working` is linear premultiplied pixels; each colour channel times 2^stops."""
    g = 2 ** stops(amount, hold, seed, frame_no)
    return [[p[0] * g, p[1] * g, p[2] * g, p[3]] for p in working]


# --- the drawing ----------------------------------------------------------------------------

DRAWINGS = N.DRAWINGS  # "card": grey, a skin box in line, a white and a black patch, a soft edge


# --- the cases ------------------------------------------------------------------------------

def case(amount=0.25, hold=1, seed=0, shift=0):
    return {"drawing": "card", "amount": amount, "hold": hold, "seed": seed, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(flicker(pixels, *(held(c, k, frame_no) for k in ("amount", "hold", "seed")),
                           frame_no), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-FLICKER-001": ("The settings as they start, amount 0.25, hold 1, seed 0: every frame has "
                       "its own brightness, every colour channel of every pixel times the same "
                       "factor 2^(0.25 u), between 2^-0.25 and 2^0.25; with seed 0, frames 0 and "
                       "1 are brighter and frames 2, 3 and 4 darker. The covering is kept, the "
                       "soft edge keeps its half covering, and the empty pixels stay empty.",
                       case(), ALL),
    "FX-FLICKER-002": ("Amount 0: the drawing, untouched, on every frame.",
                       case(amount=0), [0, 2, 4]),
    "FX-FLICKER-003": ("Amount 1: the same flicker four times as many stops, so each frame's "
                       "factor is FX-FLICKER-001's to the fourth power.",
                       case(amount=1), ALL),
    "FX-FLICKER-004": ("Amount 4, the top of its range: frame 1 is brightened by about 2.07 "
                       "stops, so the white patch goes past its covering, as Exposure's rule "
                       "does not clamp, and frame 3 is darkened by about 3.44 stops, near black; "
                       "the black patch stays black.",
                       case(amount=4), ALL),
    "FX-FLICKER-005": ("Hold 2: each brightness lasts two frames, so frames 0 and 1 are "
                       "FX-FLICKER-001's frame 0, frames 2 and 3 its frame 1, and frame 4 its "
                       "frame 2.",
                       case(hold=2), ALL),
    "FX-FLICKER-006": ("Hold 3.7: the hold counts as its whole part, 3, so frames 0 to 2 are "
                       "FX-FLICKER-001's frame 0 and frames 3 and 4 its frame 1.",
                       case(hold=3.7), ALL),
    "FX-FLICKER-007": ("Hold 100, the top of its range: all five frames are FX-FLICKER-001's "
                       "frame 0; the brightness is steady.",
                       case(hold=100), ALL),
    "FX-FLICKER-008": ("Seed 7: a different flicker from FX-FLICKER-001's seed 0; frame 0 is "
                       "brighter, frames 1 and 2 darker, frames 3 and 4 brighter.",
                       case(seed=7), ALL),
    "FX-FLICKER-009": ("Seed 7.9: the seed counts as its whole part, so this is FX-FLICKER-008.",
                       case(seed=7.9), [0, 2, 4]),
    "FX-FLICKER-010": ("Seed 100000, the top of its range: a flicker of its own.",
                       case(seed=100000), ALL),
    "FX-FLICKER-011": ("Amount keyed from 0 at frame 0 to 1 at frame 4, linear: frame 0 is the "
                       "drawing, frame 2 is amount 0.5 on its own frame, and frame 4 is "
                       "FX-FLICKER-003's frame 4.",
                       case(amount=keyed((0, 0), (4, 1))), [0, 2, 4]),
    "FX-FLICKER-012": ("Amount keyed from 0 at frame 0 to 4 at frame 4 on a curve that "
                       "overshoots: at frames 2 and 3 it would pass 4, is held at 4, and is "
                       "FX-FLICKER-004's; frame 4, at 4, is FX-FLICKER-004's too.",
                       case(amount=keyed((0, 0, OVERSHOOT), (4, 4))), [0, 2, 3, 4]),
    "FX-FLICKER-013": ("Hold keyed from 1 at frame 0 to 2 at frame 4, linear: at frames 2 and 3 "
                       "the hold is 1.5 and 1.75, which count as 1, so they are FX-FLICKER-001's "
                       "frames 2 and 3; at frame 4 the hold is 2, so frame 4 takes the "
                       "brightness of frame 2, as FX-FLICKER-005's frame 4 does.",
                       case(hold=keyed((0, 1), (4, 2))), [0, 2, 3, 4]),
    "FX-FLICKER-014": ("Seed keyed from 0 at frame 0 to 7.5 at frame 4: frame 0 is "
                       "FX-FLICKER-001's, and frame 4, at 7.5, counts as 7 and is "
                       "FX-FLICKER-008's frame 4.",
                       case(seed=keyed((0, 0), (4, 7.5))), [0, 4]),
    "FX-FLICKER-015": ("FX-FLICKER-001 moved three pixels right: the flicker belongs to the "
                       "frame, not the place, so each frame is FX-FLICKER-001's, moved.",
                       case(shift=3), ALL),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-FLICKER-016": ("Amount 5, above 4.", case(amount=5)),
    "FX-FLICKER-017": ("Amount -0.25, below 0.", case(amount=-0.25)),
    "FX-FLICKER-018": ("Hold 0, below 1.", case(hold=0)),
    "FX-FLICKER-019": ("Hold 101, above 100.", case(hold=101)),
    "FX-FLICKER-020": ("Seed -1, below 0.", case(seed=-1)),
    "FX-FLICKER-021": ("Seed 100001, above 100000.", case(seed=100001)),
    "FX-FLICKER-022": ("Amount keyed to 6 at frame 4.", case(amount=keyed((0, 0.25), (4, 6)))),
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
        "instance_id": "fx-0-0", "type_id": "core.exposure_flicker", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("amount", "hold", "seed")}}]
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

    (OUT / "expected_exposure_flicker.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    grey, soft, white, black = at(1, 1), at(15, 4), at(5, 3), at(9, 3)

    def factor(px, base=drawn):
        """The one factor a frame's colours carry: every channel of every pixel is the drawing's
        times it, the covering is the drawing's, and the empty pixels stay empty."""
        g = px[grey][0] / base[grey][0]
        for p, q in zip(px, base):
            assert p[3] == q[3] and all(abs(a - b * g) <= 1e-12 for a, b in zip(p[:3], q[:3]))
        return g

    # The rule's own pieces.
    u0 = [N.u(0, m, 0, 0, 3) for m in range(5)]
    assert [v > 0 for v in u0] == [True, True, False, False, False]
    assert stops(0.25, 1, 0, 3) == 0.25 * u0[3] and stops(0.25, 2, 0, 3) == 0.25 * u0[1]
    assert stops(0.25, 3.7, 0.9, 4) == 0.25 * u0[1] and stops(0, 1, 0, 2) == 0
    vals = [stops(4, 1, s, f) for s in range(50) for f in range(50)]
    assert all(-4 <= v <= 4 for v in vals) and min(vals) < -3.9 and max(vals) > 3.9

    # Document 25's one-line claim: every frame is the drawing (moved as the case moves it)
    # times 2^stops in red, green and blue, alpha unchanged.
    for fx, (_, cs, _) in CASES.items():
        base = plain(cs)
        for f, px in c[fx].items():
            k = 2 ** stops(*(held(cs, n, int(f)) for n in ("amount", "hold", "seed")), int(f))
            for p, q in zip(px, base):
                assert p[3] == q[3] and all(abs(a - b * k) <= 1e-9 for a, b in zip(p[:3], q[:3])), fx

    one = c["FX-FLICKER-001"]
    g1 = {f: factor(px) for f, px in one.items()}
    assert len(set(g1.values())) == 5 and all(2 ** -0.25 <= g <= 2 ** 0.25 for g in g1.values())
    assert [g > 1 for g in g1.values()] == [True, True, False, False, False]
    assert all(px[soft][3] == 128 / 255 for px in one.values())
    assert all(v == [0.0] * 4 for px in one.values() for v, d in zip(px, drawn) if d[3] == 0)
    assert all(px == drawn for px in c["FX-FLICKER-002"].values())
    for f, px in c["FX-FLICKER-003"].items():
        assert abs(factor(px) - g1[f] ** 4) < 1e-12
    four = c["FX-FLICKER-004"]
    for f, px in four.items():
        assert abs(factor(px) - g1[f] ** 16) < 1e-9
        assert px[black][:3] == [0.0] * 3
    assert four["1"][white][0] > 4 and four["3"][white][0] < 0.1
    five, six, seven = c["FX-FLICKER-005"], c["FX-FLICKER-006"], c["FX-FLICKER-007"]
    assert [five[f] for f in "01234"] == [one[f] for f in "00112"]
    assert [six[f] for f in "01234"] == [one[f] for f in "00011"]
    assert all(px == one["0"] for px in seven.values())
    eight = c["FX-FLICKER-008"]
    assert [factor(eight[f]) > 1 for f in "01234"] == [True, False, False, True, True]
    assert all(eight[f] != one[f] for f in "01234")
    assert all(c["FX-FLICKER-009"][f] == eight[f] for f in "024")
    ten = c["FX-FLICKER-010"]
    assert all(ten[f] != one[f] and ten[f] != eight[f] for f in "01234")
    eleven = c["FX-FLICKER-011"]
    assert eleven["0"] == drawn and eleven["4"] == c["FX-FLICKER-003"]["4"]
    assert abs(factor(eleven["2"]) - g1["2"] ** 2) < 1e-12
    assert eleven["2"] == render(case(amount=0.5), 2)
    twelve = c["FX-FLICKER-012"]
    assert ease(OVERSHOOT, 0.5) > 1 and ease(OVERSHOOT, 0.75) > 1
    assert twelve["0"] == drawn and all(twelve[f] == four[f] for f in "234")
    thirteen = c["FX-FLICKER-013"]
    assert thirteen["0"] == one["0"] and thirteen["2"] == one["2"] and thirteen["3"] == one["3"]
    assert thirteen["4"] == one["2"] == five["4"]
    fourteen = c["FX-FLICKER-014"]
    assert fourteen["0"] == one["0"] and fourteen["4"] == eight["4"]
    moved = c["FX-FLICKER-015"]
    for f in "01234":
        for y in range(H):
            assert moved[f][at(3, y):at(0, y + 1)] == one[f][at(0, y):at(W - 3, y)]
            assert moved[f][at(0, y):at(3, y)] == [[0.0] * 4] * 3
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
