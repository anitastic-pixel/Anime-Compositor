"""Path Stroke along a text layer's letters, worked a second way.

D-373 (P0-22, third part) adds a text layer's letter outlines as a source for Path Stroke:
`source` "text" (Path From "Text Outlines") takes the paths from the layer's words as they are
drawn at the frame, text animators (D-350) included, in the layer's own space. Each contour of
each glyph is one path, always closed. They are counted glyph by glyph in the order the words are
read (a cluster at its first character), each glyph's contours in the order the font stores them;
Path and All Masks pick among them as they pick masks. A contour begins at its first point on the
curve as the font stores it (when the first two stored points are both off the curve, at the point
halfway between them); a quadratic piece is the cubic it is, its two handles two thirds of the way
from each end to the control point; and every piece is cut into straight bits as document 19 cuts
a mask: a curved piece into n = clamp(ceil(h / 2), 16, 512) bits, h the length of its control
polygon. Each outline is placed where the build draws it: a TrueType glyph slid sideways by its
hmtx left side bearing less its header's xMin (D-372 (a)). On a layer that is not a text layer,
or with no such outline (only spaces, or Path past the last), nothing is drawn and
EFFECT_PATH_MISSING is said every frame. The stroke itself is D-356's, and this file draws it with
`tools/stroke_reference.py`'s own functions.

**This file never runs the build's code path.** It shapes with HarfBuzz (uharfbuzz, kerning off
and ligatures on, as the build), reads each glyph's stored points with fontTools, and does the rest
in double precision on lists.

Every case is a composition 48 pixels by 30 holding one text layer the same size, its anchor its
position, so its space is the composition's: "Yes" in the bundled M PLUS Rounded 1c, size 24, its
baseline starting at (4, 25), Path Stroke On Transparent so the stroke alone is the frame. The
projects go into `Fixtures/stroke` as FX-STROKE-062 to 069, the expected frames and the outlines'
numbers into `Fixtures/stroke/expected_stroke_text.json`. A case whose layer would be drawn as it
is (the letters filled) has no frames written: the build's test checks it against the same frame
with the effect switched off.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/stroke_text_reference.py
"""

import json
import math
import sys
from pathlib import Path

import uharfbuzz as hb
from fontTools.ttLib import TTFont

sys.path.insert(0, str(Path(__file__).resolve().parent))
import stroke_reference as SR  # noqa: E402
from fxkey_reference import keyed, setting_json  # noqa: E402
from adjust_reference import prop  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
FONT_FILE = ROOT / "assets" / "fonts" / "MPLUSRounded1c-Regular.ttf"
FONT = TTFont(str(FONT_FILE))
GLYF, HMTX = FONT["glyf"], FONT["hmtx"]
UPEM = FONT["head"].unitsPerEm
HB_FONT = hb.Font(hb.Face(FONT_FILE.read_bytes()))
OUT = SR.OUT
W, H = 48, 30
SIZE, AT, WORD = 24, [4, 25], "Yes"
MIN_PIECES, MAX_PIECES = 16, 512


# --- the outlines ---------------------------------------------------------------------------

def shaped(chars):
    """HarfBuzz's glyphs for one line: (glyph name, advance, x offset, y offset, character)."""
    buf = hb.Buffer()
    buf.add_str(chars)
    buf.guess_segment_properties()
    hb.shape(HB_FONT, buf, {"kern": False, "liga": True})
    return [(FONT.getGlyphName(i.codepoint), p.x_advance, p.x_offset, p.y_offset, i.cluster)
            for i, p in zip(buf.glyph_infos, buf.glyph_positions)]


def contours(name):
    """The glyph's contours as stored: each a list of (x, y, on the curve)."""
    g = GLYF[name]
    coords, ends, flags = g.getCoordinates(GLYF)
    out, a = [], 0
    for e in ends:
        out.append([(coords[i][0], coords[i][1], bool(flags[i] & 1)) for i in range(a, e + 1)])
        a = e + 1
    return out


def pieces(contour):
    """The contour as a closed run of pieces from its starting point: each (from, c1, c2, to), the
    two handles None for a straight piece. Quadratics are given by their control point, c2 None."""
    pts = [(x, y) for x, y, _ in contour]
    on = [o for _, _, o in contour]
    n = len(pts)
    if on[0]:
        start, first = pts[0], 1
    elif on[1 % n]:
        start, first = pts[1 % n], 2
    else:
        start, first = ((pts[0][0] + pts[1][0]) / 2, (pts[0][1] + pts[1][1]) / 2), 1
    # Walk once round, from just after the start back to it.
    walk = [(pts[(first + k) % n], on[(first + k) % n]) for k in range(n)]
    if on[0] or on[1 % n]:
        walk = walk[:n - 1] + [(start, True)]  # the start point itself closes the run
    else:
        walk = walk + [(start, True)]
    out, now, ctrl = [], start, None
    for p, is_on in walk:
        if is_on:
            out.append((now, ctrl, None, p) if ctrl else (now, None, None, p))
            now, ctrl = p, None
        elif ctrl is None:
            ctrl = p
        else:
            mid = ((ctrl[0] + p[0]) / 2, (ctrl[1] + p[1]) / 2)
            out.append((now, ctrl, None, mid))
            now, ctrl = mid, p
    return out


