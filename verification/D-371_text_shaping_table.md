# B-250: text shaping

D-371 (EFFECTS.md P0-7, part 2): a text layer's words shaped by `rustybuzz` (D-354), ligatures on by the owner's choice of 2026-10-09. Every expected number is `Fixtures/text_shaping/expected_text_shaping.json`, written by `tools/text_shaping_reference.py` with HarfBuzz itself (`uharfbuzz` 0.53.3) and fontTools before this code was committed, and printed in document 25 as FX-SHAPE-001 to 023. The shaping checks compare every glyph, left to right: its number in the font, its advance, its offsets and the character its cluster starts at, all whole font units, exactly. The layout checks compare each cluster's anchor within 1e-6 pixel and its drawn box within a quarter of a pixel (the build cuts curves into pieces about 2 pixels long; the reference takes each curve's exact extent), and that every other character of a cluster draws nothing of its own.

Both the reference and the build slide each TrueType outline so its left edge meets the side bearing the font's hmtx table gives, as FreeType and HarfBuzz do: D-372 (a), the owner's choice of 2026-10-09, built B-251. Before it, FX-SHAPE-020 was in dispute, its "y" 0.4 pixel from the reference's.

## FX-SHAPE-001 to 012: every glyph against HarfBuzz (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHAPE-001: Arabic joins: "as-salamu alaykum" in Noto Sans Arabic, kerning off. One right-to-left run; every letter takes its joined form, lam and alef in "salam" as the lam-alef pair, drawn from the right. | 13 glyphs, all the same: 77 60 318 18 70 49 3 74 11 71 36 72 8 | yes |
| FX-SHAPE-002: The same with kerning on. | 13 glyphs, all the same: 77 60 318 18 70 49 3 74 11 71 36 72 8 | yes |
| FX-SHAPE-003: Arabic vowel marks: "bismi" with kasra, sukun and kasra, each mark placed over or under its letter by the font's offsets, with no width of its own. | 7 glyphs, all the same: 426 77 378 36 426 316 19 | yes |
| FX-SHAPE-004: Devanagari: "namaste hindi". The conjunct sta is one glyph; the i-matra of hi is drawn before its consonant; nda is a conjunct. | 11 glyphs, all the same: 75 80 256 71 40 3 544 88 245 73 33 | yes |
| FX-SHAPE-005: Devanagari: ksha and shri, each a conjunct. | 4 glyphs, all the same: 90 3 317 33 | yes |
| FX-SHAPE-006: Latin ligatures in the bundled font, kerning off: "office fly fit staff". ffi, fl, fi and ff are each one glyph, held by their first letter. | 15 glyphs, all the same: 80 8015 68 70 1 8014 90 1 8013 85 1 84 85 66 8012 | yes |
| FX-SHAPE-007: The same with kerning on (the font kerns "ta" and "st"). | 15 glyphs, all the same: 80 8015 68 70 1 8014 90 1 8013 85 1 84 85 66 8012 | yes |
| FX-SHAPE-008: A combining accent typed after its letter: "cafe" then U+0301. The font has e-acute, so the two become that one glyph. | 4 glyphs, all the same: 68 66 71 169 | yes |
| FX-SHAPE-009: Accents the font has no precomposed letter for: A with acute and diaeresis, x with dot below and acute; each mark is a glyph of no width in its letter's cluster. | 6 glyphs, all the same: 129 712 1 89 739 705 | yes |
| FX-SHAPE-010: A mixed line, left-to-right paragraph: "Hi salam". "Hi " runs left to right (the space between Hi and the Arabic takes the paragraph's side), then the Arabic right to left where it stands. The Latin letters are the font's missing glyph. | 7 glyphs, all the same: 0 0 3 74 11 71 37 | yes |
| FX-SHAPE-011: A mixed line, right-to-left paragraph: "salam Hi". The paragraph reads from the right: the Arabic with the space after it (the paragraph's side) is drawn on the right, Hi on the left. | 7 glyphs, all the same: 0 0 3 74 11 71 37 | yes |
| FX-SHAPE-012: Figures after Arabic, right-to-left paragraph: "salam 2026". The figures read left to right and are drawn to the left of the Arabic; the space takes the Arabic's side. | 9 glyphs, all the same: 120 118 120 124 3 74 11 71 37 | yes |

## FX-SHAPE-020 to 023: laid out and drawn (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHAPE-020: "office fly fit staff" laid out at 100 pixels from 100, 200 with tracking 50: each ligature's glyph drawn at its first letter, the other letters in it with no outline. | 20 characters, 15 clusters; anchors within 1.1e-13, boxes within 0.023 px | yes |
| FX-SHAPE-021: "as-salamu alaykum" at 120 pixels from 80, 240: the first letter typed is on the right, the last on the left. | 12 characters, 12 clusters; anchors within 5.7e-14, boxes within 0.006 px | yes |
| FX-SHAPE-022: "bismi" at 200 pixels from 60, 300: each vowel mark drawn over or under its letter, its box inside the letter's cluster. | 6 characters, 3 clusters; anchors within 5.7e-14, boxes within 0.000 px | yes |
| FX-SHAPE-023: "namaste hindi" at 120 pixels from 40, 200: the i-matra drawn before its consonant, both in the cluster of the consonant typed first. | 13 characters, 8 clusters; anchors within 5.7e-14, boxes within 0.000 px | yes |

## Result

16 of 16 checks pass.
