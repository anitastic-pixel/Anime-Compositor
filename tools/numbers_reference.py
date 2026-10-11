"""Numbers, worked a second way.

D-526 adds `core.numbers`, Numbers, after After Effects' Numbers (Text), on EFFECTS.md row 387: a
number, a timecode, a time of day, a date or a hexadecimal number drawn as words on the layer,
counted by keys on its value or picked at random each frame. Adobe's page names the settings and
says what each type shows; it gives no layout, no colours and no formulas. Everything below is
this program's own rule, modelled on that page; nothing is ported.

The settings, each in the file under its own name:

- type: "number" (when added), "number_leading_zeros", "timecode_30", "timecode_25",
  "timecode_24", "time", "numerical_date", "short_date", "long_date" or "hexadecimal"
- random_values: "off" (when added) or "on"
- value: -30,000 to 30,000, 0 when added, keyable
- decimal_places: 0 to 10, 0 when added, taken down to a whole number; read by the two number
  types only
- position: two numbers, per cent of the drawing's width and height, -1000 to 1000, 50, 50
  when added: where the words' baseline meets their aligned end
- display: "fill_only" (when added), "stroke_only", "fill_over_stroke" or "stroke_over_fill"
- fill_color, "#ffffff" when added; stroke_color, "#ff3300" when added; encoded sRGB, "#rrggbb"
- stroke_width: 0 to 500 pixels, 3 when added
- size: 1 to 2000 pixels (the words' em), 64 when added
- tracking: thousandths of the size, -1000 to 1000, added after each character, 0 when added
- proportional_spacing: "on" (when added) or "off"
- composite_on_original: "on" (when added) or "off"
- font: a font file's name as text layers name one (D-263), the bundled
  "MPLUSRounded1c-Regular.ttf" when added
- alignment: "left", "center" (when added) or "right"

The rule. At composition frame n the layer's time is t = (n - in point) / fps seconds (through
its key time, as Scribble's, D-442), its frame k = n - in point, and V the value at n.

1. Random values. With random_values "on", V becomes r x M: M is V, or 30,000 when V is 0, and
   r = (u + 1) / 2 with u = `grade::unit(mix(0x4E554D42), 0, 0, k, 0)`, D-119's hash; a new
   number every frame of the layer.
2. What is shown, a string s:
   - number: V with `decimal_places` places, correctly rounded from its binary value (ties to
     even); a minus is dropped when every digit shown is 0.
   - number_leading_zeros: as number, the whole part padded with zeros to five digits, the
     minus (when kept) in front.
   - timecode_N (N 30, 25 or 24, non-drop): T = t + V seconds, or with random values T = V (a
     random time, not the layer's); f = floor(|T| x N + 1e-6) frames; s = hh:mm:ss:ff, each two
     digits (the hours more when they pass 99), a minus in front when T < 0 and f > 0.
   - time: V in hours; m = floor(V x 60 + 1e-6) minutes, wrapped into one day (0 to 1439,
     negative counting back from midnight); s = HH:MM, a 24-hour clock.
   - the dates: D = floor(V + 1e-6) days after 1 January 1995 (0 is that day, negative before
     it); numerical_date "M/D/YYYY", short_date "Mon D, YYYY", long_date "Weekday, Month D,
     YYYY", English names.
   - hexadecimal: h = floor(|V| x 65536 + 1e-6), as Adobe's 0x10000 for each 1.0; upper-case
     digits, a minus in front when V < 0 and h > 0.
   Current Time/Date (the computer's clock at render time) is not built: a frame must be the same
   on every render (document 21).
3. Layout, in the drawing's pixels, P = position / 100 x the drawing's size. With proportional
   spacing on, s is one D-263 text: font, size, `tracking`, alignment, its first baseline meeting
   its aligned end at P, the font's kerning off. With it off, every character sits in a cell
   as wide as the widest of the font's digits 0 to 9 (c, from the font's advances), centred in
   it; with a = tracking / 1000 x size, the line is L = len(s) x c + (len(s) - 1) x a wide and
   starts at x0 = P.x, P.x - L / 2 or P.x - L (left, center, right); character i is its own
   left-aligned text at x0 + i x (c + a) + (c - w_i) / 2 on the baseline P.y, w_i its own
   advance, drawn in order, each over the ones before.
4. Two pictures, each the drawing's size: F, the characters filled in the fill colour (D-263);
   and, when stroke_width > 0 and display is not fill_only, S, the characters in the stroke
   colour with D-264's stroke of that colour and width round them (the whole stroked shape). A
   character's picture is laid over the ones before as the layers of a composition are.
5. The words, W: fill_only F; fill_over_stroke F over S; stroke_only S x (1 - F's alpha), the
   ring outside the letters; stroke_over_fill (S x (1 - F's alpha)) over F. Without S, W is F
   for the two that fill and nothing for stroke_only. D-264's stroke lies wholly outside the
   letters, so stroke_over_fill differs from fill_over_stroke only along the letters' edges.
6. Composite on original "on": W over the layer as it was; "off": W alone.
7. A font this computer does not have is said each frame as `TEXT_FONT_MISSING` and nothing is
   drawn: the layer is left as it was (D-263: no other font in its place).

**This file never runs the build's code path.** It works each string with Python's own
`datetime`, string formatting and integer arithmetic, and the monospaced cells from the font's
advances read with fontTools. The pixels of words are pinned the way D-263 pins a text layer's:
every case at every frame names a second project, in `Fixtures/numbers/same/`, that draws the
same words from text layers (two compositions, the fills and the strokes, laid as rule 5 says
with an inverted alpha track matte, D-293) and no Numbers at all; the case's frame must match
that project's within 2e-5. The strings this file worked are in
`Fixtures/numbers/expected_numbers.json` for reading.

Sources read: Adobe's "Text effects" page for After Effects (helpx.adobe.com, via the
web.archive.org copy), its Numbers section; a Creative COW forum thread (creativecow.net, p=1856527)
on counting past 30,000, where Numbers' even spacing is why it is used over a text layer; an Adobe
community thread ("Number effect random timecode 30") on random timecode changing every frame with
no range of its own; Noble Desktop's "Number Counter" tutorial (blog.nobledesktop.com), which
counts with a slider keyed from 0 to 10 driving a text layer.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/numbers_reference.py   (with fontTools)
"""

