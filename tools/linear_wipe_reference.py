"""Linear Wipe, worked a second way.

D-155 adds `core.linear_wipe`. It wipes a layer away behind a straight edge that sweeps across
it, the plainest transition there is: `completion`, 0 to 100 per cent starting at 0, is how far
the edge has gone; `angle`, in degrees -3600 to 3600 starting at 90, is the way it moves, 0 up
and 90 right on the screen, so at the start the layer is wiped from the left, the edge moving
right; and `feather`, 0 to 10000 starting at 0, softens the edge over that many pixels. At
completion 0 the layer is untouched and at 100 it is gone. Feather is a distance, so a draft
preview scales it. It is this program's own method, modelled on After Effects' Linear Wipe;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. At completion 0 the output is the input exactly; at 100 every pixel is transparent,
all four channels 0, the size unchanged. Otherwise u = (sin angle, -cos angle), exact at whole
quarter turns; with w, h the drawing's own size, s_min and s_max are the smallest and largest of
u . corner over its corners (0, 0), (w, 0), (0, h) and (w, h); c = completion / 100, f = feather
and E = s_min - f / 2 + c (s_max - s_min + f). At each pixel, with (X, Y) its centre in drawing
coordinates and s = u . (X, Y), k = clamp((s - E) / f + 0.5, 0, 1) when f > 0, and when f = 0
k is 1 where s >= E and 0 otherwise, a cliff. The output is p k, all four channels
premultiplied alike. Pixels an earlier effect grew outside the drawing's box follow the same
rule, and the layer does not grow. With feather 0 the choice is a cliff: `check` asserts that no
pixel it decides sits within 1e-5 of the edge.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a cel filling the frame, drawn below. The drawing goes into
`Fixtures/linear_wipe/media`, the projects into `Fixtures/linear_wipe`, and the expected frames
into `Fixtures/linear_wipe/expected_linear_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/linear_wipe_reference.py
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
from drop_shadow_reference import unit, drop_shadow, frame  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "linear_wipe"
TOLERANCE = 2e-5  # document 25's default for a filter
COMPLETION, ANGLE, FEATHER = (0, 100), (-3600, 3600), (0, 10000)
NAMES = ("completion", "angle", "feather")
START = {"completion": 0, "angle": 90, "feather": 0}
EMPTY = [0.0] * 4
CLIFF = 1e-5   # no decided pixel may sit closer than this to a hard edge


# --- the rule -------------------------------------------------------------------------------

def edge(completion, angle, feather, size):
    """The wipe's direction u and the edge's place E along it, for a drawing of `size`."""
    ux, uy = unit(angle)
    w, h = size
    s = [ux * x + uy * y for x, y in ((0, 0), (w, 0), (0, h), (w, h))]
    c, f = completion / 100, feather
    return (ux, uy), min(s) - f / 2 + c * (max(s) - min(s) + f)


def along(layer, u):
    """s = u . (X, Y) at each of the layer's pixels, its centre in drawing coordinates."""
    return [u[0] * (x + layer["left"] + 0.5) + u[1] * (y + layer["top"] + 0.5)
            for y in range(layer["h"]) for x in range(layer["w"])]


def linear_wipe(layer, completion, angle, feather, size=(W, H)):
    """The layer wiped behind the edge; the same size and place."""
    if completion == 0:
        return layer
    if completion == 100:
        return dict(layer, px=[list(EMPTY) for _ in layer["px"]])
    u, E = edge(completion, angle, feather, size)
    out = []
    for p, s in zip(layer["px"], along(layer, u)):
        if feather > 0:
            k = min(1.0, max(0.0, (s - E) / feather + 0.5))
        else:
            k = 1.0 if s >= E else 0.0
        out.append([v * k for v in p])
    return dict(layer, px=out)


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, TRACE, NONE = R.LINE, R.SKIN, R.TRACE, S.NONE  # #1e1a24, #f6d6be, #c82828
SOFT_LINE = R.SOFT                    # the line at half covering, its antialiased edge
SOFT_SKIN = SKIN[:3] + (128,)         # the skin at half covering, a soft edge
SHADE = (220, 160, 140, 255)          # #dca08c, the skin in shadow
WHITE = (255, 255, 255, 255)
HOLE = {(7, 4), (8, 4), (7, 5), (8, 5)}


