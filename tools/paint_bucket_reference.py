"""Paint Bucket, worked a second way.

D-440 adds `core.paint_bucket`, After Effects' Paint Bucket (Generate): "a nondestructive paint
effect that fills an area with a solid color", the area holding the Fill Point, found by
"analyzing the neighboring pixels and expanding the fill area by adding pixels that match"
(Adobe's After Effects manual, the Paint Bucket effect). Its settings are After Effects': Fill
Point, Fill Selector (Color & Alpha, Straight Color, Transparency, Opacity, Alpha Channel),
Tolerance, View Threshold, Stroke (Antialias, Feather, Spread, Choke, Stroke), Invert Fill, Spread
Radius, Stroke Width, Feather Softness, Color, Opacity and Blending Mode (with Fill Only). Adobe
gives no formula; the numbers below are this program's own rule and nothing is ported.

The rule.

1. The point is per cent of the drawing's own width and height, from its top-left corner, however
   far an effect above grew the buffer: (px, py) = (point / 100 w, point / 100 h) in the drawing's
   pixels, and the pixel holding it is (floor px, floor py) in the buffer. A point past the buffer
   holds no pixel, and then no pixel matches.
2. Each pixel's alpha a and its straight colour encoded (sRGB, each held to 0..1; 0 where a is
   0), e. With t = Tolerance / 100 and the point pixel's own (E, A), a pixel matches when:
   - color_and_alpha: the largest of |e a - E A| over red, green and blue and |a - A| is at most t;
   - straight_color: the largest |e - E| is at most t;
   - transparency: a is at most t;
   - opacity: a is at least 1 - t;
   - alpha_channel: |a - A| is at most t.
3. The area: for alpha_channel every pixel that matches, anywhere; for the others the pixels that
   match joined to the point pixel by matching pixels, up, down, left and right (none when the
   point pixel does not match itself: Transparency's point must be in a clear place, Opacity's in
   an opaque one). Invert Fill turns it over: every pixel of the buffer not in it.
4. View Threshold on: every pixel opaque, white where it matches (step 2, anywhere, before Invert
   Fill), black elsewhere; nothing else is done.
5. The Stroke, with distances between pixel centres and every place past the buffer outside the
   area:
   - spread: the area and every pixel within Spread Radius of it;
   - choke: the area's pixels farther than Spread Radius from every pixel outside it;
   - stroke: the area's pixels within Stroke Width of a pixel outside it;
   - antialias and feather: the area as it is.
   The covering m is that area (1 in it, 0 out) blurred across, then down, its edges held (each
   place past the buffer reads the nearest edge pixel): for feather, document 21's Gaussian with
   sigma Feather Softness / 2, cut at ceil(3 sigma) and normalised (0 leaves it hard); for every
   other choice, a box three pixels wide, each tap a third.
6. k = m * Opacity / 100, times the pixel's own alpha for straight_color (Straight Color fills only
   where the layer shows). The fill is (C k, k), C the colour in linear light, laid on the pixel
   as a layer's blending mode lays a layer (document 21; Checkerboard's laying, D-413):
   normal, add, multiply, screen, overlay or soft_light; fill_only is the fill alone, the layer
   gone.

`fill_point` two numbers, -1000 to 1000 per cent, keyable, (50, 50) when added; `fill_selector`
`color_and_alpha` (when added), `straight_color`, `transparency`, `opacity` or `alpha_channel`;
`tolerance` 0 to 100, keyable, 10 when added; `view_threshold` `off` (when added) or `on`;
`stroke` `antialias` (when added), `feather`, `spread`, `choke` or `stroke`; `invert_fill` `off`
(when added) or `on`; `spread_radius` 0 to 10000 pixels, keyable, 3 when added; `stroke_width`
0 to 10000 pixels, keyable, 3 when added; `feather_softness` 0 to 10000 pixels, keyable, 10 when
added; `color` `#rrggbb`, red when added, read in small letters; `opacity` 0 to 100, keyable, 100
when added; `blending_mode` `normal` (when added), `add`, `multiply`, `screen`, `overlay`,
`soft_light` or `fill_only`. The radius, width and softness are distances: a draft halves them.
The layer never grows.

**This file never runs the build's code path.** It walks the area pixel by pixel, measures every
distance by trying every pixel, and blurs tap by tap in double precision on lists, straight from
the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/paint_bucket/expected_paint_bucket.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/paint_bucket_reference.py
"""

