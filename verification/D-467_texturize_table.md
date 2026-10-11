# D-467: Texturize

B-347: `core.texturize` takes After Effects' Texturize controls (Texture Layer, Light Direction, Texture Contrast, Texture Placement). Every expected pixel is `Fixtures/texturize/expected_texturize.json`, written by `tools/texturize_reference.py` before this code existed and printed in document 25 as FX-TEXTURIZE-001 to 020. Tolerance 2e-5.

## FX-TEXTURIZE-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TEXTURIZE-001 frame 0: As added: no Texture Layer, so nothing happens and the photo comes back untouched. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-002 frame 0: The dots layer as the texture, Light Direction 135, Texture Contrast 1, Tile: the two white squares pressed into the photo, their top and left edges lighter, their bottom and right edges darker, the rest as it was. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-003 frame 0: Light Direction -45: turned round, the top and left edges darker. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-004 frame 0: Texture Contrast 0: nothing happens. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-005 frame 0: Texture Contrast 2: twice as strong, the lightest edges clipped to white. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-006 frame 0: The ramp, black at the left to white at the right, as the texture, Light Direction 90: one even slope, every column but the two at the sides 17 levels of 255 lighter, those two 8.5. | largest difference 2.3e-7 | yes |
| FX-TEXTURIZE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-007 frame 0: The 8 by 5 spot as the texture, Tile: four red discs pressed in, two across and two down. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-008 frame 0: The spot, Center: one disc in the middle, the photo untouched round it. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-009 frame 0: The spot, Stretch: one oval filling the middle. | largest difference 2.2e-7 | yes |
| FX-TEXTURIZE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-010 frame 0: The dots on the two blocks with clear round them: the blocks pressed where the squares cross them, every clear pixel still clear and the covering kept exactly. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-011 frame 0: FX-TEXTURIZE-002 on the holder moved 2 right and 1 down: the same, moved, the texture lying on the drawing. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-012 frame 0: A Texture Layer that is not in the composition, `gone`: nothing happens, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-012 frame 4: A Texture Layer that is not in the composition, `gone`: nothing happens, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-TEXTURIZE-013 frame 0: Light Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 2 is Light Direction 90. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-013 frame 2: Light Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 2 is Light Direction 90. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-013 frame 4: Light Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 2 is Light Direction 90. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-014 frame 0: Texture Contrast eased from 1 at frame 0 to 2 at frame 4 on a curve that overshoots: at frame 2 it would pass 2, is held at 2, and the frame is FX-TEXTURIZE-005. | largest difference 1.6e-7 | yes |
| FX-TEXTURIZE-014 frame 2: Texture Contrast eased from 1 at frame 0 to 2 at frame 4 on a curve that overshoots: at frame 2 it would pass 2, is held at 2, and the frame is FX-TEXTURIZE-005. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-014 frame 4: Texture Contrast eased from 1 at frame 0 to 2 at frame 4 on a curve that overshoots: at frame 2 it would pass 2, is held at 2, and the frame is FX-TEXTURIZE-005. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEXTURIZE-015 frame 0: Texture Contrast 2.5, above 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-015 frame 4: Texture Contrast 2.5, above 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEXTURIZE-016 frame 0: Texture Contrast -0.5, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-016 frame 4: Texture Contrast -0.5, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEXTURIZE-017 frame 0: A placement written "mirror". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-017 frame 4: A placement written "mirror". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEXTURIZE-018 frame 0: A Texture Layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-018 frame 4: A Texture Layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEXTURIZE-019 frame 0: Light Direction 4000, past 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-019 frame 4: Light Direction 4000, past 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEXTURIZE-020 frame 0: Texture Contrast keyed to 3 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-020 frame 4: Texture Contrast keyed to 3 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TEXTURIZE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_texturize_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_texturize_009.json is saved with its four settings | {"fit":"stretch","layer":"spot","light_direction":135,"texture_contrast":1} | yes |
| fx_texturize_015.json is refused in a sentence naming Texturize and its texture contrast | Texturize's texture contrast runs from 0 to 2, and this is 2.5. | yes |
| fx_texturize_016.json is refused in a sentence naming Texturize and its texture contrast | Texturize's texture contrast runs from 0 to 2, and this is -0.5. | yes |
| fx_texturize_017.json is refused in a sentence naming Texturize and its texture placement | Texturize's texture placement is "center", "stretch" or "tile", and this is "mirror". | yes |
| fx_texturize_018.json is refused in a sentence naming Texturize and its texture layer | Texturize's texture layer is the name of a layer of this composition, and this is 3. | yes |
| fx_texturize_019.json is refused in a sentence naming Texturize and its light direction | Texturize's light direction runs from -3600 to 3600, and this is 4000. | yes |
| a file with a Texturize whose contrast is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Texturize without its light direction is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Texturize whose placement is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| texture contrast 2.5 is refused with a sentence, and nothing changes | Texturize's texture contrast runs from 0 to 2, and this is 2.5. | yes |
| texture placement mirror is refused with a sentence, and nothing changes | Texturize's texture placement is "center", "stretch" or "tile", and this is "mirror". | yes |
| texture layer the number 3 is refused with a sentence, and nothing changes | Texturize's texture layer is the name of a layer of this composition, and this is 3. | yes |
| light direction keyed to 4000 is refused with a sentence, and nothing changes | Texturize's light direction runs from -3600 to 3600, and this is 4000. | yes |
| the ramp centred, Light Direction 30, Texture Contrast 0.5 is taken | taken | yes |
| texture contrast keyed from 0 to 2 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_texturize_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_texturize_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_texturize_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_texturize_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_texturize_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_texturize_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_texturize_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_texturize_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Texturize with layer4 tiled as the texture: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 3204 pixels changed | yes |
| the reference shot, Texturize with layer4 tiled as the texture on three layers, frame 0, Full | largest difference 1 of 255, 3724 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 tiled as the texture on three layers, frame 100, Full | largest difference 1 of 255, 1429 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 tiled as the texture on three layers, frame 239, Full | largest difference 1 of 255, 1750 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 tiled as the texture on three layers, frame 0, Draft | largest difference 1 of 255, 122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 tiled as the texture on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 tiled as the texture on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 3204 pixels changed | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2 on three layers, frame 0, Full | largest difference 1 of 255, 3717 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2 on three layers, frame 100, Full | largest difference 1 of 255, 1405 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2 on three layers, frame 239, Full | largest difference 1 of 255, 1747 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2 on three layers, frame 0, Draft | largest difference 1 of 255, 122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2 on three layers, frame 100, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 stretched, Light Direction 45, Texture Contrast 2 on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 3204 pixels changed | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4 on three layers, frame 0, Full | largest difference 1 of 255, 3726 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4 on three layers, frame 100, Full | largest difference 1 of 255, 1411 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4 on three layers, frame 239, Full | largest difference 1 of 255, 1754 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4 on three layers, frame 0, Draft | largest difference 1 of 255, 123 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4 on three layers, frame 100, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Texturize with layer4 centred, Light Direction -100, Texture Contrast 0.4 on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-467 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, Texturize as added, no Texture Layer picked: the street exactly as it was; draws cleanly | [], the same as 1_before.png true | yes |
| 3_bricks.png, the brick picture as the texture, tiled, the light from the top left: a brick wall pressed into the whole street, the street's colours still there, the covering exactly as before; draws cleanly | [], changed true, coverings the same true | yes |
| 4_light_from_bottom_right.png, Light Direction -45: the light from the bottom right, so the mortar edges that 3_bricks.png lightened are darkened and the other way round, at more pixels than keep their way; draws cleanly | [], turned over at 70300 pixels, kept at 0 | yes |
| 5_contrast_2.png, Texture Contrast 2: the same bricks pressed in harder, further from the street than 3_bricks.png; draws cleanly | [], distance from the street 18092802 against 10437856 | yes |
| 6_one_brick_centred.png, Center Texture: one brick pressed into the middle, the corners and the rest of the street untouched; draws cleanly | [], the middle changed true, the corners and a quarter point untouched true | yes |

## Result

140 of 140 checks pass.
