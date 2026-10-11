# D-463: Plastic

B-343: `core.plastic` takes After Effects' CC Plastic controls (Bump Layer, Property, Softness, Height, Cut Min, Cut Max, the light, Ambient, Diffuse, Dust, Specular, Roughness, Metal). Every expected pixel is `Fixtures/plastic/expected_plastic.json`, written by `tools/plastic_reference.py` before this code existed and printed in document 25 as FX-PLASTIC-001 to 031. Tolerance 2e-5.

## FX-PLASTIC-001 to 031 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PLASTIC-001 frame 0: As added: the layer's own covering as the bump, Softness 10, Height 50, Cut 0 to 100, a white distant light from the top left at height 100, Ambient 25, Diffuse 75, Dust 0, Specular 50, Roughness 0.05, Metal 0: the two blocks keep their colours and shapes, their edges lit as if raised, lighter facing the top left. | largest difference 2.2e-7 | yes |
| FX-PLASTIC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-002 frame 0: Ambient 100, Diffuse 0, Specular 0, a white ambient light: nothing is lit, so the drawing comes back untouched. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-003 frame 0: Height 0: a flat surface, every covered pixel its colour times Ambient plus Diffuse times 0.71, and the faintest highlight. | largest difference 1.1e-7 | yes |
| FX-PLASTIC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-004 frame 0: Softness 4: a narrower, steeper rim round each block, lighter on the top and left edges, darker on the bottom and right. | largest difference 3.0e-7 | yes |
| FX-PLASTIC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-005 frame 0: The light's Direction 135, from the bottom right: the other edges light. | largest difference 1.7e-7 | yes |
| FX-PLASTIC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-006 frame 0: A point light at 25, 30 per cent, 10 pixels up: brightest near (4, 3), falling away across the blocks. | largest difference 2.3e-7 | yes |
| FX-PLASTIC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-007 frame 0: The light's Height -50, behind the layer: the flat middles face away from it, so only Ambient lights them. | largest difference 3.8e-8 | yes |
| FX-PLASTIC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-008 frame 0: Specular 100, Roughness 0.5, Metal 0, an orange light, Diffuse 0, light Height 40: a broad highlight in the light's own orange on the blue block too. | largest difference 2.1e-7 | yes |
| FX-PLASTIC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-009 frame 0: The same with Metal 100: the highlight takes each block's own colour, so the blue block's shine is blue. | largest difference 1.9e-7 | yes |
| FX-PLASTIC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-010 frame 0: A blue ambient light, Ambient 100, Diffuse 0, Specular 0: every colour multiplied by the blue, the red block near black. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-011 frame 0: Dust 100 alone (Ambient, Diffuse and Specular 0), Height 0: a softer light than Diffuse's, every covered pixel at 0.41 of its colour where Diffuse would give 0.71. | largest difference 6.4e-8 | yes |
| FX-PLASTIC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-012 frame 0: The ramp's luminance as the bump on the photo, Softness 0, Height 100: one even slope rising to the right, lit alike in every column but the two at the sides. | largest difference 2.1e-7 | yes |
| FX-PLASTIC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-013 frame 0: The same, Cut Min 30 and Cut Max 70: the darkest and brightest columns lie flat, the slope steeper between. | largest difference 4.7e-7 | yes |
| FX-PLASTIC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-014 frame 0: The same, Cut Min 70 and Cut Max 30: the slope turned inside out, falling to the right. | largest difference 1.9e-7 | yes |
| FX-PLASTIC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-015 frame 0: Cut Min and Cut Max both 50: a single step where the ramp passes 50 per cent, flat either side. | largest difference 1.6e-7 | yes |
| FX-PLASTIC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-016 frame 0: The dots layer as the bump, its covering, Softness 2, on the solid photo: two round lit bumps where the dots are, the photo whole. | largest difference 1.8e-7 | yes |
| FX-PLASTIC-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-017 frame 0: The 8 by 5 spot stretched to the drawing's size, Property Red, Softness 2, on the photo: an oval bump in the middle. | largest difference 1.7e-7 | yes |
| FX-PLASTIC-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-018 frame 0: A layer that is not in the composition, `gone`: the layer itself is the bump, as FX-PLASTIC-001, and the warning every frame. | largest difference 2.2e-7 | yes |
| FX-PLASTIC-018 frame 4: A layer that is not in the composition, `gone`: the layer itself is the bump, as FX-PLASTIC-001, and the warning every frame. | largest difference 2.2e-7 | yes |
| FX-PLASTIC-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PLASTIC-019 frame 0: Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is FX-PLASTIC-004. | largest difference 1.4e-7 | yes |
| FX-PLASTIC-019 frame 2: Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is FX-PLASTIC-004. | largest difference 3.0e-7 | yes |
| FX-PLASTIC-019 frame 4: Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is FX-PLASTIC-004. | largest difference 2.1e-7 | yes |
| FX-PLASTIC-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-020 frame 0: FX-PLASTIC-004 on the holder moved 2 right and 1 down: the same, moved. | largest difference 3.0e-7 | yes |
| FX-PLASTIC-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-021 frame 0: Cut Min eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the bump is a step at full covering, as at frame 4. | largest difference 3.0e-7 | yes |
| FX-PLASTIC-021 frame 2: Cut Min eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the bump is a step at full covering, as at frame 4. | largest difference 1.1e-7 | yes |
| FX-PLASTIC-021 frame 4: Cut Min eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the bump is a step at full covering, as at frame 4. | largest difference 1.1e-7 | yes |
| FX-PLASTIC-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-022 frame 0: Property Lightness on the photo itself, Softness 2, Height -50: the cream squares sunk below the red and blue. | largest difference 1.9e-7 | yes |
| FX-PLASTIC-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PLASTIC-023 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-023 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-024 frame 0: Height 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-024 frame 4: Height 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-025 frame 0: Cut Min -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-025 frame 4: Cut Min -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-026 frame 0: A property written "hue", which Plastic does not read. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-026 frame 4: A property written "hue", which Plastic does not read. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-027 frame 0: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-027 frame 4: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-028 frame 0: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-028 frame 4: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-029 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-029 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-030 frame 0: An ambient light colour written "blue", not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-030 frame 4: An ambient light colour written "blue", not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PLASTIC-031 frame 0: Dust keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-031 frame 4: Dust keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-PLASTIC-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_plastic_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_plastic_013.json is saved with its twenty settings | {"ambient":25,"ambient_light_color":"#ffffff","cut_max":70,"cut_min":30,"diffuse":75,"dust":0,"fit":"stretch","height":100,"layer":"ramp","light_color":"#ffffff","light_direction":-45,"light_height":100,"light_intensity":100,"light_position":[30,30],"light_type":"distant","metal":0,"property":"luminance","roughness":0.05,"softness":0,"specular":50} | yes |
| fx_plastic_023.json is refused in a sentence naming Plastic and its softness | Plastic's softness runs from 0 to 100, and this is 101. | yes |
| fx_plastic_024.json is refused in a sentence naming Plastic and its height | Plastic's height runs from -100 to 100, and this is 101. | yes |
| fx_plastic_025.json is refused in a sentence naming Plastic and its cut min | Plastic's cut min runs from 0 to 100, and this is -1. | yes |
| fx_plastic_026.json is refused in a sentence naming Plastic and its property | Plastic's property is red, green, blue, alpha, luminance or lightness, and this is "hue". | yes |
| fx_plastic_027.json is refused in a sentence naming Plastic and its light type | Plastic's light type is distant or point, and this is "spot". | yes |
| fx_plastic_028.json is refused in a sentence naming Plastic and its roughness | Plastic's roughness runs from 0.001 to 1, and this is 0. | yes |
| fx_plastic_029.json is refused in a sentence naming Plastic and its bump layer | Plastic's bump layer is the name of a layer of this composition, and this is 3. | yes |
| fx_plastic_030.json is refused in a sentence naming Plastic and its ambient light colour | Plastic's ambient light colour is written #rrggbb, and this is "blue". | yes |
| a file with a Plastic whose softness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Plastic without its metal is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Plastic whose light position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Plastic whose ambient light colour is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| softness 101 is refused with a sentence, and nothing changes | Plastic's softness runs from 0 to 100, and this is 101. | yes |
| light position 50, -1001 is refused with a sentence, and nothing changes | Plastic's light position runs from -1000 to 1000, and this is -1001. | yes |
| light type spot is refused with a sentence, and nothing changes | Plastic's light type is distant or point, and this is "spot". | yes |
| metal keyed to 150 is refused with a sentence, and nothing changes | Plastic's metal runs from 0 to 100, and this is 150. | yes |
| the ramp tiled, Luminance, softness 3, cut min 20, an orange point light, metal 60 is taken | taken | yes |
| light position keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_plastic_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_plastic_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_plastic_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_plastic_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_plastic_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_plastic_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_plastic_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_plastic_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_plastic_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Plastic as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073570 pixels changed | yes |
| the reference shot, Plastic as added on three layers, frame 0, Full | largest difference 1 of 255, 285 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic as added on three layers, frame 100, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic as added on three layers, frame 239, Full | largest difference 1 of 255, 161 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic as added on three layers, frame 0, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic as added on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic as added on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072971 pixels changed | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100 on three layers, frame 0, Full | largest difference 1 of 255, 1289 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100 on three layers, frame 100, Full | largest difference 1 of 255, 1228 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100 on three layers, frame 239, Full | largest difference 1 of 255, 1255 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100 on three layers, frame 0, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100 on three layers, frame 100, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100 on three layers, frame 239, Draft | largest difference 1 of 255, 89 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5 on three layers, frame 0, Full | largest difference 1 of 255, 572 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5 on three layers, frame 100, Full | largest difference 1 of 255, 856 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5 on three layers, frame 239, Full | largest difference 1 of 255, 603 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5 on three layers, frame 0, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5 on three layers, frame 100, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Plastic Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5 on three layers, frame 239, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-463 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added (its own covering as the bump, softness 10, height 50): the picture's edge rounded like a lit plastic tile, light from the top left, its covering exactly as before; draws cleanly | [], changed true, coverings the same true | yes |
| 3_luminance_relief.png, Luminance, softness 4, height 60: the street's light and dark parts standing out as a shiny relief, not 2_as_added.png, the covering kept; draws cleanly | [], different true, coverings the same true | yes |
| 4_metal.png and 5_plastic_orange_light.png, 3 under an orange light with Metal 100 and 0: the shine takes the street's own colours in 4 and the light's orange in 5, so they differ; draw cleanly | [] [], different true | yes |
| 6_flat.png, Ambient 100, Diffuse 0, Specular 0: the street unchanged within 1 level at its middle pixel; draws cleanly | [], middle [90, 140, 170, 255] against [90, 140, 170, 255] | yes |

## Result

191 of 191 checks pass.
