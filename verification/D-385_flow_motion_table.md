# D-385: Flow Motion

B-264, after CycoreFX's CC Flo Motion: two knots, each drawing the picture in towards itself for a positive amount and blowing it out of itself for a negative one, over a reach set by Falloff; the edges can repeat mirrored, and each pixel can average 1, 4 or 16 points. The formulas are this program's own. Every expected pixel is `Fixtures/flow_motion/expected_flow_motion.json`, written by `tools/flow_motion_reference.py` before this code existed and printed in document 25 as FX-FLOW-001 to 024. Tolerance 2e-5.

## FX-FLOW-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FLOW-001 frame 0: The settings as they start: knot 1 at a quarter of the way across, half way down, drawing in by 10; knot 2 at three quarters, blowing out by 10; falloff 5, a reach of about 2 pixels here; Tile Edges on. Round the left knot the stripes are drawn in and narrow, round the right they swell. | largest difference 2.5e-7 | yes |
| FX-FLOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-002 frame 0: Both amounts 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-FLOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-003 frame 0: Knot 1 alone, in the middle, drawing in by 10, Tile Edges off: the middle reads from twice as far out, so the band and the stripes are pulled in to half size there. | largest difference 2.5e-7 | yes |
| FX-FLOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-004 frame 0: Knot 1 alone, in the middle, blowing out by 10: the middle reads from half as far out, swelling to twice the size. | largest difference 2.5e-7 | yes |
| FX-FLOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-005 frame 0: Knot 1 alone, in the middle, drawing in by 30, falloff 10, Tile Edges on: the whole drawing reads from three to four times as far out, shrunk, and its copies, turned over by turns, fill the rest. | largest difference 2.5e-7 | yes |
| FX-FLOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-006 frame 0: FX-FLOW-005 with Tile Edges off: the shrunk drawing alone, transparent round it. | largest difference 1.8e-7 | yes |
| FX-FLOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-007 frame 0: Finer Controls on, Amount 200: 20 times finer, the same as 10 without: FX-FLOW-003 exactly. | largest difference 2.5e-7 | yes |
| FX-FLOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-008 frame 0: Falloff 0: the pull held to a fifth of a pixel round the knot: the four pixels round it move about a twentieth of a pixel, the rest far less. | largest difference 2.7e-7 | yes |
| FX-FLOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-009 frame 0: FX-FLOW-005 with Antialiasing medium: each pixel averages 2 by 2 points, the copies softer. | largest difference 1.7e-7 | yes |
| FX-FLOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-010 frame 0: FX-FLOW-005 with Antialiasing high: 4 by 4 points a pixel. | largest difference 1.6e-7 | yes |
| FX-FLOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-011 frame 0: Knot 2 alone at the top left corner, blowing out by 40, falloff 7: the corner swells across the drawing. | largest difference 2.5e-7 | yes |
| FX-FLOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-012 frame 0: Amount 1 keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 the drawing, frame 2 FX-FLOW-003. | largest difference 1.9e-7 | yes |
| FX-FLOW-012 frame 2: Amount 1 keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 the drawing, frame 2 FX-FLOW-003. | largest difference 2.5e-7 | yes |
| FX-FLOW-012 frame 4: Amount 1 keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 the drawing, frame 2 FX-FLOW-003. | largest difference 2.5e-7 | yes |
| FX-FLOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-013 frame 0: Knot 1 keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the pull slides across. | largest difference 1.9e-7 | yes |
| FX-FLOW-013 frame 2: Knot 1 keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the pull slides across. | largest difference 2.5e-7 | yes |
| FX-FLOW-013 frame 4: Knot 1 keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the pull slides across. | largest difference 2.5e-7 | yes |
| FX-FLOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-014 frame 0: FX-FLOW-003 moved three pixels right: the same, moved; nothing grows, and the three columns left of the drawing stay empty. | largest difference 2.5e-7 | yes |
| FX-FLOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-015 frame 0: Amount 1 eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-FLOW-015 frame 2: Amount 1 eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 0.0e0 | yes |
| FX-FLOW-015 frame 4: Amount 1 eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 0.0e0 | yes |
| FX-FLOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLOW-016 frame 0: Amount 1 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-016 frame 4: Amount 1 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-017 frame 0: Amount 2 -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-017 frame 4: Amount 2 -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-018 frame 0: Falloff 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-018 frame 4: Falloff 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-019 frame 0: Falloff -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-019 frame 4: Falloff -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-020 frame 0: Tile Edges written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-020 frame 4: Tile Edges written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-021 frame 0: Finer Controls written "On", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-021 frame 4: Finer Controls written "On", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-022 frame 0: Antialiasing written "best". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-022 frame 4: Antialiasing written "best". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-023 frame 0: Knot 1 at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-023 frame 4: Knot 1 at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLOW-024 frame 0: Amount 1 keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-024 frame 4: Amount 1 keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLOW-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flow_007.json is saved with its knots, amounts, falloff, tile edges, finer controls and antialiasing | {"amount_1":200,"amount_2":0,"antialiasing":"low","falloff":5,"finer_controls":"on","knot_1":[50,50],"knot_2":[75,50],"tile_edges":"off"} | yes |
| fx_flow_016.json is refused in a sentence | Flow Motion's amount 1 runs from -1000 to 1000, and this is 1001. | yes |
| fx_flow_017.json is refused in a sentence | Flow Motion's amount 2 runs from -1000 to 1000, and this is -1001. | yes |
| fx_flow_018.json is refused in a sentence | Flow Motion's falloff runs from 0 to 10, and this is 11. | yes |
| fx_flow_019.json is refused in a sentence | Flow Motion's falloff runs from 0 to 10, and this is -1. | yes |
| fx_flow_020.json is refused in a sentence | Flow Motion's tile edges is "on" or "off", and this is "yes". | yes |
| fx_flow_021.json is refused in a sentence | Flow Motion's finer controls is "on" or "off", and this is "On". | yes |
| fx_flow_022.json is refused in a sentence | Flow Motion's antialiasing is low, medium or high, and this is "best". | yes |
| fx_flow_023.json is refused in a sentence | Flow Motion's knot 1 runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Flow Motion with no `falloff` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Flow Motion whose knot 2 is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 1 1000.5 is refused with a sentence, and nothing changes | Flow Motion's amount 1 runs from -1000 to 1000, and this is 1000.5. | yes |
| knot 2 at -1000.5, 50 is refused with a sentence, and nothing changes | Flow Motion's knot 2 runs from -1000 to 1000, and this is -1000.5. | yes |
| falloff 10.5 is refused with a sentence, and nothing changes | Flow Motion's falloff runs from 0 to 10, and this is 10.5. | yes |
| antialiasing "Low", written with a capital is refused with a sentence, and nothing changes | Flow Motion's antialiasing is low, medium or high, and this is "Low". | yes |
| tile edges "mirror" is refused with a sentence, and nothing changes | Flow Motion's tile edges is "on" or "off", and this is "mirror". | yes |
| amount 2 keyed to -2000 is refused with a sentence, and nothing changes | Flow Motion's amount 2 runs from -1000 to 1000, and this is -2000. | yes |
| two knots, finer controls, high, edges off is taken | taken | yes |
| amount 1 keyed from 0 to 30 is taken | taken | yes |
| knot 2 keyed from 75, 50 to 25, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flow_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flow_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flow_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flow_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flow_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flow_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flow_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_flow_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flow_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_flow_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flow_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1994870 pixels changed | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right) on three layers, frame 0, Full | largest difference 1 of 255, 3554 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right) on three layers, frame 100, Full | largest difference 1 of 255, 2889 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right) on three layers, frame 239, Full | largest difference 1 of 255, 2455 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right) on three layers, frame 0, Draft | largest difference 1 of 255, 98 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right) on three layers, frame 100, Draft | largest difference 1 of 255, 107 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion as it starts (10 at the left, -10 at the right) on three layers, frame 239, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2064417 pixels changed | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high on three layers, frame 0, Full | largest difference 1 of 255, 482 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high on three layers, frame 100, Full | largest difference 1 of 255, 823 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high on three layers, frame 239, Full | largest difference 1 of 255, 501 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high on three layers, frame 0, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a strong pull in the middle, edges off, high on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2014397 pixels changed | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium on three layers, frame 0, Full | largest difference 1 of 255, 623 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium on three layers, frame 100, Full | largest difference 1 of 255, 865 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium on three layers, frame 239, Full | largest difference 1 of 255, 584 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a swell at the top left, finer controls, medium on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073564 pixels changed | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9 on three layers, frame 0, Full | largest difference 1 of 255, 1283 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9 on three layers, frame 100, Full | largest difference 1 of 255, 5169 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9 on three layers, frame 239, Full | largest difference 1 of 255, 8650 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9 on three layers, frame 0, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9 on three layers, frame 100, Draft | largest difference 1 of 255, 106 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Flow Motion a wide pinch, falloff 9 on three layers, frame 239, Draft | largest difference 1 of 255, 320 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-385 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: the street drawn in towards a point at the left and blown out round a point at the right; draws cleanly | [], 93941 pixels changed | yes |
| 3_amounts_0.png, both amounts 0: nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 4_swell_middle.png, -40 in the middle, falloff 3: a lens-like swell in the centre; draws cleanly | [], 86140 pixels changed | yes |
| 5_pinch_edges_off.png, 40 in the middle, falloff 8, edges off: the street drawn in towards the centre, the edges left clear; draws cleanly | [], 129475 pixels changed | yes |

## Result

171 of 171 checks pass.
