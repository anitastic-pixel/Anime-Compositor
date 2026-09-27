"""Radial Wipe, worked a second way.

D-156 adds `core.radial_wipe`: a drawing wiped away like the hand of a clock sweeping round a
centre, a transition used to reveal the picture below or to clear a card off the screen. From
`start_angle` (0 is straight up, 90 to the right) the wipe sweeps clockwise, counter-clockwise, or
both ways at once from the start line, and `completion` says how much of the full turn has been
swept away: at 0 the drawing is untouched, at 100 nothing is left. The centre is `center`, per
cent of the drawing's own width and height as Radial Blur's centre is. `feather`, in degrees,
softens the sweeping edge into a fade that many degrees wide. The wipe multiplies each pixel's
colour and covering alike, so a soft edge stays soft and a pixel that does not show stays as it
is. It is this program's own method, modelled on After Effects' Radial Wipe; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. At completion 0 the output is the input exactly; at completion 100 every pixel is
transparent, all four channels 0, the size unchanged. Otherwise, with w, h the drawing's own
size, c the centre (center_x / 100 * w, center_y / 100 * h) in the drawing's own pixels (an
earlier effect that grew the layer does not move it) and P a pixel's centre in the drawing's own
pixels (a grown pixel's too): the screen angle of P - c, atan2(x, -y) in degrees, 0 straight up
and growing clockwise, alpha(0, 0) = 0, less start_angle, taken into [0, 360), is a. Clockwise:
a* = a; counterclockwise: a* = 360 - a when a > 0, else 0; both: a* = 2 min(a, 360 - a). With
F = feather and T = completion / 100 (360 + F) - F / 2: k = clamp((a* - T) / F + 0.5, 0, 1) when
F > 0, else 1 when a* >= T and 0 otherwise; the output is p k, all four premultiplied channels.
The layer does not grow; nothing is scaled for a draft (the feather is in degrees, not a
distance). The hard edge is a cliff, and so is the start line, where a jumps from just under 360
to 0 (except for "both"): `check` asserts that no pixel sits within 1e-5 of either.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a full-frame card crossed by two lines, drawn below. The drawing goes into
`Fixtures/radial_wipe/media`, the projects into `Fixtures/radial_wipe`, and the expected frames
into `Fixtures/radial_wipe/expected_radial_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/radial_wipe_reference.py
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
import directional_blur_reference as D  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "radial_wipe"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"completion": (0, 100), "start_angle": (-3600, 3600), "center": (-1000, 1000),
          "feather": (0, 360)}
NUMBERS = ("completion", "start_angle", "center", "feather")
NAMES = ("completion", "start_angle", "center", "wipe", "feather")
CLIFF = 1e-5   # no decided pixel may sit closer than this to the edge or the start line


# --- the rule -------------------------------------------------------------------------------

def into_turn(a):
    """An angle in degrees taken into [0, 360)."""
    a %= 360.0
    return 0.0 if a >= 360.0 else a


def screen_angle(vx, vy):
    """alpha(v): 0 straight up, 90 to the right, growing clockwise on the screen."""
    if vx == 0 and vy == 0:
        return 0.0
    return into_turn(math.degrees(math.atan2(vx, -vy)))


def swept(px, py, start_angle, center, wipe, w=W, h=H):
    """(a, a*) at the point (px, py) of the drawing's own pixels."""
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    a = into_turn(screen_angle(px - cx, py - cy) - start_angle)
    if wipe == "counterclockwise":
        return a, (360 - a if a > 0 else 0.0)
    if wipe == "both":
        return a, 2 * min(a, 360 - a)
    return a, a


def edge(completion, feather):
    """T, where the sweeping edge sits in a*."""
    return completion / 100 * (360 + feather) - feather / 2


