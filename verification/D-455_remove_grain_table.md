# D-455: Remove Grain

B-335, after After Effects' Remove Grain: the grain of the layer as it reaches the effect is measured in red, green and blue by Immerkaer's fast noise estimate (1996), as Match Grain (D-454) measures it. Each pass k (1 to Passes) is a bilateral filter (Tomasi and Manduchi, 1998) over a disc of radius 2k, spatial spread k, on the encoded colour, whose colour spread is twice the measured noise times Noise Reduction (the 2-sigma rule of Zhang and Gunturk, 2008); Multichannel weighs all three channels together, Single Channel each alone. The Unsharp Mask is D-317's Sharpen. Noise Reduction 0, or no grain measured, leaves the layer as it is. The rule, ranges and starting values are ours, as Adobe publishes none. Every expected pixel is `Fixtures/remove_grain/expected_remove_grain.json`, written by `tools/remove_grain_reference.py` before this code existed and printed in document 25 as FX-REMOVEGRAIN-001 to 021. Tolerance 2e-5.

## FX-REMOVEGRAIN-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-REMOVEGRAIN-001 frame 0: The settings as added (Noise Reduction 1, one pass, Multichannel, no Unsharp Mask) on `grainy`: the grain in each half is smoothed, the edge between the halves stays hard. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-002 frame 0: Noise Reduction 0: the drawing as it is. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-003 frame 0: Noise Reduction 2: smoother than FX-REMOVEGRAIN-001. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-004 frame 0: Noise Reduction 3: smoother still, the edge still hard. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-005 frame 0: Passes 2: a second, wider pass, smoother than FX-REMOVEGRAIN-001. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-006 frame 0: Passes 4: four passes, the widest within 8 pixels. | largest difference 1.3e-7 | yes |
| FX-REMOVEGRAIN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-007 frame 0: Single Channel: each channel smoothed by its own noise. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-008 frame 0: `red_only`, Single Channel: red smoothed, green and blue, which have no grain, kept exactly. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-009 frame 0: Unsharp Mask amount 100, radius 1: FX-REMOVEGRAIN-001 sharpened, the edge crisper. | largest difference 2.2e-7 | yes |
| FX-REMOVEGRAIN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-010 frame 0: Unsharp Mask amount 200, radius 2, threshold 20: only the edge is sharpened; the smoothed halves away from it keep FX-REMOVEGRAIN-001's values. | largest difference 2.0e-7 | yes |
| FX-REMOVEGRAIN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-011 frame 0: `holes`, with one empty pixel and a half-covered column: the empty pixel stays empty and the column keeps its covering. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-012 frame 0: `flat`, one colour: no noise measured, the drawing as it is. | largest difference 6.4e-8 | yes |
| FX-REMOVEGRAIN-012 frame 2: `flat`, one colour: no noise measured, the drawing as it is. | largest difference 6.4e-8 | yes |
| FX-REMOVEGRAIN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-013 frame 0: Noise Reduction keyed from 0 at frame 0 to 3 at frame 4: frame 0 the drawing, frame 4 FX-REMOVEGRAIN-004. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-013 frame 2: Noise Reduction keyed from 0 at frame 0 to 3 at frame 4: frame 0 the drawing, frame 4 FX-REMOVEGRAIN-004. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-013 frame 4: Noise Reduction keyed from 0 at frame 0 to 3 at frame 4: frame 0 the drawing, frame 4 FX-REMOVEGRAIN-004. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-014 frame 0: Passes 2.6: its whole part, FX-REMOVEGRAIN-005. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-015 frame 0: `heavy`, three times the grain, as added: the noise measured is larger, so the same setting smooths harder. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-016 frame 0: Noise Reduction 0 with Unsharp Mask amount 100: no denoise, the drawing sharpened as Sharpen would. | largest difference 2.6e-7 | yes |
| FX-REMOVEGRAIN-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-017 frame 0: Noise Reduction 3.5, above 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-017 frame 4: Noise Reduction 3.5, above 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-018 frame 0: Passes 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-018 frame 4: Passes 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-019 frame 0: Mode "both", not one of its two words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-019 frame 4: Mode "both", not one of its two words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-020 frame 0: Unsharp Mask amount 600, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-020 frame 4: Unsharp Mask amount 600, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-021 frame 0: Unsharp Mask threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-021 frame 4: Unsharp Mask threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_removegrain_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_013.json is saved with its words and numbers as written, Noise Reduction's keys kept, and no measured noise | {"mode":"multichannel","noise_reduction":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":3}]},"passes":1,"unsharp_amount":0,"unsharp_radius":1,"unsharp_threshold":0} | yes |
| fx_removegrain_017.json is refused in a sentence naming 3.5 | Remove Grain's noise reduction runs from 0 to 3, and this is 3.5. | yes |
| fx_removegrain_018.json is refused in a sentence naming 0 | Remove Grain's passes runs from 1 to 4, and this is 0. | yes |
| fx_removegrain_019.json is refused in a sentence naming "both" | Remove Grain's mode is "multichannel" or "single_channel", and this is "both". | yes |
| fx_removegrain_020.json is refused in a sentence naming 600 | Remove Grain's unsharp amount runs from 0 to 500, and this is 600. | yes |
| a file with a Remove Grain with no `passes` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| noise reduction 3.5 is refused with a sentence, and nothing changes | Remove Grain's noise reduction runs from 0 to 3, and this is 3.5. | yes |
| mode "both" is refused with a sentence, and nothing changes | Remove Grain's mode is "multichannel" or "single_channel", and this is "both". | yes |
| passes keyed to 5 is refused with a sentence, and nothing changes | Remove Grain's passes runs from 1 to 4, and this is 5. | yes |
| noise reduction 2, two passes, single channel is taken | taken | yes |
| passes keyed from 1 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_removegrain_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_removegrain_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_removegrain_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot with Noise, Remove Grain as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072918 pixels changed | yes |
| the reference shot, Noise then Remove Grain as added on three layers, frame 0, Full | largest difference 1 of 255, 1085 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain as added on three layers, frame 100, Full | largest difference 1 of 255, 1087 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain as added on three layers, frame 239, Full | largest difference 1 of 255, 1085 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain as added on three layers, frame 0, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain as added on three layers, frame 100, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain as added on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot with Noise, Remove Grain Noise Reduction 2, three passes, single channel: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073440 pixels changed | yes |
| the reference shot, Noise then Remove Grain Noise Reduction 2, three passes, single channel on three layers, frame 0, Full | largest difference 1 of 255, 1201 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction 2, three passes, single channel on three layers, frame 100, Full | largest difference 1 of 255, 1238 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction 2, three passes, single channel on three layers, frame 239, Full | largest difference 1 of 255, 1116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction 2, three passes, single channel on three layers, frame 0, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction 2, three passes, single channel on three layers, frame 100, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction 2, three passes, single channel on three layers, frame 239, Draft | largest difference 1 of 255, 85 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot with Noise, Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072207 pixels changed | yes |
| the reference shot, Noise then Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2 on three layers, frame 0, Full | largest difference 1 of 255, 1119 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2 on three layers, frame 100, Full | largest difference 1 of 255, 1138 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2 on three layers, frame 239, Full | largest difference 1 of 255, 1085 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2 on three layers, frame 0, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2 on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise then Remove Grain Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2 on three layers, frame 239, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-455 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_clean.png, the street with no effect; draws cleanly | [] | yes |
| 2_grainy.png, the street with Noise: grain everywhere; draws cleanly | [], average 7.60 levels from the clean street | yes |
| Noise Reduction 0: the grainy street untouched; draws cleanly | [], 0 pixels changed | yes |
| 3_as_added.png, Remove Grain as added: closer to the clean street than 2, and the edges keep at least nine tenths of their step; draws cleanly | [], average 2.73 levels from the clean street; edges 245.3 against 246.2 clean | yes |
| 4_strong.png, Noise Reduction 3, three passes: plain areas smoother than in 3, at the cost of softer edges (the colour spread is now wider than most edges); draws cleanly | [], roughness of plain areas 0.74 levels against 2.26 in 3; edges 97.3 against 246.2 clean | yes |
| 5_strong_sharpened.png, 4 then Unsharp Mask 150 at radius 1.5: harder edges than 4; draws cleanly | [], edges 179.4 against 97.3 in 4 | yes |

## Result

137 of 137 checks pass.
