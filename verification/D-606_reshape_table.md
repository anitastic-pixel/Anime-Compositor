# D-606: Reshape

B-485: `core.reshape` takes After Effects' Reshape controls (Source Mask, Destination Mask, Boundary Mask, Percent, Elasticity), pulls the drawing inside the source mask into the destination's shape, the rest following and the boundary held, and draws it through D-601's backward field. Every expected pixel is `Fixtures/reshape/expected_reshape.json`, written by `tools/reshape_reference.py` before this code existed and printed in document 25 as FX-RESHAPE-001 to 022. Tolerance 2e-5.

## FX-RESHAPE-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RESHAPE-001 frame 0: The settings as they start: no source and no destination mask: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-002 frame 0: Source mask 1, a square from (4, 3) to (8, 7); destination mask 2, the same square three pixels right; Percent 100, Elasticity Normal: the stripes in the square pulled right, the drawing's edges held. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-003 frame 0: Destination a tall strip from (5, 1) to (7, 9): the square squeezed across and stretched down. | largest difference 3.1e-7 | yes |
| FX-RESHAPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-004 frame 0: FX-RESHAPE-002 at Percent 50: half way. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-005 frame 0: FX-RESHAPE-002 with boundary mask 3, (3, 1) to (13, 9): nothing outside it moves, so columns 0 to 1 and 14 to 15 are the drawing's own. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-006 frame 0: FX-RESHAPE-002 at Elasticity Stiff: the pull spread wide. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-007 frame 0: FX-RESHAPE-002 at Elasticity Liquid: the pull kept near the edges. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-008 frame 0: The destination drawn the other way round: turned to wind the source's way first, so FX-RESHAPE-002's frame. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-009 frame 0: Destination a curved circle 5 across about (8, 5): the square rounded. | largest difference 2.7e-7 | yes |
| FX-RESHAPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-010 frame 0: Percent keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-RESHAPE-004's, frame 4 FX-RESHAPE-002's. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-010 frame 2: Percent keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-RESHAPE-004's, frame 4 FX-RESHAPE-002's. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-010 frame 4: Percent keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-RESHAPE-004's, frame 4 FX-RESHAPE-002's. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-011 frame 0: FX-RESHAPE-002 with the layer moved three pixels right: the masks are the layer's, so all of it moves with the layer. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-012 frame 0: The destination's path keyed from three pixels right at frame 0 to two rows lower at frame 4: the pull turns downward. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-012 frame 2: The destination's path keyed from three pixels right at frame 0 to two rows lower at frame 4: the pull turns downward. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-012 frame 4: The destination's path keyed from three pixels right at frame 0 to two rows lower at frame 4: the pull turns downward. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-013 frame 0: Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-013 frame 2: Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-013 frame 4: Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-RESHAPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-014 frame 0: Source and destination the same mask: nothing to pull, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHAPE-015 frame 0: Source mask 4, of three. Nothing to reshape: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-015 frame 4: Source mask 4, of three. Nothing to reshape: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-015: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-RESHAPE-016 frame 0: Destination mask 2, switched off. Nothing to reshape: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-016 frame 4: Destination mask 2, switched off. Nothing to reshape: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-016: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-RESHAPE-017 frame 0: Boundary mask 3 with two masks. Nothing to reshape: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-017 frame 4: Boundary mask 3 with two masks. Nothing to reshape: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-017: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-RESHAPE-018 frame 0: Elasticity "rubber", not one of the seven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-018 frame 4: Elasticity "rubber", not one of the seven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHAPE-019 frame 0: Percent 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-019 frame 4: Percent 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHAPE-020 frame 0: Source mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-020 frame 4: Source mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHAPE-021 frame 0: Boundary mask 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-021 frame 4: Boundary mask 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHAPE-022 frame 0: Percent keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-022 frame 4: Percent keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHAPE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_reshape_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshape_001.json is saved with its five settings, the elasticity as a word, no outlines | {"boundary_mask":0,"destination_mask":0,"elasticity":"normal","percent":100,"source_mask":0} | yes |
| fx_reshape_018.json is refused in a sentence naming Reshape and its elasticity | Reshape's elasticity is stiff, less_stiff, below_normal, normal, above_normal, super_fluid or liquid, and this is "rubber". | yes |
| fx_reshape_019.json is refused in a sentence naming Reshape and its percent | Reshape's percent runs from 0 to 100, and this is 101. | yes |
| fx_reshape_020.json is refused in a sentence naming Reshape and its source mask | Reshape's source mask runs from 0 to 1000, and this is -1. | yes |
| fx_reshape_021.json is refused in a sentence naming Reshape and its boundary mask | Reshape's boundary mask runs from 0 to 1000, and this is 1001. | yes |
| a file with a Reshape whose elasticity is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Reshape whose source mask is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Reshape without its percent is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| elasticity rubber is refused with a sentence, and nothing changes | Reshape's elasticity is stiff, less_stiff, below_normal, normal, above_normal, super_fluid or liquid, and this is "rubber". | yes |
| percent 101 is refused with a sentence, and nothing changes | Reshape's percent runs from 0 to 100, and this is 101. | yes |
| the source mask keyed to end at 1001 is refused with a sentence, and nothing changes | Reshape's source mask runs from 0 to 1000, and this is 1001. | yes |
| elasticity liquid is taken | taken | yes |
| percent keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_reshape_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshape_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshape_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshape_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshape_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_reshape_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_reshape_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Reshape the square pulled to the square moved right: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1828679 pixels changed | yes |
| the reference shot, Reshape the square pulled to the square moved right on three layers, frame 0, Full | largest difference 1 of 255, 819 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the square pulled to the square moved right on three layers, frame 100, Full | largest difference 1 of 255, 763 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the square pulled to the square moved right on three layers, frame 239, Full | largest difference 1 of 255, 630 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the square pulled to the square moved right on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the square pulled to the square moved right on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the square pulled to the square moved right on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 664532 pixels changed | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent on three layers, frame 0, Full | largest difference 1 of 255, 1100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent on three layers, frame 100, Full | largest difference 1 of 255, 999 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent on three layers, frame 239, Full | largest difference 1 of 255, 916 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape the same within a boundary, Liquid, 60 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-606 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect (its four masks cut nothing), every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| As added, with no masks chosen: the street byte for byte; draws cleanly | [], 0 pixels differ | yes |
| 2_pulled_right.png, source mask 1 (a square about the middle) to destination mask 2 (the same square 60 pixels right): the middle of the street dragged right, the edges held; draws cleanly | [], 31846 pixels differ from the street | yes |
| 3_within_boundary.png, the same held at boundary mask 4 (120 to 420 across): nothing left or right of it moves, so it differs from 2_pulled_right.png; draws cleanly | [], outside the boundary untouched: true, 17048 pixels differ from 2_pulled_right.png | yes |
| 4_squeezed_tall.png, the square pulled into mask 3, a tall strip: the middle squeezed across and stretched up and down, so it differs from 2_pulled_right.png; draws cleanly | [], 84267 pixels differ from 2_pulled_right.png | yes |
| 5_stiff.png and 6_liquid.png, 2_pulled_right.png at Elasticity Stiff (the pull spread wide) and Liquid (kept near the square's edges): the two differ; both draw cleanly | [] [], 24846 pixels differ | yes |
| Source mask 9 of four: the street as it is, and EFFECT_PATH_MISSING said | ["EFFECT_PATH_MISSING Layer art's Reshape has no mask 9 that is on with two points or more, so it reshapes nothing."], 0 pixels differ | yes |

## Result

142 of 142 checks pass.
