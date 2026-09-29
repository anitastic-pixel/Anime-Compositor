"""Kaleidoscope, worked a second way.

D-205 adds `core.kaleidoscope`, After Effects' CC Kaleida by a rule of our own: one wedge of the
drawing, from a centre, is repeated round it, every other copy mirrored (Mirror) or every copy
turned the same way (Repeat). It is modelled on After Effects' CC Kaleida and not claimed to
match it; nothing is ported. Document 21 is the rule in words; this file is the reference for
the numbers document 25 pins against it.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an
earlier effect may have grown. The output is the input's size; nothing grows. The centre is
C = (center_x / 100 W, center_y / 100 H) from the two numbers of `center`, n = floor(segments) and w = 2 pi / n. At an output pixel
whose centre is P, in the drawing's own space, d = P - C, r = |d| and theta = atan2(d_x, -d_y),
clockwise from straight up. With theta' = theta - rotation (in radians), k = floor(theta' / w)
and t = theta' - k w; under Mirror, t = w - t when k is odd. With phi = rotation + t, the output
is document 21's bilinear sample of the input at S = C + r 100 / size (sin phi, -cos phi), plus
o, first folded back into the input by mirroring at its edges, each coordinate u across a side n
long becoming m = u mod 2n, or 2n - m when m > n, and then held inside the input's pixel centres,
[0.5, n - 0.5]. So a pattern that reaches past the drawing reads the drawing mirrored round it and
never goes clear. At size 100 the wedge from the rotation clockwise through w is the drawing
itself.

Under Repeat a pixel's copy is a yes-or-no choice from its angle: `check` asserts that no pixel
of a Repeat case has theta' / w within 1e-6 of a whole number.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding Wave Warp's stripes, the same size, unmoved
unless the case says. The drawing goes into `Fixtures/kaleidoscope/media`, the projects into
`Fixtures/kaleidoscope`, and the expected frames into
`Fixtures/kaleidoscope/expected_kaleidoscope.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/kaleidoscope_reference.py
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
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from wave_warp_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "kaleidoscope"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-6
TURN = 2 * math.pi
RANGES = {"segments": (2, 32), "rotation": (-3600, 3600), "size": (10, 1000),
          "center": (-1000, 1000)}
NAMES = tuple(RANGES) + ("mode",)


# --- the rule -------------------------------------------------------------------------------

def wedge(n, x, y):
    """k, t and the rest of the rule at the drawing-space point (x, y); n holds the numbers."""
    cx, cy = n["center"][0] / 100 * W, n["center"][1] / 100 * H
    dx, dy = x - cx, y - cy
    w = TURN / math.floor(n["segments"])
    theta = math.atan2(dx, -dy) - math.radians(n["rotation"])
    k = math.floor(theta / w)
    return cx, cy, math.hypot(dx, dy), w, theta / w, k, theta - k * w


def fold(u, lo, n):
    """u, in the drawing's space, folded into the input that spans lo to lo + n by mirroring at
    its edges, then held inside its pixel centres."""
    m = (u - lo) % (2 * n)
    if m > n:
        m = 2 * n - m
    return lo + min(max(m, 0.5), n - 0.5)


def kaleidoscope(layer, n, mode):
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            cx, cy, r, w, _, k, t = wedge(n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            if mode == "mirror" and k % 2:
                t = w - t
            phi = math.radians(n["rotation"]) + t
            s = r * 100 / n["size"]
            px.append(bilinear(layer, fold(cx + s * math.sin(phi), layer["left"], layer["w"]),
                               fold(cy - s * math.cos(phi), layer["top"], layer["h"])))
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(segments=6, rotation=0, size=100, center=(50, 50), mode="mirror", shift=0,
         tile=False):
    return {"drawing": "stripes", "segments": segments, "rotation": rotation, "size": size,
            "center": center, "mode": mode, "shift": shift,
            "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (tuple, list)):
        return tuple(min(hi, max(lo, e)) for e in v)
    return min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "off") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(kaleidoscope(layer_of(c), n, c["mode"]), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-KALEIDO-001": ("The settings as they start: 6 segments, Mirror, rotation 0, size 100, the "
                       "centre in the middle: the wedge from straight up through 60 degrees "
                       "clockwise is the drawing's own, and it is repeated round, every other copy "
                       "mirrored.", case(), [0]),
    "FX-KALEIDO-002": ("2 segments, Mirror: the right half is the drawing's own and the left half "
                       "its mirror image.", case(segments=2), [0]),
    "FX-KALEIDO-003": ("2 segments, Repeat: the right half is the drawing's own and the left half "
                       "the right half turned half round the centre.",
                       case(segments=2, mode="repeat"), [0]),
    "FX-KALEIDO-004": ("6 segments, Repeat: FX-KALEIDO-001's first wedge, every copy turned the "
                       "same way.", case(mode="repeat"), [0]),
    "FX-KALEIDO-005": ("Rotation 30: the wedge from 30 degrees through 90 is the drawing's own.",
                       case(rotation=30), [0]),
    "FX-KALEIDO-006": ("Size 200: the pattern twice as large, read from half as far out.",
                       case(size=200), [0]),
    "FX-KALEIDO-007": ("Size 50: the pattern half as large, reaching past the drawing, where it "
                       "reads the drawing mirrored back at its edges.", case(size=50), [0]),
    "FX-KALEIDO-008": ("The centre at 25 per cent across, 50 down.", case(center=(25, 50)), [0]),
    "FX-KALEIDO-009": ("Segments 6.9, which counts as 6: FX-KALEIDO-001.", case(segments=6.9), [0]),
    "FX-KALEIDO-010": ("Rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is "
                       "FX-KALEIDO-001 and frame 2, rotation 30, FX-KALEIDO-005.",
                       case(rotation=keyed((0, 0), (4, 60))), [0, 2, 4]),
    "FX-KALEIDO-011": ("Size keyed from 100 at frame 0 to 1000 at frame 4, eased past its end: "
                       "frame 2 would pass 1000, is held at 1000, and is size 1000.",
                       case(size=keyed((0, 100, OVERSHOOT), (4, 1000))), [0, 2]),
    "FX-KALEIDO-012": ("FX-KALEIDO-001 moved three pixels right: the pattern moves with the "
                       "drawing and the three columns left bare stay empty.", case(shift=3), [0]),
    "FX-KALEIDO-013": ("After a Motion Tile that grows the layer: the centre and the wedge are the "
                       "drawing's own, not the grown buffer's, and the pattern reads the tiles "
                       "round it.", case(tile=True), [0]),
}

INVALID = {
    "FX-KALEIDO-014": ("Segments 1, below 2.", case(segments=1)),
    "FX-KALEIDO-015": ("Segments 33, above 32.", case(segments=33)),
    "FX-KALEIDO-016": ("Size 9, below 10.", case(size=9)),
    "FX-KALEIDO-017": ("Centre across 1001, above 1000.", case(center=(1001, 50))),
    "FX-KALEIDO-018": ("Rotation 3601, above 3600.", case(rotation=3601)),
    "FX-KALEIDO-019": ("A mode \"flower\", which is not one.", case(mode="flower")),
    "FX-KALEIDO-020": ("Size keyed to 1001 at frame 4, above 1000.",
                       case(size=keyed((0, 100), (4, 1001)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.kaleidoscope",
                    "enabled": True,
                    "parameters": {k: (c[k] if k == "mode" else setting_json(c[k]))
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
    (OUT / "expected_kaleidoscope.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q: all(abs(a - b) < 1e-9 for a, b in zip(p, q))  # noqa: E731
    art = plain(case())

    # The Repeat cliff: no pixel of a Repeat case sits on a wedge's edge.
    margin = 1.0
    for fx, (says, cs, frames) in CASES.items():
        if cs["mode"] == "repeat":
            n = {k: held(cs, k, 0) for k in RANGES}
            for i in range(W * H):
                q = wedge(n, i % W + 0.5, i // W + 0.5)[4]
                margin = min(margin, abs(q - round(q)))
    assert margin > CLIFF, margin
    print(f"closest to a Repeat edge: {margin:.3g}")

    # Every covering in 0..1 and every colour inside its covering.
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx

    one = c["FX-KALEIDO-001"]["0"]
    n0 = {k: held(case(), k, 0) for k in RANGES}
    first = [i for i in range(W * H) if wedge(n0, i % W + 0.5, i // W + 0.5)[5] == 0]
    assert first and all(near(one[i], art[i]) for i in first) and one != art
    two, three = c["FX-KALEIDO-002"]["0"], c["FX-KALEIDO-003"]["0"]
    for y in range(H):
        for x in range(8, W):
            assert near(two[at(x, y)], art[at(x, y)]) and near(two[at(15 - x, y)], art[at(x, y)])
            assert near(three[at(x, y)], art[at(x, y)])
            if 0 < y:
                assert near(three[at(15 - x, 9 - y)], art[at(x, y)]), (x, y)
    four = c["FX-KALEIDO-004"]["0"]
    assert all(near(four[i], one[i]) for i in first) and four != one
    five = c["FX-KALEIDO-005"]["0"]
    n5 = dict(n0, rotation=30)
    first5 = [i for i in range(W * H) if wedge(n5, i % W + 0.5, i // W + 0.5)[5] == 0]
    assert all(near(five[i], art[i]) for i in first5) and five != one
    assert c["FX-KALEIDO-006"]["0"] != one and c["FX-KALEIDO-007"]["0"] != one
    # Past the drawing the pattern goes on: most pixels whose point falls outside are covered.
    seven, n7 = c["FX-KALEIDO-007"]["0"], dict(n0, size=50)
    past = []
    for i in range(W * H):
        cx, cy, r, w, _, k, t = wedge(n7, i % W + 0.5, i // W + 0.5)
        t = w - t if k % 2 else t
        sx, sy = cx + r * 2 * math.sin(t), cy - r * 2 * math.cos(t)
        if not (0 <= sx <= W and 0 <= sy <= H):
            past.append(seven[i][3])
    assert len(past) > 20 and sum(a > 0 for a in past) > len(past) / 2, past
    print(f"past the drawing at size 50: {len(past)}, {sum(a > 0 for a in past)} covered")
    assert c["FX-KALEIDO-008"]["0"] != one
    assert c["FX-KALEIDO-009"]["0"] == one
    ten = c["FX-KALEIDO-010"]
    assert ten["0"] == one and ten["2"] == five and ten["4"] not in (one, five)
    eleven = c["FX-KALEIDO-011"]
    assert ease(OVERSHOOT, 0.5) > 1 and eleven["0"] == one
    assert eleven["2"] == render(case(size=1000), 2)
    moved = c["FX-KALEIDO-012"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    tiled = c["FX-KALEIDO-013"]["0"]
    assert all(near(tiled[i], one[i]) for i in first) and tiled != one
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
