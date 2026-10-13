# D-715: Particle Playground's Mass

B-585 (P0-6, EFFECTS.md row 364), by the owner's decision of 2026-10-13 (FX-PMAP-039 now refuses "repel" instead of "mass"): the property mappers can set a particle's Mass, 1 when born. In every step its force is divided by its mass before it joins Gravity, which pulls every mass alike; at a mass of 0 or below the force does nothing; Static Friction weighs the pull after the division. A file written before D-715 is the effect it was. The card draws within 1 level of the processor. Adobe publishes no formula, so the rule is ours, stated in `tools/particle_mapper_mass_reference.py`. Every expected pixel is `Fixtures/particle_mapper_mass/expected_particle_mapper_mass.json` and every expected particle `Fixtures/particle_mapper_mass/particles_particle_mapper_mass.json`, written by the tool before this code existed and printed in document 25 as FX-PMASS-001 to 016. Tolerances 2e-5 and 1e-7.

## FX-PMASS-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMASS-001 frame 6: Persistent, red to Mass 0.5 to 4, green to Y Force 60: the grid pushed down, the light dots on the left furthest. | largest difference 2.9e-8 | yes |
| FX-PMASS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-002 frame 6: Persistent Mass 2 with Y Force 60: the grid falls as with Mass 1 and Y Force 30. | largest difference 2.2e-8 | yes |
| FX-PMASS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-003 frame 6: Persistent Mass 0 with Y Force 60: the force does nothing, the grid stays. | largest difference 1.8e-10 | yes |
| FX-PMASS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-004 frame 6: Persistent Mass -1 with Y Force 60: as Mass 0, the grid stays. | largest difference 1.8e-10 | yes |
| FX-PMASS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-005 frame 6: Persistent Mass 3 under Gravity 40 and no force: Gravity pulls every mass alike, the grid falls as without it. | largest difference 2.5e-8 | yes |
| FX-PMASS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-006 frame 24: The fountain, red to Mass 1 to 5, blue to X Force 40: blown right, the heavy dots on the right less. | largest difference 2.7e-8 | yes |
| FX-PMASS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-007 frame 6: Ephemeral Set, red to Mass 0.5 to 4, with Persistent Y Force 60: the push by where each dot is, step by step. | largest difference 2.9e-8 | yes |
| FX-PMASS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-008 frame 6: Ephemeral Multiply, green to Mass 1 to 4, with Persistent Y Force 60: the lower row heavier while there. | largest difference 2.9e-8 | yes |
| FX-PMASS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-009 frame 6: Persistent Mass 2, Y Force 50 and Static Friction 30 on the grid: the pull is 25, so it stays; at Mass 1 it would fall. | largest difference 1.8e-10 | yes |
| FX-PMASS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-010 frame 30: Fireworks, red to Mass 1 to 4, blue to X Force 40: the sparks keep their shell's mass and drift right. | largest difference 2.7e-8 | yes |
| FX-PMASS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-011 frame 36: The wall with Persistent Mass 2 and Y Force 60: the dots still bounce inside the box. | largest difference 2.9e-8 | yes |
| FX-PMASS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-012 frame 6: Persistent, Max Red keyed 1 to 6 over frames 0 to 6, red to Mass, with Y Force 60: the push weakens as time goes on. | largest difference 2.7e-8 | yes |
| FX-PMASS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-013 frame 12: The exploded holder, red to Mass 1 to 4, green to Y Force 0 to 80. | largest difference 1.3e-7 | yes |
| FX-PMASS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-014 frame 24: Persistent for the Cannon only, Mass 4 and X Force 40: the fountain blown right a quarter as far, the grid's dots not pushed at all. | largest difference 3.0e-8 | yes |
| FX-PMASS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMASS-015 frame 0: Persistent Map Red To "Mass", a capital M: the words are lowercase. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMASS-015 frame 4: Persistent Map Red To "Mass", a capital M: the words are lowercase. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMASS-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMASS-016 frame 0: Ephemeral Map Green To "weight", not a property. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMASS-016 frame 4: Ephemeral Map Green To "weight", not a property. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMASS-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMASS-001 frame 6: the 56 particles, each its emitter, numbers, birth, place and speed | 56 listed, largest difference 4.4e-16 | yes |
| FX-PMASS-006 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 1.8e-15 | yes |
| FX-PMASS-007 frame 6: the 56 particles, each its emitter, numbers, birth, place and speed | 56 listed, largest difference 4.4e-16 | yes |
| FX-PMASS-010 frame 30: the 16 particles, each its emitter, numbers, birth, place and speed | 16 listed, largest difference 1.8e-15 | yes |
| FX-PMASS-011 frame 36: the 9 particles, each its emitter, numbers, birth, place and speed | 9 listed, largest difference 3.6e-15 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmass_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmass_015.json is refused in a sentence naming mass | Particle Playground's persistent map red to is one of "none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y", "x_velocity", "y_velocity", "x_force", "y_force", "x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction", "static_friction", "mass", and this is "Mass". | yes |
| fx_pmass_016.json is refused in a sentence naming weight | Particle Playground's ephemeral map green to is one of "none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y", "x_velocity", "y_velocity", "x_force", "y_force", "x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction", "static_friction", "mass", and this is "weight". | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmass_001.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmass_010.json frame 30 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmass_011.json frame 36 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-715 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_grid_pushed_down_mass_1.png, frame 12: drawn without a warning, the street changed | 30648 pixels changed; warnings [] | yes |
| 2_grid_pushed_down_mass_4.png, frame 12: drawn without a warning, the street changed | 15624 pixels changed; warnings [] | yes |
| 3_fountain_blown_right_heavy_where_bright.png, frame 48: drawn without a warning, the street changed | 5266 pixels changed; warnings [] | yes |

## On the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMASS-001, fx_pmass_001.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-002, fx_pmass_002.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-003, fx_pmass_003.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-004, fx_pmass_004.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-005, fx_pmass_005.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-006, fx_pmass_006.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-007, fx_pmass_007.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-008, fx_pmass_008.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-009, fx_pmass_009.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-010, fx_pmass_010.json, frames [30] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-011, fx_pmass_011.json, frames [36] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-012, fx_pmass_012.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-013, fx_pmass_013.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-014, fx_pmass_014.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-015, fx_pmass_015.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMASS-016, fx_pmass_016.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Result

79 of 79 checks pass.
