"""Text shaping, worked a second way.

D-371 (P0-7 part 2) shapes a text layer's words: Arabic letters join, Devanagari letters build
conjuncts and move the i-matra in front, a combining accent sits over its letter, and the font's
standard ligatures ("fi", "fl", "ff") are on, by the owner's choice of 2026-10-09 ("ligatures on
by default"). The build shapes with `rustybuzz` (D-354). This file is the reference for the
numbers document 25 pins, and it never runs the build's code: it shapes with HarfBuzz itself,
through `uharfbuzz` (0.53.3), and reads outlines and advances with fontTools.

The rule, as document 21 words it.

1. A paragraph (the words between line breaks) is cut into runs of one script and one direction,
   each shaped on its own with HarfBuzz's default features, kerning on or off as the text says,
   and `liga` and `clig` on. Here each case names its runs and the order they are drawn in, left
   to right, worked by hand from the bidirectional algorithm (UAX #9) for that line: in a
   left-to-right paragraph a right-to-left run is drawn where it stands; in a right-to-left
   paragraph (its first strong letter is right to left) the runs are drawn from the right, so
   the last run is leftmost. A space between two runs goes with the paragraph's direction
   (UAX #9 N1/N2), and figures after Arabic are a left-to-right run of their own (W2, then L1).
   A Common character (a space, a figure) takes the script of the run it is in (UAX #24).
2. A glyph's cluster is the paragraph's character it starts at, counted in characters. A cluster
   (a ligature, a letter and its marks, a conjunct) is one unit: it is drawn at its first
   character, which holds its anchor and its box; the other characters in it have no outline.
3. Layout (D-263, D-264, D-350): the pen starts at the text's place and moves, cluster by
   cluster in the order drawn, by the cluster's shaped advances (kern in them when kerning is
   on) and the tracking. Each glyph of a cluster is drawn at the pen plus the advances of the
   glyphs before it in the cluster plus its own offsets (offsets up the page in font units).
   The first character's anchor is on the baseline, halfway across the cluster's glyphs' own
   advances (the font's, before kerning), as D-350 puts a character's anchor.

Glyph numbers and positions are whole font units and pinned exactly. A box is held to 0.25
pixel, as D-350's are: it is found here from the curves exactly, where the build fills straight
pieces about 2 pixels long.

The fonts: the bundled M PLUS Rounded 1c (`assets/fonts`), and for Arabic and Devanagari, which it
lacks, Noto Sans Arabic 2.013 and Noto Sans Devanagari 2.007 (`Fixtures/text_shaping/fonts`, the
unhinted TTFs from the notofonts GitHub releases NotoSansArabic-v2.013 and
NotoSansDevanagari-v2.007, SIL Open Font License 1.1, `docs/third_party/NotoSans*-OFL.txt`).
Neither Noto font has Latin letters, so a Latin letter in a mixed line is the font's missing
glyph (number 0), as the build draws a letter a font lacks.

    python tools/text_shaping_reference.py
"""
import json
from pathlib import Path

import uharfbuzz as hb
from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "text_shaping"
BOX_TOLERANCE = 0.25
TOLERANCE = 1e-6

FONTS = {
    "MPLUSRounded1c-Regular.ttf": ROOT / "assets" / "fonts" / "MPLUSRounded1c-Regular.ttf",
    "NotoSansArabic-Regular.ttf": OUT / "fonts" / "NotoSansArabic-Regular.ttf",
    "NotoSansDevanagari-Regular.ttf": OUT / "fonts" / "NotoSansDevanagari-Regular.ttf",
}
M, AR, DV = "MPLUSRounded1c-Regular.ttf", "NotoSansArabic-Regular.ttf", "NotoSansDevanagari-Regular.ttf"

SALAM = "السلام عليكم"  # as-salamu alaykum
BISM = "بِسْمِ"  # bismi, with its three vowel marks
NAMASTE = "नमस्ते हिन्दी"  # namaste hindi
KSHA = "क्ष श्री"  # ksha, shri
LATIN = "office fly fit staff"
SALAM1 = "سلام"  # salam

