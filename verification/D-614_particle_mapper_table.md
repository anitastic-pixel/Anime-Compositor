# D-614: Particle Playground's property mappers

B-494, the second step of Particle Playground (P0-6), after what After Effects' Persistent and Ephemeral Property Mappers are for: a layer's red, green and blue under each particle read into its colour, opacity, scale, angle, place, speed or force. The Persistent one sets the property and keeps it; the Ephemeral one works it through an operator (set, add, difference, subtract, multiply, min or max) for one step or for the drawing only. Nothing is carried from frame to frame: a property is worked out along the particle's own steps from its birth (document 20). A file written before D-614 is the effect it was: every new setting is as added when left out. The particles are worked out on the processor and drawn on the card too (D-619), within 1 level of the processor. Adobe publishes no formula, so the rules and the settings when added are ours, stated in `tools/particle_mapper_reference.py`. Every expected pixel is `Fixtures/particle_mapper/expected_particle_mapper.json` and every expected particle `Fixtures/particle_mapper/particles_particle_mapper.json`, written by the tool before this code existed and printed in document 25 as FX-PMAP-001 to 043. Tolerances 2e-5 and 1e-7.

## FX-PMAP-001 to 043 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMAP-001 frame 0: Persistent, red to Opacity 0 to 1: the still grid fades in from left to right. | largest difference 2.4e-8 | yes |
| FX-PMAP-001 frame 6: Persistent, red to Opacity 0 to 1: the still grid fades in from left to right. | largest difference 2.4e-8 | yes |
| FX-PMAP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-002 frame 6: Persistent, red to Scale 0.5 to 2: the grid's dots grow from left to right. | largest difference 2.1e-8 | yes |
| FX-PMAP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-003 frame 6: Persistent, green to Red and blue to Blue: the yellow dots turn mint green, less red in the top row than the bottom, with the ramp's middle blue. | largest difference 6.7e-9 | yes |
| FX-PMAP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-004 frame 24: Persistent, red to X 2.3 to 13.9: each fountain dot is pulled across to the place the ramp names under it, at every quarter frame. | largest difference 2.8e-8 | yes |
| FX-PMAP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-005 frame 24: Persistent, green to Y Velocity -40 to 10: dots low on the ramp shot up, high ones slowed; the speed kept after they leave. | largest difference 2.7e-8 | yes |
| FX-PMAP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-006 frame 24: Persistent, red to X Force -40 to 40: dots on the left pushed left, on the right pushed right, and kept pushed. | largest difference 3.0e-8 | yes |
| FX-PMAP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-007 frame 24: Persistent for the Cannon only, red to Opacity: the grid stays solid, the fountain's dots fade by place. | largest difference 3.0e-8 | yes |
| FX-PMAP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-008 frame 6: Persistent through the selection map: only the left half's dots take Scale 0.3. | largest difference 6.1e-9 | yes |
| FX-PMAP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-009 frame 24: Persistent only for dots older than about half a second (0.51), red to Opacity: young dots solid, older ones faded by where they were. | largest difference 2.7e-8 | yes |
| FX-PMAP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-010 frame 2: Persistent from the late layer (begins at frame 4) at frame 2: its map is clear, every channel 0, so Scale is its Min, 0.4, everywhere. | largest difference 2.0e-8 | yes |
| FX-PMAP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-011 frame 6: Persistent, red to Angle 0 to 90 on the exploded holder: each piece turned by where it was. | largest difference 1.5e-7 | yes |
| FX-PMAP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-012 frame 3: Persistent, red to Scale 0.5 to 1.5 on the Layer Map: each grid point the logo, smaller on the left. | largest difference 4.8e-8 | yes |
| FX-PMAP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-013 frame 30: Fireworks, the shells coloured persistently by the ramp (green to Green 0 to 1): each burst's sparks keep their shell's colour. | largest difference 2.8e-8 | yes |
| FX-PMAP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-014 frame 6: Persistent and the same property twice: red to Opacity, then green to Opacity 0.2 to 0.6; the green one wins, the grid fading by row. | largest difference 2.1e-8 | yes |
| FX-PMAP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-015 frame 24: Persistent with every channel on None: the fountain as without it. | largest difference 2.7e-8 | yes |
| FX-PMAP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-016 frame 24: Persistent, Max Red keyed 0 to 1 over frames 0 to 24, red to Opacity: the fountain's dots fainter the earlier they were mapped. | largest difference 2.6e-8 | yes |
| FX-PMAP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-017 frame 24: Ephemeral Set, red to Opacity: the fountain's dots fade by where they are now. | largest difference 2.6e-8 | yes |
| FX-PMAP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-018 frame 24: Ephemeral Add, green to Scale 0 to 1: dots bigger lower down. | largest difference 2.9e-8 | yes |
| FX-PMAP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-019 frame 6: Ephemeral Difference, red to Opacity: |1 - red|, the dots fading to the right. | largest difference 2.4e-8 | yes |
| FX-PMAP-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-020 frame 6: Ephemeral Subtract, red to Scale 0 to 0.8: dots smaller to the right. | largest difference 2.4e-8 | yes |
| FX-PMAP-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-021 frame 6: Ephemeral Multiply, green to Opacity: the grid's top row fainter than its bottom. | largest difference 2.0e-8 | yes |
| FX-PMAP-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-022 frame 6: Ephemeral Min, red to Opacity 0.3 to 1: no dot more opaque than the ramp says. | largest difference 2.6e-8 | yes |
| FX-PMAP-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-023 frame 6: Ephemeral Max, red to Scale 0 to 2: dots on the right grown, those on the left kept at 1. | largest difference 2.4e-8 | yes |
| FX-PMAP-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-024 frame 24: Ephemeral Add, red to X -2 to 2: each dot drawn moved by the ramp, its path unchanged. | largest difference 2.9e-8 | yes |
| FX-PMAP-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-025 frame 24: Ephemeral Add, red to X Velocity -30 to 30: each step pushed by the ramp under it, the push not kept. | largest difference 2.9e-8 | yes |
| FX-PMAP-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-026 frame 24: Ephemeral Set, green to Y Force -60 to 60: dots high up pulled up, low ones down, only while there. | largest difference 2.9e-8 | yes |
| FX-PMAP-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-027 frame 12: Ephemeral Add, red to Angle 0 to 45 on the exploded holder with Auto-Orient: each piece turned a little more to the right. | largest difference 1.2e-7 | yes |
| FX-PMAP-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-028 frame 6: Ephemeral through the selection map, Scale 2: only dots over its white left half drawn twice the size. | largest difference 2.6e-8 | yes |
| FX-PMAP-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-029 frame 24: Ephemeral only for dots younger than about half a second (0.51), blue to Green 1: the young red dots yellow. | largest difference 2.7e-8 | yes |
| FX-PMAP-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-030 frame 24: Both: Persistent red to Opacity 0.3 to 1, then Ephemeral Multiply green to Opacity. | largest difference 1.3e-8 | yes |
| FX-PMAP-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-031 frame 6: Ephemeral Set Scale 0: nothing drawn. | largest difference 0.0e0 | yes |
| FX-PMAP-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-032 frame 36: The wall with Ephemeral Add, red to X Velocity -20 to 20: the dots still bounce inside the box. | largest difference 2.8e-8 | yes |
| FX-PMAP-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-033 frame 30: Fireworks with Ephemeral Set, red to Red 0 to 1, for the Particle Exploder's sparks only. | largest difference 2.8e-8 | yes |
| FX-PMAP-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMAP-034 frame 6: Persistent Map Layer naming a layer that is not in the composition: no mapping, the grid drawn as without it, and EFFECT_LAYER_MISSING. | largest difference 1.8e-10 | yes |
| FX-PMAP-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PMAP-035 frame 6: Ephemeral Selection Map naming a missing layer: the mapper applies to every dot, and EFFECT_LAYER_MISSING. | largest difference 2.4e-8 | yes |
| FX-PMAP-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PMAP-036 frame 0: Persistent Min Red -10001, below -10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-036 frame 4: Persistent Min Red -10001, below -10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-037 frame 0: Ephemeral Max Blue 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-037 frame 4: Ephemeral Max Blue 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-038 frame 0: Persistent Age Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-038 frame 4: Persistent Age Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-039 frame 0: Persistent Map Red To "mass", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-039 frame 4: Persistent Map Red To "mass", not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-040 frame 0: Ephemeral Operator "divide". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-040 frame 4: Ephemeral Operator "divide". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-041 frame 0: Ephemeral Particles From "everything". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-041 frame 4: Ephemeral Particles From "everything". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-042 frame 0: A Persistent Map Layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-042 frame 4: A Persistent Map Layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMAP-043 frame 0: Ephemeral Max Green keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-043 frame 4: Ephemeral Max Green keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PMAP-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMAP-004 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 4.4e-16 | yes |
| FX-PMAP-005 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 8.9e-15 | yes |
| FX-PMAP-006 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 1.8e-15 | yes |
| FX-PMAP-013 frame 30: the 16 particles, each its emitter, numbers, birth, place and speed | 16 listed, largest difference 1.8e-15 | yes |
| FX-PMAP-025 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 7.1e-15 | yes |
| FX-PMAP-026 frame 24: the 12 particles, each its emitter, numbers, birth, place and speed | 12 listed, largest difference 1.8e-14 | yes |
| FX-PMAP-032 frame 36: the 9 particles, each its emitter, numbers, birth, place and speed | 9 listed, largest difference 3.6e-15 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmap_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmap_001.json, which writes every setting, is saved with all 73 (45 numbers, 2 positions, 14 words, 10 layers and 2 colours) | 73 settings; persistent map layer "ramp", persistent map red to "opacity", ephemeral operator "set" | yes |
| fx_pmap_036.json is refused in a sentence naming persistent min red | Particle Playground's persistent min red runs from -10000 to 10000, and this is -10001. | yes |
| fx_pmap_037.json is refused in a sentence naming ephemeral max blue | Particle Playground's ephemeral max blue runs from -10000 to 10000, and this is 10001. | yes |
| fx_pmap_038.json is refused in a sentence naming persistent age feather | Particle Playground's persistent age feather runs from 0 to 1000, and this is -1. | yes |
| fx_pmap_039.json is refused in a sentence naming persistent map red to | Particle Playground's persistent map red to is one of "none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y", "x_velocity", "y_velocity", "x_force", "y_force", "x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction", "static_friction", and this is "mass". | yes |
| fx_pmap_040.json is refused in a sentence naming ephemeral operator | Particle Playground's ephemeral operator is one of "set", "add", "difference", "subtract", "multiply", "min", "max", and this is "divide". | yes |
| fx_pmap_041.json is refused in a sentence naming ephemeral particles from | Particle Playground's ephemeral particles from is one of "all", "cannon", "grid", "layer_exploder", "particle_exploder", "cannon_grid", "cannon_layer_exploder", "cannon_particle_exploder", "grid_layer_exploder", "grid_particle_exploder", "layer_exploder_particle_exploder", "all_but_cannon", "all_but_grid", "all_but_layer_exploder", "all_but_particle_exploder", and this is "everything". | yes |
| fx_pmap_042.json is refused in a sentence naming persistent map layer | Particle Playground's persistent map layer is the name of a layer of this composition, and this is 3. | yes |
| fx_pmap_043.json is refused in a sentence naming ephemeral max green | Particle Playground's ephemeral max green runs from -10000 to 10000, and this is 20000. | yes |
| a file with a Particle Playground whose persistent max red is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Playground whose ephemeral operator is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmap_004.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmap_011.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmap_028.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmap_032.json frame 36 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-614 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_grid_unmapped.png, frame 0: drawn without a warning, the street changed | 41472 pixels changed; warnings [] | yes |
| 2_grid_coloured_by_the_street.png, frame 0: drawn without a warning, the street changed | 13974 pixels changed; warnings [] | yes |
| 3_halftone_scale_by_red.png, frame 0: drawn without a warning, the street changed | 39168 pixels changed; warnings [] | yes |
| 4_fountain_painted_by_the_street.png, frame 48: drawn without a warning, the street changed | 1633 pixels changed; warnings [] | yes |
| 5_ephemeral_push_sideways.png, frame 0: drawn without a warning, the street changed | 45950 pixels changed; warnings [] | yes |

