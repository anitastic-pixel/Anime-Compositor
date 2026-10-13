# D-712: Particle Grid's points from a layer's shape, and strings

B-582 (EFFECTS.md row 368), after what Trapcode Form's layer-shaped and string base forms are for. Base Form gains Box Strings, each row of the box grid's points drawn as a line through them (the Emitter's streak, at each start point's size, opacity and colour), and Layer, the box grid's points kept only where a fourth layer setting, the Base Layer, is half opaque or more at the point's spot: any layer of the composition, a drawing, a shape or a text layer, stretched to the drawing. With no Base Layer named, Layer draws as Box Grid. A file written before D-712 has no Base Layer and reads as none; it is saved only when named. Red Giant publishes no formula, so the rules are ours, stated in `tools/particle_grid_shape_reference.py`. Every expected pixel is `Fixtures/particle_grid_shape/expected_particle_grid_shape.json` and every expected point `Fixtures/particle_grid_shape/particles_particle_grid_shape.json`, written by the tool before this code existed, FX-PGSHP-001 to 027. Tolerances 2e-5 and 1e-7.

## FX-PGSHP-001 to 027

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PGSHP-001 frame 0: Base Form Layer with no Base Layer named: drawn as Box Grid, FX-PGRID-003's fine grid. | largest difference 1.2e-8 | yes |
| FX-PGSHP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-002 frame 0: Layer, the ring: a dot at each pixel centre on the ring, none inside or outside it. | largest difference 1.3e-8 | yes |
| FX-PGSHP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-003 frame 0: Layer, the block letters TA: dots in the letters' shape. | largest difference 1.5e-8 | yes |
| FX-PGSHP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-004 frame 0: Layer, half clear: only the left half's dots, white. | largest difference 5.8e-9 | yes |
| FX-PGSHP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-005 frame 0: Layer, opacity rising left to right, on a grid between pixel centres: the dots where it is half or more. | largest difference 1.2e-8 | yes |
| FX-PGSHP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-006 frame 0: The ring coloured by the ramp: Color Map and Base Layer together. | largest difference 2.5e-8 | yes |
| FX-PGSHP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-007 frame 0: The letters turned 40 degrees about Y and dispersed: the shape moves with the form. | largest difference 2.9e-8 | yes |
| FX-PGSHP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-008 frame 0: The ring rippling and drifting, Flow X 12, at frames 0 and 12. | largest difference 2.8e-8 | yes |
| FX-PGSHP-008 frame 12: The ring rippling and drifting, Flow X 12, at frames 0 and 12. | largest difference 3.0e-8 | yes |
| FX-PGSHP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-009 frame 0: The ring two deep, 10 pixels front to back: the far ring smaller. | largest difference 2.9e-8 | yes |
| FX-PGSHP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-010 frame 0: The letters as glowing spheres, added. | largest difference 0.0e0 | yes |
| FX-PGSHP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-011 frame 0: The ring at Opacity Random 60, Random Seed 3: the same points, some fainter. | largest difference 3.0e-8 | yes |
| FX-PGSHP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-012 frame 0: Sphere Layered ignores the Base Layer: FX-PGRID-021's ball. | largest difference 2.9e-8 | yes |
| FX-PGSHP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-013 frame 0: Box Strings: 3 strings, each a line through its 6 points. | largest difference 8.2e-9 | yes |
| FX-PGSHP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-014 frame 0: Box Strings rippling, Flow X 12, at frames 0 and 12: wavy lines. | largest difference 2.9e-8 | yes |
| FX-PGSHP-014 frame 12: Box Strings rippling, Flow X 12, at frames 0 and 12: wavy lines. | largest difference 2.9e-8 | yes |
| FX-PGSHP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-015 frame 0: Box Strings twisted 120 degrees, 2 deep and 12 pixels front to back. | largest difference 3.0e-8 | yes |
| FX-PGSHP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-016 frame 0: One point a string: nothing drawn. | largest difference 0.0e0 | yes |
| FX-PGSHP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-017 frame 0: Box Strings coloured by the ramp. | largest difference 2.8e-8 | yes |
| FX-PGSHP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-018 frame 0: Box Strings at Opacity 50, Size 3, added. | largest difference 5.9e-8 | yes |
| FX-PGSHP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-019 frame 0: Box Strings turned 30 degrees about Z, through a composition camera 12 pixels in front. | largest difference 3.0e-8 | yes |
| FX-PGSHP-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-020 frame 0: Box Strings with Color Map half clear: the lines starting in the clear half are not drawn. | largest difference 8.2e-9 | yes |
| FX-PGSHP-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-021 frame 0: Box Strings as Stars: Particle Type has no say, FX-PGSHP-013's lines. | largest difference 8.2e-9 | yes |
| FX-PGSHP-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-022 frame 0: Box Strings at Size 0.2: lines one pixel wide, at full opacity. | largest difference 2.6e-8 | yes |
| FX-PGSHP-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-023 frame 12: Box Strings with Size keyed from 1 at frame 0 to 4 at frame 24, at frame 12. | largest difference 1.2e-8 | yes |
| FX-PGSHP-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGSHP-024 frame 0: Base Form "text": not a word (a text layer is named as the Base Layer instead). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-024 frame 4: Base Form "text": not a word (a text layer is named as the Base Layer instead). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGSHP-025 frame 0: Base Form "Box_Strings": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-025 frame 4: Base Form "Box_Strings": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGSHP-026 frame 0: Base Layer 5: not a layer's name. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-026 frame 4: Base Layer 5: not a layer's name. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGSHP-027 frame 0: Particle Type "streak" on Box Strings: still not a Particle Grid kind. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-027 frame 4: Particle Type "streak" on Box Strings: still not a Particle Grid kind. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGSHP-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The points kept: where each is, its size and its opacity

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PGSHP-002 frame 0: the 44 points kept, each its number, place, size and opacity | 44 kept, the same ones: true, largest difference 1.4e-17 | yes |
| FX-PGSHP-003 frame 0: the 47 points kept, each its number, place, size and opacity | 47 kept, the same ones: true, largest difference 1.4e-17 | yes |
| FX-PGSHP-005 frame 0: the 6 points kept, each its number, place, size and opacity | 6 kept, the same ones: true, largest difference 0.0e0 | yes |
| FX-PGSHP-007 frame 0: the 47 points kept, each its number, place, size and opacity | 47 kept, the same ones: true, largest difference 2.8e-17 | yes |
| FX-PGSHP-008 frame 12: the 44 points kept, each its number, place, size and opacity | 44 kept, the same ones: true, largest difference 2.8e-17 | yes |
| FX-PGSHP-009 frame 0: the 88 points kept, each its number, place, size and opacity | 88 kept, the same ones: true, largest difference 1.4e-17 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pgshp_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgshp_002.json is saved with its 37 settings, the Base Layer and Base Form among them | 37 settings; base form "layer", base layer "ring" | yes |
| fx_pgrid_049.json, written before D-712 without a Base Layer, is still saved with its 36 settings and none added | 36 settings; base layer written: false | yes |
| fx_pgrid_049.json reads with no Base Layer named | "" | yes |
| fx_pgshp_024.json is refused in a sentence naming base form | Particle Grid's base form is one of "box_grid", "sphere_layered", "box_strings", "layer", and this is "text". | yes |
| fx_pgshp_025.json is refused in a sentence naming base form | Particle Grid's base form is one of "box_grid", "sphere_layered", "box_strings", "layer", and this is "Box_Strings". | yes |
| fx_pgshp_026.json is refused in a sentence naming base layer | Particle Grid's base layer is the name of a layer of this composition, and this is 5. | yes |
| fx_pgshp_027.json is refused in a sentence naming particle type | Particle Grid's particle type is one of "sphere", "glow_sphere", "star", "cloudlet", and this is "streak". | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pgshp_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pgshp_014.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pgshp_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-712 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_points_in_the_word.png, frame 0: drawn without a warning, the street changed | 16533 pixels changed; warnings [] | yes |
| 2_the_word_rippling_and_glowing.png, frame 48: drawn without a warning, the street changed | 15962 pixels changed; warnings [] | yes |
| 3_strings_rippling.png, frame 48: drawn without a warning, the street changed | 39910 pixels changed; warnings [] | yes |
| 4_strings_coloured_by_the_street.png, frame 0: drawn without a warning, the street changed | 57217 pixels changed; warnings [] | yes |

