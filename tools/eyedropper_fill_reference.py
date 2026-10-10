"""Eyedropper Fill, worked a second way.

D-419 adds `core.eyedropper_fill`, After Effects' Eyedropper Fill (Generate): the layer filled with
one colour taken from a small area of the layer itself. Its settings are After Effects': Sample
Point, Sample Radius, Average Pixel Colors (Skip Empty, All, All Premultiplied, Including Alpha),
Maintain Original Alpha and Blend With Original. Adobe's help page could not be read here and
gives no formula we could find; the numbers below are this program's own rule and nothing is
ported.

The rule. The point is per cent of the drawing's own width and height, from its top-left corner,
however far an effect above grew the buffer: (px, py) = (point / 100 w, point / 100 h) in the
drawing's pixels. The area is every pixel whose centre lies within the radius r of the point, row
by row: for row y, dy = y + 0.5 - py and hw = sqrt(max(0, r r - dy dy)), the columns x with
ceil(px - 0.5 - hw) <= x <= floor(px - 0.5 + hw), over the rows ceil(py - 0.5 - r) to
floor(py - 0.5 + r). When that holds no pixel (a radius too small to reach a pixel's centre), the
area is the one pixel holding the point, (floor px, floor py). A pixel of the area outside the
buffer counts as clear. n is the area's pixels, wherever they lie.

The sums are of the buffer as it reaches the effect, premultiplied in linear light: P (red, green,
blue and alpha summed), and over the pixels with alpha above 0, their straight colours (P / a)
summed and counted (m). The sampled colour c and its covering A:

- skip_empty: c = straight sum / m (black when m is 0), A = 1;
- all: c = straight sum / n, A = 1 (an empty pixel counts as black);
- all_premultiplied: c = P.rgb / n, A = 1;
- including_alpha: A = P.a / n, c = P.rgb / P.a (black when P.a is 0).

Every pixel of the buffer becomes E = (c A, A), or with Maintain Original Alpha on
E = (c A a, A a), a the pixel's own alpha; then with b = Blend With Original / 100,
E (1 - b) + P b. The layer never grows; with Maintain Original Alpha off it is filled to its edges,
the empty parts too.

`sample_point` two numbers, -1000 to 1000 per cent, keyable, (50, 50) when added; `sample_radius`
0 to 10000 pixels, keyable, 0 when added; `average_pixel_colors` `skip_empty` (when added), `all`,
`all_premultiplied` or `including_alpha`; `maintain_original_alpha` `off` (when added) or `on`;
`blend_with_original` 0 to 100, keyable, 0 when added. The radius is a distance: a draft halves it.

**This file never runs the build's code path.** It walks the rows and sums pixel by pixel in
double precision on lists, straight from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/eyedropper_fill/expected_eyedropper_fill.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/eyedropper_fill_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame, pixel  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "eyedropper_fill"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"sample_point": (-1000, 1000), "sample_radius": (0, 10000),
          "blend_with_original": (0, 100)}
WORDS = ("average_pixel_colors", "maintain_original_alpha")
NAMES = ("sample_point", "sample_radius", "average_pixel_colors", "maintain_original_alpha",
         "blend_with_original")
AVERAGES = ("skip_empty", "all", "all_premultiplied", "including_alpha")
EDGE = (12.5, 50)   # (2, 5): the empty column, the soft edge, the line and the skin within 2


# --- the rule -------------------------------------------------------------------------------

def area(px, py, r):
    """The area's rows (y, first x, last x) in the drawing's pixels, and its pixel count."""
    rows, n = [], 0
    for y in range(math.ceil(py - 0.5 - r), math.floor(py - 0.5 + r) + 1):
        dy = y + 0.5 - py
        hw = math.sqrt(max(0.0, r * r - dy * dy))
        x0, x1 = math.ceil(px - 0.5 - hw), math.floor(px - 0.5 + hw)
        if x1 >= x0:
            rows.append((y, x0, x1))
            n += x1 - x0 + 1
    if n == 0:
        rows, n = [(math.floor(py), math.floor(px), math.floor(px))], 1
    return rows, n


def sampled(layer, point, r, average):
    """The colour c (linear, straight) and its covering A."""
    rows, n = area(point[0] / 100 * W, point[1] / 100 * H, r)
    p, s, m = [0.0] * 4, [0.0] * 3, 0
    for y, x0, x1 in rows:
        if not layer["top"] <= y < layer["top"] + layer["h"]:
            continue  # clear: counted in n, adds nothing
        for x in range(max(x0, layer["left"]), min(x1, layer["left"] + layer["w"] - 1) + 1):
            q = pixel(layer, x, y)
            p = [a + b for a, b in zip(p, q)]
            if q[3] > 0:
                s = [a + b / q[3] for a, b in zip(s, q[:3])]
                m += 1
    if average == "skip_empty":
        return [v / m if m else 0.0 for v in s], 1.0
    if average == "all":
        return [v / n for v in s], 1.0
    if average == "all_premultiplied":
        return [v / n for v in p[:3]], 1.0
    return [v / p[3] if p[3] > 0 else 0.0 for v in p[:3]], p[3] / n


def eyedropper_fill(layer, n, average, maintain):
    c, A = sampled(layer, n["sample_point"], n["sample_radius"], average)
    b = n["blend_with_original"] / 100
    px = []
    for p in layer["px"]:
        a = A * (p[3] if maintain == "on" else 1.0)
        e = [v * a for v in c] + [a]
        px.append([e[ch] * (1 - b) + p[ch] * b for ch in range(4)])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(sample_point=(50, 50), sample_radius=0, average_pixel_colors="skip_empty",
         maintain_original_alpha="off", blend_with_original=0, shift=0, tile=False):
    return {"drawing": "cel", "sample_point": sample_point, "sample_radius": sample_radius,
            "average_pixel_colors": average_pixel_colors,
            "maintain_original_alpha": maintain_original_alpha,
            "blend_with_original": blend_with_original, "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, x)) for x in v]
    return min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(eyedropper_fill(layer_of(c), n, c["average_pixel_colors"],
                                 c["maintain_original_alpha"]), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-EYEFILL-001": ("The settings as they start: the point in the middle, (8, 5), radius 0, "
                       "Skip Empty: the skin under the point fills the whole layer, its line, "
                       "shadow and empty border alike, all opaque.", case(), [0]),
    "FX-EYEFILL-002": ("The point at (2, 5), radius 2, Skip Empty: twelve pixels, two of them "
                       "empty and left out, the soft edge's line counted at its full colour; a "
                       "dark colour, mostly the line's, a little skin.",
                       case(EDGE, 2), [0]),
    "FX-EYEFILL-003": ("The same area, All: the two empty pixels count as black, so a little "
                       "darker than FX-EYEFILL-002.", case(EDGE, 2, "all"), [0]),
    "FX-EYEFILL-004": ("The same area, All Premultiplied: the soft edge counts at half, so darker "
                       "again, still opaque.", case(EDGE, 2, "all_premultiplied"), [0]),
    "FX-EYEFILL-005": ("The same area, Including Alpha: the area's covering, about two "
                       "thirds, becomes the layer's; the colour that of FX-EYEFILL-004 "
                       "divided by it.", case(EDGE, 2, "including_alpha"), [0]),
    "FX-EYEFILL-006": ("FX-EYEFILL-005 with Maintain Original Alpha: the covering also times each "
                       "pixel's own, so the border stays empty and the soft edge half again.",
                       case(EDGE, 2, "including_alpha", "on"), [0]),
    "FX-EYEFILL-007": ("FX-EYEFILL-002 with Maintain Original Alpha: the colour opaque inside the "
                       "cel, half covering on its soft edge, the border empty.",
                       case(EDGE, 2, "skip_empty", "on"), [0]),
    "FX-EYEFILL-008": ("The settings as they start, Blend With Original 50: half way from the "
                       "skin to the cel, the empty border half covered with skin.",
                       case(blend_with_original=50), [0]),
    "FX-EYEFILL-009": ("Blend With Original 100: the cel as it was.",
                       case(blend_with_original=100), [0]),
    "FX-EYEFILL-010": ("The point on the edge between skin and shadow, (10, 5), radius 1.5: four "
                       "pixels, two each, an even mix of the two.",
                       case((62.5, 50), 1.5), [0]),
    "FX-EYEFILL-011": ("Radius 1000, Skip Empty: every pixel that shows, each at its full "
                       "colour, averaged; the layer's empty pixels and the plane past it left "
                       "out.", case(sample_radius=1000), [0]),
    "FX-EYEFILL-012": ("Radius 1000, All: the disc's three million pixels counted, nearly all "
                       "past the layer and black, so the fill is all but black.",
                       case(sample_radius=1000, average_pixel_colors="all"), [0]),
    "FX-EYEFILL-013": ("The point past the layer's left edge, (-50, 50), Skip Empty: nothing "
                       "shows there, so black, opaque.", case((-50, 50)), [0]),
    "FX-EYEFILL-014": ("The same point, Including Alpha: nothing shows, so the layer is empty.",
                       case((-50, 50), average_pixel_colors="including_alpha"), [0]),
    "FX-EYEFILL-015": ("A radius of 0.4 with the point at (8.5, 5.5), a pixel's centre: that "
                       "pixel alone, as radius 0, FX-EYEFILL-001's frame.",
                       case((53.125, 55), 0.4), [0]),
    "FX-EYEFILL-016": ("The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: skin at "
                       "frames 0 and 2, shadow at frame 4.",
                       case(keyed((0, (25, 50)), (4, (75, 50)))), [0, 2, 4]),
    "FX-EYEFILL-017": ("The radius keyed from 0 to 4 at the edge point: the colour changes as "
                       "the area grows.", case(EDGE, keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-EYEFILL-018": ("Blend With Original keyed from 0 to 100: frame 0 FX-EYEFILL-001's, frame "
                       "2 FX-EYEFILL-008's, frame 4 the cel.",
                       case(blend_with_original=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-EYEFILL-019": ("FX-EYEFILL-001 moved three pixels right: the layer's own pixels move, the "
                       "three columns it left are empty.", case(shift=3), [0]),
    "FX-EYEFILL-020": ("After a Motion Tile that grows the layer: the point is the drawing's own, "
                       "so the frame is FX-EYEFILL-010's.",
                       case((62.5, 50), 1.5, tile=True), [0]),
    "FX-EYEFILL-021": ("Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an "
                       "ease that passes its end: held at 100 at frame 2, the cel.",
                       case(blend_with_original=keyed((0, 40, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-EYEFILL-022": ("Including Alpha, Maintain Original Alpha and Blend With Original 50 "
                       "together at the edge point, radius 2.",
                       case(EDGE, 2, "including_alpha", "on", 50), [0]),
    "FX-EYEFILL-023": ("All Premultiplied, Maintain Original Alpha, radius 0 on the soft edge at "
                       "(1.5, 5.5): the line at half its colour, laid on each pixel's own covering.",
                       case((9.375, 55), 0, "all_premultiplied", "on"), [0]),
}

INVALID = {
    "FX-EYEFILL-024": ("Sample Point across 1001, above 1000.", case((1001, 50))),
    "FX-EYEFILL-025": ("Sample Point down -1001, below -1000.", case((50, -1001))),
    "FX-EYEFILL-026": ("Sample Radius -1, below 0.", case(sample_radius=-1)),
    "FX-EYEFILL-027": ("Sample Radius 10001, above 10000.", case(sample_radius=10001)),
    "FX-EYEFILL-028": ("Blend With Original -1, below 0.", case(blend_with_original=-1)),
    "FX-EYEFILL-029": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-EYEFILL-030": ("Average Pixel Colors \"sum\", not one of the four.",
                       case(average_pixel_colors="sum")),
    "FX-EYEFILL-031": ("Maintain Original Alpha \"yes\", not off or on.",
                       case(maintain_original_alpha="yes")),
    "FX-EYEFILL-032": ("Blend With Original keyed to 101 at frame 4, above 100.",
                       case(blend_with_original=keyed((0, 0), (4, 101)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    layer = comp["layers"][0]
    t = layer["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.eyedropper_fill",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    layer["effects"] = effects
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
    (OUT / "expected_eyedropper_fill.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked, from pixels named by hand."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    every = lambda px, q: all(near(p, q) for p in px)  # noqa: E731
    straight = lambda q: [v / q[3] for v in q[:3]]  # noqa: E731
    skin, shade, line, soft = art[at(8, 5)], art[at(12, 5)], art[at(2, 5)], art[at(1, 5)]
    assert skin[3] == shade[3] == line[3] == 1 and 0.4 < soft[3] < 0.6 and art[0] == [0.0] * 4

    one = c["FX-EYEFILL-001"]["0"]
    assert every(one, skin)
    # The edge area by hand: rows 3 and 6 reach columns 1 and 2, rows 4 and 5 columns 0 to 3.
    pixels = [soft] * 4 + [line] * 4 + [[0.0] * 4] * 2 + [skin] * 2
    n, shows = 12, [q for q in pixels if q[3] > 0]
    s = [sum(straight(q)[ch] for q in shows) for ch in range(3)]
    psum = [sum(q[ch] for q in pixels) for ch in range(4)]
    assert len(shows) == 10
    assert every(c["FX-EYEFILL-002"]["0"], [v / 10 for v in s] + [1.0])
    assert every(c["FX-EYEFILL-003"]["0"], [v / n for v in s] + [1.0])
    assert every(c["FX-EYEFILL-004"]["0"], [v / n for v in psum[:3]] + [1.0])
    A = psum[3] / n
    assert 0.6 < A < 0.7
    assert every(c["FX-EYEFILL-005"]["0"], [v / n for v in psum[:3]] + [A])
    six = c["FX-EYEFILL-006"]["0"]
    for p, q in zip(six, art):
        assert near(p, [v / n * q[3] for v in psum[:3]] + [A * q[3]])
    seven = c["FX-EYEFILL-007"]["0"]
    for p, q in zip(seven, art):
        assert near(p, [v / 10 * q[3] for v in s] + [q[3]])
    two, three, four = (c[f"FX-EYEFILL-00{i}"]["0"][0] for i in (2, 3, 4))
    assert two[0] > three[0] > four[0]
    eight = c["FX-EYEFILL-008"]["0"]
    for p, q in zip(eight, art):
        assert near(p, [(a + b) / 2 for a, b in zip(skin, q)])
    assert c["FX-EYEFILL-009"]["0"] == art
    assert every(c["FX-EYEFILL-010"]["0"], [(a + b) / 2 for a, b in zip(skin[:3], shade[:3])] + [1])
    cel = [q for q in art if q[3] > 0]
    assert every(c["FX-EYEFILL-011"]["0"],
                 [sum(straight(q)[ch] for q in cel) / len(cel) for ch in range(3)] + [1.0])
    twelve = c["FX-EYEFILL-012"]["0"][0]
    assert twelve[3] == 1.0 and max(twelve[:3]) < 1e-4
    assert every(c["FX-EYEFILL-013"]["0"], [0.0, 0.0, 0.0, 1.0])
    assert every(c["FX-EYEFILL-014"]["0"], [0.0] * 4)
    assert c["FX-EYEFILL-015"]["0"] == one
    moving = c["FX-EYEFILL-016"]
    assert moving["0"] == moving["2"] == one and every(moving["4"], shade)
    growing = c["FX-EYEFILL-017"]
    assert len({json.dumps(f) for f in growing.values()}) == 3
    blending = c["FX-EYEFILL-018"]
    assert blending["0"] == one and blending["2"] == eight and blending["4"] == art
    moved = c["FX-EYEFILL-019"]["0"]
    assert all(moved[at(x, y)] == (skin if x >= 3 else [0.0] * 4)
               for x in range(W) for y in range(H))
    assert c["FX-EYEFILL-020"]["0"] == c["FX-EYEFILL-010"]["0"]
    eased = c["FX-EYEFILL-021"]
    assert eased["0"] != art and eased["2"] == art and eased["4"] == art
    mixed = c["FX-EYEFILL-022"]["0"]
    for p, q, r in zip(mixed, six, art):
        assert near(p, [(a + b) / 2 for a, b in zip(q, r)])
    half = c["FX-EYEFILL-023"]["0"]
    for p, q in zip(half, art):
        assert near(p, [v * q[3] for v in soft[:3]] + [q[3]])
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
