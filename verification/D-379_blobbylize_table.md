# D-379: Blobbylize

B-258, after CycoreFX's CC Blobbylize: a blob map, the layer's own or another layer's red, green, blue, alpha, luminance or lightness, softened and cut, becomes the layer's covering and a surface lit by a distant or point light, ambient, diffuse and a shine whose colour leans to the layer's by Metal. The formulas are this program's own. Every expected pixel is `Fixtures/blobbylize/expected_blobbylize.json`, written by `tools/blobbylize_reference.py` before this code existed and printed in document 25 as FX-BLOB-001 to 027. Tolerance 2e-5.

## FX-BLOB-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLOB-001 frame 0: As added: the layer's own covering as the blob, Softness 10, Cut Away 0, a white distant light from the top left at height 100, Ambient 25, Diffuse 75, Specular 50, Roughness 0.05, Metal 100: the two blocks melt into one soft, lit blob, lighter on its upper left slopes. | largest difference 1.0e-7 | yes |
| FX-BLOB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-002 frame 0: Softness 0, Ambient 100, Diffuse 0, Specular 0: the blob is the covering itself and nothing is lit, so the drawing comes back untouched. | largest difference 1.5e-7 | yes |
| FX-BLOB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-003 frame 0: Softness 4, unlit: the blocks' edges go soft and the three clear columns between them fill part way, their colour spread from the blocks, red into blue. | largest difference 1.2e-7 | yes |
| FX-BLOB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-004 frame 0: The same, Cut Away 30: the softest 30 per cent cut away and the rest stretched back to full, a firmer blob, the gap thinner. | largest difference 1.7e-7 | yes |
| FX-BLOB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-005 frame 0: Cut Away 100: everything cut away, an empty frame. | largest difference 0.0e0 | yes |
| FX-BLOB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-006 frame 0: Softness 4, lit as added: lighter on the slopes that face the top left, darker on those facing away, the flat middles at Ambient plus Diffuse times 0.71. | largest difference 2.6e-7 | yes |
| FX-BLOB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-007 frame 0: The light's Direction 135, from the bottom right: the other slopes light. | largest difference 1.5e-7 | yes |
| FX-BLOB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-008 frame 0: A point light at 25, 30 per cent, 10 pixels up: brightest near (4, 3), falling away across the blocks. | largest difference 1.4e-7 | yes |
| FX-BLOB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-009 frame 0: The light's Height -50, behind the layer: the flat middles face away from it, so only Ambient lights them; a slope tipped far enough toward it catches a little. | largest difference 1.2e-7 | yes |
| FX-BLOB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-010 frame 0: Specular 100, Roughness 0.5, Metal 0, an orange light, Diffuse 0: a broad highlight in the light's own orange, not the blocks' colour. | largest difference 2.4e-7 | yes |
| FX-BLOB-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-011 frame 0: An orange light at Intensity 150, Diffuse 100: the blocks tinted warm and brighter. | largest difference 3.7e-7 | yes |
| FX-BLOB-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-012 frame 0: The dots layer as the blob, its covering, Softness 2, on the solid photo: the photo cut down to two soft round blobs where the dots are. | largest difference 8.4e-8 | yes |
| FX-BLOB-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-013 frame 0: The ramp as the blob, Property Luminance, Softness 0, unlit, on the photo: the photo fading in from clear at the left to solid at the right. | largest difference 1.9e-7 | yes |
| FX-BLOB-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-014 frame 0: The 8 by 5 spot stretched to the drawing's size, Property Red, Softness 2, on the photo: an oval blob in the middle. | largest difference 1.4e-7 | yes |
| FX-BLOB-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-015 frame 0: A layer that is not in the composition, `gone`: the layer itself is the blob, as FX-BLOB-001, and the warning every frame. | largest difference 1.0e-7 | yes |
| FX-BLOB-015 frame 4: A layer that is not in the composition, `gone`: the layer itself is the blob, as FX-BLOB-001, and the warning every frame. | largest difference 1.0e-7 | yes |
| FX-BLOB-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-BLOB-016 frame 0: Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is FX-BLOB-006. | largest difference 1.7e-7 | yes |
| FX-BLOB-016 frame 2: Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is FX-BLOB-006. | largest difference 2.6e-7 | yes |
| FX-BLOB-016 frame 4: Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is FX-BLOB-006. | largest difference 9.6e-8 | yes |
| FX-BLOB-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-017 frame 0: FX-BLOB-006 on the holder moved 2 right and 1 down: the same, moved. | largest difference 2.6e-7 | yes |
| FX-BLOB-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-018 frame 0: Cut Away eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the frame is empty, as frame 4 is. | largest difference 2.6e-7 | yes |
| FX-BLOB-018 frame 2: Cut Away eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the frame is empty, as frame 4 is. | largest difference 0.0e0 | yes |
| FX-BLOB-018 frame 4: Cut Away eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the frame is empty, as frame 4 is. | largest difference 0.0e0 | yes |
| FX-BLOB-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-019 frame 0: Property Lightness on the photo itself, Softness 2, Cut Away 20, lit: the cream squares stand up as blobs, the red and blue lower. | largest difference 1.8e-7 | yes |
| FX-BLOB-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOB-020 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-020 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-021 frame 0: Cut Away -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-021 frame 4: Cut Away -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-022 frame 0: A property written "hue", which Blobbylize does not read. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-022 frame 4: A property written "hue", which Blobbylize does not read. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-023 frame 0: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-023 frame 4: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-024 frame 0: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-024 frame 4: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-025 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-025 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-026 frame 0: A light position 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-026 frame 4: A light position 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOB-027 frame 0: Metal keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-027 frame 4: Metal keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOB-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blob_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blob_008.json is saved with its layer, fit, property, softness, light and surface | {"ambient":25,"cut_away":0,"diffuse":75,"fit":"stretch","layer":"","light_color":"#ffffff","light_direction":-45,"light_height":10,"light_intensity":100,"light_position":[25,30],"light_type":"point","metal":100,"property":"alpha","roughness":0.05,"softness":4,"specular":50} | yes |
| fx_blob_012.json is saved with its blob layer, dots | {"ambient":25,"cut_away":0,"diffuse":75,"fit":"stretch","layer":"dots","light_color":"#ffffff","light_direction":-45,"light_height":100,"light_intensity":100,"light_position":[30,30],"light_type":"distant","metal":100,"property":"alpha","roughness":0.05,"softness":2,"specular":50} | yes |
| fx_blob_020.json is refused in a sentence | Blobbylize's softness runs from 0 to 100, and this is 101. | yes |
| fx_blob_021.json is refused in a sentence | Blobbylize's cut away runs from 0 to 100, and this is -1. | yes |
| fx_blob_022.json is refused in a sentence | Blobbylize's property is red, green, blue, alpha, luminance or lightness, and this is "hue". | yes |
| fx_blob_023.json is refused in a sentence | Blobbylize's light type is distant or point, and this is "spot". | yes |
| fx_blob_024.json is refused in a sentence | Blobbylize's roughness runs from 0.001 to 1, and this is 0. | yes |
| fx_blob_025.json is refused in a sentence | Blobbylize's blob layer is the name of a layer of this composition, and this is 3. | yes |
| fx_blob_026.json is refused in a sentence | Blobbylize's light position runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Blobbylize with no `light_color` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Blobbylize whose light position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| softness 100.5 is refused with a sentence, and nothing changes | Blobbylize's softness runs from 0 to 100, and this is 100.5. | yes |
| roughness 0 is refused with a sentence, and nothing changes | Blobbylize's roughness runs from 0.001 to 1, and this is 0. | yes |
| light intensity 400.5 is refused with a sentence, and nothing changes | Blobbylize's light intensity runs from 0 to 400, and this is 400.5. | yes |
| property "saturation" is refused with a sentence, and nothing changes | Blobbylize's property is red, green, blue, alpha, luminance or lightness, and this is "saturation". | yes |
| light colour "#fff" is refused with a sentence, and nothing changes | Blobbylize's light colour is written #rrggbb, and this is "#fff". | yes |
| fit "fill" is refused with a sentence, and nothing changes | Blobbylize's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| metal keyed to 150 is refused with a sentence, and nothing changes | Blobbylize's metal runs from 0 to 100, and this is 150. | yes |
| the dots as the blob by luminance, a warm point light is taken | taken | yes |
| softness keyed from 0 to 8 is taken | taken | yes |
| light position keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blob_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blob_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blob_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_blob_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_blob_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073589 pixels changed | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10) on three layers, frame 0, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10) on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10) on three layers, frame 239, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10) on three layers, frame 0, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10) on three layers, frame 100, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize as it starts (its own alpha, softness 10) on three layers, frame 239, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light on three layers, frame 0, Full | largest difference 1 of 255, 2147 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light on three layers, frame 100, Full | largest difference 1 of 255, 2337 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light on three layers, frame 239, Full | largest difference 1 of 255, 2164 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light on three layers, frame 0, Draft | largest difference 1 of 255, 112 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light on three layers, frame 100, Draft | largest difference 1 of 255, 127 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize luminance, softness 6, cut 20, a warm point light on three layers, frame 239, Draft | largest difference 1 of 255, 122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob on three layers, frame 0, Full | largest difference 1 of 255, 1717 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob on three layers, frame 100, Full | largest difference 1 of 255, 181 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob on three layers, frame 239, Full | largest difference 1 of 255, 675 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob on three layers, frame 0, Draft | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob on three layers, frame 100, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's lightness, stretched, as the blob on three layers, frame 239, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's red, tiled, rough and dull on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine on three layers, frame 0, Full | largest difference 1 of 255, 1362 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine on three layers, frame 100, Full | largest difference 1 of 255, 758 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine on three layers, frame 239, Full | largest difference 1 of 255, 1370 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine on three layers, frame 0, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine on three layers, frame 100, Draft | largest difference 1 of 255, 108 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Blobbylize layer-4's alpha, centred, a sharp shine on three layers, frame 239, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-379 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (its own alpha, softness 10): the solid street lit flat, its edges rounding off into a soft rim; draws cleanly | [], 129600 pixels changed | yes |
| 3_luminance_shiny.png, luminance, softness 6: the lit windows and markings stand up as shiny bumps; draws cleanly | [], 129600 pixels changed | yes |
| 4_luminance_cut_50.png, luminance, cut away 50, a warm point light: the dark parts melt away, the bright parts left as warm lit blobs; draws cleanly | [], 129600 pixels changed | yes |

## Result

190 of 190 checks pass.
