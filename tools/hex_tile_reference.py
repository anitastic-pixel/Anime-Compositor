"""Hex Tile, worked a second way.

D-461 adds `core.hex_tile`, after After Effects' CC HexTile (Stylize): the picture repeated in a
honeycomb. What is known (the CycoreFX HD 1.8.1 manual's page on CC HexTile; no tutorial found
says more) is: Render chooses Tile, Fold Aligned or Fold Seamlessly; Radius is the size of the
tiles, a larger one holding more of the picture; Center is where the honeycomb starts; Lock
Center Tile, off at first, puts the honeycomb's middle at the layer's middle; Rotate turns the
honeycomb round its centre; and Smearing pulls the picture towards the centre so that each tile
shows a different part of it. The rule below is this program's own, and nothing is ported; what
the two folds do inside a tile is our reading of their names.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an
earlier effect may have grown. The output is the input's size; nothing grows.

1. The centre C = (center_x / 100 W, center_y / 100 H). The honeycomb's origin O is C, or, with
   Lock Center Tile on, the drawing's middle (W / 2, H / 2). With a = rotate in radians, c =
   cos a and s = sin a.
2. At an output pixel whose centre is P, in the drawing's own space, d = P - O, turned into the
   honeycomb's frame: u = c d_x + s d_y, v = c d_y - s d_x.
3. The tiles are hexagons with a flat top and bottom, corners at R = radius from their middles,
   the middles at (3/2 R q, sqrt(3) R (r + q / 2)) for whole q and r. The pixel's tile is the
   nearest middle: q' = u (2 / (3 R)), r' = u (-1 / (3 R)) + v (sqrt(3) / (3 R)), and with x =
   q', z = r', y = -x - z, each rounded to the nearest whole number (half up, floor(t + 0.5)), the
   one rounded farthest is put back as minus the other two (x first, then y, then z; a tie keeps
   the earlier). Its middle is (m_u, m_v) = (3/2 R) q and sqrt(3) R (r + q / 2), and the pixel's
   place in it l = (u - m_u, v - m_v).
4. Render. Tile: l as it is, every tile the same. Fold Aligned and Fold Seamlessly cut each tile
   into six triangles from its middle to its corners and fill every one from the first, the one
   from 0 to 60 degrees (the angle from the u axis towards v). The triangle k is found with no
   angle worked: e0 = l_v, e1 = l_v / 2 - h l_u and e2 = -l_v / 2 - h l_u, h = sqrt(3) / 2; with
   e0 >= 0, k is 0 if e1 < 0, else 1 if e2 < 0, else 2; with e0 < 0, k is 3 if e1 > 0, else 4 if
   e2 > 0, else 5. Then l is turned back by k 60 degrees: (C_k l_u + S_k l_v, C_k l_v - S_k l_u),
   C_k and S_k the cosine and sine of k 60 degrees. Fold Aligned stops there: six turned copies,
   which meet at a seam. Fold Seamlessly then mirrors every odd triangle across the first one's
   middle line: (l_u / 2 + h l_v, h l_u - l_v / 2). Neighbouring triangles, and neighbouring tiles,
   then meet as mirror images, with no seam.
5. The picture is read at S = C + (c l_u - s l_v, s l_u + c l_v) + m (c m_u - s m_v, s m_u + c m_v),
   m = smearing / 100: at Smearing 0 every tile shows the picture round the centre, and at 100
   with Lock Center Tile off, under Tile, the picture as it is. S plus o is folded back into the
   input by mirroring at its edges, each coordinate across a side n long becoming t mod 2n, or 2n
   - t past n, then held inside the pixel centres, as Kaleidoscope's (D-205) is, so the honeycomb
   never goes clear; and document 21's bilinear sample of the input there is the output.

Tile and Fold Aligned make a yes-or-no choice from a pixel's place (its tile, and under Fold
Aligned its triangle): `check` asserts that no pixel of those cases is within 1e-6 of a tile's
edge or a triangle's.

Settings: `render` "tile", "fold_aligned" or "fold_seamless", "tile" when added; `radius` 1 to
4000 pixels, 50, scaled with a draft as the other distances are; `center` two numbers, per cent
of the drawing, -1000 to 1000 each, (50, 50); `lock_center` "on" or "off", "off"; `rotate` -3600
to 3600 degrees, 0; `smearing` 0 to 100 per cent, 0. Words are kept as written, so a wrong one is
reported. The numbers can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Wave Warp's stripes, the same size, unmoved
unless the case says. The drawing goes into `Fixtures/hex_tile/media`, the projects into
`Fixtures/hex_tile`, and the expected frames into `Fixtures/hex_tile/expected_hex_tile.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/hex_tile_reference.py
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
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from wave_warp_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "hex_tile"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-6
H3 = math.sqrt(3) / 2
RANGES = {"radius": (1, 4000), "center": (-1000, 1000), "rotate": (-3600, 3600),
          "smearing": (0, 100)}
WORDS = {"render": ("tile", "fold_aligned", "fold_seamless"), "lock_center": ("on", "off")}
NAMES = ("render", "radius", "center", "lock_center", "rotate", "smearing")


# --- the rule -------------------------------------------------------------------------------

def place(n, x, y):
    """The honeycomb at the drawing-space point (x, y): the centre, the turn, the tile's middle,
    the place in it and how near the point is to a tile's edge."""
    cx, cy = n["center"][0] / 100 * W, n["center"][1] / 100 * H
    ox, oy = (W / 2, H / 2) if n["lock_center"] == "on" else (cx, cy)
    a = math.radians(n["rotate"])
    c, s = math.cos(a), math.sin(a)
    dx, dy = x - ox, y - oy
    u, v = c * dx + s * dy, c * dy - s * dx
    rad = n["radius"]
    qf = u * (2 / (3 * rad))
    rf = u * (-1 / (3 * rad)) + v * (math.sqrt(3) / (3 * rad))
    t = [qf, -qf - rf, rf]
    r = [math.floor(e + 0.5) for e in t]
    gap = [abs(a - b) for a, b in zip(r, t)]
    if gap[0] > gap[1] and gap[0] > gap[2]:
        r[0] = -r[1] - r[2]
    elif gap[1] > gap[2]:
        r[1] = -r[0] - r[2]
    else:
        r[2] = -r[0] - r[1]
    q, rr = r[0], r[2]
    mu, mv = 1.5 * rad * q, math.sqrt(3) * rad * (rr + q / 2)
    # How much nearer the tile's middle is than the next nearest, over the radius.
    others = sorted(math.hypot(u - 1.5 * rad * a, v - math.sqrt(3) * rad * (b + a / 2))
                    for a in range(q - 2, q + 3) for b in range(rr - 2, rr + 3))
    edge = (others[1] - others[0]) / rad
    return (cx, cy), (c, s), (mu, mv), (u - mu, v - mv), edge


