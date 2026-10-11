"""Burn Film, worked a second way.

D-460 adds `core.burn_film`, after After Effects' CC Burn Film (Stylize): film burning or
melting as it sticks in a projector. Holes open first round a centre point and then at other
places, each ringed by a scorched edge and a black char line, spreading until the layer is gone.
After Effects' controls are Burn, Center and Random Seed. What is known (the CycoreFX HD 1.8.1
manual's page on CC Burn Film, an Adobe community thread and a Creative Cow thread on making it
look more real) is: Burn sets how much has burned and is keyed to animate it; Center is where
the burning first appears; Random Seed sets where the other burns start; the burned parts become
transparent, so what is beneath shows through; and the edge colour is black, with no control to
change it. The rule below is this program's own, and nothing is ported.

The rule, for a layer w by h pixels as it reaches the effect (its own buffer, grown or not by the
effects above), D = sqrt(w^2 + h^2) its diagonal:

1. The burn points. The first is the centre, c = (Center / 100 w, Center / 100 h), starting at
   once (delay 0). Then six more, i = 1 to 6, each from D-119's number u(seed, i, 0, -1, ch) in
   -1..1 (seed = Random Seed's whole part): at ((u0 + 1) / 2 w, (u1 + 1) / 2 h), with delay
   0.25 + 0.125 (u2 + 1), between 0.25 and 0.5.
2. Each pixel's burn time, at its centre p: the least, over the points q, of |p - q| / D plus
   the point's delay, plus a ragged edge 0.12 (n + 1) / 2, where n is D-127's fractal value
   noise in -1..1 (six octaves, the seed's, channel 0) at (p / (D / 10), depth 0). The time is
   never below 0.
3. M, the latest any pixel can burn: the farthest of the buffer's four corners from c, over D,
   plus 0.12. The burn front s = Burn / 100 (M + 0.08) - 0.06, so at Burn 0 it sits 0.06 before
   any pixel and at Burn 100 0.02 past the last.
4. g = the pixel's time - s. g >= 0.06: the pixel as it is. 0 <= g < 0.06, scorched: its colour
   times g / 0.06, darkening to black at the front, its covering unchanged. -0.02 < g < 0, the
   char: black, its covering times (1 + g / 0.02), fading out. g <= -0.02: burned away, empty.
   Every step meets the next without a jump.

The layer never grows. Settings: `burn` 0 to 100, 0 when added; `center` two numbers, per cent of
the picture, -1000 to 1000 each, (50, 50); `seed` 0 to 100000, 0, its whole part. All three can
be keyed. Everything is measured against the picture's size, so a draft burns the same shapes.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding D-144's strip (`tools/mosaic_reference.py`),
unmoved unless the case says. The expected frames are in
`Fixtures/burn_film/expected_burn_film.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/burn_film_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from noise_reference import u as U  # noqa: E402
from fractal_noise_reference import F  # noqa: E402
import smooth_reference as S  # noqa: E402
import mosaic_reference as MO  # noqa: E402

W, H = MO.W, MO.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "burn_film"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"burn": (0, 100), "center": (-1000, 1000), "seed": (0, 100000)}
NAMES = tuple(RANGES)
EMPTY = [0.0] * 4
ROUGH, SCORCH, CHAR, POINTS, OCTAVES = 0.12, 0.06, 0.02, 6, 6


# --- the rule -------------------------------------------------------------------------------

def points(w, h, center, seed):
    out = [((center[0] / 100 * w, center[1] / 100 * h), 0.0)]
    for i in range(1, POINTS + 1):
        u = [U(seed, i, 0, -1, ch) for ch in range(3)]
        out.append((((u[0] + 1) / 2 * w, (u[1] + 1) / 2 * h), 0.25 + 0.125 * (u[2] + 1)))
    return out


def time_at(px, py, w, h, pts, seed):
    d = math.hypot(w, h)
    near = min(math.hypot(px - q[0], py - q[1]) / d + t for q, t in pts)
    n = F(seed, 0, px / (d / 10), py / (d / 10), 0.0, OCTAVES)
    return near + ROUGH * (n + 1) / 2


def front(w, h, center, burn):
    d = math.hypot(w, h)
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    m = max(math.hypot(x - cx, y - cy) for x in (0, w) for y in (0, h)) / d + ROUGH
    return burn / 100 * (m + SCORCH + CHAR) - SCORCH


def burn_film(layer, v):
    w, h = layer["w"], layer["h"]
    seed = math.floor(v["seed"])
    pts = points(w, h, v["center"], seed)
    s = front(w, h, v["center"], v["burn"])
    out = []
    for y in range(h):
        for x in range(w):
            p = layer["px"][y * w + x]
            g = time_at(x + 0.5, y + 0.5, w, h, pts, seed) - s
            if g >= SCORCH:
                out.append(list(p))
            elif g >= 0:
                k = g / SCORCH
                out.append([p[0] * k, p[1] * k, p[2] * k, p[3]])
            elif g > -CHAR:
                out.append([0.0, 0.0, 0.0, p[3] * (1 + g / CHAR)])
            else:
                out.append(list(EMPTY))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(burn=0, center=(50, 50), seed=0, shift=0, before=None):
    c = dict(locals())
    c["drawing"] = "strip"
    return c


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, a)) for a in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    layer = MO.layer_of(c)
    return MO.placed(layer, burn_film(layer, settings(c, frame_no)), c["shift"])


def plain(c):
    return MO.plain(c)


CASES = {
    "FX-BURN-001": ("The settings as they start: Burn 0, nothing burned, the drawing as it is.",
                    case(), [0]),
    "FX-BURN-002": ("Burn 100: everything burned away, the layer empty.", case(burn=100), [0]),
    "FX-BURN-003": ("Burn 30: a hole has opened round the middle, ringed by black char and a "
                    "scorched edge; the corners untouched.", case(burn=30), [0]),
    "FX-BURN-004": ("Burn 50: the hole has spread over most of the middle.", case(burn=50), [0]),
    "FX-BURN-005": ("Burn 75: nearly all of the layer gone, scorched edges round what is left.",
                    case(burn=75), [0]),
    "FX-BURN-006": ("Center 0, 0, Burn 25: the burning starts at the top left corner.",
                    case(burn=25, center=(0, 0)), [0]),
    "FX-BURN-007": ("Center 100, 50, the middle of the right edge, Burn 25: the burning starts "
                    "at the right, and the middle is untouched.", case(burn=25, center=(100, 50)),
                    [0]),
    "FX-BURN-008": ("Random Seed 7, Burn 50: the other burns start elsewhere, and the edges "
                    "are ragged differently, from FX-BURN-004.", case(burn=50, seed=7), [0]),
    "FX-BURN-009": ("Random Seed 7.9: its whole part, FX-BURN-008.", case(burn=50, seed=7.9),
                    [0]),
    "FX-BURN-010": ("Burn keyed from 0 at frame 0 to 100 at frame 4, linear: the drawing, then "
                    "Burn 25, 50 and 75, then empty.",
                    case(burn=keyed((0, 0), (4, 100))), [0, 1, 2, 3, 4]),
    "FX-BURN-011": ("Center keyed from 25, 50 at frame 0 to 75, 50 at frame 4, Burn 20: the "
                    "first hole moves across.",
                    case(burn=20, center=keyed((0, (25, 50)), (4, (75, 50)))), [0, 2, 4]),
    "FX-BURN-012": ("Burn keyed from 30 at frame 0 to 0 at frame 4, eased past its end: below 0 "
                    "it is held at 0, the drawing as it is.",
                    case(burn=keyed((0, 30, OVERSHOOT), (4, 0))), [0, 2, 4]),
    "FX-BURN-013": ("FX-BURN-004 moved three pixels right: the burn belongs to the layer, so it "
                    "moves with it.", case(burn=50, shift=3), [0]),
    "FX-BURN-014": ("After a directional blur (direction 90, length 4) that grows the layer two "
                    "pixels on every side, Burn 50: the burn is laid over the grown layer, 20 "
                    "by 14, its middle and size.", case(burn=50, before=(90, 4)), [0]),
}

INVALID = {
    "FX-BURN-015": ("Burn 101, above 100.", case(burn=101)),
    "FX-BURN-016": ("Burn -1, below 0.", case(burn=-1)),
    "FX-BURN-017": ("Center at 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-BURN-018": ("Random Seed -1, below 0.", case(seed=-1)),
    "FX-BURN-019": ("Random Seed 100001, above 100000.", case(seed=100001)),
    "FX-BURN-020": ("Burn keyed to 150 at frame 4, above 100.",
                    case(burn=keyed((0, 0), (4, 150)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.burn_film",
                    "enabled": True, "parameters": {k: setting_json(c[k]) for k in NAMES}})
    comp["layers"][0]["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def kinds(px, before):
    """How many pixels are untouched, scorched, char and empty (of those that showed)."""
    n = [0, 0, 0, 0]
    for p, q in zip(px, before):
        if q[3] == 0:
            continue
        n[0 if p == q else 3 if p == EMPTY else 2 if p[:3] == [0.0] * 3 else 1] += 1
    return n


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
            f"frame {f} untouched/scorched/char/empty {kinds(px, before)}"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_burn_film.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731
    drawn = plain(case())
    showing = sum(p[3] > 0 for p in drawn)
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), fx
    assert one("FX-BURN-001") == drawn
    assert all(p == EMPTY for p in one("FX-BURN-002"))
    # The burn times never fall below 0, so Burn 0 touches nothing.
    pts = points(W, H, (50, 50), 0)
    assert min(time_at(x + 0.5, y + 0.5, W, H, pts, 0) for x in range(W) for y in range(H)) >= 0
    three, four, five = (kinds(one(f), drawn) for f in ("FX-BURN-003", "FX-BURN-004",
                                                         "FX-BURN-005"))
    # Each step of the burn shows all four kinds of pixel, and burns more than the last.
    for n in (three, four):
        assert all(k > 0 for k in n), n
    assert three[3] < four[3] < five[3] and five[0] < four[0] < three[0]
    # Burn only ever takes away: covering and colour never rise as Burn does.
    for a, b in (("FX-BURN-001", "FX-BURN-003"), ("FX-BURN-003", "FX-BURN-004"),
                 ("FX-BURN-004", "FX-BURN-005"), ("FX-BURN-005", "FX-BURN-002")):
        assert all(q[k] <= p[k] + 1e-12 for p, q in zip(one(a), one(b)) for k in range(4)), (a, b)
    # Burn 15 eats the middle first: the corners of the drawing's skin are untouched.
    three_px = one("FX-BURN-003")
    assert three_px[at(8, 5)] == EMPTY
    assert three_px[at(4, 0)] == drawn[at(4, 0)] and three_px[at(11, 0)] == drawn[at(11, 0)]
    six = one("FX-BURN-006")
    assert six[at(3, 0)] != drawn[at(3, 0)] and six[at(15, 9)] == drawn[at(15, 9)]
    seven = one("FX-BURN-007")
    assert seven[at(13, 5)] != drawn[at(13, 5)] and seven[at(4, 9)] == drawn[at(4, 9)]
    assert seven[at(8, 5)] == drawn[at(8, 5)] and render(case(burn=25), 0)[at(8, 5)] != drawn[at(8, 5)]
    assert one("FX-BURN-008") != one("FX-BURN-004") and one("FX-BURN-009") == one("FX-BURN-008")
    ten = c["FX-BURN-010"]
    assert ten["0"] == drawn and all(p == EMPTY for p in ten["4"])
    assert ten["2"] == render(case(burn=50), 0)
    eleven = c["FX-BURN-011"]
    assert len({json.dumps(eleven[f]) for f in ("0", "2", "4")}) == 3
    assert eleven["2"] == render(case(burn=20), 0)
    raw = value_at(CASES["FX-BURN-012"][1]["burn"], 2)
    twelve = c["FX-BURN-012"]
    assert raw < 0 and twelve["2"] == twelve["4"] == drawn
    assert twelve["0"] == render(case(burn=30), 0)
    thirteen = one("FX-BURN-013")
    four_px = one("FX-BURN-004")
    assert all(thirteen[at(x, y)] == four_px[at(x - 3, y)] for x in range(3, W) for y in range(H))
    fourteen = one("FX-BURN-014")
    assert fourteen != render(case(burn=50, before=None), 0)
    for fx in INVALID:
        assert expected["cases"][fx]["frames"]["0"] == drawn
    print("checked", showing, "showing")


if __name__ == "__main__":
    main()