def strength(px, py, completion, start_angle, center, wipe, feather, w=W, h=H):
    """k at the point (px, py): 1 where the drawing is kept, 0 where it is wiped away."""
    if completion == 0:
        return 1.0
    if completion == 100:
        return 0.0
    t = edge(completion, feather)
    a_star = swept(px, py, start_angle, center, wipe, w, h)[1]
    if feather > 0:
        return min(1.0, max(0.0, (a_star - t) / feather + 0.5))
    return 1.0 if a_star >= t else 0.0


def radial_wipe(layer, completion, start_angle, center, wipe, feather, drawing=(W, H)):
    """The layer's premultiplied pixels, wiped; its rectangle kept. `drawing` is the drawing's
    own w by h; the layer's `left` and `top` are the growth of earlier effects."""
    out = []
    for i, p in enumerate(layer["px"]):
        x, y = layer["left"] + i % layer["w"] + 0.5, layer["top"] + i // layer["w"] + 0.5
        k = strength(x, y, completion, start_angle, center, wipe, feather, *drawing)
        out.append(list(p) if k == 1 else [v * k for v in p])
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

def case(completion=0, start_angle=0, center=(50, 50), wipe="clockwise", feather=0, shift=0,
         before=None):
    """`before` is a directional blur (direction, length) ahead of the wipe."""
    return {"drawing": "cross", "completion": completion, "start_angle": start_angle,
            "center": center, "wipe": wipe, "feather": feather, "shift": shift,
            "before": before}


def settings(c, frame_no):
    held = {}
    for k in NUMBERS:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def layer_of(c):
    """The drawing as the wipe receives it: grown by a directional blur ahead of it."""
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
    px = radial_wipe(layer, s["completion"], s["start_angle"], s["center"], c["wipe"],
                     s["feather"])
    out = []
    for y in range(H):
        for x in range(W):
            lx, ly = x - c["shift"] - layer["left"], y - layer["top"]
            inside = 0 <= lx < layer["w"] and 0 <= ly < layer["h"]
            out.append(px[ly * layer["w"] + lx] if inside else [0.0] * 4)
    return out


def plain(c):
    """The drawing untouched, moved and grown as the case moves and grows it."""
    return render(case(completion=0, shift=c["shift"], before=c["before"]), 0)


