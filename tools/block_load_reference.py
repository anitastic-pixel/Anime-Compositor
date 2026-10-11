"""Block Load, worked a second way.

D-449 adds `core.block_load`, after After Effects' CC Block Load (Stylize): the picture comes in
as an old web browser drew a progressive image, coarse square blocks first, then finer ones,
each pass sweeping down the layer. After Effects' controls are Completion, Scans, Start Cleared
and Bilinear. Adobe publishes no formula; what is known (riotproducts.co.jp's and vook.vc's
pages on CC Block Load, ae.blender-osamu.com's, and PremiumBeat's 8-bit tutorial) is: Completion
0 to 100 takes the picture from its roughest to as it is; Scans, at most 16, sets how many
passes there are; each pass repeats a scan from the top; Start Cleared, on when added, shows
nothing at Completion 0, and off shows the roughest blocks there; Bilinear smooths the blocks;
and at Completion 0 with Scans 3 the blocks are 8 pixels square. The rest of the rule below is
this program's own, and nothing is ported.

The rule, for a layer w by h pixels as it reaches the effect, the drawing's own corner at o in
that buffer (o = 0 unless an earlier effect grew the layer):

1. n = Scans' whole part. The pictures are P_k for k = 0 to n: the layer cut into square blocks
   2^(n - k) pixels wide (D-144's Mosaic: laid from the drawing's corner, cut short at the far
   edge, each block the mean of its premultiplied pixels), so P_0 is the roughest and P_n, with
   blocks one pixel wide, is the layer as it is.
2. The passes. Start Cleared on: the passes are k = 0 to n, T = n + 1 of them, and before the
   first the layer is empty. Off: the passes are k = 1 to n, T = n, and before the first the
   layer is P_0.
3. q = Completion / 100 * T; j = floor(q) passes are done and the one under way is j's, f = q - j
   of it. When j = T every pass is done and the result is P_n. Otherwise the pass under way
   paints P_k (k the j-th pass's) over what was there, row of blocks by row of blocks from the
   top: of the m rows of P_k's blocks (as Mosaic cuts them, the first and last perhaps cut
   short), the first floor(f m) are P_k; the rows below them keep the picture before this pass
   (P_(k-1), or empty before Start Cleared's first pass).
4. Bilinear, when on, replaces each flat block by a smooth blend: a pixel at (x + 0.5, y + 0.5)
   takes, across and then down, the straight-line blend of the means of the two blocks whose
   centres (the middle of each block as cut) are either side of it, or the edge block's mean
   alone beyond the first or last centre. Blocks one pixel wide are left as they are.

The layer never grows. Settings: `completion` 0 to 100, 0 when added; `scans` 1 to 16, 4, its
whole part; `start_cleared` `on` (when added) or `off`; `bilinear` `off` (when added) or `on`.
Completion and Scans can be keyed. A draft scales the blocks with the picture.

Which row a pass has reached is a choice made from a number, so `check` asserts that no case
puts f m within 1e-6 of a whole number unless exactly on one.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding D-144's strip (`tools/mosaic_reference.py`),
unmoved unless the case says. The expected frames are in
`Fixtures/block_load/expected_block_load.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/block_load_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import mosaic_reference as MO  # noqa: E402

W, H = MO.W, MO.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "block_load"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-6
RANGES = {"completion": (0, 100), "scans": (1, 16)}
WORDS = ("start_cleared", "bilinear")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def runs(n, origin, size):
    """Each block's (start, end) along n buffer columns (or rows), in order."""
    b = MO.blocks(n, origin, size)
    out = []
    for i in range(n):
        if i == 0 or b[i] != b[i - 1]:
            out.append([i, i + 1])
        else:
            out[-1][1] = i + 1
    return out


