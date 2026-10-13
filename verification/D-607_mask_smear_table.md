# D-607: Smear (Mask Smear)

B-486: `core.mask_smear` takes After Effects' Smear controls (Source Mask, Boundary Mask, Mask Offset, Mask Rotation, Mask Scale, Percent, Elasticity), carries the drawing inside the source mask as the mask is moved, turned and scaled, the rest following and the boundary held, and draws it through D-601's backward field. Every expected pixel is `Fixtures/mask_smear/expected_mask_smear.json`, written by `tools/mask_smear_reference.py` before this code existed and printed in document 25 as FX-SMEAR-001 to 022. Tolerance 2e-5.

## FX-SMEAR-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SMEAR-001 frame 0: The settings as they start: no source mask: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SMEAR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-002 frame 0: Source mask 1, a square from (4, 3) to (8, 7), offset three pixels right: the stripes in the square carried right, the drawing's edges held; FX-RESHAPE-002's frame, whose destination is that square moved so. | largest difference 2.5e-7 | yes |
| FX-SMEAR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-003 frame 0: The square turned 45 degrees clockwise about its middle: the stripes in it twisted. | largest difference 2.5e-7 | yes |
| FX-SMEAR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-004 frame 0: The square scaled to 150 per cent: the middle swollen. | largest difference 2.5e-7 | yes |
| FX-SMEAR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-005 frame 0: The square scaled to 50 per cent, turned 90 degrees and offset by (2, 1). | largest difference 2.7e-7 | yes |
| FX-SMEAR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-006 frame 0: FX-SMEAR-002 with boundary mask 2, (3, 1) to (13, 9): nothing outside it moves, so columns 0 to 1 and 14 to 15 are the drawing's own. | largest difference 2.5e-7 | yes |
| FX-SMEAR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-007 frame 0: FX-SMEAR-002 at Percent 50: half way. | largest difference 2.5e-7 | yes |
| FX-SMEAR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-008 frame 0: FX-SMEAR-003 at Elasticity Liquid: the twist kept near the square's edges. | largest difference 2.5e-7 | yes |
| FX-SMEAR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-009 frame 0: Mask Offset keyed from (0, 0) at frame 0 to (3, 0) at frame 4, linear: frame 0 the drawing, frame 4 FX-SMEAR-002's. | largest difference 1.9e-7 | yes |
| FX-SMEAR-009 frame 2: Mask Offset keyed from (0, 0) at frame 0 to (3, 0) at frame 4, linear: frame 0 the drawing, frame 4 FX-SMEAR-002's. | largest difference 2.5e-7 | yes |
| FX-SMEAR-009 frame 4: Mask Offset keyed from (0, 0) at frame 0 to (3, 0) at frame 4, linear: frame 0 the drawing, frame 4 FX-SMEAR-002's. | largest difference 2.5e-7 | yes |
| FX-SMEAR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-010 frame 0: FX-SMEAR-002 with the layer moved three pixels right: the masks are the layer's, so all of it moves with the layer. | largest difference 2.5e-7 | yes |
| FX-SMEAR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-011 frame 0: Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-SMEAR-011 frame 2: Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-SMEAR-011 frame 4: Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-SMEAR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-012 frame 0: The source's path keyed from the square at frame 0 to two rows lower at frame 4, offset three right: frame 0 FX-SMEAR-002's. | largest difference 2.5e-7 | yes |
| FX-SMEAR-012 frame 2: The source's path keyed from the square at frame 0 to two rows lower at frame 4, offset three right: frame 0 FX-SMEAR-002's. | largest difference 2.5e-7 | yes |
| FX-SMEAR-012 frame 4: The source's path keyed from the square at frame 0 to two rows lower at frame 4, offset three right: frame 0 FX-SMEAR-002's. | largest difference 2.5e-7 | yes |
| FX-SMEAR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-013 frame 0: Percent 0, the square offset: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SMEAR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMEAR-014 frame 0: Source mask 3, of two. Nothing to smear: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SMEAR-014 frame 4: Source mask 3, of two. Nothing to smear: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SMEAR-014: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SMEAR-015 frame 0: Source mask 1, switched off. Nothing to smear: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SMEAR-015 frame 4: Source mask 1, switched off. Nothing to smear: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SMEAR-015: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SMEAR-016 frame 0: Boundary mask 3 with two masks. Nothing to smear: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SMEAR-016 frame 4: Boundary mask 3 with two masks. Nothing to smear: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SMEAR-016: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SMEAR-017 frame 0: Elasticity "rubber", not one of the seven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-017 frame 4: Elasticity "rubber", not one of the seven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-018 frame 0: Mask Scale 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-018 frame 4: Mask Scale 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-019 frame 0: Mask Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-019 frame 4: Mask Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-020 frame 0: Mask Offset (10001, 0), its x above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-020 frame 4: Mask Offset (10001, 0), its x above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-021 frame 0: Percent keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-021 frame 4: Percent keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMEAR-022 frame 0: Source mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-022 frame 4: Source mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMEAR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

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
| fx_smear_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smear_001.json is saved with its seven settings, the offset as two numbers, the elasticity as a word, no outlines | {"boundary_mask":0,"elasticity":"normal","mask_offset":[0,0],"mask_rotation":0,"mask_scale":100,"percent":100,"source_mask":0} | yes |
| fx_smear_017.json is refused in a sentence naming Mask Smear and its elasticity | Mask Smear's elasticity is stiff, less_stiff, below_normal, normal, above_normal, super_fluid or liquid, and this is "rubber". | yes |
| fx_smear_018.json is refused in a sentence naming Mask Smear and its mask scale | Mask Smear's mask scale runs from 0 to 1000, and this is 1001. | yes |
| fx_smear_019.json is refused in a sentence naming Mask Smear and its mask rotation | Mask Smear's mask rotation runs from -3600 to 3600, and this is 3601. | yes |
| fx_smear_020.json is refused in a sentence naming Mask Smear and its mask offset | Mask Smear's mask offset runs from -10000 to 10000, and this is 10001. | yes |
| fx_smear_022.json is refused in a sentence naming Mask Smear and its source mask | Mask Smear's source mask runs from 0 to 1000, and this is -1. | yes |
| a file with a Mask Smear whose elasticity is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mask Smear whose mask offset is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mask Smear without its mask scale is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| elasticity rubber is refused with a sentence, and nothing changes | Mask Smear's elasticity is stiff, less_stiff, below_normal, normal, above_normal, super_fluid or liquid, and this is "rubber". | yes |
| mask scale 1001 is refused with a sentence, and nothing changes | Mask Smear's mask scale runs from 0 to 1000, and this is 1001. | yes |
| the mask offset keyed to end at (10001, 0) is refused with a sentence, and nothing changes | Mask Smear's mask offset runs from -10000 to 10000, and this is 10001. | yes |
| elasticity liquid is taken | taken | yes |
| mask rotation keyed from 0 to 90 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smear_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smear_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smear_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smear_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Mask Smear the square carried 200 right: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1828679 pixels changed | yes |
| the reference shot, Mask Smear the square carried 200 right on three layers, frame 0, Full | largest difference 1 of 255, 819 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square carried 200 right on three layers, frame 100, Full | largest difference 1 of 255, 763 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square carried 200 right on three layers, frame 239, Full | largest difference 1 of 255, 630 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square carried 200 right on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square carried 200 right on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square carried 200 right on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 682167 pixels changed | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent on three layers, frame 0, Full | largest difference 1 of 255, 1901 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent on three layers, frame 100, Full | largest difference 1 of 255, 849 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent on three layers, frame 239, Full | largest difference 1 of 255, 928 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mask Smear the square turned 30, scaled to 120, within the boundary, Liquid, 60 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-607 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect (its two masks cut nothing), every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| As added, with no source mask chosen: the street byte for byte; draws cleanly | [], 0 pixels differ | yes |
| 2_carried_right.png, source mask 1 (a square about the middle) moved 60 pixels right: the middle of the street carried right, the edges held; draws cleanly | [], 31846 pixels differ from the street | yes |
| 3_within_boundary.png, the same held at boundary mask 2 (120 to 420 across): nothing left or right of it moves, so it differs from 2_carried_right.png; draws cleanly | [], outside the boundary untouched: true, 17048 pixels differ from 2_carried_right.png | yes |
| 4_turned.png, the square turned 45 degrees clockwise (the middle of the street twisted), and 5_swollen.png, scaled to 160 per cent (the middle swollen): each differs from the street and from the other; both draw cleanly | [] [], 76776 and 77383 pixels differ from the street, 82852 from each other | yes |
| 6_stiff.png and 7_liquid.png, 4_turned.png at Elasticity Stiff (the twist spread wide) and Liquid (kept near the square's edges): the two differ; both draw cleanly | [] [], 78162 pixels differ | yes |
| Source mask 9 of two: the street as it is, and EFFECT_PATH_MISSING said | ["EFFECT_PATH_MISSING Layer art's Mask Smear has no mask 9 that is on with two points or more, so it smears nothing."], 0 pixels differ | yes |

## Result

144 of 144 checks pass.