## On the card (D-619)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMAP-001, fx_pmap_001.json, frames [0, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-002, fx_pmap_002.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-003, fx_pmap_003.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-004, fx_pmap_004.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-005, fx_pmap_005.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-006, fx_pmap_006.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-007, fx_pmap_007.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-008, fx_pmap_008.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-009, fx_pmap_009.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-010, fx_pmap_010.json, frames [2] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-011, fx_pmap_011.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-012, fx_pmap_012.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-013, fx_pmap_013.json, frames [30] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-014, fx_pmap_014.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-015, fx_pmap_015.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-016, fx_pmap_016.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-017, fx_pmap_017.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-018, fx_pmap_018.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-019, fx_pmap_019.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-020, fx_pmap_020.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-021, fx_pmap_021.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-022, fx_pmap_022.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-023, fx_pmap_023.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-024, fx_pmap_024.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-025, fx_pmap_025.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-026, fx_pmap_026.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-027, fx_pmap_027.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-028, fx_pmap_028.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-029, fx_pmap_029.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-030, fx_pmap_030.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-031, fx_pmap_031.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-032, fx_pmap_032.json, frames [36] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-033, fx_pmap_033.json, frames [30] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-034, fx_pmap_034.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-035, fx_pmap_035.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-036, fx_pmap_036.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-037, fx_pmap_037.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-038, fx_pmap_038.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-039, fx_pmap_039.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-040, fx_pmap_040.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-041, fx_pmap_041.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-042, fx_pmap_042.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PMAP-043, fx_pmap_043.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Result

208 of 208 checks pass.