def triangle(lu, lv):
    e0, e1, e2 = lv, lv / 2 - H3 * lu, -lv / 2 - H3 * lu
    if e0 >= 0:
        k = 0 if e1 < 0 else 1 if e2 < 0 else 2
    else:
        k = 3 if e1 > 0 else 4 if e2 > 0 else 5
    return k, min(abs(e0), abs(e1), abs(e2))


def fold(u, lo, n):
    """u, in the drawing's space, folded into the input that spans lo to lo + n by mirroring at
    its edges, then held inside its pixel centres."""
    m = (u - lo) % (2 * n)
    if m > n:
        m = 2 * n - m
    return lo + min(max(m, 0.5), n - 0.5)


def source(n, x, y):
    """Where the pixel whose centre is (x, y) reads, and how near it is to a cliff."""
    (cx, cy), (c, s), (mu, mv), (lu, lv), edge = place(n, x, y)
    near = edge
    if n["render"] != "tile":
        k, e = triangle(lu, lv)
        if n["render"] == "fold_aligned":
            near = min(near, e)
        a = math.radians(60 * k)
        ck, sk = math.cos(a), math.sin(a)
        lu, lv = ck * lu + sk * lv, ck * lv - sk * lu
        if n["render"] == "fold_seamless" and k % 2:
            lu, lv = lu / 2 + H3 * lv, H3 * lu - lv / 2
    m = n["smearing"] / 100
    return (cx + c * lu - s * lv + m * (c * mu - s * mv),
            cy + s * lu + c * lv + m * (s * mu + c * mv)), near


