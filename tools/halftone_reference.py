"""Halftone, worked a second way.

D-143 adds `core.halftone`: manga screentone, the picture's lightness drawn as dots of ink on
paper. A screen of round dots is laid over the drawing, one dot to each square cell `size` pixels
wide, the rows of cells turned by `angle` degrees (45 as it starts, as a printed screen is). Each
pixel's own lightness sets the dot it sits in: white leaves no dot, a mid grey a dot that covers
about half its cell, and black a dot so wide the dots run together and cover everything. Each
pixel is then ink or paper, with a hard edge and no smoothing, and is laid over the drawing's own
colour by `amount`, so at 100 every pixel that shows is exactly the ink or the paper. Pixels that
do not show stay as they are and every pixel keeps its own covering, so the dots never spill past
the drawing. The screen is fixed to the drawing, so it moves with the layer. It is this program's
own method, modelled on After Effects' Color Halftone and the screentone of manga; nothing is
ported. Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

The rule. At a pixel with covering a > 0, with (X, Y) its centre in the drawing's own coordinates
(an earlier effect that grew the layer gives its grown pixels negative ones) and t the angle in
radians: u = (X cos t + Y sin t) / size, v = (-X sin t + Y cos t) / size and rho =
sqrt((u - floor(u) - 0.5)^2 + (v - floor(v) - 0.5)^2), the distance from the cell's middle in
cells. With e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) the encoded straight colour, the
darkness k = 1 - clamp(0.2126 e_r + 0.7152 e_g + 0.0722 e_b, 0, 1), and the dot's radius r =
sqrt(k / pi) while k <= pi / 4 (the dot covers k of its cell), else r = 0.5 + 0.21 (k - pi / 4) /
(1 - pi / 4), 0.71 at black, past the cell's corner at sqrt(0.5) = 0.7071. The pixel is ink when
rho < r, else paper: a cliff. With b the straight linear colour, G the ink's linear value if ink,
else the paper's, and op = amount / 100, the output is ((b + op (G - b)) a, a): the blend mix,
normal. At amount 0 the pixel stays exactly as it is. The layer does not grow; a draft scales
`size`.

A cliff: the build decides from 32-bit pixels, this file from 8-bit values in double precision,
so `check` asserts that no pixel any case decides has rho within 1e-5 of r.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of skin, shadow and line over a ramp of greys, drawn below. The drawing goes
into `Fixtures/halftone/media`, the projects into `Fixtures/halftone`, and the expected frames
into `Fixtures/halftone/expected_halftone.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/halftone_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import directional_blur_reference as D  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "halftone"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-5      # the spec's margin: no decided pixel this close to its cliff
RANGES = {"size": (2, 200), "angle": (-3600, 3600), "amount": (0, 100)}
NUMBERS = ("size", "angle", "amount")
NAMES = ("size", "angle", "ink", "paper", "amount")


# --- the rule -------------------------------------------------------------------------------

def darkness(p):
    """k: 1 less the encoded luma of a premultiplied pixel's encoded straight colour."""
    a = p[3]
    e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in p[:3]]
    return 1 - min(1.0, max(0.0, 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]))


def radius(k):
    """The dot's radius in cells: its area k of the cell until the dots touch at pi / 4."""
    if k <= math.pi / 4:
        return math.sqrt(k / math.pi)
    return 0.5 + 0.21 * (k - math.pi / 4) / (1 - math.pi / 4)


def distance(X, Y, size, angle):
    """rho: how far the point (X, Y) of the drawing is from its cell's middle, in cells."""
    t = math.radians(angle)
    u = (X * math.cos(t) + Y * math.sin(t)) / size
    v = (-X * math.sin(t) + Y * math.cos(t)) / size
    return math.sqrt((u - math.floor(u) - 0.5) ** 2 + (v - math.floor(v) - 0.5) ** 2)


def decisions(layer, size, angle):
    """(rho, r) at each pixel that shows, None at each that does not."""
    out = []
    for i, p in enumerate(layer["px"]):
        if p[3] == 0:
            out.append(None)
            continue
        X, Y = layer["left"] + i % layer["w"] + 0.5, layer["top"] + i // layer["w"] + 0.5
        out.append((distance(X, Y, size, angle), radius(darkness(p))))
    return out


