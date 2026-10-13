# D-613: Particle Playground

B-493, the particle engine's third effect (P0-6), after what After Effects' Particle Playground (Simulation) is for: dots shot from a Cannon or laid out on a Grid, a layer cut into flying pieces by the Layer Exploder, any of them split again by the Particle Exploder, shown as a layer by the Layer Map, pulled by Gravity and kept inside a mask by the Wall, each force taking only the particles its Affects choose. Nothing is carried from frame to frame: every frame works each particle out from its birth (document 20), so a force that would need the frame before (Repel, and the maps and walls read as they move) is a gap, logged in the playtest. Drawn on the processor only. Adobe publishes no formula, so the rules and the settings when added are ours, stated in `tools/particle_playground_reference.py`. Every expected pixel is `Fixtures/particle_playground/expected_particle_playground.json` and every expected particle `Fixtures/particle_playground/particles_particle_playground.json`, written by the tool before this code existed and printed in document 25 as FX-PPG-001 to 060. Tolerances 2e-5 and 1e-7.

## FX-PPG-001 to 060 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PPG-001 frame 0: The settings as added, at frames 0, 2 and 6: 20 red dots a second shot upwards from the middle at about 130 pixels a second, so fast they are out of the 16 by 10 layer within frames; the layer's own picture is gone. | largest difference 0.0e0 | yes |
| FX-PPG-001 frame 2: The settings as added, at frames 0, 2 and 6: 20 red dots a second shot upwards from the middle at about 130 pixels a second, so fast they are out of the 16 by 10 layer within frames; the layer's own picture is gone. | largest difference 2.9e-8 | yes |
| FX-PPG-001 frame 6: The settings as added, at frames 0, 2 and 6: 20 red dots a second shot upwards from the middle at about 130 pixels a second, so fast they are out of the 16 by 10 layer within frames; the layer's own picture is gone. | largest difference 2.8e-8 | yes |
| FX-PPG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-002 frame 11: One still red dot of radius 3 in the middle, born at frame 12: nothing at frame 11, the dot at frame 13 and the same at frame 30 (it never dies). | largest difference 0.0e0 | yes |
| FX-PPG-002 frame 13: One still red dot of radius 3 in the middle, born at frame 12: nothing at frame 11, the dot at frame 13 and the same at frame 30 (it never dies). | largest difference 1.7e-8 | yes |
| FX-PPG-002 frame 30: One still red dot of radius 3 in the middle, born at frame 12: nothing at frame 11, the dot at frame 13 and the same at frame 30 (it never dies). | largest difference 1.7e-8 | yes |
| FX-PPG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-003 frame 13: A tiny dot, radius 0.35: drawn at 0.7 and a quarter as strong. | largest difference 4.5e-11 | yes |
| FX-PPG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-004 frame 10: A round barrel of radius 4 (Barrel Radius -4): dots born scattered in a disc about the middle. | largest difference 2.6e-8 | yes |
| FX-PPG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-005 frame 10: A square barrel of radius 4: scattered in a square. | largest difference 3.0e-8 | yes |
| FX-PPG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-006 frame 12: The fountain: shot upwards from near the bottom, spreading 20 degrees either side, slowing and falling back, at frames 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.8e-8 | yes |
| FX-PPG-006 frame 24: The fountain: shot upwards from near the bottom, spreading 20 degrees either side, slowing and falling back, at frames 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.7e-8 | yes |
| FX-PPG-006 frame 40: The fountain: shot upwards from near the bottom, spreading 20 degrees either side, slowing and falling back, at frames 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.5e-8 | yes |
| FX-PPG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-007 frame 24: Direction 90: the fountain thrown to the right, falling. | largest difference 1.9e-8 | yes |
| FX-PPG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-008 frame 24: Direction keyed from -60 to 60 over frames 0 to 24: the jet sweeps from upper left to upper right. | largest difference 1.1e-8 | yes |
| FX-PPG-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-009 frame 12: Particles Per Second keyed from 0 to 24 over frames 0 to 20. | largest difference 2.9e-8 | yes |
| FX-PPG-009 frame 24: Particles Per Second keyed from 0 to 24 over frames 0 to 20. | largest difference 3.0e-8 | yes |
| FX-PPG-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-010 frame 24: Cannon Color blue. | largest difference 2.7e-8 | yes |
| FX-PPG-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-011 frame 24: Gravity Direction 90: falling to the right. | largest difference 2.5e-8 | yes |
| FX-PPG-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-012 frame 24: Gravity Force Random Spread 40: each dot falls at its own rate. | largest difference 3.0e-8 | yes |
| FX-PPG-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-013 frame 24: Gravity keyed from 0 to 60 over frames 0 to 24. | largest difference 2.9e-8 | yes |
| FX-PPG-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-014 frame 24: Gravity only on dots older than about half a second (0.51): they rise straight, then fall. | largest difference 2.8e-8 | yes |
| FX-PPG-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-015 frame 24: Gravity only on dots younger than about half a second (0.51). | largest difference 2.8e-8 | yes |
| FX-PPG-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-016 frame 24: Gravity older than half a second with an Age Feather of 0.6: some dots start falling earlier, some later. | largest difference 2.8e-8 | yes |
| FX-PPG-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-017 frame 24: Gravity through the selection map: only dots over its white left half fall. | largest difference 2.8e-8 | yes |
| FX-PPG-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-018 frame 24: Gravity from the Grid only: the Cannon's dots fly on unbent. | largest difference 2.8e-8 | yes |
| FX-PPG-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-019 frame 0: The grid: 4 by 2 still yellow dots of radius 1 on a 12 by 6 grid, a new grid born on the last every frame, so the dots' soft edges fill in. | largest difference 1.9e-9 | yes |
| FX-PPG-019 frame 6: The grid: 4 by 2 still yellow dots of radius 1 on a 12 by 6 grid, a new grid born on the last every frame, so the dots' soft edges fill in. | largest difference 1.8e-10 | yes |
| FX-PPG-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-020 frame 12: The grid falling: a full grid born every frame, each falling from where it was born, streaks of dots under each grid point. | largest difference 2.6e-8 | yes |
| FX-PPG-020 frame 6: The grid falling: a full grid born every frame, each falling from where it was born, streaks of dots under each grid point. | largest difference 2.6e-8 | yes |
| FX-PPG-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-021 frame 8: Grid across keyed from 1 to 4 over frames 0 to 6, gravity 40. | largest difference 2.5e-8 | yes |
| FX-PPG-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-022 frame 0: The holder exploded into 2-pixel pieces: at frame 0 its own picture exactly, then flying apart slowly. | largest difference 1.9e-7 | yes |
| FX-PPG-022 frame 12: The holder exploded into 2-pixel pieces: at frame 0 its own picture exactly, then flying apart slowly. | largest difference 1.5e-7 | yes |
| FX-PPG-022 frame 18: The holder exploded into 2-pixel pieces: at frame 0 its own picture exactly, then flying apart slowly. | largest difference 1.7e-7 | yes |
| FX-PPG-022 frame 6: The holder exploded into 2-pixel pieces: at frame 0 its own picture exactly, then flying apart slowly. | largest difference 1.2e-7 | yes |
| FX-PPG-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-023 frame 12: Pieces of 4 pixels, Radius 2. | largest difference 1.2e-7 | yes |
| FX-PPG-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-024 frame 12: Pieces of 3 pixels, Radius 1.5: the right and bottom pieces cut short by the layer's edge. | largest difference 1.7e-7 | yes |
| FX-PPG-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-025 frame 12: The exploded pieces falling, gravity 20. | largest difference 1.5e-7 | yes |
| FX-PPG-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-026 frame 0: Exploding the logo layer, 4 by 4 in the middle: only its pieces. | largest difference 0.0e0 | yes |
| FX-PPG-026 frame 12: Exploding the logo layer, 4 by 4 in the middle: only its pieces. | largest difference 2.7e-8 | yes |
| FX-PPG-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-027 frame 12: Exploding a layer that begins at frame 4, after the in point: nothing. | largest difference 0.0e0 | yes |
| FX-PPG-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-028 frame 12: Auto-Orient on: each piece turned to the way it flies. | largest difference 1.2e-7 | yes |
| FX-PPG-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-029 frame 18: Fireworks: shells shot up, each bursting into 16 sparks about half a second (0.51) after it was shot, the sparks falling, at frames 18, 30 and 40. | largest difference 1.8e-8 | yes |
| FX-PPG-029 frame 30: Fireworks: shells shot up, each bursting into 16 sparks about half a second (0.51) after it was shot, the sparks falling, at frames 18, 30 and 40. | largest difference 2.3e-8 | yes |
| FX-PPG-029 frame 40: Fireworks: shells shot up, each bursting into 16 sparks about half a second (0.51) after it was shot, the sparks falling, at frames 18, 30 and 40. | largest difference 2.8e-8 | yes |
| FX-PPG-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-030 frame 18: Fireworks with no age limit on the exploder: every shell bursts at birth. | largest difference 2.6e-8 | yes |
| FX-PPG-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-031 frame 30: Exploder Radius 1: four sparks a shell. | largest difference 2.3e-8 | yes |
| FX-PPG-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-032 frame 12: The exploded pieces burst again into four sparks each about a quarter second (0.26) in. | largest difference 7.5e-8 | yes |
| FX-PPG-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-033 frame 30: Particles From for the exploder set to the Grid: the Cannon's shells fly on. | largest difference 2.3e-8 | yes |
| FX-PPG-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-034 frame 24: The Layer Map: each fountain dot shown as the 4 by 4 logo. | largest difference 3.2e-8 | yes |
| FX-PPG-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-035 frame 24: The Layer Map with Auto-Orient on: each logo turned to the way it flies. | largest difference 1.2e-7 | yes |
| FX-PPG-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-036 frame 36: The Layer Map only for dots older than about a second (1.01): young dots stay dots. | largest difference 5.4e-8 | yes |
| FX-PPG-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-037 frame 3: The Layer Map on the Grid: each still grid point the logo. | largest difference 0.0e0 | yes |
| FX-PPG-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-038 frame 2: The Layer Map naming the late layer, at frame 2 (not yet begun): the dots affected are not drawn. | largest difference 0.0e0 | yes |
| FX-PPG-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-039 frame 12: A wall: dots thrown every way inside the box mask bounce off its sides. | largest difference 3.0e-8 | yes |
| FX-PPG-039 frame 36: A wall: dots thrown every way inside the box mask bounce off its sides. | largest difference 2.7e-8 | yes |
| FX-PPG-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-040 frame 36: The wall with gravity 40: dots bounce on the floor of the box. | largest difference 2.9e-8 | yes |
| FX-PPG-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-041 frame 36: The wall for the Grid only: the Cannon's dots go through it. | largest difference 1.1e-8 | yes |
| FX-PPG-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-042 frame 36: Wall Mask 0: no wall even with the mask there. | largest difference 1.1e-8 | yes |
| FX-PPG-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-043 frame 22: The fountain with the layer starting at frame 10: nothing at frame 9, frame 22 the same as PG-006's frame 12. | largest difference 2.8e-8 | yes |
| FX-PPG-043 frame 9: The fountain with the layer starting at frame 10: nothing at frame 9, frame 22 the same as PG-006's frame 12. | largest difference 0.0e0 | yes |
| FX-PPG-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-044 frame 24: The fountain with the layer moved 3 pixels right. | largest difference 2.7e-8 | yes |
| FX-PPG-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-045 frame 24: Random Seed 7: a different fountain. | largest difference 2.5e-8 | yes |
| FX-PPG-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-046 frame 12: Particles Per Second 0 and no grid: nothing. | largest difference 0.0e0 | yes |
| FX-PPG-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPG-047 frame 24: The Explode Layer naming a layer that is not in the composition: no pieces, the fountain drawn, and EFFECT_LAYER_MISSING. | largest difference 2.7e-8 | yes |
| FX-PPG-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PPG-048 frame 36: Wall Mask 2 with only one mask: no wall, the dots drawn, and EFFECT_PATH_MISSING. | largest difference 1.1e-8 | yes |
| FX-PPG-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PATH_MISSING"] and ["EFFECT_PATH_MISSING"] | yes |
| FX-PPG-049 frame 0: Particles Per Second -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-049 frame 4: Particles Per Second -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-050 frame 0: Direction Random Spread 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-050 frame 4: Direction Random Spread 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-051 frame 0: Particles Across 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-051 frame 4: Particles Across 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-052 frame 0: Explode Radius 0.4, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-052 frame 4: Explode Radius 0.4, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-053 frame 0: Gravity Older/Younger Than 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-053 frame 4: Gravity Older/Younger Than 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-054 frame 0: Wall Mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-054 frame 4: Wall Mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-055 frame 0: Particles From "everything". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-055 frame 4: Particles From "everything". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-056 frame 0: Auto-Orient "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-056 frame 4: Auto-Orient "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-057 frame 0: Cannon Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-057 frame 4: Cannon Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-058 frame 0: An Explode Layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-058 frame 4: An Explode Layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-059 frame 0: Cannon Position with one number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-059 frame 4: Cannon Position with one number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPG-060 frame 0: Velocity keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-060 frame 4: Velocity keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPG-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PPG-006 frame 24: the 6 particles, each its emitter, numbers, birth, place and speed | 6 listed, largest difference 8.9e-16 | yes |
| FX-PPG-008 frame 24: the 6 particles, each its emitter, numbers, birth, place and speed | 6 listed, largest difference 1.8e-15 | yes |
| FX-PPG-014 frame 24: the 6 particles, each its emitter, numbers, birth, place and speed | 6 listed, largest difference 1.8e-15 | yes |
| FX-PPG-016 frame 24: the 6 particles, each its emitter, numbers, birth, place and speed | 6 listed, largest difference 1.8e-15 | yes |
| FX-PPG-017 frame 24: the 6 particles, each its emitter, numbers, birth, place and speed | 6 listed, largest difference 1.8e-15 | yes |
| FX-PPG-022 frame 18: the 40 particles, each its emitter, numbers, birth, place and speed | 40 listed, largest difference 1.8e-15 | yes |
| FX-PPG-029 frame 30: the 16 particles, each its emitter, numbers, birth, place and speed | 16 listed, largest difference 1.8e-15 | yes |
| FX-PPG-039 frame 36: the 9 particles, each its emitter, numbers, birth, place and speed | 9 listed, largest difference 3.6e-15 | yes |
| FX-PPG-040 frame 36: the 9 particles, each its emitter, numbers, birth, place and speed | 9 listed, largest difference 3.6e-15 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ppg_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ppg_029.json is saved with its 44 settings (29 numbers, 2 positions, 5 words, 6 layers and 2 colours) and no maps, wall, clock or history | 44 settings; cannon position [50,90], explode layer "", auto orient "off" | yes |
| fx_ppg_049.json is refused in a sentence naming particles per second | Particle Playground's particles per second runs from 0 to 1000, and this is -1. | yes |
| fx_ppg_050.json is refused in a sentence naming direction random spread | Particle Playground's direction random spread runs from 0 to 360, and this is 361. | yes |
| fx_ppg_051.json is refused in a sentence naming particles across | Particle Playground's particles across runs from 0 to 100, and this is 101. | yes |
| fx_ppg_052.json is refused in a sentence naming explode radius | Particle Playground's explode radius runs from 0.5 to 100, and this is 0.4. | yes |
| fx_ppg_053.json is refused in a sentence naming gravity older younger than | Particle Playground's gravity older younger than runs from -1000 to 1000, and this is 1001. | yes |
| fx_ppg_054.json is refused in a sentence naming wall mask | Particle Playground's wall mask runs from 0 to 1000, and this is -1. | yes |
| fx_ppg_055.json is refused in a sentence naming gravity particles from | Particle Playground's gravity particles from is one of "all", "cannon", "grid", "layer_exploder", "particle_exploder", "cannon_grid", "cannon_layer_exploder", "cannon_particle_exploder", "grid_layer_exploder", "grid_particle_exploder", "layer_exploder_particle_exploder", "all_but_cannon", "all_but_grid", "all_but_layer_exploder", "all_but_particle_exploder", and this is "everything". | yes |
| fx_ppg_056.json is refused in a sentence naming auto orient | Particle Playground's auto orient is one of "off", "on", and this is "yes". | yes |
| fx_ppg_057.json is refused in a sentence naming cannon colour | Particle Playground's cannon colour is written #rrggbb, and this is "#12345". | yes |
| fx_ppg_058.json is refused in a sentence naming explode layer | Particle Playground's explode layer is the name of a layer of this composition, and this is 3. | yes |
| fx_ppg_059.json is refused in a sentence naming cannon position | Particle Playground's cannon position is two numbers, x then y, and this has 1. | yes |
| fx_ppg_060.json is refused in a sentence naming velocity | Particle Playground's velocity runs from -10000 to 10000, and this is 20000. | yes |
| a file with a Particle Playground whose velocity is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Playground whose auto-orient is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Playground without its wall selection map is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ppg_006.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ppg_022.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ppg_029.json frame 30 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ppg_039.json frame 36 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-613 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 379 pixels changed; warnings [] | yes |
| 2_fountain_and_grid.png, frame 48: drawn without a warning, the street changed | 3485 pixels changed; warnings [] | yes |
| 3_layer_exploded.png, frame 24: drawn without a warning, the street changed | 62516 pixels changed; warnings [] | yes |
| 4_fireworks.png, frame 72: drawn without a warning, the street changed | 497 pixels changed; warnings [] | yes |
| 5_wall_box.png, frame 72: drawn without a warning, the street changed | 2431 pixels changed; warnings [] | yes |

## Result

243 of 243 checks pass.
