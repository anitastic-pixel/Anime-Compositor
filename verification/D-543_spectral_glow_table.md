# B-423: Soft Physical Glow's Spectral (pick #10, Spectral Glow)

D-543 (EFFECTS.md row 259): four new settings of Soft Physical Glow, after Deep Glow's Glow Aberration (PLUGINS.md 2.3): with Spectral on, red, green and blue each glow at their own share of Radius (80, 100 and 120 per cent as added), so the glow's tail splits into colour like light through a lens. A glow with Spectral off, or a file from before, draws exactly as before: D-353's 45 fixtures and D-542's 22 are run again here unchanged. Every expected pixel is `Fixtures/spectral_glow/expected_spectral_glow.json`, written by `tools/spectral_glow_reference.py` before this code existed (FX-SPGLOW-001 to 020), compared sample by sample against the catalogue's tolerance of 2e-5. Spectral is drawn on the card too: the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).

## D-353's FX-SGLOW-001 to 045, unchanged: Spectral off draws as before

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
| fx_sglow_001.json, a glow from before Spectral, is saved with none of Spectral's settings | {"aspect_angle":0,"aspect_ratio":1,"exposure":1,"falloff":"physical","operation":"screen","radius":12,"saturation_bias":0,"source_opacity":100,"threshold":0,"threshold_mode":"chroma","threshold_smooth":0,"unmult":"on"} | yes |