def hex_tile(layer, n):
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            (sx, sy), _ = source(n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            px.append(bilinear(layer, fold(sx, layer["left"], layer["w"]),
                               fold(sy, layer["top"], layer["h"])))
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(render="tile", radius=50, center=(50, 50), lock_center="off", rotate=0, smearing=0,
         shift=0, tile=False):
    c = dict(locals())
    c["drawing"] = "stripes"
    return c


def held(c, k, frame_no):
    if k in WORDS:
        return c[k]
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (tuple, list)):
        return tuple(min(hi, max(lo, e)) for e in v)
    return min(hi, max(lo, v))


def numbers(c, frame_no):
    return {k: held(c, k, frame_no) for k in NAMES}


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "off") if c["tile"] else layer


def render(c, frame_no):
    return frame(hex_tile(layer_of(c), numbers(c, frame_no)), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-HEX-001": ("The settings as they start: Tile, radius 50, the centre in the middle, "
                   "unlocked, no turn, no smearing. The middle tile holds all of this small "
                   "drawing, so the picture is as it was.", case(), [0]),
    "FX-HEX-002": ("Radius 3, Tile: the hexagon round the middle repeated in a honeycomb, every "
                   "tile the same.", case(radius=3), [0]),
    "FX-HEX-003": ("Radius 3, Fold Aligned: each tile six turned copies of one triangle of the "
                   "middle.", case(radius=3, render="fold_aligned"), [0]),
    "FX-HEX-004": ("Radius 3, Fold Seamlessly: each tile six triangles, every other one "
                   "mirrored, meeting their neighbours with no seam.",
                   case(radius=3, render="fold_seamless"), [0]),
    "FX-HEX-005": ("Radius 3, Rotate 30: the honeycomb turned a twelfth round.",
                   case(radius=3, rotate=30), [0]),
    "FX-HEX-006": ("Radius 3, the centre at 30 per cent across and 40 down: the tiles show the "
                   "picture there, and the honeycomb starts there.",
                   case(radius=3, center=(30, 40)), [0]),
    "FX-HEX-007": ("FX-HEX-006 with Lock Center Tile on: the same picture in the tiles, the "
                   "honeycomb started at the drawing's middle.",
                   case(radius=3, center=(30, 40), lock_center="on"), [0]),
    "FX-HEX-008": ("Radius 3, Smearing 50: each tile shows the picture half way from the centre "
                   "to its own place.", case(radius=3, smearing=50), [0]),
    "FX-HEX-009": ("Radius 3, Smearing 100, Tile, unlocked: every tile shows its own place, the "
                   "picture as it was.", case(radius=3, smearing=100), [0]),
    "FX-HEX-010": ("Radius keyed from 2 at frame 0 to 6 at frame 4, linear: frame 2 is "
                   "radius 4.", case(radius=keyed((0, 2), (4, 6))), [0, 2, 4]),
    "FX-HEX-011": ("Smearing keyed from 50 at frame 0 to 100 at frame 4, eased past its end, "
                   "radius 3: frame 2 would pass 100, is held at 100, and is FX-HEX-009.",
                   case(radius=3, smearing=keyed((0, 50, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-HEX-012": ("FX-HEX-002 moved three pixels right: the honeycomb moves with the drawing "
                   "and the three columns left bare stay empty.", case(radius=3, shift=3), [0]),
    "FX-HEX-013": ("After a Motion Tile that grows the layer, radius 3: the centre is the "
                   "drawing's own, not the grown buffer's, and each tile is FX-HEX-002's.",
                   case(radius=3, tile=True), [0]),
    "FX-HEX-014": ("Radius 4, Fold Seamlessly, Rotate 15, the centre at 40, 60, Smearing 25: "
                   "everything at once.",
                   case(radius=4, render="fold_seamless", rotate=15, center=(40, 60),
                        smearing=25), [0]),
}

INVALID = {
    "FX-HEX-015": ("Radius 0.5, below 1.", case(radius=0.5)),
    "FX-HEX-016": ("Radius 4001, above 4000.", case(radius=4001)),
    "FX-HEX-017": ("Render \"honeycomb\", which is not one.", case(render="honeycomb")),
    "FX-HEX-018": ("Lock Center Tile \"maybe\", neither on nor off.", case(lock_center="maybe")),
    "FX-HEX-019": ("Smearing 101, above 100.", case(smearing=101)),
    "FX-HEX-020": ("Rotate 3601, above 3600.", case(rotate=3601)),
    "FX-HEX-021": ("Centre across 1001, above 1000.", case(center=(1001, 50))),
    "FX-HEX-022": ("Radius keyed to 5000 at frame 4, above 4000.",
                   case(radius=keyed((0, 50), (4, 5000)))),
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
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "off"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.hex_tile",
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
    (OUT / "expected_hex_tile.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q: all(abs(a - b) < 1e-9 for a, b in zip(p, q))  # noqa: E731
    art = plain(case())

    # The cliffs: no pixel of a Tile or Fold Aligned case sits on an edge it chooses by.
    margin = 1.0
    for fx, (says, cs, frames) in CASES.items():
        if cs["render"] == "fold_seamless":
            continue
        layer = layer_of(cs)
        for f in frames:
            n = numbers(cs, f)
            for j in range(layer["h"]):
                for i in range(layer["w"]):
                    _, e = source(n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
                    margin = min(margin, e)
    assert margin > CLIFF, margin
    print(f"closest to a tile's or triangle's edge: {margin:.3g}")

    # Fold Seamlessly is seamless: points a hair either side of a tile's edge and a triangle's
    # edge read the same place.
    n4 = numbers(case(radius=3, render="fold_seamless", rotate=15), 0)
    for t in range(1, 40):
        x, y = W / 2 + 0.37 * t - 7, H / 2 + 0.23 * t - 4
        a, _ = source(n4, x, y)
        b, _ = source(n4, x + 1e-9, y + 1e-9)
        assert math.hypot(a[0] - b[0], a[1] - b[1]) < 1e-6, (x, y, a, b)

    # Every covering in 0..1, every colour inside its covering, and nothing goes clear.
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx

    one = c["FX-HEX-001"]["0"]
    assert all(near(p, q) for p, q in zip(one, art))
    two = c["FX-HEX-002"]["0"]
    assert two != art
    # Every tile is the middle one: a pixel and the one a whole tile step away agree.
    assert near(two[at(8, 5)], art[at(8, 5)])
    three, four = c["FX-HEX-003"]["0"], c["FX-HEX-004"]["0"]
    assert len({json.dumps(two), json.dumps(three), json.dumps(four)}) == 3
    assert c["FX-HEX-005"]["0"] != two
    six, seven = c["FX-HEX-006"]["0"], c["FX-HEX-007"]["0"]
    assert six != two and seven != six and seven != two
    assert c["FX-HEX-008"]["0"] not in (two, art)
    nine = c["FX-HEX-009"]["0"]
    assert all(near(p, q) for p, q in zip(nine, art))
    ten = c["FX-HEX-010"]
    assert ten["2"] == render(case(radius=4), 0) and len({json.dumps(f) for f in ten.values()}) == 3
    eleven = c["FX-HEX-011"]
    raw = value_at(CASES["FX-HEX-011"][1]["smearing"], 2)
    assert raw > 100 and eleven["2"] == nine and eleven["0"] == c["FX-HEX-008"]["0"]
    moved = c["FX-HEX-012"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # After Motion Tile the picture round the centre is the same drawing, so the tiles are too.
    tiled = c["FX-HEX-013"]["0"]
    assert all(near(p, q) for p, q in zip(tiled, two))
    assert c["FX-HEX-014"]["0"] not in (two, four, art)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