import datetime
import json
import math
import sys
from pathlib import Path

from fontTools.ttLib import TTFont

sys.path.insert(0, str(Path(__file__).resolve().parent))
from noise_reference import mix, M as MASK  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "numbers"
SAME = OUT / "same"
TOLERANCE = 2e-5  # document 25's default for a filter
FONT_NAME = "MPLUSRounded1c-Regular.ttf"
FONT = TTFont(str(ROOT / "assets" / "fonts" / FONT_NAME))
CMAP = FONT.getBestCmap()
HMTX = FONT["hmtx"]
UPEM = FONT["head"].unitsPerEm
W, H, FPS, FRAMES = 480, 160, 24, 60
BACK = [0.02, 0.05, 0.12]

TYPES = ("number", "number_leading_zeros", "timecode_30", "timecode_25", "timecode_24", "time",
         "numerical_date", "short_date", "long_date", "hexadecimal")
DISPLAYS = ("fill_only", "stroke_only", "fill_over_stroke", "stroke_over_fill")
DEFAULTS = {
    "type": "number", "random_values": "off", "value": 0, "decimal_places": 0,
    "position": [50, 50], "display": "fill_only", "fill_color": "#ffffff",
    "stroke_color": "#ff3300", "stroke_width": 3, "size": 64, "tracking": 0,
    "proportional_spacing": "on", "composite_on_original": "on", "font": FONT_NAME,
    "alignment": "center",
}
MONTHS = ("January", "February", "March", "April", "May", "June", "July", "August", "September",
          "October", "November", "December")
DAYS = ("Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday")


# --- the rule -------------------------------------------------------------------------------

def unit(base, x, y, f, ch):
    """Noise's hash from an already mixed base, as the build's `grade::unit`."""
    h = mix(mix(mix(mix(base ^ (x & MASK)) ^ (y & MASK)) ^ (f & MASK)) ^ ch)
    return (h >> 11) / 2 ** 53 * 2 - 1


def value_at(v, n):
    """A keyable setting at frame n: a number, or keys as (frame, value) pairs, linear between."""
    if not isinstance(v, dict):
        return v
    keys = v["keys"]
    if n <= keys[0][0]:
        return keys[0][1]
    for (f0, v0), (f1, v1) in zip(keys, keys[1:]):
        if n <= f1:
            return v0 + (v1 - v0) * (n - f0) / (f1 - f0)
    return keys[-1][1]


