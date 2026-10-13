# D-710: Particle Plane's textured particles and Composite With Original

B-580 (P0-6): Particle Space's Textured Disc and Textured Faded Disc (D-611) on the flat emitter, with Texture Layer, Rotation Speed and Initial Rotation, and Composite With Original, which draws the particles over the layer instead of on an empty one. After the purpose of CycoreFX's CC Particle Systems II; CycoreFX publishes no formula and no settings when added, so the rules are ours, stated in `tools/particle_plane_texture_reference.py`. Textured Square stays refused on the Plane (FX-PPLANE-055) until the owner decides on that fixture. A texture layer that is not in the composition leaves the effect out with a warning (D-189). Every expected pixel is `Fixtures/particle_plane_texture/expected_particle_plane_texture.json`, written by the tool before this code existed, FX-PPTEX-001 to 029. Tolerance 2e-5.

## FX-PPTEX-001 to 029

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PPTEX-001 frame 20: Composite With Original on, one disc: FX-PPLANE-002's dot over the drawing instead of over a clear layer. | largest difference 1.9e-7 | yes |
| FX-PPTEX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-002 frame 12: Composite With Original on, the falling burst: FX-PPLANE-025 over the drawing, at frames 12 and 24. | largest difference 1.9e-7 | yes |
| FX-PPTEX-002 frame 24: Composite With Original on, the falling burst: FX-PPLANE-025 over the drawing, at frames 12 and 24. | largest difference 1.3e-7 | yes |
| FX-PPTEX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-003 frame 20: Composite With Original on, the two discs Screened over the drawing. | largest difference 1.3e-7 | yes |
| FX-PPTEX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-004 frame 10: Composite With Original on, the layer starting at frame 10: at its first frame no particle yet, so the drawing as it is; frame 22 is FX-PPTEX-002's frame 12. | largest difference 1.9e-7 | yes |
| FX-PPTEX-004 frame 22: Composite With Original on, the layer starting at frame 10: at its first frame no particle yet, so the drawing as it is; frame 22 is FX-PPTEX-002's frame 12. | largest difference 1.9e-7 | yes |
| FX-PPTEX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-005 frame 20: Textured Disc with no texture: FX-PPLANE-002's disc. | largest difference 2.5e-8 | yes |
| FX-PPTEX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-006 frame 20: Textured Faded Disc with no texture: FX-PPLANE-003's Faded Sphere. | largest difference 2.7e-8 | yes |
| FX-PPTEX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-007 frame 20: Textured Disc with the dot texture: its red, green, blue and white quarters cut round, upright, the red at the top left. | largest difference 1.6e-7 | yes |
| FX-PPTEX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-008 frame 20: Textured Faded Disc with the dot: the quarters fading to nothing at the edge. | largest difference 9.7e-8 | yes |
| FX-PPTEX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-009 frame 20: Initial Rotation 360: the disc turned by its own random amount. | largest difference 1.6e-7 | yes |
| FX-PPTEX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-010 frame 20: Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees clockwise. | largest difference 1.7e-7 | yes |
| FX-PPTEX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-011 frame 20: Rotation Speed keyed from -180 at frame 0 to 180 at frame 24: read at the particle's birth (frame 9.2), so it turns anticlockwise for life. | largest difference 1.7e-7 | yes |
| FX-PPTEX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-012 frame 20: The 20 by 12 texture on the disc: taken down a level, its blue checker greying. | largest difference 1.1e-7 | yes |
| FX-PPTEX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-013 frame 20: Two textured discs, Composite: the newer over the older. | largest difference 1.7e-7 | yes |
| FX-PPTEX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-014 frame 20: The two, Add. | largest difference 3.3e-7 | yes |
| FX-PPTEX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-015 frame 20: The two over the drawing (Composite With Original on), Max Opacity 70. | largest difference 1.5e-7 | yes |
| FX-PPTEX-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-016 frame 20: The textured disc faded out over its life (Opacity Map Fade Out): four fifths covering. | largest difference 1.4e-7 | yes |
| FX-PPTEX-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-017 frame 10: The texture layer starting at frame 15: at frame 10 it has no picture, so nothing is drawn; at frame 20 it is FX-PPTEX-007. | largest difference 0.0e0 | yes |
| FX-PPTEX-017 frame 20: The texture layer starting at frame 15: at frame 10 it has no picture, so nothing is drawn; at frame 20 it is FX-PPTEX-007. | largest difference 1.6e-7 | yes |
| FX-PPTEX-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-018 frame 20: A texture layer not in the composition, `gone`: the effect is left out, the drawing shows, and the warning every frame. | largest difference 1.9e-7 | yes |
| FX-PPTEX-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PPTEX-019 frame 20: FX-PPTEX-007 with the layer moved 3 pixels right. | largest difference 1.6e-7 | yes |
| FX-PPTEX-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-020 frame 12: A burst of turning textured faded discs, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 1.0e-7 | yes |
| FX-PPTEX-020 frame 24: A burst of turning textured faded discs, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 1.3e-7 | yes |
| FX-PPTEX-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-021 frame 20: A tiny textured disc, size 3: drawn 1.4 pixels across and fainter, the texture taken down to its last level. | largest difference 7.4e-9 | yes |
| FX-PPTEX-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-022 frame 20: A Disc (not textured) with the texture and rotation set: FX-PPLANE-002, the texture not used. | largest difference 2.5e-8 | yes |
| FX-PPTEX-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-023 frame 20: A star over the drawing (Composite With Original on), the texture set and not used. | largest difference 1.9e-7 | yes |
| FX-PPTEX-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPTEX-024 frame 0: Particle Type "textured_square": still not one of the plane's (waits on the owner, as FX-PPLANE-055). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-024 frame 4: Particle Type "textured_square": still not one of the plane's (waits on the owner, as FX-PPLANE-055). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPTEX-025 frame 0: Rotation Speed 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-025 frame 4: Rotation Speed 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPTEX-026 frame 0: Initial Rotation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-026 frame 4: Initial Rotation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPTEX-027 frame 0: Initial Rotation 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-027 frame 4: Initial Rotation 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPTEX-028 frame 0: Composite With Original "yes": the word is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-028 frame 4: Composite With Original "yes": the word is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPTEX-029 frame 0: A texture layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-029 frame 4: A texture layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPTEX-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pptex_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pptex_008.json is saved with its 39 settings, the texture layer dot, particle type textured_faded_disc, and not the picture it reads | 39 settings; texture layer "dot", particle type "textured_faded_disc", texture None | yes |
| a file written before D-710, without rotation speed, initial rotation, texture layer or composite with original, reads them as added and is saved without them | as added: true; saved with 35 settings | yes |
| fx_pptex_024.json is refused in a sentence naming particle type | Particle Plane's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "star", "textured_disc", "textured_faded_disc", and this is "textured_square". | yes |
| fx_pptex_025.json is refused in a sentence naming rotation speed | Particle Plane's rotation speed runs from -3600 to 3600, and this is 3601. | yes |
| fx_pptex_026.json is refused in a sentence naming initial rotation | Particle Plane's initial rotation runs from 0 to 360, and this is -1. | yes |
| fx_pptex_027.json is refused in a sentence naming initial rotation | Particle Plane's initial rotation runs from 0 to 360, and this is 361. | yes |
| fx_pptex_028.json is refused in a sentence naming composite on original | Particle Plane's composite on original is one of "off", "on", and this is "yes". | yes |
| fx_pptex_029.json is refused in a sentence naming texture layer | Particle Plane's texture layer is the name of a layer of this composition, and this is 3. | yes |
| a file with a Particle Plane whose rotation speed is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pptex_008.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pptex_012.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pptex_019.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-710)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pptex_001.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_002.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_004.json, frames [10, 22] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_010.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_011.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_012.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_014.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_016.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_017.json, frames [10, 20] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_018.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_019.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_020.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_021.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_022.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_023.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_024.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_025.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_028.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pptex_029.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 16827 pixels changed | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing on three layers, frame 100, Full | largest difference 1 of 255, 1404 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing on three layers, frame 239, Full | largest difference 1 of 255, 1783 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs, no texture layer, over the drawing on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 13366 pixels changed | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing on three layers, frame 100, Full | largest difference 1 of 255, 1415 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing on three layers, frame 239, Full | largest difference 1 of 255, 1760 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured discs showing the background layer-4, turning, over the drawing on three layers, frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360 on three layers, frame 100, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360 on three layers, frame 239, Full | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360 on three layers, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360 on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-710 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_over_the_street.png, frame 24: drawn without a warning, the street changed | 3802 pixels changed; warnings [] | yes |
| 2_street_discs_fountain.png, frame 48: drawn without a warning, the street changed | 28040 pixels changed; warnings [] | yes |
| 3_street_faded_discs_vortex.png, frame 72: drawn without a warning, the street changed | 46169 pixels changed; warnings [] | yes |
| 4_street_discs_on_empty.png, frame 48: drawn without a warning, the street changed | 129596 pixels changed; warnings [] | yes |
| the street with Particle Plane's texture layer `gone`, not in the composition: the effect left out, the street unchanged, the warning said | 0 pixels changed; warnings ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."] | yes |

## Result

164 of 164 checks pass.
