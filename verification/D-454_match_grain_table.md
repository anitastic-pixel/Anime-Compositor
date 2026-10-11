# D-454: Match Grain

B-334, after After Effects' Match Grain: the grain of the whole noise source layer is measured in red, green and blue by Immerkaer's fast noise estimate (1996), the sum of a 3 by 3 Laplacian difference over every window the layer covers, and D-443's Add Grain then lays grain whose spread in each channel is that measure times the channel's intensity and the intensity. Add Grain's settings shape it. No noise source leaves the layer as it is. The rule, ranges and starting values are ours, as Adobe publishes none. Every expected pixel is `Fixtures/match_grain/expected_match_grain.json`, written by `tools/match_grain_reference.py` before this code existed and printed in document 25 as FX-MATCHGRAIN-001 to 023. Tolerance 2e-5.

## FX-MATCHGRAIN-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MATCHGRAIN-001 frame 0: The settings as added: no source layer, so nothing is measured and the card is left as it is. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-002 frame 0: Source `steady`, hidden, the rest as added (Film): the card gains grain as strong as steady's, its red grain strongest and blue weakest, a new grain each frame. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-002 frame 2: Source `steady`, hidden, the rest as added (Film): the card gains grain as strong as steady's, its red grain strongest and blue weakest, a new grain each frame. | largest difference 2.6e-7 | yes |
| FX-MATCHGRAIN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-003 frame 0: Source `seq`: light grain at frame 0, heavy grain at frame 3 when the source's drawing changes. | largest difference 2.2e-7 | yes |
| FX-MATCHGRAIN-003 frame 3: Source `seq`: light grain at frame 0, heavy grain at frame 3 when the source's drawing changes. | largest difference 3.0e-7 | yes |
| FX-MATCHGRAIN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-004 frame 0: Blending Mode Add: the grain's spread per channel is steady's. | largest difference 2.5e-7 | yes |
| FX-MATCHGRAIN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-005 frame 0: Intensity 2, Add: twice FX-MATCHGRAIN-004's grain. | largest difference 4.0e-7 | yes |
| FX-MATCHGRAIN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-006 frame 0: Monochromatic, Add: one grain pattern in all three channels, each at its measured strength. | largest difference 2.5e-7 | yes |
| FX-MATCHGRAIN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-007 frame 0: Channel intensities 0, 1 and 2, Add: no red grain, green as measured, blue doubled. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-008 frame 0: Saturation 0, Add. | largest difference 2.5e-7 | yes |
| FX-MATCHGRAIN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-009 frame 0: Size 3, Softness 0.5, Aspect Ratio 2: bigger, softer, wider grains. | largest difference 2.3e-7 | yes |
| FX-MATCHGRAIN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-010 frame 0: Shadows 0, Highlights 0, Add: no grain in the card's black and white patches. | largest difference 2.3e-7 | yes |
| FX-MATCHGRAIN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-011 frame 0: Blending Mode Overlay, intensity 2. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-012 frame 0: Animation Speed 0: the same grain on frames 0, 2 and 4. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-012 frame 2: Animation Speed 0: the same grain on frames 0, 2 and 4. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-012 frame 4: Animation Speed 0: the same grain on frames 0, 2 and 4. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-013 frame 0: Source `holes`, with one empty pixel and a half-covered column: the windows over the empty pixel are not counted. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-014 frame 0: Source `flat`, one colour: no grain measured, the card as it is. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-014 frame 2: Source `flat`, one colour: no grain measured, the card as it is. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-015 frame 0: Source `ghost`, not a layer of the composition: the card as it is, with EFFECT_LAYER_MISSING each frame. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-015 frame 3: Source `ghost`, not a layer of the composition: the card as it is, with EFFECT_LAYER_MISSING each frame. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-MATCHGRAIN-016 frame 0: Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 the card; frame 4's grain twice frame 2's. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-016 frame 2: Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 the card; frame 4's grain twice frame 2's. | largest difference 4.0e-7 | yes |
| FX-MATCHGRAIN-016 frame 4: Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 the card; frame 4's grain twice frame 2's. | largest difference 5.8e-7 | yes |
| FX-MATCHGRAIN-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-017 frame 0: Random Seed 7: a different grain from FX-MATCHGRAIN-002's. | largest difference 2.6e-7 | yes |
| FX-MATCHGRAIN-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-018 frame 0: Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-018 frame 1: Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain. | largest difference 2.4e-7 | yes |
| FX-MATCHGRAIN-018 frame 3: Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain. | largest difference 2.7e-7 | yes |
| FX-MATCHGRAIN-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MATCHGRAIN-019 frame 0: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-019 frame 4: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MATCHGRAIN-020 frame 0: Size 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-020 frame 4: Size 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MATCHGRAIN-021 frame 0: Blending Mode "screen", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-021 frame 4: Blending Mode "screen", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MATCHGRAIN-022 frame 0: Monochromatic "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-022 frame 4: Monochromatic "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MATCHGRAIN-023 frame 0: Source layer the number 5, not a layer's id. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-023 frame 4: Source layer the number 5, not a layer's id. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MATCHGRAIN-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_matchgrain_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_matchgrain_016.json is saved with its noise source, its words and numbers as written, the intensity's keys kept, and no picture | {"animate_smoothly":"on","animation_speed":0,"aspect_ratio":1,"blending_mode":"add","blue_intensity":1,"green_intensity":1,"highlights":1,"intensity":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":4}]},"layer":"steady","midpoint":0.5,"midtones":1,"monochromatic":"off","random_seed":0,"red_intensity":1,"saturation":1,"shadows":1,"size":1,"softness":0} | yes |
| fx_matchgrain_019.json is refused in a sentence | Match Grain's intensity runs from 0 to 10, and this is 11. | yes |
| fx_matchgrain_023.json is refused in a sentence | Match Grain's noise source layer is the name of a layer of this composition, and this is 5. | yes |
| fx_matchgrain_021.json is refused in a sentence naming "screen" | Match Grain's blending mode is one of film, add, overlay, and this is "screen". | yes |
| fx_matchgrain_022.json is refused in a sentence naming "yes" | Match Grain's monochromatic is "off" or "on", and this is "yes". | yes |
| a file with a Match Grain with no `layer` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Match Grain with no `size` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| intensity 10.5 is refused with a sentence, and nothing changes | Match Grain's intensity runs from 0 to 10, and this is 10.5. | yes |
| blending mode "screen" is refused with a sentence, and nothing changes | Match Grain's blending mode is one of film, add, overlay, and this is "screen". | yes |
| the noise source a number is refused with a sentence, and nothing changes | Match Grain's noise source layer is the name of a layer of this composition, and this is 4. | yes |
| intensity keyed to 12 is refused with a sentence, and nothing changes | Match Grain's intensity runs from 0 to 10, and this is 12. | yes |
| intensity 3 by Add, the noise source holes is taken | taken | yes |
| intensity keyed from 0 to 10 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_matchgrain_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_matchgrain_003.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_matchgrain_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_matchgrain_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_matchgrain_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_matchgrain_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_matchgrain_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Match Grain layer2's grain, as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 37729 pixels changed | yes |
| the reference shot, Match Grain layer2's grain, as added on three layers, frame 0, Full | largest difference 1 of 255, 967 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, as added on three layers, frame 100, Full | largest difference 1 of 255, 1202 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, as added on three layers, frame 239, Full | largest difference 1 of 255, 786 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, as added on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, as added on three layers, frame 100, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, as added on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1652054 pixels changed | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic on three layers, frame 0, Full | largest difference 1 of 255, 968 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic on three layers, frame 100, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic on three layers, frame 239, Full | largest difference 1 of 255, 866 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic on three layers, frame 0, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic on three layers, frame 100, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, intensity 3 by Add, monochromatic on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 18300 pixels changed | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5 on three layers, frame 0, Full | largest difference 1 of 255, 1814 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5 on three layers, frame 100, Full | largest difference 1 of 255, 1126 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5 on three layers, frame 239, Full | largest difference 1 of 255, 777 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5 on three layers, frame 0, Draft | largest difference 1 of 255, 95 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5 on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Match Grain layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5 on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-454 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_no_source.png, as added (no noise source): the street untouched; draws cleanly | [], 0 pixels changed | yes |
| 3_light_plate.png, matching a plate with light grain: fine grain over the street; draws cleanly | [], spread of the grain in red, green, blue [16.87, 14.32, 9.11] levels | yes |
| 4_heavy_plate.png, matching a plate with heavy grain: heavier grain than 3, in every channel; draws cleanly | [], spread [53.66, 44.69, 28.08] levels | yes |
| a noise source that is no layer: the street untouched, and the warning says which layer | ["EFFECT_LAYER_MISSING An effect on layer art reads layer ghost, which is not in this composition."], 0 pixels changed | yes |

## Result

152 of 152 checks pass.
