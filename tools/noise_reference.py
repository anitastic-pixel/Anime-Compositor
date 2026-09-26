"""Noise, worked a second way.

D-119 adds `core.noise`, film grain. It moves each shown pixel's colour by a small random amount,
up or down, as the grain of film stock does. The randomness is not random: it is a fixed hash of
the seed, the pixel's place in the drawing's own space, the frame, and the channel, so the same
file always gives the same grain, and a layer that moves carries its grain with it. With
`animate` "on" the grain changes every frame; "off" it is the same on every frame. With `mode`
"mono" the three channels move together, a grey grain; "color" moves each its own way. The
amount moves the straight colour through the sRGB curve by up to half its value either way,
held inside 0 and 1, so white can only darken and black only lighten. The covering is kept, and
a pixel that does not show stays as it is. At amount 0 it changes nothing. It is this program's
own method, modelled on After Effects' Noise; nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/noise/media`, the projects into
`Fixtures/noise`, and the expected frames into `Fixtures/noise/expected_noise.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/noise_reference.py
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

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "noise"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 100), "seed": (0, 100000)}


# --- the rule -------------------------------------------------------------------------------

M = 2 ** 64 - 1


def mix(z):
    """SplitMix64's finaliser, on 64-bit words."""
    z = (z + 0x9E3779B97F4A7C15) & M
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M
    return z ^ (z >> 31)


def u(seed, x, y, f, ch):
    """A number in -1..1 fixed by the seed, the layer-space pixel, the frame and the channel.
    A negative coordinate is its 64-bit two's complement."""
    h = mix(seed & M)
    h = mix(h ^ (x & M))
    h = mix(h ^ (y & M))
    h = mix(h ^ (f & M))
    h = mix(h ^ ch)
    return (h >> 11) / 2 ** 53 * 2 - 1


def noise(pixels, amount, mode, seed, animate, frame_no, width=W):
    """`pixels` is the drawing, `width` wide, row by row, its top-left pixel at (0, 0)."""
    out = [R.working(p) for p in pixels]
    if amount == 0:
        return out  # the build exits early
    seed, f = math.floor(seed), (frame_no if animate == "on" else 0)
    for i, w in enumerate(out):
        a = w[3]
        if a > 0:
            x, y = i % width, i // width
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            n = [u(seed, x, y, f, 0 if mode == "mono" else ch) for ch in range(3)]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, e[ch] + amount / 100 * 0.5 * n[ch]))) * a
                     for ch in range(3)]
    return out


# --- the drawing ----------------------------------------------------------------------------

WHITE = (255, 255, 255, 255)     # to show the grain held at white
BLACK = (0, 0, 0, 255)           # and at black
GREY = (128, 128, 128, 255)      # #808080
SKIN = R.SKIN                    # #f6d6be
LINE = R.LINE                    # #1e1a24
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
NONE = S.NONE


def card(x, y):
    """Column 0 and row 9 are empty; column 15 is the skin at half covering. Rows 0 to 8: a box
    of line in columns 3 to 12 and rows 1 to 7, filled with skin, with a white patch in columns
    5 to 6 and rows 3 to 4 and a black one in columns 9 to 10 and rows 3 to 4, on a grey card."""
    if x == 0 or y == 9:
        return NONE
    if x == 15:
        return SOFT
    if 3 <= x <= 12 and 1 <= y <= 7:
        if x in (3, 12) or y in (1, 7):
            return LINE
        if 3 <= y <= 4 and 5 <= x <= 6:
            return WHITE
        if 3 <= y <= 4 and 9 <= x <= 10:
            return BLACK
        return SKIN
    return GREY