def unsigned(s):
    """A minus dropped when every digit shown is 0."""
    if s.startswith("-") and not any(ch in "123456789" for ch in s):
        return s[1:]
    return s


def shows(c, n):
    """Rules 1 and 2: the string at composition frame n."""
    k = n - c["in"]
    t = k / FPS
    v = value_at(c["value"], n)
    rand = c["random_values"] == "on"
    if rand:
        r = (unit(mix(0x4E554D42), 0, 0, k, 0) + 1) / 2
        v = r * (v if v != 0 else 30000)
    d = int(math.floor(c["decimal_places"]))
    kind = c["type"]
    if kind == "number":
        return unsigned(f"{v:.{d}f}")
    if kind == "number_leading_zeros":
        whole, dot, part = f"{abs(v):.{d}f}".partition(".")
        s = whole.rjust(5, "0") + dot + part
        return unsigned("-" + s) if v < 0 else s
    if kind.startswith("timecode_"):
        rate = int(kind[-2:])
        when = v if rand else t + v
        f = math.floor(abs(when) * rate + 1e-6)
        s = "%02d:%02d:%02d:%02d" % (f // (rate * 3600), f // (rate * 60) % 60, f // rate % 60,
                                     f % rate)
        return "-" + s if when < 0 and f > 0 else s
    if kind == "time":
        m = math.floor(v * 60 + 1e-6) % 1440
        return "%02d:%02d" % (m // 60, m % 60)
    if kind.endswith("_date"):
        day = datetime.date(1995, 1, 1) + datetime.timedelta(days=math.floor(v + 1e-6))
        if kind == "numerical_date":
            return f"{day.month}/{day.day}/{day.year}"
        if kind == "short_date":
            return f"{MONTHS[day.month - 1][:3]} {day.day}, {day.year}"
        return f"{DAYS[day.weekday()]}, {MONTHS[day.month - 1]} {day.day}, {day.year}"
    h = math.floor(abs(v) * 65536 + 1e-6)
    return ("-" if v < 0 and h > 0 else "") + format(h, "X")


def advance(ch, size):
    return HMTX[CMAP[ord(ch)]][0] * size / UPEM


def linear(hexcolour):
    out = []
    for i in (1, 3, 5):
        e = int(hexcolour[i:i + 2], 16) / 255
        out.append(e / 12.92 if e <= 0.04045 else ((e + 0.055) / 1.055) ** 2.4)
    return out


def texts(c, s, colour, stroke):
    """Rule 3: the D-263 texts that draw s, in the order drawn."""
    px = [c["position"][0] / 100 * W, c["position"][1] / 100 * H]
    size = c["size"]
    extra = {"stroke": stroke} if stroke else {}
    if c["proportional_spacing"] == "on":
        if c["tracking"]:
            extra["tracking"] = c["tracking"]
        return [{"text": s, "font": c["font"], "size": size, "color": colour, "at": px,
                 "align": c["alignment"], **extra}]
    cell = max(advance(ch, size) for ch in "0123456789")
    a = c["tracking"] / 1000 * size
    line = len(s) * cell + (len(s) - 1) * a
    x0 = {"left": px[0], "center": px[0] - line / 2, "right": px[0] - line}[c["alignment"]]
    return [{"text": ch, "font": c["font"], "size": size, "color": colour,
             "at": [x0 + i * (cell + a) + (cell - advance(ch, size)) / 2, px[1]], "align": "left",
             **extra}
            for i, ch in enumerate(s)]


# --- the projects ---------------------------------------------------------------------------

def prop(base):
    return {"base": base, "keyframes": []}


def transform(at):
    return {"anchor": prop(at), "position": prop(at), "scale": prop([100, 100]),
            "rotation": prop(0), "opacity": prop(1)}


def base_layer(lid, kind, c, matte=None):
    return {"id": lid, "kind": kind, "name": lid, "enabled": True, "locked": False,
            "in_frame": c["in"], "out_frame": FRAMES, "transform": transform([W / 2, H / 2]),
            "matte": matte, "blend_mode": "normal", "effects": [], "masks": []}


def setting_json(v):
    if isinstance(v, dict):
        return {"base": v["keys"][0][1],
                "keyframes": [{"frame": f, "value": x, "interp": "linear"} for f, x in v["keys"]]}
    return v


def solid(c, effects):
    layer = base_layer("plate", "solid", c)
    layer["solid"] = {"color": BACK, "width": W, "height": H}
    layer["effects"] = effects
    return layer


def text_layer(lid, source):
    layer = base_layer(lid, "text", {"in": 0})
    layer["source_text"] = source
    return layer


def nest(lid, comp, c, matte=None):
    layer = base_layer(lid, "composition", c, matte)
    layer["composition_id"] = comp
    layer["source_offset_frames"] = 0
    layer["transform"] = transform([0, 0])
    return layer


def composition(cid, layers):
    """`layers` bottom first, as a file lists them (document 19 draws `layer_order` bottom first)."""
    return {"id": cid, "name": cid, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": FPS, "denominator": 1}, "start_frame": 0,
            "duration_frames": FRAMES, "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": [l["id"] for l in layers], "layers": layers}


def project(pid, comps):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [], "compositions": comps}