def cel(x, y):
    """Rows 0 and 9 are empty. Between them: the line at half covering down column 0 and the
    skin at half covering down column 15, soft edges; a box of line round columns 1 to 14 and
    rows 1 to 8; inside it skin in rows 2 and 3, white left of a red patch in rows 4 and 5
    with an empty hole in columns 7 and 8, and the shadow skin in rows 6 and 7."""
    if y in (0, 9):
        return NONE
    if x == 0:
        return SOFT_LINE
    if x == 15:
        return SOFT_SKIN
    if y in (1, 8) or x in (1, 14):
        return LINE
    if (x, y) in HOLE:
        return NONE
    if y in (2, 3):
        return SKIN
    if y in (6, 7):
        return SHADE
    return WHITE if x < 7 else TRACE


DRAWINGS = {"cel": [[cel(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

# A drop shadow ahead of the wipe, black, opacity 100, direction 270, distance 2, softness 0:
# the shadow two pixels left, and the layer grown two pixels on every side, to 20 by 14.
SHADOW = {"color": "#000000", "opacity": 100, "direction": 270, "distance": 2, "softness": 0}


def case(completion=0, angle=90, feather=0, move=0, shadow=False):
    return {"drawing": "cel", "completion": completion, "angle": angle, "feather": feather,
            "move": move, "shadow": shadow}


def held(v, r):
    return min(r[1], max(r[0], v))


def settings(c, frame_no):
    return (held(value_at(c["completion"], frame_no), COMPLETION),
            held(value_at(c["angle"], frame_no), ANGLE),
            held(value_at(c["feather"], frame_no), FEATHER))


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    if c["shadow"]:
        layer = drop_shadow(layer, SHADOW["color"], SHADOW["opacity"], SHADOW["direction"],
                            SHADOW["distance"], SHADOW["softness"])
    return layer


def render(c, frame_no):
    return frame(linear_wipe(layer_of(c), *settings(c, frame_no)), c["move"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["move"])


CASES = {
    "FX-LWIPE-001": ("The settings as they start, completion 0, angle 90, feather 0: the "
                     "drawing, untouched.",
                     case(), [0]),
    "FX-LWIPE-002": ("Completion 25: the edge is a quarter of the way across from the left, at "
                     "x = 4, so columns 0 to 3 are gone, all four channels 0, and columns 4 to "
                     "15 are the drawing's own, exactly.",
                     case(completion=25), [0]),
    "FX-LWIPE-003": ("Completion 50: the left half, columns 0 to 7, is gone; the right half is "
                     "untouched.",
                     case(completion=50), [0]),
    "FX-LWIPE-004": ("Completion 100: every pixel is transparent, the soft edges too.",
                     case(completion=100), [0]),
    "FX-LWIPE-005": ("Completion 50, feather 4: the edge is soft over four pixels round x = 8; "
                     "columns 0 to 5 are gone, columns 6, 7, 8 and 9 keep 1/8, 3/8, 5/8 and 7/8 "
                     "of every channel, and columns 10 to 15 are untouched.",
                     case(completion=50, feather=4), [0]),
    "FX-LWIPE-006": ("Angle 270, completion 25: the wipe comes from the right, the edge at "
                     "x = 12; columns 12 to 15 are gone, the soft skin edge with them.",
                     case(completion=25, angle=270), [0]),
    "FX-LWIPE-007": ("Angle 0, completion 30: the edge moves up, so the wipe comes from the "
                     "bottom, the edge at y = 7; rows 7 to 9 are gone.",
                     case(completion=30, angle=0), [0]),
    "FX-LWIPE-008": ("Angle 180, completion 30: the edge moves down, so the wipe comes from the "
                     "top, the edge at y = 3; rows 0 to 2 are gone.",
                     case(completion=30, angle=180), [0]),
    "FX-LWIPE-009": ("Angle 45, completion 40: the wipe comes from the bottom left corner "
                     "towards the top right along a diagonal; a pixel stays whole where x - y "
                     "is at least 1 and is gone otherwise, the bottom left triangle wiped.",
                     case(completion=40, angle=45), [0]),
    "FX-LWIPE-010": ("Angle 135, completion 50, feather 6: from the top left corner, with a "
                     "soft diagonal edge; the top left is gone, the bottom right untouched, and "
                     "between them each pixel keeps a part that grows towards the bottom right.",
                     case(completion=50, angle=135, feather=6), [0]),
    "FX-LWIPE-011": ("Angle 450, a whole turn past 90, completion 25: exactly FX-LWIPE-002.",
                     case(completion=25, angle=450), [0]),
    "FX-LWIPE-012": ("Angle -270, completion 25: -270 is 90 the other way round, exactly "
                     "FX-LWIPE-002.",
                     case(completion=25, angle=-270), [0]),
    "FX-LWIPE-013": ("Completion 50, feather 10000, the most: the edge is so soft that every "
                     "pixel keeps about half of itself, from 0.49925 of it at the left to "
                     "0.50075 at the right.",
                     case(completion=50, feather=10000), [0]),
    "FX-LWIPE-014": ("Completion 0 with angle 45 and feather 20: completion 0 changes nothing, "
                     "whatever the other settings.",
                     case(completion=0, angle=45, feather=20), [0]),
    "FX-LWIPE-015": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                     "untouched, frame 1 is FX-LWIPE-002, frame 2 FX-LWIPE-003, frame 3 columns "
                     "0 to 11 gone, and frame 4 FX-LWIPE-004; the edge sweeps left to right.",
                     case(completion=keyed((0, 0), (4, 100))), [0, 1, 2, 3, 4]),
    "FX-LWIPE-016": ("Completion held at 25 from frame 0, then 50 from frame 3: frames 0 and 2 "
                     "are FX-LWIPE-002, frames 3 and 4 FX-LWIPE-003.",
                     case(completion=keyed((0, 25, "hold"), (3, 50))), [0, 2, 3, 4]),
    "FX-LWIPE-017": ("Completion eased from 50 at frame 0 to 100 at frame 4 on a curve that "
                     "overshoots: at frame 2 it has gone past 100 and is held there, so frames 2 "
                     "and 4 are both FX-LWIPE-004, every pixel transparent.",
                     case(completion=keyed((0, 50, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-LWIPE-018": ("Angle keyed from 90 at frame 0 to 270 at frame 4, linear, completion 30: "
                     "the edge turns round, from the left (columns 0 to 4 gone) through the top "
                     "left, the top at frame 2 (FX-LWIPE-008) and the top right, to the right at "
                     "frame 4 (columns 11 to 15 gone).",
                     case(completion=30, angle=keyed((0, 90), (4, 270))), [0, 1, 2, 3, 4]),
    "FX-LWIPE-019": ("Feather keyed from 0 at frame 0 to 8 at frame 4, linear, completion 50: "
                     "frame 0 is FX-LWIPE-003, frame 2 FX-LWIPE-005, and frame 4 soft over "
                     "eight pixels, columns 4 to 11.",
                     case(completion=50, feather=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-LWIPE-020": ("FX-LWIPE-002 on the layer moved three pixels right: the edge is the "
                     "drawing's own and moves with it, so the drawing's columns 0 to 3, now "
                     "3 to 6, are gone.",
                     case(completion=25, move=3), [0, 3]),
    "FX-LWIPE-021": ("A drop shadow two pixels left, the layer moved three pixels right, "
                     "completion 0: the drawing with its shadow, untouched, the shadow's two "
                     "columns out past the drawing's left edge in columns 1 and 2 too.",
                     case(move=3, shadow=True), [0]),
    "FX-LWIPE-022": ("The same with completion 1: the edge is at x = 0.16, just inside the "
                     "drawing's left edge, so the drawing is whole but the shadow's grown "
                     "columns, left of the edge at x = -1.5 and -0.5, are gone.",
                     case(completion=1, move=3, shadow=True), [0]),
    "FX-LWIPE-023": ("The same with angle 270 and completion 99: the wipe from the right has "
                     "reached x = -0.16, so all of the drawing is gone but the shadow's grown "
                     "columns, beyond the edge, stay.",
                     case(completion=99, angle=270, move=3, shadow=True), [0]),
    "FX-LWIPE-024": ("The same with completion 100: every pixel is transparent, the shadow's "
                     "grown columns too.",
                     case(completion=100, angle=270, move=3, shadow=True), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LWIPE-025": ("Completion -1, below 0.", case(completion=-1)),
    "FX-LWIPE-026": ("Completion 100.5, above 100.", case(completion=100.5)),
    "FX-LWIPE-027": ("Angle 3601, above 3600.", case(completion=25, angle=3601)),
    "FX-LWIPE-028": ("Angle -3601, below -3600.", case(completion=25, angle=-3601)),
    "FX-LWIPE-029": ("Feather -1, below 0.", case(completion=50, feather=-1)),
    "FX-LWIPE-030": ("Feather 10001, above 10000.", case(completion=50, feather=10001)),
    "FX-LWIPE-031": ("Completion keyed to 150 at frame 4.",
                     case(completion=keyed((0, 0), (4, 150)))),
    "FX-LWIPE-032": ("Completion written \"50\", a word, not a number.", case(completion="50")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["move"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["move"], H / 2])
    effects = []
    if c["shadow"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
                        "parameters": dict(SHADOW)})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.linear_wipe",
                    "enabled": True, "parameters": {k: setting_json(c[k]) for k in NAMES}})
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

    (OUT / "expected_linear_wipe.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731
    close = lambda p, q: all(abs(a - b) < 1e-12 for a, b in zip(p, q))  # noqa: E731

    def cut(keep, base=drawn):
        """The frame with the pixels `keep(x, y)` refuses gone."""
        return [base[at(x, y)] if keep(x, y) else EMPTY for y in range(H) for x in range(W)]

    # The rule's own pieces: the direction, exact at quarter turns, the corners and the edge.
    assert unit(90) == (1, 0) and unit(0) == (0, -1) and unit(180) == (0, 1)
    assert unit(270) == (-1, 0) and unit(450) == unit(-270) == unit(90)
    assert edge(25, 90, 0, (W, H)) == ((1, 0), 4.0)
    assert edge(25, 270, 0, (W, H)) == ((-1, 0), -12.0)
    assert edge(30, 0, 0, (W, H)) == ((0, -1), -7.0)
    assert edge(50, 90, 4, (W, H))[1] == 8.0 and edge(50, 90, 8, (W, H))[1] == 8.0
    assert abs(edge(40, 45, 0, (W, H))[1] - 0.4 * math.sqrt(0.5)) < 1e-12
    assert abs(edge(1, 90, 0, (W, H))[1] - 0.16) < 1e-12
    assert abs(edge(99, 270, 0, (W, H))[1] + 0.16) < 1e-12
    shadowed = layer_of(case(shadow=True))
    assert (shadowed["left"], shadowed["top"], shadowed["w"], shadowed["h"]) == (-2, -2, 20, 14)

    # Every case: each pixel is its input times one k in 0..1, the same on all four channels;
    # with feather 0 each pixel is its input exactly or gone, and no pixel that shows sits
    # within 1e-5 of the edge (the cliff).
    for fx, (says, cc, frames) in CASES.items():
        layer = layer_of(cc)
        base = frame(layer, cc["move"])
        for f in frames:
            px = c[fx][str(f)]
            completion, angle, feather = settings(cc, f)
            for p, q in zip(px, base):
                if q[3] == 0:
                    assert p == EMPTY, fx
                    continue
                k = p[3] / q[3]
                assert -1e-15 <= k <= 1 + 1e-15 and close(p, [v * k for v in q]), fx
                if feather == 0 or completion in (0, 100):
                    assert p == q or p == EMPTY, fx
            if feather == 0 and completion not in (0, 100):
                u, E = edge(completion, angle, feather, (W, H))
                for p, s in zip(layer["px"], along(layer, u)):
                    if p[3] > 0:
                        assert abs(s - E) >= CLIFF, (fx, f, s, E)

    assert one("FX-LWIPE-001") == drawn
    assert one("FX-LWIPE-002") == cut(lambda x, y: x >= 4)
    assert one("FX-LWIPE-003") == cut(lambda x, y: x >= 8)
    assert one("FX-LWIPE-004") == [EMPTY] * (W * H)
    five = one("FX-LWIPE-005")
    for x in range(W):
        k = {6: 1 / 8, 7: 3 / 8, 8: 5 / 8, 9: 7 / 8}.get(x, 0.0 if x < 6 else 1.0)
        for y in range(H):
            assert close(five[at(x, y)], [v * k for v in drawn[at(x, y)]]), (x, y)
    assert five[at(10, 2)] == drawn[at(10, 2)] and five[at(5, 2)] == EMPTY
    assert one("FX-LWIPE-006") == cut(lambda x, y: x <= 11)
    assert one("FX-LWIPE-006")[at(15, 4)] == EMPTY != drawn[at(15, 4)]
    assert one("FX-LWIPE-007") == cut(lambda x, y: y <= 6)
    assert one("FX-LWIPE-008") == cut(lambda x, y: y >= 3)
    assert one("FX-LWIPE-009") == cut(lambda x, y: x - y >= 1)
    ten = one("FX-LWIPE-010")
    assert ten[at(1, 1)] == EMPTY and ten[at(14, 8)] == drawn[at(14, 8)]
    ks = {}
    for y in range(1, H - 1):
        for x in range(W):
            if drawn[at(x, y)][3]:
                ks[(x, y)] = ten[at(x, y)][3] / drawn[at(x, y)][3]
    assert any(0 < k < 1 for k in ks.values())
    for (x, y), k in ks.items():  # the part kept grows towards the bottom right
        for n in ((x + 1, y), (x, y + 1)):
            assert n not in ks or ks[n] >= k - 1e-12
    assert one("FX-LWIPE-011") == one("FX-LWIPE-002") == one("FX-LWIPE-012")
    thirteen = one("FX-LWIPE-013")
    for y in range(H):
        for x in range(W):
            k = 0.5 + (x + 0.5 - 8) / 10000
            assert close(thirteen[at(x, y)], [v * k for v in drawn[at(x, y)]])
    assert close([thirteen[at(0, 4)][3] / drawn[at(0, 4)][3]], [0.49925])
    assert close([thirteen[at(15, 4)][3] / drawn[at(15, 4)][3]], [0.50075])
    assert one("FX-LWIPE-014") == drawn

    fifteen = c["FX-LWIPE-015"]
    assert fifteen["0"] == drawn and fifteen["1"] == one("FX-LWIPE-002")
    assert fifteen["2"] == one("FX-LWIPE-003") and fifteen["4"] == one("FX-LWIPE-004")
    assert fifteen["3"] == cut(lambda x, y: x >= 12)
    sixteen = c["FX-LWIPE-016"]
    assert sixteen["0"] == sixteen["2"] == one("FX-LWIPE-002")
    assert sixteen["3"] == sixteen["4"] == one("FX-LWIPE-003")
    raw = value_at(CASES["FX-LWIPE-017"][1]["completion"], 2)
    assert raw > 100
    seventeen = c["FX-LWIPE-017"]
    assert seventeen["0"] == one("FX-LWIPE-003")
    assert seventeen["2"] == seventeen["4"] == one("FX-LWIPE-004")
    eighteen = c["FX-LWIPE-018"]
    assert eighteen["0"] == cut(lambda x, y: x >= 5)
    assert eighteen["2"] == one("FX-LWIPE-008")
    assert eighteen["4"] == cut(lambda x, y: x <= 10)
    assert eighteen["1"][at(1, 1)] == EMPTY and eighteen["1"][at(14, 8)] == drawn[at(14, 8)]
    assert eighteen["3"][at(14, 1)] == EMPTY and eighteen["3"][at(1, 8)] == drawn[at(1, 8)]
    nineteen = c["FX-LWIPE-019"]
    assert nineteen["0"] == one("FX-LWIPE-003") and nineteen["2"] == five
    for x in range(W):
        k = min(1.0, max(0.0, (x + 0.5 - 8) / 8 + 0.5))
        assert close(nineteen["4"][at(x, 4)], [v * k for v in drawn[at(x, 4)]])
        assert (0 < k < 1) == (4 <= x <= 11)
    twenty = c["FX-LWIPE-020"]
    assert twenty["0"] == twenty["3"]
    two = one("FX-LWIPE-002")
    assert all(twenty["0"][at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(twenty["0"][at(x, y)] == EMPTY for x in range(7) for y in range(H))

    # After a drop shadow: completion 0 and 100 hold outside the drawing's box as well, and the
    # grown pixels in between follow the same edge.
    with_shadow = frame(shadowed, 3)
    twentyone = one("FX-LWIPE-021")
    assert twentyone == with_shadow != plain(case(move=3))
    assert all(twentyone[at(x, y)][3] > 0 for x in (1, 2) for y in range(1, H - 1))
    # the edge itself, applied to the grown columns at completion 0, would have wiped them
    u, E = edge(0, 90, 0, (W, H))
    assert E == 0 and u[0] * -0.5 < E
    twentytwo = one("FX-LWIPE-022")
    assert all(twentytwo[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert all(twentytwo[at(x, y)] == with_shadow[at(x, y)] for x in range(3, W) for y in range(H))
    twentythree = one("FX-LWIPE-023")
    assert all(twentythree[at(x, y)] == with_shadow[at(x, y)] for x in (1, 2) for y in range(H))
    assert all(twentythree[at(x, y)] == EMPTY for x in range(3, W) for y in range(H))
    assert any(twentythree[at(x, y)][3] > 0 for x in (1, 2) for y in range(H))
    assert one("FX-LWIPE-024") == [EMPTY] * (W * H)
    print("checked")


if __name__ == "__main__":
    main()
