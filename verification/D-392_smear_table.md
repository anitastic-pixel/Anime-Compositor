# D-392: Smear

B-271, after CycoreFX's CC Smear: a round patch Radius pixels across dragged from From toward To, Reach per cent of the way (minus drags it back), the drag full along the line between them and easing to nothing at the patch's edge. The formulas are this program's own. Every expected pixel is `Fixtures/smear/expected_smear.json`, written by `tools/smear_reference.py` before this code existed and printed in document 25 as FX-SMEAR-001 to 020. Tolerance 2e-5.

## FX-SMEAR-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SMEAR-001 frame 0: The settings as they start: from 40, 50 to 60, 50, radius 70, wider than the drawing: the picture at 6.4, 5 drawn out 3.2 pixels to the right, everything right of it pushed along. | largest difference 1.9e-7 | yes |
| FX-SMEAR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-002 frame 0: From 25, 50 to 75, 50, radius 4: a band 4 pixels each side of the middle row dragged 8 pixels right, the stripes at 4, 5 drawn out along it, the top and bottom rows untouched. | largest difference 2.5e-7 | yes |
| FX-SMEAR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-003 frame 0: FX-SMEAR-002 at reach 200: dragged twice as far. | largest difference 2.5e-7 | yes |
| FX-SMEAR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-004 frame 0: FX-SMEAR-002 at reach -100: dragged 8 pixels left from 4, 5. | largest difference 1.9e-7 | yes |
| FX-SMEAR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-005 frame 0: FX-SMEAR-002 at reach 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SMEAR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-006 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SMEAR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-007 frame 0: From 50, 20 to 50, 80, radius 5: dragged down the middle column, the blue band pulled down. | largest difference 2.5e-7 | yes |
| FX-SMEAR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-008 frame 0: From and to both at 50, 50: no drag, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-SMEAR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-009 frame 0: Radius keyed from 0 at frame 0 to 8 at frame 4, linear, from 25, 50 to 75, 50: frame 0 the drawing, frame 2 FX-SMEAR-002. | largest difference 1.9e-7 | yes |
| FX-SMEAR-009 frame 2: Radius keyed from 0 at frame 0 to 8 at frame 4, linear, from 25, 50 to 75, 50: frame 0 the drawing, frame 2 FX-SMEAR-002. | largest difference 2.5e-7 | yes |
| FX-SMEAR-009 frame 4: Radius keyed from 0 at frame 0 to 8 at frame 4, linear, from 25, 50 to 75, 50: frame 0 the drawing, frame 2 FX-SMEAR-002. | largest difference 2.5e-7 | yes |
| FX-SMEAR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-010 frame 0: To keyed from 25, 50 at frame 0 to 75, 50 at frame 4, from 25, 50, radius 4: frame 0 no drag, frame 4 FX-SMEAR-002: the drag grows out. | largest difference 1.9e-7 | yes |
| FX-SMEAR-010 frame 2: To keyed from 25, 50 at frame 0 to 75, 50 at frame 4, from 25, 50, radius 4: frame 0 no drag, frame 4 FX-SMEAR-002: the drag grows out. | largest difference 2.5e-7 | yes |
| FX-SMEAR-010 frame 4: To keyed from 25, 50 at frame 0 to 75, 50 at frame 4, from 25, 50, radius 4: frame 0 no drag, frame 4 FX-SMEAR-002: the drag grows out. | largest difference 2.5e-7 | yes |
| FX-SMEAR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-011 frame 0: FX-SMEAR-002 moved three pixels right: the same, moved; nothing grows. | largest difference 2.5e-7 | yes |
| FX-SMEAR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-012 frame 0: Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SMEAR-012 frame 2: Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 2.5e-7 | yes |
| FX-SMEAR-012 frame 4: Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 2.5e-7 | yes |
| FX-SMEAR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-013 frame 0: From 25, 50 to 30, 50, reach 1000, radius 3: the 0.8 pixel step ten times over, 8 pixels, the same drag as FX-SMEAR-002 but narrower. | largest difference 2.5e-7 | yes |
| FX-SMEAR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-014 frame 0: Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-014 frame 4: Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-015 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-015 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-016 frame 0: Reach 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-016 frame 4: Reach 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-017 frame 0: Reach -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-017 frame 4: Reach -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-018 frame 0: From at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-018 frame 4: From at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-019 frame 0: To at 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-019 frame 4: To at 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-020 frame 0: Reach keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-020 frame 4: Reach keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smear_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_013.json is saved with its from, to, reach and radius | {"from":[25,50],"radius":3,"reach":1000,"to":[30,50]} | yes |
| fx_smear_014.json is refused in a sentence | Smear's radius runs from 0 to 1000, and this is 1001. | yes |
| fx_smear_015.json is refused in a sentence | Smear's radius runs from 0 to 1000, and this is -1. | yes |
| fx_smear_016.json is refused in a sentence | Smear's reach runs from -1000 to 1000, and this is 1001. | yes |
| fx_smear_017.json is refused in a sentence | Smear's reach runs from -1000 to 1000, and this is -1001. | yes |
| fx_smear_018.json is refused in a sentence | Smear's from runs from -1000 to 1000, and this is 1001. | yes |
| fx_smear_019.json is refused in a sentence | Smear's to runs from -1000 to 1000, and this is -1001. | yes |
| a file with a Smear with no `radius` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Smear whose from is three numbers is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 1000.5 is refused with a sentence, and nothing changes | Smear's radius runs from 0 to 1000, and this is 1000.5. | yes |
| reach -1000.5 is refused with a sentence, and nothing changes | Smear's reach runs from -1000 to 1000, and this is -1000.5. | yes |
| to at 60, 1000.5 is refused with a sentence, and nothing changes | Smear's to runs from -1000 to 1000, and this is 1000.5. | yes |
| radius keyed to -10 is refused with a sentence, and nothing changes | Smear's radius runs from 0 to 1000, and this is -10. | yes |
| from 30, 40 to 70, 55, reach 150, radius 40 is taken | taken | yes |
| reach keyed from 0 to 300 is taken | taken | yes |
| to keyed from 60, 50 to 60, 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smear_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smear_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_smear_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_smear_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_smear_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_smear_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_smear_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 58377 pixels changed | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70) on three layers, frame 0, Full | largest difference 1 of 255, 2324 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70) on three layers, frame 100, Full | largest difference 1 of 255, 1298 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70) on three layers, frame 239, Full | largest difference 1 of 255, 1389 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70) on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear as it starts (40, 50 to 60, 50, radius 70) on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear a long drag, reach 300, radius 200: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 543537 pixels changed | yes |
| the reference shot, Smear a long drag, reach 300, radius 200 on three layers, frame 0, Full | largest difference 1 of 255, 736 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear a long drag, reach 300, radius 200 on three layers, frame 100, Full | largest difference 1 of 255, 911 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear a long drag, reach 300, radius 200 on three layers, frame 239, Full | largest difference 1 of 255, 576 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear a long drag, reach 300, radius 200 on three layers, frame 0, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear a long drag, reach 300, radius 200 on three layers, frame 100, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear a long drag, reach 300, radius 200 on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 73630 pixels changed | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120 on three layers, frame 0, Full | largest difference 1 of 255, 3782 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120 on three layers, frame 239, Full | largest difference 1 of 255, 1773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Smear dragged back down a diagonal, reach -150, radius 120 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-392 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: a patch in the middle dragged a little to the right; draws cleanly | [], 6828 pixels changed | yes |
| 3_long_drag.png, a long drag up and to the right, reach 200: the street pulled out in a streak; draws cleanly | [], 33473 pixels changed | yes |
| 4_pushed_back.png, from 50, 60 toward 50, 30 at reach -100: the buildings dragged down into the road instead of up; draws cleanly | [], 4572 pixels changed | yes |

## Result

141 of 141 checks pass.
