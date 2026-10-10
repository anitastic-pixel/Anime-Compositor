# D-398: Color Grade, the second unit

B-277, after Lumetri Color under our own name: RGB curves, hue versus saturation, the three colour wheels and HSL Secondary, after D-397's sections and before the vignette. Every expected pixel is `Fixtures/color_grade_2/expected_color_grade_2.json`, written by `tools/color_grade_2_reference.py` before this code existed and printed in document 25 as FX-GRADE2-001 to 040.

## FX-GRADE2-001 to 040 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE2-001 frame 0: Every setting as added, the new ones written: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE2-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-002 frame 0: Master curve, an S through (64, 40) and (192, 215): darks darker, lights lighter, black and white kept. | largest difference 1.6e-7 | yes |
| FX-GRADE2-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-003 frame 0: Red curve lifted through (128, 170), blue lowered through (128, 90): warmer midtones, black and white kept. | largest difference 1.4e-7 | yes |
| FX-GRADE2-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-004 frame 0: Hue versus saturation, one point at saturation 0: every colour its own grey, whatever its hue. | largest difference 1.2e-7 | yes |
| FX-GRADE2-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-005 frame 0: Hue versus saturation through (0, 100), (120, 200), (240, 100): the greens twice as strong, pure red and pure blue unchanged. | largest difference 2.1e-7 | yes |
| FX-GRADE2-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-006 frame 0: Hue versus saturation through (30, 150) and (200, 40): the curve runs round through 360 from the second point back to the first; oranges stronger, cyans and blues duller. | largest difference 2.1e-7 | yes |
| FX-GRADE2-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-007 frame 0: Hue versus saturation, every point at 100: no step, the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE2-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-008 frame 0: Shadow wheel hue 195 at 60: the darks teal, the lights untouched. | largest difference 1.9e-7 | yes |
| FX-GRADE2-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-009 frame 0: Midtone wheel hue 30 at 50 with midtone lightness 20: the middle warmer and lighter, black and white untouched. | largest difference 1.9e-7 | yes |
| FX-GRADE2-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-010 frame 0: Highlight wheel hue 220 at 40 with highlight lightness -30: the lights cooler and lower, the darks untouched. | largest difference 1.2e-7 | yes |
| FX-GRADE2-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-011 frame 0: Shadow lightness 40 and highlight lightness -20, no colour: black lifted to 0.2, white lowered to 0.9, greys only. | largest difference 1.8e-7 | yes |
| FX-GRADE2-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-012 frame 0: Key the reds (hue 0, 20 either side, no softness), secondary saturation 0: red grey, everything else, the skin and orange too, untouched. | largest difference 1.9e-7 | yes |
| FX-GRADE2-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-013 frame 0: The same key with hue softness 30: hues between 20 and 50 degrees away partly grey, the skin (about 25) and orange (about 33); yellow untouched. | largest difference 1.7e-7 | yes |
| FX-GRADE2-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-014 frame 0: Key saturation 50 to 100, softness 10, secondary lightness -40: the strong colours darker, greys and the dull tones untouched. | largest difference 5.9e-7 | yes |
| FX-GRADE2-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-015 frame 0: Key lightness 55 to 100, secondary temperature -50: the light colours cooler, the darks untouched. | largest difference 1.7e-7 | yes |
| FX-GRADE2-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-016 frame 0: FX-GRADE2-012 inverted: everything but the reds grey. | largest difference 1.2e-7 | yes |
| FX-GRADE2-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-017 frame 0: FX-GRADE2-013's key shown as the mask: white where the key takes all, black where none, greys between. | largest difference 1.1e-6 | yes |
| FX-GRADE2-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-018 frame 0: The mask view with no correction set, inverted: the mask still shown. | largest difference 3.0e-8 | yes |
| FX-GRADE2-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-019 frame 0: Key everything (as added), secondary contrast 60 and tint 30: a second correction over the whole drawing. | largest difference 1.8e-7 | yes |
| FX-GRADE2-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-020 frame 0: Key the skin (hue 25, 10 either side, softness 20), secondary wheel hue 200 at 70: the skin and warm tones cooled, the rest untouched. | largest difference 3.7e-7 | yes |
| FX-GRADE2-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-021 frame 0: Key hue 350, 20 either side: the key wraps through 0 and takes red; secondary saturation 0. | largest difference 1.9e-7 | yes |
| FX-GRADE2-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-022 frame 0: A whole grade: D-397's sections, the master S-curve, a hue curve, the three wheels, a skin key cooled and a dark vignette, each in its order. | largest difference 3.7e-7 | yes |
| FX-GRADE2-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-023 frame 0: Shadow lightness keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 untouched, black lifted to 0.15 at frame 2 and 0.3 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE2-023 frame 2: Shadow lightness keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 untouched, black lifted to 0.15 at frame 2 and 0.3 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE2-023 frame 4: Shadow lightness keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 untouched, black lifted to 0.15 at frame 2 and 0.3 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE2-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-024 frame 0: Key hue keyed from 0 at frame 0 to 120 at frame 4, 20 either side, secondary saturation 0: the reds grey at frame 0, the greens at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE2-024 frame 4: Key hue keyed from 0 at frame 0 to 120 at frame 4, 20 either side, secondary saturation 0: the reds grey at frame 0, the greens at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE2-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-025 frame 0: Wheels and a key moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-GRADE2-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-026 frame 0: The mask view with a dark vignette: the mask shown, the vignette not drawn over it. | largest difference 3.0e-8 | yes |
| FX-GRADE2-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-027 frame 0: A D-397 file without the new settings, contrast 40: D-397's own picture. | largest difference 1.7e-7 | yes |
| FX-GRADE2-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE2-028 frame 0: Key hue range 181, past 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-028 frame 4: Key hue range 181, past 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-029 frame 0: Key softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-029 frame 4: Key softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-030 frame 0: Secondary saturation 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-030 frame 4: Secondary saturation 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-031 frame 0: Shadow lightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-031 frame 4: Shadow lightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-032 frame 0: Master curve with a point at 256, past 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-032 frame 4: Master curve with a point at 256, past 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-033 frame 0: Red curve with its ins not rising. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-033 frame 4: Red curve with its ins not rising. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-034 frame 0: Green curve of one point. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-034 frame 4: Green curve of one point. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-035 frame 0: Hue versus saturation with a hue of 360, a full turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-035 frame 4: Hue versus saturation with a hue of 360, a full turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-036 frame 0: Hue versus saturation with a saturation of 201. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-036 frame 4: Hue versus saturation with a saturation of 201. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-037 frame 0: Hue versus saturation with its hues not rising. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-037 frame 4: Hue versus saturation with its hues not rising. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-038 frame 0: Hue versus saturation of 17 points, past 16. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-038 frame 4: Hue versus saturation of 17 points, past 16. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-039 frame 0: Key invert "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-039 frame 4: Key invert "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE2-040 frame 0: Key view "grey", not "off" or "mask". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-040 frame 4: Key view "grey", not "off" or "mask". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE2-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade2_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade2_022.json is saved with its 54 settings: the look, 46 numbers, four curves, the hue curve and two words | {"blacks":-10,"contrast":30,"creative_saturation":95,"curve_blue":[[0,0],[255,255]],"curve_green":[[0,0],[255,255]],"curve_master":[[0,0],[64,40],[192,215],[255,255]],"curve_red":[[0,0],[255,255]],"exposure":0.3,"faded_film":20,"highlight_lightness":0,"highlight_tint_amount":50,"highlight_tint_hue":35,"highlight_wheel_amount":20,"highlight_wheel_hue":40,"highlights":-20,"hue_saturation":[[0,110],[120,90],[240,120]],"key_hue":25,"key_hue_range":10,"key_hue_softness":20,"key_invert":"off","key_lightness_high":100,"key_lightness_low":0,"key_saturation_high":100,"key_saturation_low":0,"key_softness":0,"key_view":"off","look":"asset-look","look_intensity":80,"midtone_lightness":10,"midtone_wheel_amount":0,"midtone_wheel_hue":0,"saturation":110,"secondary_contrast":0,"secondary_lightness":0,"secondary_saturation":80,"secondary_temperature":0,"secondary_tint":0,"secondary_wheel_amount":30,"secondary_wheel_hue":200,"shadow_lightness":0,"shadow_tint_amount":60,"shadow_tint_hue":195,"shadow_wheel_amount":30,"shadow_wheel_hue":200,"shadows":20,"temperature":20,"tint":-10,"tint_balance":-20,"vibrance":30,"vignette_amount":-1.5,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":0,"whites":10} | yes |
| fx_grade2_027.json, a D-397 file without the new settings, is saved as before: none of them written while each is where it starts (D-121's rule) | {"blacks":0,"contrast":40,"creative_saturation":100,"exposure":0,"faded_film":0,"highlight_tint_amount":0,"highlight_tint_hue":0,"highlights":0,"look":"","look_intensity":100,"saturation":100,"shadow_tint_amount":0,"shadow_tint_hue":0,"shadows":0,"temperature":0,"tint":0,"tint_balance":0,"vibrance":0,"vignette_amount":0,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":0,"whites":0} | yes |
| fx_grade2_028.json is refused in a sentence naming key hue range | Color Grade's key hue range runs from 0 to 180, and this is 181. | yes |
| fx_grade2_029.json is refused in a sentence naming key softness | Color Grade's key softness runs from 0 to 100, and this is 101. | yes |
| fx_grade2_030.json is refused in a sentence naming secondary saturation | Color Grade's secondary saturation runs from 0 to 200, and this is 201. | yes |
| fx_grade2_031.json is refused in a sentence naming shadow lightness | Color Grade's shadow lightness runs from -100 to 100, and this is -101. | yes |
| fx_grade2_032.json is refused in a sentence naming Color Grade's master curve | Color Grade's master curve's points run from 0 to 255, and this has 256. | yes |
| fx_grade2_033.json is refused in a sentence naming Color Grade's red curve | Color Grade's red curve's in goes up from point to point, and 128 is not above 128. | yes |
| fx_grade2_034.json is refused in a sentence naming Color Grade's green curve | Color Grade's green curve takes 2 to 16 points, and this has 1. | yes |
| fx_grade2_035.json is refused in a sentence naming hue versus saturation | Color Grade's hue versus saturation takes hues from 0 to under 360 and saturations from 0 to 200, and this point is [360.0, 50.0]. | yes |
| fx_grade2_036.json is refused in a sentence naming hue versus saturation | Color Grade's hue versus saturation takes hues from 0 to under 360 and saturations from 0 to 200, and this point is [90.0, 201.0]. | yes |
| fx_grade2_037.json is refused in a sentence naming hue versus saturation | Color Grade's hue versus saturation's hue goes up from point to point, and 100 is not above 200. | yes |
| fx_grade2_038.json is refused in a sentence naming hue versus saturation | Color Grade's hue versus saturation takes up to 16 points, and this has 17. | yes |
| fx_grade2_039.json is refused in a sentence naming key invert | Color Grade's key invert is "off" or "on", and this is "yes". | yes |
| fx_grade2_040.json is refused in a sentence naming key view | Color Grade's key view is "off" or "mask", and this is "grey". | yes |
| a file with a Color Grade whose key invert is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Grade whose red curve is numbers, not points is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| key hue range 181 is refused with a sentence, and nothing changes | Color Grade's key hue range runs from 0 to 180, and this is 181. | yes |
| key invert "yes" is refused with a sentence, and nothing changes | Color Grade's key invert is "off" or "on", and this is "yes". | yes |
| hue versus saturation with its hues falling is refused with a sentence, and nothing changes | Color Grade's hue versus saturation's hue goes up from point to point, and 100 is not above 200. | yes |
| shadow lightness keyed to 101 is refused with a sentence, and nothing changes | Color Grade's shadow lightness runs from -100 to 100, and this is 101. | yes |
| a red curve, a hue curve and the mask view is taken | taken | yes |
| key hue keyed from 0 to 120 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade2_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade2_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade2_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade2_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade2_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade2_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade2_024.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade2_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade2_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger on three layers, frame 0, Full | largest difference 1 of 255, 561 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger on three layers, frame 100, Full | largest difference 1 of 255, 760 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger on three layers, frame 239, Full | largest difference 1 of 255, 622 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger on three layers, frame 0, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade curves and hue: a master S-curve, the red lifted, the greens stronger on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073599 pixels changed | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered on three layers, frame 0, Full | largest difference 1 of 255, 609 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered on three layers, frame 100, Full | largest difference 1 of 255, 1161 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered on three layers, frame 239, Full | largest difference 1 of 255, 635 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered on three layers, frame 0, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade wheels: teal shadows, warm midtones lifted, cool highlights lowered on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 38889 pixels changed | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled on three layers, frame 0, Full | largest difference 1 of 255, 3777 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled on three layers, frame 239, Full | largest difference 1 of 255, 1768 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade HSL secondary: the warm tones keyed with soft edges, cooled and dulled on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-398 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| Color Grade as added, the new settings written, changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_s_curve.png, a master S-curve: the brightness spread wider; draws cleanly | [], mean distance from the mean brightness 139 before, 200 after | yes |
| 3_hue_curve.png, hue versus saturation: greens stronger, the blue sky grey; draws cleanly | [], 128544 pixels changed | yes |
| 4_wheels.png, teal shadow wheel and orange highlight wheel: the street changed; draws cleanly | [], 129600 pixels changed | yes |
| 5_colour_pop.png, the reds keyed and inverted, secondary saturation 0: everything but the reds grey; draws cleanly | [], grey pixels 1056 before, 120564 after, of 129600 | yes |
| 6_key_mask.png, the same key shown as the mask: every pixel grey, white where the correction applies; draws cleanly | [], grey pixels 129600 of 129600 | yes |

## Result

235 of 235 checks pass.
