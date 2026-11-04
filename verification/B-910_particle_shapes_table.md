# D-1031: Particle Space's Tri, Quad and Motion Polygon

B-910 (wave 12, gap 8): three more particle types, after the purpose of CC Particle World's flat shapes: Quad (D-611's Textured Square with no texture), Tri (an equilateral triangle of side 2R, turned by Initial Rotation and Rotation Speed) and Motion Polygon (a triangle from the particle back to where a Line's tail is drawn, 2R wide at its base). Never lit, never textured. The rules are ours, stated in `tools/particle_shapes_reference.py`. The card hands the three back to the processor, so every frame below is drawn there. Every expected pixel is `Fixtures/particle_shapes/expected_particle_shapes.json`, written by the tool before this code existed and printed in document 25 as FX-PSHAPE-001 to 018. Tolerance 2e-5.

## FX-PSHAPE-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PSHAPE-001 frame 20: Quad: a square about 9 pixels across, upright, in the Color Map's colour, unlit: FX-PTEX-001's Textured Square with no texture. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-002 frame 20: Quad with the dot texture named: the texture is not used, FX-PSHAPE-001. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-003 frame 20: Tri: a triangle about 9 pixels a side, its point straight up. | largest difference 2.8e-8 | yes |
| FX-PSHAPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-004 frame 20: Tri, Initial Rotation 360: turned by its own random amount. | largest difference 2.9e-8 | yes |
| FX-PSHAPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-005 frame 20: Tri, Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees clockwise. | largest difference 2.9e-8 | yes |
| FX-PSHAPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-006 frame 20: Tri, Rotation Speed keyed -180 at frame 0 to 180 at frame 24: read at its birth, so it turns anticlockwise. | largest difference 2.9e-8 | yes |
| FX-PSHAPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-007 frame 20: Motion Polygon, one drifting particle: a thin triangle, its point where the particle is and its base trailing behind. | largest difference 2.6e-8 | yes |
| FX-PSHAPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-008 frame 20: Motion Polygon, a particle that does not move: nothing drawn. | largest difference 0.0e0 | yes |
| FX-PSHAPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-009 frame 12: A burst of Motion Polygons at frames 12 and 24: each a streak pointing outwards, as a Line's, but wide at its tail. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-009 frame 24: A burst of Motion Polygons at frames 12 and 24: each a streak pointing outwards, as a Line's, but wide at its tail. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-010 frame 12: A burst of turning Quads, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-010 frame 24: A burst of turning Quads, Rotation Speed 180 and Initial Rotation 360, at frames 12 and 24. | largest difference 2.9e-8 | yes |
| FX-PSHAPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-011 frame 20: A spread of Tris at many depths, each its own turn. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-012 frame 20: Two Tris, Screen. | largest difference 2.8e-8 | yes |
| FX-PSHAPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-013 frame 12: The Motion Polygon burst with Depth Cue Fog towards blue, distance 3. | largest difference 3.0e-8 | yes |
| FX-PSHAPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-014 frame 20: FX-PSHAPE-003 with the layer moved 3 pixels right. | largest difference 2.8e-8 | yes |
| FX-PSHAPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-015 frame 20: A tiny Tri, size 1: drawn as one 0.7 radius, fainter. | largest difference 4.2e-9 | yes |
| FX-PSHAPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-016 frame 20: Tris over the drawing (Composite With Original on), Max Opacity 70. | largest difference 1.9e-7 | yes |
| FX-PSHAPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSHAPE-017 frame 0: Particle Type "cube": not built (D-1032, PROPOSED), so not one of the eleven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSHAPE-017 frame 4: Particle Type "cube": not built (D-1032, PROPOSED), so not one of the eleven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSHAPE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSHAPE-018 frame 0: Particle Type "triangle": the word is "tri". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSHAPE-018 frame 4: Particle Type "triangle": the word is "tri". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSHAPE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pshape_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pshape_007.json is saved with the particle type motion_polygon | particle type "motion_polygon" | yes |
| fx_pshape_017.json is refused in a sentence naming particle type | Particle Space's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "textured_square", "textured_disc", "textured_faded_disc", "tri", "quad", "motion_polygon", and this is "cube". | yes |
| fx_pshape_018.json is refused in a sentence naming particle type | Particle Space's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "textured_square", "textured_disc", "textured_faded_disc", "tri", "quad", "motion_polygon", and this is "triangle". | yes |

## The card hands them back (D-1031)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pshape_001.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_002.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_003.json, frames [20] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_004.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_005.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_006.json, frames [20] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_007.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_008.json, frames [20] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_009.json, frames [12, 24] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_010.json, frames [12, 24] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_011.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_012.json, frames [20] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_013.json, frames [12] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_014.json, frames [20] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_015.json, frames [20] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_016.json, frames [20] at Full and Draft, through the card's path | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_017.json, frames [0, 4] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pshape_018.json, frames [0, 4] at Full and Draft, through the card's path | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Pictures (verification/B-910 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_tris_turning.png, frame 48: drawn without a warning, the street changed | 11342 pixels changed; warnings [] | yes |
| 2_quads_confetti.png, frame 48: drawn without a warning, the street changed | 16295 pixels changed; warnings [] | yes |
| 3_motion_polygon_sparks.png, frame 48: drawn without a warning, the street changed | 63147 pixels changed; warnings [] | yes |

## Result

82 of 82 checks pass.
