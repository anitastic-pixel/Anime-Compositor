"""Twirl, worked a second way.

D-151 adds `core.twirl`: a drawing turned about a centre, most at the centre and less and less
further out, so straight lines bend into a spiral, as a whirlpool, a spell or a dizzy spell
does in a cartoon. Inside a circle of `radius` pixels about `center` (per cent of the drawing's
own width and height, as Radial Blur's centre is) each pixel takes the picture from a point
turned back about the centre; the turn is `angle` degrees at the centre itself and falls off
to none at the circle's edge. Pixels on or past the circle are kept exactly. A positive angle
turns the picture clockwise on the screen. It is this program's own method, modelled on After
Effects' Twirl; nothing is ported. Document 21 is the rule in words; this file is the reference
for the numbers document 25 pins against it.

D-744 (2026-10-23) brings it closer to After Effects, measured on tutorial stills: the turn
falls off as smoothstep, s(t) = t^2 (3 - 2 t), not as t^2, and the circle's size can be given
as After Effects gives it, `twirl_radius`, per cent of the drawing's larger side (0 to 100,
After Effects' range). `radius`, pixels, stays: a file from before D-744 holds it, and where it
is above 0 it is the circle's size, as before; only where it is 0 does `twirl_radius` set it.

D-845 (2026-10-25) fits the fall-off closer still: tutorial pairs from eight creators turn about
0.04 of the angle more than smoothstep from 0.6 to 1.0 of the reach, and the two cleanest (Jake
In Motion's grid, AM Film's photo) follow f(u) = (exp(-K u^2) - exp(-K)) / (1 - exp(-K)), with
u = d / r and K = 2.25, within 0.004 of the angle at every ring. It is still the whole angle at
the centre and none at the edge.

The rule. With w, h the drawing's own size, c the centre (center_x / 100 * w, center_y / 100 *
h) in the drawing's own pixels (an earlier effect that grew the layer does not move it) and P a
pixel's centre: the reach r is radius where radius > 0, else twirl_radius / 100 * max(w, h)
(twirl_radius 0 where the file has none). Angle 0 or r 0 leaves every pixel as it is. Otherwise
d = |P - c|; at d >= r the pixel is kept exactly; else t = 1 - d / r,
beta = radians(angle) f(d / r) (D-845's fall-off above), and the output is document 21's bilinear sample of the
layer at c + R(-beta)(P - c), transparent outside the layer, with
R(b)(x, y) = (x cos b - y sin b, x sin b + y cos b), clockwise on the screen for a positive b.
Premultiplied red, green, blue and alpha are sampled alike. The layer does not grow; the radius
is a distance, so a draft scales it, while twirl_radius is a share of the drawing's own size,
which a draft scales already.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a full-frame card crossed by two lines, drawn below. The drawing goes into
`Fixtures/twirl/media`, the projects into `Fixtures/twirl`, and the expected frames into
`Fixtures/twirl/expected_twirl.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/twirl_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import directional_blur_reference as D  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "twirl"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"angle": (-3600, 3600), "radius": (0, 10000), "center": (-1000, 1000),
          "twirl_radius": (0, 100)}
NAMES = ("angle", "radius", "center")


# --- the rule -------------------------------------------------------------------------------

K = 2.25  # D-845: fitted on Jake In Motion's and AM Film's twirls (2.23 and 2.27 apart)


def falloff(u):
    """D-845: the share of the angle at u = distance / reach, 1 at the centre, 0 at the edge."""
    return (math.exp(-K * u * u) - math.exp(-K)) / (1 - math.exp(-K))


def smoothstep(t):
    """D-744's fall-off, kept for the checks that compare against it."""
    return t * t * (3 - 2 * t)


def reach(radius, twirl_radius, w=W, h=H):
    """D-744: pixels where radius is above 0, else twirl_radius per cent of the larger side."""
    return radius if radius > 0 else twirl_radius / 100 * max(w, h)