def halftone(layer, size, angle, ink, paper, amount):
    """The layer's premultiplied pixels, screened; its rectangle kept. The layer's `left` and
    `top` are the growth of earlier effects, so its pixel (0, 0) is the drawing's (left, top)."""
    I = [srgb_to_linear(v / 255) for v in R.hex_color(ink.lower())]
    P = [srgb_to_linear(v / 255) for v in R.hex_color(paper.lower())]
    op = amount / 100
    out = []
    for p, d in zip(layer["px"], decisions(layer, size, angle)):
        if d is None or op == 0:
            out.append(list(p))
            continue
        G = I if d[0] < d[1] else P
        a = p[3]
        b = [v / a for v in p[:3]]
        out.append([(b[c] + op * (G[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

SKIN = R.SKIN                    # #f6d6be
LINE = R.LINE                    # #1e1a24
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
SOFT = (246, 214, 190, 128)      # the skin at half covering, the drawing's soft left edge
NONE = S.NONE
# Column x's grey in rows 7 to 9: white at column 1, black at column 15, in fourteen steps.
RAMP = {x: round(255 * (15 - x) / 14) for x in range(1, W)}


def tones(x, y):
    """Skin in rows 0 to 3, shadow in rows 4 and 5, the line along row 6, and in rows 7 to 9 a
    ramp of greys from white in column 1 to black in column 15; column 0 the skin at half
    covering, a soft edge; and the top right corner, columns 14 and 15 of rows 0 and 1, empty."""
    if x >= 14 and y <= 1:
        return NONE
    if x == 0:
        return SOFT
    if y <= 3:
        return SKIN
    if y <= 5:
        return SHADE
    if y == 6:
        return LINE
    return (RAMP[x],) * 3 + (255,)


DRAWINGS = {"tones": [[tones(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

SEPIA = ("#1e1a24", "#f6d6be")   # the line's colour for ink, the skin's for paper


def case(size=8, angle=45, ink="#000000", paper="#ffffff", amount=100, shift=0, before=None):
    """`before` is a directional blur (direction, length) ahead of the halftone."""
    return {"drawing": "tones", "size": size, "angle": angle, "ink": ink, "paper": paper,
            "amount": amount, "shift": shift, "before": before}


def settings(c, frame_no):
    held = {}
    for k in NUMBERS:
        lo, hi = RANGES[k]
        held[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return held


def layer_of(c):
    """The drawing as the halftone receives it: grown by a directional blur ahead of it."""
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
    px = halftone(layer, s["size"], s["angle"], c["ink"], c["paper"], s["amount"])
    out = []
    for y in range(H):
        for x in range(W):
            lx, ly = x - c["shift"] - layer["left"], y - layer["top"]
            inside = 0 <= lx < layer["w"] and 0 <= ly < layer["h"]
            out.append(px[ly * layer["w"] + lx] if inside else [0.0] * 4)
    return out


def plain(c):
    """The drawing untouched, moved and grown as the case moves and grows it."""
    return render(case(amount=0, shift=c["shift"], before=c["before"]), 0)


CASES = {
    "FX-HALFTONE-001": ("The settings as they start: size 8, angle 45, ink #000000, paper "
                        "#ffffff, amount 100. Every pixel that shows turns black or white at its "
                        "own covering, the soft edge at half covering; the dots grow with the "
                        "darkness: the skin keeps a few small black dots, 4 of its 56 pixels "
                        "right of the soft edge, the shadow larger ones, 13 of its 30, the line "
                        "is black but for one pixel at a cell's corner, (6, 6), and along the ramp white is all paper and black all "
                        "ink; the empty corner stays empty.",
                        case(), [0]),
    "FX-HALFTONE-002": ("Amount 50: each pixel half way from its own colour to the black or "
                        "white FX-HALFTONE-001 gives it, in linear values.",
                        case(amount=50), [0]),
    "FX-HALFTONE-003": ("Amount 0: the drawing, untouched.",
                        case(amount=0), [0]),
    "FX-HALFTONE-004": ("Size 4: cells half as wide, so four times as many dots, each half as "
                        "wide.",
                        case(size=4), [0]),
    "FX-HALFTONE-005": ("Size 2: the finest screen, a dot every two pixels.",
                        case(size=2), [0]),
    "FX-HALFTONE-006": ("Angle 0: the cells square to the frame, so the screen repeats every 8 "
                        "pixels across: in each band, pixel (x, y) and pixel (x + 8, y) are "
                        "decided alike.",
                        case(angle=0), [0]),
    "FX-HALFTONE-007": ("Angle 90: the square screen turned a quarter onto itself, the same as "
                        "angle 0, FX-HALFTONE-006.",
                        case(angle=90), [0]),
    "FX-HALFTONE-008": ("Ink #1e1a24, the line's colour, on paper #f6d6be, the skin's: the same "
                        "dots as FX-HALFTONE-001 in sepia, each ink pixel the line's colour and "
                        "each paper pixel the skin's, so the skin's paper pixels are unchanged.",
                        case(ink=SEPIA[0], paper=SEPIA[1]), [0]),
    "FX-HALFTONE-009": ("FX-HALFTONE-008 with the colours written in capitals, #1E1A24 and "
                        "#F6D6BE: the same.",
                        case(ink=SEPIA[0].upper(), paper=SEPIA[1].upper()), [0]),
    "FX-HALFTONE-010": ("Ink #ffffff on paper #000000, the two swapped: every pixel that shows "
                        "is the opposite of FX-HALFTONE-001's, white dots on black.",
                        case(ink="#ffffff", paper="#000000"), [0]),
    "FX-HALFTONE-011": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the "
                        "drawing, frame 2 FX-HALFTONE-002 and frame 4 FX-HALFTONE-001.",
                        case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-HALFTONE-012": ("Size keyed from 4 at frame 0 to 8 at frame 4, linear: the dots swell, "
                        "frame 0 FX-HALFTONE-004, frame 2 size 6 and frame 4 FX-HALFTONE-001.",
                        case(size=keyed((0, 4), (4, 8))), [0, 2, 4]),
    "FX-HALFTONE-013": ("Angle keyed from 0 at frame 0 to 90 at frame 4, linear: the screen "
                        "turns, frame 0 FX-HALFTONE-006, frame 2 angle 45, FX-HALFTONE-001, and "
                        "frame 4 FX-HALFTONE-007.",
                        case(angle=keyed((0, 0), (4, 90))), [0, 2, 4]),
    "FX-HALFTONE-014": ("Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                        "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                        "FX-HALFTONE-001; frame 0 is the drawing.",
                        case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-HALFTONE-015": ("FX-HALFTONE-001 moved three pixels right: the same, moved; the screen "
                        "moves with the drawing.",
                        case(shift=3), [0, 3]),
    "FX-HALFTONE-016": ("A directional blur, direction 90 and length 4, then the halftone as it "
                        "starts: the blur grew the layer two pixels on every side, and the "
                        "screen is still the drawing's own, fixed to its own top left corner, "
                        "not the grown layer's, and the blurred pixels, the empty corner now "
                        "partly covered among them, are screened at their own covering.",
                        case(before=(90, 4)), [0]),
    "FX-HALFTONE-017": ("Angle 405, a whole turn past 45: the same as FX-HALFTONE-001.",
                        case(angle=405), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-HALFTONE-018": ("Size 1, below 2.", case(size=1)),
    "FX-HALFTONE-019": ("Size 201, above 200.", case(size=201)),
    "FX-HALFTONE-020": ("Angle 3601, past ten turns.", case(angle=3601)),
    "FX-HALFTONE-021": ("Amount -1, below 0.", case(amount=-1)),
    "FX-HALFTONE-022": ("Amount 101, above 100.", case(amount=101)),
    "FX-HALFTONE-023": ("Ink \"black\", a name, not #rrggbb.", case(ink="black")),
    "FX-HALFTONE-024": ("Paper \"#fffff\", five digits, not six.", case(paper="#fffff")),
    "FX-HALFTONE-025": ("Size keyed to 250 at frame 4.", case(size=keyed((0, 8), (4, 250)))),
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
        "instance_id": f"fx-0-{len(effects)}", "type_id": "core.halftone", "enabled": True,
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

    (OUT / "expected_halftone.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def inked(c, frame_no=0):
    """Which pixels of the case's frame are ink: True, False, or None where nothing shows."""
    layer, s = layer_of(c), settings(c, frame_no)
    d = decisions(layer, s["size"], s["angle"])
    out = []
    for y in range(H):
        for x in range(W):
            lx, ly = x - c["shift"] - layer["left"], y - layer["top"]
            inside = 0 <= lx < layer["w"] and 0 <= ly < layer["h"]
            e = d[ly * layer["w"] + lx] if inside else None
            out.append(None if e is None else e[0] < e[1])
    return out


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    BLACK, WHITE = [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]

    # The rule's own pieces: the dot covers k of its cell until pi / 4, where the two radii
    # meet at 0.5; at black it passes the cell's corner, so black is ink everywhere, and white
    # has no dot, so it is paper everywhere.
    assert radius(0) == 0 and abs(radius(0.3) ** 2 * math.pi - 0.3) < 1e-15
    assert abs(radius(math.pi / 4) - 0.5) < 1e-15 and abs(radius(math.pi / 4 + 1e-12) - 0.5) < 1e-11
    assert radius(1) == 0.71 > math.sqrt(0.5)
    assert distance(4, 4, 8, 0) == 0 and abs(distance(0, 0, 8, 0) - math.sqrt(0.5)) < 1e-15
    assert abs(distance(3.3, 1.7, 8, 45) - distance(3.3, 1.7, 8, 405)) < 1e-12
    assert darkness(R.working((255, 255, 255, 255))) < 1e-12
    assert darkness(R.working((0, 0, 0, 255))) == 1

    # The cliff: no pixel any case decides lies within 1e-5 of its dot's edge; and a white pixel,
    # whose dot is nothing in double precision but may be a hair in single, is well clear of a
    # dot's middle.
    margin = 1.0
    for fx, (says, cs, frames) in CASES.items():
        for f in frames:
            s = settings(cs, f)
            for d in decisions(layer_of(cs), s["size"], s["angle"]):
                if d is not None:
                    margin = min(margin, abs(d[0] - d[1]))
                    assert d[1] > 1e-6 or d[0] > 1e-3, (fx, f, d)
    assert margin > CLIFF, margin
    print(f"closest to the cliff: {margin:.3g}")

    # Every case keeps every covering and every empty pixel, and no channel passes its covering.
    for fx, frames in c.items():
        s = expected["cases"][fx]
        base = plain(CASES[fx][1] if fx in CASES else INVALID[fx][1])
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= u <= px[i][3] + 1e-12 for u in px[i][:3]), (fx, i)
        if "warning" in s:
            assert all(px == base for px in frames.values())

    # 001: black or white at the pixel's own covering; the counts the case gives.
    one, ink1 = c["FX-HALFTONE-001"]["0"], inked(case())
    for i in shows:
        a = drawn[i][3]
        assert close(one[i], [u * a for u in (BLACK if ink1[i] else WHITE)] + [a]), i
    band = lambda xs, ys, m=ink1: sum(1 for x in xs for y in ys if m[at(x, y)])  # noqa: E731
    assert band(range(1, 16), range(4)) == 4 and band(range(1, 16), (4, 5)) == 13
    assert band(range(1, 16), (6,)) == 14 and not ink1[at(6, 6)]
    assert band((1,), range(7, 10)) == 0 and band((15,), range(7, 10)) == 3
    assert one[at(0, 4)][3] == 128 / 255
    # Down the ramp the dark end carries more ink than the light end.
    ramp = [band((x,), range(7, 10)) for x in range(1, 16)]
    assert ramp[0] == 0 and ramp[-1] == 3 and sum(ramp[:4]) < sum(ramp[-4:])
    # 002: half way, in linear straight values.
    two = c["FX-HALFTONE-002"]["0"]
    for i in shows:
        a = drawn[i][3]
        assert close(two[i], [(drawn[i][k] + one[i][k]) / 2 for k in range(3)] + [a]), i
    assert c["FX-HALFTONE-003"]["0"] == drawn
    # 004, 005: finer screens, the cells half and a quarter as wide, the same dots scaled down.
    assert abs(distance(1.5, 2.5, 4, 45) - distance(3, 5, 8, 45)) < 1e-12
    assert abs(distance(1.5, 2.5, 2, 45) - distance(6, 10, 8, 45)) < 1e-12
    four, five = inked(case(size=4)), inked(case(size=2))
    assert four != ink1 and five != four
    # 006: angle 0 repeats every 8 pixels across; 007 is the same frame.
    six, ink6 = c["FX-HALFTONE-006"]["0"], inked(case(angle=0))
    for y in range(H):
        for x in range(1, 8):
            if drawn[at(x, y)] == drawn[at(x + 8, y)] and drawn[at(x, y)][3] > 0:
                assert ink6[at(x, y)] == ink6[at(x + 8, y)], (x, y)
    assert c["FX-HALFTONE-007"]["0"] == six and six != one
    # 008, 009: the same dots in sepia; the skin's paper pixels unchanged.
    eight = c["FX-HALFTONE-008"]["0"]
    L, K = ([srgb_to_linear(v / 255) for v in R.hex_color(h)] for h in SEPIA)
    for i in shows:
        a = drawn[i][3]
        assert close(eight[i], [u * a for u in (L if ink1[i] else K)] + [a]), i
    assert all(eight[at(x, y)] == drawn[at(x, y)] or ink1[at(x, y)]
               for x in range(1, 16) for y in range(4) if drawn[at(x, y)][3] > 0)
    assert c["FX-HALFTONE-009"]["0"] == eight
    # 010: swapped, each pixel the opposite of 001's.
    ten = c["FX-HALFTONE-010"]["0"]
    assert all(close(ten[i], [drawn[i][3] - u for u in one[i][:3]] + [drawn[i][3]])
               for i in shows)
    # Keyed and held.
    eleven = c["FX-HALFTONE-011"]
    assert eleven["0"] == drawn and eleven["2"] == two and eleven["4"] == one
    twelve = c["FX-HALFTONE-012"]
    assert twelve["0"] == c["FX-HALFTONE-004"]["0"] and twelve["4"] == one
    assert twelve["2"] == render(case(size=6), 0) and twelve["2"] not in (one, twelve["0"])
    thirteen = c["FX-HALFTONE-013"]
    assert thirteen["0"] == six and thirteen["2"] == one and thirteen["4"] == six
    fourteen = c["FX-HALFTONE-014"]
    assert ease(OVERSHOOT, 0.5) > 1 and fourteen["0"] == drawn and fourteen["2"] == one
    # Moved: the screen moves with the drawing.
    moved = c["FX-HALFTONE-015"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # After a directional blur: the screen is fixed to the drawing's corner, not the grown
    # layer's; the grown pixels at their own covering.
    grown = case(before=(90, 4))
    blurred, got, ink16 = plain(grown), c["FX-HALFTONE-016"]["0"], inked(grown)
    layer = layer_of(grown)
    corner = decisions(dict(layer, left=0, top=0), 8, 45)
    moved_screen = [corner[(y + 2) * (W + 4) + x + 2] for y in range(H) for x in range(W)]
    assert any(d is not None and (d[0] < d[1]) != ink16[i] for i, d in enumerate(moved_screen))
    for i in range(W * H):
        a = blurred[i][3]
        if a > 0:
            assert close(got[i], [u * a for u in (BLACK if ink16[i] else WHITE)] + [a]), i
    assert c["FX-HALFTONE-017"]["0"] == one
    print("checked")


if __name__ == "__main__":
    main()
