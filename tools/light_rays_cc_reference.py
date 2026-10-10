"""Light Rays with CC Light Rays' controls, worked a second way.

D-423 gives `core.light_rays` the controls of CycoreFX's CC Light Rays: Intensity, Center,
Radius, Warp Softness, Shape (Round, Square), Direction, Color from Source, Allow Brightening,
Color and Transfer Mode (None, Add, Lighten, Screen). Its manual (CycoreFX HD 1.8.9, pages 43 and
44) says what each control does in words and gives no formula, ranges or defaults, so the rule,
the ranges and the defaults are ours; nothing is ported. A Light Rays saved before D-423, whose
settings are D-124's (`length`, `threshold`, `intensity` 0 to 10, `color`), still opens as
D-124's and draws exactly as before: `tools/light_rays_reference.py` and FX-RAYS-001 to 024 keep
pinning it. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. With c the centre in layer pixels, (center_x / 100 * width, center_y / 100 * height),
p a pixel's centre and d = p - c:

(1) The light source, a shape of size `radius` pixels about c (the manual: "the radius (size)
of the light source"): Round, the distance |d|; Square, d turned by -`direction` degrees (on
screen, clockwise for a positive direction) to (u, v), the distance max(|u|, |v|), so `radius`
is half the square's side; Direction does nothing for Round (the manual: "Not in use if Round
is selected"). Each pixel's share of the source m = clamp(radius + 0.5 - distance, 0, 1), a
pixel-wide soft edge.
(2) The light Lt = m times the layer, as it is, with `color_from_source` "on"; with it "off",
m times the layer's covering in every channel (white at the covering), to be tinted by Color.
(3) The rays Ry: the light through Radial Blur's zoom about c at amount 100
(`radial_blur_reference.blurred`, "zoom"), each pixel the plain average of the light on the line
from the centre out to it, so the light streaks outward from the source and fades as the line
grows longer past it: a larger radius reaches further and is brighter (the manual: "This affects
how far the light rays reach. Higher values produce brighter results"); then Radial Blur's spin
about c of `warp_softness` / 10 degrees, both ways evenly, which melts neighbouring rays
together (the manual: "higher values softens (warps) light rays together into larger, more
subtle, rays"); warp softness 0 leaves the zoom as it is.
(4) k = intensity / 100, held at 1 or less when `allow_brightening` is "off" (our reading of the
manual's "give brightness a boost at higher Intensity values, 100 or greater"). The rays' colour
R = k T Ry, Ry's colour with Color from Source on, Ry's covering with it off; T white with
Color from Source on, Color in linear light with it off. Their covering a = min(1, k Ry.a).
(5) Laid on the layer O by `transfer_mode`, premultiplied, channel by channel:
  "none"     R + O (1 - a), covering a + O.a (1 - a): the rays over the layer, normally;
  "add"      O + R, covering min(1, O.a + k Ry.a): D-124's last step;
  "screen"   O + R - O R, covering O.a + a - O.a a;
  "lighten"  max(O, R), covering max(O.a, a).
Nothing is cut off at white. The layer does not grow: the rays stop at its edge. `radius` is a
distance: a draft divides it by its scale.

Ranges and defaults (ours): intensity 0 to 2000, 100; center -1000 to 1000 per cent, (50, 50);
radius 0 to 10000 pixels, 50; warp_softness 0 to 1000, 50; shape "round"; direction -3600 to
3600 degrees, 0; color_from_source "on"; allow_brightening "on"; color #ffffff; transfer_mode
"none". The numbers are keyable.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding Light Rays' lamp drawing, unmoved unless the
case says. The drawing goes into `Fixtures/light_rays_cc/media`, the projects into
`Fixtures/light_rays_cc`, and the expected frames into
`Fixtures/light_rays_cc/expected_light_rays_cc.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/light_rays_cc_reference.py
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
import radial_blur_reference as RB  # noqa: E402
import light_rays_reference as LR  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "light_rays_cc"
TOLERANCE = 2e-5  # document 25's default for a filter
INTENSITY, CENTER, RADIUS, WARP, DIRECTION = (0, 2000), (-1000, 1000), (0, 10000), (0, 1000), \
    (-3600, 3600)
NAMES = ("intensity", "center", "radius", "warp_softness", "shape", "direction",
         "color_from_source", "allow_brightening", "color", "transfer_mode")


# --- the rule -------------------------------------------------------------------------------

def share(x, y, c, radius, shape, direction):
    """The pixel (x, y)'s share of the light source."""
    dx, dy = x + 0.5 - c[0], y + 0.5 - c[1]
    if shape == "square":
        t = math.radians(direction)
        u = dx * math.cos(t) + dy * math.sin(t)
        v = -dx * math.sin(t) + dy * math.cos(t)
        distance = max(abs(u), abs(v))
    else:
        distance = math.sqrt(dx * dx + dy * dy)
    return min(1.0, max(0.0, radius + 0.5 - distance))


