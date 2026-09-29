"""Block Dissolve, worked a second way.

D-214 adds `core.block_dissolve`. It is a transition that makes a layer vanish in random blocks:
the drawing is cut into a grid of rectangles `block_width` by `block_height` pixels, laid from
its top left corner, and as `completion` goes from 0 to 100 the blocks go one by one, in an order
fixed by chance, until at 100 nothing is left. `feather` softens the blocks' edges over that many
pixels. It is After Effects' Block Dissolve (Transition) in purpose and names, and this program's
own rule; its Soft Edges switch is left out. Nothing is ported. Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it.

The rule. At completion 0 the output is the input exactly; at 100 every pixel is transparent, all
four channels 0. Otherwise let L = completion / 100. The block in column i and row j, counting
from 0 at the drawing's top left corner and going on past its edges, is the rectangle
[i bw, (i+1) bw) by [j bh, (j+1) bh), and it has a number r(i, j) = (u + 1) / 2 in 0 up to 1,
where u is D-119's number for seed 0 and the whole numbers (i, j, 0, 0) (Noise's, in
`tools/noise_reference.py`). The block is kept, K = 1, when r >= L, and gone, K = 0, when r < L,
so a block once gone stays gone as the completion rises. With the feather f = 0 a pixel takes
the K of the block holding its centre. With f > 0 it takes the mean of K over the square f on a
side centred on its centre, blocks past the drawing's edges counted as any others. The output is
p k, all four premultiplied channels alike, so the colour is kept and only the covering falls.
The layer does not grow. The block sizes and the feather are distances, so a draft preview
scales them, a block never below 1 pixel.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers; it
finds each mean block by block, where the build reads it from a table of running sums.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Venetian Blinds' window (`tools/venetian_blinds_reference.py`). The drawing goes into
`Fixtures/block_dissolve/media`, the projects into `Fixtures/block_dissolve`, and the expected
frames into `Fixtures/block_dissolve/expected_block_dissolve.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/block_dissolve_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from noise_reference import u  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import venetian_blinds_reference as V  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "block_dissolve"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"completion": (0, 100), "block_width": (1, 10000), "block_height": (1, 10000),
          "feather": (0, 10000)}
START = {"completion": 0, "block_width": 1, "block_height": 1, "feather": 0}
NAMES = tuple(START)
CLIFF = 1e-9   # no block's number may sit closer than this to the level
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def number(i, j):
    """r, the block's place in the order, 0 up to 1."""
    return (u(0, i, j, 0, 0) + 1) / 2


def kept(i, j, level):
    return 1.0 if number(i, j) >= level else 0.0


def share(X, Y, level, bw, bh, feather):
    """k at the pixel centred on (X, Y) of the drawing."""
    if feather == 0:
        return kept(math.floor(X / bw), math.floor(Y / bh), level)
    x0, x1, y0, y1 = X - feather / 2, X + feather / 2, Y - feather / 2, Y + feather / 2
    total = 0.0
    for j in range(math.floor(y0 / bh), math.floor(y1 / bh) + 1):
        oy = min(y1, (j + 1) * bh) - max(y0, j * bh)
        for i in range(math.floor(x0 / bw), math.floor(x1 / bw) + 1):
            ox = min(x1, (i + 1) * bw) - max(x0, i * bw)
            if ox > 0 and oy > 0:
                total += ox * oy * kept(i, j, level)
    return min(1.0, max(0.0, total / (feather * feather)))


def dissolve(layer, completion, block_width, block_height, feather):
    """The layer, `w` by `h` with its top left at drawing coordinates (left, top), dissolved."""
    if completion <= 0:
        return [list(p) for p in layer["px"]]
    if completion >= 100:
        return [list(EMPTY) for _ in layer["px"]]
    level = completion / 100
    out = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            p = layer["px"][y * layer["w"] + x]
            X, Y = x + layer["left"] + 0.5, y + layer["top"] + 0.5
            k = share(X, Y, level, block_width, block_height, feather)
            out.append([v * k for v in p])
    return out


# --- the drawing ----------------------------------------------------------------------------

DRAWINGS = {"window": V.DRAWINGS["window"]}
drawn_layer = V.drawn_layer


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **settings):
    c = {"drawing": "window", "shift": shift}
    c.update({n: settings.get(n, START[n]) for n in NAMES})
    return c


def held(c, name, frame_no):
    lo, hi = RANGES[name]
    return min(hi, max(lo, value_at(c[name], frame_no)))


