"""Timecode, worked a second way.

D-527 adds `core.timecode`, Timecode, after After Effects' Timecode (Text), on EFFECTS.md row 388:
the layer's or the composition's time, or a count of the program's own, burnt onto the picture as
SMPTE timecode, a frame number or feet and frames of film, on a box. Adobe's page names the
settings and says what each does; it gives no layout and no formulas. Everything below is this
program's own rule, modelled on that page; nothing is ported.

The settings, each in the file under its own name:

- display_format: "smpte" (when added), "frames", "feet_frames_35mm" or "feet_frames_16mm"
- time_source: "layer_source" (when added), "composition" or "custom"
- time_units: 1 to 999 frames a second, 30 when added, taken down to a whole number; read with
  the custom source only
- drop_frame: "off" (when added) or "on"; read with the custom source only. "on" needs the time
  units to be a multiple of 30 (whatever the source), and a file that asks for it at other units
  is refused in a sentence
- starting_frame: -10,000,000 to 10,000,000, 0 when added, taken down to a whole number: the
  number the layer's first frame is given; read with the custom source only
- text_position: two numbers, per cent of the drawing's width and height, -1000 to 1000, 50, 85
  when added: the middle of the words' baseline
- text_size: 1 to 2000 pixels (the words' em), 36 when added
- text_color: "#ffffff" when added; box_color: "#000000" when added; encoded sRGB, "#rrggbb"
- show_box: "on" (when added) or "off"
- opacity: 0 to 100 per cent, the box's, 100 when added
- composite_on_original: "on" (when added) or "off"

The rule. At composition frame n, with the composition's frame rate fps (a fraction) and the
layer's in point `in`:

1. The count f (a whole number of frames) and the rate R it is labelled at:
   - layer_source: f = floor(s + 1e-6), s the layer's source time at n in frames (document 20,
     D-216: (n - in) x 100 / stretch + source offset, or its Time Remap); R = fps rounded to a
     whole number. A file's own starting timecode (a QuickTime timecode track, a DPX header) is
     not read: every source counts from its first frame, 0, and a layer with a file says so each
     frame (PROJECT_FEATURE_UNSUPPORTED, information). A solid, which has no file, counts from
     its in point; a composition layer from its source offset, through its stretch.
   - composition: f = n, the composition's own frame number (its start frame names its first
     frame); R as for layer_source.
   - custom: t = (u - in) / fps seconds, u the layer's key time at n (through its time stretch,
     as Numbers', D-526); f = starting_frame + floor(t x U + 1e-6), U the time units; R = U.
2. Drop frame. With the custom source, drop frame is the setting. With the other two it is on
   exactly when the composition's rate is an NTSC one, a denominator of 1001 with R a multiple
   of 30 (29.97, 59.94 and so on); 23.976 is counted non-drop at 24.
3. The string s, a minus in front when f < 0, worked on |f|:
   - smpte, non-drop: hh:mm:ss:ff at R frames a second, each two digits (the hours more when
     they pass 99).
   - smpte, drop frame: D = R / 15 frame numbers are left out at the start of every minute but
     each tenth; the frame f is relabelled f' = f + 9 D d + D floor((m - D) / (60 R - D)) when
     m > D, else f + 9 D d, with d = f div (600 R - 9 D) and m = f mod (600 R - 9 D); f' is
     written as non-drop but with a semicolon before the frames, hh:mm:ss;ff.
   - frames: |f| written with at least five digits, zeros in front.
   - feet_frames_35mm (16 frames a foot) and feet_frames_16mm (40 frames a foot): the feet with
     at least four digits, a plus, the frames left over with two, "0002+05".
4. Layout, in the drawing's pixels, P = text_position / 100 x the drawing's size. Every character
   sits in a cell as wide as the widest of the bundled font's digits (c, from its advances),
   centred in it, as Numbers' with proportional spacing off: the line is L = len(s) x c wide,
   starts at x0 = P.x - L / 2, and character i is its own left-aligned D-263 text in the bundled
   font, text_size and the text colour, at x0 + i x c + (c - w_i) / 2 on the baseline P.y.
5. The box, with show_box "on": the rectangle from x0 - p to x0 + L + p across and from
   P.y - k - p to P.y + p down, p = text_size / 5 and k the top of the font's "0" above its
   baseline at text_size (its outline's highest point, from the font's glyph table), each edge
   taken to the nearest whole pixel (a half up), filled with the box colour at the opacity, the
   words over it.
6. Composite on original "on": the box and the words over the layer as it was; "off": alone.

**This file never runs the build's code path.** It works each string with Python's integer
arithmetic and checks the drop-frame labels by counting them out one frame at a time, a second
way. The pixels are pinned the way Numbers' are: every case at every frame names a second
project, in `Fixtures/timecode/same/`, that draws the same box (a solid layer of that colour at
that opacity) and the same characters (text layers) and no Timecode at all; the case's frame
must match that project's within 2e-5. The strings are in `Fixtures/timecode/expected_timecode.json`
for reading.

Sources read: Adobe's "Text effects" page for After Effects (helpx.adobe.com, via the
web.archive.org copy), its Timecode section; AEJuice's "How to add timecode in After Effects"
(aejuice.com, 2025), which builds a counting timecode on a text layer from a Slider Control and an
expression instead (not reproduced: our expressions are their own, D-291); the ae-expressions
documentation's time conversion page (timeToTimecode, timeToNTSCTimecode, timeToFrames,
timeToFeetAndFrames: 30 frames a second by default, 16 frames a foot for 35mm); Chris Meyers'
expressions article on ProVideo Coalition (p=4062), where a composition's first frame is time 0.
16 frames a foot (35mm, four perforations) and 40 (16mm) are the film trade's own counts.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/timecode_reference.py   (with fontTools)
"""

