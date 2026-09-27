"""Vignette, worked a second way.

D-126 adds `core.vignette`: the edges and corners of a picture darkened, or tinted, toward a
colour, as an old lens darkens its corners and as a finishing pass draws the eye to the middle.
An ellipse the drawing's shape (or, with `roundness`, a circle) is drawn about `center`, per cent
of the drawing's own width and height as Radial Blur's centre is; nothing changes inside its
inner edge, the colour is laid on fully at its outer edge and beyond, and between the two it
fades in along a smooth curve. `size` scales the ellipse, `softness` sets how wide the fade is,
and `amount` how strongly the colour is laid on. Pixels that do not show stay as they are and
every pixel keeps its own covering, so the vignette never spills past the drawing. It is this
program's own method, modelled on After Effects' Lumetri Color vignette and the vignette of a
camera raw grade; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. With w, h the drawing's own size, c the centre (center_x / 100 * w, center_y / 100 *
h) in the drawing's own pixels (an earlier effect that grew the layer does not move it), m =
roundness / 100, rx = (1 - m) w / 2 + m sqrt(w h) / 2, ry = (1 - m) h / 2 + m sqrt(w h) / 2 and
P a pixel's centre: d = sqrt(((P.x - c.x) / rx)^2 + ((P.y - c.y) / ry)^2) / sqrt(2), which is 1
at the drawing's corners when roundness is 0 and the centre is the middle. outer = size / 100
and inner = outer * (1 - softness / 100). When inner == outer, v = 1 where d >= outer and 0
elsewhere; otherwise t = clamp((d - inner) / (outer - inner), 0, 1) and v = t^2 (3 - 2t). At a
pixel with covering a > 0, with b its straight linear colour and G the colour's linear value,
op = v * amount / 100 and the output is ((b + op (G - b)) * a, a): the blend mix, normal. A
pixel with a = 0 stays as it is; the layer does not grow, and nothing is scaled for a draft.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a full-frame backdrop, drawn below. The drawing goes into
`Fixtures/vignette/media`, the projects into `Fixtures/vignette`, and the expected frames into
`Fixtures/vignette/expected_vignette.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/vignette_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "vignette"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 100), "size": (1, 200), "roundness": (0, 100), "softness": (0, 100),
          "center": (-1000, 1000)}
NUMBERS = ("amount", "size", "roundness", "softness", "center")
NAMES = ("amount", "color", "size", "roundness", "softness", "center")


# --- the rule -------------------------------------------------------------------------------

def strength(px, py, center, size, roundness, softness, w=W, h=H):
    """v at the point (px, py) of the drawing's own pixels: 0 inside the inner edge, 1 past
    the outer."""
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    m = roundness / 100
    rx = (1 - m) * w / 2 + m * math.sqrt(w * h) / 2
    ry = (1 - m) * h / 2 + m * math.sqrt(w * h) / 2
    d = math.sqrt(((px - cx) / rx) ** 2 + ((py - cy) / ry) ** 2) / math.sqrt(2)
    outer = size / 100
    inner = outer * (1 - softness / 100)
    if inner == outer:
        return 1.0 if d >= outer else 0.0
    t = min(1.0, max(0.0, (d - inner) / (outer - inner)))
    return t * t * (3 - 2 * t)


def vignette(layer, amount, color, size, roundness, softness, center, drawing=(W, H)):
    """The layer's premultiplied pixels, vignetted; its rectangle kept. `drawing` is the
    drawing's own w by h; the layer's `left` and `top` are the growth of earlier effects."""
    G = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    out = []
    for i, p in enumerate(layer["px"]):
        a = p[3]
        x, y = layer["left"] + i % layer["w"] + 0.5, layer["top"] + i // layer["w"] + 0.5
        op = 0.0 if a == 0 else strength(x, y, center, size, roundness, softness,
                                         *drawing) * amount / 100
        if op == 0:
            out.append(list(p))
            continue
        b = [v / a for v in p[:3]]
        out.append([(b[c] + op * (G[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

SKIN = R.SKIN                    # #f6d6be
LINE = R.LINE                    # #1e1a24
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
SOFT = (246, 214, 190, 128)      # the skin at half covering, the backdrop's soft left edge
NONE = S.NONE
VIOLET = "#6450a0"


def backdrop(x, y):
    """The whole frame filled: skin above a line along row 7, shadow in rows 8 and 9; column 0
    the skin at half covering, a soft edge; and the top right corner, columns 14 and 15 of rows
    0 and 1, empty."""
    if x >= 14 and y <= 1:
        return NONE
    if x == 0:
        return SOFT
    if y == 7:
        return LINE
    if y >= 8:
        return SHADE
    return SKIN


DRAWINGS = {"backdrop": [[backdrop(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(amount=50, color="#000000", size=100, roundness=0, softness=50, center=(50, 50),
         shift=0, before=None):
    """`before` is a directional blur (direction, length) ahead of the vignette."""
    return {"drawing": "backdrop", "amount": amount, "color": color, "size": size,
            "roundness": roundness, "softness": softness, "center": center, "shift": shift,
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
    """The drawing as the vignette receives it: grown by a directional blur ahead of it."""
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
    px = vignette(layer, s["amount"], c["color"], s["size"], s["roundness"], s["softness"],
                  s["center"])
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
    "FX-VIGNETTE-001": ("The settings as they start: amount 50, #000000, size 100, roundness 0, "
                        "softness 50, about the middle. The corners are darkened most, the "
                        "bottom right pixel 0.465 of the way to black; the fade reaches in only "
                        "to the ellipse half the size, so the middle, columns 3 to 12 of rows 3 "
                        "to 6 among it, is untouched; nothing is lightened, and the empty corner "
                        "stays empty.",
                        case(), [0]),
    "FX-VIGNETTE-002": ("Amount 100: the same fade twice as strong, each pixel twice as far "
                        "toward black as in FX-VIGNETTE-001.",
                        case(amount=100), [0]),
    "FX-VIGNETTE-003": ("Amount 0: the drawing, untouched.",
                        case(amount=0), [0]),
    "FX-VIGNETTE-004": ("Colour #6450a0, a violet: FX-VIGNETTE-001's fade toward violet, so the "
                        "line along row 7 is lightened at its ends and the skin darkened.",
                        case(color=VIOLET), [0]),
    "FX-VIGNETTE-005": ("Size 50: the ellipse half the size, so the fade runs from a quarter of "
                        "the way out to half way, and every pixel past half way, among them the "
                        "top and bottom rows and the two outer columns on each side, takes the "
                        "full amount, half way to black; the four middle pixels are untouched.",
                        case(size=50), [0]),
    "FX-VIGNETTE-006": ("Size 200: the fade starts at the corners' own distance and beyond, so "
                        "nothing in the drawing is reached: the drawing, untouched.",
                        case(size=200), [0]),
    "FX-VIGNETTE-007": ("Softness 0, size 60: a hard-edged ellipse, each pixel either untouched "
                        "or half way to black, nothing between.",
                        case(softness=0, size=60), [0]),
    "FX-VIGNETTE-008": ("Softness 100: the fade starts at the centre itself, so every pixel that "
                        "shows is darkened, the four middle pixels least, and more toward the "
                        "edges.",
                        case(softness=100), [0]),
    "FX-VIGNETTE-009": ("Roundness 100: a circle rather than an ellipse the drawing's shape, so "
                        "pixels equally far from the middle are darkened alike: (3, 1) and "
                        "(4, 0), each 5.70 pixels from it, where roundness 0 darkens (4, 0) about "
                        "twice as much; the sides are darkened more than in FX-VIGNETTE-001, and "
                        "the middle of the top row less.",
                        case(roundness=100), [0]),
    "FX-VIGNETTE-010": ("Centre 0, 0, the top left corner: the vignette is about that corner, "
                        "so the top left pixels are untouched and the bottom right is half way "
                        "to black.",
                        case(center=(0, 0)), [0]),
    "FX-VIGNETTE-011": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the "
                        "drawing, frame 2 FX-VIGNETTE-001 and frame 4 FX-VIGNETTE-002.",
                        case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-VIGNETTE-012": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame "
                        "0 is FX-VIGNETTE-001, frame 2 is about 25, 25 and frame 4 is "
                        "FX-VIGNETTE-010.",
                        case(center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-VIGNETTE-013": ("Size keyed from 200 at frame 0 to 50 at frame 4, linear: the vignette "
                        "closes in, frame 0 the drawing, frame 2 size 125 and frame 4 "
                        "FX-VIGNETTE-005.",
                        case(size=keyed((0, 200), (4, 50))), [0, 2, 4]),
    "FX-VIGNETTE-014": ("Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                        "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                        "FX-VIGNETTE-002; frame 0 is the drawing.",
                        case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-VIGNETTE-015": ("Size 1, amount 100: every pixel is past the tiny ellipse, so every "
                        "pixel that shows turns black at its own covering, the soft edge black "
                        "at half covering, and the empty corner stays empty.",
                        case(size=1, amount=100), [0]),
    "FX-VIGNETTE-016": ("FX-VIGNETTE-004 with the colour written in capitals, #6450A0: the same.",
                        case(color=VIOLET.upper()), [0]),
    "FX-VIGNETTE-017": ("FX-VIGNETTE-001 moved three pixels right: the same, moved; the "
                        "vignette moves with the drawing.",
                        case(shift=3), [0, 3]),
    "FX-VIGNETTE-018": ("A directional blur, direction 90 and length 4, then the vignette as it "
                        "starts: the blur grew the layer two pixels on every side, and the "
                        "ellipse is still the drawing's own, about its own middle.",
                        case(before=(90, 4)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-VIGNETTE-019": ("Amount 101, above 100.", case(amount=101)),
    "FX-VIGNETTE-020": ("Size 0, below 1.", case(size=0)),
    "FX-VIGNETTE-021": ("Roundness -1, below 0.", case(roundness=-1)),
    "FX-VIGNETTE-022": ("Softness 101, above 100.", case(softness=101)),
    "FX-VIGNETTE-023": ("Centre 50, -1001, past ten heights.", case(center=(50, -1001))),
    "FX-VIGNETTE-024": ("Size keyed to 250 at frame 4.", case(size=keyed((0, 100), (4, 250)))),
    "FX-VIGNETTE-025": ("Colour \"black\", a name, not #rrggbb.", case(color="black")),
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
        "instance_id": f"fx-0-{len(effects)}", "type_id": "core.vignette", "enabled": True,
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

    (OUT / "expected_vignette.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    near = lambda a, b, e=1e-12: all(close(p, q, e) for p, q in zip(a, b))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    # How far toward black a pixel went, on its red channel.
    darkened = lambda f, i: 1 - f[i][0] / drawn[i][0]  # noqa: E731
    v = lambda x, y, **kw: strength(x + 0.5, y + 0.5, **dict(  # noqa: E731
        dict(center=(50, 50), size=100, roundness=0, softness=50), **kw))

    # The rule's own pieces: 1 at the drawing's corners, 0 at the middle; a circle at round 100.
    assert strength(0, 0, (50, 50), 100, 0, 50) == 1 == strength(16, 10, (50, 50), 100, 0, 50)
    assert strength(8, 5, (50, 50), 100, 0, 50) == 0
    assert strength(8, 0, (50, 50), 100, 100, 100) == strength(3, 5, (50, 50), 100, 100, 100)
    assert strength(8, 0, (50, 50), 100, 0, 100) > strength(3, 5, (50, 50), 100, 0, 100)
    assert strength(1, 1, (50, 50), 60, 0, 0) == 1 and strength(4.5, 1.5, (50, 50), 60, 0, 0) == 0

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

    one = c["FX-VIGNETTE-001"]["0"]
    assert abs(darkened(one, at(15, 9)) - 0.465) < 0.0005
    for i in shows:
        x, y = i % W, i // W
        assert (one[i] == drawn[i]) == (v(x, y) == 0), i
        assert all(one[i][k] <= drawn[i][k] for k in range(3))
    assert all(one[at(x, y)] == drawn[at(x, y)] for x in range(3, 13) for y in range(3, 7))
    two = c["FX-VIGNETTE-002"]["0"]
    assert all(abs(darkened(two, i) - 2 * darkened(one, i)) < 1e-12 for i in shows)
    assert c["FX-VIGNETTE-003"]["0"] == drawn
    four = c["FX-VIGNETTE-004"]["0"]
    assert all(four[at(1, 7)][k] > drawn[at(1, 7)][k] for k in range(3))
    assert all(four[at(1, 1)][k] < drawn[at(1, 1)][k] for k in range(3))
    assert four[at(15, 9)][2] > drawn[at(15, 9)][2]  # violet's blue is above the shadow's
    five = c["FX-VIGNETTE-005"]["0"]
    for i in shows:
        x, y = i % W, i // W
        assert v(x, y, size=50) == 1 or not (y in (0, 9) or x in (0, 1, 14, 15))
        if v(x, y, size=50) == 1:
            assert close(five[i], [u / 2 for u in drawn[i][:3]] + [drawn[i][3]]), i
    assert all(five[at(x, y)] == drawn[at(x, y)] for x in (7, 8) for y in (4, 5))
    assert c["FX-VIGNETTE-006"]["0"] == drawn
    seven = c["FX-VIGNETTE-007"]["0"]
    halved = [i for i in shows if seven[i] != drawn[i]]
    assert halved and len(halved) < len(shows)
    assert all(close(seven[i], [u / 2 for u in drawn[i][:3]] + [drawn[i][3]]) for i in halved)
    eight = c["FX-VIGNETTE-008"]["0"]
    assert all(eight[i] != drawn[i] for i in shows)
    middle = [darkened(eight, at(x, y)) for x in (7, 8) for y in (4, 5)]
    assert max(middle) < min(darkened(eight, i) for i in shows
                             if (i % W, i // W) not in ((7, 4), (8, 4), (7, 5), (8, 5)))
    nine = c["FX-VIGNETTE-009"]["0"]
    assert abs(darkened(nine, at(3, 1)) - darkened(nine, at(4, 0))) < 1e-12
    assert darkened(one, at(4, 0)) > 1.9 * darkened(one, at(3, 1))
    assert darkened(nine, at(0, 4)) > darkened(one, at(0, 4))
    assert darkened(nine, at(7, 0)) < darkened(one, at(7, 0))
    ten = c["FX-VIGNETTE-010"]["0"]
    assert ten[at(0, 0)] == drawn[at(0, 0)] and ten[at(3, 2)] == drawn[at(3, 2)]
    assert close(ten[at(15, 9)], [u / 2 for u in drawn[at(15, 9)][:3]] + [1.0])
    eleven = c["FX-VIGNETTE-011"]
    assert eleven["0"] == drawn and eleven["2"] == one and eleven["4"] == two
    twelve = c["FX-VIGNETTE-012"]
    assert twelve["0"] == one and twelve["4"] == ten
    assert twelve["2"] == render(case(center=(25, 25)), 0)
    thirteen = c["FX-VIGNETTE-013"]
    assert thirteen["0"] == drawn and thirteen["4"] == five
    assert thirteen["2"] == render(case(size=125), 0)
    fourteen = c["FX-VIGNETTE-014"]
    assert ease(OVERSHOOT, 0.5) > 1 and fourteen["0"] == drawn and fourteen["2"] == two
    fifteen = c["FX-VIGNETTE-015"]["0"]
    assert all(fifteen[i] == [0.0, 0.0, 0.0, drawn[i][3]] for i in shows)
    assert fifteen[at(0, 4)][3] == 128 / 255
    assert c["FX-VIGNETTE-016"]["0"] == four
    moved = c["FX-VIGNETTE-017"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # After a directional blur: the ellipse is the drawing's own, about its own middle, and not
    # the grown layer's, nor about the grown layer's corner.
    grown = case(before=(90, 4))
    blurred, got = plain(grown), c["FX-VIGNETTE-018"]["0"]
    for i in range(W * H):
        op = v(i % W, i // W) / 2 if blurred[i][3] > 0 else 0
        assert close(got[i], [u * (1 - op) for u in blurred[i][:3]] + [blurred[i][3]]), i
    layer = layer_of(grown)
    wide = vignette(dict(layer, left=0, top=0), 50, "#000000", 100, 0, 50, (50, 50),
                    drawing=(W + 4, H + 4))
    corner = vignette(dict(layer, left=0, top=0), 50, "#000000", 100, 0, 50, (50, 50))
    inside = lambda px: [px[(y + 2) * (W + 4) + x + 2] for y in range(H) for x in range(W)]  # noqa: E731,E501
    assert not near(inside(wide), got, 1e-6) and not near(inside(corner), got, 1e-6)
    print("checked")


if __name__ == "__main__":
    main()