DRAWINGS = {"card": [[card(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(amount=10, mode="mono", seed=0, animate="on", shift=0):
    return {"drawing": "card", "amount": amount, "mode": mode, "seed": seed,
            "animate": animate, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(noise(pixels, held(c, "amount", frame_no), c["mode"],
                         held(c, "seed", frame_no), c["animate"], frame_no), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-NOISE-001": ("The defaults, amount 10, mono, seed 0, animated: every shown pixel's "
                     "three channels move together by up to 0.05 through the sRGB curve, and "
                     "the grain is different on frames 0, 2 and 4; the empty pixels stay empty "
                     "and the soft edge keeps its half covering.",
                     case(), [0, 2, 4]),
    "FX-NOISE-002": ("Animate off: the grain is the same on frames 0, 2 and 4, and is "
                     "FX-NOISE-001's frame 0.",
                     case(animate="off"), [0, 2, 4]),
    "FX-NOISE-003": ("Mode color: the three channels move each their own way; the grey card "
                     "turns speckled with colour.",
                     case(mode="color", animate="off"), [0]),
    "FX-NOISE-004": ("Seed 7: a different grain from FX-NOISE-002's seed 0.",
                     case(seed=7, animate="off"), [0]),
    "FX-NOISE-005": ("Seed 3.",
                     case(seed=3, animate="off"), [0]),
    "FX-NOISE-006": ("Seed 3.7: the seed counts as its whole part, so this is FX-NOISE-005.",
                     case(seed=3.7, animate="off"), [0]),
    "FX-NOISE-007": ("Seed 100000, the top of its range: a grain of its own.",
                     case(seed=100000, animate="off"), [0]),
    "FX-NOISE-008": ("Amount 0: the drawing, untouched.",
                     case(amount=0), [0, 2]),
    "FX-NOISE-009": ("Amount 100, color: channels move by up to 0.5; the white patch can only "
                     "darken, so where the grain would lighten it the channel stays exactly "
                     "white, and the black patch stays exactly black where it would darken.",
                     case(amount=100, mode="color", animate="off"), [0]),
    "FX-NOISE-010": ("Amount keyed from 0 at frame 0 to 40 at frame 4, animate off: frame 0 "
                     "is the drawing; frame 4's grain is frame 2's, twice as strong through "
                     "the sRGB curve.",
                     case(amount=keyed((0, 0), (4, 40)), animate="off"), [0, 2, 4]),
    "FX-NOISE-011": ("Seed keyed from 0 at frame 0 to 3.5 at frame 4, animate off: frame 0 is "
                     "FX-NOISE-002, and frame 4, at 3.5, counts as 3 and is FX-NOISE-005.",
                     case(seed=keyed((0, 0), (4, 3.5)), animate="off"), [0, 4]),
    "FX-NOISE-012": ("FX-NOISE-001 moved three pixels right: the grain is worked in the "
                     "drawing's own space, so it moves with the drawing, frame by frame; each "
                     "frame is FX-NOISE-001's, moved.",
                     case(shift=3), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-NOISE-013": ("Amount 101, above 100.", case(amount=101)),
    "FX-NOISE-014": ("Seed -1, below 0.", case(seed=-1)),
    "FX-NOISE-015": ("Seed 100001, above 100000.", case(seed=100001)),
    "FX-NOISE-016": ("Amount keyed to 150 at frame 4.", case(amount=keyed((0, 10), (4, 150)))),
    "FX-NOISE-017": ("Mode \"Mono\", in capitals, which is kept as written and is not the word.",
                     case(mode="Mono")),
    "FX-NOISE-018": ("Animate \"yes\", which is not a choice.", case(animate="yes")),
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
        "instance_id": "fx-0-0", "type_id": "core.noise", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("amount", "mode", "seed", "animate")}}]
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

    (OUT / "expected_noise.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    step = lambda f, i: [a - b for a, b in zip(enc(f[i]), enc(drawn[i]))]  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    differ = lambda f, g: sum(not near(f[i], g[i]) for i in shown) > len(shown) * 0.9  # noqa: E731
    grey, soft, white, black = at(1, 1), at(15, 4), at(5, 3), at(9, 3)

    # The rule's own pieces.
    assert mix(0) == 0xE220A8397B1DCDAF  # SplitMix64's first output from state 0
    vals = [u(1, x, y, 0, 0) for x in range(64) for y in range(64)]
    assert all(-1 <= v < 1 for v in vals) and abs(sum(vals) / len(vals)) < 0.05
    assert min(vals) < -0.99 and max(vals) > 0.99
    assert u(0, -1, 0, 0, 0) == u(0, M, 0, 0, 0) != u(0, 1, 0, 0, 0)

    one = c["FX-NOISE-001"]
    assert differ(one["0"], one["2"]) and differ(one["2"], one["4"]) and differ(one["0"], one["4"])
    for f in one.values():
        for i in range(W * H):
            if drawn[i][3] == 0:
                assert f[i] == [0.0] * 4
            else:
                assert f[i][3] == drawn[i][3]
                d = step(f, i)
                assert all(abs(v) <= 0.05 + 1e-12 for v in d)
                free = [d[ch] for ch in range(3) if 1e-9 < enc(f[i])[ch] < 1 - 1e-9]
                assert near(free, free[:1] * len(free)), i  # mono: together
        assert f[soft][3] == 128 / 255
    two = c["FX-NOISE-002"]
    assert two["0"] == two["2"] == two["4"] == one["0"]
    three = c["FX-NOISE-003"]["0"]
    d = step(three, grey)
    assert abs(d[0] - d[1]) > 1e-6 and abs(d[1] - d[2]) > 1e-6
    assert near(step(three, grey)[:1], step(two["0"], grey)[:1])  # red is mono's channel 0
    assert differ(c["FX-NOISE-004"]["0"], two["0"])
    assert c["FX-NOISE-006"]["0"] == c["FX-NOISE-005"]["0"]
    assert differ(c["FX-NOISE-005"]["0"], two["0"])
    assert differ(c["FX-NOISE-007"]["0"], two["0"])
    assert c["FX-NOISE-008"]["0"] == drawn == c["FX-NOISE-008"]["2"]
    nine = c["FX-NOISE-009"]["0"]
    whites = [at(x, y) for x in (5, 6) for y in (3, 4)]
    blacks = [at(x, y) for x in (9, 10) for y in (3, 4)]
    kept = lightened = 0
    for i in whites + blacks:
        x, y = i % W, i // W
        for ch in range(3):
            n = u(0, x, y, 0, ch)
            if i in whites:
                assert nine[i][ch] == 1.0 if n >= 0 else nine[i][ch] < 1.0
                kept += n >= 0
            else:
                assert nine[i][ch] == 0.0 if n <= 0 else nine[i][ch] > 0.0
                lightened += n > 0
    assert 0 < kept < 12 and 0 < lightened < 12
    ten = c["FX-NOISE-010"]
    assert ten["0"] == drawn
    for i in shown:
        for ch in range(3):
            if 1e-9 < enc(ten["4"][i])[ch] < 1 - 1e-9:  # not held at black or white
                assert abs(step(ten["4"], i)[ch] - 2 * step(ten["2"], i)[ch]) < 1e-9
    eleven = c["FX-NOISE-011"]
    assert eleven["0"] == two["0"] and eleven["4"] == c["FX-NOISE-005"]["0"]
    moved = c["FX-NOISE-012"]
    for f in ("0", "2", "4"):
        for y in range(H):
            assert moved[f][at(3, y):at(0, y + 1)] == one[f][at(0, y):at(W - 3, y)]
            assert moved[f][at(0, y):at(3, y)] == [[0.0] * 4] * 3
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(0 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