def flatten(run):
    """Document 19's cutting of a closed path, from each piece's start."""
    pts = []
    for a, q, _, d in run:
        pts.append(a)
        if q is None:
            continue
        b = (a[0] + (q[0] - a[0]) * 2 / 3, a[1] + (q[1] - a[1]) * 2 / 3)
        c = (d[0] + (q[0] - d[0]) * 2 / 3, d[1] + (q[1] - d[1]) * 2 / 3)
        h = math.dist(a, b) + math.dist(b, c) + math.dist(c, d)
        m = min(MAX_PIECES, max(MIN_PIECES, math.ceil(h / 2)))
        for k in range(1, m):
            t = k / m
            u = 1 - t
            w = (u ** 3, 3 * u * u * t, 3 * u * t * t, t ** 3)
            pts.append(tuple(w[0] * a[j] + w[1] * b[j] + w[2] * c[j] + w[3] * d[j] for j in (0, 1)))
    return pts


def outlines(words, size, at, move=None):
    """Every contour of the words as a closed path in the layer's pixels, in reading order.
    `move`, per character, an (x, y) a text animator moves it by (D-350's Position)."""
    scale = size / UPEM
    out, x = [], at[0]
    for name, advance, dx, dy, ch in shaped(words):
        slide = HMTX[name][1] - GLYF[name].xMin if GLYF[name].numberOfContours else 0
        m = move[ch] if move else (0, 0)
        ox, oy = x + (dx + slide) * scale + m[0], at[1] - dy * scale + m[1]
        for contour in contours(name):
            run = [tuple(None if p is None else (ox + p[0] * scale, oy - p[1] * scale) for p in piece)
                   for piece in pieces(contour)]
            out.append(flatten(run))
        x += advance * scale
    return out


def length(path):
    return SR.measured(path)[1]


# --- the cases ------------------------------------------------------------------------------

def case(words=WORD, move=None, kind="text", **settings):
    c = SR.case(masks=[], paint_style="on_transparent", brush_size=2, **settings)
    c.update({"words": words, "move": move, "kind": kind, "source": "text"})
    return c


def paths_of(c, f):
    if c["kind"] != "text":
        return None
    usable = outlines(c["words"], SIZE, AT, c["move"])
    if c["all_masks"] == "on":
        chosen = usable
    else:
        i = math.floor(SR.held(c, "mask", f)) - 1
        chosen = usable[i:i + 1]
    return [(p, True) for p in chosen] or None


def render(c, f):
    paths = paths_of(c, f)
    if paths is None:
        return None
    n = {k: SR.held(c, k, f) for k in SR.RANGES}
    n["sequential"] = c["all_masks"] == "on" and c["stroke_sequentially"] == "on"
    layer = {"px": [[0.0] * 4] * (W * H), "left": 0, "top": 0, "w": W, "h": H}
    return SR.stroke(layer, paths, n, c["color"], c["paint_style"])["px"]


# D-350: an animator, Position (0, -6), Start 50 of "Yes"'s three characters: "e" half picked,
# moved up 3; "s" wholly, up 6.
LIFT = [(0, 0), (0, -3), (0, -6)]
LIFT_ANIMATOR = {"position": [0, -6], "start": 50}

