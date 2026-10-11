# D-561: CC Glass, its full control set

B-441: `core.glass` takes the rest of After Effects' CC Glass controls (Light Type, Light Height, Light Position, Ambient, Diffuse, Specular, Roughness, Metal) beside D-317's. A file without the light type keeps D-317's light. Every expected pixel is `Fixtures/glass/expected_glass.json`, written by `tools/glass_reference.py` before this code existed and printed in document 25 as FX-GLASS-001 to 027. Tolerance 2e-5.

## FX-GLASS-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLASS-001 frame 0: As added: the layer's own covering as the bump, Softness 10, Height 50, Displacement 100, a white distant light from the top left at height 25, Ambient 75, Diffuse 90, Specular 25, Roughness 0.1, Metal 100: the two blocks bent outward by their soft edges and lit, lighter toward the top left. | largest difference 6.1e-6 | yes |
| FX-GLASS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-002 frame 0: A file written before D-561, without the eight new settings, Displacement 20 and Light Intensity 70: read as the classic light, so it draws as CC Glass drew before, D-317's bend and Bevel Alpha's light. | largest difference 1.9e-6 | yes |
| FX-GLASS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-003 frame 0: The same settings with the light type written `classic`: the same as FX-GLASS-002. | largest difference 1.9e-6 | yes |
| FX-GLASS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-004 frame 0: Ambient 100, Diffuse 0, Specular 0, Displacement 0: nothing is bent or lit, so the drawing comes back untouched. | largest difference 1.5e-7 | yes |
| FX-GLASS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-005 frame 0: Height 0: a flat surface, nothing bent, every covered pixel its colour times Ambient plus Diffuse times 0.24, and the faintest highlight in its own colour. | largest difference 1.6e-7 | yes |
| FX-GLASS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-006 frame 0: Softness 4, Displacement 0: the light alone, lighter on the top and left edges of each block, darker on the bottom and right. | largest difference 2.2e-7 | yes |
| FX-GLASS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-007 frame 0: The light's Direction 135, from the bottom right: the other edges light. | largest difference 1.9e-7 | yes |
| FX-GLASS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-008 frame 0: A point light at 25, 30 per cent, 10 pixels up: brightest near (4, 3), falling away across the blocks. | largest difference 3.5e-7 | yes |
| FX-GLASS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-009 frame 0: The light's Height -50, behind the layer: the flat middles face away from it, so only Ambient lights them, at 0.75 of their colour. | largest difference 9.9e-8 | yes |
| FX-GLASS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-010 frame 0: Specular 100, Roughness 0.5, Metal 0, an orange light, Ambient and Diffuse 0, light Height 40: a broad highlight in the light's own orange on the blue block too. | largest difference 2.1e-7 | yes |
| FX-GLASS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-011 frame 0: The same with Metal 100: the highlight takes each block's own colour, so the blue block's shine is blue. | largest difference 1.6e-7 | yes |
| FX-GLASS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-012 frame 0: Light Intensity 0, Displacement 20: the blocks bent, every covered pixel at Ambient, 0.75, of the colour bent onto it. | largest difference 1.9e-6 | yes |
| FX-GLASS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-013 frame 0: The ramp's luminance as the bump on the photo, Softness 0, Height 100, Displacement 3: the squares pushed sideways along the even slope, lit alike. | largest difference 2.8e-7 | yes |
| FX-GLASS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-014 frame 0: The dots layer as the bump, its covering, Softness 2, Displacement 10, on the solid photo: the photo bent and lit round two squares. | largest difference 4.4e-7 | yes |
| FX-GLASS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-015 frame 0: The 8 by 5 spot stretched to the drawing's size, Property Red, Softness 2, Displacement 10, on the photo: an oval lens in the middle. | largest difference 4.5e-7 | yes |
| FX-GLASS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-016 frame 0: Property Intensity on the photo itself, Softness 2, Height -50, Displacement 10: the cream squares sunk below the red and blue. | largest difference 6.4e-7 | yes |
| FX-GLASS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-017 frame 0: A layer that is not in the composition, `gone`: the layer itself is the bump, as FX-GLASS-001, and the warning every frame. | largest difference 6.1e-6 | yes |
| FX-GLASS-017 frame 4: A layer that is not in the composition, `gone`: the layer itself is the bump, as FX-GLASS-001, and the warning every frame. | largest difference 6.1e-6 | yes |
| FX-GLASS-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-GLASS-018 frame 0: Softness keyed from 0 at frame 0 to 8 at frame 4, linear, Displacement 0: frame 2 is FX-GLASS-006. | largest difference 1.8e-7 | yes |
| FX-GLASS-018 frame 2: Softness keyed from 0 at frame 0 to 8 at frame 4, linear, Displacement 0: frame 2 is FX-GLASS-006. | largest difference 2.2e-7 | yes |
| FX-GLASS-018 frame 4: Softness keyed from 0 at frame 0 to 8 at frame 4, linear, Displacement 0: frame 2 is FX-GLASS-006. | largest difference 2.4e-7 | yes |
| FX-GLASS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-019 frame 0: FX-GLASS-006 on the holder moved 2 right and 1 down: the same, moved. | largest difference 2.2e-7 | yes |
| FX-GLASS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-020 frame 0: Ambient eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, as at frame 4. | largest difference 1.0e-7 | yes |
| FX-GLASS-020 frame 2: Ambient eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, as at frame 4. | largest difference 2.7e-7 | yes |
| FX-GLASS-020 frame 4: Ambient eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, as at frame 4. | largest difference 2.7e-7 | yes |
| FX-GLASS-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-021 frame 0: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-021 frame 4: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASS-022 frame 0: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-022 frame 4: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASS-023 frame 0: Ambient 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-023 frame 4: Ambient 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASS-024 frame 0: Light Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-024 frame 4: Light Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASS-025 frame 0: Specular -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-025 frame 4: Specular -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASS-026 frame 0: Light Position 50, -1001, beyond -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-026 frame 4: Light Position 50, -1001, beyond -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASS-027 frame 0: Metal keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-027 frame 4: Metal keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glass_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_013.json is saved with its seventeen settings | {"ambient":75,"diffuse":90,"displacement":3,"fit":"stretch","height":100,"layer":"ramp","light_angle":-45,"light_color":"#ffffff","light_height":25,"light_intensity":100,"light_position":[50,50],"light_type":"distant","metal":100,"property":"luminance","roughness":0.1,"softness":0,"specular":25} | yes |
| fx_glass_002.json, written before D-561, is saved with its nine settings as read, no light type added | {"displacement":20,"fit":"stretch","height":50,"layer":"","light_angle":-45,"light_color":"#ffffff","light_intensity":70,"property":"alpha","softness":10} | yes |
| fx_glass_021.json is refused in a sentence naming CC Glass and its light type | CC Glass's light type is classic, distant or point, and this is "spot". | yes |
| fx_glass_022.json is refused in a sentence naming CC Glass and its roughness | CC Glass's roughness runs from 0.001 to 1, and this is 0. | yes |
| fx_glass_023.json is refused in a sentence naming CC Glass and its ambient | CC Glass's ambient runs from 0 to 100, and this is 101. | yes |
| fx_glass_024.json is refused in a sentence naming CC Glass and its light height | CC Glass's light height runs from -1000 to 1000, and this is 1001. | yes |
| fx_glass_025.json is refused in a sentence naming CC Glass and its specular | CC Glass's specular runs from 0 to 100, and this is -1. | yes |
| fx_glass_026.json is refused in a sentence naming CC Glass and its light position | CC Glass's light position runs from -1000 to 1000, and this is -1001. | yes |
| a file with a CC Glass whose ambient is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Glass with a light type but without its metal is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Glass whose light position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Glass whose light type is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| ambient 101 is refused with a sentence, and nothing changes | CC Glass's ambient runs from 0 to 100, and this is 101. | yes |
| light position 50, -1001 is refused with a sentence, and nothing changes | CC Glass's light position runs from -1000 to 1000, and this is -1001. | yes |
| light type spot is refused with a sentence, and nothing changes | CC Glass's light type is classic, distant or point, and this is "spot". | yes |
| metal keyed to 150 is refused with a sentence, and nothing changes | CC Glass's metal runs from 0 to 100, and this is 150. | yes |
| the ramp, Luminance, softness 3, an orange point light 60 up, specular 70, roughness 0.4, metal 0 is taken | taken | yes |
| the classic light is taken | taken | yes |
| light position keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glass_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glass_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glass_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Glass as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2071971 pixels changed | yes |
| the reference shot, CC Glass as added on three layers, frame 0, Full | largest difference 1 of 255, 205 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass as added on three layers, frame 100, Full | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass as added on three layers, frame 239, Full | largest difference 1 of 255, 102 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass as added on three layers, frame 0, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass as added on three layers, frame 100, Draft | largest difference 1 of 255, 104 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass as added on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 153166 pixels changed | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70 on three layers, frame 0, Full | largest difference 1 of 255, 3782 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70 on three layers, frame 239, Full | largest difference 1 of 255, 1773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass the classic light, displacement 20, intensity 70 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2071766 pixels changed | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0 on three layers, frame 0, Full | largest difference 1 of 255, 993 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0 on three layers, frame 100, Full | largest difference 1 of 255, 1104 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0 on three layers, frame 239, Full | largest difference 1 of 255, 1022 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0 on three layers, frame 0, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0 on three layers, frame 100, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Glass layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0 on three layers, frame 239, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-561 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added (its own covering as the bump, softness 10, height 50, displacement 100, a white distant light): the picture's edge bent like thick glass and lit from the top left; draws cleanly | [], changed true | yes |
| 3_classic_light.png, the same with the classic light (as older projects open): the bend of 2 with Bevel Alpha's light instead, so it differs from 2_as_added.png; draws cleanly | [], different true | yes |
| 4_luminance_metal.png and 5_luminance_orange_light.png, the street's lightness as glass ripples under an orange light with Metal 100 and 0: the shine takes the street's own colours in 4 and the light's orange in 5, so they differ; draw cleanly | [] [], different true | yes |
| 6_point_light.png, a point light at 25, 30 per cent, 80 pixels up: brightest toward the top left, not 2_as_added.png; draws cleanly | [], different true | yes |
| 7_flat.png, Displacement 0, Ambient 100, Diffuse 0, Specular 0: the street unchanged within 1 level at its middle pixel; draws cleanly | [], middle [90, 140, 170, 255] against [90, 140, 170, 255] | yes |

## Result

175 of 175 checks pass.
