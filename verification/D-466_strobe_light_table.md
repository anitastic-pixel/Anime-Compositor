# D-466: Strobe Light

B-346: `core.strobe_light` takes After Effects' Strobe Light controls (Strobe Color, Blend With Original, Strobe Duration, Strobe Period, Random Strobe Probability, Strobe, Strobe Operator, Random Seed). Every expected pixel is `Fixtures/strobe_light/expected_strobe_light.json`, written by `tools/strobe_light_reference.py` before this code existed and printed in document 25 as FX-STROBE-001 to 030. Tolerance 2e-5.

## FX-STROBE-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STROBE-001 frame 0: The settings as they start: white, Copy, a strobe 0.1 seconds long every second. Frames 0, 1 and 2 (0 to 0.083 seconds) are white where the drawing is; frame 3 (0.125) is the drawing; frame 24 (1 second) is white again. | largest difference 3.0e-8 | yes |
| FX-STROBE-001 frame 2: The settings as they start: white, Copy, a strobe 0.1 seconds long every second. Frames 0, 1 and 2 (0 to 0.083 seconds) are white where the drawing is; frame 3 (0.125) is the drawing; frame 24 (1 second) is white again. | largest difference 3.0e-8 | yes |
| FX-STROBE-001 frame 24: The settings as they start: white, Copy, a strobe 0.1 seconds long every second. Frames 0, 1 and 2 (0 to 0.083 seconds) are white where the drawing is; frame 3 (0.125) is the drawing; frame 24 (1 second) is white again. | largest difference 3.0e-8 | yes |
| FX-STROBE-001 frame 3: The settings as they start: white, Copy, a strobe 0.1 seconds long every second. Frames 0, 1 and 2 (0 to 0.083 seconds) are white where the drawing is; frame 3 (0.125) is the drawing; frame 24 (1 second) is white again. | largest difference 1.9e-7 | yes |
| FX-STROBE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-002 frame 0: Strobe Operator Add with #8040c0, frame 0. | largest difference 6.7e-8 | yes |
| FX-STROBE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-003 frame 0: Strobe Operator Subtract with #8040c0, frame 0. | largest difference 2.0e-7 | yes |
| FX-STROBE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-004 frame 0: Strobe Operator Multiply with #8040c0, frame 0. | largest difference 7.4e-8 | yes |
| FX-STROBE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-005 frame 0: Strobe Operator Difference with #8040c0, frame 0. | largest difference 2.0e-7 | yes |
| FX-STROBE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-006 frame 0: Strobe Operator And with #8040c0, frame 0. | largest difference 3.0e-8 | yes |
| FX-STROBE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-007 frame 0: Strobe Operator Or with #8040c0, frame 0. | largest difference 3.0e-8 | yes |
| FX-STROBE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-008 frame 0: Strobe Operator Xor with #8040c0, frame 0. | largest difference 3.0e-8 | yes |
| FX-STROBE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-009 frame 0: Strobe Operator Minimum with #8040c0, frame 0. | largest difference 1.3e-7 | yes |
| FX-STROBE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-010 frame 0: Strobe Operator Maximum with #8040c0, frame 0. | largest difference 1.9e-7 | yes |
| FX-STROBE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-011 frame 0: Strobe Operator Darken with #8040c0, frame 0. | largest difference 3.0e-8 | yes |
| FX-STROBE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-012 frame 0: Strobe Operator Lighten with #8040c0, frame 0. | largest difference 1.9e-7 | yes |
| FX-STROBE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-013 frame 0: Copy in red with Blend With Original 50: halfway between the drawing and red. | largest difference 8.2e-8 | yes |
| FX-STROBE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-014 frame 0: Makes Layer Transparent: frame 0 is empty, frame 3 the drawing. | largest difference 0.0e0 | yes |
| FX-STROBE-014 frame 3: Makes Layer Transparent: frame 0 is empty, frame 3 the drawing. | largest difference 1.9e-7 | yes |
| FX-STROBE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-015 frame 0: Makes Layer Transparent with Blend With Original 25: the drawing at a quarter strength. | largest difference 4.7e-8 | yes |
| FX-STROBE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-016 frame 0: Strobe Period 0.05, below the 0.1 duration: on at every frame. | largest difference 3.0e-8 | yes |
| FX-STROBE-016 frame 13: Strobe Period 0.05, below the 0.1 duration: on at every frame. | largest difference 3.0e-8 | yes |
| FX-STROBE-016 frame 3: Strobe Period 0.05, below the 0.1 duration: on at every frame. | largest difference 3.0e-8 | yes |
| FX-STROBE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-017 frame 0: Strobe Duration 0: never on. | largest difference 1.9e-7 | yes |
| FX-STROBE-017 frame 1: Strobe Duration 0: never on. | largest difference 1.9e-7 | yes |
| FX-STROBE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-018 frame 0: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 1.9e-7 | yes |
| FX-STROBE-018 frame 1: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 3.0e-8 | yes |
| FX-STROBE-018 frame 2: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 1.9e-7 | yes |
| FX-STROBE-018 frame 3: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 1.9e-7 | yes |
| FX-STROBE-018 frame 4: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 3.0e-8 | yes |
| FX-STROBE-018 frame 5: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 1.9e-7 | yes |
| FX-STROBE-018 frame 6: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 3.0e-8 | yes |
| FX-STROBE-018 frame 7: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 3.0e-8 | yes |
| FX-STROBE-018 frame 8: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 1.9e-7 | yes |
| FX-STROBE-018 frame 9: Random Strobe Probability 50, seed 3, no regular strobe (duration 0, period 100): some frames strobe and some do not. | largest difference 1.9e-7 | yes |
| FX-STROBE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-019 frame 0: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 3.0e-8 | yes |
| FX-STROBE-019 frame 1: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 3.0e-8 | yes |
| FX-STROBE-019 frame 2: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 3.0e-8 | yes |
| FX-STROBE-019 frame 3: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 1.9e-7 | yes |
| FX-STROBE-019 frame 4: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 1.9e-7 | yes |
| FX-STROBE-019 frame 5: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 3.0e-8 | yes |
| FX-STROBE-019 frame 6: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 1.9e-7 | yes |
| FX-STROBE-019 frame 7: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 1.9e-7 | yes |
| FX-STROBE-019 frame 8: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 1.9e-7 | yes |
| FX-STROBE-019 frame 9: Random Strobe Probability 50, seed 4: a different choice of frames from FX-STROBE-018. | largest difference 3.0e-8 | yes |
| FX-STROBE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-020 frame 0: Random Strobe Probability 100: every frame strobes. | largest difference 3.0e-8 | yes |
| FX-STROBE-020 frame 3: Random Strobe Probability 100: every frame strobes. | largest difference 3.0e-8 | yes |
| FX-STROBE-020 frame 7: Random Strobe Probability 100: every frame strobes. | largest difference 3.0e-8 | yes |
| FX-STROBE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-021 frame 0: Blend With Original keyed from 0 at frame 0 to 100 at frame 4, linear, Strobe Period 0.01 so every frame strobes: frame 2 is half strength, frame 4 the drawing. | largest difference 3.0e-8 | yes |
| FX-STROBE-021 frame 2: Blend With Original keyed from 0 at frame 0 to 100 at frame 4, linear, Strobe Period 0.01 so every frame strobes: frame 2 is half strength, frame 4 the drawing. | largest difference 9.3e-8 | yes |
| FX-STROBE-021 frame 4: Blend With Original keyed from 0 at frame 0 to 100 at frame 4, linear, Strobe Period 0.01 so every frame strobes: frame 2 is half strength, frame 4 the drawing. | largest difference 1.9e-7 | yes |
| FX-STROBE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-022 frame 0: FX-STROBE-002 moved three pixels right. | largest difference 6.7e-8 | yes |
| FX-STROBE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-023 frame 0: After a drop shadow that grows the layer to 10 by 8, Copy in purple: the shadow is purple too, at its own alpha. | largest difference 3.0e-8 | yes |
| FX-STROBE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-024 frame 2: The layer starting at frame 2: its time counts from there, so frame 2 and 4 strobe and frame 5 does not. | largest difference 3.0e-8 | yes |
| FX-STROBE-024 frame 4: The layer starting at frame 2: its time counts from there, so frame 2 and 4 strobe and frame 5 does not. | largest difference 3.0e-8 | yes |
| FX-STROBE-024 frame 5: The layer starting at frame 2: its time counts from there, so frame 2 and 4 strobe and frame 5 does not. | largest difference 1.9e-7 | yes |
| FX-STROBE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROBE-025 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-025 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROBE-026 frame 0: Strobe Duration -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-026 frame 4: Strobe Duration -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROBE-027 frame 0: Strobe Operator "screen", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-027 frame 4: Strobe Operator "screen", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROBE-028 frame 0: Strobe "flash", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-028 frame 4: Strobe "flash", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROBE-029 frame 0: Strobe Color "white", which is not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-029 frame 4: Strobe Color "white", which is not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROBE-030 frame 0: Random Strobe Probability keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-030 frame 4: Random Strobe Probability keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STROBE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_strobe_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_strobe_013.json is saved with its eight settings, and not the time | {"blend_with_original":50,"random_seed":0,"random_strobe_probability":0,"strobe":"color_only","strobe_color":"#ff0000","strobe_duration":0.1,"strobe_operator":"copy","strobe_period":1} | yes |
| fx_strobe_025.json is refused in a sentence naming Strobe Light and its blend with original | Strobe Light's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_strobe_026.json is refused in a sentence naming Strobe Light and its strobe duration | Strobe Light's strobe duration runs from 0 to 100, and this is -1. | yes |
| fx_strobe_027.json is refused in a sentence naming Strobe Light and its strobe operator | Strobe Light's strobe operator is one of copy, add, subtract, multiply, difference, and, or, xor, minimum, maximum, darken, lighten, and this is "screen". | yes |
| fx_strobe_028.json is refused in a sentence naming Strobe Light and its strobe is | Strobe Light's strobe is "color_only" or "transparent", and this is "flash". | yes |
| fx_strobe_029.json is refused in a sentence naming Strobe Light and its strobe colour | Strobe Light's strobe colour is written #rrggbb, and this is "white". | yes |
| a file with a Strobe Light whose duration is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Strobe Light without its random seed is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Strobe Light whose colour is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blend with original 101 is refused with a sentence, and nothing changes | Strobe Light's blend with original runs from 0 to 100, and this is 101. | yes |
| strobe operator screen is refused with a sentence, and nothing changes | Strobe Light's strobe operator is one of copy, add, subtract, multiply, difference, and, or, xor, minimum, maximum, darken, lighten, and this is "screen". | yes |
| strobe flash is refused with a sentence, and nothing changes | Strobe Light's strobe is "color_only" or "transparent", and this is "flash". | yes |
| strobe colour white is refused with a sentence, and nothing changes | Strobe Light's strobe colour is written #rrggbb, and this is "white". | yes |
| random strobe probability keyed to 150 is refused with a sentence, and nothing changes | Strobe Light's random strobe probability runs from 0 to 100, and this is 150. | yes |
| green, blend 12.5, every half second, Makes Layer Transparent, Xor, seed 42 is taken | taken | yes |
| blend with original keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_strobe_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_strobe_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_strobe_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_strobe_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_strobe_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_strobe_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_strobe_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Strobe Light Copy in purple, always on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Strobe Light Copy in purple, always on on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Copy in purple, always on on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Copy in purple, always on on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Copy in purple, always on on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Copy in purple, always on on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Copy in purple, always on on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on on three layers, frame 0, Full | largest difference 1 of 255, 2630 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on on three layers, frame 100, Full | largest difference 1 of 255, 116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on on three layers, frame 239, Full | largest difference 1 of 255, 662 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on on three layers, frame 0, Draft | largest difference 1 of 255, 123 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on on three layers, frame 100, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Difference in orange, Blend With Original 30, always on on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Xor in teal, always on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Strobe Light Xor in teal, always on on three layers, frame 0, Full | largest difference 1 of 255, 773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Xor in teal, always on on three layers, frame 100, Full | largest difference 1 of 255, 257 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Xor in teal, always on on three layers, frame 239, Full | largest difference 1 of 255, 705 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Xor in teal, always on on three layers, frame 0, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Xor in teal, always on on three layers, frame 100, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Xor in teal, always on on three layers, frame 239, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on on three layers, frame 0, Full | largest difference 1 of 255, 1673 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on on three layers, frame 100, Full | largest difference 1 of 255, 1257 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on on three layers, frame 239, Full | largest difference 1 of 255, 2847 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on on three layers, frame 0, Draft | largest difference 1 of 255, 95 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on on three layers, frame 100, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Strobe Light Makes Layer Transparent, Blend With Original 40, always on on three layers, frame 239, Draft | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-466 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street filling the frame; draws cleanly | [], covered 129600 of 129600 | yes |
| 2_as_added_frame_0.png, Strobe Light as added at frame 0, inside the first strobe: the whole street white; draws cleanly | [], all white true | yes |
| 3_as_added_frame_3.png, as added at frame 3 (0.125 seconds, past the 0.1-second strobe): the street as it was; draws cleanly | [], the same as 1_before.png true | yes |
| 4_difference_purple.png, Difference with purple: the street's colours turned over against purple, nearly every pixel changed, still opaque; draws cleanly | [], unchanged 0 of 129600, opaque true | yes |
| 5_red_blend_50.png, Copy in red with Blend With Original 50: halfway to red, so every pixel at least as red and no more green or blue than the street; draws cleanly | [], toward red everywhere true, not the street true | yes |
| 6_transparent_blend_25.png, Makes Layer Transparent with Blend With Original 25: the street at a quarter strength (alpha 64 of 255), its colours kept; draws cleanly | [], a quarter everywhere true | yes |
| Random Strobe Probability 50 on the street, frames 0 to 4: seed 3 strobes at 1 and 4, seed 4 at 0, 1 and 2, as FX-STROBE-018 and 019 say | seed 3 [1, 4], seed 4 [0, 1, 2] | yes |

## Result

214 of 214 checks pass.
