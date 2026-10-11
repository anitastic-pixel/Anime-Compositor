# B-425: Lens Dirt (pick #14)

D-545 (EFFECTS.md row 261): Soft Physical Glow and Lens Flare each gain a Dirt Layer: the picture of another layer (a dirty lens, smudges, specks), centred on the layer and mirror-repeated past its edges unless Repeat Edges is off, shown only where the glow or the flare is bright, as dirt on a real lens lights up only in strong light. The matte is the light's brightness (Inherit Glow Color 0) or the light channel by channel (100), brightened by Matte Exposure and shaped by Matte Gamma; the dirt goes over the light at Opacity. Deep Glow 2's controls. Both effects draw as before without a dirt layer, so D-353's 45 fixtures, D-542's 22, D-543's 20, D-544's 18 and D-426's 19 are run again here unchanged. Every expected pixel is `Fixtures/lens_dirt/expected_lens_dirt.json`, written by `tools/lens_dirt_reference.py` before this code existed (FX-DIRT-001 to 029), compared sample by sample against the catalogue's tolerance of 2e-5. The dirt is drawn on the card too: the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).

## D-353's FX-SGLOW-001 to 045, unchanged: Soft Physical Glow draws as before

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
| fx_sglow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## D-542's FX-GTINT-001 to 022, unchanged

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

## D-543's FX-SPGLOW-001 to 020, unchanged

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

