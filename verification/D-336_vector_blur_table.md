# B-220: CC Vector Blur

D-336, from P-26's tutorial 2. Every expected pixel is `Fixtures/vector_blur/expected_vector_blur.json`, written by `tools/vector_blur_reference.py` before this code existed and printed in document 25 as FX-VBLUR-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-VBLUR-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-VBLUR-001 frame 0: As added: Natural, Amount 10, Ridge Smoothness 10, the layer's own lightness as the map, Map Softness 30: a soft smear along the slopes of its blurred brightness. | largest difference 2.5e-7 | yes |
| FX-VBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-002 frame 0: Amount 0: nothing moves. | largest difference 1.5e-7 | yes |
| FX-VBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-003 frame 0: The ramp as the map, Constant Length, Amount 3, Map Softness 0: its slope runs left to right, so rows 1 to 8 are smeared straight across, 3 pixels each way, evenly. | largest difference 1.6e-7 | yes |
| FX-VBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-004 frame 0: The same as Natural, Ridge Smoothness 10: the ramp's gentle slope, 1/15 a pixel, makes the smear 3 * 6.67 / sqrt(6.67^2 + 10^2), 1.66 pixels each way, fading. | largest difference 2.2e-7 | yes |
| FX-VBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-005 frame 0: The same as Perpendicular: along the slope's contour, so rows are left alone and columns smeared up and down. | largest difference 1.9e-7 | yes |
| FX-VBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-006 frame 0: Natural with Angle Offset 90: exactly Perpendicular, FX-VBLUR-005. | largest difference 1.9e-7 | yes |
| FX-VBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-007 frame 0: A black solid as the map, Direction Center, Angle Offset 90, Amount 3: the height is 0 everywhere, so every pixel is smeared 3 pixels each way across, evenly; rows 1 to 8 are FX-VBLUR-003. | largest difference 8.3e-8 | yes |
| FX-VBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-008 frame 0: The same as Direction Fading: only forward, from the pixels to the right. | largest difference 1.6e-7 | yes |
| FX-VBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-009 frame 0: The ramp with Direction Center, Revolutions 1, Amount 2: the direction turns once round from black to white, straight up at the left. | largest difference 3.8e-7 | yes |
| FX-VBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-010 frame 0: FX-VBLUR-004 with Map Softness 4: the ramp's edges soften, so the slope bends near them. | largest difference 2.4e-7 | yes |
| FX-VBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-011 frame 0: A white disc as the map, Natural, Ridge Smoothness 0, Map Softness 0, Amount 2: smeared in and out across its rim, and nowhere else. | largest difference 1.5e-7 | yes |
| FX-VBLUR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-012 frame 0: The disc as Perpendicular: smeared round its rim. | largest difference 1.8e-7 | yes |
| FX-VBLUR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-013 frame 0: Property Alpha on the layer itself, Constant Length, Amount 2, Map Softness 0: only the pixels on the drawing's edges and round its clear corner have a slope. | largest difference 1.5e-7 | yes |
| FX-VBLUR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-014 frame 0: Property Hue on the layer itself, Ridge Smoothness 5, Amount 3, Map Softness 2. | largest difference 2.6e-7 | yes |
| FX-VBLUR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-015 frame 0: Property Saturation on the layer itself, Constant Length, Amount 2, Map Softness 2. | largest difference 1.0e-5 | yes |
| FX-VBLUR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-016 frame 0: FX-VBLUR-003 with Amount keyed from 0 at frame 0 to 6 at frame 4: frame 0 untouched and frame 2 is FX-VBLUR-003. | largest difference 1.5e-7 | yes |
| FX-VBLUR-016 frame 2: FX-VBLUR-003 with Amount keyed from 0 at frame 0 to 6 at frame 4: frame 0 untouched and frame 2 is FX-VBLUR-003. | largest difference 1.6e-7 | yes |
| FX-VBLUR-016 frame 4: FX-VBLUR-003 with Amount keyed from 0 at frame 0 to 6 at frame 4: frame 0 untouched and frame 2 is FX-VBLUR-003. | largest difference 1.3e-7 | yes |
| FX-VBLUR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-017 frame 0: A layer that is not in the composition, `gone`: the layer itself is the map, as FX-VBLUR-001, and the warning every frame. | largest difference 2.5e-7 | yes |
| FX-VBLUR-017 frame 4: A layer that is not in the composition, `gone`: the layer itself is the map, as FX-VBLUR-001, and the warning every frame. | largest difference 2.5e-7 | yes |
| FX-VBLUR-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-VBLUR-018 frame 0: FX-VBLUR-003 on the holder moved 2 right and 1 down: the same picture moved, since the map lies on the layer. | largest difference 1.3e-7 | yes |
| FX-VBLUR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VBLUR-019 frame 0: A type written "Natural", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-019 frame 4: A type written "Natural", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-020 frame 0: Amount 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-020 frame 4: Amount 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-021 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-021 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-022 frame 0: Ridge Smoothness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-022 frame 4: Ridge Smoothness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-023 frame 0: Map Softness keyed to -1 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-023 frame 4: Map Softness keyed to -1 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-024 frame 0: A property written "brightness". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-024 frame 4: A property written "brightness". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-025 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-025 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-026 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-026 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VBLUR-027 frame 0: Angle Offset 3601, past 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-027 frame 4: Angle Offset 3601, past 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-VBLUR-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vblur_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vblur_019.json is refused in a sentence | CC Vector Blur's type is natural, constant, perpendicular, direction_center or direction_fading, and this is "Natural". | yes |
| fx_vblur_024.json is refused in a sentence | CC Vector Blur's property is red, green, blue, alpha, luminance, lightness, hue or saturation, and this is "brightness". | yes |
| fx_vblur_026.json is refused in a sentence | CC Vector Blur's vector map is the name of a layer of this composition, and this is 3. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| type "Natural" is refused with a sentence, and nothing changes | CC Vector Blur's type is natural, constant, perpendicular, direction_center or direction_fading, and this is "Natural". | yes |
| amount 501 is refused with a sentence, and nothing changes | CC Vector Blur's amount runs from 0 to 500, and this is 501. | yes |
| type perpendicular, amount 4, is taken | taken | yes |
| type direction_fading, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vblur_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vblur_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vblur_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vblur_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a street, in `verification/D-336 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street, draws cleanly | [] | yes |
| as_added.png, as added: Natural, Amount 10, its own lightness, Map Softness 30 | [], 12107 pixels changed, 7608 of them more than 125 from the middle | yes |
| natural_disc.png, Natural over the disc, Amount 12, Ridge Smoothness 0: smeared out from the middle, and nowhere past the disc's edge | [], 17682 pixels changed, 0 of them more than 125 from the middle | yes |
| perpendicular_disc.png, Perpendicular over the disc, Amount 12, Ridge Smoothness 0: smeared round the middle, and nowhere past the disc's edge | [], 16234 pixels changed, 0 of them more than 125 from the middle | yes |
| twist_disc.png, Direction Center over the disc, Amount 6, Revolutions 1: the smear's way turns once from the edge to the middle | [], 33434 pixels changed, 19327 of them more than 125 from the middle | yes |
| tutorial.png, tutorial 2's setting, Natural, Amount 4, Ridge Smoothness 20, its own lightness | [], 9079 pixels changed, 5776 of them more than 125 from the middle | yes |

## Result

111 of 111 checks pass.
