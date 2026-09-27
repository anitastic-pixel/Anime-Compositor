"""Venetian Blinds, worked a second way.

D-157 adds `core.venetian_blinds`. It is a transition that wipes a layer away in stripes, as a
set of window blinds closing does: the layer is cut into parallel slats `width` pixels wide, and
as `completion` goes from 0 to 100 each slat is cleared from one of its edges, the same part of
every slat at once, until at 100 nothing is left. `angle`, in degrees, turns the slats: at 0 they
lie level, each cleared from its bottom edge upward; at 90 they stand upright, each cleared from
its left edge rightward. `feather` softens the moving edge over that many pixels. `width` and
`feather` are distances, so a draft preview scales them. It is this program's own method,
modelled on After Effects' Venetian Blinds; nothing is ported. Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it.

The rule. At completion 0 the output is the input exactly; at 100 every pixel is transparent,
all four channels 0. Otherwise, at the pixel whose centre in drawing coordinates is (X, Y),
s = u(angle) . (X, Y) with u(t) = (sin t, -cos t), exact at whole quarter turns;
t = s - width floor(s / width), where the pixel sits across its slat, 0 up to width;
E = completion / 100 (width + f) - f / 2 with f the feather; k = clamp((t - E) / f + 0.5, 0, 1)
when f > 0, and with f = 0, k = 1 when t >= E and 0 otherwise. The output is p k, all four
premultiplied channels alike, so the colour is kept and only the covering falls. The layer does
not grow. The f = 0 choice is a cliff, and so is the slat's own edge where t wraps from width
back to 0 (a feathered slat is soft only on its moving edge): `check` asserts that no pixel it
decides sits within 1e-5 of either.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a window, drawn below. The drawing goes into `Fixtures/venetian_blinds/media`,
the projects into `Fixtures/venetian_blinds`, and the expected frames into
`Fixtures/venetian_blinds/expected_venetian_blinds.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/venetian_blinds_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from drop_shadow_reference import unit  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "venetian_blinds"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"completion": (0, 100), "angle": (-3600, 3600), "width": (1, 10000),
          "feather": (0, 10000)}
START = {"completion": 0, "angle": 0, "width": 20, "feather": 0}
NAMES = tuple(START)
CLIFF = 1e-5   # no decided pixel may sit closer than this to a cliff
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def across(X, Y, angle, width):
    """t, where the point (X, Y) of the drawing sits across its slat, 0 up to width."""
    ux, uy = unit(angle)
    s = ux * X + uy * Y
    return s - width * math.floor(s / width)


def edge(completion, width, feather):
    """E, the moving edge's place across every slat."""
    return completion / 100 * (width + feather) - feather / 2


def cover(t, E, feather):
    """k, how much of the pixel is kept."""
    if feather > 0:
        return min(1.0, max(0.0, (t - E) / feather + 0.5))
    return 1.0 if t >= E else 0.0


def blinds(layer, completion, angle, width, feather):
    """The layer, `w` by `h` with its top left at drawing coordinates (left, top), wiped."""
    if completion <= 0:
        return [list(p) for p in layer["px"]]
    if completion >= 100:
        return [list(EMPTY) for _ in layer["px"]]
    E = edge(completion, width, feather)
    out = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            p = layer["px"][y * layer["w"] + x]
            X, Y = x + layer["left"] + 0.5, y + layer["top"] + 0.5
            k = cover(across(X, Y, angle, width), E, feather)
            out.append([v * k for v in p])
    return out


# --- the drawing ----------------------------------------------------------------------------

SKY = (58, 111, 216, 255)          # #3a6fd8, the mirror's and the ball's blue
SKIN, TRACE, NONE = R.SKIN, R.TRACE, S.NONE   # #f6d6be, #c82828
SOFT = R.SOFT                      # the line #1e1a24 at half covering, a soft edge


def window(x, y):
    """Columns 0 and 15 are empty. Columns 1 and 14 are the line at half covering, a soft
    frame down both sides. Between, columns 2 to 13: sky in rows 0 to 4, skin in rows 5 to 9,
    with a red sill across row 7, columns 4 to 11."""
    if x in (0, 15):
        return NONE
    if x in (1, 14):
        return SOFT
    if y == 7 and 4 <= x <= 11:
        return TRACE
    return SKY if y <= 4 else SKIN