def picture(layer, size, bilinear):
    """P for blocks `size` wide: each pixel its block's mean, or the blend of the means."""
    w, h, src = layer["w"], layer["h"], layer["px"]
    if size <= 1:
        return [list(p) for p in src]
    cols, rows = runs(w, -layer["left"], size), runs(h, -layer["top"], size)
    means = [[None] * len(cols) for _ in rows]
    for j, (y0, y1) in enumerate(rows):
        for i, (x0, x1) in enumerate(cols):
            ps = [src[y * w + x] for y in range(y0, y1) for x in range(x0, x1)]
            means[j][i] = [sum(p[k] for p in ps) / len(ps) for k in range(4)]
    of = lambda rs: [i for i, (a, b) in enumerate(rs) for _ in range(a, b)]  # noqa: E731
    col_of, row_of = of(cols), of(rows)
    if not bilinear:
        return [list(means[row_of[y]][col_of[x]]) for y in range(h) for x in range(w)]

    def pair(rs, c, p):
        """The two blocks either side of pixel centre p in block c, and how far between."""
        centres = [(a + b) / 2 for a, b in rs]
        a, b = (c - 1, c) if p < centres[c] else (c, c + 1)
        if a < 0:
            return 0, 0, 0.0
        if b >= len(rs):
            return len(rs) - 1, len(rs) - 1, 0.0
        return a, b, (p - centres[a]) / (centres[b] - centres[a])

    out = []
    for y in range(h):
        ya, yb, ty = pair(rows, row_of[y], y + 0.5)
        for x in range(w):
            xa, xb, tx = pair(cols, col_of[x], x + 0.5)
            top = [(1 - tx) * means[ya][xa][k] + tx * means[ya][xb][k] for k in range(4)]
            bottom = [(1 - tx) * means[yb][xa][k] + tx * means[yb][xb][k] for k in range(4)]
            out.append([(1 - ty) * top[k] + ty * bottom[k] for k in range(4)])
    return out


def progress(completion, n, cleared):
    """(k, rows) of the pass under way, or (None, None) when every pass is done; also f."""
    first = 0 if cleared else 1
    t = n + 1 - first
    q = completion / 100 * t
    j = math.floor(q)
    if j >= t:
        return None, 0.0
    return first + j, q - j


