# D-714: Particle Plane's Textured Square

B-584 (P0-6): Particle Space's Textured Square (D-611) on the flat emitter, turned and textured as D-710's textured discs, by the owner's decision of 2026-10-13 (FX-PPLANE-055 and FX-PPTEX-024 now refuse "cube"). After the purpose of CycoreFX's CC Particle Systems II; CycoreFX publishes no formula, so the rule is ours, stated in `tools/particle_plane_square_reference.py`. Every expected pixel is `Fixtures/particle_plane_square/expected_particle_plane_square.json`, written by the tool before this code existed, FX-PPSQ-001 to 018. Tolerance 2e-5.

## FX-PPSQ-001 to 018

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PPSQ-001 frame 20: Textured Square with no texture: a square in the Color Map's colour, where FX-PPLANE-002 draws its disc. | largest difference 1.4e-8 | yes |
| FX-PPSQ-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-002 frame 20: Textured Square with the dot texture: its red, green, blue and white quarters, upright, the red at the top left. | largest difference 1.5e-7 | yes |
| FX-PPSQ-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-003 frame 20: Initial Rotation 360: the square turned by its own random amount. | largest difference 1.5e-7 | yes |
| FX-PPSQ-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-004 frame 20: Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees clockwise. | largest difference 1.7e-7 | yes |
| FX-PPSQ-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-005 frame 20: Rotation Speed keyed from -180 at frame 0 to 180 at frame 24: read at the particle's birth, so it turns anticlockwise for life. | largest difference 1.7e-7 | yes |
| FX-PPSQ-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-006 frame 20: The 20 by 12 texture on the square: taken down a level, its blue checker greying. | largest difference 1.1e-7 | yes |
| FX-PPSQ-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-007 frame 20: Two textured squares, Composite: the newer over the older. | largest difference 1.6e-7 | yes |
| FX-PPSQ-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-008 frame 20: The two, Add. | largest difference 3.3e-7 | yes |
| FX-PPSQ-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-009 frame 20: The two, Screen, over the drawing (Composite With Original on), Max Opacity 70. | largest difference 1.8e-7 | yes |
| FX-PPSQ-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-010 frame 20: The textured square faded out over its life (Opacity Map Fade Out). | largest difference 1.4e-7 | yes |
| FX-PPSQ-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-011 frame 10: The texture layer starting at frame 15: at frame 10 nothing is drawn; at frame 20 it is FX-PPSQ-002. | largest difference 0.0e0 | yes |
| FX-PPSQ-011 frame 20: The texture layer starting at frame 15: at frame 10 nothing is drawn; at frame 20 it is FX-PPSQ-002. | largest difference 1.5e-7 | yes |
| FX-PPSQ-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-012 frame 20: A texture layer not in the composition, `gone`: the effect is left out, the drawing shows, and the warning every frame. | largest difference 1.9e-7 | yes |
| FX-PPSQ-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PPSQ-013 frame 20: FX-PPSQ-002 with the layer moved 3 pixels right. | largest difference 1.5e-7 | yes |
| FX-PPSQ-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-014 frame 12: A burst of turning textured squares, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 1.7e-7 | yes |
| FX-PPSQ-014 frame 24: A burst of turning textured squares, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 1.7e-7 | yes |
| FX-PPSQ-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-015 frame 20: A tiny textured square, size 3: 1.4 pixels across and fainter, the texture taken down to its last level. | largest difference 7.3e-9 | yes |
| FX-PPSQ-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-016 frame 20: A large square, size 60, turned by its own random amount (Initial Rotation 360): its corners reach past the disc's radius. | largest difference 2.3e-8 | yes |
| FX-PPSQ-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPSQ-017 frame 0: Particle Type "Textured Square": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPSQ-017 frame 4: Particle Type "Textured Square": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPSQ-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPSQ-018 frame 0: Particle Type "textured_polygon": not one of the plane's kinds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPSQ-018 frame 4: Particle Type "textured_polygon": not one of the plane's kinds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPSQ-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ppsq_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppsq_002.json is saved with its 39 settings, the texture layer dot and particle type textured_square | 39 settings; texture layer "dot", particle type "textured_square" | yes |
| fx_ppsq_017.json is refused in a sentence naming particle type and textured_square among its words | Particle Plane's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "star", "textured_disc", "textured_faded_disc", "textured_square", and this is "Textured Square". | yes |
| fx_ppsq_018.json is refused in a sentence naming particle type and textured_square among its words | Particle Plane's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "star", "textured_disc", "textured_faded_disc", "textured_square", and this is "textured_polygon". | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ppsq_002.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ppsq_013.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ppsq_016.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-714)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ppsq_001.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_002.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_004.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_010.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_011.json, frames [10, 20] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_012.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_014.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_016.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_017.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_ppsq_018.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 21370 pixels changed | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing on three layers, frame 100, Full | largest difference 1 of 255, 1396 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing on three layers, frame 239, Full | largest difference 1 of 255, 1784 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares, no texture layer, turning, over the drawing on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add on three layers, frame 100, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add on three layers, frame 239, Full | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add on three layers, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane textured squares showing the background layer-4, turning, Add on three layers, frame 239, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-714 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_street_squares_fountain_over_the_street.png, frame 48: drawn without a warning, the street changed | 30848 pixels changed; warnings [] | yes |
| 2_street_squares_upright_on_empty.png, frame 48: drawn without a warning, the street changed | 129582 pixels changed; warnings [] | yes |

## Result

98 of 98 checks pass.