import json
import math
import sys
from collections import deque
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "paint_bucket"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"fill_point": (-1000, 1000), "tolerance": (0, 100), "spread_radius": (0, 10000),
          "stroke_width": (0, 10000), "feather_softness": (0, 10000), "opacity": (0, 100)}
WORDS = ("fill_selector", "view_threshold", "stroke", "invert_fill", "color", "blending_mode")
NAMES = ("fill_point", "fill_selector", "tolerance", "view_threshold", "stroke", "invert_fill",
         "spread_radius", "stroke_width", "feather_softness", "color", "opacity",
         "blending_mode")
RED = "#ff0000"
BLUE = "#3080ff"
SKIN_AT = (53.125, 55)    # the centre of pixel (8, 5), skin
LINE_AT = (15.625, 55)    # (2, 5), the line
EMPTY_AT = (3.125, 5)     # (0, 0), the empty border
# Every distance step 2 compares, against every tolerance a case uses, must be at least this far
# from it, so the build's single precision cannot turn a pixel over.
MARGIN = 1e-4


# --- the rule -------------------------------------------------------------------------------

def values(p):
    """(e, a): the straight colour encoded, each held to 0..1, and the alpha."""
    a = p[3]
    if a <= 0:
        return [0.0, 0.0, 0.0], 0.0
    return [K.to_srgb(v / a) for v in p[:3]], a


def distance(selector, p, q):
    """How far the pixel p is from the point pixel q, as step 2 compares it with t (for
    transparency and opacity, q plays no part)."""
    (e, a), (E, A) = values(p), values(q)
    if selector == "color_and_alpha":
        return max([abs(e[c] * a - E[c] * A) for c in range(3)] + [abs(a - A)])
    if selector == "straight_color":
        return max(abs(e[c] - E[c]) for c in range(3))
    if selector == "transparency":
        return a
    if selector == "opacity":
        return 1 - a
    return abs(a - A)


SEEN = []  # every (distance, t) step 2 compared, for the margin check


def matches(layer, n, selector):
    """Step 1 and 2: which buffer pixels match, or None when the point holds no pixel."""
    w, h = layer["w"], layer["h"]
    i = math.floor(n["fill_point"][0] / 100 * W - layer["left"])
    j = math.floor(n["fill_point"][1] / 100 * H - layer["top"])
    if not (0 <= i < w and 0 <= j < h):
        return None, None
    t = n["tolerance"] / 100
    q = layer["px"][j * w + i]
    out = []
    for p in layer["px"]:
        d = distance(selector, p, q)
        SEEN.append((d, t))
        out.append(d <= t)
    return out, (i, j)


def area(layer, n, selector, invert):
    """Step 3: 1 in the area, 0 out."""
    w, h = layer["w"], layer["h"]
    ok, seed = matches(layer, n, selector)
    if ok is None:
        got = [False] * (w * h)
    elif selector == "alpha_channel":
        got = list(ok)
    else:
        got = [False] * (w * h)
        i, j = seed
        if ok[j * w + i]:
            got[j * w + i] = True
            todo = deque([(i, j)])
            while todo:
                x, y = todo.popleft()
                for u, v in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
                    if 0 <= u < w and 0 <= v < h and ok[v * w + u] and not got[v * w + u]:
                        got[v * w + u] = True
                        todo.append((u, v))
    if invert == "on":
        got = [not g for g in got]
    return got


