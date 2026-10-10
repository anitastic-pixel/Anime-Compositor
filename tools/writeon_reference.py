"""Write-on, worked a second way.

D-441 adds `core.write_on`, after After Effects' Write-on (Generate): a brush that paints where
its Brush Position has been, so keying the position writes a line on over time. Adobe's manual
(After Effects CS6 help, "Write-on effect") says what each control does in a sentence:

- Brush Position: "The position of the brush. Animate this property to create a stroke."
- Stroke Length (secs): "The duration, in seconds, of each brush mark. If this value is 0, the
  brush mark has unlimited duration."
- Brush Spacing (secs): "The time interval, in seconds, between brush marks."
- Paint Time Properties and Brush Time Properties: "whether paint properties and brush
  properties are applied to each brush mark or to the entire stroke. Choose None to apply values
  at each time to all brush marks in the stroke. Choose a property name for each brush mark to
  retain the value for that property at the time that the brush mark was drawn."
- Paint Style: On Original Image, On Transparent, Reveal Original Image.

It publishes no formula and no defaults; the rule below is this program's own reading, the brush
itself is D-356's Path Stroke brush, and nothing is ported.

The rule. W and H are the drawing's own size; fps the composition's frames per second. At
composition frame f, the layer's key time is u = in + (f - in) 100 / stretch (document 20, D-216:
`in` the layer's in point, `stretch` its time stretch), so the marks stretch with the layer.

1. Marks. s = `brush_spacing` and L = `stroke_length`, both read now (at u). Mark k = 0, 1, 2 ...
   was laid at the key time u_k = in + k (s fps), for every k up to n = floor((u - in) / (s fps)
   + 1e-9). With L > 0 a mark is kept only while u - u_k < L fps - 1e-9: each mark lasts L
   seconds of the layer's time. With L = 0 every mark is kept.
2. Each mark's settings. Its centre is `brush_position` at u_k (per cent of W and H). Its Brush
   Size and Hardness are each read at u_k when Brush Time Properties names it (`size`,
   `hardness`, `size_and_hardness`), and now otherwise (`none`); its Brush Opacity is read at u_k
   when Paint Time Properties names it (`opacity`, `color_and_opacity`), and now otherwise. Every
   value read is held to its range first, as any keyed setting is. The colour is always the one
   now: a colour is not a keyable setting in this program, so `color` reads as `none` and
   `color_and_opacity` as `opacity`, until colours can be keyed.
3. The brush, D-356's: at a pixel's centre, d_k the distance to mark k's centre, r_k = size_k / 2,
   w_k = max(r_k (1 - hardness_k / 100), 1), t_k = clamp((r_k + 0.5 - d_k) / w_k, 0, 1), and the
   mark covers c_k = opacity_k / 100 t_k^2 (3 - 2 t_k); a mark of size 0 covers nothing. Marks do
   not build up: c is the largest c_k. With every setting the same this is Path Stroke's brush
   along the marks.
4. Paint Style, Path Stroke's: On Original Image lays the colour C (linear) over the layer O,
   O.rgb (1 - c) + C c and O.a (1 - c) + c; On Transparent is the stroke alone, C c and c; Reveal
   Original Image is the layer only where the stroke is, O c. The layer never grows. A draft
   halves every mark's size as a distance; the positions are shares of the drawing.

Settings, in Adobe's order: `brush_position` -1000 to 1000 per cent each way, 50, 50; `color`
`#rrggbb`, `#ffffff`; `brush_size` 0 to 200, 6; `brush_hardness` 0 to 100, 75; `brush_opacity` 0
to 100, 100; `stroke_length` 0 to 3600 seconds, 0; `brush_spacing` 0.001 to 10 seconds, 0.01;
`paint_time_properties` one of none, color, opacity, color_and_opacity, `none`;
`brush_time_properties` one of none, size, hardness, size_and_hardness, `none`; `paint_style`
one of on_original, on_transparent, reveal, `on_original`. The numbers are keyable. The values
when added are chosen here (Adobe's page gives none).

**This file never runs the build's code path.** It lists every mark and works in double
precision on lists, straight from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 at 24 frames a second, 12 frames long, holding
Bulge's striped drawing the same size, unmoved unless the case says. The drawing goes into
`Fixtures/writeon/media`, the projects into `Fixtures/writeon`, and the expected frames into
`Fixtures/writeon/expected_writeon.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/writeon_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import bulge_reference as B  # noqa: E402

W, H = R.W, R.H
FPS = 24
FRAMES = 12
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "writeon"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"brush_position": (-1000, 1000), "brush_size": (0, 200), "brush_hardness": (0, 100),
          "brush_opacity": (0, 100), "stroke_length": (0, 3600), "brush_spacing": (0.001, 10)}
NAMES = ("brush_position", "color", "brush_size", "brush_hardness", "brush_opacity",
         "stroke_length", "brush_spacing", "paint_time_properties", "brush_time_properties",
         "paint_style")
EMPTY = [0.0] * 4
RED = "#ff3020"


# --- the rule -------------------------------------------------------------------------------

def held(c, k, u):
    v = value_at(c[k], u)
    lo, hi = RANGES[k]
    return (tuple(min(hi, max(lo, x)) for x in v) if isinstance(v, (list, tuple))
            else min(hi, max(lo, v)))


def key_time(c, f):
    return c["in"] + (f - c["in"]) * 100 / c["stretch"] if c["stretch"] != 100 else f


def marks(c, f):
    """Each mark kept at frame f: (x, y, size, hardness, opacity), x and y in pixels."""
    u = key_time(c, f)
    step = held(c, "brush_spacing", u) * FPS
    L = held(c, "stroke_length", u)
    n = math.floor((u - c["in"]) / step + 1e-9)
    brush, paint = c["brush_time_properties"], c["paint_time_properties"]
    out = []
    for k in range(n + 1):
        uk = c["in"] + k * step
        if L > 0 and not (u - uk < L * FPS - 1e-9):
            continue
        x, y = held(c, "brush_position", uk)
        size = held(c, "brush_size", uk if brush in ("size", "size_and_hardness") else u)
        hard = held(c, "brush_hardness", uk if brush in ("hardness", "size_and_hardness") else u)
        opa = held(c, "brush_opacity", uk if paint in ("opacity", "color_and_opacity") else u)
        out.append((x / 100 * W, y / 100 * H, size, hard, opa))
    return out


def covering(ms, X, Y):
    c = 0.0
    for x, y, size, hard, opa in ms:
        if size == 0:
            continue
        r = size / 2
        w = max(r * (1 - hard / 100), 1.0)
        t = min(1.0, max(0.0, (r + 0.5 - math.hypot(X - x, Y - y)) / w))
        c = max(c, opa / 100 * t * t * (3 - 2 * t))
    return c


def write_on(layer, ms, color, style):
    C = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            c = covering(ms, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            if style == "on_original":
                px.append([p[ch] * (1 - c) + C[ch] * c for ch in range(3)] + [p[3] * (1 - c) + c])
            elif style == "on_transparent":
                px.append([C[ch] * c for ch in range(3)] + [c])
            else:
                px.append([v * c for v in p])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **more):
    c = {"drawing": "stripes", "shift": shift, "in": 0, "stretch": 100,
         "brush_position": (50, 50), "color": "#ffffff", "brush_size": 6, "brush_hardness": 75,
         "brush_opacity": 100, "stroke_length": 0, "brush_spacing": 0.01,
         "paint_time_properties": "none", "brush_time_properties": "none",
         "paint_style": "on_original"}
    c.update(more)
    return c


def shown(c, f):
    return c["in"] <= f < FRAMES


def render(c, frame_no):
    if not shown(c, frame_no):
        return [EMPTY] * (W * H)
    layer = B.drawn_layer(c["drawing"])
    out = write_on(layer, marks(c, frame_no), c["color"], c["paint_style"])
    return [out["px"][y * W + x - c["shift"]] if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


def plain(c, frame_no=0):
    if not shown(c, frame_no):
        return [EMPTY] * (W * H)
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


# The brush runs left to right along the middle row, from 10 per cent at frame 0 to 90 at frame
# 8, linear: 1.6 pixels a frame, a mark every 0.02 seconds (0.48 frames), Brush Size 3.
SWEEP = dict(brush_position=keyed((0, (10, 50)), (8, (90, 50))), brush_size=3,
             brush_spacing=0.02)
GROW = dict(SWEEP, brush_size=keyed((0, 1), (8, 6)))

CASES = {
    "FX-WRITEON-001": ("The settings as added: the brush at the middle and never moved, Brush "
                       "Size 6, Hardness 75, white: every mark at the same place, one white "
                       "dot in the middle.", case(), [0, 4]),
    "FX-WRITEON-002": ("The brush keyed from 10, 50 at frame 0 to 90, 50 at frame 8, Brush Size "
                       "3, Brush Spacing 0.02 seconds: a white line written on left to right "
                       "along the middle, a dot at frame 0, reaching the right at frame 8.",
                       case(**SWEEP), [0, 2, 4, 8]),
    "FX-WRITEON-003": ("FX-WRITEON-002 with Brush Spacing 0.25 seconds (6 frames): marks far "
                       "apart, one dot at frame 4, two at frame 8.",
                       case(**dict(SWEEP, brush_spacing=0.25)), [4, 8]),
    "FX-WRITEON-004": ("FX-WRITEON-002 with Stroke Length 0.1 seconds (2.4 frames): only the "
                       "last 0.1 seconds of the line, a short dash travelling right.",
                       case(**dict(SWEEP, stroke_length=0.1)), [4, 8]),
    "FX-WRITEON-005": ("FX-WRITEON-002 with Hardness 0: soft from its middle out.",
                       case(**dict(SWEEP, brush_hardness=0)), [8]),
    "FX-WRITEON-006": ("FX-WRITEON-002 with Hardness 100: hard, smoothed over one pixel.",
                       case(**dict(SWEEP, brush_hardness=100)), [8]),
    "FX-WRITEON-007": ("FX-WRITEON-002 with Brush Opacity 50: half covered at most.",
                       case(**dict(SWEEP, brush_opacity=50)), [8]),
    "FX-WRITEON-008": ("FX-WRITEON-002 in red, #ff3020.", case(**dict(SWEEP, color=RED)), [8]),
    "FX-WRITEON-009": ("FX-WRITEON-002 On Transparent: the line alone, the drawing gone.",
                       case(**dict(SWEEP, paint_style="on_transparent")), [4]),
    "FX-WRITEON-010": ("FX-WRITEON-002 Reveal Original Image: the drawing only under the line, "
                       "the colour unused.", case(**dict(SWEEP, paint_style="reveal")), [4]),
    "FX-WRITEON-011": ("FX-WRITEON-002 with Brush Size 0: nothing is drawn; the drawing, "
                       "untouched.", case(**dict(SWEEP, brush_size=0)), [4]),
    "FX-WRITEON-012": ("Brush Size 0, On Transparent: nothing at all.",
                       case(**dict(SWEEP, brush_size=0, paint_style="on_transparent")), [4]),
    "FX-WRITEON-013": ("Brush Size keyed from 1 at frame 0 to 6 at frame 8, Brush Time "
                       "Properties None: the whole line takes the size now, thin at frame 2 "
                       "and thick at frame 8.", case(**GROW), [2, 8]),
    "FX-WRITEON-014": ("FX-WRITEON-013 with Brush Time Properties Size: each mark keeps the "
                       "size it was laid with, so the line swells from thin on the left to "
                       "thick on the right.", case(**dict(GROW, brush_time_properties="size")),
                       [2, 8]),
    "FX-WRITEON-015": ("Hardness keyed from 0 at frame 0 to 100 at frame 8, Brush Size 5, Brush "
                       "Time Properties Hardness: soft on the left, hard on the right.",
                       case(**dict(SWEEP, brush_size=5, brush_hardness=keyed((0, 0), (8, 100)),
                                   brush_time_properties="hardness")), [8]),
    "FX-WRITEON-016": ("FX-WRITEON-014 with Hardness keyed too, from 0 to 100, Brush Time "
                       "Properties Size & Hardness: thin and soft to thick and hard.",
                       case(**dict(GROW, brush_hardness=keyed((0, 0), (8, 100)),
                                   brush_time_properties="size_and_hardness")), [8]),
    "FX-WRITEON-017": ("Brush Opacity keyed from 100 at frame 0 to 20 at frame 8, Paint Time "
                       "Properties None: the whole line fades together, to 20 at frame 8.",
                       case(**dict(SWEEP, brush_opacity=keyed((0, 100), (8, 20)))), [8]),
    "FX-WRITEON-018": ("FX-WRITEON-017 with Paint Time Properties Opacity: each mark keeps its "
                       "opacity, so the line fades from solid on the left to faint on the "
                       "right.", case(**dict(SWEEP, brush_opacity=keyed((0, 100), (8, 20)),
                                             paint_time_properties="opacity")), [8]),
    "FX-WRITEON-019": ("FX-WRITEON-017 with Paint Time Properties Color: the colour cannot be "
                       "keyed, so FX-WRITEON-017's frame.",
                       case(**dict(SWEEP, brush_opacity=keyed((0, 100), (8, 20)),
                                   paint_time_properties="color")), [8]),
    "FX-WRITEON-020": ("FX-WRITEON-017 with Paint Time Properties Color & Opacity: "
                       "FX-WRITEON-018's frame.",
                       case(**dict(SWEEP, brush_opacity=keyed((0, 100), (8, 20)),
                                   paint_time_properties="color_and_opacity")), [8]),
    "FX-WRITEON-021": ("The brush keyed through three places, 10, 20 at frame 0, 50, 80 at frame "
                       "4 and 90, 20 at frame 8: a V written on.",
                       case(**dict(SWEEP, brush_position=keyed((0, (10, 20)), (4, (50, 80)),
                                                               (8, (90, 20))))), [4, 8]),
    "FX-WRITEON-022": ("The brush held at 25, 50 until frame 4, then at 75, 50: two dots by "
                       "frame 8, no line between.",
                       case(**dict(SWEEP, brush_position=keyed((0, (25, 50), "hold"),
                                                               (4, (75, 50))))), [2, 8]),
    "FX-WRITEON-023": ("FX-WRITEON-002 with the layer moved three pixels right: the same, "
                       "moved; nothing grows.", case(3, **SWEEP), [8]),
    "FX-WRITEON-024": ("FX-WRITEON-002 on a layer that starts at frame 2: nothing before it, "
                       "and the first mark laid at frame 2, where the brush then is.",
                       case(**dict(SWEEP, **{"in": 2})), [1, 2, 8]),
    "FX-WRITEON-025": ("Brush Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                       "overshoots, Paint Time Properties Opacity: each mark's opacity held at "
                       "100 before it is used.",
                       case(**dict(SWEEP, brush_opacity=keyed((0, 0, OVERSHOOT), (4, 100)),
                                   paint_time_properties="opacity")), [4]),
    "FX-WRITEON-026": ("FX-WRITEON-003's spacing with Stroke Length 0.1 seconds: at frame 4 the "
                       "only mark is older than its length, so nothing is drawn; at frame 8 "
                       "the second mark, laid two frames before, is.",
                       case(**dict(SWEEP, brush_spacing=0.25, stroke_length=0.1)), [4, 8]),
    "FX-WRITEON-027": ("FX-WRITEON-002 on a layer stretched to 200 per cent: the keys and the "
                       "marks stretch with it, so frame 8 is FX-WRITEON-002's frame 4.",
                       case(**dict(SWEEP, stretch=200)), [8]),
}

INVALID = {
    "FX-WRITEON-028": ("Brush Position at 1001, 50, past ten widths.",
                       case(brush_position=(1001, 50))),
    "FX-WRITEON-029": ("Brush Size 201, above 200.", case(brush_size=201)),
    "FX-WRITEON-030": ("Brush Size -1, below 0.", case(brush_size=-1)),
    "FX-WRITEON-031": ("Brush Hardness 101, above 100.", case(brush_hardness=101)),
    "FX-WRITEON-032": ("Brush Opacity 101, above 100.", case(brush_opacity=101)),
    "FX-WRITEON-033": ("Stroke Length -1, below 0.", case(stroke_length=-1)),
    "FX-WRITEON-034": ("Stroke Length 3601, above 3600 seconds.", case(stroke_length=3601)),
    "FX-WRITEON-035": ("Brush Spacing 0, below 0.001 seconds.", case(brush_spacing=0)),
    "FX-WRITEON-036": ("Brush Spacing 11, above 10 seconds.", case(brush_spacing=11)),
    "FX-WRITEON-037": ("Paint Time Properties \"size\", not a word it takes.",
                       case(paint_time_properties="size")),
    "FX-WRITEON-038": ("Brush Time Properties \"color\", not a word it takes.",
                       case(brush_time_properties="color")),
    "FX-WRITEON-039": ("Paint Style \"glow\", not a word it takes.", case(paint_style="glow")),
    "FX-WRITEON-040": ("Colour \"#12345\", not #rrggbb.", case(color="#12345")),
    "FX-WRITEON-041": ("Brush Size keyed to 300 at frame 4.",
                       case(brush_size=keyed((0, 6), (4, 300)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    comp["duration_frames"] = FRAMES
    comp["work_area"] = {"start_frame": 0, "end_frame_exclusive": FRAMES}
    assert comp["frame_rate"] == {"numerator": FPS, "denominator": 1}
    layer = comp["layers"][0]
    layer["in_frame"], layer["out_frame"] = c["in"], FRAMES
    if c["stretch"] != 100:
        layer["time_stretch"] = c["stretch"]
    t = layer["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    layer["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.write_on", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_writeon.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    white = [1.0, 1.0, 1.0, 1.0]

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    # The marks.
    sweep = case(**SWEEP)
    assert len(marks(sweep, 0)) == 1 and len(marks(sweep, 8)) == 17  # 8 / 0.48 = 16.67
    assert near([m[0] for m in marks(case(**dict(SWEEP, brush_spacing=0.25)), 8)],
                [1.6, 1.6 + 6 * 1.6], 1e-9)
    assert len(marks(case(**dict(SWEEP, stroke_length=0.1)), 8)) == 5  # within 2.4 frames
    assert marks(case(**dict(SWEEP, brush_spacing=0.25, stroke_length=0.1)), 4) == []
    grow = marks(case(**dict(GROW, brush_time_properties="size")), 8)
    assert grow[0][2] == 1 and grow[-1][2] < 6 and all(a[2] < b[2] for a, b in zip(grow, grow[1:]))
    assert all(m[2] == 6 for m in marks(case(**GROW), 8))
    assert all(m[4] == 100 for m in marks(case(**dict(
        SWEEP, brush_opacity=keyed((0, 0, OVERSHOOT), (4, 100)),
        paint_time_properties="opacity")), 4)[6:])
    assert abs(marks(case(**dict(SWEEP, **{"in": 2})), 2)[0][0] - (0.1 * W + 2 * 1.6)) < 1e-9

    one = c["FX-WRITEON-001"]
    assert one["0"] == one["4"] != drawn and one["0"][at(8, 5)] == white
    two = c["FX-WRITEON-002"]
    assert two["0"] != two["2"] != two["4"] != two["8"]
    # Frame 8: the line reaches 14.4 pixels across; its middle row white from 2 to 13.
    assert all(two["8"][at(x, 4)] == white or two["8"][at(x, 5)] == white for x in range(2, 14))
    assert two["4"][at(13, 5)] == drawn[at(13, 5)]
    assert c["FX-WRITEON-004"]["8"][at(3, 5)] == drawn[at(3, 5)]
    assert c["FX-WRITEON-004"]["8"][at(13, 5)] != drawn[at(13, 5)]
    assert c["FX-WRITEON-011"]["4"] == drawn
    assert all(p == EMPTY for p in c["FX-WRITEON-012"]["4"])
    assert c["FX-WRITEON-013"]["2"] != c["FX-WRITEON-014"]["2"]
    assert c["FX-WRITEON-019"]["8"] == c["FX-WRITEON-017"]["8"]
    assert c["FX-WRITEON-020"]["8"] == c["FX-WRITEON-018"]["8"]
    assert c["FX-WRITEON-017"]["8"] != c["FX-WRITEON-018"]["8"]
    hold = c["FX-WRITEON-022"]["8"]
    assert hold[at(8, 5)] == drawn[at(8, 5)] and hold[at(4, 5)] != drawn[at(4, 5)]
    three = c["FX-WRITEON-023"]["8"]
    assert all(three[at(x + 3, y)] == two["8"][at(x, y)] for x in range(W - 3) for y in range(H))
    late = c["FX-WRITEON-024"]
    assert all(p == EMPTY for p in late["1"]) and late["2"] != drawn
    assert c["FX-WRITEON-026"]["4"] == drawn and c["FX-WRITEON-026"]["8"] != drawn
    assert all(near(p, q) for p, q in zip(c["FX-WRITEON-027"]["8"], two["4"]))
    print("checked")


if __name__ == "__main__":
    main()