## D-542's FX-GTINT-001 to 022, unchanged: a tint with Spectral off draws as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GTINT-001 frame 0: Tint One Colour, orange, radius 12, everything else at Soft Physical Glow's defaults: the yellow, brown and purple patches all glow orange. | largest difference 1.7e-7 | yes |
| FX-GTINT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-002 frame 0: Tint Core to Edge at its defaults, white core and blue edge, radius 12: white close to each patch, turning blue as the glow spreads. | largest difference 2.2e-7 | yes |
| FX-GTINT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-003 frame 0: Core to Edge, radius 100: seven levels, so the change from white to blue runs over more steps. | largest difference 2.1e-7 | yes |
| FX-GTINT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-004 frame 0: Core to Edge, Tint Amount 50: halfway between the glow's own colours and the tint. | largest difference 2.2e-7 | yes |
| FX-GTINT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-005 frame 0: Core to Edge, Tint Amount 0: the glow's own colours, the same as Soft Physical Glow with no tint (FX-SGLOW-001). | largest difference 2.4e-7 | yes |
| FX-GTINT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-006 frame 0: Tint Off written in the file, with colours set: Soft Physical Glow as before (FX-SGLOW-001). | largest difference 2.4e-7 | yes |
| FX-GTINT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-007 frame 0: One Colour, orange, on the drawing with a grey patch: the grey glows orange too, as brightly as a colour of its brightness would. | largest difference 2.0e-7 | yes |
| FX-GTINT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-008 frame 0: Core to Edge, orange core and purple edge, Threshold 30 in Luminance: only the yellow glows, orange near it and purple further out. | largest difference 1.8e-7 | yes |
| FX-GTINT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-009 frame 0: Core to Edge, radius 0: nothing spreads; the light laid on itself takes the core colour, white, so each patch brightens towards grey-white. | largest difference 1.1e-7 | yes |
| FX-GTINT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-010 frame 0: Core to Edge, radius 2.25: the one level, half in, is the widest, so it is blue, and the half laid on itself is white. | largest difference 1.3e-7 | yes |
| FX-GTINT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-011 frame 0: Tint Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-GTINT-005, frame 2 is FX-GTINT-004, frame 4 is FX-GTINT-002. | largest difference 2.4e-7 | yes |
| FX-GTINT-011 frame 2: Tint Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-GTINT-005, frame 2 is FX-GTINT-004, frame 4 is FX-GTINT-002. | largest difference 2.2e-7 | yes |
| FX-GTINT-011 frame 4: Tint Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-GTINT-005, frame 2 is FX-GTINT-004, frame 4 is FX-GTINT-002. | largest difference 2.2e-7 | yes |
| FX-GTINT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-012 frame 0: Core to Edge, Blend Mode Add, exposure 3: the tinted light adds up past white. | largest difference 3.7e-7 | yes |
| FX-GTINT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-013 frame 0: Core to Edge, Unmult off, Source Opacity 0: the tinted glow alone, on black. | largest difference 9.5e-8 | yes |
| FX-GTINT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-014 frame 0: Core to Edge, Aspect Ratio 1.5 at Angle 30, moved three pixels right: the leaning glow tinted, and the layer grown to hold it. | largest difference 2.5e-7 | yes |
| FX-GTINT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-015 frame 0: Core to Edge, the patches half covering: the light counts at half, so the tint is half as bright. | largest difference 1.4e-7 | yes |
| FX-GTINT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-016 frame 0: Core to Edge, radius 2,000: on a drawing this small the wide blue levels spread their light far past the frame, so what stays is mostly the small, near-white levels: a faint wash, white touched with blue. | largest difference 2.0e-7 | yes |
| FX-GTINT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GTINT-017 frame 0: Tint "duocolor", which is not off, one_color or core_to_edge. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-017 frame 4: Tint "duocolor", which is not off, one_color or core_to_edge. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GTINT-018 frame 0: Inner Colour "#12345", not six hexadecimal digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-018 frame 4: Inner Colour "#12345", not six hexadecimal digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GTINT-019 frame 0: Outer Colour "blue", not a colour word the file takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-019 frame 4: Outer Colour "blue", not a colour word the file takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GTINT-020 frame 0: Tint Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-020 frame 4: Tint Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GTINT-021 frame 0: Tint Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-021 frame 4: Tint Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GTINT-022 frame 0: Tint Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-022 frame 4: Tint Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GTINT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-SPGLOW-001 to 020

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPGLOW-001 frame 0: Spectral on at its defaults (red 80, green 100, blue 120 per cent), radius 12, everything else at Soft Physical Glow's defaults: red stays closer to each patch and blue reaches further, so the tail turns bluer outward. | largest difference 2.4e-7 | yes |
| FX-SPGLOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-002 frame 0: Spectral on, all three at 100: the same as Soft Physical Glow with Spectral off (FX-SGLOW-001). | largest difference 2.4e-7 | yes |
| FX-SPGLOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-003 frame 0: Spectral off written in the file, with sizes set: Soft Physical Glow as before (FX-SGLOW-001). | largest difference 2.4e-7 | yes |
| FX-SPGLOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-004 frame 0: Spectral on, radius 100, red 50, green 100, blue 200: a strong fringe, the blue tail far wider than the red. | largest difference 2.1e-7 | yes |
| FX-SPGLOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-005 frame 0: Spectral on, red 200, green 100, blue 50, the other way round: a red tail outside a blue core. | largest difference 2.4e-7 | yes |
| FX-SPGLOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-006 frame 0: Spectral on, red 0: red does not spread at all, the light laid on itself only; green and blue glow as before. | largest difference 2.4e-7 | yes |
| FX-SPGLOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-007 frame 0: Spectral on with the grey drawing: the grey patch, white light, glows white close in and fringes in colour further out. | largest difference 2.2e-7 | yes |
| FX-SPGLOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-008 frame 0: Spectral on with Tint Core to Edge: each channel's levels tinted at its own radius, then the channels taken one from each. | largest difference 2.2e-7 | yes |
| FX-SPGLOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-009 frame 0: Spectral on with Tint One Colour orange at 60 per cent, Threshold 30 and Smooth 50 in Luminance. | largest difference 2.1e-7 | yes |
| FX-SPGLOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-010 frame 0: Blue keyed from 100 at frame 0 to 200 at frame 4, linear, red and green at 100: frame 0 is FX-SPGLOW-002, frame 2 has blue at 150. | largest difference 2.4e-7 | yes |
| FX-SPGLOW-010 frame 2: Blue keyed from 100 at frame 0 to 200 at frame 4, linear, red and green at 100: frame 0 is FX-SPGLOW-002, frame 2 has blue at 150. | largest difference 2.4e-7 | yes |
| FX-SPGLOW-010 frame 4: Blue keyed from 100 at frame 0 to 200 at frame 4, linear, red and green at 100: frame 0 is FX-SPGLOW-002, frame 2 has blue at 150. | largest difference 2.4e-7 | yes |
| FX-SPGLOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-011 frame 0: Spectral on, Blend Mode Add, exposure 3: the fringed light adds up past white. | largest difference 4.8e-7 | yes |
| FX-SPGLOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-012 frame 0: Spectral on, Unmult off, Source Opacity 0: the fringed glow alone, on black. | largest difference 8.0e-8 | yes |
| FX-SPGLOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-013 frame 0: Spectral on, Aspect Ratio 1.5 at Angle 30, moved three pixels right. | largest difference 2.3e-7 | yes |
| FX-SPGLOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-014 frame 0: Spectral on, radius 2.25, red 50, blue 200: red has no level (radius 1.125), green one half-in level, blue two. | largest difference 1.1e-7 | yes |
| FX-SPGLOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-015 frame 0: Spectral on, radius 2,000, red 50, blue 200: blue's radius 4,000, past Radius's own top, is worked as it is. | largest difference 1.9e-7 | yes |
| FX-SPGLOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-016 frame 0: Spectral on, the patches half covering: the light counts at half. | largest difference 1.7e-7 | yes |
| FX-SPGLOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPGLOW-017 frame 0: Spectral "yes", which is not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-017 frame 4: Spectral "yes", which is not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPGLOW-018 frame 0: Spectral Red 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-018 frame 4: Spectral Red 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPGLOW-019 frame 0: Spectral Blue -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-019 frame 4: Spectral Blue -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPGLOW-020 frame 0: Spectral Green keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-020 frame 4: Spectral Green keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPGLOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spglow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spglow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a spectral red written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a spectral written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| fx_spglow_017.json is refused in a sentence naming it | Soft Physical Glow's spectral is "off" or "on", and this is "yes". | yes |
| fx_spglow_018.json is refused in a sentence naming it | Soft Physical Glow's spectral red runs from 0 to 200, and this is 201. | yes |
| fx_spglow_019.json is refused in a sentence naming it | Soft Physical Glow's spectral blue runs from 0 to 200, and this is -1. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| spectral "yes" is refused with a sentence, and nothing changes | Soft Physical Glow's spectral is "off" or "on", and this is "yes". | yes |
| spectral red 201 is refused with a sentence, and nothing changes | Soft Physical Glow's spectral red runs from 0 to 200, and this is 201. | yes |
| spectral blue -1 is refused with a sentence, and nothing changes | Soft Physical Glow's spectral blue runs from 0 to 200, and this is -1. | yes |
| red 50, blue 200 is taken | taken | yes |
| spectral blue keyed 100 to 200 over frames 0 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spglow_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spglow_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spglow_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spglow_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spglow_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spglow_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120): the processor's frame 100 differs from the same glow with Spectral off, so the comparisons below test Spectral | 1452379 pixels changed | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120) on three layers, frame 0, Full | largest difference 1 of 255, 458 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120) on three layers, frame 100, Full | largest difference 1 of 255, 468 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120) on three layers, frame 239, Full | largest difference 1 of 255, 306 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120) on three layers, frame 0, Draft | largest difference 1 of 255, 25 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120) on three layers, frame 100, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on as added (radius 500; 80, 100, 120) on three layers, frame 239, Draft | largest difference 1 of 255, 17 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50: the processor's frame 100 differs from the same glow with Spectral off, so the comparisons below test Spectral | 1178549 pixels changed | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50 on three layers, frame 0, Full | largest difference 1 of 255, 470 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50 on three layers, frame 100, Full | largest difference 1 of 255, 606 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50 on three layers, frame 239, Full | largest difference 1 of 255, 337 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50 on three layers, frame 0, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50 on three layers, frame 100, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50 on three layers, frame 239, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off: the processor's frame 100 differs from the same glow with Spectral off, so the comparisons below test Spectral | 414261 pixels changed | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off on three layers, frame 100, Full | largest difference 1 of 255, 91 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off on three layers, frame 100, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the town, set in a larger frame, in `verification/D-543 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| spectral_off.png, Soft Physical Glow, threshold 60 with smooth 50, radius 60, Spectral off, for comparison | the tail 10 pixels out, red 255 green 171 blue 126 covering 8; warnings [] | yes |
| spectral_on.png, the same, Spectral on as added (red 80, green 100, blue 120): the tail bluer than with Spectral off | the tail 10 pixels out, red 255 green 186 blue 165 covering 6; warnings [] | yes |
| spectral_strong.png, the same, red 50 and blue 200: a strong fringe, blue far out, red held close | the tail 10 pixels out, red 212 green 196 blue 255 covering 6; warnings [] | yes |
| spectral_reversed.png, the same, red 200 and blue 50: the other way round, the tail redder than with Spectral off | the tail 10 pixels out, red 255 green 153 blue 39 covering 10; warnings [] | yes |

## Result

327 of 327 checks pass.