def light_rays(pixels, intensity, center, radius, warp, shape, direction, from_source,
               brighten, color, mode):
    """The layer's pixels with the rays laid on. `pixels` is the drawing's 8-bit straight
    pixels, row by row, 16 by 10."""
    working = [R.working(p) for p in pixels]
    c = (center[0] / 100 * W, center[1] / 100 * H)
    # 1, 2. The light: the source's share of the layer, or of its covering.
    lit = []
    for i, q in enumerate(working):
        m = share(i % W, i // W, c, radius, shape, direction)
        lit.append([m * v for v in q] if from_source == "on" else [m * q[3]] * 4)
    layer = lambda px: {"px": px, "left": 0, "top": 0, "w": W, "h": H}  # noqa: E731
    # 3. The rays: zoomed out from the centre, then turned both ways by the warp.
    zoomed = [RB.blurred(layer(lit), "zoom", 100, center, i % W, i // W) for i in range(W * H)]
    turn = warp / 10
    rays = [RB.blurred(layer(zoomed), "spin", turn, center, i % W, i // W)
            for i in range(W * H)] if turn else zoomed
    # 4. The strength and the tint.
    k = intensity / 100 if brighten == "on" else min(intensity / 100, 1.0)
    tint = [1.0] * 3 if from_source == "on" else \
        [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    out = []
    for o, ry in zip(working, rays):
        r = [k * tint[ch] * (ry[ch] if from_source == "on" else ry[3]) for ch in range(3)]
        a = min(1.0, k * ry[3])
        # 5. Laid on.
        if mode == "add":
            out.append([o[ch] + r[ch] for ch in range(3)] + [min(1.0, o[3] + k * ry[3])])
        elif mode == "screen":
            out.append([o[ch] + r[ch] - o[ch] * r[ch] for ch in range(3)]
                       + [o[3] + a - o[3] * a])
        elif mode == "lighten":
            out.append([max(o[ch], r[ch]) for ch in range(3)] + [max(o[3], a)])
        else:
            out.append([r[ch] + o[ch] * (1 - a) for ch in range(3)] + [a + o[3] * (1 - a)])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(intensity=100, center=(50, 50), radius=50, warp_softness=50, shape="round", direction=0,
         color_from_source="on", allow_brightening="on", color="#ffffff", transfer_mode="none",
         shift=0):
    return {"drawing": "lamp", "intensity": intensity, "center": center, "radius": radius,
            "warp_softness": warp_softness, "shape": shape, "direction": direction,
            "color_from_source": color_from_source, "allow_brightening": allow_brightening,
            "color": color, "transfer_mode": transfer_mode, "shift": shift}


def small(**kw):
    """A source of radius 3, inside the drawing, the warp 0 unless said."""
    kw.setdefault("radius", 3)
    kw.setdefault("warp_softness", 0)
    return case(**kw)


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [p for row in LR.DRAWINGS[c["drawing"]] for p in row]
    center = [clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    number = lambda k, r: clamp(value_at(c[k], frame_no), r)  # noqa: E731
    return R.frame(light_rays(pixels, number("intensity", INTENSITY), center,
                              number("radius", RADIUS), number("warp_softness", WARP),
                              c["shape"], number("direction", DIRECTION),
                              c["color_from_source"], c["allow_brightening"], c["color"],
                              c["transfer_mode"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(intensity=0, shift=c["shift"]), 0)


CASES = {
    "FX-RAYSCC-001": ("The settings as they start: intensity 100, centre 50, 50, the point "
                      "(8, 5), radius 50, warp softness 50, Round, Color from Source on, Allow "
                      "Brightening on, Transfer Mode None. The source covers the whole drawing, "
                      "so all of it is light: each pixel is the drawing averaged along the line "
                      "from the centre out to it, turned 5 degrees both ways, laid over the "
                      "drawing. The empty columns and rows round the drawing take the patches' "
                      "rays; the patches show through where the rays are thin.",
                      case(), [0]),
    "FX-RAYSCC-002": ("Intensity 0: the drawing, untouched.",
                      case(intensity=0), [0]),
    "FX-RAYSCC-003": ("Radius 3, warp softness 0: only the pixels within 3 of the centre give "
                      "light, the yellow's columns 5 to 7 and the brown's column 10, their four "
                      "corner pixels only in part (the source's soft edge); the yellow's half "
                      "edge in column 4, the brown's column 11 and the purple, outside, give "
                      "none. Their rays streak outward, the yellow's to the left edge, the "
                      "brown's to the right, laid over the drawing.",
                      small(), [0]),
    "FX-RAYSCC-004": ("FX-RAYSCC-003 with a Square source: the square reaches 3 each way and "
                      "into its corners, so the four corner pixels the round source takes only "
                      "in part give all their light.",
                      small(shape="square"), [0]),
    "FX-RAYSCC-005": ("The square turned 45 degrees: a diamond, reaching 4.2 along the row and "
                      "the column through the centre, so the brown's column 11 gives part of "
                      "its light, and less to the corners.",
                      small(shape="square", direction=45), [0]),
    "FX-RAYSCC-006": ("Round with direction 45: direction does nothing for a round source; "
                      "FX-RAYSCC-003.",
                      small(direction=45), [0]),
    "FX-RAYSCC-007": ("Warp softness 300: FX-RAYSCC-003's rays turned 30 degrees both ways, "
                      "so they spread round the centre and melt together.",
                      small(warp_softness=300), [0]),
    "FX-RAYSCC-008": ("Color from Source off, colour white: the light is white at the "
                      "source's covering, so the yellow and the brown give white rays.",
                      small(color_from_source="off"), [0]),
    "FX-RAYSCC-009": ("Color from Source off, colour #ff8000, orange: orange rays, red as much "
                      "as the rays' covering, a fifth as much green, no blue.",
                      small(color_from_source="off", color="#ff8000"), [0]),
    "FX-RAYSCC-010": ("Color from Source on with colour #ff8000: the colour does not count; "
                      "FX-RAYSCC-003.",
                      small(color="#ff8000"), [0]),
    "FX-RAYSCC-011": ("FX-RAYSCC-009 with its colour written in capitals, #FF8000: the same.",
                      small(color_from_source="off", color="#FF8000"), [0]),
    "FX-RAYSCC-012": ("Transfer Mode Add: the rays added onto the drawing, D-124's way, so the "
                      "lit patches brighten rather than being covered.",
                      small(transfer_mode="add"), [0]),
    "FX-RAYSCC-013": ("Transfer Mode Screen: brighter than the drawing everywhere the rays "
                      "fall, but less than Add where both are bright.",
                      small(transfer_mode="screen"), [0]),
    "FX-RAYSCC-014": ("Transfer Mode Lighten: each channel the larger of the drawing's and "
                      "the rays'.",
                      small(transfer_mode="lighten"), [0]),
    "FX-RAYSCC-015": ("Intensity 300, Add: three times FX-RAYSCC-012's rays, past white, not "
                      "cut off; the covering stops at full.",
                      small(intensity=300, transfer_mode="add"), [0]),
    "FX-RAYSCC-016": ("Intensity 300, Add, Allow Brightening off: held at 100; "
                      "FX-RAYSCC-012.",
                      small(intensity=300, transfer_mode="add", allow_brightening="off"), [0]),
    "FX-RAYSCC-017": ("Intensity 50, Allow Brightening off: below 100 nothing is held; the "
                      "same as with it on.",
                      small(intensity=50, allow_brightening="off"), [0]),
    "FX-RAYSCC-018": ("Intensity 50, Allow Brightening on, the pair of FX-RAYSCC-017.",
                      small(intensity=50), [0]),
    "FX-RAYSCC-019": ("Radius 0: the centre sits on a pixel corner, every pixel centre at "
                      "least 0.7 from it, so nothing is light; the drawing, untouched.",
                      small(radius=0), [0]),
    "FX-RAYSCC-020": ("Centre 25, 50, the point (4, 5), radius 3: the yellow alone is the "
                      "source, its rays streaking left to the edge and right across the gap "
                      "and over the brown and the purple.",
                      small(center=(25, 50)), [0]),
    "FX-RAYSCC-021": ("Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is "
                      "FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the "
                      "purple.",
                      small(radius=keyed((0, 0), (4, 6))), [0, 2, 4]),
    "FX-RAYSCC-022": ("A Square source with direction keyed from 0 at frame 0 to 90 at frame "
                      "4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a "
                      "square, is frame 0 again.",
                      small(shape="square", direction=keyed((0, 0), (4, 90))), [0, 2, 4]),
    "FX-RAYSCC-023": ("Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that "
                      "overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as "
                      "frame 4 is.",
                      small(intensity=keyed((0, 0, OVERSHOOT), (4, 2000)), transfer_mode="add"),
                      [0, 2, 4]),
    "FX-RAYSCC-024": ("FX-RAYSCC-003 moved three pixels right: the rays are drawn on the "
                      "drawing before it is moved, so it is FX-RAYSCC-003 moved, and the three "
                      "columns left of the drawing stay empty, as the layer does not grow.",
                      small(shift=3), [0, 3]),
    "FX-RAYSCC-025": ("Intensity 250, None: the rays' colour runs past their covering, laid "
                      "over the drawing; their covering stops at full.",
                      small(intensity=250), [0]),
    "FX-RAYSCC-026": ("Square 4, direction 30, warp 120, Color from Source off #40c0ff, "
                      "intensity 180, Screen, centre 40, 60: the controls together.",
                      small(shape="square", radius=4, direction=30, warp_softness=120,
                            color_from_source="off", color="#40c0ff", intensity=180,
                            transfer_mode="screen", center=(40, 60)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RAYSCC-027": ("Intensity -1, below 0.", case(intensity=-1)),
    "FX-RAYSCC-028": ("Intensity 2001, above 2000.", case(intensity=2001)),
    "FX-RAYSCC-029": ("Intensity keyed to 3000 at frame 4.",
                      case(intensity=keyed((0, 100), (4, 3000)))),
    "FX-RAYSCC-030": ("Centre 50, -1001, past ten heights.", case(center=(50, -1001))),
    "FX-RAYSCC-031": ("Radius -1, below 0.", case(radius=-1)),
    "FX-RAYSCC-032": ("Radius 10001, above 10000.", case(radius=10001)),
    "FX-RAYSCC-033": ("Warp softness 1001, above 1000.", case(warp_softness=1001)),
    "FX-RAYSCC-034": ("Direction 3601, past ten turns.", case(direction=3601)),
    "FX-RAYSCC-035": ("Shape \"triangle\", neither \"round\" nor \"square\".",
                      case(shape="triangle")),
    "FX-RAYSCC-036": ("Color from Source \"yes\", neither \"off\" nor \"on\".",
                      case(color_from_source="yes")),
    "FX-RAYSCC-037": ("Allow Brightening \"yes\", neither \"off\" nor \"on\".",
                      case(allow_brightening="yes")),
    "FX-RAYSCC-038": ("Transfer Mode \"multiply\", not one of the four.",
                      case(transfer_mode="multiply")),
    "FX-RAYSCC-039": ("A colour written \"#12345\", one digit short.", case(color="#12345")),
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
        "instance_id": "fx-0-0", "type_id": "core.light_rays", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in LR.DRAWINGS.items():
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

    (OUT / "expected_light_rays_cc.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    cs = (8.0, 5.0)

    # Everywhere the covering stays inside 0 to 1.
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-15 for p in px), fx

    # The source's share: round 3 takes the yellow's columns 6 and 7 and the brown's column 10
    # in part or whole, and nothing of the purple; square takes corners round leaves out.
    assert share(7, 4, cs, 3, "round", 0) == 1 and share(14, 4, cs, 3, "round", 0) == 0
    assert share(4, 4, cs, 3, "round", 0) == 0 and share(11, 4, cs, 3, "round", 0) == 0
    assert 0 < share(10, 3, cs, 3, "round", 0) < 1 and share(10, 3, cs, 3, "square", 0) == 1
    assert 0 < share(5, 6, cs, 3, "round", 0) < 1 and share(5, 6, cs, 3, "square", 0) == 1
    # Turned 45, the square is a diamond: it reaches further along the row, less to a corner.
    assert 0 < share(11, 4, cs, 3, "square", 45) < 1 and share(11, 4, cs, 3, "square", 0) == 0
    assert share(10, 3, cs, 3, "square", 45) < 1
    # Radius 0 with the centre on a corner: nothing.
    assert all(share(x, y, cs, 0, s, 0) == 0 for x in range(W) for y in range(H)
               for s in ("round", "square"))

    one = c["FX-RAYSCC-001"]["0"]
    assert one != drawn and all(one[at(x, 0)][3] > 0 for x in (5, 6, 7, 10, 11))
    assert c["FX-RAYSCC-002"]["0"] == drawn
    three = c["FX-RAYSCC-003"]["0"]
    # Rays reach the empty columns either side, from the yellow and the brown.
    assert drawn[at(1, 5)][3] == 0 < three[at(1, 5)][3]
    assert drawn[at(13, 5)][3] == 0 < three[at(13, 5)][3]
    assert c["FX-RAYSCC-004"]["0"] != three and c["FX-RAYSCC-005"]["0"] != c["FX-RAYSCC-004"]["0"]
    assert c["FX-RAYSCC-006"]["0"] == three
    # 007: the warp spreads the rays into the empty rows the plain zoom leaves dark there.
    seven = c["FX-RAYSCC-007"]["0"]
    assert seven != three and sum(p[3] for p in seven) != sum(p[3] for p in three)
    # 008, 009: white and orange at the covering; 010 the colour unread; 011 capitals.
    eight, nine = c["FX-RAYSCC-008"]["0"], c["FX-RAYSCC-009"]["0"]
    for i in range(W * H):
        assert eight[i][3] == nine[i][3]
    i = at(1, 5)
    assert abs(eight[i][0] - eight[i][2]) < 1e-12 and nine[i][2] == 0 and nine[i][0] > nine[i][1] > 0
    assert c["FX-RAYSCC-010"]["0"] == three and c["FX-RAYSCC-011"]["0"] == nine
    # 012 to 014: the modes; add brightest, lighten the least over the lit patch.
    add, scr, lit = (c[f"FX-RAYSCC-01{n}"]["0"] for n in (2, 3, 4))
    i = at(7, 4)
    assert add[i][0] >= scr[i][0] >= lit[i][0] >= drawn[i][0] and add[i][0] > scr[i][0]
    for j in range(W * H):
        assert all(lit[j][ch] >= drawn[j][ch] - 1e-15 for ch in range(4))
        assert all(scr[j][ch] >= drawn[j][ch] - 1e-15 for ch in range(4))
    # 015, 016: past white; held at 100.
    assert any(v > 1 for p in c["FX-RAYSCC-015"]["0"] for v in p[:3])
    assert c["FX-RAYSCC-016"]["0"] == add
    assert c["FX-RAYSCC-017"]["0"] == c["FX-RAYSCC-018"]["0"]
    assert c["FX-RAYSCC-019"]["0"] == drawn
    # 020: the yellow's rays cross the gap rightward over the brown and purple.
    twenty = c["FX-RAYSCC-020"]["0"]
    assert twenty[at(9, 5)][3] > 0 and twenty[at(14, 5)] != drawn[at(14, 5)]
    # The keyed cases meet the plain ones at their frames.
    k = c["FX-RAYSCC-021"]
    assert k["0"] == drawn and k["2"] == three and k["4"][at(10, 5)] != three[at(10, 5)]
    k = c["FX-RAYSCC-022"]
    assert near(k["2"], c["FX-RAYSCC-005"]["0"]) and near(k["4"], k["0"])
    assert near(k["0"], c["FX-RAYSCC-004"]["0"])
    k = c["FX-RAYSCC-023"]
    assert ease(OVERSHOOT, 0.5) > 1 and k["0"] == drawn and k["2"] == k["4"]
    moved = c["FX-RAYSCC-024"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # 025: colour past the covering, covering at most full.
    tf = c["FX-RAYSCC-025"]["0"]
    assert any(p[0] > p[3] for p in tf)
    # Add with Color from Source on and a source over the whole drawing is D-124's last step on the source's light, exactly.
    pixels = [p for row in LR.DRAWINGS["lamp"] for p in row]
    whole = light_rays(pixels, 100, (50, 50), 10000, 0, "round", 0, "on", "on", "#ffffff", "add")
    old = LR.light_rays(pixels, (50, 50), 100, 0, 1, "#ffffff")
    assert near(whole, old)
    print("checked")


if __name__ == "__main__":
    main()