def effect_json(c):
    return {"instance_id": "fx-1", "type_id": "core.numbers", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in DEFAULTS}}


def fixture(fx, c):
    return project("proj-" + fx.lower(), [composition("comp-main", [solid(c, [effect_json(c)])])])


def works(c):
    return c["valid"] and c["font"] == FONT_NAME


def same(fx, c, n):
    """Rules 3 to 7 as text layers: the project that must draw what the case draws at frame n."""
    comps = []
    main = [solid(c, [])] if c["composite_on_original"] == "on" or not works(c) else []
    if works(c):
        s = shows(c, n)
        fills = texts(c, s, linear(c["fill_color"]), None)
        comps.append(composition("comp-fills", [text_layer(f"fill-{i}", t) for i, t in enumerate(fills)]))
        stroked = c["stroke_width"] > 0 and c["display"] != "fill_only"
        if stroked:
            strokes = texts(c, s, linear(c["stroke_color"]),
                            {"color": linear(c["stroke_color"]), "width": c["stroke_width"]})
            comps.append(composition("comp-strokes",
                                     [text_layer(f"stroke-{i}", t) for i, t in enumerate(strokes)]))
        ring = [nest("ring", "comp-strokes", c, {"layer_id": "cut", "mode": "alpha_inverted",
                                                   "matte_only": True}),
                nest("cut", "comp-fills", c)] if stroked else []
        d = c["display"]
        if d == "fill_only":
            main += [nest("fills", "comp-fills", c)]
        elif d == "fill_over_stroke":
            main += ([nest("strokes", "comp-strokes", c)] if stroked else []) + [nest("fills", "comp-fills", c)]
        elif d == "stroke_only":
            main += ring
        else:
            main += [nest("fills", "comp-fills", c)] + ring
    return project(f"proj-{fx.lower()}-same-{n}", [composition("comp-main", main)] + comps)


def case(**settings):
    c = {**DEFAULTS, "in": 0, "valid": True}
    c.update(settings)
    return c


# --- the cases ------------------------------------------------------------------------------

