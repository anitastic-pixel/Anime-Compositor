"""Scatter, worked a second way.

D-465 adds `core.scatter`, after After Effects' Scatter (Stylize): the layer's pixels jumbled,
each moved a random distance within the Scatter Amount, without changing any colour. What is
known (Qt 3D Studio's "Stylize Scatter Effect" page and Qt Quick 3D's Scatter type, which carry
After Effects' control names; Adobe's own page could not be read) is: Scatter Amount sets how far
pixels move, Qt's port taking 0 to 127 and starting at 10; Grain moves them both ways,
horizontally only or vertically only; and Randomize Every Frame draws a new jumble each frame.
The rule below is this program's own, and nothing is ported: the hash, the even spread of
whole-pixel moves and the layer not growing are ours.

The rule. The input as it reaches the effect is w by h pixels, the pixels an earlier effect grew
included; the output is the same size and sits where the input did.

1. The reach is n = floor(a + 1/2) whole pixels, a the Scatter Amount (draft-scaled). With n 0
   the output is the input.
2. The frame number used is f, the composition frame when Randomize Every Frame is on, else 0.
3. For output pixel (x, y), counted from the input's corner, the hash of (x, y, f, k), k 0 for
   across and 1 for down, each as 32 unsigned bits: t = x 0x9E3779B1 xor y 0x85EBCA77 xor f
   0xC2B2AE3D xor k 0x27D4EB2F, t = t xor (t >> 15), t = t 0x2C1B3C6D, t = t xor (t >> 12), t = t
   0x297A2D39, t = t xor (t >> 15), all kept to 32 bits.
4. The move across is dx = (t(k = 0) mod (2 n + 1)) - n, 0 when Grain is Vertical; the move down
   is dy = (t(k = 1) mod (2 n + 1)) - n, 0 when Grain is Horizontal. Each is a whole number from
   -n to n.
5. The output pixel is the input pixel at (x + dx, y + dy), exactly, or clear where that lies
   outside the input. Every choice is made from whole numbers, so nothing sits on a cliff, and
   no pixel is mixed: the layer's edge frays inwards and nothing grows past it.

Settings: `scatter_amount` 0 to 127 pixels, 10 when added, scaled with a draft as the other
distances are; `grain` "both", "horizontal" or "vertical", "both"; `randomize` "on" or "off",
"off". Words are kept as written, so a wrong one is reported. The amount can be keyed.

**This file never runs the build's code path.** It works on lists, straight from the drawing's
8-bit values.

Every case is a composition 16 pixels by 10 holding Motion Tile's drawing, 6 pixels by 4, turned
neither way alike, its top left corner at (5, 3), unmoved unless the case says. The drawing goes
into `Fixtures/scatter/media`, the projects into `Fixtures/scatter`, and the expected frames into
`Fixtures/scatter/expected_scatter.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/scatter_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import drop_shadow, frame  # noqa: E402
from motion_tile_reference import DRAWINGS, DW, DH, SHADOW, drawn_layer  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "scatter"
TOLERANCE = 2e-5  # document 25's default for a filter
EMPTY = [0.0] * 4
GRAINS = ("both", "horizontal", "vertical")
NAMES = ("scatter_amount", "grain", "randomize")
WORDS = ("grain", "randomize")
M32 = 0xFFFFFFFF


# --- the rule -------------------------------------------------------------------------------

def reach(a):
    return math.floor(a + 0.5)


def scatter_hash(x, y, f, k):
    t = ((x & M32) * 0x9E3779B1 ^ (y & M32) * 0x85EBCA77 ^ (f & M32) * 0xC2B2AE3D
         ^ k * 0x27D4EB2F) & M32
    t ^= t >> 15
    t = (t * 0x2C1B3C6D) & M32
    t ^= t >> 12
    t = (t * 0x297A2D39) & M32
    t ^= t >> 15
    return t


def move(n, grain, x, y, f):
    span = 2 * n + 1
    dx = 0 if grain == "vertical" else scatter_hash(x, y, f, 0) % span - n
    dy = 0 if grain == "horizontal" else scatter_hash(x, y, f, 1) % span - n
    return dx, dy


def scatter(layer, s, frame_no):
    w, h, src = layer["w"], layer["h"], layer["px"]
    n = reach(s["scatter_amount"])
    if n == 0:
        return layer
    f = frame_no if s["randomize"] == "on" else 0
    out = []
    for y in range(h):
        for x in range(w):
            dx, dy = move(n, s["grain"], x, y, f)
            u, v = x + dx, y + dy
            out.append(list(src[v * w + u]) if 0 <= u < w and 0 <= v < h else list(EMPTY))
    return {**layer, "px": out}


# --- the cases ------------------------------------------------------------------------------

def case(scatter_amount=10, grain="both", randomize="off", shift=0, shadow=False):
    c = dict(locals())
    c["drawing"] = "patch"
    return c


def numbers(c, frame_no):
    return {k: (c[k] if k in WORDS else min(127, max(0, value_at(c[k], frame_no))))
            for k in NAMES}


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    if c["shadow"]:
        layer = drop_shadow(layer, SHADOW["color"], SHADOW["opacity"], SHADOW["direction"],
                            SHADOW["distance"], SHADOW["softness"])
    return layer


def render(c, frame_no):
    return frame(scatter(layer_of(c), numbers(c, frame_no), frame_no), c["shift"])


def plain(c):
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-SCATTER-001": ("The settings as they start: Scatter Amount 10, both ways, the same jumble "
                       "every frame. On a 6 by 4 drawing most pixels read from outside it and "
                       "are clear.", case(), [0]),
    "FX-SCATTER-002": ("Scatter Amount 1, both ways: each pixel from one of its nine "
                       "neighbours or itself.", case(1), [0]),
    "FX-SCATTER-003": ("Scatter Amount 2, both ways.", case(2), [0]),
    "FX-SCATTER-004": ("Scatter Amount 2, Grain Horizontal: every pixel stays in its own row.",
                       case(2, "horizontal"), [0]),
    "FX-SCATTER-005": ("Scatter Amount 2, Grain Vertical: every pixel stays in its own column.",
                       case(2, "vertical"), [0]),
    "FX-SCATTER-006": ("Scatter Amount 0: the picture as it was.", case(0), [0]),
    "FX-SCATTER-007": ("Scatter Amount 0.4, which rounds to no reach: the picture as it was.",
                       case(0.4), [0]),
    "FX-SCATTER-008": ("Scatter Amount 0.5, which rounds to a reach of 1: FX-SCATTER-002.",
                       case(0.5), [0]),
    "FX-SCATTER-009": ("Scatter Amount 2, Randomize Every Frame on: a new jumble each frame; "
                       "frame 0 is FX-SCATTER-003.", case(2, randomize="on"), [0, 1, 2]),
    "FX-SCATTER-010": ("Scatter Amount 2, Randomize Every Frame off: frame 3 is frame 0.",
                       case(2), [0, 3]),
    "FX-SCATTER-011": ("Scatter Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame "
                       "1 is 1, frame 2 is 2 and FX-SCATTER-003.",
                       case(keyed((0, 0), (4, 4))), [0, 1, 2, 4]),
    "FX-SCATTER-012": ("Scatter Amount keyed from 100 at frame 0 to 127 at frame 4, eased past "
                       "its end: frame 2 would pass 127, is held at 127, and is frame 4.",
                       case(keyed((0, 100, OVERSHOOT), (4, 127))), [0, 2, 4]),
    "FX-SCATTER-013": ("FX-SCATTER-003 moved three pixels right: the jumble moves with the "
                       "drawing.", case(2, shift=3), [0]),
    "FX-SCATTER-014": ("After a drop shadow that grows the layer to 10 by 8, Scatter Amount 1: "
                       "the shadow is jumbled with the drawing.", case(1, shadow=True), [0]),
}

INVALID = {
    "FX-SCATTER-015": ("Scatter Amount -1, below 0.", case(-1)),
    "FX-SCATTER-016": ("Scatter Amount 128, above 127.", case(128)),
    "FX-SCATTER-017": ("Grain \"diagonal\", which is not one.", case(grain="diagonal")),
    "FX-SCATTER-018": ("Randomize Every Frame \"sometimes\", which is not one.",
                       case(randomize="sometimes")),
    "FX-SCATTER-019": ("Scatter Amount keyed to 200 at frame 4, above 127.",
                       case(keyed((0, 0), (4, 200)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([DW / 2, DH / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["shadow"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
                        "parameters": dict(SHADOW)})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.scatter",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    comp["layers"][0]["effects"] = effects
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
    (OUT / "expected_scatter.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    one = lambda fx: c[fx]["0"]  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    layer = drawn_layer("patch")
    d = lambda x, y: layer["px"][y * DW + x]  # noqa: E731
    art = plain(case())
    LEFT, TOP = layer["left"], layer["top"]

    # The moves are spread over every whole number in reach, and the two directions differ.
    for n in (1, 2, 5):
        seen = {move(n, "both", x, y, 0) for x in range(40) for y in range(40)}
        assert seen == {(a, b) for a in range(-n, n + 1) for b in range(-n, n + 1)}, n
    assert move(3, "horizontal", 4, 7, 0)[1] == 0 and move(3, "vertical", 4, 7, 0)[0] == 0
    assert move(3, "both", 4, 7, 0) != move(3, "both", 4, 7, 1)
    assert reach(0.4) == 0 and reach(0.5) == 1 and reach(127) == 127

    # Nothing is mixed: every pixel is the drawing's, the shadow's or empty.
    pixels = layer["px"] + [EMPTY]
    for fx, frames in c.items():
        if fx == "FX-SCATTER-014":
            continue
        for px in frames.values():
            assert all(p in pixels for p in px), fx
            # Nothing past the drawing's own place.
            assert all(px[at(x, y)] == EMPTY for x in range(W) for y in range(H)
                       if not (LEFT + c_shift(fx) <= x < LEFT + c_shift(fx) + DW
                               and TOP <= y < TOP + DH)), fx

    # Each pixel comes from within reach of its place: checked against the rule's own move.
    for fx, n, grain in (("FX-SCATTER-002", 1, "both"), ("FX-SCATTER-003", 2, "both"),
                         ("FX-SCATTER-004", 2, "horizontal"), ("FX-SCATTER-005", 2, "vertical")):
        px = one(fx)
        for y in range(DH):
            for x in range(DW):
                dx, dy = move(n, grain, x, y, 0)
                assert abs(dx) <= n and abs(dy) <= n
                want = d(x + dx, y + dy) if 0 <= x + dx < DW and 0 <= y + dy < DH else EMPTY
                assert px[at(LEFT + x, TOP + y)] == want, fx
        assert px != art, fx
    assert one("FX-SCATTER-003") != one("FX-SCATTER-002")
    assert len({json.dumps(one(f"FX-SCATTER-00{n}")) for n in (3, 4, 5)}) == 3
    # Horizontal keeps each row's pixels in that row; vertical each column's in that column.
    hz, vt = one("FX-SCATTER-004"), one("FX-SCATTER-005")
    for y in range(DH):
        row = [d(x, y) for x in range(DW)] + [EMPTY]
        assert all(hz[at(LEFT + x, TOP + y)] in row for x in range(DW))
    for x in range(DW):
        col = [d(x, y) for y in range(DH)] + [EMPTY]
        assert all(vt[at(LEFT + x, TOP + y)] in col for y in range(DH))
    # The default reach of 10 leaves more than half the drawing clear.
    first = one("FX-SCATTER-001")
    assert sum(first[at(LEFT + x, TOP + y)] == EMPTY for x in range(DW) for y in range(DH)) > 12
    assert one("FX-SCATTER-006") == one("FX-SCATTER-007") == art
    assert one("FX-SCATTER-008") == one("FX-SCATTER-002")
    k9 = c["FX-SCATTER-009"]
    assert k9["0"] == one("FX-SCATTER-003") and len({json.dumps(v) for v in k9.values()}) == 3
    k10 = c["FX-SCATTER-010"]
    assert k10["0"] == k10["3"] == one("FX-SCATTER-003")
    k11 = c["FX-SCATTER-011"]
    assert k11["0"] == art and k11["1"] == one("FX-SCATTER-002")
    assert k11["2"] == one("FX-SCATTER-003") and k11["4"] != k11["2"]
    k12 = c["FX-SCATTER-012"]
    raw = value_at(CASES["FX-SCATTER-012"][1]["scatter_amount"], 2)
    assert raw > 127 and k12["2"] == k12["4"]
    moved = one("FX-SCATTER-013")
    three = one("FX-SCATTER-003")
    assert all(moved[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    shadowed = layer_of(case(shadow=True))
    assert (shadowed["left"], shadowed["top"], shadowed["w"], shadowed["h"]) == (3, 1, 10, 8)
    sp = shadowed["px"] + [EMPTY]
    sh = one("FX-SCATTER-014")
    assert sh != frame(shadowed, 0) and all(p in sp for p in sh)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


def c_shift(fx):
    return CASES[fx][1]["shift"] if fx in CASES else 0


if __name__ == "__main__":
    main()
