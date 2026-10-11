# D-462: Mr. Smoothie

B-342: `core.mr_smoothie` takes After Effects' CC Mr. Smoothie controls (Flow Layer, Property, Smoothness, Sample A, Sample B, Phase, Color Loop). Every expected pixel is `Fixtures/mr_smoothie/expected_mr_smoothie.json`, written by `tools/mr_smoothie_reference.py` before this code existed and printed in document 25 as FX-SMOOTHIE-001 to 026. Tolerance 2e-5.

## FX-SMOOTHIE-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SMOOTHIE-001 frame 0: As added: the layer's own luminance, Smoothness 5, Sample A at 25, 50 per cent and B at 75, 50, Phase 0, Color Loop ABA: the layer repainted in soft bands of the colours between the two points. | largest difference 2.0e-7 | yes |
| FX-SMOOTHIE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-002 frame 0: The ramp as the flow layer, Smoothness 0, AB: each column takes the colour as far from A toward B as the ramp is bright there, A at the black left, B at the white right. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-003 frame 0: The same, BA: the gradient the other way, B at the left. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-004 frame 0: The same, ABA: A at both sides, B in the middle. | largest difference 1.7e-7 | yes |
| FX-SMOOTHIE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-005 frame 0: The same, BAB: B at both sides, A in the middle. | largest difference 1.8e-7 | yes |
| FX-SMOOTHIE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-006 frame 0: FX-SMOOTHIE-002 with Phase 90: the gradient moved a quarter along, the brightest columns wrapping round to start again near A. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-007 frame 0: FX-SMOOTHIE-002 with Sample A and B swapped: FX-SMOOTHIE-003. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-008 frame 0: Sample A and B both at 25, 50 on the two blocks: every covered pixel the red found there, the blue block turned red, the clear round them kept clear. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-009 frame 0: The two blocks' own luminance, Smoothness 4, AB: the blocks recoloured, their coverings exactly as they were. | largest difference 2.1e-7 | yes |
| FX-SMOOTHIE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-010 frame 0: Property Hue, Smoothness 0: each pixel placed along the gradient by its own hue. | largest difference 1.9e-7 | yes |
| FX-SMOOTHIE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-011 frame 0: Property Saturation of the photo, Smoothness 2, BAB. | largest difference 2.8e-7 | yes |
| FX-SMOOTHIE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-012 frame 0: The dots layer's covering, Smoothness 2, AB: B's colour in two soft round patches where the dots are, A's elsewhere. | largest difference 1.0e-7 | yes |
| FX-SMOOTHIE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-013 frame 0: The 8 by 5 spot, centred and clear round it, Property Red, Smoothness 2: a round patch from further along the gradient in the middle. | largest difference 2.3e-7 | yes |
| FX-SMOOTHIE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-014 frame 0: A layer that is not in the composition, `gone`: the layer itself is the flow layer, as FX-SMOOTHIE-001, and the warning every frame. | largest difference 2.0e-7 | yes |
| FX-SMOOTHIE-014 frame 4: A layer that is not in the composition, `gone`: the layer itself is the flow layer, as FX-SMOOTHIE-001, and the warning every frame. | largest difference 2.0e-7 | yes |
| FX-SMOOTHIE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-SMOOTHIE-015 frame 0: Smoothness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is Smoothness 4. | largest difference 1.7e-7 | yes |
| FX-SMOOTHIE-015 frame 2: Smoothness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is Smoothness 4. | largest difference 1.9e-7 | yes |
| FX-SMOOTHIE-015 frame 4: Smoothness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is Smoothness 4. | largest difference 2.0e-7 | yes |
| FX-SMOOTHIE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-016 frame 0: FX-SMOOTHIE-009 on the holder moved 2 right and 1 down: the same, moved. | largest difference 2.1e-7 | yes |
| FX-SMOOTHIE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-017 frame 0: Sample A keyed from 0, 50 at frame 0 to 100, 50 at frame 4, linear: frame 2 has A at 50, 50. | largest difference 2.3e-7 | yes |
| FX-SMOOTHIE-017 frame 2: Sample A keyed from 0, 50 at frame 0 to 100, 50 at frame 4, linear: frame 2 has A at 50, 50. | largest difference 1.3e-7 | yes |
| FX-SMOOTHIE-017 frame 4: Sample A keyed from 0, 50 at frame 0 to 100, 50 at frame 4, linear: frame 2 has A at 50, 50. | largest difference 2.1e-7 | yes |
| FX-SMOOTHIE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-018 frame 0: Phase keyed from 0 at frame 0 to 180 at frame 4 on the ramp, AB, linear: frame 0 is FX-SMOOTHIE-002 and frame 2 FX-SMOOTHIE-006. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-018 frame 2: Phase keyed from 0 at frame 0 to 180 at frame 4 on the ramp, AB, linear: frame 0 is FX-SMOOTHIE-002 and frame 2 FX-SMOOTHIE-006. | largest difference 1.5e-7 | yes |
| FX-SMOOTHIE-018 frame 4: Phase keyed from 0 at frame 0 to 180 at frame 4 on the ramp, AB, linear: frame 0 is FX-SMOOTHIE-002 and frame 2 FX-SMOOTHIE-006. | largest difference 1.2e-7 | yes |
| FX-SMOOTHIE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTHIE-019 frame 0: Smoothness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-019 frame 4: Smoothness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-020 frame 0: A property written "value", which Mr. Smoothie does not read. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-020 frame 4: A property written "value", which Mr. Smoothie does not read. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-021 frame 0: A colour loop written "abab". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-021 frame 4: A colour loop written "abab". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-022 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-022 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-023 frame 0: Sample B at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-023 frame 4: Sample B at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-024 frame 0: Phase 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-024 frame 4: Phase 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-025 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-025 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTHIE-026 frame 0: Smoothness keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-026 frame 4: Smoothness keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SMOOTHIE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smoothie_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smoothie_011.json is saved with its eight settings | {"color_loop":"bab","fit":"stretch","layer":"photo","phase":0,"property":"saturation","sample_a":[25,50],"sample_b":[75,50],"smoothness":2} | yes |
| fx_smoothie_019.json is refused in a sentence naming Mr. Smoothie and its smoothness | Mr. Smoothie's smoothness runs from 0 to 100, and this is 101. | yes |
| fx_smoothie_020.json is refused in a sentence naming Mr. Smoothie and its property | Mr. Smoothie's property is red, green, blue, alpha, luminance, lightness, hue or saturation, and this is "value". | yes |
| fx_smoothie_021.json is refused in a sentence naming Mr. Smoothie and its color loop | Mr. Smoothie's color loop is ab, ba, aba or bab, and this is "abab". | yes |
| fx_smoothie_022.json is refused in a sentence naming Mr. Smoothie and its flow layer | Mr. Smoothie's flow layer is the name of a layer of this composition, and this is 3. | yes |
| fx_smoothie_023.json is refused in a sentence naming Mr. Smoothie and its sample b | Mr. Smoothie's sample b runs from -1000 to 1000, and this is 1001. | yes |
| fx_smoothie_024.json is refused in a sentence naming Mr. Smoothie and its phase | Mr. Smoothie's phase runs from -3600 to 3600, and this is 3601. | yes |
| fx_smoothie_025.json is refused in a sentence naming Mr. Smoothie and its fit | Mr. Smoothie's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| a file with a Mr. Smoothie whose smoothness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mr. Smoothie without its color loop is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mr. Smoothie whose sample A is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mr. Smoothie whose property is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| smoothness 101 is refused with a sentence, and nothing changes | Mr. Smoothie's smoothness runs from 0 to 100, and this is 101. | yes |
| sample B 50, -1001 is refused with a sentence, and nothing changes | Mr. Smoothie's sample b runs from -1000 to 1000, and this is -1001. | yes |
| color loop abab is refused with a sentence, and nothing changes | Mr. Smoothie's color loop is ab, ba, aba or bab, and this is "abab". | yes |
| smoothness keyed to 150 is refused with a sentence, and nothing changes | Mr. Smoothie's smoothness runs from 0 to 100, and this is 150. | yes |
| the ramp tiled, Hue, smoothness 3, samples 10, 20 and 90, 80, phase 45, BAB is taken | taken | yes |
| sample A keyed from 0, 50 to 100, 50 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smoothie_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smoothie_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smoothie_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smoothie_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smoothie_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smoothie_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smoothie_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smoothie_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_smoothie_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Mr. Smoothie as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072929 pixels changed | yes |
| the reference shot, Mr. Smoothie as added on three layers, frame 0, Full | largest difference 1 of 255, 4205 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie as added on three layers, frame 100, Full | largest difference 1 of 255, 5380 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie as added on three layers, frame 239, Full | largest difference 1 of 255, 4368 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie as added on three layers, frame 0, Draft | largest difference 1 of 255, 116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie as added on three layers, frame 100, Draft | largest difference 1 of 255, 145 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie as added on three layers, frame 239, Draft | largest difference 1 of 255, 135 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073581 pixels changed | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB on three layers, frame 0, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB on three layers, frame 100, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB on three layers, frame 239, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073532 pixels changed | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mr. Smoothie Alpha, smoothness 0, AB, phase 45 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-462 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added (its own luminance, smoothness 5, ABA): the street repainted in bands of the colours between a quarter and three quarters across its middle, its covering exactly as before; draws cleanly | [], changed true, coverings the same true | yes |
| 3_hue_bab.png, Hue, smoothness 10, BAB: not 2_as_added.png, the covering kept; draws cleanly | [], different true, coverings the same true | yes |
| 4_phase_180.png, as added with Phase 180: the bands' colours moved half way round, not 2_as_added.png; Phase 360 brings back 2_as_added.png within 1 level at almost every pixel; draws cleanly | [] [], different true, Phase 360 within 1 level at 129600 of 129600 | yes |
| 5_one_point.png, both samples at the middle: every pixel the one colour found there (within 1 level); draws cleanly | [], 129600 of 129600 pixels the colour [137, 170, 194, 255] | yes |
| 6_smoothness_40.png, smoothness 40: wider, smoother bands than 2_as_added.png, the covering kept; draws cleanly | [], different true, coverings the same true | yes |

## Result

172 of 172 checks pass.
