# D-473: Basic Text

B-353: `core.basic_text`, after After Effects' Basic Text (Obsolete): words typed into the effect, drawn on the layer as Numbers draws its words (D-526), proportional spacing on. Every expected frame is a project in `Fixtures/basic_text/same/`, written by `tools/basic_text_reference.py` before this code existed and printed in document 25 as FX-BTEXT-001 to 025: the words the case must show, as text layers. The build's frame is compared sample by sample with that project's frame; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BTEXT-001 to 025 (document 25): each frame against the words it must show

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BTEXT-001 frame 0: As added: Text in white, centred, over the plate. Shows "Text" | largest difference 0.0e0 | yes |
| FX-BTEXT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-002 frame 0: Hello, world at size 48, left-aligned from 5 per cent. Shows "Hello, world" | largest difference 0.0e0 | yes |
| FX-BTEXT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-003 frame 0: Right-aligned at 95, 70 per cent, tracking 200. Shows "Spaced" | largest difference 0.0e0 | yes |
| FX-BTEXT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-004 frame 0: Stroke only, width 4: the ring outside the letters. Shows "Ring" | largest difference 0.0e0 | yes |
| FX-BTEXT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-005 frame 0: Fill over stroke, width 6. Shows "Edge" | largest difference 0.0e0 | yes |
| FX-BTEXT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-006 frame 0: Stroke over fill, width 6. Shows "Edge" | largest difference 6.0e-8 | yes |
| FX-BTEXT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-007 frame 0: Stroke only with width 0: nothing drawn. Shows "Gone" | largest difference 0.0e0 | yes |
| FX-BTEXT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-008 frame 0: Composite off, stroke over fill in blues: the words alone on transparent. Shows "Alone" | largest difference 0.0e0 | yes |
| FX-BTEXT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-009 frame 0: Two lines, Basic over Text, at size 40: the second a line below at the font's own line height. Shows "Basic\nText" | largest difference 0.0e0 | yes |
| FX-BTEXT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-010 frame 0: Kana in the bundled font: konnichiwa. Shows "こんにちは" | largest difference 0.0e0 | yes |
| FX-BTEXT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-011 frame 0: An empty text: nothing drawn, the plate as it was. Shows "" | largest difference 0.0e0 | yes |
| FX-BTEXT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-012 frame 0: Size keyed from 20 at frame 0 to 120 at frame 10: 70 at frame 5, 120 from frame 10 on. Shows "Grow" | largest difference 0.0e0 | yes |
| FX-BTEXT-012 frame 12: Size keyed from 20 at frame 0 to 120 at frame 10: 70 at frame 5, 120 from frame 10 on. Shows "Grow" | largest difference 0.0e0 | yes |
| FX-BTEXT-012 frame 5: Size keyed from 20 at frame 0 to 120 at frame 10: 70 at frame 5, 120 from frame 10 on. Shows "Grow" | largest difference 0.0e0 | yes |
| FX-BTEXT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-013 frame 4: Position keyed from 10, 50 per cent at frame 0 to 90, 50 at frame 8, left-aligned: 50, 50 at frame 4. Shows "Slide" | largest difference 0.0e0 | yes |
| FX-BTEXT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BTEXT-014 frame 0: A font this computer does not have: said each frame, nothing drawn, the plate as it was. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-014: what opening it warns of, and what frame 4 warns of | [] and ["TEXT_FONT_MISSING"] | yes |
| FX-BTEXT-020 frame 0: Display written "outline". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BTEXT-021 frame 0: A text of 1001 characters, past 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BTEXT-022 frame 0: Font written "../fonts/x.ttf", a path. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BTEXT-023 frame 0: Fill colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BTEXT-024 frame 0: Alignment written "justify". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BTEXT-025 frame 0: Size keyed to 3000 at frame 4, past 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-BTEXT-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_btext_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_btext_009.json is saved with its two lines as typed and no Numbers settings of its own | {"alignment":"center","composite_on_original":"on","display":"fill_only","fill_color":"#ffffff","font":"MPLUSRounded1c-Regular.ttf","position":[50,40],"size":40,"stroke_color":"#ff3300","stroke_width":3,"text":"Basic\nText","tracking":0} | yes |
| fx_btext_020.json is refused in a sentence naming it | Basic Text's display is one of fill_only, stroke_only, fill_over_stroke, stroke_over_fill, and this is "outline". | yes |
| fx_btext_021.json is refused in a sentence naming it | Basic Text's text is at most 1000 characters, and this is 1001. | yes |
| fx_btext_023.json is refused in a sentence naming it | Basic Text's fill colour is written #rrggbb, and this is "white". | yes |
| fx_btext_024.json is refused in a sentence naming it | Basic Text's alignment is one of left, center, right, and this is "justify". | yes |
| fx_btext_022.json is refused in a sentence naming the font | Basic Text needs a font's file name ending .ttf, .otf or .ttc, not "../fonts/x.ttf". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 0 is refused with a sentence, and nothing changes | Basic Text's size runs from 1 to 2000, and this is 0. | yes |
| a text of 1001 characters is refused with a sentence, and nothing changes | Basic Text's text is at most 1000 characters, and this is 1001. | yes |
| display "Fill Only", written as shown is refused with a sentence, and nothing changes | Basic Text's display is one of fill_only, stroke_only, fill_over_stroke, stroke_over_fill, and this is "Fill Only". | yes |
| stroke width keyed to 600 is refused with a sentence, and nothing changes | Basic Text's stroke width runs from 0 to 500, and this is 600. | yes |
| The words changed to Goodbye, is taken | taken | yes |
| Stroke over fill, width 5, right-aligned, is taken | taken | yes |
| size keyed from 20 to 200 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## A half-size draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| A half-size draft halves the size and the stroke, and keeps the position (a per cent) and tracking (a share of the size) | BasicText { text: "Hello, world", position: [5.0, 50.0], display: "fill_only", fill_color: "#ffffff", stroke_color: "#ff3300", stroke_width: 1.5, size: 24.0, tracking: 0.0, composite_on_original: "on", font: "MPLUSRounded1c-Regular.ttf", alignment: "left" } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_btext_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_btext_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_btext_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_btext_013.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_btext_001.json frame 0, Full | largest 1 level(s), 38 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_001.json frame 0, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_002.json frame 0, Full | largest 1 level(s), 57 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_002.json frame 0, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_003.json frame 0, Full | largest 1 level(s), 90 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_003.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_004.json frame 0, Full | largest 1 level(s), 79 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_004.json frame 0, Draft | largest 1 level(s), 5 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_005.json frame 0, Full | largest 1 level(s), 65 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_005.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_006.json frame 0, Full | largest 1 level(s), 36 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_006.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_008.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_009.json frame 0, Full | largest 1 level(s), 64 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_009.json frame 0, Draft | largest 1 level(s), 7 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_010.json frame 0, Full | largest 1 level(s), 72 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_010.json frame 0, Draft | largest 1 level(s), 6 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_012.json frame 0, Full | largest 1 level(s), 16 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_012.json frame 12, Full | largest 1 level(s), 106 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_012.json frame 12, Draft | largest 1 level(s), 8 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_012.json frame 5, Full | largest 1 level(s), 60 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_012.json frame 5, Draft | largest 1 level(s), 5 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_013.json frame 4, Full | largest 1 level(s), 28 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_013.json frame 4, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_btext_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "TEXT_FONT_MISSING" | yes |
| fx_btext_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "TEXT_FONT_MISSING" | yes |
| fx_btext_020.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_020.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_021.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_021.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_022.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_022.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_023.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_023.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_024.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_024.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_025.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_btext_025.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| a 480 by 160 picture, as added, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, as added, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, Hello, world, stroke over fill, size 40, from the left, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, Hello, world, stroke over fill, size 40, from the left, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, two lines, fill over stroke, composite off, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, two lines, fill over stroke, composite off, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, kana, stroke only, tracking 100, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, kana, stroke only, tracking 100, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: in `verification/D-473 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png, as added: the word Text in white in the middle of the plate | warnings "" | yes |
| 2_hello_world.png, Hello, world, smaller, from the left edge | warnings "" | yes |
| 3_spaced_right.png, Spaced, its letters set wide, ending at the right | warnings "" | yes |
| 4_stroke_only.png, Ring as an orange outline, the plate showing through the letters | warnings "" | yes |
| 5_stroke_over_fill.png, Edge in white with a thick orange edge | warnings "" | yes |
| 6_composite_off.png, Alone in light blue with a dark blue edge on transparent (shown on black) | warnings "" | yes |
| 7_two_lines.png, Basic above Text, two centred lines | warnings "" | yes |
| 8_kana.png, konnichiwa in kana | warnings "" | yes |
| 9_grow_frame_5.png, Grow at size 70, halfway through its size keys | warnings "" | yes |
| 10_missing_font.png, a font this computer does not have: the plain plate, and a warning | warnings "TEXT_FONT_MISSING" | yes |

## Result

138 of 138 checks pass.
