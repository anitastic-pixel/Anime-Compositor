# D-966: Remove Grain's further groups

B-846, after After Effects' Remove Grain: Channel Noise Reduction scales each channel's measured grain; Chroma Suppression divides the colour part of a difference by 1 + it; Noise Size Bias widens the later passes and narrows the earlier ones; Clean Solid Areas is one more pass; Texture moves back toward the layer as given; Temporal Filtering first averages each pixel with the frames before and after where their colour is near; the Viewing Mode shows the result, a Preview region inside the layer as given, the sampled area or the matte. Every expected pixel is `Fixtures/remove_grain/expected_remove_grain_more.json`, written by `tools/remove_grain_more_reference.py` before this code existed and printed in document 25 as FX-REMOVEGRAIN-022 to 040. Tolerance 2e-5.

## FX-REMOVEGRAIN-022 to 040 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-REMOVEGRAIN-022 frame 0: As After Effects adds it now (D-965, D-966): three passes, Chroma Suppression 1, Texture 0.1, the rest neutral; a little grain passed through. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-023 frame 0: Chroma Suppression 3 alone: colour grain averaged harder than FX-REMOVEGRAIN-001. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-024 frame 0: Texture 0.5: half way between FX-REMOVEGRAIN-001 and the drawing. | largest difference 1.7e-7 | yes |
| FX-REMOVEGRAIN-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-025 frame 0: Texture 1: all the grain passed through, the drawing as it is. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-026 frame 0: Noise Size Bias -1 over three passes. | largest difference 1.3e-7 | yes |
| FX-REMOVEGRAIN-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-027 frame 0: Noise Size Bias 1 over three passes. | largest difference 1.3e-7 | yes |
| FX-REMOVEGRAIN-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-028 frame 0: Clean Solid Areas 2: one more pass, smoother than FX-REMOVEGRAIN-001. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-029 frame 0: Single Channel, Red Noise Reduction 0: red kept exactly, green and blue smoothed. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-030 frame 0: Viewing Mode Preview, the region 8 by 6 pixels round 25 % 50 %: inside FX-REMOVEGRAIN-001 with its edge white, outside the drawing. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-031 frame 0: Preview with Show Box off: no white edge. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-032 frame 0: Viewing Mode Noise Samples: the drawing as given, its edge white. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-033 frame 0: Viewing Mode Blending Matte on `holes`: white at each pixel's covering. | largest difference 3.0e-8 | yes |
| FX-REMOVEGRAIN-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-034 frame 0: Temporal Filtering on, as added (100 %, 0.9), on `seq`, grain drawn again each frame: frames 0 and 4 have one neighbour, frame 2 two. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-034 frame 2: Temporal Filtering on, as added (100 %, 0.9), on `seq`, grain drawn again each frame: frames 0 and 4 have one neighbour, frame 2 two. | largest difference 1.4e-7 | yes |
| FX-REMOVEGRAIN-034 frame 4: Temporal Filtering on, as added (100 %, 0.9), on `seq`, grain drawn again each frame: frames 0 and 4 have one neighbour, frame 2 two. | largest difference 1.5e-7 | yes |
| FX-REMOVEGRAIN-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-035 frame 2: Temporal Filtering off on `seq`: each frame alone. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-036 frame 2: Temporal Filtering Amount 50, Motion Sensitivity 0.3 on `cut`, whose right half turns green at frame 3: the green does not streak into frame 2. | largest difference 1.7e-7 | yes |
| FX-REMOVEGRAIN-036 frame 3: Temporal Filtering Amount 50, Motion Sensitivity 0.3 on `cut`, whose right half turns green at frame 3: the green does not streak into frame 2. | largest difference 1.7e-7 | yes |
| FX-REMOVEGRAIN-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REMOVEGRAIN-037 frame 0: Viewing Mode "wide", not one of its four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-037 frame 4: Viewing Mode "wide", not one of its four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-038 frame 0: Texture 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-038 frame 4: Texture 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-039 frame 0: Temporal Filtering "maybe", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-039 frame 4: Temporal Filtering "maybe", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REMOVEGRAIN-040 frame 0: Noise Size Bias -2, below -1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-040 frame 4: Noise Size Bias -2, below -1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-REMOVEGRAIN-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-REMOVEGRAIN-001 to 021 still draw as before (D-455)

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
| fx_removegrain_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_removegrain_001.json, written before D-966, is saved without any of the further settings | {"mode":"multichannel","noise_reduction":1,"passes":1,"unsharp_amount":0,"unsharp_radius":1,"unsharp_threshold":0} | yes |
| fx_removegrain_037.json is refused in a sentence that starts Remove Grain | Remove Grain's viewing mode is one of "final_output", "preview", "noise_samples", "blending_matte", and this is "wide". | yes |
| fx_removegrain_038.json is refused in a sentence that starts Remove Grain | Remove Grain's texture runs from 0 to 1, and this is 2. | yes |
| fx_removegrain_039.json is refused in a sentence that starts Remove Grain | Remove Grain's temporal filtering is "off" or "on", and this is "maybe". | yes |
| fx_removegrain_040.json is refused in a sentence that starts Remove Grain | Remove Grain's noise size bias runs from -1 to 1, and this is -2. | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_removegrain_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_030.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_removegrain_034.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-REMOVEGRAIN-022, fx_removegrain_022.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-023, fx_removegrain_023.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-024, fx_removegrain_024.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-025, fx_removegrain_025.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-026, fx_removegrain_026.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-027, fx_removegrain_027.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-028, fx_removegrain_028.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-029, fx_removegrain_029.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-030, fx_removegrain_030.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-031, fx_removegrain_031.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-032, fx_removegrain_032.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-033, fx_removegrain_033.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-034, fx_removegrain_034.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-035, fx_removegrain_035.json, frames [2] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-036, fx_removegrain_036.json, frames [2, 3] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-037, fx_removegrain_037.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-038, fx_removegrain_038.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-039, fx_removegrain_039.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-REMOVEGRAIN-040, fx_removegrain_040.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Result

142 of 142 checks pass.
