# B-406: Numbers

D-526: Numbers, after After Effects' Numbers (Text): a number, timecode, time, date or hexadecimal number drawn as words on the layer. Every expected frame is a project in `Fixtures/numbers/same/`, written by `tools/numbers_reference.py` before this code existed and printed in document 25 as FX-NUM-001 to 045: the words the case must show, as text layers. The build's frame is compared sample by sample with that project's frame; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-NUM-001 to 045 (document 25): each frame against the words it must show

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NUM-001 frame 0: As added: 0 in white, centred, over the plate. Shows "0" | largest difference 0.0e0 | yes |
| FX-NUM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-002 frame 0: 1234.5678 with two places: 1234.57. Shows "1234.57" | largest difference 0.0e0 | yes |
| FX-NUM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-003 frame 0: -12.5 with no places: -12, the tie to even. Shows "-12" | largest difference 0.0e0 | yes |
| FX-NUM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-004 frame 0: -0.004 with two places shows 0.00, its minus dropped. Shows "0.00" | largest difference 0.0e0 | yes |
| FX-NUM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-005 frame 0: Leading zeros, 42.75 with one place: 00042.8. Shows "00042.8" | largest difference 0.0e0 | yes |
| FX-NUM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-006 frame 0: Leading zeros, -7: -00007. Shows "-00007" | largest difference 0.0e0 | yes |
| FX-NUM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-007 frame 0: Decimal places 2.9 is taken down to 2: 3.14. Shows "3.14" | largest difference 0.0e0 | yes |
| FX-NUM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-008 frame 0: Timecode 30 at frames 0 and 50 of a 24-frame composition: 00:00:00:00 and 00:00:02:02. Shows "00:00:00:00" | largest difference 0.0e0 | yes |
| FX-NUM-008 frame 50: Timecode 30 at frames 0 and 50 of a 24-frame composition: 00:00:00:00 and 00:00:02:02. Shows "00:00:02:02" | largest difference 0.0e0 | yes |
| FX-NUM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-009 frame 50: Timecode 25 with value 3600: an hour on, 01:00:02:02 at frame 50. Shows "01:00:02:02" | largest difference 0.0e0 | yes |
| FX-NUM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-010 frame 34: Timecode 24 on a layer starting at frame 10: 00:00:01:00 at frame 34. Shows "00:00:01:00" | largest difference 0.0e0 | yes |
| FX-NUM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-011 frame 12: Timecode 30 with value -1 at frame 12: -00:00:00:15. Shows "-00:00:00:15" | largest difference 0.0e0 | yes |
| FX-NUM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-012 frame 0: Time 13.5: 13:30. Shows "13:30" | largest difference 0.0e0 | yes |
| FX-NUM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-013 frame 0: Time 25.25 wraps round the day: 01:15. Shows "01:15" | largest difference 0.0e0 | yes |
| FX-NUM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-014 frame 0: Time -0.5: 23:30. Shows "23:30" | largest difference 0.0e0 | yes |
| FX-NUM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-015 frame 0: Numerical date 0: 1/1/1995. Shows "1/1/1995" | largest difference 0.0e0 | yes |
| FX-NUM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-016 frame 0: Short date 10000: May 19, 2022. Shows "May 19, 2022" | largest difference 0.0e0 | yes |
| FX-NUM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-017 frame 0: Long date 365: Monday, January 1, 1996, left-aligned from 3 per cent. Shows "Monday, January 1, 1996" | largest difference 0.0e0 | yes |
| FX-NUM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-018 frame 0: Long date -1: Saturday, December 31, 1994. Shows "Saturday, December 31, 1994" | largest difference 0.0e0 | yes |
| FX-NUM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-019 frame 0: Hexadecimal 1.5: 18000. Shows "18000" | largest difference 0.0e0 | yes |
| FX-NUM-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-020 frame 0: Hexadecimal -0.25: -4000. Shows "-4000" | largest difference 0.0e0 | yes |
| FX-NUM-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-021 frame 0: Random values up to 100 with two places, a new number each frame. Shows "21.36" | largest difference 0.0e0 | yes |
| FX-NUM-021 frame 1: Random values up to 100 with two places, a new number each frame. Shows "7.37" | largest difference 0.0e0 | yes |
| FX-NUM-021 frame 7: Random values up to 100 with two places, a new number each frame. Shows "90.47" | largest difference 0.0e0 | yes |
| FX-NUM-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-022 frame 3: Random values with value 0: anywhere up to 30,000. Shows "9700" | largest difference 0.0e0 | yes |
| FX-NUM-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-023 frame 5: Random timecode 30 up to 60 seconds: a random time, not the layer's. Shows "00:00:02:28" | largest difference 0.0e0 | yes |
| FX-NUM-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-024 frame 12: A counter: value keyed from 0 at frame 0 to 100 at frame 10, 40 at frame 4 and 100 from frame 10 on. Shows "100" | largest difference 0.0e0 | yes |
| FX-NUM-024 frame 4: A counter: value keyed from 0 at frame 0 to 100 at frame 10, 40 at frame 4 and 100 from frame 10 on. Shows "40" | largest difference 0.0e0 | yes |
| FX-NUM-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-025 frame 0: Stroke only, width 4: the ring outside the letters. Shows "2026" | largest difference 0.0e0 | yes |
| FX-NUM-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-026 frame 0: Fill over stroke, width 6. Shows "2026" | largest difference 0.0e0 | yes |
| FX-NUM-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-027 frame 0: Stroke over fill, width 6. Shows "2026" | largest difference 6.0e-8 | yes |
| FX-NUM-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-028 frame 0: Stroke only with width 0: nothing drawn. Shows "2026" | largest difference 0.0e0 | yes |
| FX-NUM-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-029 frame 0: Monospaced 1.11 with tracking 100: each character in a digit's cell. Shows "1.11" | largest difference 0.0e0 | yes |
| FX-NUM-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-030 frame 37: Monospaced timecode 25, fill over stroke, left-aligned from 5 per cent: the colons in cells as wide as the digits. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-NUM-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-031 frame 0: Monospaced, stroke only, right-aligned at 95 per cent. Shows "-3.5" | largest difference 0.0e0 | yes |
| FX-NUM-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-032 frame 0: Monospaced, stroke over fill, composite off: the words alone on transparent. Shows "88" | largest difference 0.0e0 | yes |
| FX-NUM-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-033 frame 0: Composite off, right-aligned at 90, 70 per cent, size 40, tracking -50. Shows "512" | largest difference 0.0e0 | yes |
| FX-NUM-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NUM-034 frame 0: A font this computer does not have: said each frame, nothing drawn, the plate as it was. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-034: what opening it warns of, and what frame 4 warns of | [] and ["TEXT_FONT_MISSING"] | yes |
| FX-NUM-040 frame 0: Type written "roman". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NUM-041 frame 0: Display written "outline". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NUM-042 frame 0: Value 40,000, past 30,000. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NUM-043 frame 0: Decimal places 11. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NUM-044 frame 0: Font written "../fonts/x.ttf", a path. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NUM-045 frame 0: Fill colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-NUM-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_num_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_num_001.json: the clock worked each frame is never saved; the type is saved as type | {"alignment":"center","composite_on_original":"on","decimal_places":0,"display":"fill_only","fill_color":"#ffffff","font":"MPLUSRounded1c-Regular.ttf","position":[50,50],"proportional_spacing":"on","random_values":"off","size":64,"stroke_color":"#ff3300","stroke_width":3,"tracking":0,"type":"number","value":0} | yes |
| fx_num_024.json: the clock worked each frame is never saved; the type is saved as type | {"alignment":"center","composite_on_original":"on","decimal_places":0,"display":"fill_only","fill_color":"#ffffff","font":"MPLUSRounded1c-Regular.ttf","position":[50,50],"proportional_spacing":"on","random_values":"off","size":64,"stroke_color":"#ff3300","stroke_width":3,"tracking":0,"type":"number","value":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":10,"interp":"linear","value":100}]}} | yes |
| fx_num_040.json is refused in a sentence naming it | Numbers's type is one of number, number_leading_zeros, timecode_30, timecode_25, timecode_24, time, numerical_date, short_date, long_date, hexadecimal, and this is "roman". | yes |
| fx_num_041.json is refused in a sentence naming it | Numbers's display is one of fill_only, stroke_only, fill_over_stroke, stroke_over_fill, and this is "outline". | yes |
| fx_num_042.json is refused in a sentence naming it | Numbers's value runs from -30000 to 30000, and this is 40000. | yes |
| fx_num_043.json is refused in a sentence naming it | Numbers's decimal places runs from 0 to 10, and this is 11. | yes |
| fx_num_045.json is refused in a sentence naming it | Numbers's fill colour is written #rrggbb, and this is "white". | yes |
| fx_num_044.json is refused in a sentence naming the font | Numbers needs a font's file name ending .ttf, .otf or .ttc, not "../fonts/x.ttf". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Size 0 is refused with a sentence, and nothing changes | Numbers's size runs from 1 to 2000, and this is 0. | yes |
| Type Long Date, value 365, is taken | taken | yes |
| Stroke over fill, width 5, monospaced, right-aligned, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_num_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_num_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_num_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_num_030.json frame 37 in tiles of 1 and of 64 | byte-identical | yes |
| fx_num_032.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_num_001.json frame 0, Full | largest 1 level(s), 20 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_001.json frame 0, Draft | largest 1 level(s), 2 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_002.json frame 0, Full | largest 1 level(s), 90 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_002.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_003.json frame 0, Full | largest 1 level(s), 19 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_004.json frame 0, Full | largest 1 level(s), 58 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_004.json frame 0, Draft | largest 1 level(s), 5 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_005.json frame 0, Full | largest 1 level(s), 105 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_005.json frame 0, Draft | largest 1 level(s), 8 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_006.json frame 0, Full | largest 1 level(s), 75 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_006.json frame 0, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_007.json frame 0, Full | largest 1 level(s), 50 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_008.json frame 0, Full | largest 1 level(s), 162 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_008.json frame 0, Draft | largest 1 level(s), 10 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_008.json frame 50, Full | largest 1 level(s), 152 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_008.json frame 50, Draft | largest 1 level(s), 10 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_009.json frame 50, Full | largest 1 level(s), 136 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_009.json frame 50, Draft | largest 1 level(s), 9 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_010.json frame 34, Full | largest 1 level(s), 144 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_010.json frame 34, Draft | largest 1 level(s), 9 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_011.json frame 12, Full | largest 1 level(s), 133 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_011.json frame 12, Draft | largest 1 level(s), 7 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_012.json frame 0, Full | largest 1 level(s), 54 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_012.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_013.json frame 0, Full | largest 1 level(s), 48 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_013.json frame 0, Draft | largest 1 level(s), 2 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_014.json frame 0, Full | largest 1 level(s), 61 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_014.json frame 0, Draft | largest 1 level(s), 5 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_015.json frame 0, Full | largest 1 level(s), 91 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_015.json frame 0, Draft | largest 1 level(s), 2 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_016.json frame 0, Full | largest 1 level(s), 151 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_016.json frame 0, Draft | largest 1 level(s), 8 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_017.json frame 0, Full | largest 1 level(s), 118 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_017.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_018.json frame 0, Full | largest 1 level(s), 145 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_018.json frame 0, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_019.json frame 0, Full | largest 1 level(s), 81 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_019.json frame 0, Draft | largest 1 level(s), 6 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_020.json frame 0, Full | largest 1 level(s), 63 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_020.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_021.json frame 0, Full | largest 1 level(s), 60 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_021.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_021.json frame 1, Full | largest 1 level(s), 55 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_021.json frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_021.json frame 7, Full | largest 1 level(s), 60 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_021.json frame 7, Draft | largest 1 level(s), 5 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_022.json frame 3, Full | largest 1 level(s), 59 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_022.json frame 3, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_023.json frame 5, Full | largest 1 level(s), 157 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_023.json frame 5, Draft | largest 1 level(s), 11 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_024.json frame 12, Full | largest 1 level(s), 43 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_024.json frame 12, Draft | largest 1 level(s), 2 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_024.json frame 4, Full | largest 1 level(s), 32 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_024.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_025.json frame 0, Full | largest 1 level(s), 133 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_025.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_026.json frame 0, Full | largest 1 level(s), 126 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_026.json frame 0, Draft | largest 1 level(s), 2 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_027.json frame 0, Full | largest 1 level(s), 82 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_027.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_028.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_028.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_029.json frame 0, Full | largest 1 level(s), 17 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_029.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_030.json frame 37, Full | largest 1 level(s), 210 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_030.json frame 37, Draft | largest 1 level(s), 4 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_031.json frame 0, Full | largest 1 level(s), 74 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_031.json frame 0, Draft | largest 1 level(s), 2 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_032.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_032.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_033.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_033.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_num_034.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "TEXT_FONT_MISSING" | yes |
| fx_num_034.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "TEXT_FONT_MISSING" | yes |
| fx_num_040.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_040.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_041.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_041.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_042.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_042.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_043.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_043.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_044.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_044.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_045.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_num_045.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| a 480 by 160 picture, as added, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, as added, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, timecode 30, monospaced, fill over stroke, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, timecode 30, monospaced, fill over stroke, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, long date, size 30, stroke over fill, composite off, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, long date, size 30, stroke over fill, composite off, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, random values with two places, stroke only, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, random values with two places, stroke only, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: in `verification/D-526 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| as_added.png, as added: a white 0 in the middle of the dark blue plate | warnings "" | yes |
| two_places.png, 1234.57 | warnings "" | yes |
| timecode_30.png, the layer's time as 30-frame timecode at frame 50: 00:00:02:02 | warnings "" | yes |
| long_date.png, Monday, January 1, 1996, smaller, from the left | warnings "" | yes |
| hexadecimal.png, 18000, 1.5 in hexadecimal | warnings "" | yes |
| random.png, a random number up to 100 with two places at frame 7: 90.47 | warnings "" | yes |
| counter_frame_4.png, the counter keyed from 0 to 100, at frame 4: 40 | warnings "" | yes |
| stroke_only.png, 2026 as an orange ring round where the letters would be, the plate showing through them | warnings "" | yes |
| fill_over_stroke.png, 2026 in white with a thick orange edge | warnings "" | yes |
| stroke_over_fill.png, 2026 in white with a thick orange edge, as fill over stroke (D-264's stroke lies outside the letters) | warnings "" | yes |
| monospaced.png, 1.11 with every character, the point too, in a cell as wide as a digit | warnings "" | yes |
| monospaced_timecode.png, 00:00:01:13 in 25-frame timecode, monospaced, with an orange edge, from the left | warnings "" | yes |
| composite_off.png, a light blue 88 with a dark blue edge on transparent (shown on black) | warnings "" | yes |
| missing_font.png, a font this computer does not have: the plain plate, and a warning | warnings "TEXT_FONT_MISSING" | yes |

## Result

252 of 252 checks pass.