# case: (says, font, words, kerning, runs drawn left to right as (first, end, script, direction))
CASES = {
    "FX-SHAPE-001": ("Arabic joins: \"as-salamu alaykum\" in Noto Sans Arabic, kerning off. One "
                     "right-to-left run; every letter takes its joined form, lam and alef in "
                     "\"salam\" as the lam-alef pair, drawn from the right.",
                     AR, SALAM, False, [(0, 12, "Arab", "rtl")]),
    "FX-SHAPE-002": ("The same with kerning on.", AR, SALAM, True, [(0, 12, "Arab", "rtl")]),
    "FX-SHAPE-003": ("Arabic vowel marks: \"bismi\" with kasra, sukun and kasra, each mark placed "
                     "over or under its letter by the font's offsets, with no width of its own.",
                     AR, BISM, False, [(0, 6, "Arab", "rtl")]),
    "FX-SHAPE-004": ("Devanagari: \"namaste hindi\". The conjunct sta is one glyph; the i-matra of "
                     "hi is drawn before its consonant; nda is a conjunct.",
                     DV, NAMASTE, False, [(0, 13, "Deva", "ltr")]),
    "FX-SHAPE-005": ("Devanagari: ksha and shri, each a conjunct.",
                     DV, KSHA, False, [(0, 8, "Deva", "ltr")]),
    "FX-SHAPE-006": ("Latin ligatures in the bundled font, kerning off: \"office fly fit staff\". "
                     "ffi, fl, fi and ff are each one glyph, held by their first letter.",
                     M, LATIN, False, [(0, 20, "Latn", "ltr")]),
    "FX-SHAPE-007": ("The same with kerning on (the font kerns \"ta\" and \"st\").",
                     M, LATIN, True, [(0, 20, "Latn", "ltr")]),
    "FX-SHAPE-008": ("A combining accent typed after its letter: \"cafe\" then U+0301. The font "
                     "has e-acute, so the two become that one glyph.",
                     M, "café", False, [(0, 5, "Latn", "ltr")]),
    "FX-SHAPE-009": ("Accents the font has no precomposed letter for: A with acute and diaeresis, "
                     "x with dot below and acute; each mark is a glyph of no width in its "
                     "letter's cluster.",
                     M, "Á̈ x̣́", False, [(0, 7, "Latn", "ltr")]),
    "FX-SHAPE-010": ("A mixed line, left-to-right paragraph: \"Hi salam\". \"Hi \" runs left to "
                     "right (the space between Hi and the Arabic takes the paragraph's side), then "
                     "the Arabic right to left where it stands. The Latin letters are the font's "
                     "missing glyph.",
                     AR, "Hi " + SALAM1, False, [(0, 3, "Latn", "ltr"), (3, 7, "Arab", "rtl")]),
    "FX-SHAPE-011": ("A mixed line, right-to-left paragraph: \"salam Hi\". The paragraph reads from "
                     "the right: the Arabic with the space after it (the paragraph's side) is "
                     "drawn on the right, Hi on the left.",
                     AR, SALAM1 + " Hi", False, [(5, 7, "Latn", "ltr"), (0, 5, "Arab", "rtl")]),
    "FX-SHAPE-012": ("Figures after Arabic, right-to-left paragraph: \"salam 2026\". The figures "
                     "read left to right and are drawn to the left of the Arabic; the space takes "
                     "the Arabic's side.",
                     AR, SALAM1 + " 2026", False, [(5, 9, "Arab", "ltr"), (0, 5, "Arab", "rtl")]),
}

# Layout checks, placed as the build places them: (says, shaping case, size, at, tracking).
LAYOUTS = {
    "FX-SHAPE-020": ("\"office fly fit staff\" laid out at 100 pixels from 100, 200 with tracking "
                     "50: each ligature's glyph drawn at its first letter, the other letters in it "
                     "with no outline.", "FX-SHAPE-006", 100.0, [100.0, 200.0], 50.0),
    "FX-SHAPE-021": ("\"as-salamu alaykum\" at 120 pixels from 80, 240: the first letter typed is on "
                     "the right, the last on the left.", "FX-SHAPE-001", 120.0, [80.0, 240.0], 0.0),
    "FX-SHAPE-022": ("\"bismi\" at 200 pixels from 60, 300: each vowel mark drawn over or under its "
                     "letter, its box inside the letter's cluster.", "FX-SHAPE-003", 200.0,
                     [60.0, 300.0], 0.0),
    "FX-SHAPE-023": ("\"namaste hindi\" at 120 pixels from 40, 200: the i-matra drawn before its "
                     "consonant, both in the cluster of the consonant typed first.", "FX-SHAPE-004",
                     120.0, [40.0, 200.0], 0.0),
}