def block_load(layer, v, cleared, bilinear):
    n = math.floor(v["scans"])
    k, f = progress(v["completion"], n, cleared)
    if k is None:
        return picture(layer, 1, bilinear)
    size = 2 ** (n - k)
    rows = runs(layer["h"], -layer["top"], size)
    done = math.floor(f * len(rows))
    edge = rows[done][0] if done < len(rows) else layer["h"]
    now = picture(layer, size, bilinear)
    before = [EMPTY] * len(now) if k == 0 else picture(layer, 2 ** (n - k + 1), bilinear)
    w = layer["w"]
    return [list(now[i] if i // w < edge else before[i]) for i in range(len(now))]


# --- the cases ------------------------------------------------------------------------------

def case(completion=0, scans=4, start_cleared="on", bilinear="off", shift=0, before=None):
    c = dict(locals())
    c["drawing"] = "strip"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    layer = MO.layer_of(c)
    v = {k: held(c, k, frame_no) for k in RANGES}
    px = block_load(layer, v, c["start_cleared"] == "on", c["bilinear"] == "on")
    return MO.placed(layer, px, c["shift"])


def plain(c):
    return MO.plain(c)


CASES = {
    "FX-BLOCKLOAD-001": ("The settings as they start: Completion 0, Scans 4, Start Cleared on: "
                         "nothing shows yet.", case(), [0]),
    "FX-BLOCKLOAD-002": ("Completion 100: every pass done, the drawing as it is.",
                         case(completion=100), [0]),
    "FX-BLOCKLOAD-003": ("Scans 3, Start Cleared off, Completion 0: the roughest picture, blocks "
                         "8 pixels square, as Mosaic size 8 draws it (FX-MOSAIC-009).",
                         case(scans=3, start_cleared="off"), [0]),
    "FX-BLOCKLOAD-004": ("Scans 2, Completion 20: three passes, the first, blocks 4 wide, a "
                         "little over half under way: its first row of blocks, rows 0 to 3, "
                         "drawn; below it nothing yet.", case(completion=20, scans=2), [0]),
    "FX-BLOCKLOAD-005": ("Scans 2, Completion 50: the second pass, blocks 2 wide, half under "
                         "way: rows 0 to 3 in blocks 2 wide, rows 4 to 9 still in blocks 4 "
                         "wide.", case(completion=50, scans=2), [0]),
    "FX-BLOCKLOAD-006": ("Scans 2, Start Cleared off, Completion 50: two passes, the first done "
                         "and the last not begun: all in blocks 2 wide, Mosaic size 2.",
                         case(completion=50, scans=2, start_cleared="off"), [0]),
    "FX-BLOCKLOAD-007": ("Scans 2, Start Cleared off, Completion 78: the last pass a little "
                         "over half way: rows 0 to 4 the drawing as it is, rows 5 to 9 in blocks "
                         "2 wide.", case(completion=78, scans=2, start_cleared="off"), [0]),
    "FX-BLOCKLOAD-008": ("Scans 1, Start Cleared off, Completion 0: blocks 2 wide.",
                         case(scans=1, start_cleared="off"), [0]),
    "FX-BLOCKLOAD-009": ("Scans 16, Start Cleared off, Completion 0: one block, the whole "
                         "drawing's mean (FX-MOSAIC-007).",
                         case(scans=16, start_cleared="off"), [0]),
    "FX-BLOCKLOAD-010": ("Scans 2.7: its whole part, FX-BLOCKLOAD-005.",
                         case(completion=50, scans=2.7), [0]),
    "FX-BLOCKLOAD-011": ("Bilinear, Scans 2, Start Cleared off, Completion 0: the blocks 4 wide "
                         "blended smoothly into each other instead of flat.",
                         case(scans=2, start_cleared="off", bilinear="on"), [0]),
    "FX-BLOCKLOAD-012": ("Bilinear, Scans 2, Completion 50: FX-BLOCKLOAD-005 blended.",
                         case(completion=50, scans=2, bilinear="on"), [0]),
    "FX-BLOCKLOAD-013": ("Bilinear, Completion 100: the drawing as it is.",
                         case(completion=100, bilinear="on"), [0]),
    "FX-BLOCKLOAD-014": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear, Scans 2: "
                         "frame 0 nothing; frame 1 the first pass three quarters done, rows 0 "
                         "to 7 in blocks 4 wide; frame 2 FX-BLOCKLOAD-005; frame 3 the last pass "
                         "a quarter done, rows 0 and 1 as drawn, the rest in blocks 2 wide; "
                         "frame 4 the drawing.",
                         case(completion=keyed((0, 0), (4, 100)), scans=2), [0, 1, 2, 3, 4]),
    "FX-BLOCKLOAD-015": ("Scans keyed from 1 at frame 0 to 3 at frame 4, eased past its end, "
                         "Start Cleared off, Completion 0: blocks 2 wide at frame 0, then 4 or "
                         "8.",
                         case(scans=keyed((0, 1, OVERSHOOT), (4, 3)), start_cleared="off"),
                         [0, 2, 4]),
    "FX-BLOCKLOAD-016": ("FX-BLOCKLOAD-005 moved three pixels right: the blocks are laid from "
                         "the drawing's own corner, so they move with it.",
                         case(completion=50, scans=2, shift=3), [0]),
    "FX-BLOCKLOAD-017": ("After a directional blur (direction 90, length 4) that grows the layer "
                         "two pixels on every side, Scans 2, Completion 50: the blocks are laid "
                         "from the drawing's corner, and the pass's rows of blocks are the grown "
                         "layer's, the two grown rows above the drawing a short row of their "
                         "own.", case(completion=50, scans=2, before=(90, 4)), [0]),
}

INVALID = {
    "FX-BLOCKLOAD-018": ("Completion 101, above 100.", case(completion=101)),
    "FX-BLOCKLOAD-019": ("Completion -1, below 0.", case(completion=-1)),
    "FX-BLOCKLOAD-020": ("Scans 0.5, below 1.", case(scans=0.5)),
    "FX-BLOCKLOAD-021": ("Scans 17, above 16.", case(scans=17)),
    "FX-BLOCKLOAD-022": ("Start Cleared \"yes\", not on or off.", case(start_cleared="yes")),
    "FX-BLOCKLOAD-023": ("Bilinear \"On\", in capitals, not the word.", case(bilinear="On")),
    "FX-BLOCKLOAD-024": ("Completion keyed to 150 at frame 4, above 100.",
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
    effects = []
    if c["before"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.directional_blur",
                        "enabled": True,
                        "parameters": {"direction": c["before"][0], "length": c["before"][1]}})
    params = {k: setting_json(c[k]) for k in RANGES}
    params.update({k: c[k] for k in WORDS})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.block_load",
                    "enabled": True, "parameters": params})
    comp["layers"][0]["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in MO.DRAWINGS.items():
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
    (OUT / "expected_block_load.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731
    drawn = plain(case())
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    rows_same = lambda f, g, ys: all(near(f[at(x, y)], g[at(x, y)])  # noqa: E731
                                     for x in range(W) for y in ys)
    mosaic = lambda size, cs=case(): MO.placed(MO.layer_of(cs), MO.mosaic(MO.layer_of(cs), size),  # noqa: E731,E501
                                               cs["shift"])

    # No pass's row edge within the cliff of a whole number unless exactly on it.
    for fx, (says, cs, frames) in CASES.items():
        for fr in frames:
            n = math.floor(held(cs, "scans", fr))
            k, f = progress(held(cs, "completion", fr), n, cs["start_cleared"] == "on")
            if k is None:
                continue
            m = len(runs(MO.layer_of(cs)["h"], -MO.layer_of(cs)["top"], 2 ** (n - k)))
            d = abs(f * m - round(f * m))
            assert d == 0 or d > CLIFF, (fx, fr, f * m)
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), fx
    assert all(p == EMPTY for p in one("FX-BLOCKLOAD-001"))
    assert one("FX-BLOCKLOAD-002") == drawn
    assert rows_same(one("FX-BLOCKLOAD-003"), mosaic(8), range(H))
    four = one("FX-BLOCKLOAD-004")
    assert rows_same(four, mosaic(4), range(4))
    assert all(four[at(x, y)] == EMPTY for x in range(W) for y in range(4, H))
    five = one("FX-BLOCKLOAD-005")
    assert rows_same(five, mosaic(2), range(4)) and rows_same(five, mosaic(4), range(4, H))
    assert not rows_same(five, mosaic(4), range(4))
    assert rows_same(one("FX-BLOCKLOAD-006"), mosaic(2), range(H))
    seven = one("FX-BLOCKLOAD-007")
    assert rows_same(seven, drawn, range(5)) and rows_same(seven, mosaic(2), range(5, H))
    assert rows_same(one("FX-BLOCKLOAD-008"), mosaic(2), range(H))
    assert rows_same(one("FX-BLOCKLOAD-009"), mosaic(16), range(H))
    assert one("FX-BLOCKLOAD-010") == five
    eleven = one("FX-BLOCKLOAD-011")
    flat4 = mosaic(4)
    assert not rows_same(eleven, flat4, range(H))
    # Before the first centres across and down, the edge block's mean alone.
    assert near(eleven[at(0, 0)], flat4[at(0, 0)]) and near(eleven[at(1, 1)], flat4[at(1, 1)])
    # Between two centres the blend is a straight line: pixel centre 4.5 lies 5/8 of the way
    # from centre 2 to centre 6 (row 0 is above the first centre down, so only across).
    mid = eleven[at(4, 0)]
    assert near(mid, [(3 * flat4[at(0, 0)][k] + 5 * flat4[at(4, 0)][k]) / 8 for k in range(4)])
    twelve = one("FX-BLOCKLOAD-012")
    assert rows_same(twelve, eleven, range(4, H))
    assert not rows_same(twelve, five, range(H))
    assert one("FX-BLOCKLOAD-013") == drawn
    fourteen = c["FX-BLOCKLOAD-014"]
    assert all(p == EMPTY for p in fourteen["0"])
    assert rows_same(fourteen["1"], mosaic(4), range(8))
    assert all(fourteen["1"][at(x, y)] == EMPTY for x in range(W) for y in (8, 9))
    assert fourteen["2"] == five and fourteen["4"] == drawn
    assert rows_same(fourteen["3"], drawn, range(2)) and rows_same(fourteen["3"], mosaic(2), range(2, H))
    fifteen = c["FX-BLOCKLOAD-015"]
    assert value_at(keyed((0, 1, OVERSHOOT), (4, 3)), 2) > 2
    assert rows_same(fifteen["0"], mosaic(2), range(H))
    assert rows_same(fifteen["2"], mosaic(8), range(H)) and rows_same(fifteen["4"], mosaic(8), range(H))
    sixteen = one("FX-BLOCKLOAD-016")
    assert all(sixteen[at(x, y)] == five[at(x - 3, y)] for x in range(3, W) for y in range(H))
    grown = case(completion=50, scans=2, before=(90, 4))
    gl = MO.layer_of(grown)
    assert runs(gl["h"], -gl["top"], 2)[:2] == [[0, 2], [2, 4]] and len(runs(gl["h"], 2, 2)) == 7
    seventeen = one("FX-BLOCKLOAD-017")
    # Seven rows of blocks 2 wide, floor(0.5 * 7) = 3 drawn: buffer rows 0 to 5, frame rows 0 to 3.
    assert rows_same(seventeen, mosaic(2, grown), range(4))
    assert rows_same(seventeen, mosaic(4, grown), range(4, H))
    for fx in INVALID:
        assert expected["cases"][fx]["frames"]["0"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
