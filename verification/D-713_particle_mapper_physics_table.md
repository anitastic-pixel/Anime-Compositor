# D-713: Particle Playground's further mapper properties

B-583 (P0-6, EFFECTS.md row 364): the property mappers can now set X and Y Scale (a particle drawn stretched along its own axes), Angular Velocity and Torque (degrees a second, and a second a second), Lifespan (seconds from its own birth), and Kinetic and Static Friction (pixels a second a second: kinetic slows a moving particle, static holds a still one while the pull on it is no more). Mass came later (D-715). Gradient Velocity, Character and Font Size are not built and are refused by name. A file written before D-713 is the effect it was. The card draws within 1 level of the processor. Adobe publishes no formula, so the rules are ours, stated in `tools/particle_mapper_physics_reference.py`. Every expected pixel is `Fixtures/particle_mapper_physics/expected_particle_mapper_physics.json` and every expected particle `Fixtures/particle_mapper_physics/particles_particle_mapper_physics.json`, written by the tool before this code existed and printed in document 25 as FX-PMPH-001 to 028. Tolerances 2e-5 and 1e-7.

## FX-PMPH-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMPH-001 frame 6: Persistent, red to X Scale 0.5 to 3: the grid's dots stretched wider from left to right. | largest difference 2.8e-8 | yes |
| FX-PMPH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-002 frame 6: Persistent, red to Y Scale 2 and green to Angle 0 to 90: tall dots, the lower row turned further. | largest difference 2.8e-8 | yes |
| FX-PMPH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-003 frame 12: Persistent, red to Angular Velocity 0 to 360: the exploded holder's pieces spin, faster to the right. | largest difference 1.7e-7 | yes |
| FX-PMPH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-004 frame 12: Persistent, red to Torque -720 to 720: pieces on the left spin up one way, on the right the other. | largest difference 1.4e-7 | yes |
| FX-PMPH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-005 frame 24: Persistent, red to Lifespan 0.13 to 0.73 seconds: the fountain's dots die young on the left. | largest difference 2.7e-8 | yes |
| FX-PMPH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-006 frame 24: Persistent Lifespan 0.51 seconds everywhere: only the fountain's dots younger than that are drawn. | largest difference 2.7e-8 | yes |
| FX-PMPH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-007 frame 24: Persistent Kinetic Friction 15: the fountain's dots slow and stay lower. | largest difference 2.7e-8 | yes |
| FX-PMPH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-008 frame 6: Persistent Static Friction 50 on the grid falling at 40: it stays where it was born. | largest difference 1.8e-10 | yes |
| FX-PMPH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-009 frame 6: Persistent Static Friction 30 on the grid falling at 40: not enough, it falls as without it. | largest difference 2.5e-8 | yes |
| FX-PMPH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-010 frame 36: The wall with Persistent Kinetic Friction 8: the dots still bounce inside the box, slowing. | largest difference 2.9e-8 | yes |
| FX-PMPH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-011 frame 3: Persistent, red to X Scale 0.5 to 1.5 on the Layer Map: each grid point the logo, narrower on the left. | largest difference 1.8e-8 | yes |
| FX-PMPH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-012 frame 6: Persistent, green to Y Scale 0.5 to 2 on the exploded holder: pieces squashed at the top, drawn out at the bottom. | largest difference 1.8e-7 | yes |
| FX-PMPH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-013 frame 44: Fireworks, Persistent Lifespan 0.8 seconds: the shells burst, and each spark lives 0.8 seconds from its own birth: at frame 44 the first burst is gone and the second shell still climbing. | largest difference 3.0e-8 | yes |
| FX-PMPH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-014 frame 24: Persistent for the Cannon only, red to X Scale 3: the grid's dots stay round, the fountain's stretched. | largest difference 3.0e-8 | yes |
| FX-PMPH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-015 frame 12: Persistent, Max Red keyed 0 to 720 over frames 0 to 12, red to Torque: the pieces spin up harder as time goes on. | largest difference 1.4e-7 | yes |
| FX-PMPH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-016 frame 24: Persistent X Scale 2 with Auto-Orient: the fountain's dots stretched along the way they move. | largest difference 2.9e-8 | yes |
| FX-PMPH-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-017 frame 6: Ephemeral Set, red to X Scale 0 to 3: drawn stretched by where they are, narrow on the left. | largest difference 2.7e-8 | yes |
| FX-PMPH-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-018 frame 6: Ephemeral Multiply, green to Y Scale 1 to 3: the lower row drawn taller. | largest difference 2.6e-8 | yes |
| FX-PMPH-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-019 frame 12: Ephemeral Add, red to Angular Velocity 0 to 360: pieces spin by where they are, the spin not kept. | largest difference 1.7e-7 | yes |
| FX-PMPH-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-020 frame 12: Ephemeral Set, red to Torque -720 to 720: the pieces' spin pushed while they stand there. | largest difference 1.4e-7 | yes |
| FX-PMPH-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-021 frame 24: Ephemeral Set, red to Lifespan 0 to 1 second: a dot drawn only while younger than the ramp under it says. | largest difference 2.7e-8 | yes |
| FX-PMPH-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-022 frame 24: Ephemeral Multiply, green to Lifespan: an endless lifespan stays endless, the fountain as without it. | largest difference 2.7e-8 | yes |
| FX-PMPH-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-023 frame 24: Ephemeral Set, red to Kinetic Friction 0 to 40: dots on the right braked, only while there. | largest difference 2.7e-8 | yes |
| FX-PMPH-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-024 frame 6: Ephemeral Set, red to Static Friction 0 to 80 on the grid falling at 40: the right half stays, the left half falls. | largest difference 2.5e-8 | yes |
| FX-PMPH-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-025 frame 6: Both: Persistent red to X Scale 2, Ephemeral Set green to Y Scale 0.5 to 2. | largest difference 2.9e-8 | yes |
| FX-PMPH-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMPH-026 frame 0: Persistent Map Red To "gradient_velocity", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMPH-026 frame 4: Persistent Map Red To "gradient_velocity", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMPH-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMPH-027 frame 0: Ephemeral Map Green To "character", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMPH-027 frame 4: Ephemeral Map Green To "character", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMPH-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMPH-028 frame 0: Ephemeral Map Blue To "font_size", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMPH-028 frame 4: Ephemeral Map Blue To "font_size", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMPH-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMPH-005 frame 24: the 4 particles, each its emitter, numbers, birth, place and speed | 4 listed, largest difference 0.0e0 | yes |
| FX-PMPH-007 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 8.9e-16 | yes |
| FX-PMPH-008 frame 6: the 56 particles, each its emitter, numbers, birth, place and speed | 56 listed, largest difference 0.0e0 | yes |
| FX-PMPH-010 frame 36: the 9 particles, each its emitter, numbers, birth, place and speed | 9 listed, largest difference 3.6e-15 | yes |
| FX-PMPH-013 frame 44: the 1 particles, each its emitter, numbers, birth, place and speed | 1 listed, largest difference 0.0e0 | yes |
| FX-PMPH-023 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 2.9e-15 | yes |
| FX-PMPH-024 frame 6: the 56 particles, each its emitter, numbers, birth, place and speed | 56 listed, largest difference 4.4e-16 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmph_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmph_026.json is refused in a sentence naming gradient_velocity | Particle Playground's persistent map red to is one of "none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y", "x_velocity", "y_velocity", "x_force", "y_force", "x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction", "static_friction", "mass", and this is "gradient_velocity". | yes |
| fx_pmph_027.json is refused in a sentence naming character | Particle Playground's ephemeral map green to is one of "none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y", "x_velocity", "y_velocity", "x_force", "y_force", "x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction", "static_friction", "mass", and this is "character". | yes |
| fx_pmph_028.json is refused in a sentence naming font_size | Particle Playground's ephemeral map blue to is one of "none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y", "x_velocity", "y_velocity", "x_force", "y_force", "x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction", "static_friction", "mass", and this is "font_size". | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmph_002.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmph_012.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmph_017.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmph_010.json frame 36 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-713 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_dots_stretched_and_turned_by_the_street.png, frame 0: drawn without a warning, the street changed | 56305 pixels changed; warnings [] | yes |
| 2_fountain_short_lived_where_dark.png, frame 48: drawn without a warning, the street changed | 1900 pixels changed; warnings [] | yes |
| 3_fountain_with_kinetic_friction.png, frame 48: drawn without a warning, the street changed | 4177 pixels changed; warnings [] | yes |
| 4_grid_held_except_on_the_dark_road.png, frame 24: drawn without a warning, the street changed | 38160 pixels changed; warnings [] | yes |

## On the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMPH-001, fx_pmph_001.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-002, fx_pmph_002.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-003, fx_pmph_003.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-004, fx_pmph_004.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-005, fx_pmph_005.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-006, fx_pmph_006.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-007, fx_pmph_007.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-008, fx_pmph_008.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-009, fx_pmph_009.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-010, fx_pmph_010.json, frames [36] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-011, fx_pmph_011.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-012, fx_pmph_012.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-013, fx_pmph_013.json, frames [44] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-014, fx_pmph_014.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-015, fx_pmph_015.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-016, fx_pmph_016.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-017, fx_pmph_017.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-018, fx_pmph_018.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-019, fx_pmph_019.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-020, fx_pmph_020.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-021, fx_pmph_021.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-022, fx_pmph_022.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-023, fx_pmph_023.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-024, fx_pmph_024.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-025, fx_pmph_025.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-026, fx_pmph_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-027, fx_pmph_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMPH-028, fx_pmph_028.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Result

133 of 133 checks pass.
