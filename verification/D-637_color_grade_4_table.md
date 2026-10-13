# D-637: Color Grade, the fourth unit

B-517, after Lumetri Color under our own name: the Creative section's Sharpen (-100 to 100; below 0 it softens), after the faded film and before the vibrance, and the vignette's roundness below 0, its squarer shapes. Every expected pixel is `Fixtures/color_grade_4/expected_color_grade_4.json`, written by `tools/color_grade_4_reference.py` before this code existed and printed in document 25 as FX-GRADE4-001 to 016.

## FX-GRADE4-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE4-001 frame 0: Every setting as added, sharpen written at 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE4-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-002 frame 0: Sharpen 100: each colour pushed away from its 1-pixel blur, so the edges between the colour columns harder; flat runs untouched. | largest difference 2.5e-7 | yes |
| FX-GRADE4-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-003 frame 0: Sharpen 50: half FX-GRADE4-002's push. | largest difference 2.0e-7 | yes |
| FX-GRADE4-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-004 frame 0: Sharpen -100: each colour its 1-pixel blur, the drawing softened. | largest difference 1.5e-7 | yes |
| FX-GRADE4-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-005 frame 0: Sharpen -40: four tenths of the way to the blur. | largest difference 1.7e-7 | yes |
| FX-GRADE4-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-006 frame 0: Sharpen keyed from -100 at frame 0 to 100 at frame 4, linear: the blur at frame 0, the drawing at frame 2, FX-GRADE4-002 at frame 4. | largest difference 1.5e-7 | yes |
| FX-GRADE4-006 frame 2: Sharpen keyed from -100 at frame 0 to 100 at frame 4, linear: the blur at frame 0, the drawing at frame 2, FX-GRADE4-002 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE4-006 frame 4: Sharpen keyed from -100 at frame 0 to 100 at frame 4, linear: the blur at frame 0, the drawing at frame 2, FX-GRADE4-002 at frame 4. | largest difference 2.5e-7 | yes |
| FX-GRADE4-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-007 frame 0: Faded film 40, sharpen 80, vibrance 50: the sharpen works on the faded colours and the vibrance on the sharpened ones, in that order. | largest difference 2.5e-7 | yes |
| FX-GRADE4-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-008 frame 0: Sharpen 60 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-GRADE4-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-009 frame 0: A dark vignette (amount -3, feather 20) at roundness -100: a rectangle the layer's shape, so the corners darken no more than the edges' middles. | largest difference 1.5e-7 | yes |
| FX-GRADE4-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-010 frame 0: The same at roundness -50: between the rectangle and roundness 0's ellipse, the corners a little darker than the edges' middles. | largest difference 1.9e-7 | yes |
| FX-GRADE4-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-011 frame 0: A light vignette (amount 2, midpoint 20, feather 10) at roundness -20: just squarer than roundness 0. | largest difference 1.3e-7 | yes |
| FX-GRADE4-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-012 frame 0: FX-GRADE4-009's vignette with roundness keyed from -100 at frame 0 to 100 at frame 4, linear: the rectangle, roundness 0's ellipse, a circle. | largest difference 1.5e-7 | yes |
| FX-GRADE4-012 frame 2: FX-GRADE4-009's vignette with roundness keyed from -100 at frame 0 to 100 at frame 4, linear: the rectangle, roundness 0's ellipse, a circle. | largest difference 1.9e-7 | yes |
| FX-GRADE4-012 frame 4: FX-GRADE4-009's vignette with roundness keyed from -100 at frame 0 to 100 at frame 4, linear: the rectangle, roundness 0's ellipse, a circle. | largest difference 1.9e-7 | yes |
| FX-GRADE4-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-013 frame 0: A whole grade with sharpen 30 and roundness -60, every section in its order. | largest difference 2.6e-7 | yes |
| FX-GRADE4-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-014 frame 0: A D-571 file without sharpen, a strong vignette: D-571's own picture. | largest difference 1.9e-7 | yes |
| FX-GRADE4-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE4-015 frame 0: Sharpen 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE4-015 frame 4: Sharpen 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE4-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE4-016 frame 0: Sharpen -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE4-016 frame 4: Sharpen -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE4-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade4_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade4_013.json is saved with its 68 settings: the look, 55 numbers, four RGB curves, five hue and saturation curves and three words | {"blacks":-10,"contrast":30,"creative_saturation":95,"curve_blue":[[0,0],[255,255]],"curve_green":[[0,0],[255,255]],"curve_master":[[0,0],[64,40],[192,215],[255,255]],"curve_red":[[0,0],[255,255]],"exposure":0.3,"faded_film":20,"highlight_lightness":0,"highlight_tint_amount":50,"highlight_tint_hue":35,"highlight_wheel_amount":0,"highlight_wheel_hue":0,"highlights":-20,"hue_hue":[[30,-10],[200,20]],"hue_luma":[],"hue_saturation":[],"key_blur":0,"key_denoise":0,"key_hue":0,"key_hue_range":180,"key_hue_softness":0,"key_invert":"off","key_lightness_high":100,"key_lightness_low":0,"key_saturation_high":100,"key_saturation_low":0,"key_softness":0,"key_view":"off","look":"asset-look","look_intensity":80,"luma_saturation":[],"midtone_lightness":0,"midtone_wheel_amount":0,"midtone_wheel_hue":0,"saturation":110,"saturation_saturation":[],"secondary_contrast":0,"secondary_highlight_lightness":0,"secondary_highlight_wheel_amount":0,"secondary_highlight_wheel_hue":0,"secondary_lightness":0,"secondary_saturation":100,"secondary_shadow_lightness":0,"secondary_shadow_wheel_amount":0,"secondary_shadow_wheel_hue":0,"secondary_temperature":0,"secondary_tint":0,"secondary_wheel_amount":0,"secondary_wheel_hue":0,"secondary_wheels":"one","shadow_lightness":0,"shadow_tint_amount":60,"shadow_tint_hue":195,"shadow_wheel_amount":30,"shadow_wheel_hue":200,"shadows":20,"sharpen":30,"temperature":20,"tint":-10,"tint_balance":-20,"vibrance":30,"vignette_amount":-1.5,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":-60,"whites":10} | yes |
| fx_grade4_014.json, a D-571 file without sharpen, is saved as before: 67 settings, sharpen not written while it is where it starts (D-121's rule) | {"blacks":0,"contrast":0,"creative_saturation":100,"curve_blue":[[0,0],[255,255]],"curve_green":[[0,0],[255,255]],"curve_master":[[0,0],[255,255]],"curve_red":[[0,0],[255,255]],"exposure":0,"faded_film":0,"highlight_lightness":0,"highlight_tint_amount":0,"highlight_tint_hue":0,"highlight_wheel_amount":0,"highlight_wheel_hue":0,"highlights":0,"hue_hue":[],"hue_luma":[],"hue_saturation":[],"key_blur":0,"key_denoise":0,"key_hue":0,"key_hue_range":180,"key_hue_softness":0,"key_invert":"off","key_lightness_high":100,"key_lightness_low":0,"key_saturation_high":100,"key_saturation_low":0,"key_softness":0,"key_view":"off","look":"","look_intensity":100,"luma_saturation":[],"midtone_lightness":0,"midtone_wheel_amount":0,"midtone_wheel_hue":0,"saturation":100,"saturation_saturation":[],"secondary_contrast":0,"secondary_highlight_lightness":0,"secondary_highlight_wheel_amount":0,"secondary_highlight_wheel_hue":0,"secondary_lightness":0,"secondary_saturation":100,"secondary_shadow_lightness":0,"secondary_shadow_wheel_amount":0,"secondary_shadow_wheel_hue":0,"secondary_temperature":0,"secondary_tint":0,"secondary_wheel_amount":0,"secondary_wheel_hue":0,"secondary_wheels":"one","shadow_lightness":0,"shadow_tint_amount":0,"shadow_tint_hue":0,"shadow_wheel_amount":0,"shadow_wheel_hue":0,"shadows":0,"temperature":0,"tint":0,"tint_balance":0,"vibrance":0,"vignette_amount":-3,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":40,"whites":0} | yes |
| fx_grade4_015.json is refused in a sentence naming sharpen | Color Grade's sharpen runs from -100 to 100, and this is 101. | yes |
| fx_grade4_016.json is refused in a sentence naming sharpen | Color Grade's sharpen runs from -100 to 100, and this is -101. | yes |
| a roundness of -101 is refused in a sentence naming vignette roundness | Color Grade's vignette roundness runs from -100 to 100, and this is -101. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview leaves the sharpen and the roundness as they are (the sharpen's blur is 1 pixel of the picture drawn, D-637) | true | yes |
| it grows the drawing's bounds by nothing | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| vignette roundness -101 is refused with a sentence, and nothing changes | Color Grade's vignette roundness runs from -100 to 100, and this is -101. | yes |
| sharpen keyed to 101 is refused with a sentence, and nothing changes | Color Grade's sharpen runs from -100 to 100, and this is 101. | yes |
| sharpen -50 with a vignette at roundness -80 is taken | taken | yes |
| roundness keyed from -100 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade4_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade4_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade4_006.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade4_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade4_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade4_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade4_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade4_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade4_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1006879 pixels changed | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle on three layers, frame 0, Full | largest difference 1 of 255, 3834 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle on three layers, frame 100, Full | largest difference 1 of 255, 1471 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle on three layers, frame 239, Full | largest difference 1 of 255, 1825 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a dark vignette at roundness -100, a rectangle on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a light vignette at roundness -40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1579875 pixels changed | yes |
| the reference shot, Color Grade a light vignette at roundness -40 on three layers, frame 0, Full | largest difference 1 of 255, 3813 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a light vignette at roundness -40 on three layers, frame 100, Full | largest difference 1 of 255, 1539 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a light vignette at roundness -40 on three layers, frame 239, Full | largest difference 1 of 255, 1804 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a light vignette at roundness -40 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a light vignette at roundness -40 on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade a light vignette at roundness -40 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 731974 pixels changed | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor) on three layers, frame 0, Full | largest difference 1 of 255, 29529 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor) on three layers, frame 100, Full | largest difference 1 of 255, 28466 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor) on three layers, frame 239, Full | largest difference 1 of 255, 29109 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor) on three layers, frame 0, Draft | largest difference 1 of 255, 3071 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor) on three layers, frame 100, Draft | largest difference 1 of 255, 3076 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade sharpen 60 (drawn by the processor) on three layers, frame 239, Draft | largest difference 1 of 255, 3023 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-637 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| Color Grade as added, sharpen written at 0, changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_sharpen_100.png, sharpen 100: harder edges, the neighbour-to-neighbour differences larger than the street's; draws cleanly | [], edges 3361605909 before, 5849075848 after | yes |
| 3_sharpen_minus_100.png, sharpen -100: softer, the neighbour-to-neighbour differences smaller; draws cleanly | [], edges 3361605909 before, 3264315233 after | yes |
| 4_vignette_round_0.png, roundness 0, the frame's own oval: the corner darkened more than the edge's middle; draws cleanly | [], corner 53% darker, edge middle 29% darker | yes |
| 5_vignette_round_minus_50.png, roundness -50, squarer: the corner still darker than the edge's middle, by less; draws cleanly | [], corner 53% darker, edge middle 49% darker | yes |
| 6_vignette_round_minus_100.png, roundness -100, a rectangle: the corner and the edge's middle darkened alike; draws cleanly | [], corner 53% darker, edge middle 55% darker | yes |

## Result

115 of 115 checks pass.
