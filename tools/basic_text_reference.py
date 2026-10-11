"""Basic Text, worked a second way.

D-473 adds `core.basic_text`, Basic Text, after After Effects' Basic Text (Obsolete), on EFFECTS.md
row 397: words typed into the effect, drawn on the layer. Adobe's "Text effects" page names the
settings (Font, Style, Direction, Alignment; Position, Display Options, Fill Color, Stroke Color,
Stroke Width, Size, Tracking, Line Spacing, Composite On Original) and gives no layout, colours or
formulas. Numbers (D-526) draws its words with every one of those settings but the words
themselves, so Basic Text is Numbers' words rule with the words typed: rules 3 to 7 of
`tools/numbers_reference.py`, proportional spacing on, the string s the setting `text`.

The settings, each in the file under its own name:

- text: the words, up to 1000 characters, "Text" when added; a line break starts a new line
  below at the font's own line height (D-263)
- position: two numbers, per cent of the drawing's width and height, -1000 to 1000, 50, 50 when
  added: where the first line's baseline meets its aligned end; keyable
- display: "fill_only" (when added), "stroke_only", "fill_over_stroke" or "stroke_over_fill"
- fill_color, "#ffffff" when added; stroke_color, "#ff3300" when added; encoded sRGB, "#rrggbb"
- stroke_width: 0 to 500 pixels, 3 when added; keyable
- size: 1 to 2000 pixels (the words' em), 64 when added; keyable
- tracking: thousandths of the size, -1000 to 1000, added after each character, 0 when added;
  keyable
- composite_on_original: "on" (when added) or "off"
- font: a font file's name as text layers name one (D-263), the bundled
  "MPLUSRounded1c-Regular.ttf" when added
- alignment: "left", "center" (when added) or "right"

The values when added are Numbers' (D-526), so the two agree; After Effects' own are not
published here. Not built, and said so in EFFECTS.md: Style (a font file names its style, as a
text layer's does), Direction (Vertical and Rotate: our text engine lays words across only) and
Line Spacing (After Effects' unit is not published; the font's own line height is used).

An empty text draws nothing: the layer as it was with composite on original "on", nothing with it
"off". A font this computer does not have is said each frame as `TEXT_FONT_MISSING` and nothing is
drawn, the layer left as it was (D-263).

**This file never runs the build's code path.** The pixels are pinned the way D-526 pins Numbers':
every case at every frame names a second project, in `Fixtures/basic_text/same/`, that draws the
same words from text layers (numbers_reference's `same`, with the words typed) and no Basic Text
at all; the case's frame must match that project's within 2e-5.

Sources read: Adobe's "Obsolete effects" page for After Effects (helpx.adobe.com, read through
search excerpts; the page itself refuses this computer): Basic Text is kept for projects made in
older versions, places text on an existing layer (unlike a text layer, which is its own layer),
works in 8 bits a channel, and text layers are advised for new work; Adobe's "Text effects" page,
its Basic Text section, for the names of the settings (through the Numbers lane's reading, D-526).
No tutorial on Basic Text itself was found (the old Adobe and O'Reilly chapters refuse this
computer); the cases below type the words and key Size and Position, as one animates any text.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/basic_text_reference.py   (with fontTools)
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import numbers_reference as N  # noqa: E402

OUT = N.ROOT / "Fixtures" / "basic_text"
SAME = OUT / "same"
W, H, FRAMES = N.W, N.H, N.FRAMES

DEFAULTS = {
    "text": "Text", "position": [50, 50], "display": "fill_only", "fill_color": "#ffffff",
    "stroke_color": "#ff3300", "stroke_width": 3, "size": 64, "tracking": 0,
    "composite_on_original": "on", "font": N.FONT_NAME, "alignment": "center",
}
KEYABLE = ("position", "stroke_width", "size", "tracking")

# Numbers' words rule with the words typed: `same` draws what `shows` gives.
N.shows = lambda c, n: c["text"]


def case(**settings):
    c = {**DEFAULTS, "proportional_spacing": "on", "in": 0, "valid": True}
    c.update(settings)
    return c


def value_at(v, n):
    """A keyable setting at frame n: a number or a pair, or keys as (frame, value) pairs, linear
    between."""
    if not isinstance(v, dict):
        return v
    keys = v["keys"]
    if n <= keys[0][0]:
        return keys[0][1]
    for (f0, v0), (f1, v1) in zip(keys, keys[1:]):
        if n <= f1:
            k = (n - f0) / (f1 - f0)
            if isinstance(v0, list):
                return [a + (b - a) * k for a, b in zip(v0, v1)]
            return v0 + (v1 - v0) * k
    return keys[-1][1]


def at(c, n):
    """The case with its keyed settings worked at frame n."""
    return {**c, **{k: value_at(c[k], n) for k in KEYABLE}}


def effect_json(c):
    return {"instance_id": "fx-1", "type_id": "core.basic_text", "enabled": True,
            "parameters": {k: N.setting_json(c[k]) for k in DEFAULTS}}


def fixture(fx, c):
    return N.project("proj-" + fx.lower(),
                     [N.composition("comp-main", [N.solid(c, [effect_json(c)])])])


def same(fx, c, n):
    c = at(c, n)
    if c["text"] == "":
        # Nothing to draw: the plate, or nothing at all.
        main = [N.solid(c, [])] if c["composite_on_original"] == "on" else []
        return N.project(f"proj-{fx.lower()}-same-{n}", [N.composition("comp-main", main)])
    return N.same(fx, c, n)


CASES = {
    "FX-BTEXT-001": ("As added: Text in white, centred, over the plate.", case(), [0]),
    "FX-BTEXT-002": ("Hello, world at size 48, left-aligned from 5 per cent.",
                     case(text="Hello, world", size=48, alignment="left", position=[5, 50]), [0]),
    "FX-BTEXT-003": ("Right-aligned at 95, 70 per cent, tracking 200.",
                     case(text="Spaced", alignment="right", position=[95, 70], tracking=200), [0]),
    "FX-BTEXT-004": ("Stroke only, width 4: the ring outside the letters.",
                     case(text="Ring", display="stroke_only", stroke_width=4), [0]),
    "FX-BTEXT-005": ("Fill over stroke, width 6.",
                     case(text="Edge", display="fill_over_stroke", stroke_width=6), [0]),
    "FX-BTEXT-006": ("Stroke over fill, width 6.",
                     case(text="Edge", display="stroke_over_fill", stroke_width=6), [0]),
    "FX-BTEXT-007": ("Stroke only with width 0: nothing drawn.",
                     case(text="Gone", display="stroke_only", stroke_width=0), [0]),
    "FX-BTEXT-008": ("Composite off, stroke over fill in blues: the words alone on transparent.",
                     case(text="Alone", composite_on_original="off", display="stroke_over_fill",
                          fill_color="#20c0ff", stroke_color="#102040"), [0]),
    "FX-BTEXT-009": ("Two lines, Basic over Text, at size 40: the second a line below at the "
                     "font's own line height.", case(text="Basic\nText", size=40,
                                                      position=[50, 40]), [0]),
    "FX-BTEXT-010": ("Kana in the bundled font: konnichiwa.", case(text="こんにちは"),
                     [0]),
    "FX-BTEXT-011": ("An empty text: nothing drawn, the plate as it was.", case(text=""), [0]),
    "FX-BTEXT-012": ("Size keyed from 20 at frame 0 to 120 at frame 10: 70 at frame 5, 120 from "
                     "frame 10 on.", case(text="Grow", size={"keys": [(0, 20), (10, 120)]}),
                     [0, 5, 12]),
    "FX-BTEXT-013": ("Position keyed from 10, 50 per cent at frame 0 to 90, 50 at frame 8, "
                     "left-aligned: 50, 50 at frame 4.",
                     case(text="Slide", alignment="left", size=40,
                          position={"keys": [(0, [10, 50]), (8, [90, 50])]}), [4]),
    "FX-BTEXT-014": ("A font this computer does not have: said each frame, nothing drawn, the "
                     "plate as it was.", case(font="NotAFontAnywhere.ttf"), [0]),
}

INVALID = {
    "FX-BTEXT-020": ("Display written \"outline\".", case(display="outline", valid=False)),
    "FX-BTEXT-021": ("A text of 1001 characters, past 1000.", case(text="x" * 1001, valid=False)),
    "FX-BTEXT-022": ("Font written \"../fonts/x.ttf\", a path.", case(font="../fonts/x.ttf", valid=False)),
    "FX-BTEXT-023": ("Fill colour written \"white\".", case(fill_color="white", valid=False)),
    "FX-BTEXT-024": ("Alignment written \"justify\".", case(alignment="justify", valid=False)),
    "FX-BTEXT-025": ("Size keyed to 3000 at frame 4, past 2000.",
                     case(size={"keys": [(0, 64), (4, 3000)]}, valid=False)),
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    SAME.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": N.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in list(CASES.items()) + [(k, (s, c, [0])) for k, (s, c) in INVALID.items()]:
        file = fx.lower().replace("-", "_") + ".json"
        (OUT / file).write_text(json.dumps(fixture(fx, c), indent=2) + "\n", encoding="utf-8")
        entry = {"says": says, "project": file, "frames": {}}
        for n in frames:
            twin = f"same/{fx.lower().replace('-', '_')}_{n}.json"
            (OUT / twin).write_text(json.dumps(same(fx, c, n), indent=2) + "\n", encoding="utf-8")
            entry["frames"][str(n)] = {"same": twin, "shows": c["text"] if N.works(c) else None}
        if not c["valid"]:
            entry["says"] += (" The file is read, the effect is kept as written and left out of "
                              "every frame, with a warning.")
            entry["warning"] = "EFFECT_PARAMETER_INVALID"
        elif not N.works(c):
            entry["frame_warning"] = "TEXT_FONT_MISSING"
        expected["cases"][fx] = entry
        print(fx, ascii({n: v["shows"] for n, v in entry["frames"].items()}))
    (OUT / "expected_basic_text.json").write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the projects just written."""
    load = lambda name: json.loads((OUT / name).read_text(encoding="utf-8"))  # noqa: E731
    texts = lambda p: [l["source_text"] for comp in p["compositions"] for l in comp["layers"]  # noqa: E731
                       if l["kind"] == "text"]
    cases = expected["cases"]
    one = texts(load(cases["FX-BTEXT-001"]["frames"]["0"]["same"]))
    assert len(one) == 1 and one[0]["text"] == "Text" and one[0]["at"] == [240, 80], one
    # Keyed size: 70 halfway, 120 after the last key.
    grow = cases["FX-BTEXT-012"]["frames"]
    assert [texts(load(grow[n]["same"]))[0]["size"] for n in ("0", "5", "12")] == [20, 70, 120]
    slide = texts(load(cases["FX-BTEXT-013"]["frames"]["4"]["same"]))[0]
    assert slide["at"] == [240, 80] and slide["align"] == "left", slide
    # Stroke only with no stroke, the empty text and the missing font show the plate alone.
    for fx in ("FX-BTEXT-007", "FX-BTEXT-011", "FX-BTEXT-014"):
        main = load(cases[fx]["frames"]["0"]["same"])["compositions"][0]
        assert main["layer_order"] == ["plate"], fx
    # Two pictures for the three that stroke.
    for fx in ("FX-BTEXT-004", "FX-BTEXT-005", "FX-BTEXT-006", "FX-BTEXT-008"):
        assert len(texts(load(cases[fx]["frames"]["0"]["same"]))) == 2, fx
    assert texts(load(cases["FX-BTEXT-009"]["frames"]["0"]["same"]))[0]["text"] == "Basic\nText"
    print("checks pass")


if __name__ == "__main__":
    main()