def render(c, frame_no):
    v = {n: held(c, n, frame_no) for n in NAMES}
    return R.frame(dissolve(drawn_layer(c["drawing"]), **v), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame(drawn_layer(c["drawing"])["px"], c["shift"])


FOUR = {"block_width": 4, "block_height": 4}

CASES = {
    "FX-BDISSOLVE-001": ("The settings as they start, completion 0, blocks 1 by 1, feather 0: "
                         "the drawing, untouched.",
                         case(), [0]),
    "FX-BDISSOLVE-002": ("Completion 50, blocks 1 by 1: each pixel kept exactly or gone, about "
                         "half of those shown gone, scattered.",
                         case(completion=50), [0]),
    "FX-BDISSOLVE-003": ("Completion 50, blocks 4 by 4: each block of four columns and four rows "
                         "from the top left is kept exactly or gone whole; the bottom row of "
                         "blocks is two rows tall, cut by the drawing's edge.",
                         case(completion=50, **FOUR), [0]),
    "FX-BDISSOLVE-004": ("Completion 50, blocks 8 wide by 2 tall: flat bricks, each kept or gone "
                         "whole.",
                         case(completion=50, block_width=8, block_height=2), [0]),
    "FX-BDISSOLVE-005": ("Completion 100, blocks 4 by 4, feather 3: every pixel is transparent, "
                         "the feather too.",
                         case(completion=100, feather=3, **FOUR), [0]),
    "FX-BDISSOLVE-006": ("Completion 25, blocks 4 by 4: fewer blocks gone, and every block gone "
                         "here is gone in FX-BDISSOLVE-003 too.",
                         case(completion=25, **FOUR), [0]),
    "FX-BDISSOLVE-007": ("Completion 50, blocks 4 by 4, feather 2: a pixel whose square of 2 lies "
                         "inside one block is kept exactly or gone, as in FX-BDISSOLVE-003; one "
                         "next to a block of the other kind is part kept, a soft edge.",
                         case(completion=50, feather=2, **FOUR), [0]),
    "FX-BDISSOLVE-008": ("Completion 50, blocks 4 by 4, feather 40: the square is ten blocks "
                         "across, so every shown pixel is kept at nearly the same share, the "
                         "drawing fading almost evenly.",
                         case(completion=50, feather=40, **FOUR), [0]),
    "FX-BDISSOLVE-009": ("Completion keyed from 0 at frame 0 to 100 at frame 4, blocks 4 by 4, "
                         "linear: frame 0 untouched, frame 2 is FX-BDISSOLVE-003, frame 4 is "
                         "empty, and a block once gone never comes back.",
                         case(completion=keyed((0, 0), (4, 100)), **FOUR), [0, 1, 2, 3, 4]),
    "FX-BDISSOLVE-010": ("Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                         "overshoots, blocks 4 by 4: at frame 2 it has gone past 100 and is held "
                         "there, so frames 2 and 4 are both every pixel transparent.",
                         case(completion=keyed((0, 0, OVERSHOOT), (4, 100)), **FOUR), [0, 2, 4]),
    "FX-BDISSOLVE-011": ("Feather keyed from 0 at frame 0 to 4 at frame 4, completion 50, blocks "
                         "4 by 4: frame 0 is FX-BDISSOLVE-003, frame 2 is FX-BDISSOLVE-007, and "
                         "frame 4 softer still.",
                         case(completion=50, feather=keyed((0, 0), (4, 4)), **FOUR), [0, 2, 4]),
    "FX-BDISSOLVE-012": ("Completion 50, blocks 5 wide by 3 tall, which do not fit the drawing "
                         "evenly: the bottom row of blocks is one row tall, cut by the drawing's "
                         "edge.",
                         case(completion=50, block_width=5, block_height=3), [0]),
    "FX-BDISSOLVE-013": ("FX-BDISSOLVE-003 moved three pixels right: the blocks are the "
                         "drawing's own, so it is the same, moved.",
                         case(completion=50, shift=3, **FOUR), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BDISSOLVE-014": ("Completion -1, below 0.", case(completion=-1)),
    "FX-BDISSOLVE-015": ("Completion 100.5, above 100.", case(completion=100.5)),
    "FX-BDISSOLVE-016": ("Block width 0.5, below 1.", case(completion=50, block_width=0.5)),
    "FX-BDISSOLVE-017": ("Block height 10001, above 10000.",
                         case(completion=50, block_height=10001)),
    "FX-BDISSOLVE-018": ("Feather -1, below 0.", case(completion=50, feather=-1)),
    "FX-BDISSOLVE-019": ("Completion keyed to 150 at frame 4.",
                         case(completion=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.block_dissolve", "enabled": True,
        "parameters": {n: setting_json(c[n]) for n in NAMES}}]
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

    (OUT / "expected_block_dissolve.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731
    shown = [i for i in range(W * H) if drawn[i][3] > 0]

    def ks(f, base=drawn):
        """k per shown pixel, read back from the covering."""
        return {i: f[i][3] / base[i][3] for i in range(W * H) if base[i][3] > 0}

    # The numbers: all in 0 up to 1, and spread.
    rs = [number(i, j) for i in range(-20, 20) for j in range(-20, 20)]
    assert all(0 <= r < 1 for r in rs) and 0.4 < sum(r < 0.5 for r in rs) / len(rs) < 0.6

    # Every case scales every pixel by one k in 0..1, all four channels alike, keeps the empty
    # pixels empty, and puts no block's number within 1e-9 of the level.
    for fx, frames in c.items():
        cc = (CASES.get(fx) or INVALID.get(fx))[1]
        base = plain(cc)
        for f, px in frames.items():
            for i in range(W * H):
                b, p = base[i], px[i]
                if b[3] == 0:
                    assert p == EMPTY, (fx, i)
                    continue
                k = p[3] / b[3]
                assert -1e-15 <= k <= 1 + 1e-15, (fx, i)
                assert all(abs(q - r * k) < 1e-15 for q, r in zip(p, b)), (fx, i)
            if fx in CASES:
                v = {n: held(cc, n, int(f)) for n in NAMES}
                level = v["completion"] / 100
                reach = v["feather"] / 2 + 1
                for j in range(math.floor(-reach / v["block_height"]),
                               math.floor((H + reach) / v["block_height"]) + 1):
                    for i in range(math.floor((-reach - cc["shift"]) / v["block_width"]),
                                   math.floor((W + reach) / v["block_width"]) + 1):
                        assert abs(number(i, j) - level) >= CLIFF, (fx, f, i, j)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn

    assert one("FX-BDISSOLVE-001") == drawn
    two = ks(one("FX-BDISSOLVE-002"))
    assert set(two.values()) == {0, 1} and 0.35 < sum(k == 0 for k in two.values()) / len(two) < 0.65

    def by_block(fx, bw, bh):
        """Each block's k, checked the same over every shown pixel of it."""
        blocks = {}
        for i, k in ks(one(fx)).items():
            blocks.setdefault(((i % W) // bw, (i // W) // bh), set()).add(k)
        assert all(len(v) == 1 for v in blocks.values()), fx
        return {b: v.pop() for b, v in blocks.items()}

    three = by_block("FX-BDISSOLVE-003", 4, 4)
    assert set(three.values()) == {0, 1}
    assert len(three) == 12 and {b for b in three if b[1] == 2} == {(i, 2) for i in range(4)}
    four = by_block("FX-BDISSOLVE-004", 8, 2)
    assert set(four.values()) == {0, 1} and len(four) == 10
    assert one("FX-BDISSOLVE-005") == [EMPTY] * (W * H)
    six = by_block("FX-BDISSOLVE-006", 4, 4)
    assert sum(six.values()) > sum(three.values())
    assert all(three[b] == 0 for b, k in six.items() if k == 0)
    seven = ks(one("FX-BDISSOLVE-007"))
    soft = 0
    for i, k in seven.items():
        X, Y = i % W + 0.5, i // W + 0.5
        inside = all(1 <= t % 4 <= 3 for t in (X, Y))
        if inside:
            assert k == three[((i % W) // 4, (i // W) // 4)], i
        soft += 0 < k < 1
    assert soft > 10
    eight = list(ks(one("FX-BDISSOLVE-008")).values())
    assert max(eight) - min(eight) < 0.1 and 0.3 < sum(eight) / len(eight) < 0.7
    nine = c["FX-BDISSOLVE-009"]
    assert nine["0"] == drawn and nine["2"] == one("FX-BDISSOLVE-003")
    assert nine["4"] == [EMPTY] * (W * H) and nine["1"] != drawn and nine["3"] != nine["4"]
    for a, b in zip("0123", "1234"):  # blocks once gone never come back
        assert all(nine[b][i] == EMPTY for i in range(W * H) if nine[a][i] == EMPTY)
    ten = c["FX-BDISSOLVE-010"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert value_at(CASES["FX-BDISSOLVE-010"][1]["completion"], 2) > 100
    assert ten["0"] == drawn and ten["2"] == ten["4"] == [EMPTY] * (W * H)
    eleven = c["FX-BDISSOLVE-011"]
    assert eleven["0"] == one("FX-BDISSOLVE-003") and eleven["2"] == one("FX-BDISSOLVE-007")
    assert sum(0 < k < 1 for k in ks(eleven["4"]).values()) > soft
    twelve = by_block("FX-BDISSOLVE-012", 5, 3)
    assert set(twelve.values()) == {0, 1} and len(twelve) == 12
    assert {b for b in twelve if b[1] == 3} == {(i, 3) for i in range(3)}
    moved, three_px = one("FX-BDISSOLVE-013"), one("FX-BDISSOLVE-003")
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three_px[at(0, y):at(W - 3, y)]
        assert moved[at(0, y):at(3, y)] == [EMPTY] * 3
    assert len(shown) > 0
    print("checked")


if __name__ == "__main__":
    main()