CASES = {
    "FX-NUM-001": ("As added: 0 in white, centred, over the plate.", case(), [0]),
    "FX-NUM-002": ("1234.5678 with two places: 1234.57.", case(value=1234.5678, decimal_places=2), [0]),
    "FX-NUM-003": ("-12.5 with no places: -12, the tie to even.", case(value=-12.5), [0]),
    "FX-NUM-004": ("-0.004 with two places shows 0.00, its minus dropped.",
                   case(value=-0.004, decimal_places=2), [0]),
    "FX-NUM-005": ("Leading zeros, 42.75 with one place: 00042.8.",
                   case(type="number_leading_zeros", value=42.75, decimal_places=1), [0]),
    "FX-NUM-006": ("Leading zeros, -7: -00007.", case(type="number_leading_zeros", value=-7), [0]),
    "FX-NUM-007": ("Decimal places 2.9 is taken down to 2: 3.14.",
                   case(value=3.14159, decimal_places=2.9), [0]),
    "FX-NUM-008": ("Timecode 30 at frames 0 and 50 of a 24-frame composition: 00:00:00:00 and "
                   "00:00:02:02.", case(type="timecode_30"), [0, 50]),
    "FX-NUM-009": ("Timecode 25 with value 3600: an hour on, 01:00:02:02 at frame 50.",
                   case(type="timecode_25", value=3600), [50]),
    "FX-NUM-010": ("Timecode 24 on a layer starting at frame 10: 00:00:01:00 at frame 34.",
                   case(type="timecode_24", **{"in": 10}), [34]),
    "FX-NUM-011": ("Timecode 30 with value -1 at frame 12: -00:00:00:15.",
                   case(type="timecode_30", value=-1), [12]),
    "FX-NUM-012": ("Time 13.5: 13:30.", case(type="time", value=13.5), [0]),
    "FX-NUM-013": ("Time 25.25 wraps round the day: 01:15.", case(type="time", value=25.25), [0]),
    "FX-NUM-014": ("Time -0.5: 23:30.", case(type="time", value=-0.5), [0]),
    "FX-NUM-015": ("Numerical date 0: 1/1/1995.", case(type="numerical_date"), [0]),
    "FX-NUM-016": ("Short date 10000: May 19, 2022.", case(type="short_date", value=10000), [0]),
    "FX-NUM-017": ("Long date 365: Monday, January 1, 1996, left-aligned from 3 per cent.",
                   case(type="long_date", value=365, size=36, alignment="left", position=[3, 50]),
                   [0]),
    "FX-NUM-018": ("Long date -1: Saturday, December 31, 1994.",
                   case(type="long_date", value=-1, size=36), [0]),
    "FX-NUM-019": ("Hexadecimal 1.5: 18000.", case(type="hexadecimal", value=1.5), [0]),
    "FX-NUM-020": ("Hexadecimal -0.25: -4000.", case(type="hexadecimal", value=-0.25), [0]),
    "FX-NUM-021": ("Random values up to 100 with two places, a new number each frame.",
                   case(random_values="on", value=100, decimal_places=2), [0, 1, 7]),
    "FX-NUM-022": ("Random values with value 0: anywhere up to 30,000.",
                   case(random_values="on"), [3]),
    "FX-NUM-023": ("Random timecode 30 up to 60 seconds: a random time, not the layer's.",
                   case(type="timecode_30", random_values="on", value=60), [5]),
    "FX-NUM-024": ("A counter: value keyed from 0 at frame 0 to 100 at frame 10, 40 at frame 4 "
                   "and 100 from frame 10 on.",
                   case(value={"keys": [(0, 0), (10, 100)]}), [4, 12]),
    "FX-NUM-025": ("Stroke only, width 4: the ring outside the letters.",
                   case(value=2026, display="stroke_only", stroke_width=4), [0]),
    "FX-NUM-026": ("Fill over stroke, width 6.",
                   case(value=2026, display="fill_over_stroke", stroke_width=6), [0]),
    "FX-NUM-027": ("Stroke over fill, width 6.",
                   case(value=2026, display="stroke_over_fill", stroke_width=6), [0]),
    "FX-NUM-028": ("Stroke only with width 0: nothing drawn.",
                   case(value=2026, display="stroke_only", stroke_width=0), [0]),
    "FX-NUM-029": ("Monospaced 1.11 with tracking 100: each character in a digit's cell.",
                   case(value=1.11, decimal_places=2, proportional_spacing="off", tracking=100),
                   [0]),
    "FX-NUM-030": ("Monospaced timecode 25, fill over stroke, left-aligned from 5 per cent: the "
                   "colons in cells as wide as the digits.",
                   case(type="timecode_25", proportional_spacing="off", display="fill_over_stroke",
                        alignment="left", position=[5, 60]), [37]),
    "FX-NUM-031": ("Monospaced, stroke only, right-aligned at 95 per cent.",
                   case(value=-3.5, decimal_places=1, proportional_spacing="off",
                        display="stroke_only", alignment="right", position=[95, 50]), [0]),
    "FX-NUM-032": ("Monospaced, stroke over fill, composite off: the words alone on transparent.",
                   case(value=88, proportional_spacing="off", display="stroke_over_fill",
                        composite_on_original="off", fill_color="#20c0ff", stroke_color="#102040"),
                   [0]),
    "FX-NUM-033": ("Composite off, right-aligned at 90, 70 per cent, size 40, tracking -50.",
                   case(value=512, composite_on_original="off", alignment="right",
                        position=[90, 70], size=40, tracking=-50), [0]),
    "FX-NUM-034": ("A font this computer does not have: said each frame, nothing drawn, the "
                   "plate as it was.", case(font="NotAFontAnywhere.ttf"), [0]),
}