import json
import math
from pathlib import Path

from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "timecode"
SAME = OUT / "same"
TOLERANCE = 2e-5  # document 25's default for a filter
FONT_NAME = "MPLUSRounded1c-Regular.ttf"
FONT = TTFont(str(ROOT / "assets" / "fonts" / FONT_NAME))
CMAP = FONT.getBestCmap()
HMTX = FONT["hmtx"]
UPEM = FONT["head"].unitsPerEm
ZERO_TOP = FONT["glyf"][CMAP[ord("0")]].yMax
W, H, FRAMES = 480, 160, 60
BACK = [0.02, 0.05, 0.12]

DEFAULTS = {
    "display_format": "smpte", "time_source": "layer_source", "time_units": 30,
    "drop_frame": "off", "starting_frame": 0, "text_position": [50, 85], "text_size": 36,
    "text_color": "#ffffff", "show_box": "on", "box_color": "#000000", "opacity": 100,
    "composite_on_original": "on",
}


# --- the rule -------------------------------------------------------------------------------

def count(c, n):
    """Rules 1 and 2: the frame count, its rate and whether it drops frames, at frame n."""
    num, den = c["fps"]
    fps = num / den
    nominal = round(fps)
    ntsc = den == 1001 and nominal % 30 == 0
    first = c["start"] + c["in"]
    if c["time_source"] == "layer_source":
        s = (n - first) * 100 / c["stretch"] + c["offset"]
        return math.floor(s + 1e-6), nominal, ntsc
    if c["time_source"] == "composition":
        return n, nominal, ntsc
    u = first + (n - first) * 100 / c["stretch"]
    t = (u - first) / fps
    units = math.floor(c["time_units"])
    return math.floor(c["starting_frame"]) + math.floor(t * units + 1e-6), units, c["drop_frame"] == "on"


