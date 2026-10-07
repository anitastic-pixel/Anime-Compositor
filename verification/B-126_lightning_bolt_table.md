# B-126: lightning bolt

D-190, accepted by the owner on 2026-09-28. Every expected pixel is `Fixtures/lightning_bolt/expected_lightning_bolt.json`, written by `tools/lightning_bolt_reference.py` before this code existed and printed in document 25 as FX-BOLT-001 to 032. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BOLT-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BOLT-001 frame 0: The settings as they start: from (40, 0) to (60, 100) per cent, jaggedness 40, detail 6, branches 30, width 3, glow 24, white in a blue glow #6e8cff, opacity 100, hold 2, seed 0. On a frame this small the glow reaches every pixel. Frames 0 and 1 are one bolt, held for two frames; frame 2 is a new one. | largest difference 6.6e-8 | yes |
| FX-BOLT-001 frame 1: The settings as they start: from (40, 0) to (60, 100) per cent, jaggedness 40, detail 6, branches 30, width 3, glow 24, white in a blue glow #6e8cff, opacity 100, hold 2, seed 0. On a frame this small the glow reaches every pixel. Frames 0 and 1 are one bolt, held for two frames; frame 2 is a new one. | largest difference 6.6e-8 | yes |
| FX-BOLT-001 frame 2: The settings as they start: from (40, 0) to (60, 100) per cent, jaggedness 40, detail 6, branches 30, width 3, glow 24, white in a blue glow #6e8cff, opacity 100, hold 2, seed 0. On a frame this small the glow reaches every pixel. Frames 0 and 1 are one bolt, held for two frames; frame 2 is a new one. | largest difference 6.3e-8 | yes |
| FX-BOLT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-002 frame 0: Opacity 0: the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-BOLT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-003 frame 0: Width 0 and glow 0: the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-BOLT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-004 frame 0: Jaggedness 0, branches 0, width 1, glow 0, from (-10, 45) to (110, 45): a straight white line one pixel thick along the middle of row 4, its ends outside the frame. Row 4 is white added to the sky on the left, and white on the clear on the right; nothing else changes. | largest difference 5.1e-8 | yes |
| FX-BOLT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-005 frame 0: FX-BOLT-004 at 50 per cent down, on the edge between rows 4 and 5: each row half covered. | largest difference 1.5e-8 | yes |
| FX-BOLT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-006 frame 0: FX-BOLT-005 two pixels wide: rows 4 and 5 both fully covered, rows 3 and 6 untouched. | largest difference 5.1e-8 | yes |
| FX-BOLT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-007 frame 0: FX-BOLT-004 with width 0 and glow 3: only the glow, full on row 4, (2/3)^2 of it on rows 3 and 5, (1/3)^2 on rows 2 and 6, nothing further. | largest difference 4.5e-8 | yes |
| FX-BOLT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-008 frame 0: FX-BOLT-004 with glow 3: row 4 the white core, rows 2, 3, 5 and 6 FX-BOLT-007's glow. | largest difference 5.1e-8 | yes |
| FX-BOLT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-009 frame 0: Jaggedness 40, detail 6, no branches, width 1, glow 2, from the middle of the top edge to the middle of the bottom: a jagged line with both ends fixed. Frames 0 and 1 the same bolt, frame 2 a new one. | largest difference 6.2e-8 | yes |
| FX-BOLT-009 frame 1: Jaggedness 40, detail 6, no branches, width 1, glow 2, from the middle of the top edge to the middle of the bottom: a jagged line with both ends fixed. Frames 0 and 1 the same bolt, frame 2 a new one. | largest difference 6.2e-8 | yes |
| FX-BOLT-009 frame 2: Jaggedness 40, detail 6, no branches, width 1, glow 2, from the middle of the top edge to the middle of the bottom: a jagged line with both ends fixed. Frames 0 and 1 the same bolt, frame 2 a new one. | largest difference 2.9e-8 | yes |
| FX-BOLT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-010 frame 0: FX-BOLT-009 with branches 100: the same bolt, with forks that thin to nothing at their tips added to it. | largest difference 6.2e-8 | yes |
| FX-BOLT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-011 frame 0: FX-BOLT-009 with detail 1: the line is halved once, two straight pieces meeting at one bend. | largest difference 6.2e-8 | yes |
| FX-BOLT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-012 frame 0: FX-BOLT-009 with seed 7: another bolt between the same ends. | largest difference 6.4e-8 | yes |
| FX-BOLT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-013 frame 0: FX-BOLT-009 with hold 1: a new bolt on every frame, frame 1 different from frame 0. | largest difference 6.2e-8 | yes |
| FX-BOLT-013 frame 1: FX-BOLT-009 with hold 1: a new bolt on every frame, frame 1 different from frame 0. | largest difference 2.9e-8 | yes |
| FX-BOLT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-014 frame 0: FX-BOLT-008 in colour, an orange core #ff8000 in a green glow #00ff00: row 4 orange, the glow's rows green; the covering as FX-BOLT-008's. | largest difference 5.1e-8 | yes |
| FX-BOLT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-015 frame 0: FX-BOLT-014 with its colours written in capitals: the same. | largest difference 5.1e-8 | yes |
| FX-BOLT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-016 frame 0: FX-BOLT-008 at opacity 50: half the light. | largest difference 1.5e-8 | yes |
| FX-BOLT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-017 frame 0: FX-BOLT-004 with both points keyed down from 5 per cent at frame 0 to 85 at frame 4, linear: the line on row 0 at frame 0, on row 4 at frame 2, as FX-BOLT-004, and on row 8 at frame 4. | largest difference 5.1e-8 | yes |
| FX-BOLT-017 frame 2: FX-BOLT-004 with both points keyed down from 5 per cent at frame 0 to 85 at frame 4, linear: the line on row 0 at frame 0, on row 4 at frame 2, as FX-BOLT-004, and on row 8 at frame 4. | largest difference 5.1e-8 | yes |
| FX-BOLT-017 frame 4: FX-BOLT-004 with both points keyed down from 5 per cent at frame 0 to 85 at frame 4, linear: the line on row 0 at frame 0, on row 4 at frame 2, as FX-BOLT-004, and on row 8 at frame 4. | largest difference 5.1e-8 | yes |
| FX-BOLT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-018 frame 0: FX-BOLT-008 with opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-BOLT-008, as frame 4 is. | largest difference 7.3e-9 | yes |
| FX-BOLT-018 frame 2: FX-BOLT-008 with opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-BOLT-008, as frame 4 is. | largest difference 5.1e-8 | yes |
| FX-BOLT-018 frame 4: FX-BOLT-008 with opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-BOLT-008, as frame 4 is. | largest difference 5.1e-8 | yes |
| FX-BOLT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-019 frame 0: FX-BOLT-009 moved three pixels right: the bolt moves with the drawing, and nothing is drawn outside the layer, so the three columns left of it stay empty. | largest difference 6.2e-8 | yes |
| FX-BOLT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-020 frame 0: Start and end both at the middle, (50, 50), width 4, glow 0: a round dot 4 pixels across. | largest difference 5.1e-8 | yes |
| FX-BOLT-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-021 frame 0: FX-BOLT-010 with detail 8, the most: finer steps and more forks. | largest difference 6.1e-8 | yes |
| FX-BOLT-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOLT-022 frame 0: Jaggedness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-022 frame 4: Jaggedness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-023 frame 0: Detail 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-023 frame 4: Detail 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-024 frame 0: Detail 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-024 frame 4: Detail 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-025 frame 0: Width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-025 frame 4: Width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-026 frame 0: Glow 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-026 frame 4: Glow 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-027 frame 0: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-027 frame 4: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-028 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-028 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-029 frame 0: A start at (1001, 0), past 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-029 frame 4: A start at (1001, 0), past 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-030 frame 0: Branches keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-030 frame 4: Branches keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-031 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-031 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOLT-032 frame 0: A glow colour written "blue". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-032 frame 4: A glow colour written "blue". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BOLT-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, it never grows the drawing's bounds | 0 | yes |
| glow 500, the most, does not grow them either | 0 | yes |
| width 100, the most, does not grow them either | 0 | yes |
| a half-size draft preview halves the width and the glow, and nothing else | LightningBolt { start: [40.0, 0.0], end: [60.0, 100.0], jagged: 40.0, detail: 6.0, branches: 30.0, width: 1.5, glow: 12.0, opacity: 100.0, hold: 2.0, seed: 0.0, color: "#ffffff", glow_color: "#6e8cff", composite: "on", kind: "direction", turbulence: 0.0, decay: 0.0, conductivity: 0.0, obstacle: 0.0, path: "split", core: "hard", frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bolt_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_015.json, its colours written in capitals, is saved in small letters | "#ff8000" "#00ff00" | yes |
| the frame the bolt is drawn at is never saved | None | yes |
| a file with no `hold` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a start that is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a width that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| start x 1001 is refused with a sentence, and nothing changes | Lightning Bolt's start runs from -1000 to 1000, and this is 1001. | yes |
| end y -1001 is refused with a sentence, and nothing changes | Lightning Bolt's end runs from -1000 to 1000, and this is -1001. | yes |
| jaggedness 101 is refused with a sentence, and nothing changes | Lightning Bolt's jagged runs from 0 to 100, and this is 101. | yes |
| detail 0 is refused with a sentence, and nothing changes | Lightning Bolt's detail runs from 1 to 8, and this is 0. | yes |
| detail 9 is refused with a sentence, and nothing changes | Lightning Bolt's detail runs from 1 to 8, and this is 9. | yes |
| branches -1 is refused with a sentence, and nothing changes | Lightning Bolt's branches runs from 0 to 100, and this is -1. | yes |
| width 101 is refused with a sentence, and nothing changes | Lightning Bolt's width runs from 0 to 100, and this is 101. | yes |
| glow 501 is refused with a sentence, and nothing changes | Lightning Bolt's glow runs from 0 to 500, and this is 501. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Lightning Bolt's opacity runs from 0 to 100, and this is 101. | yes |
| hold 0 is refused with a sentence, and nothing changes | Lightning Bolt's hold runs from 1 to 100, and this is 0. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Lightning Bolt's seed runs from 0 to 100000, and this is 100001. | yes |
| colour "#12345" is refused with a sentence, and nothing changes | Lightning Bolt's colour is written #rrggbb, and this is "#12345". | yes |
| glow colour "blue" is refused with a sentence, and nothing changes | Lightning Bolt's glow colour is written #rrggbb, and this is "blue". | yes |
| branches keyed to 150 is refused with a sentence, and nothing changes | Lightning Bolt's branches runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| detail keyed from 1 to 8 is taken | taken | yes |
| the start keyed across the layer is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a night scene at a quarter of 1920 by 1080, in `verification/B-126 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the scene with no effect, draws cleanly | [] | yes |
| as_added_frame_0.png, the proposal's bolt at frame 0, draws cleanly, changes some pixels, lights its two ends (144, 37) and (307, 205), and leaves the corners [(2, 2), (2, 267), (477, 2)] exactly | [], 3590 changed, ends lit [true, true], corners left [true, true, true] | yes |
| as_added_frame_2.png, at frame 2, a new bolt, draws cleanly, changes some pixels, lights its two ends (144, 37) and (307, 205), and leaves the corners [(2, 2), (2, 267), (477, 2)] exactly | [], 3143 changed, ends lit [true, true], corners left [true, true, true] | yes |
| branches_80_detail_8.png, branches 80 and detail 8, draws cleanly, changes some pixels, lights its two ends (144, 37) and (307, 205), and leaves the corners [(2, 2), (2, 267), (477, 2)] exactly | [], 4407 changed, ends lit [true, true], corners left [true, true, true] | yes |
| jaggedness_20.png, jaggedness 20, draws cleanly, changes some pixels, lights its two ends (144, 37) and (307, 205), and leaves the corners [(2, 2), (2, 267), (477, 2)] exactly | [], 3137 changed, ends lit [true, true], corners left [true, true, true] | yes |
| red_and_gold.png, width 8, glow 60, a gold core in red, draws cleanly, changes some pixels, lights its two ends (144, 37) and (307, 205), and leaves the corners [(2, 2), (2, 267), (477, 2)] exactly | [], 9014 changed, ends lit [true, true], corners left [true, true, true] | yes |
| the bolt changes with the hold: frame 2 is not frame 0 | true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bolt_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bolt_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bolt_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bolt_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

138 of 138 checks pass.
