# D-571: Color Grade, the third unit

B-450, after Lumetri Color under our own name: the hue versus hue, hue versus luma, luma versus saturation and saturation versus saturation curves after hue versus saturation, and HSL Secondary's key denoise and blur and its three wheels. Every expected pixel is `Fixtures/color_grade_3/expected_color_grade_3.json`, written by `tools/color_grade_3_reference.py` before this code existed and printed in document 25 as FX-GRADE3-001 to 034.

## FX-GRADE3-001 to 034 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE3-001 frame 0: Every setting as added, the new ones written: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE3-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-002 frame 0: Hue versus hue, one point shifting 60 degrees: every colour turned a sixth of the way round (red to yellow, green to cyan), its lightness and saturation kept; greys, black and white untouched. | largest difference 2.6e-7 | yes |
| FX-GRADE3-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-003 frame 0: Hue versus hue through (0, 0), (120, 60), (240, 0): the greens turned toward cyan, pure red and pure blue unchanged. | largest difference 2.6e-7 | yes |
| FX-GRADE3-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-004 frame 0: Hue versus hue, every shift 0, and every other new curve flat: no step, the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE3-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-005 frame 0: Hue versus luma, one point at 50: each colour lighter by a quarter of its chroma (pure colours by 0.25), greys untouched. | largest difference 2.0e-7 | yes |
| FX-GRADE3-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-006 frame 0: Hue versus luma through (0, -60) and (180, 60): reds darker, cyans lighter, greys untouched. | largest difference 2.2e-7 | yes |
| FX-GRADE3-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-007 frame 0: Luma versus saturation, one point at saturation 0: every colour its own grey, whatever its luma. | largest difference 1.2e-7 | yes |
| FX-GRADE3-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-008 frame 0: Luma versus saturation through (20, 0) and (80, 150): colours darker than luma 20 grey, lighter than 80 half as strong again, eased between. | largest difference 2.0e-7 | yes |
| FX-GRADE3-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-009 frame 0: Saturation versus saturation through (0, 100), (50, 100), (100, 40): the strong colours duller, the dull ones untouched. | largest difference 2.9e-7 | yes |
| FX-GRADE3-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-010 frame 0: Saturation versus saturation through (0, 200) and (40, 100): the dull colours stronger, those of saturation 40 and up untouched. | largest difference 1.9e-7 | yes |
| FX-GRADE3-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-011 frame 0: Three secondary wheels, the key taking everything, the shadow wheel hue 200 at 60: the darks teal, white untouched. | largest difference 1.9e-7 | yes |
| FX-GRADE3-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-012 frame 0: Three secondary wheels: highlight wheel hue 40 at 50 with highlight lightness -20, and the secondary (midtone) lightness 20: the lights warmer and lower, the middle lighter, black untouched. | largest difference 1.7e-7 | yes |
| FX-GRADE3-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-013 frame 0: Three secondary wheels with only the secondary wheel, hue 30 at 50: now the midtones' wheel, so black and white are untouched where one wheel would move them. | largest difference 1.7e-7 | yes |
| FX-GRADE3-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-014 frame 0: One secondary wheel (as added) with the shadow and highlight wheels set: they are not used, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-GRADE3-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-015 frame 0: FX-GRADE2-017's red key (softness 30) shown as the mask with denoise 30: the key through a median of radius 3: red's lone keyed column, among unkeyed neighbours, is outvoted, and the warm tones' key evens out across their columns. | largest difference 6.5e-7 | yes |
| FX-GRADE3-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-016 frame 0: The red key shown as the mask with blur 20: the key softened by a Gaussian of sigma 2, red's column spread to its neighbours. | largest difference 3.0e-8 | yes |
| FX-GRADE3-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-017 frame 0: The softened red key (softness 30) with denoise 15 and blur 10, secondary saturation 0: red's lone column outvoted (a quarter grey once the blur brings some key back), the warm tones mostly grey, and blue beside them partly grey where the blurred key reaches it. | largest difference 1.4e-7 | yes |
| FX-GRADE3-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-018 frame 0: Denoise 5, a radius of half a pixel: below 1, nothing; FX-GRADE2-012's picture. | largest difference 1.9e-7 | yes |
| FX-GRADE3-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-019 frame 0: The red key's mask with blur keyed from 0 at frame 0 to 40 at frame 4, linear: a hard key at frame 0, sigma 2 at frame 2, sigma 4 at frame 4. | largest difference 3.0e-8 | yes |
| FX-GRADE3-019 frame 2: The red key's mask with blur keyed from 0 at frame 0 to 40 at frame 4, linear: a hard key at frame 0, sigma 2 at frame 2, sigma 4 at frame 4. | largest difference 3.0e-8 | yes |
| FX-GRADE3-019 frame 4: The red key's mask with blur keyed from 0 at frame 0 to 40 at frame 4, linear: a hard key at frame 0, sigma 2 at frame 2, sigma 4 at frame 4. | largest difference 3.0e-8 | yes |
| FX-GRADE3-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-020 frame 0: Hue versus hue and a blurred key moved three pixels right: the same, moved. | largest difference 2.9e-7 | yes |
| FX-GRADE3-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-021 frame 0: A whole grade: D-397's and D-398's sections, the four new curves, a skin key denoised and blurred with three wheels, and a dark vignette, each in its order. | largest difference 2.3e-7 | yes |
| FX-GRADE3-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-022 frame 0: A D-398 file without the new settings, a red key at secondary saturation 0: D-398's own picture. | largest difference 1.9e-7 | yes |
| FX-GRADE3-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE3-023 frame 0: Hue versus hue with a shift of 181, past 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-023 frame 4: Hue versus hue with a shift of 181, past 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-024 frame 0: Hue versus hue with a hue of 360, a full turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-024 frame 4: Hue versus hue with a hue of 360, a full turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-025 frame 0: Hue versus luma with a luma of -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-025 frame 4: Hue versus luma with a luma of -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-026 frame 0: Hue versus luma with its hues not rising. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-026 frame 4: Hue versus luma with its hues not rising. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-027 frame 0: Luma versus saturation with a luma of 101, past 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-027 frame 4: Luma versus saturation with a luma of 101, past 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-028 frame 0: Luma versus saturation with a point of three numbers. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-028 frame 4: Luma versus saturation with a point of three numbers. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-029 frame 0: Saturation versus saturation with a saturation of 201. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-029 frame 4: Saturation versus saturation with a saturation of 201. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-030 frame 0: Saturation versus saturation of 17 points, past 16. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-030 frame 4: Saturation versus saturation of 17 points, past 16. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-031 frame 0: Secondary wheels "two", not "one" or "three". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-031 frame 4: Secondary wheels "two", not "one" or "three". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-032 frame 0: Key denoise 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-032 frame 4: Key denoise 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-033 frame 0: Key blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-033 frame 4: Key blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE3-034 frame 0: Secondary highlight lightness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-034 frame 4: Secondary highlight lightness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE3-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade3_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade3_021.json is saved with its 67 settings: the look, 54 numbers, four RGB curves, five hue and saturation curves and three words | {"blacks":-10,"contrast":30,"creative_saturation":95,"curve_blue":[[0,0],[255,255]],"curve_green":[[0,0],[255,255]],"curve_master":[[0,0],[64,40],[192,215],[255,255]],"curve_red":[[0,0],[255,255]],"exposure":0.3,"faded_film":20,"highlight_lightness":0,"highlight_tint_amount":50,"highlight_tint_hue":35,"highlight_wheel_amount":0,"highlight_wheel_hue":0,"highlights":-20,"hue_hue":[[30,-10],[200,20]],"hue_luma":[[0,10],[240,-20]],"hue_saturation":[[0,110],[120,90],[240,120]],"key_blur":8,"key_denoise":12,"key_hue":25,"key_hue_range":10,"key_hue_softness":20,"key_invert":"off","key_lightness_high":100,"key_lightness_low":0,"key_saturation_high":100,"key_saturation_low":0,"key_softness":0,"key_view":"off","look":"asset-look","look_intensity":80,"luma_saturation":[[10,80],[90,110]],"midtone_lightness":10,"midtone_wheel_amount":0,"midtone_wheel_hue":0,"saturation":110,"saturation_saturation":[[0,120],[80,90]],"secondary_contrast":0,"secondary_highlight_lightness":-10,"secondary_highlight_wheel_amount":0,"secondary_highlight_wheel_hue":0,"secondary_lightness":0,"secondary_saturation":80,"secondary_shadow_lightness":0,"secondary_shadow_wheel_amount":20,"secondary_shadow_wheel_hue":220,"secondary_temperature":0,"secondary_tint":0,"secondary_wheel_amount":30,"secondary_wheel_hue":200,"secondary_wheels":"three","shadow_lightness":0,"shadow_tint_amount":60,"shadow_tint_hue":195,"shadow_wheel_amount":30,"shadow_wheel_hue":200,"shadows":20,"temperature":20,"tint":-10,"tint_balance":-20,"vibrance":30,"vignette_amount":-1.5,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":0,"whites":10} | yes |
| fx_grade3_022.json, a D-398 file without the new settings, is saved as before: 54 settings, none of the new ones written while each is where it starts (D-121's rule) | {"blacks":0,"contrast":0,"creative_saturation":100,"curve_blue":[[0,0],[255,255]],"curve_green":[[0,0],[255,255]],"curve_master":[[0,0],[255,255]],"curve_red":[[0,0],[255,255]],"exposure":0,"faded_film":0,"highlight_lightness":0,"highlight_tint_amount":0,"highlight_tint_hue":0,"highlight_wheel_amount":0,"highlight_wheel_hue":0,"highlights":0,"hue_saturation":[],"key_hue":0,"key_hue_range":20,"key_hue_softness":0,"key_invert":"off","key_lightness_high":100,"key_lightness_low":0,"key_saturation_high":100,"key_saturation_low":0,"key_softness":0,"key_view":"off","look":"","look_intensity":100,"midtone_lightness":0,"midtone_wheel_amount":0,"midtone_wheel_hue":0,"saturation":100,"secondary_contrast":0,"secondary_lightness":0,"secondary_saturation":0,"secondary_temperature":0,"secondary_tint":0,"secondary_wheel_amount":0,"secondary_wheel_hue":0,"shadow_lightness":0,"shadow_tint_amount":0,"shadow_tint_hue":0,"shadow_wheel_amount":0,"shadow_wheel_hue":0,"shadows":0,"temperature":0,"tint":0,"tint_balance":0,"vibrance":0,"vignette_amount":0,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":0,"whites":0} | yes |
| fx_grade3_023.json is refused in a sentence naming hue versus hue | Color Grade's hue versus hue takes hues from 0 to under 360 and shifts from -180 to 180, and this point is [0.0, 181.0]. | yes |
| fx_grade3_024.json is refused in a sentence naming hue versus hue | Color Grade's hue versus hue takes hues from 0 to under 360 and shifts from -180 to 180, and this point is [360.0, 20.0]. | yes |
| fx_grade3_025.json is refused in a sentence naming hue versus luma | Color Grade's hue versus luma takes hues from 0 to under 360 and lumas from -100 to 100, and this point is [90.0, -101.0]. | yes |
| fx_grade3_026.json is refused in a sentence naming hue versus luma | Color Grade's hue versus luma's hue goes up from point to point, and 100 is not above 200. | yes |
| fx_grade3_027.json is refused in a sentence naming luma versus saturation | Color Grade's luma versus saturation takes lumas from 0 to 100 and saturations from 0 to 200, and this point is [101.0, 50.0]. | yes |
| fx_grade3_028.json is refused in a sentence naming luma versus saturation | Each point of Color Grade's luma versus saturation is two numbers, luma then saturation, and this one is [0.0, 100.0, 5.0]. | yes |
| fx_grade3_029.json is refused in a sentence naming saturation versus saturation | Color Grade's saturation versus saturation takes saturations from 0 to 100 and saturations from 0 to 200, and this point is [50.0, 201.0]. | yes |
| fx_grade3_030.json is refused in a sentence naming saturation versus saturation | Color Grade's saturation versus saturation takes up to 16 points, and this has 17. | yes |
| fx_grade3_031.json is refused in a sentence naming secondary wheels | Color Grade's secondary wheels are "one" or "three", and this is "two". | yes |
| fx_grade3_032.json is refused in a sentence naming key denoise | Color Grade's key denoise runs from 0 to 100, and this is 101. | yes |
| fx_grade3_033.json is refused in a sentence naming key blur | Color Grade's key blur runs from 0 to 100, and this is -1. | yes |
| fx_grade3_034.json is refused in a sentence naming secondary highlight lightness | Color Grade's secondary highlight lightness runs from -100 to 100, and this is 101. | yes |
| a file with a Color Grade whose secondary wheels is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Grade whose hue versus luma is numbers, not points is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the key denoise and blur, distances, and leaves the other settings | key denoise 6, key blur 4 | yes |
| it grows the drawing's bounds by nothing | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue versus hue with a shift of 181 is refused with a sentence, and nothing changes | Color Grade's hue versus hue takes hues from 0 to under 360 and shifts from -180 to 180, and this point is [0.0, 181.0]. | yes |
| secondary wheels "two" is refused with a sentence, and nothing changes | Color Grade's secondary wheels are "one" or "three", and this is "two". | yes |
| luma versus saturation with its lumas falling is refused with a sentence, and nothing changes | Color Grade's luma versus saturation's luma goes up from point to point, and 20 is not above 80. | yes |
| key blur keyed to 101 is refused with a sentence, and nothing changes | Color Grade's key blur runs from 0 to 100, and this is 101. | yes |
| hue versus luma, saturation versus saturation and three secondary wheels is taken | taken | yes |
| key denoise keyed from 0 to 40 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade3_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_019.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade3_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade3_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 2 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade3_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1902763 pixels changed | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger on three layers, frame 0, Full | largest difference 1 of 255, 3005 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger on three layers, frame 100, Full | largest difference 1 of 255, 1211 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger on three layers, frame 239, Full | largest difference 1 of 255, 1172 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger on three layers, frame 0, Draft | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 38876 pixels changed | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered on three layers, frame 0, Full | largest difference 1 of 255, 3777 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered on three layers, frame 239, Full | largest difference 1 of 255, 1768 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-571 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| Color Grade as added, the new settings written, changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_hue_vs_hue.png, hue versus hue, the greens turned toward cyan and the blues toward purple: the street changed; draws cleanly | [], 128544 pixels changed | yes |
| 3_hue_vs_luma.png, hue versus luma, the blue sky darker and the warm walls lighter: the street changed; draws cleanly | [], 128544 pixels changed | yes |
| 4_luma_vs_saturation.png, luma versus saturation, the shadows grey and the lights stronger: the street changed; draws cleanly | [], 123864 pixels changed | yes |
| 5_saturation_vs_saturation.png, saturation versus saturation, the dull colours stronger and the strong ones duller: the street changed; draws cleanly | [], 100704 pixels changed | yes |
| 6_three_wheels.png, every colour keyed, three secondary wheels: teal shadows, warm highlights, the midtones as they were: the street changed; draws cleanly | [], 129600 pixels changed | yes |
| 7_key_mask.png, the reds keyed, shown as the mask: every pixel grey; draws cleanly | [], grey pixels 129600 of 129600 | yes |
| 8_key_denoised.png, the same key denoised 30 (a median of 3 pixels round): the street has no specks to remove, so the key's square corners come out rounded, still hard-edged, every pixel black or white; draws cleanly | [], 426 pixels changed from 7, soft-edge pixels 0 | yes |
| 9_key_blurred.png, the same key blurred 30 (a blur of 3 pixels): a wider soft edge than 7; draws cleanly | [], soft-edge pixels 0 before, 15973 after | yes |

## Result

195 of 195 checks pass.
