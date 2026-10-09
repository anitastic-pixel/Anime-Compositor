# B-233: the shared soft-glow engine and Soft Physical Glow

D-353 (EFFECTS.md P0-21 and pick #1): Glow's new "physical" falloff. A soft threshold picks the light (Threshold Mode, Smooth, Saturation Bias), several blur sizes a doubling apart are added in linear light so the glow has a bright core and a long, soft tail, Exposure brightens it, and the untouched layer goes back on top. A Glow without the falloff, or with "classic", is the Glow it always was. Every expected pixel is `Fixtures/soft_glow/expected_soft_glow.json`, written by `tools/soft_glow_reference.py` before this code existed and printed in document 25 as FX-SGLOW-001 to 045. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. The glow is drawn on the card too: the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).

## FX-SGLOW-001 to 045 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SGLOW-001 frame 0: Radius 12, everything else at the effect's defaults (threshold 0, so everything glows; Screen; exposure 1): a soft glow with a bright core and a long tail spreads round every patch into the empty space. | largest difference 2.4e-7 | yes |
| FX-SGLOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-002 frame 0: The effect's own defaults, radius 500: the glow is spread so wide that on a drawing this small it is a faint wash; the drawing shows through brighter where it doubles on itself. | largest difference 2.1e-7 | yes |
| FX-SGLOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-003 frame 0: Radius 0: nothing spreads; the light is laid on itself, so each pixel is doubled and held at white. | largest difference 1.1e-7 | yes |
| FX-SGLOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-004 frame 0: Radius 1.5: the first level is too small to count yet, so this is still FX-SGLOW-003. | largest difference 1.1e-7 | yes |
| FX-SGLOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-005 frame 0: Radius 2.25: the first level is half in, so the glow is half spread and half laid on itself. | largest difference 1.2e-7 | yes |
| FX-SGLOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-006 frame 0: Radius 100: seven levels, the larger three worked on cells of 8, 4 and 2 pixels. | largest difference 2.1e-7 | yes |
| FX-SGLOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-007 frame 0: Threshold 30, Chroma: each channel tested on its own: the yellow's red and green glow and its blue does not, the brown's red glows, the purple does not glow. | largest difference 2.2e-7 | yes |
| FX-SGLOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-008 frame 0: Threshold 30, Luminance: one brightness per pixel: only the yellow glows, all three of its channels. | largest difference 2.2e-7 | yes |
| FX-SGLOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-009 frame 0: Threshold 30, Smooth 50: a channel between 15 % and 30 % glows in proportion: the yellow's blue, at 19 %, glows at a quarter. | largest difference 2.2e-7 | yes |
| FX-SGLOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-010 frame 0: Saturation Bias 100, threshold 50, the grey drawing: the test is on saturation alone: the yellow and the purple glow, the grey does not. | largest difference 2.2e-7 | yes |
| FX-SGLOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-011 frame 0: Saturation Bias -100, threshold 50: the other way round: only the grey glows. | largest difference 1.8e-7 | yes |
| FX-SGLOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-012 frame 0: Saturation Bias 50, threshold 40: halfway: the colourful patches glow in every channel, the grey, bright as it is, does not. | largest difference 2.2e-7 | yes |
| FX-SGLOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-013 frame 0: Aspect Ratio 1.6: the glow is stretched sideways and squeezed top to bottom. | largest difference 2.5e-7 | yes |
| FX-SGLOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-014 frame 0: Aspect Ratio 0.5: taller than wide. | largest difference 2.6e-7 | yes |
| FX-SGLOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-015 frame 0: Aspect Ratio 1.5 at Angle 30: the oval leans, its long side 30 degrees below the horizontal, going right. | largest difference 2.3e-7 | yes |
| FX-SGLOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-016 frame 0: Aspect Ratio 1.5 at Angle 60: steeper, worked the other way round (slanted line along x). | largest difference 2.4e-7 | yes |
| FX-SGLOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-017 frame 0: Aspect Ratio 2: a flat streak, no spread top to bottom except the softening that working on cells gives. | largest difference 2.6e-7 | yes |
| FX-SGLOW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-018 frame 0: Blend Mode Add, exposure 3: the light adds up and is not cut off at white. | largest difference 5.3e-7 | yes |
| FX-SGLOW-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-019 frame 0: Blend Mode Screen, exposure 3: the same, held at white. | largest difference 1.9e-7 | yes |
| FX-SGLOW-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-020 frame 0: Source Opacity 0: the glow alone, the drawing not laid back. | largest difference 9.5e-8 | yes |
| FX-SGLOW-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-021 frame 0: Source Opacity 50: the drawing laid back at half strength. | largest difference 1.8e-7 | yes |
| FX-SGLOW-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-022 frame 0: Unmult off: the glow sits on solid black, every pixel fully covering. | largest difference 2.4e-7 | yes |
| FX-SGLOW-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-023 frame 0: Unmult off, Source Opacity 0: the glow alone on black. | largest difference 9.5e-8 | yes |
| FX-SGLOW-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-024 frame 0: The patches half covering, threshold 50: the colour is multiplied by its covering first, so the yellow counts at half and nothing glows: the drawing, untouched. | largest difference 9.5e-8 | yes |
| FX-SGLOW-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-025 frame 0: The patches half covering, threshold 0: they glow at half the strength of FX-SGLOW-001. | largest difference 1.7e-7 | yes |
| FX-SGLOW-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-026 frame 0: FX-SGLOW-001 moved three pixels right: the glow that spread past the drawing's left edge shows in columns 0 to 2; the layer grew to hold it. | largest difference 2.4e-7 | yes |
| FX-SGLOW-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-027 frame 0: FX-SGLOW-001 moved six pixels left: the yellow patch is off the composition, and its glow still lights the frame's left side. | largest difference 9.6e-8 | yes |
| FX-SGLOW-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-028 frame 0: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SGLOW-003, frame 4 is FX-SGLOW-006. | largest difference 1.1e-7 | yes |
| FX-SGLOW-028 frame 2: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SGLOW-003, frame 4 is FX-SGLOW-006. | largest difference 2.1e-7 | yes |
| FX-SGLOW-028 frame 4: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SGLOW-003, frame 4 is FX-SGLOW-006. | largest difference 2.1e-7 | yes |
| FX-SGLOW-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-029 frame 0: Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-SGLOW-001. | largest difference 1.5e-7 | yes |
| FX-SGLOW-029 frame 2: Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-SGLOW-001. | largest difference 2.4e-7 | yes |
| FX-SGLOW-029 frame 4: Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-SGLOW-001. | largest difference 1.3e-7 | yes |
| FX-SGLOW-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-030 frame 0: Threshold 100: nothing in the drawing is that bright: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-SGLOW-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-031 frame 0: Radius 2,000, the largest: eleven levels; on a drawing this small the glow is a very faint wash. | largest difference 1.9e-7 | yes |
| FX-SGLOW-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-032 frame 0: Threshold 30, Smooth 50, Saturation Bias 50, Luminance, Add, exposure 1.5, Aspect Ratio 0.7 at Angle -20, Source Opacity 80, radius 30: everything at once. | largest difference 2.2e-7 | yes |
| FX-SGLOW-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SGLOW-033 frame 0: Falloff "gaussian", which is not classic or physical. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-033 frame 4: Falloff "gaussian", which is not classic or physical. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-034 frame 0: Radius 2001, above 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-034 frame 4: Radius 2001, above 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-035 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-035 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-036 frame 0: Radius keyed to 2500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-036 frame 4: Radius keyed to 2500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-037 frame 0: Saturation Bias 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-037 frame 4: Saturation Bias 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-038 frame 0: Threshold Smooth 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-038 frame 4: Threshold Smooth 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-039 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-039 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-040 frame 0: Aspect Ratio 2.1, above 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-040 frame 4: Aspect Ratio 2.1, above 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-041 frame 0: Exposure -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-041 frame 4: Exposure -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-042 frame 0: Source Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-042 frame 4: Source Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-043 frame 0: Threshold Mode "rgb", which is not chroma or luminance. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-043 frame 4: Threshold Mode "rgb", which is not chroma or luminance. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-044 frame 0: Blend Mode "multiply", which is not add or screen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-044 frame 4: Blend Mode "multiply", which is not add or screen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SGLOW-045 frame 0: Unmult "yes", which is not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-045 frame 4: Unmult "yes", which is not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SGLOW-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sglow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sglow_001.json: a Soft Physical Glow writes its falloff, "physical", and none of the classic Glow's settings | {"aspect_angle":0,"aspect_ratio":1,"exposure":1,"falloff":"physical","operation":"screen","radius":12,"saturation_bias":0,"source_opacity":100,"threshold":0,"threshold_mode":"chroma","threshold_smooth":0,"unmult":"on"} | yes |
| a file with Soft Physical Glow with no radius is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an exposure written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with unmult written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| fx_sglow_033.json is refused in a sentence naming it | Glow's falloff is "classic" or "physical", and this is "gaussian". | yes |
| fx_sglow_034.json is refused in a sentence naming it | Soft Physical Glow's radius runs from 0 to 2000, and this is 2001. | yes |
| fx_sglow_035.json is refused in a sentence naming it | Soft Physical Glow's radius runs from 0 to 2000, and this is -1. | yes |
| fx_sglow_037.json is refused in a sentence naming it | Soft Physical Glow's saturation bias runs from -100 to 100, and this is 101. | yes |
| fx_sglow_038.json is refused in a sentence naming it | Soft Physical Glow's threshold smooth runs from 0 to 100, and this is 101. | yes |
| fx_sglow_039.json is refused in a sentence naming it | Soft Physical Glow's threshold runs from 0 to 100, and this is -1. | yes |
| fx_sglow_040.json is refused in a sentence naming it | Soft Physical Glow's aspect ratio runs from 0 to 2, and this is 2.1. | yes |
| fx_sglow_041.json is refused in a sentence naming it | Soft Physical Glow's exposure runs from 0 to 100, and this is -1. | yes |
| fx_sglow_042.json is refused in a sentence naming it | Soft Physical Glow's source opacity runs from 0 to 100, and this is 101. | yes |
| fx_sglow_043.json is refused in a sentence naming it | Soft Physical Glow's threshold mode is "chroma" or "luminance", and this is "rgb". | yes |
| fx_sglow_044.json is refused in a sentence naming it | Soft Physical Glow's blend mode is "add" or "screen", and this is "multiply". | yes |
| fx_sglow_045.json is refused in a sentence naming it | Soft Physical Glow's unmult is "off" or "on", and this is "yes". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 2001 is refused with a sentence, and nothing changes | Soft Physical Glow's radius runs from 0 to 2000, and this is 2001. | yes |
| threshold mode "rgb" is refused with a sentence, and nothing changes | Soft Physical Glow's threshold mode is "chroma" or "luminance", and this is "rgb". | yes |
| threshold 30 in luminance, exposure 2 is taken | taken | yes |
| radius keyed 0 to 100 over frames 0 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sglow_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sglow_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sglow_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sglow_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sglow_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sglow_032.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sglow_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_sglow_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_sglow_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| the reference shot, Soft Physical Glow as added (radius 500): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Soft Physical Glow as added (radius 500) on three layers, frame 0, Full | largest difference 1 of 255, 438 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow as added (radius 500) on three layers, frame 100, Full | largest difference 1 of 255, 534 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow as added (radius 500) on three layers, frame 239, Full | largest difference 1 of 255, 341 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow as added (radius 500) on three layers, frame 0, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow as added (radius 500) on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow as added (radius 500) on three layers, frame 239, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2032399 pixels changed | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5 on three layers, frame 0, Full | largest difference 1 of 255, 597 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5 on three layers, frame 100, Full | largest difference 1 of 255, 224 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5 on three layers, frame 239, Full | largest difference 1 of 255, 297 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5 on three layers, frame 0, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5 on three layers, frame 100, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5 on three layers, frame 239, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073584 pixels changed | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off on three layers, frame 100, Full | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off on three layers, frame 0, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off on three layers, frame 100, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off on three layers, frame 239, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| a threshold with smooth 0 is a hard step, so it is left to the processor (D-122's reason) | 0 of 3 on the card | yes |

## Pictures: the town, set in a larger frame, in `verification/D-353 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the town with no glow: nothing outside the town | covering 40 pixels out 0, warnings [] | yes |
| classic_glow.png, the classic Glow, Bright parts above 60 %, radius 60, for comparison: one blur, a short even halo | covering 40 pixels out 1, in the far corner 0; warnings [] | yes |
| soft_glow_threshold_60.png, Soft Physical Glow, threshold 60 with smooth 50, radius 60: a bright core on the light parts and a long soft tail, reaching past the town | covering 40 pixels out 1, in the far corner 0; warnings [] | yes |
| soft_glow_as_added.png, Soft Physical Glow as added (radius 500, threshold 0): the whole town glows, a wide haze filling the frame to its corners | covering 40 pixels out 12, in the far corner 2; warnings [] | yes |
| soft_glow_stretched.png, Soft Physical Glow, threshold 60 with smooth 50, radius 120, aspect 1.8: the glow stretched sideways, an anamorphic streak | covering 40 pixels out 14, in the far corner 0; warnings [] | yes |

## Result

238 of 238 checks pass.
