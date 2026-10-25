# D-611: Particle Space's textured particles

B-491 (P0-6): three more particle types, Textured Square, Textured Disc and Textured Faded Disc, that turn (Rotation Speed, Initial Rotation) and show a layer of the composition (Texture Layer, read whole at its own size), taken down through its own smaller copies as the particle shrinks. After the purpose of CycoreFX's CC Particle World textured types; CycoreFX publishes no formula and no settings when added, so the rules and those settings are ours, stated in `tools/particle_texture_reference.py`. A texture layer that is not in the composition leaves the effect out with a warning (D-189); one with no picture at the frame draws no textured particles. Every expected pixel is `Fixtures/particle_texture/expected_particle_texture.json`, written by the tool before this code existed and printed in document 25 as FX-PTEX-001 to 030. Tolerance 2e-5.

## FX-PTEX-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PTEX-001 frame 20: Textured Square with no texture: a square about 9 pixels across in the Color Map's colour, a fifth of the way from yellow to red, unlit. | largest difference 3.0e-8 | yes |
| FX-PTEX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-002 frame 20: Textured Disc with no texture, D-610's size: D-610's Disc (FX-PSPACE-002). | largest difference 2.9e-8 | yes |
| FX-PTEX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-003 frame 20: Textured Faded Disc with no texture, D-610's size: D-610's Faded Sphere (FX-PSPACE-003). | largest difference 2.5e-8 | yes |
| FX-PTEX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-004 frame 20: Textured Square with the dot texture: its red, green, blue and white quarters, upright, the red one at the top left with a fainter pixel. | largest difference 1.5e-7 | yes |
| FX-PTEX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-005 frame 20: Textured Disc with the dot: the same quarters cut round. | largest difference 1.5e-7 | yes |
| FX-PTEX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-006 frame 20: Textured Faded Disc with the dot: the quarters fading to nothing at the edge. | largest difference 1.3e-7 | yes |
| FX-PTEX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-007 frame 20: Initial Rotation 360: the square turned by its own random amount. | largest difference 1.8e-7 | yes |
| FX-PTEX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-008 frame 20: Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees clockwise. | largest difference 1.7e-7 | yes |
| FX-PTEX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-009 frame 20: Rotation Speed keyed from -180 at frame 0 to 180 at frame 24: read at the particle's birth (frame 9.2), so it turns anticlockwise at about 42 a second for life. | largest difference 1.7e-7 | yes |
| FX-PTEX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-010 frame 20: The 20 by 12 texture on the square: taken down between its second and third levels, so its blue checker greys. | largest difference 9.4e-8 | yes |
| FX-PTEX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-011 frame 20: A spread of turned textured squares at many depths: each read at its own level. | largest difference 1.7e-7 | yes |
| FX-PTEX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-012 frame 20: The big texture on the spread: the far ones greyer. | largest difference 1.2e-7 | yes |
| FX-PTEX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-013 frame 20: Depth Cue Fog towards blue, distance 3: the texture two thirds of the way to blue, its fainter pixel fainter blue. | largest difference 6.0e-8 | yes |
| FX-PTEX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-014 frame 20: Depth Cue Fade, distance 3: the texture at a third. | largest difference 6.1e-8 | yes |
| FX-PTEX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-015 frame 20: Two textured discs, Composite: the nearer over the farther. | largest difference 1.6e-7 | yes |
| FX-PTEX-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-016 frame 20: The two, Screen. | largest difference 1.0e-7 | yes |
| FX-PTEX-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-017 frame 20: The two, Add. | largest difference 3.2e-7 | yes |
| FX-PTEX-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-018 frame 20: The two over the drawing (Composite With Original on), Max Opacity 70. | largest difference 1.9e-7 | yes |
| FX-PTEX-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-019 frame 10: The texture layer starting at frame 15: at frame 10 it has no picture, so nothing is drawn; at frame 20 it is FX-PTEX-005. | largest difference 0.0e0 | yes |
| FX-PTEX-019 frame 20: The texture layer starting at frame 15: at frame 10 it has no picture, so nothing is drawn; at frame 20 it is FX-PTEX-005. | largest difference 1.5e-7 | yes |
| FX-PTEX-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-020 frame 20: A texture layer not in the composition, `gone`: the effect is left out, the drawing shows, and the warning every frame. | largest difference 1.9e-7 | yes |
| FX-PTEX-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PTEX-021 frame 20: FX-PTEX-004 with the layer moved 3 pixels right. | largest difference 1.5e-7 | yes |
| FX-PTEX-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-022 frame 20: The composition's camera, as added, on the spread. | largest difference 1.2e-7 | yes |
| FX-PTEX-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-023 frame 12: A burst of turning textured squares, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 8.6e-8 | yes |
| FX-PTEX-023 frame 24: A burst of turning textured squares, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 9.1e-8 | yes |
| FX-PTEX-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-024 frame 20: A tiny textured disc, size 1: drawn 1.4 pixels across and fainter, the texture taken down to its last levels. | largest difference 2.7e-8 | yes |
| FX-PTEX-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-025 frame 20: A Disc (not textured) with the texture and rotation set: D-610's Disc, the texture not used (FX-PSPACE-002). | largest difference 2.9e-8 | yes |
| FX-PTEX-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTEX-026 frame 0: Particle Type "textured_polygon": not one of the eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-026 frame 4: Particle Type "textured_polygon": not one of the eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PTEX-027 frame 0: Rotation Speed 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-027 frame 4: Rotation Speed 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PTEX-028 frame 0: Initial Rotation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-028 frame 4: Initial Rotation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PTEX-029 frame 0: Initial Rotation 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-029 frame 4: Initial Rotation 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PTEX-030 frame 0: A texture layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-030 frame 4: A texture layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PTEX-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ptex_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptex_004.json is saved with its 62 settings, the texture layer dot, particle type textured_square, and not the picture it reads | 62 settings; texture layer "dot", particle type "textured_square", texture None | yes |
| a file written before D-611, without rotation speed, initial rotation or texture layer, reads them as added and is saved without them | as added: true; saved with 59 settings | yes |
| fx_ptex_026.json is refused in a sentence naming particle type | Particle Space's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "textured_square", "textured_disc", "textured_faded_disc", and this is "textured_polygon". | yes |
| fx_ptex_027.json is refused in a sentence naming rotation speed | Particle Space's rotation speed runs from -3600 to 3600, and this is 3601. | yes |
| fx_ptex_028.json is refused in a sentence naming initial rotation | Particle Space's initial rotation runs from 0 to 360, and this is -1. | yes |
| fx_ptex_029.json is refused in a sentence naming initial rotation | Particle Space's initial rotation runs from 0 to 360, and this is 361. | yes |
| fx_ptex_030.json is refused in a sentence naming texture layer | Particle Space's texture layer is the name of a layer of this composition, and this is 3. | yes |
| a file with a Particle Space whose rotation speed is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ptex_011.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ptex_012.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ptex_023.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-611)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ptex_001.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_002.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_004.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_010.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_011.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_012.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_014.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_016.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_017.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_018.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_019.json, frames [10, 20] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_020.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_021.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_022.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_023.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_024.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_025.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_028.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_029.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ptex_030.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Space textured squares, no texture layer: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Space textured squares, no texture layer on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured squares, no texture layer on three layers, frame 100, Full | largest difference 1 of 255, 19 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured squares, no texture layer on three layers, frame 239, Full | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured squares, no texture layer on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured squares, no texture layer on three layers, frame 100, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured squares, no texture layer on three layers, frame 239, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 8503 pixels changed | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing on three layers, frame 100, Full | largest difference 1 of 255, 1408 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing on three layers, frame 239, Full | largest difference 1 of 255, 1747 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured discs showing the background layer-4, turning, over the drawing on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360 on three layers, frame 100, Full | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360 on three layers, frame 239, Full | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360 on three layers, frame 100, Draft | largest difference 1 of 255, 11 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360 on three layers, frame 239, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-611 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_squares_no_texture.png, frame 48: drawn without a warning, the street changed | 3770 pixels changed; warnings [] | yes |
| 2_street_squares_turning.png, frame 48: drawn without a warning, the street changed | 38867 pixels changed; warnings [] | yes |
| 3_street_discs_fountain.png, frame 48: drawn without a warning, the street changed | 13972 pixels changed; warnings [] | yes |
| 4_street_faded_discs_fog.png, frame 72: drawn without a warning, the street changed | 34984 pixels changed; warnings [] | yes |
| the street with Particle Space's texture layer `gone`, not in the composition: the effect left out, the street unchanged, the warning said | 0 pixels changed; warnings ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."] | yes |

## Result

164 of 164 checks pass.