def source(px, py, angle, radius, center, w=W, h=H, twirl_radius=0):
    """Where the pixel whose centre is (px, py), in the drawing's own pixels, takes its picture
    from; None where it is kept exactly."""
    radius = reach(radius, twirl_radius, w, h)
    if angle == 0 or radius == 0:
        return None
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    dx, dy = px - cx, py - cy
    d = math.sqrt(dx * dx + dy * dy)
    if d >= radius:
        return None
    b = math.radians(angle) * falloff(d / radius)
    # c + R(-b)(P - c)
    return (cx + dx * math.cos(b) + dy * math.sin(b), cy - dx * math.sin(b) + dy * math.cos(b))


def twirl(layer, angle, radius, center, drawing=(W, H), twirl_radius=0):
    """The layer's premultiplied pixels, twirled; its rectangle kept. `drawing` is the drawing's
    own w by h; the layer's `left` and `top` are the growth of earlier effects."""
    out = []
    for i, p in enumerate(layer["px"]):
        x, y = layer["left"] + i % layer["w"] + 0.5, layer["top"] + i // layer["w"] + 0.5
        at = source(x, y, angle, radius, center, *drawing, twirl_radius=twirl_radius)
        out.append(list(p) if at is None else bilinear(layer, *at))
    return out


# --- the drawing ----------------------------------------------------------------------------

SKIN = R.SKIN                    # #f6d6be
LINE = R.LINE                    # #1e1a24
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
SOFT = (246, 214, 190, 128)      # the skin at half covering, the card's soft left edge
NONE = S.NONE


def cross(x, y):
    """The whole frame filled: skin, crossed by a line down column 8 and a line along row 5,
    shadow in rows 8 and 9; column 0 the skin at half covering, a soft edge; and the top right
    corner, columns 14 and 15 of rows 0 and 1, empty."""
    if x >= 14 and y <= 1:
        return NONE
    if x == 0:
        return SOFT
    if x == 8 or y == 5:
        return LINE
    if y >= 8:
        return SHADE
    return SKIN