## D-544's FX-HALO-001 to 018, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HALO-001 frame 0: Halation as added (threshold 60, smooth 50, amount 100, red-orange #ff4020), radius 12: the yellow patch, the only light bright enough, gets a red-orange halo round it, strongest at its edge; the yellow itself is untouched. | largest difference 1.5e-7 | yes |
| FX-HALO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-002 frame 0: Threshold 0: every pixel's light counts, so each patch spreads onto anything darker beside it: halation round all three patches and on the darker patches' near edges. | largest difference 1.5e-7 | yes |
| FX-HALO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-003 frame 0: Radius 0: nothing spreads, so there is no halation: the drawing as it is. | largest difference 1.5e-7 | yes |
| FX-HALO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-004 frame 0: Radius 100: the halation spread wide and faint. | largest difference 1.5e-7 | yes |
| FX-HALO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-005 frame 0: Amount 300: three times as strong as FX-HALO-001 off the drawing. | largest difference 1.7e-7 | yes |
| FX-HALO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-006 frame 0: Amount 0: no halation: the drawing as it is. | largest difference 1.5e-7 | yes |
| FX-HALO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-007 frame 0: Colour white: the same halo, colourless. | largest difference 1.5e-7 | yes |
| FX-HALO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-008 frame 0: Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-HALO-001. | largest difference 1.5e-7 | yes |
| FX-HALO-008 frame 2: Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-HALO-001. | largest difference 1.5e-7 | yes |
| FX-HALO-008 frame 4: Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-HALO-001. | largest difference 1.5e-7 | yes |
| FX-HALO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-009 frame 0: The grey drawing: the grey patch is bright enough to give some halation too. | largest difference 1.5e-7 | yes |
| FX-HALO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-010 frame 0: The patches half covering: the light counts at half, and the halation outside the drawing covers by its brightest channel. | largest difference 9.5e-8 | yes |
| FX-HALO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-011 frame 0: Moved three pixels right: the halation follows, and none is cut off at the layer's edge. | largest difference 1.5e-7 | yes |
| FX-HALO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-012 frame 0: Threshold 30, smooth 0: a hard step: the yellow glows fully and nothing else. | largest difference 1.5e-7 | yes |
| FX-HALO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-013 frame 0: Radius 2.25: one level, half in: a thin halo one pixel wide. | largest difference 1.5e-7 | yes |
| FX-HALO-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-014 frame 0: Threshold 90, smooth 50: the yellow only partly bright enough, a weaker halo. | largest difference 1.5e-7 | yes |
| FX-HALO-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-015 frame 0: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-015 frame 4: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALO-016 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-016 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALO-017 frame 0: Colour "orange", not a colour #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-017 frame 4: Colour "orange", not a colour #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALO-018 frame 0: Amount keyed to 500 at frame 4, above 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-018 frame 4: Amount keyed to 500 at frame 4, above 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## D-426's FX-FLARE-001 to 019, unchanged: Lens Flare draws as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FLARE-001 frame 0: As added: the 50-300mm zoom flare at 30, 30 per cent, brightness 100: a white-hot core and a warm glow round (4.8, 3), six faint rays, a blue halo, and coloured ghosts along the line through the middle. | largest difference 1.9e-7 | yes |
| FX-FLARE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-002 frame 0: The flare at the middle, 50, 50: every ghost sits on the flare itself, so the picture is round about the middle. | largest difference 2.0e-7 | yes |
| FX-FLARE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-003 frame 0: Brightness 0: no light; the photo untouched. | largest difference 1.5e-7 | yes |
| FX-FLARE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-004 frame 0: Brightness 300: three times FX-FLARE-001's light. | largest difference 2.9e-7 | yes |
| FX-FLARE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-005 frame 0: The 35mm prime: a smaller glow, eight rays, three ghosts. | largest difference 1.8e-7 | yes |
| FX-FLARE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-006 frame 0: The 105mm prime: a bigger, warmer glow, twelve long rays, an orange halo, two ghosts. | largest difference 2.4e-7 | yes |
| FX-FLARE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-007 frame 0: Blend With Original 100: the photo untouched. | largest difference 1.5e-7 | yes |
| FX-FLARE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-008 frame 0: Blend With Original 50: halfway between FX-FLARE-001 and the photo. | largest difference 1.8e-7 | yes |
| FX-FLARE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-009 frame 0: On the shapes drawing, clear between its blocks: the light shows in the clear parts too, as covering as it is bright. | largest difference 1.9e-7 | yes |
| FX-FLARE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-010 frame 0: FX-FLARE-001 on the holder moved 2 right and 1 down: the same, moved; the flare is drawn on the drawing before it moves. | largest difference 1.9e-7 | yes |
| FX-FLARE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-011 frame 0: The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: frame 2 is FX-FLARE-002. | largest difference 1.9e-7 | yes |
| FX-FLARE-011 frame 2: The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: frame 2 is FX-FLARE-002. | largest difference 2.0e-7 | yes |
| FX-FLARE-011 frame 4: The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: frame 2 is FX-FLARE-002. | largest difference 1.8e-7 | yes |
| FX-FLARE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-012 frame 0: Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-FLARE-012 frame 2: Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is. | largest difference 2.9e-7 | yes |
| FX-FLARE-012 frame 4: Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is. | largest difference 2.9e-7 | yes |
| FX-FLARE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-013 frame 0: The flare off the layer, at -50, 150: only its ghosts and the tail of its glow and rays reach the drawing. | largest difference 1.8e-7 | yes |
| FX-FLARE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-014 frame 0: The 105mm prime at 70, 20, brightness 150, blend 25, on the ramp: the controls together. | largest difference 1.3e-7 | yes |
| FX-FLARE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-015 frame 0: Brightness 301, above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-015 frame 4: Brightness 301, above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-016 frame 0: Brightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-016 frame 4: Brightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-017 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-017 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-018 frame 0: A lens type written "200mm". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-018 frame 4: A lens type written "200mm". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-019 frame 0: A centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-019 frame 4: A centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_flare_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## FX-DIRT-001 to 029

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DIRT-001 frame 0: Soft Physical Glow, radius 12, with the 6 by 4 smudge as its dirt, every dirt setting as added (opacity 100, inherit 0, exposure 0, gamma 1, repeat on): the smudge, mirror-repeated over the whole grown layer, shows where the glow is bright, in its own colours, brightest beside the yellow patch. | largest difference 2.3e-7 | yes |
| FX-DIRT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-002 frame 0: Repeat Edges off: the smudge only in the middle, columns 5 to 10 and rows 3 to 6; elsewhere FX-SGLOW-001's glow. | largest difference 2.4e-7 | yes |
| FX-DIRT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-003 frame 0: Inherit Glow Color 100: the matte is the glow channel by channel, so the dirt takes the glow's warm colour. | largest difference 2.3e-7 | yes |
| FX-DIRT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-004 frame 0: Inherit Glow Color 50: halfway between FX-DIRT-001's matte and FX-DIRT-003's. | largest difference 2.0e-7 | yes |
| FX-DIRT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-005 frame 0: Matte Exposure 2: the matte four times as bright, held at 1, so the dirt shows further out from the patches. | largest difference 1.9e-7 | yes |
| FX-DIRT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-006 frame 0: Matte Exposure -2: the matte a quarter as bright, the dirt fainter. | largest difference 2.5e-7 | yes |
| FX-DIRT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-007 frame 0: Matte Gamma 2.2: the matte's faint parts raised, the dirt reaching further into the glow's tail. | largest difference 1.2e-7 | yes |
| FX-DIRT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-008 frame 0: Matte Gamma 0.5: the matte squared, the dirt only where the glow is strongest. | largest difference 2.5e-7 | yes |
| FX-DIRT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-009 frame 0: Opacity 0: no dirt, FX-SGLOW-001's glow exactly. | largest difference 2.4e-7 | yes |
| FX-DIRT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-010 frame 0: Opacity 50: half FX-DIRT-001's dirt. | largest difference 2.7e-7 | yes |
| FX-DIRT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-011 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the glow alone, frame 2 FX-DIRT-010, frame 4 FX-DIRT-001. | largest difference 2.4e-7 | yes |
| FX-DIRT-011 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the glow alone, frame 2 FX-DIRT-010, frame 4 FX-DIRT-001. | largest difference 2.7e-7 | yes |
| FX-DIRT-011 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the glow alone, frame 2 FX-DIRT-010, frame 4 FX-DIRT-001. | largest difference 2.3e-7 | yes |
| FX-DIRT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-012 frame 0: The 20 by 12 specks, bigger than the layer: centred, its two red edge columns fall in the grown ring outside the drawing, past the comp's view; the specks show where the glow is bright. | largest difference 2.3e-7 | yes |
| FX-DIRT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-013 frame 0: Moved three pixels right: the glow and its dirt move together, the dirt lying on the layer. | largest difference 2.3e-7 | yes |
| FX-DIRT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-014 frame 0: Tint core to edge (white to blue) with Inherit 100: the dirt takes the tinted glow's colour, bluer away from the patches. | largest difference 2.4e-7 | yes |
| FX-DIRT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-015 frame 0: Blend Mode Add, Exposure 3: the glow not held at 1, the matte held at 1, so the dirt is never brighter than itself. | largest difference 5.5e-7 | yes |
| FX-DIRT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-016 frame 0: A dirt layer that is not in the composition, `gone`: the layer drawn without the effect, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-DIRT-016 frame 4: A dirt layer that is not in the composition, `gone`: the layer drawn without the effect, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-DIRT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-DIRT-017 frame 0: Lens Flare as added on the photo with the smudge as its dirt: the smudge lit where the flare is bright, round its core at (4.8, 3) and on its ghosts. | largest difference 2.3e-7 | yes |
| FX-DIRT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-018 frame 0: Lens Flare, Inherit Glow Color 100: the dirt takes the flare's colours. | largest difference 2.5e-7 | yes |
| FX-DIRT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-019 frame 0: Lens Flare, Blend With Original 50: halfway between FX-DIRT-017 and the photo. | largest difference 2.2e-7 | yes |
| FX-DIRT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-020 frame 0: Lens Flare, Brightness 0: no light, so no matte and no dirt: the photo untouched. | largest difference 1.5e-7 | yes |
| FX-DIRT-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-021 frame 0: Lens Flare, Repeat Edges off, Matte Exposure 3: the dirt only in the middle, strong. | largest difference 3.5e-7 | yes |
| FX-DIRT-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-022 frame 0: Lens Flare, no dirt layer named: FX-FLARE-001 exactly. | largest difference 1.9e-7 | yes |
| FX-DIRT-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRT-023 frame 0: Soft Physical Glow, dirt opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-023 frame 4: Soft Physical Glow, dirt opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRT-024 frame 0: Soft Physical Glow, dirt inherit -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-024 frame 4: Soft Physical Glow, dirt inherit -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRT-025 frame 0: Soft Physical Glow, dirt exposure 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-025 frame 4: Soft Physical Glow, dirt exposure 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRT-026 frame 0: Soft Physical Glow, dirt gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-026 frame 4: Soft Physical Glow, dirt gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRT-027 frame 0: Soft Physical Glow, dirt repeat written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-027 frame 4: Soft Physical Glow, dirt repeat written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRT-028 frame 0: Lens Flare, dirt opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-028 frame 4: Lens Flare, dirt opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRT-029 frame 0: Lens Flare, dirt gamma keyed to 11 at frame 4, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-029 frame 4: Lens Flare, dirt gamma keyed to 11 at frame 4, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DIRT-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dirt_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirt_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a dirt opacity written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a dirt repeat written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| fx_dirt_023.json is refused in a sentence naming it | Soft Physical Glow's dirt opacity runs from 0 to 100, and this is 101. | yes |
| fx_dirt_026.json is refused in a sentence naming it | Soft Physical Glow's dirt gamma runs from 0.1 to 10, and this is 0.05. | yes |
| fx_dirt_027.json is refused in a sentence naming it | Soft Physical Glow's dirt repeat is "on" or "off", and this is "yes". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| dirt opacity 101 is refused with a sentence, and nothing changes | Soft Physical Glow's dirt opacity runs from 0 to 100, and this is 101. | yes |
| dirt gamma 0 is refused with a sentence, and nothing changes | Soft Physical Glow's dirt gamma runs from 0.1 to 10, and this is 0. | yes |
| dirt repeat "yes" is refused with a sentence, and nothing changes | Soft Physical Glow's dirt repeat is "on" or "off", and this is "yes". | yes |
| dirt layer specks, inherit 100, exposure 1, repeat off is taken | taken | yes |
| dirt opacity keyed 0 to 100 over frames 0 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| dirt gamma keyed 1 to 2, saved and opened again: the keys are kept | kept | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dirt_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirt_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirt_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirt_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirt_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirt_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dirt_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dirt_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_dirt_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added: the processor's frame 100 differs from the same at dirt opacity 0, so the comparisons below test the dirt | 156188 pixels changed | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added, on three layers, frame 0, Full | largest difference 1 of 255, 1535 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added, on three layers, frame 100, Full | largest difference 1 of 255, 621 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added, on three layers, frame 239, Full | largest difference 1 of 255, 356 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added, on three layers, frame 0, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added, on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added, on three layers, frame 239, Draft | largest difference 1 of 255, 25 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off: the processor's frame 100 differs from the same at dirt opacity 0, so the comparisons below test the dirt | 159114 pixels changed | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off, on three layers, frame 0, Full | largest difference 1 of 255, 851 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off, on three layers, frame 100, Full | largest difference 1 of 255, 339 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off, on three layers, frame 239, Full | largest difference 1 of 255, 300 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off, on three layers, frame 0, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off, on three layers, frame 100, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off, on three layers, frame 239, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2: the processor's frame 100 differs from the same at dirt opacity 0, so the comparisons below test the dirt | 140283 pixels changed | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2, on three layers, frame 0, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2, on three layers, frame 100, Full | largest difference 1 of 255, 1051 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2, on three layers, frame 239, Full | largest difference 1 of 255, 1056 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2, on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2, on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare, the fourth layer as its dirt, exposure 2, on three layers, frame 239, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the town, set in a larger frame, with smudges of our own as the dirt, in `verification/D-545 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| glow_plain.png, Soft Physical Glow (threshold 50, radius 40), no dirt: for comparison | 0 pixels differ from the same without dirt; warnings [] | yes |
| glow_dirt.png, the same with the smudges as its dirt, every dirt setting as added: smudges lit round the town's lights | 75232 pixels differ from the same without dirt; warnings [] | yes |
| glow_dirt_strong.png, Matte Exposure 3, Matte Gamma 2: the smudges show further into the glow | 87744 pixels differ from the same without dirt; warnings [] | yes |
| glow_dirt_colour.png, Inherit Glow Color 100: the smudges in the glow's own colours | 80825 pixels differ from the same without dirt; warnings [] | yes |
| glow_dirt_once.png, Repeat Edges off, Matte Exposure 3: the smudge picture once, in the middle | 8235 pixels differ from the same without dirt; warnings [] | yes |
| flare_plain.png, Lens Flare as added, no dirt: for comparison | 0 pixels differ from the same without dirt; warnings [] | yes |
| flare_dirt.png, the same with the smudges as its dirt, Matte Exposure 2: smudges lit round the flare | 31720 pixels differ from the same without dirt; warnings [] | yes |

## Result

463 of 463 checks pass.
