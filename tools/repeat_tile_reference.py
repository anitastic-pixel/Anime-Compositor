"""Repeat Tile, worked a second way.

D-464 adds `core.repeat_tile`, after After Effects' CC RepeTile (Stylize): the layer grown by a
number of pixels on each side and the new room filled with copies of the picture, laid one of
sixteen ways, with the seams between copies optionally blended away. What is known (the
CycoreFX HD 1.8.1 manual's page on CC RepeTile, and the "Blend patterns with RepeTile" lesson of
After Effects for Photography) is: Expand Right, Left, Down and Up are how far the layer grows
each way, in pixels; Tiling chooses Repeat, Checker Flip H, Checker Flip V, Unfold, Checker 180,
Checker Flip 45, Checker 90 CW, Checker 90 CCW, Rosette, Random, None, Turn CW, Turn CCW, Twist,
Slide or Brick; and Blend Borders softens the seams, and does nothing under Unfold, None or Slide.
Slide repeats the edge pixels outwards, as Photoshop's offset filter can. The rule below is this
program's own, and nothing is ported; what Random, Turn CW, Turn CCW and Twist lay where is our
reading of their names, and a copy turned a quarter round is squeezed to the tile's own shape.

The rule. The input as it reaches the effect is w by h pixels, the pixels an earlier effect grew
included, as Motion Tile's (D-154) is; that whole input is one tile.

1. Each expansion is made a whole number of pixels by rounding up: l, r, u and d. The output is
   w + 2 gx by h + 2 gy, gx = max(l, r) and gy = max(u, d), its origin moved by (gx, gy) so the
   picture stays where it was; the layer grows the same both ways, as Power Pin's (D-389) does.
2. Output pixel (X, Y) is at x = X - gx, y = Y - gy from the input's corner. Outside -l <= x <
   w + r and -u <= y < h + d it is clear.
3. Slide: the input pixel at (x, y) held inside it, clamp(x, 0, w - 1) and clamp(y, 0, h - 1).
   None: the input pixel at (x, y) inside the input, clear outside it. Every other: the tile i =
   floor(x / w) across and j = floor(y / h) down, Brick first moving every odd row of tiles half
   a tile right (x - floor(w / 2) when j is odd), the place in it lx = x - i w, ly = y - j h, and
   the tile's way o, a number 0 to 7 whose 1 bit flips across, 2 bit flips down and 4 bit swaps
   across and down: (u, v) = (lx, ly), or with 4 set (q(ly, h, w), q(lx, w, h)), q(t, n, m) =
   floor((2 t + 1) m / (2 n)), the place scaled from one side to the other; then u = w - 1 - u
   with 1 set and v = h - 1 - v with 2 set. So 0 is the picture, 1 flipped across, 2 flipped down,
   3 turned half round, 4 flipped on its diagonal, 5 turned a quarter anticlockwise, 6 a quarter
   clockwise and 7 flipped on the other diagonal. The way for each Tiling, with p = (i + j) mod 2
   and m the floor-based remainder:
   Repeat and Brick 0; Unfold (i mod 2) + 2 (j mod 2); Checker Flip H, Checker Flip V, Checker
   180, Checker Flip 45, Checker 90 CW and Checker 90 CCW 1, 2, 3, 4, 6 and 5 when p is 1, else 0;
   Rosette 0, 6, 3 and 5 for (i mod 2, j mod 2) (0, 0), (1, 0), (1, 1) and (0, 1); Turn CW, Turn
   CCW and Twist a number of quarter turns clockwise, (i + j), -(i + j) and (i - j) mod 4, 0 to 3
   being ways 0, 6, 3 and 5; Random the hash t of i and j, each as 32 unsigned bits, t = i
   0x9E3779B1 xor j 0x85EBCA77, t = t xor (t >> 15), t = t 0x2C1B3C6D, t = t xor (t >> 12), all
   kept to 32 bits, and the way t mod 8.
4. Blend Borders (b = blend_borders / 100), except under Unfold, None and Slide: with (u, v) the
   input pixel step 3 chose, the pixel half a tile on, u' = (u + floor(w / 2)) mod w and v' = (v +
   floor(h / 2)) mod h, is mixed in towards the tile's edges: wx = clamp(1 - min(u + 1/2, w - u -
   1/2) / (b w / 2), 0, 1), wy likewise from v and h, each 0 when b is 0, and the output is (1 -
   wy) ((1 - wx) P(u, v) + wx P(u', v)) + wy ((1 - wx) P(u, v') + wx P(u', v')), all four channels.
   Each side of a seam is then mostly the picture's middle, which runs on across it.
5. Without blending the output is the input pixel at (u, v), exactly. Every choice is made from
   whole numbers, so nothing sits on a cliff.

All four expansions 0, and Blend Borders 0 or a Tiling that ignores it: the output is the input.

Settings: `expand_right`, `expand_left`, `expand_down` and `expand_up` 0 to 4000 pixels, 0 when
added, scaled with a draft as the other distances are; `tiling` one of the sixteen words below,
"unfold" when added; `blend_borders` 0 to 100 per cent, 0. Words are kept as written, so a wrong
one is reported. The numbers can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Motion Tile's drawing, 6 pixels by 4, turned
neither way alike, its top left corner at (5, 3), unmoved unless the case says. The drawing goes
into `Fixtures/repeat_tile/media`, the projects into `Fixtures/repeat_tile`, and the expected
frames into `Fixtures/repeat_tile/expected_repeat_tile.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/repeat_tile_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "repeat_tile"
TOLERANCE = 2e-5  # document 25's default for a filter
EMPTY = [0.0] * 4
TILINGS = ("repeat", "checker_flip_h", "checker_flip_v", "unfold", "checker_180",
           "checker_flip_45", "checker_90_cw", "checker_90_ccw", "rosette", "random", "none",
           "turn_cw", "turn_ccw", "twist", "slide", "brick")
NO_BLEND = ("unfold", "none", "slide")
EXPANDS = ("expand_right", "expand_left", "expand_down", "expand_up")
RANGES = {**{k: (0, 4000) for k in EXPANDS}, "blend_borders": (0, 100)}
NAMES = EXPANDS + ("tiling", "blend_borders")
QUARTERS = (0, 6, 3, 5)
CHECKER = {"checker_flip_h": 1, "checker_flip_v": 2, "checker_180": 3, "checker_flip_45": 4,
           "checker_90_cw": 6, "checker_90_ccw": 5}
M32 = 0xFFFFFFFF


# --- the rule -------------------------------------------------------------------------------

def growth(n):
    l, r, u, d = (math.ceil(n[k]) for k in ("expand_left", "expand_right", "expand_up",
                                            "expand_down"))
    return (l, r, u, d), max(l, r), max(u, d)


def random_way(i, j):
    t = ((i & M32) * 0x9E3779B1 ^ (j & M32) * 0x85EBCA77) & M32
    t ^= t >> 15
    t = (t * 0x2C1B3C6D) & M32
    t ^= t >> 12
    return t % 8


def way(tiling, i, j):
    p = (i + j) % 2
    if tiling in CHECKER:
        return CHECKER[tiling] if p else 0
    if tiling == "unfold":
        return i % 2 + 2 * (j % 2)
    if tiling == "rosette":
        return {(0, 0): 0, (1, 0): 6, (1, 1): 3, (0, 1): 5}[(i % 2, j % 2)]
    if tiling == "turn_cw":
        return QUARTERS[(i + j) % 4]
    if tiling == "turn_ccw":
        return QUARTERS[-(i + j) % 4]
    if tiling == "twist":
        return QUARTERS[(i - j) % 4]
    if tiling == "random":
        return random_way(i, j)
    return 0  # repeat, brick


def q(t, n, m):
    return (2 * t + 1) * m // (2 * n)


def oriented(o, lx, ly, w, h):
    u, v = (q(ly, h, w), q(lx, w, h)) if o & 4 else (lx, ly)
    if o & 1:
        u = w - 1 - u
    if o & 2:
        v = h - 1 - v
    return u, v


def chosen(n, x, y, w, h):
    """The input pixel output place (x, y), from the input's corner, reads, or None."""
    (l, r, u, d), _, _ = growth(n)
    if not (-l <= x < w + r and -u <= y < h + d):
        return None
    t = n["tiling"]
    if t == "slide":
        return min(max(x, 0), w - 1), min(max(y, 0), h - 1)
    if t == "none":
        return (x, y) if 0 <= x < w and 0 <= y < h else None
    j = y // h
    if t == "brick" and j % 2:
        x -= w // 2
    i = x // w
    return oriented(way(t, i, j), x - i * w, y - j * h, w, h)