def shape(font, words, kerning, runs):
    """Every glyph, left to right: [glyph, advance, x offset, y offset, character]."""
    hbfont = hb.Font(hb.Face(hb.Blob.from_file_path(str(FONTS[font]))))
    out = []
    for first, end, script, direction in runs:
        b = hb.Buffer()
        b.add_codepoints([ord(c) for c in words[first:end]])
        b.script = script
        b.direction = direction
        hb.shape(hbfont, b, {"kern": kerning, "liga": True, "clig": True})
        for info, pos in zip(b.glyph_infos, b.glyph_positions):
            out.append([info.codepoint, pos.x_advance, pos.x_offset, pos.y_offset, first + info.cluster])
    return out


def lay_out(font, words, glyphs, size, at, tracking):
    tt = TTFont(str(FONTS[font]))
    order, hmtx, gs = tt.getGlyphOrder(), tt["hmtx"], tt.getGlyphSet()
    scale = size / tt["head"].unitsPerEm
    clusters = []  # (character, [glyphs]) in the order drawn
    for g in glyphs:
        if clusters and clusters[-1][0] == g[4]:
            clusters[-1][1].append(g)
        else:
            clusters.append((g[4], [g]))
    heads = {c for c, _ in clusters}
    chars = {i: {"index": i, "char": words[i], "anchor": None, "box": None} for i in range(len(words))}
    x, y = at
    for ch, gl in clusters:
        own = sum(hmtx[order[g[0]]][0] for g in gl)
        box, gx = None, 0
        for gid, adv, xo, yo, _ in gl:
            pen = BoundsPen(gs)
            # Font units go up, pixels down.
            gs[order[gid]].draw(TransformPen(pen, (scale, 0, 0, -scale, x + (gx + xo) * scale, y - yo * scale)))
            if pen.bounds:
                b = pen.bounds
                box = list(b) if box is None else [min(box[0], b[0]), min(box[1], b[1]),
                                                   max(box[2], b[2]), max(box[3], b[3])]
            gx += adv
        chars[ch]["anchor"] = [x + own * scale / 2, y]
        chars[ch]["box"] = [round(v, 4) for v in box] if box else None
        x += gx * scale + tracking / 1000 * size
    for i, c in chars.items():
        c["head"] = i in heads
    return [chars[i] for i in range(len(words))]


def main():
    expected = {"tolerance": TOLERANCE, "box_tolerance": BOX_TOLERANCE, "shaping": {}, "layout": {}}
    for case, (says, font, words, kerning, runs) in CASES.items():
        glyphs = shape(font, words, kerning, runs)
        expected["shaping"][case] = {"says": says, "font": font, "text": words, "kerning": kerning,
                                     "runs_drawn_left_to_right": [list(r) for r in runs],
                                     "glyphs": glyphs}
        print(case, len(words), "chars", len(glyphs), "glyphs", [g[0] for g in glyphs])
    for case, (says, of, size, at, tracking) in LAYOUTS.items():
        s = expected["shaping"][of]
        expected["layout"][case] = {
            "says": says, "shaping": of,
            "text": {"text": s["text"], "font": s["font"], "size": size, "at": at,
                     "tracking": tracking, "kerning": s["kerning"]},
            "chars": lay_out(s["font"], s["text"], s["glyphs"], size, at, tracking)}
        print(case, [(c["index"], c["anchor"] and round(c["anchor"][0], 2)) for c in expected["layout"][case]["chars"]])
    (OUT / "expected_text_shaping.json").write_text(
        json.dumps(expected, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
