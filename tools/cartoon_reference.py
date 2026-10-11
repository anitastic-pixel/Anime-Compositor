"""Cartoon, worked a second way.

D-448 adds `core.cartoon`, after After Effects' Cartoon (Stylize): the picture simplified into
flat areas of colour, its shading cut into a few steps, with dark lines drawn along its edges.
After Effects' controls are Render (Fill, Edges, Fill & Edges), Detail Radius and Detail
Threshold (how the picture is smoothed first), Fill's Shading Steps and Shading Smoothness,
Edge's Threshold, Width, Softness and Opacity, and the advanced Edge Enhancement, Edge Black
Level and Edge Contrast. Adobe publishes no formula and no starting values; the rule and the
values it starts at are this program's own, and nothing is ported.

The rule, for a layer:

1. Smoothing. The layer goes through D-358's Bilateral Blur with Radius = Detail Radius,
   Threshold = Detail Threshold and Colorize on (`tools/bilateral_blur_reference.py`); call
   the result S. Every pixel keeps its covering.
2. Edges. Each pixel of S has its picture luma, its colour over black, encoded (D-146's, as
   Find Edges). Sobel's sums over the 3 by 3 round it, the border repeated, give gx and gy, and
   the edge measure g = 10 sqrt(gx^2 + gy^2) / 2 (a hard step from black to white gives 20).
   The pixel's edge E = clamp((g - Edge Threshold) / (10.01 - 10 Edge Contrast), 0, 1).
3. The line. With W = Edge Width and h = 0.5 + Edge Softness / 100 * W, a pixel's ink is
   Edge Opacity / 100 times the largest, over the pixels of S (itself among them) at distance
   d from it, of E there times clamp((W + 0.5 - d) / (2 h) + 0.5, 0, 1). Width 0 with
   Softness 0 is the edge pixel alone.
4. The fill, for a pixel of S that shows, its straight colour c: e = c through the sRGB curve
   for each of red, green and blue, and l = the luminance 0.2126 c_r + 0.7152 c_g + 0.0722 c_b
   through the curve (both held inside 0 to 1). With n = Shading Steps (its whole part) - 1 and
   s = Shading Smoothness / 100: q = l n, k = floor(q), t = q - k; the ramp is 1 if t >= 0.5
   and 0 if not when s is 0, else clamp((t - 0.5) / s + 0.5, 0, 1); l' = (k + ramp) / n. The
   fill is clamp(e + l' - l, 0, 1). So Shading Steps sets how many shades there are, and
   Smoothness 100 leaves the shading as it was.
5. Edge Enhancement, x = Edge Enhancement / 100, when not 0: the fill f moves to
   clamp(f + x E (f - m), 0, 1), m the mean of the fills of the 3 by 3 round the pixel that
   show (the border repeated). Above 0 an edge's two sides are pushed apart, sharper; below 0
   they are drawn together, softer.
6. The result, encoded, b = Edge Black Level: Fill & Edges, f (1 - ink) + b ink; Fill, f;
   Edges, (1 - b)(1 - ink) + b ink, the line on paper (black on white at 0, white on black at
   1). Back to linear, premultiplied by the pixel's own covering. A pixel that does not show
   stays empty; the layer never grows.

Settings: `render` `fill`, `edges` or `fill_and_edges` (when added); `detail_radius` 0 to 50
pixels, 8 when added; `detail_threshold` 0 to 255, 10; `shading_steps` 2 to 64, 8, its whole
part; `shading_smoothness` 0 to 100, 40; `edge_threshold` 0 to 10, 1.6; `edge_width` 0 to 10,
1.5; `edge_softness` 0 to 100, 60; `edge_opacity` 0 to 100, 100; `edge_enhancement` -100 to
100, 0; `edge_black_level` 0 to 1, 0; `edge_contrast` 0 to 1, 0.5. Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding D-203's skin with specks, a hole, a line two pixels
wide, a patch of grain and a column at half covering (`tools/median_smart_blur_reference.py`),
unmoved unless the case says. The expected frames are in
`Fixtures/cartoon/expected_cartoon.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cartoon_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
import median_smart_blur_reference as M  # noqa: E402
from bilateral_blur_reference import bilateral_blur  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cartoon"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"detail_radius": (0, 50), "detail_threshold": (0, 255), "shading_steps": (2, 64),
          "shading_smoothness": (0, 100), "edge_threshold": (0, 10), "edge_width": (0, 10),
          "edge_softness": (0, 100), "edge_opacity": (0, 100), "edge_enhancement": (-100, 100),
          "edge_black_level": (0, 1), "edge_contrast": (0, 1)}
NAMES = ("render",) + tuple(RANGES)
RENDERS = ("fill", "edges", "fill_and_edges")
LUMA = (0.2126, 0.7152, 0.0722)
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo=0.0, hi=1.0):
    return min(hi, max(lo, v))


def picture_luma(p):
    return S.linear_to_srgb(clamp(sum(k * v for k, v in zip(LUMA, p[:3]))))


def edges(px, w, h, threshold, contrast):
    lum = [picture_luma(p) for p in px]
    Y = lambda i, j: lum[min(max(j, 0), h - 1) * w + min(max(i, 0), w - 1)]  # noqa: E731
    out = []
    for y in range(h):
        for x in range(w):
            gx = ((Y(x + 1, y - 1) + 2 * Y(x + 1, y) + Y(x + 1, y + 1))
                  - (Y(x - 1, y - 1) + 2 * Y(x - 1, y) + Y(x - 1, y + 1)))
            gy = ((Y(x - 1, y + 1) + 2 * Y(x, y + 1) + Y(x + 1, y + 1))
                  - (Y(x - 1, y - 1) + 2 * Y(x, y - 1) + Y(x + 1, y - 1)))
            g = 10 * math.sqrt(gx * gx + gy * gy) / 2
            out.append(clamp((g - threshold) / (10.01 - 10 * contrast)))
    return out


def fill(p, n, s):
    """The fill of a pixel that shows, encoded."""
    a = p[3]
    c = [p[i] / a for i in range(3)]
    e = [S.linear_to_srgb(clamp(v)) for v in c]
    lum = S.linear_to_srgb(clamp(sum(k * v for k, v in zip(LUMA, c))))
    q = lum * n
    k = math.floor(q)
    t = q - k
    ramp = (1.0 if t >= 0.5 else 0.0) if s == 0 else clamp((t - 0.5) / s + 0.5)
    shade = (k + ramp) / n
    return [clamp(v + shade - lum) for v in e]


def cartoon(layer, v, render):
    w, h = layer["w"], layer["h"]
    sm = bilateral_blur(layer, v["detail_radius"], v["detail_threshold"], "on")["px"]
    E = edges(sm, w, h, v["edge_threshold"], v["edge_contrast"])
    n = math.floor(v["shading_steps"]) - 1
    s = v["shading_smoothness"] / 100
    fills = [None if p[3] <= 0 else fill(p, n, s) for p in sm]
    x_enh = v["edge_enhancement"] / 100
    width = v["edge_width"]
    soft = 0.5 + v["edge_softness"] / 100 * width
    reach = math.floor(width + 0.5 + soft) + 1
    opacity = v["edge_opacity"] / 100
    b = v["edge_black_level"]
    out = []
    for y in range(h):
        for x in range(w):
            i = y * w + x
            a = sm[i][3]
            if a <= 0:
                out.append(EMPTY)
                continue
            f = fills[i]
            if x_enh != 0:
                around = [fills[min(max(y + dy, 0), h - 1) * w + min(max(x + dx, 0), w - 1)]
                          for dy in (-1, 0, 1) for dx in (-1, 0, 1)]
                around = [g for g in around if g is not None]
                m = [sum(g[c] for g in around) / len(around) for c in range(3)]
                f = [clamp(f[c] + x_enh * E[i] * (f[c] - m[c])) for c in range(3)]
            ink = 0.0
            for dy in range(-reach, reach + 1):
                for dx in range(-reach, reach + 1):
                    tx, ty = x + dx, y + dy
                    if not (0 <= tx < w and 0 <= ty < h):
                        continue
                    wt = clamp((width + 0.5 - math.sqrt(dx * dx + dy * dy)) / (2 * soft) + 0.5)
                    ink = max(ink, E[ty * w + tx] * wt)
            ink *= opacity
            if render == "fill":
                c = f
            elif render == "edges":
                c = [(1 - b) * (1 - ink) + b * ink] * 3
            else:
                c = [f[k] * (1 - ink) + b * ink for k in range(3)]
            out.append([srgb_to_linear(c[k]) * a for k in range(3)] + [a])
    return dict(layer, px=out)


# --- the cases ------------------------------------------------------------------------------

def case(render="fill_and_edges", detail_radius=8, detail_threshold=10, shading_steps=8,
         shading_smoothness=40, edge_threshold=1.6, edge_width=1.5, edge_softness=60,
         edge_opacity=100, edge_enhancement=0, edge_black_level=0, edge_contrast=0.5, shift=0,
         tile=False):
    c = dict(locals())
    c["drawing"] = "specks"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def layer_of(c):
    layer = M.drawn_layer(c["drawing"])
    return motion_tile(layer, 300, 300, "off") if c["tile"] else layer


def render(c, frame_no):
    v = {k: held(c, k, frame_no) for k in RANGES}
    return frame(cartoon(layer_of(c), v, c["render"]), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


TOGETHER = dict(detail_radius=3, detail_threshold=30, shading_steps=5, shading_smoothness=20,
                edge_threshold=2.5, edge_width=2.5, edge_softness=30, edge_opacity=80,
                edge_enhancement=40, edge_black_level=0.2, edge_contrast=0.7)

CASES = {
    "FX-CARTOON-001": ("The settings as they start: Fill & Edges, Detail Radius 8, Threshold 10, "
                       "8 shading steps at smoothness 40, edges at threshold 1.6, width 1.5, "
                       "softness 60: the grain smoothed into the skin, the skin's shading cut "
                       "into steps, and dark lines along the line, the specks, the hole and the "
                       "drawing's outline.", case(), [0]),
    "FX-CARTOON-002": ("Render Fill: the smoothed, stepped colours, no lines.", case("fill"),
                       [0]),
    "FX-CARTOON-003": ("Render Edges: the lines alone, black on white, at the drawing's own "
                       "covering.", case("edges"), [0]),
    "FX-CARTOON-004": ("Detail Radius 0: nothing smoothed, so the grain keeps its edges.",
                       case(detail_radius=0), [0]),
    "FX-CARTOON-005": ("Detail Threshold 255: the smoothing hardly minds the colours, a soft "
                       "blur that softens the lines.", case(detail_threshold=255), [0]),
    "FX-CARTOON-006": ("Render Fill, 2 shading steps, smoothness 0: every pixel's shade "
                       "pushed to black or white.",
                       case("fill", shading_steps=2, shading_smoothness=0), [0]),
    "FX-CARTOON-007": ("Render Fill, Detail Radius 0, smoothness 100: the drawing as it was.",
                       case("fill", detail_radius=0, shading_smoothness=100), [0]),
    "FX-CARTOON-008": ("Edge Threshold 10: only the strongest edges are drawn.",
                       case(edge_threshold=10), [0]),
    "FX-CARTOON-009": ("Edge Width 0, softness 0: lines one pixel wide, on the edges alone.",
                       case(edge_width=0, edge_softness=0), [0]),
    "FX-CARTOON-010": ("Edge Width 4: thick lines.", case(edge_width=4), [0]),
    "FX-CARTOON-011": ("Edge Softness 0: the lines' sides hard.", case(edge_softness=0), [0]),
    "FX-CARTOON-012": ("Edge Opacity 0: no lines, the same as Render Fill.",
                       case(edge_opacity=0), [0]),
    "FX-CARTOON-013": ("Edge Opacity 50: the lines at half strength.", case(edge_opacity=50),
                       [0]),
    "FX-CARTOON-014": ("Render Fill, Edge Enhancement 100: the colours either side of an edge "
                       "pushed apart.", case("fill", edge_enhancement=100), [0]),
    "FX-CARTOON-015": ("Render Fill, Edge Enhancement -100: the colours either side of an edge "
                       "drawn together.", case("fill", edge_enhancement=-100), [0]),
    "FX-CARTOON-016": ("Edge Black Level 0.5: grey lines.", case(edge_black_level=0.5), [0]),
    "FX-CARTOON-017": ("Render Edges, Edge Black Level 1: white lines on black.",
                       case("edges", edge_black_level=1), [0]),
    "FX-CARTOON-018": ("Edge Contrast 1: every edge past the threshold drawn fully.",
                       case(edge_contrast=1), [0]),
    "FX-CARTOON-019": ("Edge Contrast 0: the lines fade in slowly with the edge.",
                       case(edge_contrast=0), [0]),
    "FX-CARTOON-020": ("Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at "
                       "frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at "
                       "frame 2.",
                       case(detail_radius=keyed((0, 0), (4, 16)),
                            edge_width=keyed((0, 0, OVERSHOOT), (4, 10))), [0, 2, 4]),
    "FX-CARTOON-021": ("FX-CARTOON-001 moved three pixels right.", case(shift=3), [0]),
    "FX-CARTOON-022": ("After a Motion Tile that grows the layer, unmirrored: the edges are the "
                       "grown buffer's, so the tile's empty row and column now meet the top row "
                       "and the left column, and lines run along the top and the left.",
                       case(tile=True), [0]),
    "FX-CARTOON-023": ("Detail Radius 3, Threshold 30, 5 steps at smoothness 20, edges at "
                       "threshold 2.5, width 2.5, softness 30, opacity 80, enhancement 40, black "
                       "level 0.2, contrast 0.7: the controls together.",
                       case(**TOGETHER), [0]),
}

INVALID = {
    "FX-CARTOON-024": ("Render \"outline\", not one of fill, edges or fill_and_edges.",
                       case("outline")),
    "FX-CARTOON-025": ("Detail Radius 51, above 50.", case(detail_radius=51)),
    "FX-CARTOON-026": ("Detail Threshold -1, below 0.", case(detail_threshold=-1)),
    "FX-CARTOON-027": ("Shading Steps 1, below 2.", case(shading_steps=1)),
    "FX-CARTOON-028": ("Shading Steps 65, above 64.", case(shading_steps=65)),
    "FX-CARTOON-029": ("Edge Width 11, above 10.", case(edge_width=11)),
    "FX-CARTOON-030": ("Edge Enhancement -101, below -100.", case(edge_enhancement=-101)),
    "FX-CARTOON-031": ("Edge Black Level 1.5, above 1.", case(edge_black_level=1.5)),
    "FX-CARTOON-032": ("Edge Contrast keyed to 2 at frame 4, above 1.",
                       case(edge_contrast=keyed((0, 0.5), (4, 2)))),
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
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "off"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.cartoon",
                    "enabled": True,
                    "parameters": {k: (c[k] if k == "render" else setting_json(c[k]))
                                   for k in NAMES}})
    comp["layers"][0]["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in M.DRAWINGS.items():
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
    (OUT / "expected_cartoon.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(f[i], g[i], e) for i in range(W * H))  # noqa: E731
    changed = lambda f, g: sum(not near(f[i], g[i], 1e-6) for i in range(W * H))  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    # How much ink: the sum, over pixels that show, of how far the Edges render is from white.
    inked = lambda px: sum(1 - enc(p)[0] for p in px if p[3] > 0)  # noqa: E731
    grain = [(x, y) for x in range(8) for y in range(5, 9) if M.MAP[y][x] in "+-"]

    one, fill_only, lines = c["FX-CARTOON-001"], c["FX-CARTOON-002"], c["FX-CARTOON-003"]
    # Every pixel keeps its covering; a pixel that does not show stays empty.
    for name, px in c.items():
        for i, p in enumerate(px):
            assert abs(p[3] - (plain(case(tile=name == "FX-CARTOON-022",
                                         shift=3 if name == "FX-CARTOON-021" else 0))[i][3])) < 1e-12
            assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    # The line columns are inked; the lines darken the fill.
    assert all(enc(lines[at(x, 4)])[0] < 0.2 for x in (8, 11))
    assert enc(lines[at(5, 5)])[0] > 0.8  # the smoothed grain, away from the edges, is near paper
    assert all(enc(one[at(x, 4)])[0] <= enc(fill_only[at(x, 4)])[0] + 1e-12 for x in range(W - 1))
    # The grain is smoothed: its spread in the fill is far smaller than with no smoothing.
    spread = lambda px: max(enc(px[at(x, y)])[2] for x, y in grain) - min(enc(px[at(x, y)])[2] for x, y in grain)  # noqa: E731,E501
    assert spread(fill_only) < spread(c["FX-CARTOON-004"]) / 2 or spread(fill_only) < 1e-9
    assert changed(c["FX-CARTOON-005"], one) > 10
    six = c["FX-CARTOON-006"]
    assert changed(six, fill_only) > 50
    assert same(c["FX-CARTOON-007"], drawn, 1e-12)
    assert inked(lines) > 0
    eight = c["FX-CARTOON-008"]
    assert changed(eight, one) > 0
    assert changed(c["FX-CARTOON-009"], one) > 0
    assert changed(c["FX-CARTOON-010"], one) > 10
    assert changed(c["FX-CARTOON-011"], one) > 0
    assert c["FX-CARTOON-012"] == fill_only
    half = c["FX-CARTOON-013"]
    assert all(enc(one[i])[0] - 1e-12 <= enc(half[i])[0] <= enc(fill_only[i])[0] + 1e-12
               for i in range(W * H) if half[i][3] > 0)
    fourteen, fifteen = c["FX-CARTOON-014"], c["FX-CARTOON-015"]
    assert changed(fourteen, fill_only) > 10 and changed(fifteen, fill_only) > 10
    # Beside the line, pushed apart the skin is lighter and the line darker; drawn together the
    # other way.
    assert enc(fourteen[at(11, 3)])[0] >= enc(fill_only[at(11, 3)])[0] - 1e-12
    assert enc(fifteen[at(11, 3)])[0] < enc(fill_only[at(11, 3)])[0]
    assert enc(fifteen[at(10, 3)])[0] > enc(fill_only[at(10, 3)])[0]
    assert changed(c["FX-CARTOON-016"], one) > 10
    white_on_black = c["FX-CARTOON-017"]
    assert all(near(enc(white_on_black[i]), [1 - v for v in enc(lines[i])], 1e-9)
               for i in range(W * H) if lines[i][3] > 0)
    assert changed(c["FX-CARTOON-018"], one) > 0 and changed(c["FX-CARTOON-019"], one) > 0
    twenty = expected["cases"]["FX-CARTOON-020"]["frames"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 10)), 2) > 10
    assert same(twenty["2"], render(case(edge_width=10), 0))
    assert changed(twenty["0"], twenty["4"]) > 0
    moved = c["FX-CARTOON-021"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    tiled = c["FX-CARTOON-022"]
    assert all(enc(tiled[at(x, 0)])[0] < enc(one[at(x, 0)])[0] for x in (5, 6))
    assert changed(c["FX-CARTOON-023"], one) > 50
    for fx in INVALID:
        assert expected["cases"][fx]["frames"]["0"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