def repeat_tile(layer, n):
    w, h, src = layer["w"], layer["h"], layer["px"]
    _, gx, gy = growth(n)
    b = n["blend_borders"] / 100 if n["tiling"] not in NO_BLEND else 0
    at = lambda u, v: src[v * w + u]  # noqa: E731

    def weight(t, size):
        if b == 0:
            return 0.0
        return min(max(1 - min(t + 0.5, size - t - 0.5) / (b * size / 2), 0.0), 1.0)

    out = []
    for yy in range(h + 2 * gy):
        for xx in range(w + 2 * gx):
            c = chosen(n, xx - gx, yy - gy, w, h)
            if c is None:
                out.append(list(EMPTY))
                continue
            u, v = c
            wx, wy = weight(u, w), weight(v, h)
            if wx == 0 and wy == 0:
                out.append(list(at(u, v)))
                continue
            u2, v2 = (u + w // 2) % w, (v + h // 2) % h
            out.append([(1 - wy) * ((1 - wx) * a + wx * bb) + wy * ((1 - wx) * cc + wx * dd)
                        for a, bb, cc, dd in zip(at(u, v), at(u2, v), at(u, v2), at(u2, v2))])
    return {"px": out, "left": layer["left"] - gx, "top": layer["top"] - gy,
            "w": w + 2 * gx, "h": h + 2 * gy}


# --- the cases ------------------------------------------------------------------------------

def case(tiling="unfold", expand_right=0, expand_left=0, expand_down=0, expand_up=0,
         blend_borders=0, shift=0, shadow=False):
    c = dict(locals())
    c["drawing"] = "patch"
    return c


def all_round(tiling, **kw):
    """Grown 5 across and 3 down, to fill the frame."""
    return case(tiling, expand_right=5, expand_left=5, expand_down=3, expand_up=3, **kw)


def held(c, k, frame_no):
    if k == "tiling":
        return c[k]
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def numbers(c, frame_no):
    return {k: held(c, k, frame_no) for k in NAMES}


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    if c["shadow"]:
        layer = drop_shadow(layer, SHADOW["color"], SHADOW["opacity"], SHADOW["direction"],
                            SHADOW["distance"], SHADOW["softness"])
    return layer


def render(c, frame_no):
    return frame(repeat_tile(layer_of(c), numbers(c, frame_no)), c["shift"])


def plain(c):
    return frame(drawn_layer(c["drawing"]), c["shift"])


WORDS_SAID = {"repeat": "Repeat: the picture copied as it is", "checker_flip_h":
              "Checker Flip H: every other copy, as a chequerboard, flipped across",
              "checker_flip_v": "Checker Flip V: every other copy flipped down",
              "unfold": "Unfold: copies flipped across in odd columns and down in odd rows, so "
                        "they meet edge to matching edge",
              "checker_180": "Checker 180: every other copy turned half round",
              "checker_flip_45": "Checker Flip 45: every other copy flipped on its diagonal, "
                                 "squeezed to the tile's shape",
              "checker_90_cw": "Checker 90 CW: every other copy turned a quarter clockwise",
              "checker_90_ccw": "Checker 90 CCW: every other copy turned a quarter anticlockwise",
              "rosette": "Rosette: each square of four copies turned a quarter on round it",
              "random": "Random: each copy one of the eight ways, chosen by its place",
              "none": "None: the room grows but stays clear",
              "turn_cw": "Turn CW: each copy a quarter turn clockwise on from the one before it",
              "turn_ccw": "Turn CCW: each copy a quarter turn anticlockwise on",
              "twist": "Twist: turning clockwise across and anticlockwise down",
              "slide": "Slide: the edge pixels drawn on outwards",
              "brick": "Brick: copies as they are, every other row moved half a tile across"}

CASES = {
    "FX-REPEAT-001": ("The settings as they start: Unfold, nothing grown, no blending. The "
                      "picture as it was, and nothing grows.", case(), [0]),
}
for _n, _t in enumerate(TILINGS, start=2):
    CASES[f"FX-REPEAT-{_n:03d}"] = (WORDS_SAID[_t] + ", grown 5 across and 3 down each way.",
                                    all_round(_t), [0])
CASES.update({
    "FX-REPEAT-018": ("Repeat, grown 4 right, 1 left, 2 up and none down: the layer grows 4 "
                      "both ways across and 2 both ways down, the room past each expansion "
                      "clear.", case("repeat", expand_right=4, expand_left=1, expand_up=2), [0]),
    "FX-REPEAT-019": ("Repeat, nothing grown, Blend Borders 50: the picture's own edges mixed "
                      "with its middle, ready to repeat without a seam.",
                      case("repeat", blend_borders=50), [0]),
    "FX-REPEAT-020": ("Repeat grown all round, Blend Borders 100: the copies meet with no seam.",
                      all_round("repeat", blend_borders=100), [0]),
    "FX-REPEAT-021": ("Unfold grown all round, Blend Borders 100: Unfold ignores the blending "
                      "and is FX-REPEAT-005.", all_round("unfold", blend_borders=100), [0]),
    "FX-REPEAT-022": ("Expand Right keyed from 0 at frame 0 to 6 at frame 4, linear, Repeat: "
                      "frame 1 is 1.5, rounded up to 2; frame 2 is 3.",
                      case("repeat", expand_right=keyed((0, 0), (4, 6))), [0, 1, 2, 4]),
    "FX-REPEAT-023": ("Blend Borders keyed from 50 at frame 0 to 100 at frame 4, eased past its "
                      "end, Repeat grown all round: frame 2 would pass 100, is held at 100, and "
                      "is FX-REPEAT-020.",
                      all_round("repeat", blend_borders=keyed((0, 50, OVERSHOOT), (4, 100))),
                      [0, 2]),
    "FX-REPEAT-024": ("FX-REPEAT-002 moved three pixels right: the copies move with the drawing "
                      "and the three columns left bare stay empty.",
                      all_round("repeat", shift=3), [0]),
    "FX-REPEAT-025": ("After a drop shadow that grows the layer to 10 by 8, Checker Flip H "
                      "grown 2 each way: the copies are of the grown layer, shadow and all.",
                      case("checker_flip_h", expand_right=2, expand_left=2, expand_down=2,
                           expand_up=2, shadow=True), [0]),
})

INVALID = {
    "FX-REPEAT-026": ("Expand Right -1, below 0.", case(expand_right=-1)),
    "FX-REPEAT-027": ("Expand Up 4001, above 4000.", case(expand_up=4001)),
    "FX-REPEAT-028": ("Tiling \"wallpaper\", which is not one.", case(tiling="wallpaper")),
    "FX-REPEAT-029": ("Blend Borders 101, above 100.", case(blend_borders=101)),
    "FX-REPEAT-030": ("Expand Left keyed to 5000 at frame 4, above 4000.",
                      case(expand_left=keyed((0, 0), (4, 5000)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.repeat_tile",
                    "enabled": True,
                    "parameters": {k: (c[k] if k == "tiling" else setting_json(c[k]))
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
    (OUT / "expected_repeat_tile.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    tile = {t: one(f"FX-REPEAT-{n:03d}") for n, t in enumerate(TILINGS, start=2)}
    # The tile whose corner is at (5 + 6 i, 3 + 4 j), as the frame shows it.
    in_tile = lambda px, i, j, x, y: px[at(LEFT + 6 * i + x, TOP + 4 * j + y)]  # noqa: E731

    # The ways: each is one of the eight symmetries of the tile's grid, and they are all
    # different; turning a quarter four times, or flipping twice, is the picture.
    ways = [[oriented(o, x, y, 6, 4) for y in range(4) for x in range(6)] for o in range(8)]
    assert len({tuple(w) for w in ways}) == 8
    sq = lambda o: [oriented(o, x, y, 5, 5) for y in range(5) for x in range(5)]  # noqa: E731
    # On a square, way 6 takes (x, y) from (y, 4 - x): the picture turned clockwise.
    assert sq(6) == [(y, 4 - x) for y in range(5) for x in range(5)]
    assert sq(5) == [(4 - y, x) for y in range(5) for x in range(5)]
    assert sq(3) == [(4 - x, 4 - y) for y in range(5) for x in range(5)]
    # q is a fair spread: every place of the shorter side used, in order.
    assert [q(t, 6, 4) for t in range(6)] == [0, 1, 1, 2, 3, 3]
    assert [q(t, 4, 6) for t in range(4)] == [0, 2, 3, 5]
    # Random uses all eight ways over a small field, and is the same for the same tile.
    assert {random_way(i, j) for i in range(-4, 4) for j in range(-4, 4)} == set(range(8))

    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx
    # No mixing without blending: every pixel is the drawing's, or empty.
    pixels = layer["px"] + [EMPTY]
    for n in list(range(1, 19)) + [21, 22, 24]:
        for px in c[f"FX-REPEAT-{n:03d}"].values():
            assert all(p in pixels for p in px), n

    assert one("FX-REPEAT-001") == art
    rep = tile["repeat"]
    assert all(rep[at(x, y)] == d((x - 5) % 6, (y - 3) % 4) for x in range(W) for y in range(H))
    # The middle tile is always the picture itself.
    for t in TILINGS:
        assert all(in_tile(tile[t], 0, 0, x, y) == d(x, y) for x in range(6) for y in range(4)), t
    assert len({json.dumps(px) for px in tile.values()}) == 16
    fh, fv = tile["checker_flip_h"], tile["checker_flip_v"]
    assert all(in_tile(fh, 1, 0, x, 0) == d(5 - x, 0) for x in range(5))
    assert all(in_tile(fh, 1, 1, x, 0) == d(x, 0) for x in range(5))
    assert all(in_tile(fv, 0, 1, x, y) == d(x, 3 - y) for x in range(6) for y in range(2))
    un = tile["unfold"]
    assert all(in_tile(un, -1, 0, x, 1) == d(5 - x, 1) for x in range(1, 6))
    assert all(in_tile(un, 0, -1, x, 3) == d(x, 0) for x in range(6))
    assert in_tile(un, -1, -1, 5, 3) == d(0, 0)
    c180 = tile["checker_180"]
    assert all(in_tile(c180, 1, 0, x, y) == d(5 - x, 3 - y) for x in range(5) for y in range(2))
    # The quarter turns: the red dot at (1, 1) lands where the way sends it.
    cw = tile["checker_90_cw"]
    assert all(in_tile(cw, 1, 0, x, y) == d(q(y, 4, 6), 3 - q(x, 6, 4))
               for x in range(5) for y in range(2))
    assert all(in_tile(tile["checker_90_ccw"], 1, 0, x, y) == d(5 - q(y, 4, 6), q(x, 6, 4))
               for x in range(5) for y in range(2))
    assert all(in_tile(tile["checker_flip_45"], 1, 0, x, y) == d(q(y, 4, 6), q(x, 6, 4))
               for x in range(5) for y in range(2))
    ros = tile["rosette"]
    assert all(in_tile(ros, 1, 0, x, y) == in_tile(cw, 1, 0, x, y) for x in range(5)
               for y in range(2))
    assert all(in_tile(ros, -1, 0, x, y) == in_tile(cw, 1, 0, x, y) for x in range(1, 5)
               for y in range(4))
    assert all(in_tile(tile["turn_cw"], 1, 0, x, y) == in_tile(cw, 1, 0, x, y)
               for x in range(5) for y in range(2))
    assert all(in_tile(tile["turn_ccw"], 1, 0, x, y) == in_tile(tile["checker_90_ccw"], 1, 0, x, y)
               for x in range(5) for y in range(2))
    assert all(in_tile(tile["twist"], 0, 1, x, y) == in_tile(tile["turn_ccw"], 0, 1, x, y)
               for x in range(6) for y in range(2))
    assert tile["twist"] not in (tile["turn_cw"], tile["turn_ccw"])
    none = tile["none"]
    assert none == art
    sl = tile["slide"]
    assert all(sl[at(x, y)] == d(min(max(x - 5, 0), 5), min(max(y - 3, 0), 3))
               for x in range(W) for y in range(H))
    br = tile["brick"]
    assert all(br[at(x, y)] == d((x - 5) % 6, (y - 3) % 4) for x in range(W) for y in range(3, 7))
    assert all(br[at(x, y)] == d((x - 5 - 3) % 6, (y - 3) % 4) for x in range(W)
               for y in (0, 1, 2, 7, 8, 9))
    uneven = one("FX-REPEAT-018")
    assert all(uneven[at(x, y)] == (d((x - 5) % 6, (y - 3) % 4) if 4 <= x < 15 and 1 <= y < 7
                                    else EMPTY) for x in range(W) for y in range(H))
    nineteen = one("FX-REPEAT-019")
    assert nineteen != art and all(nineteen[at(x, y)] == EMPTY for x in range(W) for y in range(H)
                                   if not (5 <= x < 11 and 3 <= y < 7))
    twenty = one("FX-REPEAT-020")
    assert twenty != rep and all(twenty[at(x, y)] == twenty[at(x + 6, y)] for x in range(W - 6)
                                 for y in range(H))
    assert one("FX-REPEAT-021") == un
    k22 = c["FX-REPEAT-022"]
    assert k22["0"] == art
    assert k22["1"] == render(case("repeat", expand_right=2), 0) != k22["2"]
    assert k22["2"] == render(case("repeat", expand_right=3), 0)
    assert all(k22["4"][at(x, y)] == (d((x - 5) % 6, y - 3) if 3 <= y < 7 and 5 <= x < 16
                                      else EMPTY) for x in range(W) for y in range(H))
    k23 = c["FX-REPEAT-023"]
    raw = value_at(CASES["FX-REPEAT-023"][1]["blend_borders"], 2)
    assert raw > 100 and k23["2"] == twenty and k23["0"] != twenty
    moved = one("FX-REPEAT-024")
    assert all(moved[at(x, y)] == rep[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    shadowed = layer_of(case(shadow=True))
    assert (shadowed["left"], shadowed["top"], shadowed["w"], shadowed["h"]) == (3, 1, 10, 8)
    s = lambda x, y: shadowed["px"][y * 10 + x]  # noqa: E731
    sh = one("FX-REPEAT-025")
    assert all(sh[at(x, y)] == s(x - 3, y - 1) for x in range(3, 13) for y in range(1, 9))
    assert all(sh[at(x, y)] == s(9 - (x - 13), y - 1) for x in range(13, 15) for y in range(1, 9))
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