INVALID = {
    "FX-NUM-040": ("Type written \"roman\".", case(type="roman", valid=False)),
    "FX-NUM-041": ("Display written \"outline\".", case(display="outline", valid=False)),
    "FX-NUM-042": ("Value 40,000, past 30,000.", case(value=40000, valid=False)),
    "FX-NUM-043": ("Decimal places 11.", case(decimal_places=11, valid=False)),
    "FX-NUM-044": ("Font written \"../fonts/x.ttf\", a path.", case(font="../fonts/x.ttf", valid=False)),
    "FX-NUM-045": ("Fill colour written \"white\".", case(fill_color="white", valid=False)),
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    SAME.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in list(CASES.items()) + [(k, (s, c, [0])) for k, (s, c) in INVALID.items()]:
        file = fx.lower().replace("-", "_") + ".json"
        (OUT / file).write_text(json.dumps(fixture(fx, c), indent=2) + "\n", encoding="utf-8")
        entry = {"says": says, "project": file, "frames": {}}
        for n in frames:
            twin = f"same/{fx.lower().replace('-', '_')}_{n}.json"
            (OUT / twin).write_text(json.dumps(same(fx, c, n), indent=2) + "\n", encoding="utf-8")
            entry["frames"][str(n)] = {"same": twin, "shows": shows(c, n) if works(c) else None}
        if not c["valid"]:
            entry["says"] += (" The file is read, the effect is kept as written and left out of "
                              "every frame, with a warning.")
            entry["warning"] = "EFFECT_PARAMETER_INVALID"
        elif not works(c):
            entry["frame_warning"] = "TEXT_FONT_MISSING"
        expected["cases"][fx] = entry
        print(fx, {n: v["shows"] for n, v in entry["frames"].items()})
    (OUT / "expected_numbers.json").write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the strings just worked."""
    s = {fx: {int(n): v["shows"] for n, v in e["frames"].items()} for fx, e in expected["cases"].items()}
    want = {
        "FX-NUM-001": {0: "0"}, "FX-NUM-002": {0: "1234.57"}, "FX-NUM-003": {0: "-12"},
        "FX-NUM-004": {0: "0.00"}, "FX-NUM-005": {0: "00042.8"}, "FX-NUM-006": {0: "-00007"},
        "FX-NUM-007": {0: "3.14"}, "FX-NUM-008": {0: "00:00:00:00", 50: "00:00:02:02"},
        "FX-NUM-009": {50: "01:00:02:02"}, "FX-NUM-010": {34: "00:00:01:00"},
        "FX-NUM-011": {12: "-00:00:00:15"}, "FX-NUM-012": {0: "13:30"}, "FX-NUM-013": {0: "01:15"},
        "FX-NUM-014": {0: "23:30"}, "FX-NUM-015": {0: "1/1/1995"},
        "FX-NUM-016": {0: "May 19, 2022"}, "FX-NUM-017": {0: "Monday, January 1, 1996"},
        "FX-NUM-018": {0: "Saturday, December 31, 1994"}, "FX-NUM-019": {0: "18000"},
        "FX-NUM-020": {0: "-4000"}, "FX-NUM-024": {4: "40", 12: "100"},
        "FX-NUM-029": {0: "1.11"}, "FX-NUM-030": {37: "00:00:01:13"}, "FX-NUM-031": {0: "-3.5"},
    }
    for fx, frames in want.items():
        assert s[fx] == frames, (fx, s[fx], frames)
    r = s["FX-NUM-021"]
    assert len(set(r.values())) == 3 and all(0 <= float(v) < 100 for v in r.values()), r
    assert 0 <= float(s["FX-NUM-022"][3]) < 30000
    assert s["FX-NUM-023"][5] < "00:01:00:00"
    # The cells: in the bundled font every digit is 620 units, a colon 364.
    assert advance("0", 1000) == 620 and advance(":", 1000) == 364
    print("checks pass")


if __name__ == "__main__":
    main()