def nearest(w, h, inside, want):
    """For every pixel, its distance to the nearest pixel whose `inside` is `want`, counting the
    places past the buffer as outside the area (so as `want` False); by trying them all."""
    spots = [(k % w, k // w) for k in range(w * h) if inside[k] == want]
    out = []
    for k in range(w * h):
        x, y = k % w, k // w
        best = math.inf
        for u, v in spots:
            best = min(best, math.hypot(x - u, y - v))
        if not want:
            best = min(best, x + 1, y + 1, w - x, h - y)  # the nearest place past the buffer
        out.append(best)
    return out


def stroked(layer, got, stroke, n):
    """Step 5's area."""
    w, h = layer["w"], layer["h"]
    if stroke == "spread":
        d = nearest(w, h, got, True)
        return [g or d[k] <= n["spread_radius"] for k, g in enumerate(got)]
    if stroke == "choke":
        d = nearest(w, h, got, False)
        return [g and d[k] > n["spread_radius"] for k, g in enumerate(got)]
    if stroke == "stroke":
        d = nearest(w, h, got, False)
        return [g and d[k] <= n["stroke_width"] for k, g in enumerate(got)]
    return got


def weights(softness):
    sigma = softness / 2
    if sigma <= 0:
        return [1.0]
    r = math.ceil(3 * sigma)
    w = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    s = sum(w)
    return [v / s for v in w]


def blurred(w, h, m, taps):
    """Across then down, each place past the buffer reading the nearest edge pixel."""
    r = len(taps) // 2
    across = [sum(taps[i] * m[y * w + min(max(x + i - r, 0), w - 1)] for i in range(len(taps)))
              for y in range(h) for x in range(w)]
    return [sum(taps[i] * across[min(max(y + i - r, 0), h - 1) * w + x] for i in range(len(taps)))
            for y in range(h) for x in range(w)]


def covering(layer, n, c):
    got = stroked(layer, area(layer, n, c["fill_selector"], c["invert_fill"]), c["stroke"], n)
    m = [1.0 if g else 0.0 for g in got]
    taps = weights(n["feather_softness"]) if c["stroke"] == "feather" else [1 / 3] * 3
    return blurred(layer["w"], layer["h"], m, taps)


def paint_bucket(layer, n, c):
    if c["view_threshold"] == "on":
        ok, _ = matches(layer, n, c["fill_selector"])
        ok = ok or [False] * (layer["w"] * layer["h"])
        return dict(layer, px=[[1.0] * 4 if g else [0.0, 0.0, 0.0, 1.0] for g in ok])
    m = covering(layer, n, c)
    C = [srgb_to_linear(v / 255) for v in R.hex_color(c["color"].lower())]
    o = n["opacity"] / 100
    mode = "none" if c["blending_mode"] == "fill_only" else c["blending_mode"]
    px = []
    for k, p in enumerate(layer["px"]):
        cov = m[k] * (p[3] if c["fill_selector"] == "straight_color" else 1.0)
        px.append(K.lay(p, cov, C, o, mode))
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(fill_point=(50, 50), fill_selector="color_and_alpha", tolerance=10, view_threshold="off",
         stroke="antialias", invert_fill="off", spread_radius=3, stroke_width=3,
         feather_softness=10, color=RED, opacity=100, blending_mode="normal", shift=0,
         tile=False):
    return {"drawing": "cel", "fill_point": fill_point, "fill_selector": fill_selector,
            "tolerance": tolerance, "view_threshold": view_threshold, "stroke": stroke,
            "invert_fill": invert_fill, "spread_radius": spread_radius,
            "stroke_width": stroke_width, "feather_softness": feather_softness, "color": color,
            "opacity": opacity, "blending_mode": blending_mode, "shift": shift, "tile": tile}


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
    return frame(paint_bucket(layer_of(c), n, c), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-PAINTBUCKET-001": ("The settings as they start: the point in the middle on the skin, Color "
                           "& Alpha, tolerance 10, Antialias, red, opacity 100, normal: the skin "
                           "inside the line, columns 3 to 9 and rows 2 to 7, turns red, its edge "
                           "softened a pixel into the line and the shadow.", case(), [0]),
    "FX-PAINTBUCKET-002": ("Tolerance 25: the shadow, 54 levels from the skin at most, matches "
                           "too, so the whole inside of the line turns red.",
                           case(tolerance=25), [0]),
    "FX-PAINTBUCKET-003": ("The point on the line, Color & Alpha: the line's box turns red; its "
                           "half-covered left edge differs by half in alpha and stays.",
                           case(LINE_AT), [0]),
    "FX-PAINTBUCKET-004": ("The same point, Straight Color: the soft edge has the line's own "
                           "colour, so it is filled too, red only as far as it shows.",
                           case(LINE_AT, "straight_color"), [0]),
    "FX-PAINTBUCKET-005": ("Transparency with the point in the empty corner: the empty border all "
                           "round the cel turns red and opaque.",
                           case(EMPTY_AT, "transparency"), [0]),
    "FX-PAINTBUCKET-006": ("Transparency with the point on the skin: the point must be in a clear "
                           "place, so nothing is filled: the cel as it was.",
                           case(fill_selector="transparency"), [0]),
    "FX-PAINTBUCKET-007": ("Opacity with the point on the skin: every opaque pixel joined to it, "
                           "line, skin and shadow, turns red; the soft edge stays.",
                           case(fill_selector="opacity"), [0]),
    "FX-PAINTBUCKET-008": ("Alpha Channel with the point on the skin: every pixel as opaque as the "
                           "skin, the same as FX-PAINTBUCKET-007 here.",
                           case(fill_selector="alpha_channel"), [0]),
    "FX-PAINTBUCKET-009": ("Invert Fill: everything but the skin turns red, the empty border "
                           "included.", case(invert_fill="on"), [0]),
    "FX-PAINTBUCKET-010": ("View Threshold: the skin white, everything else black, all opaque.",
                           case(view_threshold="on"), [0]),
    "FX-PAINTBUCKET-011": ("View Threshold at tolerance 25: skin and shadow white.",
                           case(tolerance=25, view_threshold="on"), [0]),
    "FX-PAINTBUCKET-012": ("Feather, softness 2: the red skin's edge fades over a few pixels.",
                           case(stroke="feather", feather_softness=2), [0]),
    "FX-PAINTBUCKET-013": ("Spread, radius 1: the red reaches a pixel further, over the line and "
                           "into the shadow, then is softened.",
                           case(stroke="spread", spread_radius=1), [0]),
    "FX-PAINTBUCKET-014": ("Choke, radius 1: the red pulls a pixel in from the line and the "
                           "shadow.", case(stroke="choke", spread_radius=1), [0]),
    "FX-PAINTBUCKET-015": ("Stroke, width 1: only the skin's outer ring of pixels turns red.",
                           case(stroke="stroke", stroke_width=1), [0]),
    "FX-PAINTBUCKET-016": ("Spread radius 0: nothing added, FX-PAINTBUCKET-001's frame.",
                           case(stroke="spread", spread_radius=0), [0]),
    "FX-PAINTBUCKET-017": ("Fill Only: the red fill alone, the rest of the layer clear.",
                           case(blending_mode="fill_only"), [0]),
    "FX-PAINTBUCKET-018": ("Blue, multiply: the skin darkened toward blue.",
                           case(color=BLUE, blending_mode="multiply"), [0]),
    "FX-PAINTBUCKET-019": ("Blue, screen: the skin lightened toward blue.",
                           case(color=BLUE, blending_mode="screen"), [0]),
    "FX-PAINTBUCKET-020": ("Opacity 50: half way from the skin to red.", case(opacity=50), [0]),
    "FX-PAINTBUCKET-021": ("Opacity 0: the cel exactly as it was.", case(opacity=0), [0]),
    "FX-PAINTBUCKET-022": ("The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: the "
                           "skin at frames 0 and 2, the shadow at frame 4.",
                           case(keyed((0, (25, 50)), (4, (75, 50)))), [0, 2, 4]),
    "FX-PAINTBUCKET-023": ("Tolerance keyed from 10 to 30: skin only at frames 0 and 2 (20), skin "
                           "and shadow at frame 4.",
                           case(tolerance=keyed((0, 10), (4, 30))), [0, 2, 4]),
    "FX-PAINTBUCKET-024": ("Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that "
                           "passes its end: held at 100 at frame 2, FX-PAINTBUCKET-001's frame.",
                           case(opacity=keyed((0, 40, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-PAINTBUCKET-025": ("FX-PAINTBUCKET-001 moved three pixels right: the layer's own pixels "
                           "move, the three columns it left are empty.", case(shift=3), [0]),
    "FX-PAINTBUCKET-026": ("After a Motion Tile that grows the layer: the point is the drawing's "
                           "own, and the skin is closed in by its line, so the frame is "
                           "FX-PAINTBUCKET-001's.", case(tile=True), [0]),
    "FX-PAINTBUCKET-027": ("The point past the layer's left edge, (-50, 50), with Invert Fill: no "
                           "pixel matches, so the whole layer turns red and opaque.",
                           case((-50, 50), invert_fill="on"), [0]),
    "FX-PAINTBUCKET-028": ("Straight Color on the line with Fill Only: the fill alone, the soft "
                           "edge at half covering.",
                           case(LINE_AT, "straight_color", blending_mode="fill_only"), [0]),
    "FX-PAINTBUCKET-029": ("Choke, radius 2: only the skin's middle, columns 5 to 7 and rows 4 "
                           "and 5, is filled, then softened.",
                           case(stroke="choke", spread_radius=2), [0]),
    "FX-PAINTBUCKET-030": ("Blue, add: the skin lifted by blue, held at 1.",
                           case(color=BLUE, blending_mode="add"), [0]),
    "FX-PAINTBUCKET-031": ("Transparency in the empty corner at tolerance 60: the soft edge, half "
                           "covered, matches too.", case(EMPTY_AT, "transparency", 60), [0]),
}

INVALID = {
    "FX-PAINTBUCKET-032": ("Fill Point across 1001, above 1000.", case((1001, 50))),
    "FX-PAINTBUCKET-033": ("Tolerance -1, below 0.", case(tolerance=-1)),
    "FX-PAINTBUCKET-034": ("Tolerance 101, above 100.", case(tolerance=101)),
    "FX-PAINTBUCKET-035": ("Spread Radius 10001, above 10000.", case(spread_radius=10001)),
    "FX-PAINTBUCKET-036": ("Stroke Width -1, below 0.", case(stroke_width=-1)),
    "FX-PAINTBUCKET-037": ("Feather Softness 10001, above 10000.", case(feather_softness=10001)),
    "FX-PAINTBUCKET-038": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-PAINTBUCKET-039": ("Fill Selector \"luma\", not one of the five.",
                           case(fill_selector="luma")),
    "FX-PAINTBUCKET-040": ("Stroke \"glow\", not one of the five.", case(stroke="glow")),
    "FX-PAINTBUCKET-041": ("View Threshold \"yes\", not off or on.", case(view_threshold="yes")),
    "FX-PAINTBUCKET-042": ("Invert Fill \"yes\", not off or on.", case(invert_fill="yes")),
    "FX-PAINTBUCKET-043": ("Blending mode \"darken\", which this program does not have.",
                           case(blending_mode="darken")),
    "FX-PAINTBUCKET-044": ("Colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-PAINTBUCKET-045": ("Tolerance keyed to 101 at frame 4, above 100.",
                           case(tolerance=keyed((0, 10), (4, 101)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.paint_bucket",
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
    (OUT / "expected_paint_bucket.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked, from pixels named by hand."""
    close = min(abs(d - t) for d, t in SEEN)
    assert close >= MARGIN, close
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    red = [1.0, 0.0, 0.0, 1.0]
    clear = [0.0] * 4
    skin = lambda x, y: 3 <= x <= 9 and 2 <= y <= 7  # noqa: E731
    inside = lambda x, y: 3 <= x <= 13 and 2 <= y <= 7  # noqa: E731
    one = c["FX-PAINTBUCKET-001"]["0"]
    # The middle of the skin, a pixel in from its edge, is wholly red; far from it, untouched.
    for y in range(H):
        for x in range(W):
            if 4 <= x <= 8 and 3 <= y <= 6:
                assert near(one[at(x, y)], red)
            elif not (2 <= x <= 10 and 1 <= y <= 8):
                assert one[at(x, y)] == art[at(x, y)]
    # The corner of the skin: the box sees 4 of its 9 in the skin; the line beside its edge, 3.
    assert abs(one[at(3, 2)][1] - art[at(3, 2)][1] * 5 / 9) < 1e-9
    assert near(one[at(2, 5)][:3], [1 / 3 + art[at(2, 5)][0] * 2 / 3,
                                    art[at(2, 5)][1] * 2 / 3, art[at(2, 5)][2] * 2 / 3])
    two = c["FX-PAINTBUCKET-002"]["0"]
    assert all(near(two[at(x, y)], red) for x in range(4, 13) for y in range(3, 7))
    laid = lambda x, y, k: K.lay(art[at(x, y)], k, red[:3], 1.0, "normal")  # noqa: E731
    three = c["FX-PAINTBUCKET-003"]["0"]
    # The line is a pixel wide: above its corner the box finds two of its pixels in row 1 (the
    # row past the top read again as row 0, empty); on its top edge, a row of three.
    assert near(three[at(2, 0)], [2 / 9, 0, 0, 2 / 9]) and near(three[at(8, 1)], laid(8, 1, 1 / 3))
    assert near(three[at(1, 5)], K.lay(art[at(1, 5)], 1 / 3, red[:3], 1.0, "normal"))
    four = c["FX-PAINTBUCKET-004"]["0"]
    assert four[at(1, 5)] != three[at(1, 5)]
    five = c["FX-PAINTBUCKET-005"]["0"]
    # The corners: 8 of the box's 9 (the edges held), the soft edge or line its ninth.
    assert near(five[at(0, 0)], [8 / 9, 0, 0, 8 / 9]) and near(five[at(15, 9)], [8 / 9, 0, 0, 8 / 9])
    assert near(five[at(0, 5)], [2 / 3, 0, 0, 2 / 3])  # the soft edge beside it is not taken
    assert five[at(8, 5)] == art[at(8, 5)]
    assert c["FX-PAINTBUCKET-006"]["0"] == art
    seven = c["FX-PAINTBUCKET-007"]["0"]
    assert all(near(seven[at(x, y)], red) for x in range(3, 14) for y in range(2, 8))
    assert c["FX-PAINTBUCKET-008"]["0"] == seven
    nine = c["FX-PAINTBUCKET-009"]["0"]
    assert near(nine[at(0, 0)], red) and near(nine[at(12, 5)], red)
    assert nine[at(6, 4)] == art[at(6, 4)]
    ten = c["FX-PAINTBUCKET-010"]["0"]
    assert all(ten[at(x, y)] == ([1.0] * 4 if skin(x, y) else [0.0, 0.0, 0.0, 1.0])
               for x in range(W) for y in range(H))
    eleven = c["FX-PAINTBUCKET-011"]["0"]
    assert all(eleven[at(x, y)] == ([1.0] * 4 if inside(x, y) else [0.0, 0.0, 0.0, 1.0])
               for x in range(W) for y in range(H))
    assert c["FX-PAINTBUCKET-012"]["0"] != one
    thirteen, fourteen = c["FX-PAINTBUCKET-013"]["0"], c["FX-PAINTBUCKET-014"]["0"]
    # Spread 1 takes in column 2 and column 10, so the skin's own edge columns are wholly red
    # (FX-PAINTBUCKET-001 has them at two thirds) and the line two thirds.
    assert near(thirteen[at(3, 5)], red) and near(thirteen[at(9, 5)], red)
    assert near(one[at(3, 5)], laid(3, 5, 2 / 3)) and near(thirteen[at(2, 5)], laid(2, 5, 2 / 3))
    # Choke 1 leaves columns 4 to 8, so column 3 sees one column of three; Stroke 1 leaves only
    # column 3, which sees itself alone.
    assert near(fourteen[at(3, 5)], laid(3, 5, 1 / 3))
    fifteen = c["FX-PAINTBUCKET-015"]["0"]
    assert near(fifteen[at(3, 5)], laid(3, 5, 1 / 3))
    assert fifteen[at(6, 4)] != one[at(6, 4)]
    assert c["FX-PAINTBUCKET-016"]["0"] == one
    seventeen = c["FX-PAINTBUCKET-017"]["0"]
    assert seventeen[at(0, 0)] == clear and near(seventeen[at(6, 4)], red)
    assert seventeen[at(12, 5)] == clear
    eighteen, nineteen = c["FX-PAINTBUCKET-018"]["0"], c["FX-PAINTBUCKET-019"]["0"]
    assert eighteen[at(6, 4)][0] < art[at(6, 4)][0] and nineteen[at(6, 4)][2] > art[at(6, 4)][2]
    twenty = c["FX-PAINTBUCKET-020"]["0"]
    assert near(twenty[at(6, 4)], [(a + b) / 2 for a, b in zip(art[at(6, 4)], red)])
    assert c["FX-PAINTBUCKET-021"]["0"] == art
    moving = c["FX-PAINTBUCKET-022"]
    assert moving["0"] == moving["2"] == one and moving["4"] != one
    tol = c["FX-PAINTBUCKET-023"]
    assert tol["0"] == tol["2"] == one and tol["4"] == two
    eased = c["FX-PAINTBUCKET-024"]
    assert eased["0"] != one and eased["2"] == one and eased["4"] == one
    moved = c["FX-PAINTBUCKET-025"]["0"]
    assert all(moved[at(x, y)] == (one[at(x - 3, y)] if x >= 3 else clear)
               for x in range(W) for y in range(H))
    assert c["FX-PAINTBUCKET-026"]["0"] == one
    assert all(near(p, red) for p in c["FX-PAINTBUCKET-027"]["0"])
    twentyeight = c["FX-PAINTBUCKET-028"]["0"]
    # Columns 1 and 2 of the box are the area: 6 of 9, times the soft edge's own alpha.
    k = 6 / 9 * art[at(1, 5)][3]
    assert near(twentyeight[at(1, 5)], [k, 0.0, 0.0, k]) and twentyeight[at(0, 0)] == clear
    nine29 = c["FX-PAINTBUCKET-029"]["0"]
    # Choke 2 leaves columns 5 to 7 in rows 4 and 5: (6, 4) sees six of them.
    assert near(nine29[at(6, 4)], laid(6, 4, 6 / 9)) and nine29[at(3, 2)] == art[at(3, 2)]
    thirty = c["FX-PAINTBUCKET-030"]["0"]
    assert thirty[at(6, 4)][2] > art[at(6, 4)][2]
    thirtyone = c["FX-PAINTBUCKET-031"]["0"]
    assert near(thirtyone[at(0, 5)], red) and thirtyone[at(1, 5)] != five[at(1, 5)]
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print(f"checked (nearest comparison {close:.4g} from its tolerance)")


if __name__ == "__main__":
    main()
