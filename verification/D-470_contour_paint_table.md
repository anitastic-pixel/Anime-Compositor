# D-470: Contour Paint

B-350: `core.contour_paint` takes After Effects' Contour Paint controls (View, Stylization, Cleanliness, Smear Iterations, Apply to Alpha, Draw Color Fills, Reduction Method, Posterization Level, Low and High Strength, Offset and Quantize Flow Angles, External Edge Map). Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by `tools/contour_paint_reference.py` before this code existed and printed in document 25 as FX-CONTOUR-001 to 032. Tolerance 2e-5.

## FX-CONTOUR-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CONTOUR-001 frame 0: As added: Stylization 50, Cleanliness 50, 3 smear passes: the photo's two-pixel squares smeared along their edges into diagonal strokes. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-002 frame 0: Smear Iterations 0, no fills: nothing happens. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-003 frame 0: Stylization 0: short strokes, nearer the photo than FX-CONTOUR-001. | largest difference 2.1e-7 | yes |
| FX-CONTOUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-004 frame 0: Stylization 100: long strokes, further from it. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-005 frame 0: Cleanliness 0: the flow follows each small square. | largest difference 1.8e-7 | yes |
| FX-CONTOUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-006 frame 0: Cleanliness 100: the flow follows only the broad pattern. | largest difference 1.9e-7 | yes |
| FX-CONTOUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-007 frame 0: Smear Iterations 12: smeared again and again, the strokes run together. | largest difference 1.8e-7 | yes |
| FX-CONTOUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-008 frame 0: The two blocks with clear round them, Apply to Alpha off: the colours smear, every covering kept exactly, every clear pixel clear. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-009 frame 0: The same with Apply to Alpha on: the blocks' edges smear out into the clear round them. | largest difference 1.8e-7 | yes |
| FX-CONTOUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-010 frame 0: Draw Color Fills Over Smeared Image, K-Means, 4 colours: every pixel one of four flat colours. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-011 frame 0: The same with Median: four colours by median cut alone. | largest difference 1.0e-7 | yes |
| FX-CONTOUR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-012 frame 0: Fills Only, Low Strength 38: the fills only where the flow is at least 38 per cent strong, clear elsewhere. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-013 frame 0: Fills Over Smeared Image, High Strength 38: the fills where the flow is weak, the smear where it is strong. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-014 frame 0: View Edge Map: the flow's strength in grey, two cyan lines along the flow in the two cells. | largest difference 2.4e-7 | yes |
| FX-CONTOUR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-015 frame 0: Offset Flow Angles 90: the strokes run across the edges instead. | largest difference 1.6e-7 | yes |
| FX-CONTOUR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-016 frame 0: Quantize Flow Angles 90: every stroke straight across or straight down. | largest difference 1.8e-7 | yes |
| FX-CONTOUR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-017 frame 0: The ramp as the External Edge Map: its edges all run straight down, so the photo is smeared straight down. | largest difference 1.9e-7 | yes |
| FX-CONTOUR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-018 frame 0: Edge Map with the ramp as the External Edge Map: the grey the same down each column, and one cyan line straight down the first cell (the second cell's middle lies past the drawing's right edge, and its line with it). | largest difference 1.4e-8 | yes |
| FX-CONTOUR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-019 frame 0: An External Edge Map that is not in the composition, `gone`: the layer's own edges, FX-CONTOUR-001, and the warning every frame. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-019 frame 4: An External Edge Map that is not in the composition, `gone`: the layer's own edges, FX-CONTOUR-001, and the warning every frame. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CONTOUR-020 frame 0: FX-CONTOUR-001 on the holder moved 2 right and 1 down: the same, moved. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-021 frame 0: Stylization keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is Stylization 50, FX-CONTOUR-001. | largest difference 2.1e-7 | yes |
| FX-CONTOUR-021 frame 2: Stylization keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is Stylization 50, FX-CONTOUR-001. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-021 frame 4: Stylization keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is Stylization 50, FX-CONTOUR-001. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-022 frame 0: Posterization Level eased from 3 at frame 0 to 32 at frame 4 on a curve that overshoots: at frame 2 it would pass 32, is held at 32. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-022 frame 2: Posterization Level eased from 3 at frame 0 to 32 at frame 4 on a curve that overshoots: at frame 2 it would pass 32, is held at 32. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-022 frame 4: Posterization Level eased from 3 at frame 0 to 32 at frame 4 on a curve that overshoots: at frame 2 it would pass 32, is held at 32. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-023 frame 0: Stylization 120, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-023 frame 4: Stylization 120, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-024 frame 0: Smear Iterations -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-024 frame 4: Smear Iterations -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-025 frame 0: A View written "arrows". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-025 frame 4: A View written "arrows". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-026 frame 0: Draw Color Fills written "under". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-026 frame 4: Draw Color Fills written "under". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-027 frame 0: Reduction Method "custom", not one of the method's words (After Effects' Match to Nearest Custom Color is written "match_to_nearest_custom_color", D-584). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-027 frame 4: Reduction Method "custom", not one of the method's words (After Effects' Match to Nearest Custom Color is written "match_to_nearest_custom_color", D-584). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-028 frame 0: Posterization Level 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-028 frame 4: Posterization Level 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-029 frame 0: Quantize Flow Angles 100, past 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-029 frame 4: Quantize Flow Angles 100, past 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-030 frame 0: An External Edge Map written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-030 frame 4: An External Edge Map written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-031 frame 0: Apply to Alpha written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-031 frame 4: Apply to Alpha written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-032 frame 0: Smear Iterations keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-032 frame 4: Smear Iterations keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_017.json is saved with its thirteen settings, and nothing the program works out for itself | {"apply_to_alpha":"off","cleanliness":50,"draw_color_fills":"none","edge_map_layer":"ramp","high_strength":100,"low_strength":0,"offset_flow_angles":0,"posterization_level":8,"quantize_flow_angles":0,"reduction_method":"k_means","smear_iterations":3,"stylization":50,"view":"final_result"} | yes |
| fx_contour_023.json is refused in a sentence naming Contour Paint and its stylization | Contour Paint's stylization runs from 0 to 100, and this is 120. | yes |
| fx_contour_024.json is refused in a sentence naming Contour Paint and its smear iterations | Contour Paint's smear iterations runs from 0 to 100, and this is -1. | yes |
| fx_contour_025.json is refused in a sentence naming Contour Paint and its view | Contour Paint's view is "final_result" or "edge_map", and this is "arrows". | yes |
| fx_contour_026.json is refused in a sentence naming Contour Paint and its draw color fills | Contour Paint's draw color fills is "none", "over_smeared_image" or "only", and this is "under". | yes |
| fx_contour_027.json is refused in a sentence naming Contour Paint and its reduction method | Contour Paint's reduction method is "k_means", "median" or "match_to_nearest_custom_color", and this is "custom". | yes |
| fx_contour_028.json is refused in a sentence naming Contour Paint and its posterization level | Contour Paint's posterization level runs from 2 to 32, and this is 1. | yes |
| fx_contour_029.json is refused in a sentence naming Contour Paint and its quantize flow angles | Contour Paint's quantize flow angles runs from 0 to 90, and this is 100. | yes |
| fx_contour_030.json is refused in a sentence naming Contour Paint and its external edge map | Contour Paint's external edge map is the name of a layer of this composition, and this is 3. | yes |
| fx_contour_031.json is refused in a sentence naming Contour Paint and its apply to alpha | Contour Paint's apply to alpha is "off" or "on", and this is "yes". | yes |
| a file with a Contour Paint whose stylization is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Contour Paint without its cleanliness is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Contour Paint whose view is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| stylization 120 is refused with a sentence, and nothing changes | Contour Paint's stylization runs from 0 to 100, and this is 120. | yes |
| view arrows is refused with a sentence, and nothing changes | Contour Paint's view is "final_result" or "edge_map", and this is "arrows". | yes |
| external edge map the number 3 is refused with a sentence, and nothing changes | Contour Paint's external edge map is the name of a layer of this composition, and this is 3. | yes |
| smear iterations keyed to 150 is refused with a sentence, and nothing changes | Contour Paint's smear iterations runs from 0 to 100, and this is 150. | yes |
| the ramp as the edge map, the flow turned 30 and snapped to 45, fills only is taken | taken | yes |
| posterization level keyed from 2 to 32 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Contour Paint as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 409789 pixels changed | yes |
| the reference shot, Contour Paint as added on three layers, frame 0, Full | largest difference 1 of 255, 1958 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint as added on three layers, frame 100, Full | largest difference 1 of 255, 892 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint as added on three layers, frame 239, Full | largest difference 1 of 255, 789 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint as added on three layers, frame 0, Draft | largest difference 1 of 255, 117 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint as added on three layers, frame 100, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint as added on three layers, frame 239, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1977886 pixels changed | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30 on three layers, frame 0, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30 on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30 on three layers, frame 239, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 411765 pixels changed | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor on three layers, frame 0, Full | largest difference 1 of 255, 16561 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor on three layers, frame 100, Full | largest difference 1 of 255, 14715 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor on three layers, frame 239, Full | largest difference 1 of 255, 15894 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor on three layers, frame 0, Draft | largest difference 1 of 255, 8892 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor on three layers, frame 100, Draft | largest difference 1 of 255, 9104 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint the flow snapped to 15 degrees: a step, drawn by the processor on three layers, frame 239, Draft | largest difference 1 of 255, 8839 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073579 pixels changed | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map on three layers, frame 0, Full | largest difference 1 of 255, 1526 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map on three layers, frame 100, Full | largest difference 1 of 255, 1143 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map on three layers, frame 239, Full | largest difference 1 of 255, 1524 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint View Edge Map with layer4 as the External Edge Map on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-470 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, Contour Paint as added: the street smeared along its own edges into strokes, the covering exactly as before; draws cleanly | [], changed true, coverings the same true | yes |
| 3_long_strokes.png, Stylization 100 and 6 smears: longer strokes, further from the street than 2_as_added.png; draws cleanly | [], distance from the street 948095 against 395370 | yes |
| 4_fills_6_colours.png, Draw Color Fills Over Smeared Image, K-Means, 6 colours: the street in at most 6 flat colours; draws cleanly | [], 6 colours against 1492 in 2_as_added.png | yes |
| 5_fills_only_strong_edges.png, Fills Only, Low Strength 10: flat colours along the buildings' and road's edges only, clear between (6_edge_map.png shows the strengths); draws cleanly | [], 90488 of 129600 pixels clear, 6 colours | yes |
| 6_edge_map.png, View Edge Map: the edges' strength in grey, opaque everywhere, with short cyan lines along the flow; draws cleanly | [], opaque true, 1778 cyan pixels | yes |
| 7_steered_by_bricks.png, a brick wall (brick_wall.png) as the External Edge Map, Stylization 100: the street smeared along the wall's mortar lines instead of its own edges, so it differs from the same smear steered by the street; draws cleanly | [], differs from the street's own flow true | yes |

## Result

194 of 194 checks pass.