DRAWINGS = {"window": [[window(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


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
    return R.frame(blinds(drawn_layer(c["drawing"]), **v), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame(drawn_layer(c["drawing"])["px"], c["shift"])


CASES = {
    "FX-BLINDS-001": ("The settings as they start, completion 0, angle 0, width 20, feather 0: "
                      "the drawing, untouched.",
                      case(), [0]),
    "FX-BLINDS-002": ("Completion 50, width 4, angle 0: level slats four pixels tall from the "
                      "drawing's top edge, each cleared from its bottom edge up to its middle, "
                      "so rows 2, 3, 6 and 7 are transparent and every other pixel is the "
                      "drawing's exactly.",
                      case(completion=50, width=4), [0]),
    "FX-BLINDS-003": ("Completion 60 with the starting width 20: one slat is taller than the "
                      "whole drawing, reaching down to row 19, and 60 per cent of it, 12 pixels, "
                      "is cleared up from there, so only rows 8 and 9 are transparent.",
                      case(completion=60), [0]),
    "FX-BLINDS-004": ("Completion 100, width 4, feather 3: every pixel is transparent, the "
                      "feather too.",
                      case(completion=100, width=4, feather=3), [0]),
    "FX-BLINDS-005": ("Completion 50, width 4, angle 90: upright slats four columns wide, each "
                      "cleared from its left edge, so columns 0, 1, 4, 5, 8, 9, 12 and 13 are "
                      "transparent and the rest exact.",
                      case(completion=50, width=4, angle=90), [0]),
    "FX-BLINDS-006": ("Completion 50, width 4, angle 180: level slats cleared from their top "
                      "edge down, so rows 0, 1, 4, 5, 8 and 9 are transparent.",
                      case(completion=50, width=4, angle=180), [0]),
    "FX-BLINDS-007": ("Completion 50, width 4, angle 270: upright slats cleared from their "
                      "right edge, so columns 2, 3, 6, 7, 10, 11, 14 and 15 are transparent.",
                      case(completion=50, width=4, angle=270), [0]),
    "FX-BLINDS-008": ("Completion 50, width 4, angle 30: slats tilted to fall 30 degrees to "
                      "the right, each cleared from its lower edge; every pixel is either kept "
                      "exactly or transparent, about half of those shown cleared.",
                      case(completion=50, width=4, angle=30), [0]),
    "FX-BLINDS-009": ("Completion 50, width 4, angle -270, three quarter turns the other way: "
                      "the same direction as 90, so exactly FX-BLINDS-005.",
                      case(completion=50, width=4, angle=-270), [0]),
    "FX-BLINDS-010": ("Completion 50, width 2, angle 0: slats two pixels tall, so every other "
                      "row, 1, 3, 5, 7 and 9, is transparent.",
                      case(completion=50, width=2), [0]),
    "FX-BLINDS-011": ("Completion 50, width 5, angle 0, feather 2: rows 0 and 1 of each slat "
                      "are kept, the moving edge at its middle row is at half covering (rows 2 "
                      "and 7), and rows 3, 4, 8 and 9 are transparent.",
                      case(completion=50, width=5, feather=2), [0]),
    "FX-BLINDS-012": ("Completion 30, width 5, feather 4: a graded edge, each slat's rows kept "
                      "at 1, 1, 0.95, 0.7 and 0.45 from its top down; the covering jumps back "
                      "to 1 at the next slat's top, as a slat's fixed edge stays hard.",
                      case(completion=30, width=5, feather=4), [0]),
    "FX-BLINDS-013": ("Completion 50, width 4, feather 10000, the most: the feather is far "
                      "wider than a slat, so every shown pixel is kept at about half, within "
                      "0.0002 of it, the drawing fading evenly.",
                      case(completion=50, width=4, feather=10000), [0]),
    "FX-BLINDS-014": ("Completion keyed from 0 at frame 0 to 100 at frame 4, width 4, linear: "
                      "frame 0 untouched, then rows 3 and 7 go, then 2, 3, 6 and 7 (frame 2 is "
                      "FX-BLINDS-002), then all but rows 0, 4 and 8, and frame 4 is empty; "
                      "a pixel once cleared never comes back.",
                      case(completion=keyed((0, 0), (4, 100)), width=4), [0, 1, 2, 3, 4]),
    "FX-BLINDS-015": ("Completion held at 25 from frame 0, then 75 from frame 3, width 4: "
                      "frames 0 and 2 have rows 3 and 7 cleared, frames 3 and 4 all but "
                      "rows 0, 4 and 8.",
                      case(completion=keyed((0, 25, "hold"), (3, 75)), width=4), [0, 2, 3, 4]),
    "FX-BLINDS-016": ("Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                      "overshoots, width 4: at frame 2 it has gone past 100 and is held there, "
                      "so frames 2 and 4 are both every pixel transparent.",
                      case(completion=keyed((0, 0, OVERSHOOT), (4, 100)), width=4), [0, 2, 4]),
    "FX-BLINDS-017": ("Angle keyed from 0 at frame 0 to 60 at frame 4, completion 50, width 4: "
                      "frame 0 is FX-BLINDS-002, frame 2 is FX-BLINDS-008 (angle 30), and "
                      "frame 4 tilts further, to 60; the slats turn.",
                      case(completion=50, width=4, angle=keyed((0, 0), (4, 60))), [0, 2, 4]),
    "FX-BLINDS-018": ("FX-BLINDS-005 moved three pixels right: the slats are the drawing's "
                      "own, so it is the same, moved.",
                      case(completion=50, width=4, angle=90, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BLINDS-019": ("Completion -1, below 0.", case(completion=-1)),
    "FX-BLINDS-020": ("Completion 100.5, above 100.", case(completion=100.5)),
    "FX-BLINDS-021": ("Width 0.5, below 1.", case(completion=50, width=0.5)),
    "FX-BLINDS-022": ("Width 10001, above 10000.", case(completion=50, width=10001)),
    "FX-BLINDS-023": ("Feather -1, below 0.", case(completion=50, feather=-1)),
    "FX-BLINDS-024": ("Angle 3601, above 3600.", case(completion=50, angle=3601)),
    "FX-BLINDS-025": ("Completion keyed to 150 at frame 4.",
                      case(completion=keyed((0, 0), (4, 150)))),
    "FX-BLINDS-026": ("Completion written \"50\", a word, not a number.", case(completion="50")),
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
        "instance_id": "fx-0-0", "type_id": "core.venetian_blinds", "enabled": True,
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

    (OUT / "expected_venetian_blinds.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731

    def cleared(f, rows=None, cols=None):
        """Frame f is the drawing with exactly the given rows (or columns) transparent."""
        return f == [EMPTY if (rows is not None and y in rows) or (cols is not None and x in cols)
                     else drawn[at(x, y)] for y in range(H) for x in range(W)]

    def kept(f):
        """k per pixel, read back from the covering where the drawing shows."""
        return {i: f[i][3] / drawn[i][3] for i in range(W * H) if drawn[i][3] > 0}

    # The rule's pieces: exact directions at quarter turns, t always 0 up to width, and the ends.
    assert unit(0) == (0, -1) and unit(90) == (1, 0) and unit(-270) == (1, 0)
    assert unit(3600) == (0, -1) and unit(180) == (0, 1) and unit(270) == (-1, 0)
    assert across(0.5, 0.5, 0, 4) == 3.5 and across(0.5, 4.5, 0, 4) == 3.5
    assert across(5.5, 0.5, 90, 4) == 1.5 and across(0.5, 0.5, 0, 20) == 19.5
    assert edge(0, 4, 3) == -1.5 and edge(100, 4, 3) == 5.5 and edge(50, 5, 2) == 2.5
    assert cover(0, edge(0, 4, 3), 3) == 1 and cover(4, edge(100, 4, 3), 3) == 0
    assert cover(2, 2, 0) == 1 and cover(1.99, 2, 0) == 0

    # Every case scales every pixel by one k in 0..1, all four channels alike, keeps the empty
    # pixels empty, and decides no pixel within 1e-5 of a cliff: E where there is no feather,
    # and the slat's own edge, where t wraps.
    for fx, frames in c.items():
        cc = (CASES.get(fx) or INVALID.get(fx))[1]
        base = plain(cc)
        for f, px in frames.items():
            v = {n: held(cc, n, int(f)) for n in NAMES} if fx in CASES else None
            for i in range(W * H):
                b, p = base[i], px[i]
                if b[3] == 0:
                    assert p == EMPTY, (fx, i)
                    continue
                k = p[3] / b[3]
                assert -1e-15 <= k <= 1 + 1e-15, (fx, i)
                assert all(abs(q - r * k) < 1e-15 for q, r in zip(p, b)), (fx, i)
                if v is None or not 0 < v["completion"] < 100:
                    continue
                X, Y = i % W - cc["shift"] + 0.5, i // W + 0.5
                t = across(X, Y, v["angle"], v["width"])
                assert min(t, v["width"] - t) >= CLIFF, (fx, f, i, t)
                if v["feather"] == 0:
                    assert abs(t - edge(v["completion"], v["width"], 0)) >= CLIFF, (fx, f, i, t)
                    assert k in (0, 1), (fx, i)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn

    assert one("FX-BLINDS-001") == drawn
    assert cleared(one("FX-BLINDS-002"), rows={2, 3, 6, 7})
    assert cleared(one("FX-BLINDS-003"), rows={8, 9})
    assert cleared(one("FX-BLINDS-004"), rows=set(range(H)))
    assert cleared(one("FX-BLINDS-005"), cols={0, 1, 4, 5, 8, 9, 12, 13})
    assert cleared(one("FX-BLINDS-006"), rows={0, 1, 4, 5, 8, 9})
    assert cleared(one("FX-BLINDS-007"), cols={2, 3, 6, 7, 10, 11, 14, 15})
    eight = kept(one("FX-BLINDS-008"))
    assert set(eight.values()) == {0, 1}
    assert 0.35 < sum(k == 0 for k in eight.values()) / len(eight) < 0.65
    assert one("FX-BLINDS-008") not in (one("FX-BLINDS-002"), one("FX-BLINDS-005"))
    assert one("FX-BLINDS-009") == one("FX-BLINDS-005")
    assert render(case(completion=50, width=4, angle=-3510), 0) == one("FX-BLINDS-005")
    assert cleared(one("FX-BLINDS-010"), rows={1, 3, 5, 7, 9})
    eleven = kept(one("FX-BLINDS-011"))
    for i, k in eleven.items():
        assert k == {0: 1, 1: 1, 2: 0.5, 3: 0, 4: 0}[(i // W) % 5], i
    twelve = kept(one("FX-BLINDS-012"))
    for i, k in twelve.items():
        assert abs(k - (1, 1, 0.95, 0.7, 0.45)[(i // W) % 5]) < 1e-12, (i, k)
    thirteen = kept(one("FX-BLINDS-013"))
    assert all(abs(k - 0.5) < 2e-4 for k in thirteen.values())
    assert max(thirteen.values()) > 0.5 > min(thirteen.values())
    fourteen = c["FX-BLINDS-014"]
    assert fourteen["0"] == drawn and cleared(fourteen["1"], rows={3, 7})
    assert fourteen["2"] == one("FX-BLINDS-002") == render(case(completion=50, width=4), 0)
    assert cleared(fourteen["3"], rows={1, 2, 3, 5, 6, 7, 9})
    assert cleared(fourteen["4"], rows=set(range(H)))
    for a, b in zip("0123", "1234"):  # cleared pixels never come back
        assert all(fourteen[b][i] == EMPTY for i in range(W * H) if fourteen[a][i] == EMPTY)
    fifteen = c["FX-BLINDS-015"]
    assert fifteen["0"] == fifteen["2"] == fourteen["1"]
    assert fifteen["3"] == fifteen["4"] == fourteen["3"]
    sixteen = c["FX-BLINDS-016"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert value_at(CASES["FX-BLINDS-016"][1]["completion"], 2) > 100
    assert sixteen["0"] == drawn and sixteen["2"] == sixteen["4"] == one("FX-BLINDS-004")
    seventeen = c["FX-BLINDS-017"]
    assert seventeen["0"] == one("FX-BLINDS-002")
    assert seventeen["2"] == one("FX-BLINDS-008")
    assert seventeen["4"] == render(case(completion=50, width=4, angle=60), 0)
    assert set(kept(seventeen["4"]).values()) == {0, 1} and seventeen["4"] != seventeen["2"]
    moved = c["FX-BLINDS-018"]
    assert moved["0"] == moved["3"]
    five = one("FX-BLINDS-005")
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == five[at(0, y):at(W - 3, y)]
        assert moved["0"][at(0, y):at(3, y)] == [EMPTY] * 3
    print("checked")


if __name__ == "__main__":
    main()