DRAWINGS = {"cross": [[cross(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(angle=90, radius=50, center=(50, 50), shift=0, before=None, twirl_radius=None):
    """`before` is a directional blur (direction, length) ahead of the twirl. `twirl_radius`
    None leaves it out of the file, as a file from before D-744 has it."""
    return {"drawing": "cross", "angle": angle, "radius": radius, "center": center,
            "shift": shift, "before": before, "twirl_radius": twirl_radius}


def names(c):
    return NAMES + (("twirl_radius",) if c["twirl_radius"] is not None else ())


def settings(c, frame_no):
    held = {"twirl_radius": 0}
    for k in names(c):
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def layer_of(c):
    """The drawing as the twirl receives it: grown by a directional blur ahead of it."""
    drawn = [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row]
    layer = {"px": drawn, "left": 0, "top": 0, "w": W, "h": H}
    if c["before"]:
        direction, length = c["before"]
        g = math.ceil(length / 2)
        layer = {"px": [D.blurred(drawn, direction, length, x, y)
                        for y in range(-g, H + g) for x in range(-g, W + g)],
                 "left": -g, "top": -g, "w": W + 2 * g, "h": H + 2 * g}
    return layer


def render(c, frame_no):
    layer = layer_of(c)
    s = settings(c, frame_no)
    px = twirl(layer, s["angle"], s["radius"], s["center"], twirl_radius=s["twirl_radius"])
    out = []
    for y in range(H):
        for x in range(W):
            lx, ly = x - c["shift"] - layer["left"], y - layer["top"]
            inside = 0 <= lx < layer["w"] and 0 <= ly < layer["h"]
            out.append(px[ly * layer["w"] + lx] if inside else [0.0] * 4)
    return out


def plain(c):
    """The drawing untouched, moved and grown as the case moves and grows it."""
    return render(case(angle=0, shift=c["shift"], before=c["before"]), 0)


CASES = {
    "FX-TWIRL-001": ("The settings as they start: angle 90, radius 50, about the middle, the "
                     "point (8, 5). The whole drawing lies inside the circle, so every pixel is "
                     "turned clockwise, by about 90 degrees beside the centre and by about 83 "
                     "at the corners: the two lines bend into a spiral about the middle, and "
                     "column 15 and rows 2 to 9 of column 0, which take their picture from past "
                     "the drawing's top and bottom edges, are empty. Pixels deep in the skin "
                     "take skin and look unchanged.",
                     case(), [0]),
    "FX-TWIRL-002": ("Angle -90: the same twirl the other way, counter-clockwise; each pixel "
                     "takes its picture from the point FX-TWIRL-001 takes it from, mirrored "
                     "across the line from the centre to the pixel.",
                     case(angle=-90), [0]),
    "FX-TWIRL-003": ("Angle 0: the drawing, untouched.",
                     case(angle=0), [0]),
    "FX-TWIRL-004": ("Radius 0: the drawing, untouched.",
                     case(radius=0), [0]),
    "FX-TWIRL-005": ("Radius 4: only the pixels whose centres are less than 4 pixels from the "
                     "middle are turned, the crossing of the two lines swirled; every pixel 4 "
                     "or more from it is kept exactly.",
                     case(radius=4), [0]),
    "FX-TWIRL-006": ("Angle 360, radius 8: a whole turn at the very centre, less further out, "
                     "so the lines wind round the middle; the corners, 8 or more from it, are "
                     "kept exactly.",
                     case(angle=360, radius=8), [0]),
    "FX-TWIRL-007": ("Angle 3600, the most: ten turns at the centre, the picture wound so "
                     "tightly that the middle pixels sample it almost at random.",
                     case(angle=3600), [0]),
    "FX-TWIRL-008": ("Radius 10000: the falloff is so slow that every pixel is turned by "
                     "within a fifth of a degree of 90, nearly a plain quarter turn about the "
                     "middle: the drawing stands on end in columns 3 to 12, the line down column "
                     "8 now runs along row 5 and the line along row 5 runs down column 7, and "
                     "the three columns on either side, turned in from past the drawing's top "
                     "and bottom, are empty or nearly.",
                     case(radius=10000), [0]),
    "FX-TWIRL-009": ("Centre 0, 0, the top left corner, radius 6: only the pixels less than 6 "
                     "from that corner are turned, the soft edge among them; the rest is kept "
                     "exactly.",
                     case(center=(0, 0), radius=6), [0]),
    "FX-TWIRL-010": ("Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the "
                     "drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180.",
                     case(angle=keyed((0, 0), (4, 180))), [0, 2, 4]),
    "FX-TWIRL-011": ("Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the "
                     "drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every "
                     "pixel further than radius 50 does.",
                     case(radius=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-TWIRL-012": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 "
                     "is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top "
                     "left corner.",
                     case(center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-TWIRL-013": ("Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 3600, is held at 3600, and is "
                     "FX-TWIRL-007; frame 0 is the drawing.",
                     case(angle=keyed((0, 0, OVERSHOOT), (4, 3600))), [0, 2]),
    "FX-TWIRL-014": ("FX-TWIRL-001 moved three pixels right: the same, moved; the twirl moves "
                     "with the drawing, and nothing is drawn left of the drawing's edge.",
                     case(shift=3), [0, 3]),
    "FX-TWIRL-015": ("A directional blur, direction 90 and length 4, then angle 90 about centre "
                     "25, 25: the blur grew the layer two pixels on every side, its grown "
                     "pixels are turned too, and the centre is still the drawing's own point "
                     "(4, 2.5), not a point of the grown layer.",
                     case(center=(25, 25), before=(90, 4)), [0]),
    "FX-TWIRL-022": ("D-744, the settings as a new Twirl starts: angle 90, radius 0, twirl "
                     "radius 30, After Effects' start, about the middle. The circle is 30 per "
                     "cent of the drawing's larger side, 16, so 4.8 pixels: the crossing is "
                     "swirled, a little wider than FX-TWIRL-005's radius 4, and every pixel 4.8 "
                     "or more from the middle is kept exactly.",
                     case(radius=0, twirl_radius=30), [0]),
    "FX-TWIRL-023": ("Radius 4 with twirl radius 100: radius, in pixels, is above 0, so it is "
                     "the circle's size and twirl radius is not used; FX-TWIRL-005 exactly.",
                     case(radius=4, twirl_radius=100), [0]),
    "FX-TWIRL-024": ("Radius 0, twirl radius keyed from 0 at frame 0 to 60 at frame 4, linear: "
                     "frame 0 the drawing, frame 2 FX-TWIRL-022, and frame 4 a circle of 9.6 "
                     "pixels.",
                     case(radius=0, twirl_radius=keyed((0, 0), (4, 60))), [0, 2, 4]),
    "FX-TWIRL-025": ("FX-TWIRL-015's directional blur, then radius 0, twirl radius 50 about "
                     "centre 25, 25: the circle is half the drawing's own larger side, 8 "
                     "pixels, not half the grown layer's 20.",
                     case(radius=0, twirl_radius=50, center=(25, 25), before=(90, 4)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TWIRL-016": ("Angle 3601, above 3600.", case(angle=3601)),
    "FX-TWIRL-017": ("Angle -3601, below -3600.", case(angle=-3601)),
    "FX-TWIRL-018": ("Radius -1, below 0.", case(radius=-1)),
    "FX-TWIRL-019": ("Radius 10001, above 10000.", case(radius=10001)),
    "FX-TWIRL-020": ("Centre 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-TWIRL-021": ("Radius keyed to 20000 at frame 4.",
                     case(radius=keyed((0, 50), (4, 20000)))),
    "FX-TWIRL-026": ("Twirl radius 101, above 100.", case(radius=0, twirl_radius=101)),
    "FX-TWIRL-027": ("Twirl radius -1, below 0.", case(radius=0, twirl_radius=-1)),
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
    effects.append({
        "instance_id": f"fx-0-{len(effects)}", "type_id": "core.twirl", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in names(c)}})
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

    (OUT / "expected_twirl.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    dist = lambda x, y, cx=8, cy=5: math.hypot(x + 0.5 - cx, y + 0.5 - cy)  # noqa: E731
    turn = lambda x, y, angle=90, radius=50: angle * falloff(dist(x, y) / radius)  # noqa: E731

    # The rule's own pieces. A positive angle turns the picture clockwise: a pixel right of the
    # centre takes the picture from above it, one below from the right of it.
    sx, sy = source(12, 5, 90, 1e300, (50, 50))
    assert abs(sx - 8) < 1e-12 and abs(sy - 1) < 1e-12
    sx, sy = source(8, 9, 90, 1e300, (50, 50))
    assert abs(sx - 12) < 1e-12 and abs(sy - 5) < 1e-12
    # The turn is the angle at the centre, falls off as D-845's f of the distance, and the
    # distance from the centre is kept.
    assert source(8, 5, 90, 50, (50, 50)) == (8, 5)
    for x in range(W):
        for y in range(H):
            q = source(x + 0.5, y + 0.5, 90, 50, (50, 50))
            assert abs(math.hypot(q[0] - 8, q[1] - 5) - dist(x, y)) < 1e-12
            got = math.degrees(math.atan2(y + 0.5 - 5, x + 0.5 - 8)
                               - math.atan2(q[1] - 5, q[0] - 8)) % 360
            assert abs(got - turn(x, y)) < 1e-9, (x, y)
    # On or past the circle nothing moves, and no fixture pixel sits within 1e-5 of it.
    assert source(8 + 4, 5, 90, 4, (50, 50)) is None and source(8 + 3.999, 5, 90, 4, (50, 50))
    for radius, cx, cy in ((4, 8, 5), (8, 8, 5), (6, 0, 0), (50, 8, 5), (4.8, 8, 5), (9.6, 8, 5)):
        assert all(abs(dist(x, y, cx, cy) - radius) > 1e-5 for x in range(W) for y in range(H))

    # Every valid case keeps each channel inside its covering; the invalid ones are the drawing.
    for fx, frames in c.items():
        s = expected["cases"][fx]
        base = plain(CASES[fx][1] if fx in CASES else INVALID[fx][1])
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= u <= p[3] + 1e-12
                                                          for u in p[:3]), (fx, p)
        if "warning" in s:
            assert all(px == base for px in frames.values())

    one = c["FX-TWIRL-001"]["0"]
    assert all(source(x + 0.5, y + 0.5, 90, 50, (50, 50)) for x in range(W) for y in range(H))
    assert 89.9 < turn(7, 4) < 90 and 83 < turn(0, 0) < 84 and 83 < turn(15, 9) < 84
    # The lines bend: the crossing's arms leave column 8 and row 5.
    assert one[at(8, 1)] != drawn[at(8, 1)] and one[at(2, 5)] != drawn[at(2, 5)]
    assert all(one[at(15, y)] == [0.0] * 4 for y in range(H))
    assert all(one[at(0, y)] == [0.0] * 4 for y in range(2, H))
    assert sum(one[i] != drawn[i] for i in range(W * H)) > 120
    two = c["FX-TWIRL-002"]["0"]
    assert two != one
    for x in range(W):
        for y in range(H):
            p = (x + 0.5 - 8, y + 0.5 - 5)
            q1, q2 = (source(x + 0.5, y + 0.5, a, 50, (50, 50)) for a in (90, -90))
            q1, q2 = (q1[0] - 8, q1[1] - 5), (q2[0] - 8, q2[1] - 5)
            n = math.hypot(*p)
            u = (p[0] / n, p[1] / n)
            along = q1[0] * u[0] + q1[1] * u[1]
            mirrored = (2 * along * u[0] - q1[0], 2 * along * u[1] - q1[1])
            assert abs(mirrored[0] - q2[0]) < 1e-12 and abs(mirrored[1] - q2[1]) < 1e-12
    assert c["FX-TWIRL-003"]["0"] == drawn and c["FX-TWIRL-004"]["0"] == drawn
    for fx, radius, cx, cy in (("FX-TWIRL-005", 4, 8, 5), ("FX-TWIRL-006", 8, 8, 5),
                               ("FX-TWIRL-009", 6, 0, 0)):
        got = c[fx]["0"]
        for x in range(W):
            for y in range(H):
                if dist(x, y, cx, cy) >= radius:
                    assert got[at(x, y)] == drawn[at(x, y)], (fx, x, y)
        assert sum(got[i] != drawn[i] for i in range(W * H)) > 4, fx
    assert all(c["FX-TWIRL-006"]["0"][at(x, y)] == drawn[at(x, y)] for x in (0, 15) for y in (0, 9))
    assert c["FX-TWIRL-009"]["0"][at(0, 1)] != drawn[at(0, 1)]  # the soft edge is turned
    assert c["FX-TWIRL-007"]["0"] != one
    # Radius 10000: within a fifth of a degree of a quarter turn; the lines swap over.
    eight = c["FX-TWIRL-008"]["0"]
    assert all(89.8 < turn(x, y, radius=10000) < 90 for x in range(W) for y in range(H))
    lum = lambda p: p[0] / p[3]  # noqa: E731  (red over covering: low on the line)
    line, skin = R.working(LINE), R.working(SKIN)
    assert all(abs(lum(eight[at(x, 5)]) - lum(line)) < 0.01 for x in range(4, 12) if x not in (7,))
    assert all(abs(lum(eight[at(7, y)]) - lum(line)) < 0.01 for y in range(1, 9))
    assert abs(lum(eight[at(9, 2)]) - lum(skin)) < 0.01
    assert lum(drawn[at(9, 5)]) == lum(line) and lum(drawn[at(7, 2)]) == lum(skin)
    assert all(eight[at(x, y)][3] < 0.02 for x in (0, 1, 2, 13, 14, 15) for y in range(H))
    assert all(eight[at(x, y)][3] > 0.98 for x in range(3, 13) for y in range(H))
    ten, eleven, twelve = c["FX-TWIRL-010"], c["FX-TWIRL-011"], c["FX-TWIRL-012"]
    assert ten["0"] == drawn and near(ten["2"], one) and near(ten["4"], render(case(angle=180), 0))
    assert eleven["0"] == drawn and near(eleven["2"], one)
    assert all(turn(x, y, radius=100) > turn(x, y) for x in range(W) for y in range(H))
    assert near(eleven["4"], render(case(radius=100), 0))
    assert near(twelve["0"], one) and near(twelve["4"], render(case(center=(0, 0)), 0))
    assert near(twelve["2"], render(case(center=(25, 25)), 0))
    thirteen = c["FX-TWIRL-013"]
    assert ease(OVERSHOOT, 0.5) > 1 and thirteen["0"] == drawn
    assert thirteen["2"] == c["FX-TWIRL-007"]["0"]
    moved = c["FX-TWIRL-014"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # After a directional blur: turned about the drawing's own (4, 2.5), not about the point
    # 25 per cent of the grown layer from its corner, nor 4, 2.5 from the grown layer's corner.
    grown = case(center=(25, 25), before=(90, 4))
    fifteen = c["FX-TWIRL-015"]["0"]
    layer = layer_of(grown)
    lw, lh = layer["w"], layer["h"]
    inside = lambda px: [px[(y + 2) * lw + x + 2] for y in range(H) for x in range(W)]  # noqa: E731
    assert near(inside(twirl(layer, 90, 50, (25, 25))), fifteen)
    wrong_a = twirl(layer, 90, 50, (25, 25), drawing=(lw, lh))
    wrong_b = twirl(dict(layer, left=0, top=0), 90, 50, (25, 25))
    assert not near(inside(wrong_a), fifteen, 1e-6) and not near(inside(wrong_b), fifteen, 1e-6)
    assert fifteen != render(case(center=(25, 25)), 0)
    # D-744: the reach is radius where it is above 0, else twirl_radius of the larger side.
    assert reach(50, 30) == 50 and reach(0, 30) == 0.3 * 16 and reach(0, 0) == 0
    assert smoothstep(0) == 0 and smoothstep(1) == 1 and smoothstep(0.5) == 0.5
    assert abs(smoothstep(0.25) - 0.15625) < 1e-15  # gentler than t^2's 0.0625 near the edge
    # D-845: whole at the centre, none at the edge, falling all the way; half the angle at 0.51
    # of the reach (smoothstep 0.5), and about 0.04 more than smoothstep from 0.6 to 0.9, as
    # the tutorials measured (Jake In Motion 0.38 and 0.17 at 0.6 and 0.78).
    assert falloff(0) == 1 and abs(falloff(1)) < 1e-15
    assert all(falloff(i / 100) > falloff((i + 1) / 100) for i in range(100))
    assert abs(falloff(0.51) - 0.505) < 0.002
    assert all(0.02 < falloff(u) - smoothstep(1 - u) < 0.05 for u in (0.6, 0.7, 0.8, 0.9))
    assert abs(falloff(0.6) - 0.379) < 0.002 and abs(falloff(0.78) - 0.167) < 0.002
    # Grown-layer pixel centres of FX-TWIRL-025 stay clear of its circle, 8 about (4, 2.5).
    assert all(abs(math.hypot(x + 0.5 - 4, y + 0.5 - 2.5) - 8) > 1e-5
               for x in range(-2, W + 2) for y in range(-2, H + 2))
    t22 = c["FX-TWIRL-022"]["0"]
    for x in range(W):
        for y in range(H):
            if dist(x, y) >= 4.8:
                assert t22[at(x, y)] == drawn[at(x, y)], (x, y)
    assert sum(t22[i] != drawn[i] for i in range(W * H)) > sum(
        c["FX-TWIRL-005"]["0"][i] != drawn[i] for i in range(W * H))
    assert c["FX-TWIRL-023"]["0"] == c["FX-TWIRL-005"]["0"]
    t24 = c["FX-TWIRL-024"]
    assert t24["0"] == drawn and near(t24["2"], t22)
    assert near(t24["4"], render(case(radius=9.6), 0))
    t25 = c["FX-TWIRL-025"]["0"]
    layer = layer_of(case(before=(90, 4)))
    assert near(inside(twirl(layer, 90, 8, (25, 25))), t25)
    assert not near(inside(twirl(layer, 90, 10, (25, 25))), t25, 1e-6)
    print("checked")


if __name__ == "__main__":
    main()
