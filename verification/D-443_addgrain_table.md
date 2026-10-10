# D-443: Add Grain

B-323, after After Effects' Add Grain: a value noise of Size pixels (Aspect Ratio wider), blocky at Softness 0 and smooth at 1, one number for all three channels when Monochromatic, else three pulled towards their mean by Saturation, scaled by each channel's intensity and weighted by the pixel's brightness through Shadows, Midtones and Highlights about Midpoint, laid on by Film, Add or Overlay at a tenth of Intensity a channel, through the sRGB curve. The grain belongs to the drawing, moves with it, and changes Animation Speed times a frame, gliding when Animate Smoothly is on. Every expected pixel is `Fixtures/addgrain/expected_addgrain.json`, written by `tools/addgrain_reference.py` before this code existed and printed in document 25 as FX-ADDGRAIN-001 to 041. Tolerance 2e-5.

## FX-ADDGRAIN-001 to 041 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADDGRAIN-001 frame 0: The settings as they start: intensity 1, size 1, Film, in colour, a new grain every frame; every shown pixel moves by up to 0.1 a channel through the sRGB curve, most in the middle of each channel, nothing in the white and black patches; the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-001 frame 2: The settings as they start: intensity 1, size 1, Film, in colour, a new grain every frame; every shown pixel moves by up to 0.1 a channel through the sRGB curve, most in the middle of each channel, nothing in the white and black patches; the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-001 frame 4: The settings as they start: intensity 1, size 1, Film, in colour, a new grain every frame; every shown pixel moves by up to 0.1 a channel through the sRGB curve, most in the middle of each channel, nothing in the white and black patches; the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 2.7e-7 | yes |
| FX-ADDGRAIN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-002 frame 0: Animation Speed 0: the same grain on frames 0, 2 and 4, FX-ADDGRAIN-001's frame 0. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-002 frame 2: Animation Speed 0: the same grain on frames 0, 2 and 4, FX-ADDGRAIN-001's frame 0. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-002 frame 4: Animation Speed 0: the same grain on frames 0, 2 and 4, FX-ADDGRAIN-001's frame 0. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-003 frame 1: Animation Speed 0.5, smooth: frame 1 is half way between FX-ADDGRAIN-001's frames 0 and 1 in the noise, and frame 2 is its frame 1. | largest difference 2.5e-7 | yes |
| FX-ADDGRAIN-003 frame 2: Animation Speed 0.5, smooth: frame 1 is half way between FX-ADDGRAIN-001's frames 0 and 1 in the noise, and frame 2 is its frame 1. | largest difference 2.5e-7 | yes |
| FX-ADDGRAIN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-004 frame 0: Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain and frame 3 is FX-ADDGRAIN-001's frame 1. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-004 frame 1: Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain and frame 3 is FX-ADDGRAIN-001's frame 1. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-004 frame 3: Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain and frame 3 is FX-ADDGRAIN-001's frame 1. | largest difference 2.5e-7 | yes |
| FX-ADDGRAIN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-005 frame 0: Monochromatic, Add: the three channels move together, a grey grain. | largest difference 2.2e-7 | yes |
| FX-ADDGRAIN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-006 frame 0: Saturation 0, Add: each channel takes the three's mean, so they move together, but less than FX-ADDGRAIN-005's. | largest difference 2.2e-7 | yes |
| FX-ADDGRAIN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-007 frame 0: Saturation 0.5, Add: half way. | largest difference 2.3e-7 | yes |
| FX-ADDGRAIN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-008 frame 0: Size 4, Add: grains of 4 by 4 pixels, each one number. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-009 frame 0: Size 4, Softness 1, Add: the grains blend smoothly into each other. | largest difference 2.1e-7 | yes |
| FX-ADDGRAIN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-010 frame 0: Size 4, Softness 0.5, Add: half way between FX-ADDGRAIN-008 and 009 in the noise. | largest difference 2.0e-7 | yes |
| FX-ADDGRAIN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-011 frame 0: Size 2, Aspect Ratio 2, Add: grains 4 across and 2 down. | largest difference 2.1e-7 | yes |
| FX-ADDGRAIN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-012 frame 0: Channel intensities 2, 0 and 0.5, Add: red's grain doubled, green untouched, blue's halved. | largest difference 2.1e-7 | yes |
| FX-ADDGRAIN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-013 frame 0: Shadows 0, Highlights 0, Add: no grain in the black or the white patch, and most in the middle tones. | largest difference 2.2e-7 | yes |
| FX-ADDGRAIN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-014 frame 0: Shadows 3, Midtones 0, Highlights 0.5, Midpoint 0.3, Add: strong grain in the dark line, none at brightness 0.3. | largest difference 2.3e-7 | yes |
| FX-ADDGRAIN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-015 frame 0: Add, intensity 3: the white patch can only darken and the black only lighten; elsewhere up to 0.3 either way. | largest difference 2.0e-7 | yes |
| FX-ADDGRAIN-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-016 frame 0: Overlay, intensity 2: the grain laid on in Overlay. | largest difference 2.4e-7 | yes |
| FX-ADDGRAIN-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-017 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-017 frame 2: Intensity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-018 frame 0: Intensity 10, Add: the grain at its strongest. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-019 frame 0: Random Seed 7: a different grain from FX-ADDGRAIN-001's. | largest difference 2.5e-7 | yes |
| FX-ADDGRAIN-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-020 frame 0: Random Seed 3. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-021 frame 0: Random Seed 3.7: its whole part counts, so this is FX-ADDGRAIN-020. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-022 frame 0: Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 is the drawing; frame 4's grain is frame 2's, twice as strong. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-022 frame 2: Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 is the drawing; frame 4's grain is frame 2's, twice as strong. | largest difference 2.1e-7 | yes |
| FX-ADDGRAIN-022 frame 4: Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 is the drawing; frame 4's grain is frame 2's, twice as strong. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-023 frame 0: Size keyed from 1 at frame 0 to 5 at frame 4, speed 0: frame 2 is size 3. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-023 frame 2: Size keyed from 1 at frame 0 to 5 at frame 4, speed 0: frame 2 is size 3. | largest difference 2.7e-7 | yes |
| FX-ADDGRAIN-023 frame 4: Size keyed from 1 at frame 0 to 5 at frame 4, speed 0: frame 2 is size 3. | largest difference 2.7e-7 | yes |
| FX-ADDGRAIN-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-024 frame 0: FX-ADDGRAIN-001 moved three pixels right: the grain is the drawing's own, so it moves with it. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-024 frame 2: FX-ADDGRAIN-001 moved three pixels right: the grain is the drawing's own, so it moves with it. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-025 frame 0: After a Motion Tile that grows the layer: the grain is worked in the drawing's own pixels, so the frame is FX-ADDGRAIN-001's. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-025 frame 2: After a Motion Tile that grows the layer: the grain is worked in the drawing's own pixels, so the frame is FX-ADDGRAIN-001's. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-026 frame 1: Animation Speed 2: frame 1 is FX-ADDGRAIN-001's frame 2. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-026 frame 2: Animation Speed 2: frame 1 is FX-ADDGRAIN-001's frame 2. | largest difference 2.7e-7 | yes |
| FX-ADDGRAIN-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-027 frame 0: Midpoint keyed from 0.2 at frame 0 to 0.8 at frame 4 with Shadows 0 and Highlights 2, speed 0, Add: the dark tones gain grain as the midpoint rises. | largest difference 2.2e-7 | yes |
| FX-ADDGRAIN-027 frame 4: Midpoint keyed from 0.2 at frame 0 to 0.8 at frame 4 with Shadows 0 and Highlights 2, speed 0, Add: the dark tones gain grain as the midpoint rises. | largest difference 2.3e-7 | yes |
| FX-ADDGRAIN-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-028 frame 0: Monochromatic, Size 3, Softness 0.3, Aspect 0.5, Film, intensity 2, speed 0.25 smooth, seed 11: the controls together. | largest difference 3.2e-7 | yes |
| FX-ADDGRAIN-028 frame 1: Monochromatic, Size 3, Softness 0.3, Aspect 0.5, Film, intensity 2, speed 0.25 smooth, seed 11: the controls together. | largest difference 3.2e-7 | yes |
| FX-ADDGRAIN-028 frame 2: Monochromatic, Size 3, Softness 0.3, Aspect 0.5, Film, intensity 2, speed 0.25 smooth, seed 11: the controls together. | largest difference 3.0e-7 | yes |
| FX-ADDGRAIN-028 frame 3: Monochromatic, Size 3, Softness 0.3, Aspect 0.5, Film, intensity 2, speed 0.25 smooth, seed 11: the controls together. | largest difference 3.1e-7 | yes |
| FX-ADDGRAIN-028 frame 4: Monochromatic, Size 3, Softness 0.3, Aspect 0.5, Film, intensity 2, speed 0.25 smooth, seed 11: the controls together. | largest difference 3.0e-7 | yes |
| FX-ADDGRAIN-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-029 frame 0: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-029 frame 4: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-030 frame 0: Size 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-030 frame 4: Size 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-031 frame 0: Softness 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-031 frame 4: Softness 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-032 frame 0: Aspect Ratio 5, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-032 frame 4: Aspect Ratio 5, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-033 frame 0: Green Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-033 frame 4: Green Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-034 frame 0: Saturation 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-034 frame 4: Saturation 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-035 frame 0: Midpoint 1, above 0.99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-035 frame 4: Midpoint 1, above 0.99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-036 frame 0: Animation Speed 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-036 frame 4: Animation Speed 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-037 frame 0: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-037 frame 4: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-038 frame 0: Blending Mode "screen", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-038 frame 4: Blending Mode "screen", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-039 frame 0: Monochromatic "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-039 frame 4: Monochromatic "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-040 frame 0: Animate Smoothly "On", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-040 frame 4: Animate Smoothly "On", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ADDGRAIN-041 frame 0: Shadows keyed to 20 at frame 4, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-041 frame 4: Shadows keyed to 20 at frame 4, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_addgrain_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_028.json is saved with its words and numbers as written, and no frame | {"animate_smoothly":"on","animation_speed":0.25,"aspect_ratio":0.5,"blending_mode":"film","blue_intensity":1,"green_intensity":1,"highlights":1,"intensity":2,"midpoint":0.5,"midtones":1,"monochromatic":"on","random_seed":11,"red_intensity":1,"saturation":1,"shadows":1,"size":3,"softness":0.3} | yes |
| fx_addgrain_022.json is saved with Intensity's keys kept | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":4}]} | yes |
| fx_addgrain_029.json is refused in a sentence | Add Grain's intensity runs from 0 to 10, and this is 11. | yes |
| fx_addgrain_030.json is refused in a sentence | Add Grain's size runs from 0.1 to 100, and this is 0.05. | yes |
| fx_addgrain_031.json is refused in a sentence | Add Grain's softness runs from 0 to 1, and this is 1.5. | yes |
| fx_addgrain_032.json is refused in a sentence | Add Grain's aspect ratio runs from 0.25 to 4, and this is 5. | yes |
| fx_addgrain_033.json is refused in a sentence | Add Grain's green intensity runs from 0 to 10, and this is -1. | yes |
| fx_addgrain_034.json is refused in a sentence | Add Grain's saturation runs from 0 to 1, and this is 2. | yes |
| fx_addgrain_035.json is refused in a sentence | Add Grain's midpoint runs from 0.01 to 0.99, and this is 1. | yes |
| fx_addgrain_036.json is refused in a sentence | Add Grain's animation speed runs from 0 to 10, and this is 11. | yes |
| fx_addgrain_037.json is refused in a sentence | Add Grain's random seed runs from 0 to 100000, and this is 100001. | yes |
| fx_addgrain_038.json is refused in a sentence | Add Grain's blending mode is one of film, add, overlay, and this is "screen". | yes |
| fx_addgrain_039.json is refused in a sentence | Add Grain's monochromatic is "off" or "on", and this is "yes". | yes |
| fx_addgrain_040.json is refused in a sentence | Add Grain's animate smoothly is "on" or "off", and this is "On". | yes |
| a file with an Add Grain with no `blending_mode` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Add Grain whose animate smoothly is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Add Grain whose size is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| intensity 11 is refused with a sentence, and nothing changes | Add Grain's intensity runs from 0 to 10, and this is 11. | yes |
| size 0.05 is refused with a sentence, and nothing changes | Add Grain's size runs from 0.1 to 100, and this is 0.05. | yes |
| midpoint 1 is refused with a sentence, and nothing changes | Add Grain's midpoint runs from 0.01 to 0.99, and this is 1. | yes |
| monochromatic "yes" is refused with a sentence, and nothing changes | Add Grain's monochromatic is "off" or "on", and this is "yes". | yes |
| blending mode "screen" is refused with a sentence, and nothing changes | Add Grain's blending mode is one of film, add, overlay, and this is "screen". | yes |
| animate smoothly "On" is refused with a sentence, and nothing changes | Add Grain's animate smoothly is "on" or "off", and this is "On". | yes |
| shadows keyed to 20 is refused with a sentence, and nothing changes | Add Grain's shadows runs from 0 to 10, and this is 20. | yes |
| intensity 2.5, size 3, softness 0.4, aspect 1.5, channels 2, 0.5 and 3, saturation 0.25, shadows 2, midtones 0.5, highlights 4, midpoint 0.3, speed 2, seed 9, monochromatic, Overlay, not smooth is taken | taken | yes |
| intensity keyed from 0 to 4 is taken | taken | yes |
| size keyed from 1 to 5 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_addgrain_001.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_addgrain_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_addgrain_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_addgrain_024.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_addgrain_025.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_addgrain_028.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_addgrain_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_addgrain_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_addgrain_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_addgrain_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2070576 pixels changed | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour) on three layers, frame 0, Full | largest difference 1 of 255, 873 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour) on three layers, frame 100, Full | largest difference 1 of 255, 1094 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour) on three layers, frame 239, Full | largest difference 1 of 255, 765 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour) on three layers, frame 0, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour) on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain as added (intensity 1, size 1, Film, in colour) on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2001645 pixels changed | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3 on three layers, frame 0, Full | largest difference 1 of 255, 882 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3 on three layers, frame 100, Full | largest difference 1 of 255, 1063 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3 on three layers, frame 239, Full | largest difference 1 of 255, 791 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3 on three layers, frame 0, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3 on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2057903 pixels changed | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42 on three layers, frame 0, Full | largest difference 1 of 255, 1011 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42 on three layers, frame 100, Full | largest difference 1 of 255, 1118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42 on three layers, frame 239, Full | largest difference 1 of 255, 863 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42 on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42 on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42 on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072861 pixels changed | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25 on three layers, frame 0, Full | largest difference 1 of 255, 868 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25 on three layers, frame 100, Full | largest difference 1 of 255, 930 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25 on three layers, frame 239, Full | largest difference 1 of 255, 775 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25 on three layers, frame 0, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25 on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Add Grain intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25 on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-443 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: a fine colour grain over the whole street; draws cleanly | [], 129575 pixels changed of 129600, the largest by 25 of 255 | yes |
| 3_coarse_mono.png, frame 0, size 3, softness 0.5, monochromatic, intensity 2: a coarser grey grain; draws cleanly | [], 127012 pixels changed of 129600, the largest by 50 of 255 | yes |
| 4_shadows_overlay.png, frame 12, Overlay, intensity 3, shadows 4, midtones 0.5, highlights 0: heavy grain in the dark parts, none in the sky's brightest; draws cleanly | [], 127629 pixels changed of 129600, the largest by 56 of 255 | yes |
| 5_frame_24.png, frame 24, as added at frame 24: a different grain from frame 0's; draws cleanly | [], 129564 pixels changed of 129600, the largest by 25 of 255 | yes |

## Result

264 of 264 checks pass.
