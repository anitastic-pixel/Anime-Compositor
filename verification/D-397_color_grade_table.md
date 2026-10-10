# D-397: Color Grade

B-276, after Lumetri Color under our own name, its first unit: Basic Correction (white balance, exposure, contrast, highlights, shadows, whites, blacks, saturation), Creative (a look file at its intensity, faded film, vibrance, saturation, shadow and highlight tints) and Vignette, each step in that order. Every expected pixel is `Fixtures/color_grade/expected_color_grade.json`, written by `tools/color_grade_reference.py` before this code existed and printed in document 25 as FX-GRADE-001 to 034.

## FX-GRADE-001 to 034 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE-001 frame 0: Every setting as added, no look: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-002 frame 0: Temperature 60: warmer, red up and blue down in linear light, white's brightness kept, so white itself turns cream with its red held at 1. | largest difference 1.6e-7 | yes |
| FX-GRADE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-003 frame 0: Temperature -60, tint 40: cooler and toward magenta. | largest difference 1.6e-7 | yes |
| FX-GRADE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-004 frame 0: Exposure 1: one stop brighter in linear light, the brightest colours held at white. | largest difference 2.0e-7 | yes |
| FX-GRADE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-005 frame 0: Exposure -1.5: a stop and a half darker. | largest difference 7.3e-8 | yes |
| FX-GRADE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-006 frame 0: Contrast 80: darks darker, lights lighter, middle grey and the pure colours' 0 and 1 kept. | largest difference 1.5e-7 | yes |
| FX-GRADE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-007 frame 0: Contrast -60: flatter, everything toward the middle. | largest difference 3.0e-7 | yes |
| FX-GRADE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-008 frame 0: Highlights -70, shadows 70: the lights brought down and the darks opened up, black and white kept. | largest difference 2.6e-7 | yes |
| FX-GRADE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-009 frame 0: Whites 50, blacks -50: white past 1 and held, black below 0 and held, more contrast at the ends. | largest difference 1.8e-7 | yes |
| FX-GRADE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-010 frame 0: Whites -60, blacks 60: white lowered and black lifted, a faded picture. | largest difference 1.0e-7 | yes |
| FX-GRADE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-011 frame 0: Saturation 0: every colour its own grey, L of its encoded colour. | largest difference 1.2e-7 | yes |
| FX-GRADE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-012 frame 0: Saturation 160: stronger colours, the pure ones already at the edge held there. | largest difference 2.3e-7 | yes |
| FX-GRADE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-013 frame 0: The look at intensity 100: the look file's table as Color Lookup reads it. | largest difference 1.7e-7 | yes |
| FX-GRADE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-014 frame 0: The look at intensity 50: halfway from the drawing to the look. | largest difference 2.1e-7 | yes |
| FX-GRADE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-015 frame 0: The look at intensity 200: the look's change doubled, held inside 0 to 1. | largest difference 2.3e-7 | yes |
| FX-GRADE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-016 frame 0: The look named, intensity 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-017 frame 0: Faded film 70: black lifted to about 0.18 and white lowered to 0.93, a washed-out print. | largest difference 1.7e-7 | yes |
| FX-GRADE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-018 frame 0: Vibrance 80: the dull colours (skin, the warm tones) strengthened more than the strong ones; greys unchanged. | largest difference 1.9e-7 | yes |
| FX-GRADE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-019 frame 0: Vibrance -50 with creative saturation 140. | largest difference 2.6e-7 | yes |
| FX-GRADE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-020 frame 0: Shadow tint hue 195 at 60, highlight tint hue 35 at 50: teal shadows, orange highlights, middle grey untouched. | largest difference 1.8e-7 | yes |
| FX-GRADE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-021 frame 0: The same with tint balance 60: the shadows' tint weaker, the highlights' stronger. | largest difference 1.8e-7 | yes |
| FX-GRADE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-022 frame 0: Vignette -3, midpoint 50, roundness 0, feather 50: the corners darkened toward black by 60 per cent, the middle untouched. | largest difference 1.6e-7 | yes |
| FX-GRADE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-023 frame 0: Vignette 2, midpoint 20, roundness 100, feather 10: a round, firm white edge. | largest difference 1.9e-7 | yes |
| FX-GRADE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-024 frame 0: A whole grade: white balance, exposure, the tone sliders, saturation, the look at 80, faded film, vibrance, teal and orange tints and a dark vignette, each step in its order. | largest difference 2.7e-7 | yes |
| FX-GRADE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-025 frame 0: Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 untouched, frame 2 one stop up, frame 4 two. | largest difference 1.9e-7 | yes |
| FX-GRADE-025 frame 2: Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 untouched, frame 2 one stop up, frame 4 two. | largest difference 2.0e-7 | yes |
| FX-GRADE-025 frame 4: Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 untouched, frame 2 one stop up, frame 4 two. | largest difference 2.9e-7 | yes |
| FX-GRADE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-026 frame 0: Contrast 50 and vignette -4 moved three pixels right: the same, moved; the vignette goes with the layer. | largest difference 1.4e-7 | yes |
| FX-GRADE-026 frame 3: Contrast 50 and vignette -4 moved three pixels right: the same, moved; the vignette goes with the layer. | largest difference 1.4e-7 | yes |
| FX-GRADE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE-027 frame 0: Contrast 50 with the look naming the drawing, which is not a lookup file: drawn with the contrast and without the look, with a warning on opening and on each frame. | largest difference 1.5e-7 | yes |
| FX-GRADE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-028 frame 0: Temperature 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-028 frame 4: Temperature 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-029 frame 0: Exposure -5.5, below -5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-029 frame 4: Exposure -5.5, below -5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-030 frame 0: Saturation 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-030 frame 4: Saturation 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-031 frame 0: Look intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-031 frame 4: Look intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-032 frame 0: Shadow tint hue 361, past a full turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-032 frame 4: Shadow tint hue 361, past a full turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-033 frame 0: Vignette amount 5.5, above 5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-033 frame 4: Vignette amount 5.5, above 5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADE-034 frame 0: Vignette roundness -20: Lumetri's squarer vignette, below 0, which this effect does not draw (a gap). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-034 frame 4: Vignette roundness -20: Lumetri's squarer vignette, below 0, which this effect does not draw (a gap). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade_024.json is saved with its 23 settings, the look and 22 numbers | {"blacks":-10,"contrast":30,"creative_saturation":95,"exposure":0.3,"faded_film":20,"highlight_tint_amount":50,"highlight_tint_hue":35,"highlights":-20,"look":"asset-look","look_intensity":80,"saturation":110,"shadow_tint_amount":60,"shadow_tint_hue":195,"shadows":20,"temperature":20,"tint":-10,"tint_balance":-20,"vibrance":30,"vignette_amount":-1.5,"vignette_feather":50,"vignette_midpoint":50,"vignette_roundness":0,"whites":10} | yes |
| fx_grade_028.json is refused in a sentence naming temperature | Color Grade's temperature runs from -100 to 100, and this is 101. | yes |
| fx_grade_029.json is refused in a sentence naming exposure | Color Grade's exposure runs from -5 to 5, and this is -5.5. | yes |
| fx_grade_030.json is refused in a sentence naming saturation | Color Grade's saturation runs from 0 to 200, and this is 201. | yes |
| fx_grade_031.json is refused in a sentence naming look intensity | Color Grade's look intensity runs from 0 to 200, and this is -1. | yes |
| fx_grade_032.json is refused in a sentence naming shadow tint hue | Color Grade's shadow tint hue runs from 0 to 360, and this is 361. | yes |
| fx_grade_033.json is refused in a sentence naming vignette amount | Color Grade's vignette amount runs from -5 to 5, and this is 5.5. | yes |
| fx_grade_034.json is refused in a sentence naming vignette roundness | Color Grade's vignette roundness runs from 0 to 100, and this is -20. | yes |
| a file with a Color Grade whose exposure is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Grade without its look is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| temperature 101 is refused with a sentence, and nothing changes | Color Grade's temperature runs from -100 to 100, and this is 101. | yes |
| look "asset-colours", a picture and not a lookup file is refused with a sentence, and nothing changes | Color Grade cannot use asset-colours: it is not a lookup file of this project. | yes |
| exposure keyed to 6 is refused with a sentence, and nothing changes | Color Grade's exposure runs from -5 to 5, and this is 6. | yes |
| contrast 40, faded film 30, the look file is taken | taken | yes |
| exposure keyed from -2 to 2 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade_026.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120 on three layers, frame 0, Full | largest difference 1 of 255, 665 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120 on three layers, frame 100, Full | largest difference 1 of 255, 687 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120 on three layers, frame 239, Full | largest difference 1 of 255, 632 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120 on three layers, frame 0, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120 on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120 on three layers, frame 239, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072603 pixels changed | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights on three layers, frame 0, Full | largest difference 1 of 255, 3356 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights on three layers, frame 100, Full | largest difference 1 of 255, 670 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights on three layers, frame 239, Full | largest difference 1 of 255, 3806 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights on three layers, frame 0, Draft | largest difference 1 of 255, 168 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade creative: faded film 40, vibrance 50, teal shadows and orange highlights on three layers, frame 239, Draft | largest difference 1 of 255, 189 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2068133 pixels changed | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3 on three layers, frame 0, Full | largest difference 1 of 255, 1524 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3 on three layers, frame 100, Full | largest difference 1 of 255, 1547 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3 on three layers, frame 239, Full | largest difference 1 of 255, 1402 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3 on three layers, frame 0, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3 on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Grade highlights -40, whites 30 and a vignette of -3 on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-397 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| Color Grade as added changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_warm.png, temperature 60: the street warmer; draws cleanly | [], red less blue -34.0 before, -6.5 after | yes |
| 3_teal_orange.png, contrast 30, teal shadows and orange highlights: the street changed; draws cleanly | [], 129600 pixels changed | yes |
| 4_look.png, the fixtures' own look file at intensity 100: the street changed; draws cleanly | [], 129600 pixels changed | yes |
| 5_faded_film.png, faded film 70: the darkest level lifted; draws cleanly | [], darkest level 40 before, 71 after | yes |
| 6_vignette.png, vignette -3: the corners darker, the middle as it was; draws cleanly | [], corner 520 before, 342 after; middle 400 before, 400 after | yes |

## Result

196 of 196 checks pass.