## On the card (D-618)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PGSHP-001, fx_pgshp_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-002, fx_pgshp_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-003, fx_pgshp_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-004, fx_pgshp_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-005, fx_pgshp_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-006, fx_pgshp_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-007, fx_pgshp_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-008, fx_pgshp_008.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-009, fx_pgshp_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-010, fx_pgshp_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-011, fx_pgshp_011.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-012, fx_pgshp_012.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-013, fx_pgshp_013.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-014, fx_pgshp_014.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-015, fx_pgshp_015.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-016, fx_pgshp_016.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-017, fx_pgshp_017.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-018, fx_pgshp_018.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-019, fx_pgshp_019.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-020, fx_pgshp_020.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-021, fx_pgshp_021.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-022, fx_pgshp_022.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-023, fx_pgshp_023.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-024, fx_pgshp_024.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-025, fx_pgshp_025.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-026, fx_pgshp_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGSHP-027, fx_pgshp_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Box Strings rippling, added: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, Box Strings rippling, added, frame 0, Full | largest difference 1 of 255, 261 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Box Strings rippling, added, frame 100, Full | largest difference 1 of 255, 503 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Box Strings rippling, added, frame 239, Full | largest difference 1 of 255, 311 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Box Strings rippling, added, frame 0, Draft | largest difference 1 of 255, 520 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Box Strings rippling, added, frame 100, Draft | largest difference 1 of 255, 405 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Box Strings rippling, added, frame 239, Draft | largest difference 1 of 255, 435 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, points on layer-4's shape, coloured by it: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, points on layer-4's shape, coloured by it, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, points on layer-4's shape, coloured by it, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, points on layer-4's shape, coloured by it, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, points on layer-4's shape, coloured by it, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, points on layer-4's shape, coloured by it, frame 100, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, points on layer-4's shape, coloured by it, frame 239, Draft | largest difference 1 of 255, 448 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |

## Result

148 of 148 checks pass.