CASES = {
    "FX-STROKE-062": ("\"Yes\", All Masks on: every letter's outline drawn in white, Brush Size 2, "
                      "the e's and its eye's both.", case(all_masks="on"), [0]),
    "FX-STROKE-063": ("\"Yes\", Path 2: the e's eye alone. The Y has one outline and the e two, "
                      "and the font stores the e's eye before its outer edge.", case(mask=2), [0]),
    "FX-STROKE-064": ("\"Yes\", All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 "
                      "to 100 at frame 4: nothing, then the outlines one after another as one "
                      "length to half way, then all.",
                      case(all_masks="on", stroke_sequentially="on", end=keyed((0, 0), (4, 100))),
                      [0, 2, 4]),
    "FX-STROKE-065": ("\"Yes\", All Masks on, End 50, Stroke Sequentially off: each outline drawn "
                      "half way round from where it begins.", case(all_masks="on", end=50), [0]),
    "FX-STROKE-066": ("\"Yes\" with a text animator, Position (0, -6), Start 50: the e moved up 3, "
                      "the s up 6, and the stroke follows them.",
                      case(all_masks="on", move=LIFT), [0]),
}
MISSING = {
    "FX-STROKE-067": ("Path From Text Outlines on a shape layer with no shapes, not a text layer.",
                      case(kind="shape"), True),
    "FX-STROKE-068": ("A text layer of one space, which has no outline.", case(words=" "), True),
    "FX-STROKE-069": ("\"Yes\", Path 9: it has four outlines.", case(mask=9), False),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    parameters = {k: (c[k] if k in SR.WORDS else setting_json(c[k])) for k in SR.NAMES}
    parameters["source"] = "text"
    effects = [{"instance_id": "fx-0-0", "type_id": "core.stroke", "enabled": True,
                "parameters": parameters}]
    if c["move"]:
        effects.insert(0, {"instance_id": "fx-0-a", "type_id": "core.text_animator",
                           "enabled": True, "parameters": {
                               "position": [0, 0], "scale": [100, 100], "rotation": 0,
                               "opacity": 100, "fill": "off", "color": [1, 0, 0], "tracking": 0,
                               "start": 0, "end": 100, "offset": 0, "amount": 100,
                               "based_on": "characters", "shape": "square", "smoothness": 100,
                               "ease_high": 0, "ease_low": 0, **LIFT_ANIMATOR}})
    layer = {"id": "art", "kind": c["kind"], "name": "art", "enabled": True, "locked": False,
             "in_frame": 0, "out_frame": 16}
    if c["kind"] == "text":
        layer["source_text"] = {"text": c["words"], "font": FONT_FILE.name, "size": SIZE,
                                "color": [1, 1, 1], "at": AT, "align": "left"}
    else:
        layer["shapes"] = []
    layer.update({"transform": {"anchor": prop([W / 2, H / 2]), "position": prop([W / 2, H / 2]),
                                "scale": prop([100, 100]), "rotation": prop(0),
                                "opacity": prop(1)},
                  "masks": [], "matte": None, "blend_mode": "normal", "effects": effects})
    return {
        "schema_version": 0, "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 16,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 16},
            "layer_order": ["art"], "layers": [layer]}],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    paths = outlines(WORD, SIZE, AT)
    lengths = [length(p) for p in paths]
    total = sum(lengths)
    # Stroke Sequentially, End 50: what each outline draws, from and to along its own length.
    half = [list(w) if w else None for _, w in SR.windows([(p, True) for p in paths], 0, 50, True)]
    expected = {"tolerance": SR.TOLERANCE, "width": W, "height": H,
                "outlines": {"says": f"\"{WORD}\" in {FONT_FILE.name}, size {SIZE}, its baseline "
                             f"starting at {AT}: its outlines in reading order, each one's points "
                             "after cutting, its length, and with Stroke Sequentially at End 50 "
                             "what each draws along its own length, from and to.",
                             "count": len(paths), "points": [len(p) for p in paths],
                             "lengths": lengths, "total": total, "sequential_end_50": half,
                             "starts": [list(p[0]) for p in paths],
                             "length_tolerance": 1e-6},
                "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(p[3] > 0 for p in px)} covered"
                                    for f, px in rendered.items()))
    for fx, (says, c, empty) in MISSING.items():
        says += (" Nothing to draw along: the layer is drawn without the effect, which is kept "
                 "as written, and EFFECT_PATH_MISSING is said every frame.")
        assert paths_of(c, 0) is None
        clear = [[0.0] * 4 for _ in range(W * H)]
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": clear, "4": clear} if empty else {},
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    (OUT / "expected_stroke_text.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected, paths)


def check(expected, paths):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    o = expected["outlines"]
    assert o["count"] == 4, o["count"]                    # Y, e and its eye, s
    # Reading order: each outline starts right of or inside the one before's letter.
    assert paths[0][0][0] < 4 + 610 * SIZE / UPEM < min(x for x, _ in paths[1])
    assert max(x for x, _ in paths[2]) < min(x for x, _ in paths[3])
    # The font stores the e's eye first: outline 2 is inside outline 3.
    xs = lambda p: [x for x, _ in p]  # noqa: E731
    assert min(xs(paths[2])) < min(xs(paths[1])) and max(xs(paths[1])) < max(xs(paths[2]))
    # End 50 sequentially: half the total drawn, outlines before the middle whole.
    drawn = sum(w[1] - w[0] for w in o["sequential_end_50"] if w)
    assert abs(drawn - o["total"] / 2) < 1e-9, drawn
    seq = c["FX-STROKE-064"]
    assert all(p[3] == 0 for p in seq["0"])
    assert seq["4"] == c["FX-STROKE-062"]["0"]
    covered = lambda px: sum(p[3] > 0 for p in px)  # noqa: E731
    assert 0 < covered(seq["2"]) < covered(seq["4"])
    assert covered(c["FX-STROKE-063"]["0"]) < covered(c["FX-STROKE-062"]["0"])
    assert c["FX-STROKE-065"]["0"] != seq["2"]
    lifted = c["FX-STROKE-066"]["0"]
    # The Y is not moved: its columns are the same; the s's are not.
    y_cols = range(0, int(4 + 610 * SIZE / UPEM) - 2)
    at = lambda px, x, y: px[y * W + x]  # noqa: E731
    assert all(at(lifted, x, y) == at(c["FX-STROKE-062"]["0"], x, y) for x in y_cols for y in range(H))
    assert lifted != c["FX-STROKE-062"]["0"]
    for px in [p for f in c.values() for p in f.values()]:
        assert all(0 <= p[3] <= 1 + 1e-12 for p in px)
    print("checked")


if __name__ == "__main__":
    main()