CASES = {
    "FX-RWIPE-001": ("The settings as they start: completion 0, start angle 0, centre 50, 50, "
                     "clockwise, feather 0: the drawing, untouched.",
                     case(), [0]),
    "FX-RWIPE-002": ("Completion 25: a quarter turn swept clockwise from straight up about the "
                     "middle, the point (8, 5), so the top right quarter, columns 8 to 15 of "
                     "rows 0 to 4, is transparent, and every other pixel is kept exactly.",
                     case(completion=25), [0]),
    "FX-RWIPE-003": ("Completion 50: half a turn swept, from straight up round to straight "
                     "down, so the right half, columns 8 to 15, is transparent; the line down "
                     "column 8 goes with it, and the left half is kept exactly.",
                     case(completion=50), [0]),
    "FX-RWIPE-004": ("Completion 75: three quarters swept, so only the top left quarter, columns "
                     "0 to 7 of rows 0 to 4, the soft edge among it, is left.",
                     case(completion=75), [0]),
    "FX-RWIPE-005": ("Completion 100: every pixel transparent, all four channels 0.",
                     case(completion=100), [0]),
    "FX-RWIPE-006": ("Counterclockwise, completion 25: the quarter turn swept the other way from "
                     "straight up, so the top left quarter, columns 0 to 7 of rows 0 to 4, is "
                     "transparent instead.",
                     case(completion=25, wipe="counterclockwise"), [0]),
    "FX-RWIPE-007": ("Both, completion 50: the wipe sweeps both ways from straight up at once, a "
                     "quarter turn each way, so the top half, rows 0 to 4, is transparent and "
                     "the line along row 5 and everything below it is kept.",
                     case(completion=50, wipe="both"), [0]),
    "FX-RWIPE-008": ("Start angle 90, completion 25: the quarter turn swept clockwise from "
                     "straight right, so the bottom right quarter, columns 8 to 15 of rows 5 to "
                     "9, is transparent.",
                     case(completion=25, start_angle=90), [0]),
    "FX-RWIPE-009": ("Start angle 30, completion 50: half a turn swept from a line 30 degrees "
                     "clockwise of straight up, so the half of the drawing right of a slanted "
                     "line through the middle is transparent: 76 pixels, the top of column 8 "
                     "kept and the bottom of column 7 gone.",
                     case(completion=50, start_angle=30), [0]),
    "FX-RWIPE-010": ("Feather 90, completion 50: the sweeping edge, straight down, fades over 90 "
                     "degrees, so pixels less than 135 degrees round from straight up are "
                     "transparent, those more than 225 round are kept exactly, and between, down "
                     "and either side of the line down column 8, each keeps a part of its colour "
                     "and covering; the start line, straight up, stays a hard edge.",
                     case(completion=50, feather=90), [0]),
    "FX-RWIPE-011": ("Feather 360, completion 50: the fade is a whole turn wide, so each pixel "
                     "keeps exactly its angle round from straight up over 360 of itself, a fan "
                     "clear just right of straight up and nearly whole just left of it.",
                     case(completion=50, feather=360), [0]),
    "FX-RWIPE-012": ("Counterclockwise, start angle 180, feather 60, completion 50: swept from "
                     "straight down back round through the right, so the right half is "
                     "transparent except near the top, where a soft edge 60 degrees wide "
                     "straddles straight up; the start line, straight down, stays hard.",
                     case(completion=50, start_angle=180, wipe="counterclockwise", feather=60),
                     [0]),
    "FX-RWIPE-013": ("Centre 0, 0, the top left corner, completion 40: every pixel lies between "
                     "90 and 180 degrees round from that corner, so the sweep, 144 degrees, "
                     "takes every pixel less than 54 degrees down from straight right of the "
                     "corner, the top right of the drawing, and keeps the pixels nearer "
                     "straight down from it, the bottom left, exactly.",
                     case(completion=40, center=(0, 0)), [0]),
    "FX-RWIPE-014": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear: the clock "
                     "hand sweeps round once, frame 0 the drawing, frames 1 to 3 FX-RWIPE-002, "
                     "003 and 004, and frame 4 FX-RWIPE-005.",
                     case(completion=keyed((0, 0), (4, 100))), [0, 1, 2, 3, 4]),
    "FX-RWIPE-015": ("Start angle keyed from 0 at frame 0 to 360 at frame 4 with completion 25, "
                     "linear: the swept quarter turns round the middle, frame 0 FX-RWIPE-002, "
                     "frame 1 FX-RWIPE-008, frame 2 the bottom left quarter, and frame 4 back "
                     "to FX-RWIPE-002.",
                     case(completion=25, start_angle=keyed((0, 0), (4, 360))), [0, 1, 2, 4]),
    "FX-RWIPE-016": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4 with completion "
                     "40, linear: frame 0 is completion 40 about the middle, frame 2 about "
                     "25, 25, and frame 4 FX-RWIPE-013.",
                     case(completion=40, center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-RWIPE-017": ("Feather keyed from 0 at frame 0 to 360 at frame 4 with completion 50, "
                     "linear: frame 0 is FX-RWIPE-003, frame 2 feather 180, and frame 4 "
                     "FX-RWIPE-011.",
                     case(completion=50, feather=keyed((0, 0), (4, 360))), [0, 2, 4]),
    "FX-RWIPE-018": ("Completion held at 25 from frame 0, then 75 from frame 3: frames 0 and 2 "
                     "are FX-RWIPE-002, frames 3 and 4 FX-RWIPE-004.",
                     case(completion=keyed((0, 25, "hold"), (3, 75))), [0, 2, 3, 4]),
    "FX-RWIPE-019": ("Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                     "FX-RWIPE-005; frame 0 is the drawing.",
                     case(completion=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-RWIPE-020": ("FX-RWIPE-002 moved three pixels right: the same, moved; the wipe moves "
                     "with the drawing.",
                     case(completion=25, shift=3), [0, 3]),
    "FX-RWIPE-021": ("A directional blur, direction 90 and length 4, then completion 70 about "
                     "centre 25, 25, all moved three pixels right: the blur grew the layer two "
                     "pixels on every side, the two grown columns left of the drawing show in "
                     "columns 1 and 2 and are wiped by the same rule, and the centre is still "
                     "the drawing's own point (4, 2.5), not a point of the grown layer.",
                     case(completion=70, center=(25, 25), shift=3, before=(90, 4)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RWIPE-022": ("Completion 101, above 100.", case(completion=101)),
    "FX-RWIPE-023": ("Start angle -3601, below -3600.", case(start_angle=-3601)),
    "FX-RWIPE-024": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-RWIPE-025": ("Feather 361, above 360.", case(feather=361)),
    "FX-RWIPE-026": ("Feather -1, below 0.", case(feather=-1)),
    "FX-RWIPE-027": ("Wipe \"spiral\", which is not a choice.", case(wipe="spiral")),
    "FX-RWIPE-028": ("Wipe \"Clockwise\", in capitals, which is kept as written and is not the "
                     "word.", case(wipe="Clockwise")),
    "FX-RWIPE-029": ("Completion keyed to 150 at frame 4.",
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
    effects.append({
        "instance_id": f"fx-0-{len(effects)}", "type_id": "core.radial_wipe", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}})
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

    (OUT / "expected_radial_wipe.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    clear = [0.0] * 4
    k_of = lambda x, y, **kw: strength(x + 0.5, y + 0.5, **dict(  # noqa: E731
        dict(completion=50, start_angle=0, center=(50, 50), wipe="clockwise", feather=0), **kw))

    def gone(f):
        return {(i % W, i // W) for i in range(W * H) if f[i] == clear and drawn[i] != clear}

    def quarter(xs, ys):
        return {(x, y) for x in xs for y in ys if drawn[at(x, y)] != clear}

    # The rule's own pieces: the screen angle, the words, and the edge.
    assert screen_angle(0, -1) == 0 and screen_angle(1, 0) == 90
    assert screen_angle(0, 1) == 180 and screen_angle(-1, 0) == 270 and screen_angle(0, 0) == 0
    assert abs(screen_angle(1, -1) - 45) < 1e-12 and into_turn(-1e-300) == 0
    assert swept(9, 4, 0, (50, 50), "counterclockwise")[1] == 360 - swept(9, 4, 0, (50, 50),
                                                                           "clockwise")[1]
    assert swept(8, 1, 0, (50, 50), "counterclockwise") == (0.0, 0.0)
    assert abs(swept(7, 4, 0, (50, 50), "both")[1] - swept(9, 4, 0, (50, 50), "both")[1]) < 1e-12
    assert edge(0, 90) == -45 and edge(100, 90) == 405 and edge(50, 360) == 180
    # A negative start angle is the same as that angle plus 360.
    assert all(abs(swept(x + 0.5, y + 0.5, -90, (50, 50), "clockwise")[0]
                   - swept(x + 0.5, y + 0.5, 270, (50, 50), "clockwise")[0]) < 1e-9
               for x in range(W) for y in range(H))
    assert all(k_of(x, y, completion=0, feather=360) == 1 for x in range(W) for y in range(H))

    # Every valid case keeps each pixel a part k of itself, one k for all four channels; no
    # decided pixel, grown ones too, sits within 1e-5 of the hard edge or of the start line.
    for fx, frames in c.items():
        s = expected["cases"][fx]
        cc = CASES[fx][1] if fx in CASES else INVALID[fx][1]
        base = plain(cc)
        if "warning" in s:
            assert all(px == base for px in frames.values())
            continue
        layer = layer_of(cc)
        for f, px in frames.items():
            for p, q in zip(px, base):
                if q[3] > 0:
                    k = p[3] / q[3]
                    assert -1e-15 <= k <= 1 + 1e-15 and near([p], [[v * k for v in q]]), (fx, p)
                else:
                    assert p == q, fx
            st = settings(cc, int(f))
            if st["completion"] in (0, 100):
                continue
            t = edge(st["completion"], st["feather"])
            for i in range(len(layer["px"])):
                x = layer["left"] + i % layer["w"] + 0.5
                y = layer["top"] + i // layer["w"] + 0.5
                a, a_star = swept(x, y, st["start_angle"], st["center"], cc["wipe"])
                if cc["wipe"] != "both":
                    assert CLIFF <= a <= 360 - CLIFF, (fx, f, x, y, a)
                if st["feather"] == 0:
                    assert abs(a_star - t) >= CLIFF, (fx, f, x, y, a_star, t)

    one = c["FX-RWIPE-001"]["0"]
    assert one == drawn
    two, three, four = (c[f"FX-RWIPE-00{n}"]["0"] for n in (2, 3, 4))
    assert gone(two) == quarter(range(8, 16), range(5))
    assert gone(three) == quarter(range(8, 16), range(H))
    assert gone(four) == quarter(range(W), range(H)) - quarter(range(8), range(5))
    for f in (two, three, four):  # the rest is kept exactly: a hard wipe, no part pixels
        assert all(p == q or p == clear for p, q in zip(f, drawn))
    assert four[at(0, 2)] == drawn[at(0, 2)] and drawn[at(0, 2)][3] == 128 / 255
    assert all(p == clear for p in c["FX-RWIPE-005"]["0"])
    six = c["FX-RWIPE-006"]["0"]
    assert gone(six) == quarter(range(8), range(5))
    assert gone(c["FX-RWIPE-007"]["0"]) == quarter(range(W), range(5))
    eight = c["FX-RWIPE-008"]["0"]
    assert gone(eight) == quarter(range(8, 16), range(5, H))
    nine = gone(c["FX-RWIPE-009"]["0"])
    assert len(nine) == 76 and (8, 0) not in nine and (7, 9) in nine and (15, 9) in nine
    assert nine == {(x, y) for x in range(W) for y in range(H) if drawn[at(x, y)] != clear
                    and swept(x + 0.5, y + 0.5, 30, (50, 50), "clockwise")[0] < 180}
    ten = c["FX-RWIPE-010"]["0"]
    parts = [(x, y) for x in range(W) for y in range(H)
             if drawn[at(x, y)] != clear and ten[at(x, y)] not in (clear, drawn[at(x, y)])]
    assert parts and all(135 < swept(x + 0.5, y + 0.5, 0, (50, 50), "clockwise")[0] < 225
                         for x, y in parts)
    assert {(7, 9), (8, 9), (7, 6), (8, 6)} <= set(parts)
    for x in range(W):
        for y in range(H):
            a = swept(x + 0.5, y + 0.5, 0, (50, 50), "clockwise")[0]
            if a <= 135:
                assert ten[at(x, y)] == clear
            if a >= 225:
                assert ten[at(x, y)] == drawn[at(x, y)]
    assert ten[at(7, 0)] == drawn[at(7, 0)] and ten[at(8, 0)] == clear  # the start line: hard
    eleven = c["FX-RWIPE-011"]["0"]
    for x in range(W):
        for y in range(H):
            a = swept(x + 0.5, y + 0.5, 0, (50, 50), "clockwise")[0]
            assert near([eleven[at(x, y)]], [[v * a / 360 for v in drawn[at(x, y)]]]), (x, y)
    assert eleven[at(8, 0)][3] < 0.03 and eleven[at(7, 0)][3] > 0.97
    twelve = c["FX-RWIPE-012"]["0"]
    for x in range(W):
        for y in range(H):
            a = swept(x + 0.5, y + 0.5, 0, (50, 50), "clockwise")[0]
            k = twelve[at(x, y)][3] / drawn[at(x, y)][3] if drawn[at(x, y)][3] else None
            if k is None:
                continue
            if 30 <= a <= 180:
                assert k == 0, (x, y)
            elif 180 < a < 330:
                assert k == 1, (x, y)
            else:
                assert 0 < k < 1, (x, y)
    assert twelve[at(7, 9)] == drawn[at(7, 9)] and twelve[at(8, 9)] == clear  # straight down
    thirteen = c["FX-RWIPE-013"]["0"]
    for x in range(W):
        for y in range(H):
            a = screen_angle(x + 0.5, y + 0.5)
            assert 90 < a < 180
            if drawn[at(x, y)] != clear:
                assert thirteen[at(x, y)] == (clear if a < 144 else drawn[at(x, y)]), (x, y)
    assert thirteen[at(15, 0)] == clear == thirteen[at(15, 9)] and thirteen[at(0, 9)] != clear
    fourteen = c["FX-RWIPE-014"]
    assert fourteen["0"] == drawn and fourteen["1"] == two and fourteen["2"] == three
    assert fourteen["3"] == four and fourteen["4"] == c["FX-RWIPE-005"]["0"]
    fifteen = c["FX-RWIPE-015"]
    assert fifteen["0"] == two and fifteen["1"] == eight and fifteen["4"] == two
    assert gone(fifteen["2"]) == quarter(range(8), range(5, H))
    sixteen = c["FX-RWIPE-016"]
    assert sixteen["0"] == render(case(completion=40), 0)
    assert sixteen["2"] == render(case(completion=40, center=(25, 25)), 0)
    assert sixteen["4"] == thirteen and sixteen["2"] != sixteen["0"]
    seventeen = c["FX-RWIPE-017"]
    assert seventeen["0"] == three and seventeen["4"] == eleven
    assert seventeen["2"] == render(case(completion=50, feather=180), 0)
    eighteen = c["FX-RWIPE-018"]
    assert eighteen["0"] == eighteen["2"] == two and eighteen["3"] == eighteen["4"] == four
    nineteen = c["FX-RWIPE-019"]
    assert ease(OVERSHOOT, 0.5) > 1 and nineteen["0"] == drawn
    assert nineteen["2"] == c["FX-RWIPE-005"]["0"]
    moved = c["FX-RWIPE-020"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == clear for x in range(3) for y in range(H))
    # After a directional blur: the grown columns left of the drawing are wiped by the same
    # rule, about the drawing's own (4, 2.5), not about a point of the grown layer.
    grown = case(completion=70, center=(25, 25), shift=3, before=(90, 4))
    blurred, got = plain(grown), c["FX-RWIPE-021"]["0"]
    for x in range(W):
        for y in range(H):
            k = strength(x - 3 + 0.5, y + 0.5, 70, 0, (25, 25), "clockwise", 0)
            assert got[at(x, y)] == (blurred[at(x, y)] if k else clear), (x, y)
    assert blurred[at(1, 0)][3] > 0 and got[at(1, 0)] == blurred[at(1, 0)]
    assert blurred[at(1, 9)][3] > 0 and got[at(1, 9)] == clear
    layer = layer_of(grown)
    lw, lh = layer["w"], layer["h"]
    inside = lambda px: [px[(y + 2) * lw + x + 2] for y in range(H) for x in range(W)]  # noqa: E731
    right = inside(radial_wipe(layer, 70, 0, (25, 25), "clockwise", 0))
    wrong_a = inside(radial_wipe(layer, 70, 0, (25, 25), "clockwise", 0, drawing=(lw, lh)))
    wrong_b = inside(radial_wipe(dict(layer, left=0, top=0), 70, 0, (25, 25), "clockwise", 0))
    assert right != wrong_a and right != wrong_b
    assert all(got[at(x, y)] == right[at(x - 3, y)] for x in range(3, W) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