def label(f, rate, drop, fmt):
    """Rule 3."""
    sign, f = ("-" if f < 0 else ""), abs(f)
    if fmt == "frames":
        return sign + str(f).rjust(5, "0")
    if fmt.startswith("feet_frames_"):
        per = 16 if fmt.endswith("35mm") else 40
        return sign + str(f // per).rjust(4, "0") + "+" + str(f % per).rjust(2, "0")
    sep = ":"
    if drop:
        d_ = rate // 15
        per_10 = 600 * rate - 9 * d_
        d, m = divmod(f, per_10)
        f += 9 * d_ * d + (d_ * ((m - d_) // (60 * rate - d_)) if m > d_ else 0)
        sep = ";"
    return sign + "%02d:%02d:%02d%s%02d" % (f // (rate * 3600), f // (rate * 60) % 60,
                                            f // rate % 60, sep, f % rate)


def shows(c, n):
    f, rate, drop = count(c, n)
    return label(f, rate, drop, c["display_format"])


def advance(ch, size):
    return HMTX[CMAP[ord(ch)]][0] * size / UPEM


def linear(hexcolour):
    out = []
    for i in (1, 3, 5):
        e = int(hexcolour[i:i + 2], 16) / 255
        out.append(e / 12.92 if e <= 0.04045 else ((e + 0.055) / 1.055) ** 2.4)
    return out


def layout(c, s):
    """Rules 4 and 5: the D-263 texts in the order drawn, and the box's edges."""
    size = c["text_size"]
    px = [c["text_position"][0] / 100 * W, c["text_position"][1] / 100 * H]
    cell = max(advance(ch, size) for ch in "0123456789")
    line = len(s) * cell
    x0 = px[0] - line / 2
    texts = [{"text": ch, "font": FONT_NAME, "size": size, "color": linear(c["text_color"]),
              "at": [x0 + i * cell + (cell - advance(ch, size)) / 2, px[1]], "align": "left"}
             for i, ch in enumerate(s)]
    p = size / 5
    k = ZERO_TOP * size / UPEM
    edge = lambda v: math.floor(v + 0.5)  # noqa: E731
    box = [edge(x0 - p), edge(px[1] - k - p), edge(x0 + line + p), edge(px[1] + p)]
    return texts, box


# --- the projects ---------------------------------------------------------------------------

def prop(base):
    return {"base": base, "keyframes": []}


def transform(at, anchor=None, opacity=1):
    return {"anchor": prop(anchor or at), "position": prop(at), "scale": prop([100, 100]),
            "rotation": prop(0), "opacity": prop(opacity)}


def base_layer(lid, kind, c):
    layer = {"id": lid, "kind": kind, "name": lid, "enabled": True, "locked": False,
             "in_frame": c["start"] + c["in"], "out_frame": c["start"] + FRAMES,
             "transform": transform([W / 2, H / 2]), "matte": None, "blend_mode": "normal",
             "effects": [], "masks": []}
    return layer


def solid(c, effects):
    layer = base_layer("plate", "solid", c)
    layer["solid"] = {"color": BACK, "width": W, "height": H}
    layer["effects"] = effects
    return layer


def nested(c):
    """Whether the case's layer is a composition layer, which a stretch or a source offset needs
    (D-216 and D-67: a solid carries neither)."""
    return c["stretch"] != 100 or c["offset"] != 0


def holder(c, effects):
    """The layer that carries the effect: the plate, or a composition layer showing it."""
    if not nested(c):
        return solid(c, effects)
    layer = base_layer("plate", "composition", c)
    layer["composition_id"] = "comp-inner"
    layer["source_offset_frames"] = c["offset"]
    if c["stretch"] != 100:
        layer["time_stretch"] = c["stretch"]
    layer["effects"] = effects
    return layer


def box_layer(c, box):
    l, t, r, b = box
    bw, bh = r - l, b - t
    layer = base_layer("box", "solid", {**c, "in": 0})
    layer["solid"] = {"color": linear(c["box_color"]), "width": bw, "height": bh}
    layer["transform"] = transform([l + bw / 2, t + bh / 2], [bw / 2, bh / 2], c["opacity"] / 100)
    return layer


def text_layer(lid, source, c):
    layer = base_layer(lid, "text", {**c, "in": 0})
    layer["source_text"] = source
    return layer


def composition(cid, layers, c):
    """`layers` bottom first, as a file lists them (document 19 draws `layer_order` bottom first)."""
    num, den = c["fps"]
    return {"id": cid, "name": cid, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": num, "denominator": den}, "start_frame": c["start"],
            "duration_frames": FRAMES,
            "work_area": {"start_frame": c["start"], "end_frame_exclusive": c["start"] + FRAMES},
            "layer_order": [l["id"] for l in layers], "layers": layers}


def project(pid, comps):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [], "compositions": comps}


def effect_json(c):
    return {"instance_id": "fx-1", "type_id": "core.timecode", "enabled": True,
            "parameters": {k: c[k] for k in DEFAULTS}}


def fixture(fx, c):
    comps = [composition("comp-main", [holder(c, [effect_json(c)])], c)]
    if nested(c):
        comps.append(composition("comp-inner", [solid({**c, "in": 0}, [])], c))
    return project("proj-" + fx.lower(), comps)


def same(fx, c, n):
    """Rules 4 to 6 as a solid and text layers: the project that must draw what the case draws."""
    main = [solid(c, [])] if c["composite_on_original"] == "on" or not c["valid"] else []
    if c["valid"]:
        texts, box = layout(c, shows(c, n))
        if c["show_box"] == "on":
            main.append(box_layer(c, box))
        main += [text_layer(f"char-{i}", t, c) for i, t in enumerate(texts)]
    return project(f"proj-{fx.lower()}-same-{n}", [composition("comp-main", main, c)])


def case(**settings):
    c = {**DEFAULTS, "in": 0, "start": 0, "fps": (24, 1), "stretch": 100, "offset": 0,
         "valid": True}
    c.update(settings)
    return c


# --- the cases ------------------------------------------------------------------------------

NTSC = (30000, 1001)
CASES = {
    "FX-TC-001": ("As added: the layer's own time in SMPTE on a black box, at frames 0 and 37 of "
                  "a 24-frame composition: 00:00:00:00 and 00:00:01:13.", case(), [0, 37]),
    "FX-TC-002": ("Layer source on a layer starting at frame 10: its source counts from there, "
                  "00:00:01:00 at frame 34.", case(**{"in": 10}), [34]),
    "FX-TC-003": ("Layer source on a composition layer stretched to 200 per cent: source frame "
                  "12.5 at frame 25, taken down to 00:00:00:12.", case(stretch=200), [25]),
    "FX-TC-004": ("Composition source on a layer starting at frame 10: the composition's frame, "
                  "00:00:01:10 at frame 34.", case(time_source="composition", **{"in": 10}), [34]),
    "FX-TC-005": ("Composition source in a composition starting at frame 86,400: an hour at 24, "
                  "01:00:00:05 at its sixth frame.", case(time_source="composition", start=86400),
                  [86405]),
    "FX-TC-006": ("Composition source at 29.97: drop frame, 00:00:59;29 then 00:01:00;02, the two "
                  "frame numbers of the new minute left out.",
                  case(time_source="composition", fps=NTSC, start=1790), [1799, 1800]),
    "FX-TC-007": ("Composition source at 29.97 at the tenth minute, where nothing is left out: "
                  "00:09:59;29 then 00:10:00;00.",
                  case(time_source="composition", fps=NTSC, start=17975), [17981, 17982]),
    "FX-TC-008": ("Composition source at 25: 00:00:01:05 at frame 30.",
                  case(time_source="composition", fps=(25, 1)), [30]),
    "FX-TC-009": ("Composition source at 23.976: counted non-drop at 24, 00:00:01:06 at frame 30.",
                  case(time_source="composition", fps=(24000, 1001)), [30]),
    "FX-TC-010": ("Custom at 30 time units in a 24-frame composition: half a second, 00:00:00:15, "
                  "at frame 12.", case(time_source="custom"), [12]),
    "FX-TC-011": ("Custom, drop frame at 30, starting frame 107,892: the layer's first frame is "
                  "01:00:00;00.", case(time_source="custom", drop_frame="on",
                                       starting_frame=107892), [0]),
    "FX-TC-012": ("Custom, drop frame at 60 time units, starting frame 3,600 (a minute of "
                  "frames, the first minute dropping none): 00:01:00;04, four numbers left out.",
                  case(time_source="custom", drop_frame="on", time_units=60, starting_frame=3600),
                  [0]),
    "FX-TC-013": ("Custom at 25 time units, starting frame -50: one second in, -00:00:01:00.",
                  case(time_source="custom", time_units=25, starting_frame=-50), [24]),
    "FX-TC-014": ("Custom on a composition layer stretched to 50 per cent counts by its key "
                  "time: 00:00:00:15 at frame 6.", case(time_source="custom", stretch=50), [6]),
    "FX-TC-015": ("Custom with time units 29.9, taken down to 29: one second in is 29 frames, "
                  "00:00:01:00, at frame 24.", case(time_source="custom", time_units=29.9), [24]),
    "FX-TC-016": ("Frames: 00037 at frame 37.", case(display_format="frames"), [37]),
    "FX-TC-017": ("Frames past five digits: 123456 as it is.",
                  case(display_format="frames", time_source="custom", starting_frame=123456), [0]),
    "FX-TC-018": ("Feet and frames of 35mm: 37 frames is 0002+05.",
                  case(display_format="feet_frames_35mm"), [37]),
    "FX-TC-019": ("Feet and frames of 16mm: frame 1,000 is 0025+00.",
                  case(display_format="feet_frames_16mm", time_source="custom",
                       starting_frame=1000), [0]),
    "FX-TC-020": ("No box: the words alone over the plate.", case(show_box="off"), [37]),
    "FX-TC-021": ("A blue box at 50 per cent under yellow words.",
                  case(box_color="#3050a0", opacity=50, text_color="#ffd040"), [37]),
    "FX-TC-022": ("Composite off: the box and the words alone on transparent.",
                  case(composite_on_original="off"), [37]),
    "FX-TC-023": ("Composite off and no box: the words alone on transparent.",
                  case(composite_on_original="off", show_box="off"), [37]),
    "FX-TC-024": ("Near the top left corner, size 28: the box runs off the picture's edges.",
                  case(text_position=[8, 12], text_size=28), [37]),
    "FX-TC-025": ("Size 80, in the middle.", case(text_position=[50, 70], text_size=80), [37]),
    "FX-TC-026": ("Box opacity 0: only the words show.", case(opacity=0), [37]),
    "FX-TC-027": ("Layer source on a composition layer starting at frame 10 with a source offset "
                  "of 48: 00:00:02:00 at frame 10.", case(offset=48, **{"in": 10}), [10]),
}

INVALID = {
    "FX-TC-040": ("Display format written \"seconds\".", case(display_format="seconds", valid=False)),
    "FX-TC-041": ("Time source written \"clock\".", case(time_source="clock", valid=False)),
    "FX-TC-042": ("Time units 0.", case(time_units=0, valid=False)),
    "FX-TC-043": ("Drop frame at 25 time units.", case(drop_frame="on", time_units=25, valid=False)),
    "FX-TC-044": ("Text colour written \"white\".", case(text_color="white", valid=False)),
    "FX-TC-045": ("Opacity 150.", case(opacity=150, valid=False)),
    "FX-TC-046": ("Starting frame 20,000,000.", case(starting_frame=20000000, valid=False)),
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    SAME.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    every = list(CASES.items()) + [(k, (s, c, [c["start"]])) for k, (s, c) in INVALID.items()]
    for fx, (says, c, frames) in every:
        file = fx.lower().replace("-", "_") + ".json"
        (OUT / file).write_text(json.dumps(fixture(fx, c), indent=2) + "\n", encoding="utf-8")
        entry = {"says": says, "project": file, "frames": {}}
        for n in frames:
            twin = f"same/{fx.lower().replace('-', '_')}_{n}.json"
            (OUT / twin).write_text(json.dumps(same(fx, c, n), indent=2) + "\n", encoding="utf-8")
            entry["frames"][str(n)] = {"same": twin, "shows": shows(c, n) if c["valid"] else None}
        if not c["valid"]:
            entry["says"] += (" The file is read, the effect is kept as written and left out of "
                              "every frame, with a warning.")
            entry["warning"] = "EFFECT_PARAMETER_INVALID"
        expected["cases"][fx] = entry
        print(fx, {n: v["shows"] for n, v in entry["frames"].items()})
    (OUT / "expected_timecode.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def counted_out(frames, rate):
    """Drop-frame labels the slow way: every label in turn, leaving out frames 0 to D - 1 at the
    start of each minute that is not a tenth."""
    d_ = rate // 15
    hh = mm = ss = ff = 0
    out = []
    for _ in range(frames):
        out.append("%02d:%02d:%02d;%02d" % (hh, mm, ss, ff))
        ff += 1
        if ff == rate:
            ff, ss = 0, ss + 1
            if ss == 60:
                ss, mm = 0, mm + 1
                if mm == 60:
                    mm, hh = 0, hh + 1
                if mm % 10:
                    ff = d_
    return out


def check(expected):
    """The claims the cases make, checked on the strings just worked."""
    s = {fx: {int(n): v["shows"] for n, v in e["frames"].items()} for fx, e in expected["cases"].items()}
    want = {
        "FX-TC-001": {0: "00:00:00:00", 37: "00:00:01:13"}, "FX-TC-002": {34: "00:00:01:00"},
        "FX-TC-003": {25: "00:00:00:12"}, "FX-TC-004": {34: "00:00:01:10"},
        "FX-TC-005": {86405: "01:00:00:05"},
        "FX-TC-006": {1799: "00:00:59;29", 1800: "00:01:00;02"},
        "FX-TC-007": {17981: "00:09:59;29", 17982: "00:10:00;00"},
        "FX-TC-008": {30: "00:00:01:05"}, "FX-TC-009": {30: "00:00:01:06"},
        "FX-TC-010": {12: "00:00:00:15"}, "FX-TC-011": {0: "01:00:00;00"},
        "FX-TC-012": {0: "00:01:00;04"}, "FX-TC-013": {24: "-00:00:01:00"},
        "FX-TC-014": {6: "00:00:00:15"}, "FX-TC-015": {24: "00:00:01:00"},
        "FX-TC-016": {37: "00037"}, "FX-TC-017": {0: "123456"}, "FX-TC-018": {37: "0002+05"},
        "FX-TC-019": {0: "0025+00"}, "FX-TC-027": {10: "00:00:02:00"},
    }
    for fx, frames in want.items():
        assert s[fx] == frames, (fx, s[fx], frames)
    # Every drop-frame label of the first 70 minutes at 30, and 12 at 60, counted out one by one.
    for rate, minutes in ((30, 70), (60, 12)):
        slow = counted_out(minutes * 60 * rate, rate)
        assert all(label(f, rate, True, "smpte") == slow[f] for f in range(len(slow))), rate
    # The box of the case as added, by hand: 11 characters of 22.32 pixels (620 units at 36),
    # 245.52 wide centred on 240, p 7.2, the 0 26.64 high above the baseline at 136.
    texts, box = layout(case(), "00:00:01:13")
    assert ZERO_TOP == 740 and advance("0", 1000) == 620, (ZERO_TOP, advance("0", 1000))
    assert box == [110, 102, 370, 143], box
    print("checks pass")


if __name__ == "__main__":
    main()
