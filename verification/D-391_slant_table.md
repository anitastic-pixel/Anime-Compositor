# D-391: Slant

B-270, after CycoreFX's CC Slant: the layer leaned over by Slant degrees (the top to the right for plus), its height scaled by Height per cent toward a level floor line through Floor, and without Stretching shortened as well by the cosine of the slant, as if tipped over; Set Color fills it with one colour, keeping its coverage. The formulas are this program's own. Every expected pixel is `Fixtures/slant/expected_slant.json`, written by `tools/slant_reference.py` before this code existed and printed in document 25 as FX-SLANT-001 to 024. Tolerance 2e-5.

## FX-SLANT-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SLANT-001 frame 0: The settings as they start: slant 0, height 100: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SLANT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-002 frame 0: Slant 30, stretching off, the floor at the bottom: the top leans right and the picture sinks to cos 30 of its height, its leant edges as long as they stood. | largest difference 1.9e-7 | yes |
| FX-SLANT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-003 frame 0: Slant 30, stretching on: the top leans right and the height stays. | largest difference 2.5e-7 | yes |
| FX-SLANT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-004 frame 0: Slant -30, stretching on: the top leans left. | largest difference 2.5e-7 | yes |
| FX-SLANT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-005 frame 0: Slant 30, stretching on, the floor through the middle: the top leans right and the bottom left. | largest difference 2.5e-7 | yes |
| FX-SLANT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-006 frame 0: Slant 30, stretching on, the floor at the top: the top row stays and the bottom leans left. | largest difference 2.5e-7 | yes |
| FX-SLANT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-007 frame 0: Slant 0, height 50, stretching on: squashed to half height onto the floor, the top half transparent. | largest difference 1.9e-7 | yes |
| FX-SLANT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-008 frame 0: Slant 0, height 200, stretching on, the floor through the middle: stretched to twice the height about the middle. | largest difference 1.9e-7 | yes |
| FX-SLANT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-009 frame 0: Height 0: nothing drawn. | largest difference 0.0e0 | yes |
| FX-SLANT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-010 frame 0: Slant 45, stretching on, height 50, set color on in red: a red shadow lying down to the right, as soft at its edges as the drawing. | largest difference 1.5e-8 | yes |
| FX-SLANT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-011 frame 0: Slant keyed from 0 at frame 0 to 60 at frame 4, linear, stretching on: frame 0 the drawing, frame 2 FX-SLANT-003. | largest difference 1.9e-7 | yes |
| FX-SLANT-011 frame 2: Slant keyed from 0 at frame 0 to 60 at frame 4, linear, stretching on: frame 0 the drawing, frame 2 FX-SLANT-003. | largest difference 2.5e-7 | yes |
| FX-SLANT-011 frame 4: Slant keyed from 0 at frame 0 to 60 at frame 4, linear, stretching on: frame 0 the drawing, frame 2 FX-SLANT-003. | largest difference 2.5e-7 | yes |
| FX-SLANT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-012 frame 0: Height eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, stretching on: at frame 2 it would pass 1000 and is held there. | largest difference 0.0e0 | yes |
| FX-SLANT-012 frame 2: Height eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, stretching on: at frame 2 it would pass 1000 and is held there. | largest difference 8.7e-8 | yes |
| FX-SLANT-012 frame 4: Height eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, stretching on: at frame 2 it would pass 1000 and is held there. | largest difference 8.7e-8 | yes |
| FX-SLANT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-013 frame 0: FX-SLANT-003 moved three pixels right: the same, moved; nothing grows. | largest difference 2.5e-7 | yes |
| FX-SLANT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-014 frame 0: Slant 80, stretching off: leant almost flat, cos 80 of its height, most of it past the right edge and cut. | largest difference 2.2e-7 | yes |
| FX-SLANT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-015 frame 0: Floor keyed from 50, 100 at frame 0 to 50, 0 at frame 4, slant 30, stretching on: frame 0 FX-SLANT-003, frame 2 FX-SLANT-005, frame 4 FX-SLANT-006. | largest difference 2.5e-7 | yes |
| FX-SLANT-015 frame 2: Floor keyed from 50, 100 at frame 0 to 50, 0 at frame 4, slant 30, stretching on: frame 0 FX-SLANT-003, frame 2 FX-SLANT-005, frame 4 FX-SLANT-006. | largest difference 2.5e-7 | yes |
| FX-SLANT-015 frame 4: Floor keyed from 50, 100 at frame 0 to 50, 0 at frame 4, slant 30, stretching on: frame 0 FX-SLANT-003, frame 2 FX-SLANT-005, frame 4 FX-SLANT-006. | largest difference 2.5e-7 | yes |
| FX-SLANT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SLANT-016 frame 0: Slant 81, above 80. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-016 frame 4: Slant 81, above 80. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-017 frame 0: Slant -81, below -80. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-017 frame 4: Slant -81, below -80. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-018 frame 0: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-018 frame 4: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-019 frame 0: Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-019 frame 4: Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-020 frame 0: Stretching "yes", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-020 frame 4: Stretching "yes", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-021 frame 0: Set Color "maybe", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-021 frame 4: Set Color "maybe", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-022 frame 0: Color "black", not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-022 frame 4: Color "black", not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-023 frame 0: Floor at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-023 frame 4: Floor at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SLANT-024 frame 0: Slant keyed to 90 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-024 frame 4: Slant keyed to 90 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SLANT-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slant_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_slant_010.json is saved with its slant, stretching, height, floor, set color and colour | {"color":"#ff0000","floor":[50,100],"height":50,"set_color":"on","slant":45,"stretching":"on"} | yes |
| fx_slant_016.json is refused in a sentence | Slant's slant runs from -80 to 80, and this is 81. | yes |
| fx_slant_017.json is refused in a sentence | Slant's slant runs from -80 to 80, and this is -81. | yes |
| fx_slant_018.json is refused in a sentence | Slant's height runs from 0 to 1000, and this is 1001. | yes |
| fx_slant_019.json is refused in a sentence | Slant's height runs from 0 to 1000, and this is -1. | yes |
| fx_slant_020.json is refused in a sentence | Slant's stretching is "on" or "off", and this is "yes". | yes |
| fx_slant_021.json is refused in a sentence | Slant's set color is "on" or "off", and this is "maybe". | yes |
| fx_slant_022.json is refused in a sentence | Slant's colour is written #rrggbb, and this is "black". | yes |
| fx_slant_023.json is refused in a sentence | Slant's floor runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Slant with no `height` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Slant whose floor is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| slant 80.5 is refused with a sentence, and nothing changes | Slant's slant runs from -80 to 80, and this is 80.5. | yes |
| height -0.5 is refused with a sentence, and nothing changes | Slant's height runs from 0 to 1000, and this is -0.5. | yes |
| stretching "On", written with a capital is refused with a sentence, and nothing changes | Slant's stretching is "on" or "off", and this is "On". | yes |
| colour "#fff" is refused with a sentence, and nothing changes | Slant's colour is written #rrggbb, and this is "#fff". | yes |
| slant keyed to 100 is refused with a sentence, and nothing changes | Slant's slant runs from -80 to 80, and this is 100. | yes |
| slant -25, stretched, height 60, floor 30, 80, set to a dark blue is taken | taken | yes |
| slant keyed from 0 to 60 is taken | taken | yes |
| floor keyed from 50, 100 to 50, 0 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slant_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_slant_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_slant_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_slant_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_slant_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_slant_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slant_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_slant_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_slant_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_slant_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Slant leaning 30, stretched: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2024921 pixels changed | yes |
| the reference shot, Slant leaning 30, stretched on three layers, frame 0, Full | largest difference 1 of 255, 504 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 30, stretched on three layers, frame 100, Full | largest difference 1 of 255, 665 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 30, stretched on three layers, frame 239, Full | largest difference 1 of 255, 419 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 30, stretched on three layers, frame 0, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 30, stretched on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 30, stretched on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2046228 pixels changed | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle on three layers, frame 0, Full | largest difference 1 of 255, 2239 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle on three layers, frame 100, Full | largest difference 1 of 255, 848 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle on three layers, frame 239, Full | largest difference 1 of 255, 651 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning -20, tipped, height 60 toward a floor across the middle on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 45, set to orange: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Slant leaning 45, set to orange on three layers, frame 0, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 45, set to orange on three layers, frame 100, Full | largest difference 1 of 255, 107 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 45, set to orange on three layers, frame 239, Full | largest difference 1 of 255, 121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 45, set to orange on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 45, set to orange on three layers, frame 100, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Slant leaning 45, set to orange on three layers, frame 239, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-391 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts, upright at full height: nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_lean.png, leaning 30, tipped: the street leaning right and shorter, its top left clear; draws cleanly | [], 106401 pixels changed | yes |
| 4_lean_stretched.png, leaning 30, stretched: leaning right at full height, the corners clear; draws cleanly | [], 55774 pixels changed | yes |
| 5_half_height.png, height 50 toward a floor across the middle: squashed to a band, top and bottom clear; draws cleanly | [], 119916 pixels changed | yes |
| 6_red.png, leaning -20, set to red: a red shape leaning left; draws cleanly | [], 129600 pixels changed | yes |

## Result

164 of 164 checks pass.
