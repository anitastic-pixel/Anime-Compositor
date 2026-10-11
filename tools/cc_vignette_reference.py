"""CC Vignette, worked a second way.

D-562 (B-442) adds `core.cc_vignette`, after After Effects' CC Vignette (Stylize): a picture's
edges darkened as a real lens darkens them, the light falling off with the angle it reaches the
film at, or, with a negative amount, lightened. It is a second effect beside D-126's Vignette
(`core.vignette`), not a second name over it: CC Vignette's Amount, Angle of View and Pin
Highlights do not map onto Vignette's colour, size, roundness and softness (D-562 says why).
CycoreFX publishes no formula and Adobe's page for it could not be read (it answers 403 to this
machine), so the rule is this program's own, built on the cos^4 law of a lens's natural
vignetting; nothing is ported. Document 21 is the rule in words; this file is the reference for
the numbers document 25 pins against it.

The rule. `amount`, -100..100; `angle_of_view`, degrees in 0..179; `center`, per cent of the
drawing's own width and height, each -1000..1000; `pin_highlights`, 0..100; every number
keyable and held inside its range at each frame. Starting at 100, 90, (50, 50) and 0 (this
program's choice). With w, h the drawing's own size, c the centre (center_x / 100 * w,
center_y / 100 * h) in the drawing's own pixels (an earlier effect that grew the layer does not
move it), R = sqrt(w^2 + h^2) / 2 and P a pixel's centre: s = |P - c| / R tan(angle_of_view /
2), f = 1 / (1 + s^2)^2 (cos^4 of the angle whose tangent is s: 1 at the centre, falling away
from it) and the gain g = f^(amount / 100), below 1 for a positive amount and above 1 for a
negative one. At a pixel with covering a > 0, with b its straight linear colour and L the luma
0.2126 e_r + 0.7152 e_g + 0.0722 e_b of its colour through the sRGB curve held inside 0..1, the
pinned gain is G = g + (1 - g) L pin_highlights / 100, so a bright pixel keeps more of itself,
and each channel is min(b G, max(b, 1)) a: lightening never takes a channel past 1, nor pulls
one already past it back. The covering is kept; a pixel with a = 0 stays as it is; amount 0 or
angle of view 0 leave the layer exactly as it is. The layer does not grow, and nothing is scaled
for a draft (every setting is a share of the drawing or an angle).

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Vignette's full-frame backdrop (tools/vignette_reference.py). The drawing goes
into `Fixtures/cc_vignette/media`, the projects into `Fixtures/cc_vignette`, and the expected
frames into `Fixtures/cc_vignette/expected_cc_vignette.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_vignette_reference.py
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
import vignette_reference as V  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_vignette"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (-100, 100), "angle_of_view": (0, 179), "center": (-1000, 1000),
          "pin_highlights": (0, 100)}
NAMES = ("amount", "angle_of_view", "center", "pin_highlights")


# --- the rule -------------------------------------------------------------------------------

def to_srgb(v):
    return 12.92 * v if v <= 0.0031308 else 1.055 * v ** (1 / 2.4) - 0.055


def falloff(px, py, center, angle_of_view, w=W, h=H):
    """f at the point (px, py) of the drawing's own pixels: cos^4 of the angle the light
    reaches it at, 1 at the centre."""
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    r = math.hypot(px - cx, py - cy) / (math.hypot(w, h) / 2)
    s = r * math.tan(math.radians(angle_of_view) / 2)
    return 1 / (1 + s * s) ** 2


def luma(b):
    e = [to_srgb(min(1.0, max(0.0, v))) for v in b]
    return 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]


def cc_vignette(layer, amount, angle_of_view, center, pin_highlights, drawing=(W, H)):
    """The layer's premultiplied pixels, vignetted; its rectangle kept. `drawing` is the
    drawing's own w by h; the layer's `left` and `top` are the growth of earlier effects."""
    if amount == 0 or angle_of_view == 0:
        return [list(p) for p in layer["px"]]
    out = []
    for i, p in enumerate(layer["px"]):
        a = p[3]
        if a == 0:
            out.append(list(p))
            continue
        x, y = layer["left"] + i % layer["w"] + 0.5, layer["top"] + i // layer["w"] + 0.5
        g = falloff(x, y, center, angle_of_view, *drawing) ** (amount / 100)
        b = [v / a for v in p[:3]]
        G = g + (1 - g) * luma(b) * pin_highlights / 100
        out.append([min(v * G, max(v, 1.0)) * a for v in b] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = V.DRAWINGS  # the backdrop


def case(amount=100, angle_of_view=90, center=(50, 50), pin_highlights=0, shift=0, before=None):
    """`before` is a directional blur (direction, length) ahead of CC Vignette."""
    return {"drawing": "backdrop", "amount": amount, "angle_of_view": angle_of_view,
            "center": center, "pin_highlights": pin_highlights, "shift": shift,
            "before": before}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def layer_of(c):
    """The drawing as CC Vignette receives it: grown by a directional blur ahead of it."""
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
    px = cc_vignette(layer, s["amount"], s["angle_of_view"], s["center"], s["pin_highlights"])
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
    "FX-CCVIG-001": ("The settings as they start: amount 100, angle of view 90, about the "
                     "middle, pin highlights 0. Each pixel's linear light is multiplied by the "
                     "cos^4 falloff: the bottom right pixel keeps 0.289 of it, the middle four "
                     "keep 0.989; nothing is lightened, and the empty corner stays "
                     "empty.",
                     case(), [0]),
    "FX-CCVIG-002": ("Amount 50: the square root of FX-CCVIG-001's gain at every pixel, the "
                     "bottom right keeping 0.538.",
                     case(amount=50), [0]),
    "FX-CCVIG-003": ("Amount 0: the drawing, untouched.",
                     case(amount=0), [0]),
    "FX-CCVIG-004": ("Amount -100: the edges lightened by the inverse of FX-CCVIG-001's gain, "
                     "every channel held at 1: the skin near the corners turns white, the "
                     "middle barely changes.",
                     case(amount=-100), [0]),
    "FX-CCVIG-005": ("Amount -50: lightened by the inverse square root, less than FX-CCVIG-004.",
                     case(amount=-50), [0]),
    "FX-CCVIG-006": ("Angle of view 0: no falloff at all: the drawing, untouched.",
                     case(angle_of_view=0), [0]),
    "FX-CCVIG-007": ("Angle of view 150: a wide lens, a much stronger falloff, the bottom right "
                     "keeping 0.0059 of its light.",
                     case(angle_of_view=150), [0]),
    "FX-CCVIG-008": ("Angle of view 30: a long lens, a gentle falloff, the bottom right keeping "
                     "0.887.",
                     case(angle_of_view=30), [0]),
    "FX-CCVIG-009": ("Centre 0, 0, the top left corner: the falloff about that corner, so the "
                     "top left pixel is all but untouched and the bottom right darkest of all.",
                     case(center=(0, 0)), [0]),
    "FX-CCVIG-010": ("Pin highlights 100: each pixel keeps more of itself the brighter it is: "
                     "the skin (luma 0.86) loses only about a seventh of what FX-CCVIG-001 takes "
                     "from it, the dark line along row 7 nearly all of it.",
                     case(pin_highlights=100), [0]),
    "FX-CCVIG-011": ("Pin highlights 50: half way between FX-CCVIG-001's gain and "
                     "FX-CCVIG-010's at every pixel.",
                     case(pin_highlights=50), [0]),
    "FX-CCVIG-012": ("Amount -100, pin highlights 100: the lightening held back on bright "
                     "pixels as the darkening is.",
                     case(amount=-100, pin_highlights=100), [0]),
    "FX-CCVIG-013": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the "
                     "drawing, frame 2 FX-CCVIG-002 and frame 4 FX-CCVIG-001.",
                     case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-CCVIG-014": ("Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                     "FX-CCVIG-001; frame 0 is the drawing.",
                     case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-CCVIG-015": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 "
                     "is FX-CCVIG-001, frame 2 is about 25, 25 and frame 4 is FX-CCVIG-009.",
                     case(center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-CCVIG-016": ("Angle of view keyed from 0 at frame 0 to 150 at frame 4, linear: frame 0 "
                     "the drawing, frame 4 FX-CCVIG-007.",
                     case(angle_of_view=keyed((0, 0), (4, 150))), [0, 2, 4]),
    "FX-CCVIG-017": ("FX-CCVIG-001 moved three pixels right: the same, moved; the vignette "
                     "moves with the drawing.",
                     case(shift=3), [0, 3]),
    "FX-CCVIG-018": ("A directional blur, direction 90 and length 4, then CC Vignette as it "
                     "starts: the blur grew the layer two pixels on every side, and the falloff "
                     "is still the drawing's own, about its own middle.",
                     case(before=(90, 4)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CCVIG-019": ("Amount 101, above 100.", case(amount=101)),
    "FX-CCVIG-020": ("Amount -101, below -100.", case(amount=-101)),
    "FX-CCVIG-021": ("Angle of view 180, above 179.", case(angle_of_view=180)),
    "FX-CCVIG-022": ("Angle of view -1, below 0.", case(angle_of_view=-1)),
    "FX-CCVIG-023": ("Pin highlights 101, above 100.", case(pin_highlights=101)),
    "FX-CCVIG-024": ("Centre 50, -1001, past ten heights.", case(center=(50, -1001))),
    "FX-CCVIG-025": ("Pin highlights keyed to 150 at frame 4.",
                     case(pin_highlights=keyed((0, 0), (4, 150)))),
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
        "instance_id": f"fx-0-{len(effects)}", "type_id": "core.cc_vignette", "enabled": True,
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

    (OUT / "expected_cc_vignette.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    kept = lambda f, i: f[i][2] / drawn[i][2]  # noqa: E731  # share kept, on the blue channel
    f0 = lambda x, y, **kw: falloff(x + 0.5, y + 0.5, **dict(  # noqa: E731
        dict(center=(50, 50), angle_of_view=90), **kw))

    # The rule's own pieces: 1 at the centre, cos^4 of the angle elsewhere.
    assert falloff(8, 5, (50, 50), 90) == 1 and falloff(0, 0, (50, 50), 0) == 1
    assert abs(falloff(16, 10, (50, 50), 90) - math.cos(math.radians(45)) ** 4) < 1e-12
    assert abs(luma([1, 1, 1]) - 1) < 1e-12 and luma([0, 0, 0]) == 0
    assert abs(to_srgb(srgb_to_linear(0.4)) - 0.4) < 1e-12

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

    one = c["FX-CCVIG-001"]["0"]
    assert abs(kept(one, at(15, 9)) - 0.289) < 0.0005
    assert all(kept(one, at(x, y)) > 0.988 for x in (7, 8) for y in (4, 5))
    for i in shows:
        x, y = i % W, i // W
        assert all(abs(one[i][k] - drawn[i][k] * f0(x, y)) < 1e-12 for k in range(3)), i
    two = c["FX-CCVIG-002"]["0"]
    assert abs(kept(two, at(15, 9)) - 0.538) < 0.0005
    assert all(abs(kept(two, i) - math.sqrt(kept(one, i))) < 1e-12 for i in shows)
    assert c["FX-CCVIG-003"]["0"] == drawn
    four = c["FX-CCVIG-004"]["0"]
    assert all(four[i][k] >= drawn[i][k] for i in shows for k in range(3))
    assert close(four[at(1, 1)], [1.0, 1.0, 1.0, 1.0])  # skin near the corner turns white
    assert all(four[at(8, 4)][k] <= drawn[at(8, 4)][k] * 1.012 for k in range(3))
    five = c["FX-CCVIG-005"]["0"]
    assert all(drawn[i][k] <= five[i][k] <= four[i][k] for i in shows for k in range(3))
    assert five != four
    assert c["FX-CCVIG-006"]["0"] == drawn
    seven = c["FX-CCVIG-007"]["0"]
    assert abs(kept(seven, at(15, 9)) - 0.0059) < 0.00005
    assert all(kept(seven, i) <= kept(one, i) for i in shows)
    eight = c["FX-CCVIG-008"]["0"]
    assert abs(kept(eight, at(15, 9)) - 0.887) < 0.0005
    assert all(kept(eight, i) >= kept(one, i) for i in shows)
    nine = c["FX-CCVIG-009"]["0"]
    assert kept(nine, at(0, 0)) > 0.988 and kept(nine, at(15, 9)) < kept(one, at(15, 9))
    assert min(kept(nine, i) for i in shows) == kept(nine, at(15, 9))
    ten = c["FX-CCVIG-010"]["0"]
    skin, line = at(2, 2), at(2, 7)
    lost = lambda f, i: 1 - kept(f, i)  # noqa: E731
    assert abs(luma([v / drawn[skin][3] for v in drawn[skin][:3]]) - 0.86) < 0.005
    assert lost(ten, skin) < lost(one, skin) / 6
    assert lost(ten, line) > 0.85 * lost(one, line)
    eleven = c["FX-CCVIG-011"]["0"]
    assert all(abs(kept(eleven, i) - (kept(one, i) + kept(ten, i)) / 2) < 1e-12 for i in shows)
    twelve = c["FX-CCVIG-012"]["0"]
    assert all(drawn[i][k] <= twelve[i][k] <= four[i][k] for i in shows for k in range(3))
    assert twelve[at(15, 9)][2] < four[at(15, 9)][2]
    thirteen = c["FX-CCVIG-013"]
    assert thirteen["0"] == drawn and thirteen["2"] == two and thirteen["4"] == one
    fourteen = c["FX-CCVIG-014"]
    assert ease(OVERSHOOT, 0.5) > 1 and fourteen["0"] == drawn and fourteen["2"] == one
    fifteen = c["FX-CCVIG-015"]
    assert fifteen["0"] == one and fifteen["4"] == nine
    assert fifteen["2"] == render(case(center=(25, 25)), 0)
    sixteen = c["FX-CCVIG-016"]
    assert sixteen["0"] == drawn and sixteen["4"] == seven
    assert sixteen["2"] == render(case(angle_of_view=75), 0)
    moved = c["FX-CCVIG-017"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # After a directional blur: the falloff is the drawing's own, about its own middle.
    grown = case(before=(90, 4))
    blurred, got = plain(grown), c["FX-CCVIG-018"]["0"]
    for i in range(W * H):
        g = f0(i % W, i // W) if blurred[i][3] > 0 else 1
        assert close(got[i], [u * g for u in blurred[i][:3]] + [blurred[i][3]]), i
    layer = layer_of(grown)
    wide = cc_vignette(dict(layer, left=0, top=0), 100, 90, (50, 50), 0, drawing=(W + 4, H + 4))
    inside = [wide[(y + 2) * (W + 4) + x + 2] for y in range(H) for x in range(W)]
    assert not all(close(p, q, 1e-6) for p, q in zip(inside, got))
    print("checked")


if __name__ == "__main__":
    main()
