# D-711: Particle Emitter's second system

B-581 (P0-6, EFFECTS.md row 367): particles thrown by particles, after what Trapcode Particular's auxiliary system is for. Aux Emit Continuously drops a trail as each particle goes; At Bounce throws sparks where it bounces on the floor. Each thrown particle has its own life, speed (every way, plus a share of its thrower's), size and opacity going straight to their at-death levels, coloured as its thrower was when thrown or its own colour, drawn as any of the five kinds with the particles. Red Giant publishes no formula and no settings when added, so the rules are ours, stated in `tools/particle_emitter_aux_reference.py`. A file written before reads the second system as added, off, and is saved without it. Every expected pixel is `Fixtures/particle_emitter_aux/expected_particle_emitter_aux.json` and every expected particle `Fixtures/particle_emitter_aux/particles_particle_emitter_aux.json`, written by the tool before this code existed, FX-PEAUX-001 to 030. Tolerances 2e-5 and 1e-7.

## FX-PEAUX-001 to 030

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PEAUX-001 frame 0: The settings as added, the second system off: FX-PEMIT-001 at frames 0, 6 and 12. | largest difference 0.0e0 | yes |
| FX-PEAUX-001 frame 12: The settings as added, the second system off: FX-PEMIT-001 at frames 0, 6 and 12. | largest difference 2.6e-8 | yes |
| FX-PEAUX-001 frame 6: The settings as added, the second system off: FX-PEMIT-001 at frames 0, 6 and 12. | largest difference 2.9e-8 | yes |
| FX-PEAUX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-002 frame 20: A trail: one particle crossing to the right leaves a still dot a frame, each fading out over half a second, at frames 20 and 30. | largest difference 1.7e-8 | yes |
| FX-PEAUX-002 frame 30: A trail: one particle crossing to the right leaves a still dot a frame, each fading out over half a second, at frames 20 and 30. | largest difference 2.8e-8 | yes |
| FX-PEAUX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-003 frame 20: The trail's dots shrinking to nothing instead of fading (Aux Size at Death 0, Aux Opacity at Death 100). | largest difference 2.8e-8 | yes |
| FX-PEAUX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-004 frame 20: Aux Velocity 20: the dots thrown out every way as it goes. | largest difference 2.6e-8 | yes |
| FX-PEAUX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-005 frame 20: Aux Inherit Velocity 100 and no Aux Velocity: the dots ride along with the particle, over it. | largest difference 1.6e-8 | yes |
| FX-PEAUX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-006 frame 20: Aux Set Color Own, red: a red trail behind a white particle. | largest difference 2.6e-8 | yes |
| FX-PEAUX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-007 frame 20: The particle coloured Over Life, the trail From Main: each dot the particle's colour when it was dropped. | largest difference 2.6e-8 | yes |
| FX-PEAUX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-008 frame 20: Aux Particle Type Streak, thrown at 20: short lines. | largest difference 2.9e-8 | yes |
| FX-PEAUX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-009 frame 20: Glowing aux spheres 3 across, Add. | largest difference 8.7e-8 | yes |
| FX-PEAUX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-010 frame 24: At Bounce: the fountain's particles throw four sparks at each bounce on the floor, at frames 24 and 36. | largest difference 3.0e-8 | yes |
| FX-PEAUX-010 frame 36: At Bounce: the fountain's particles throw four sparks at each bounce on the floor, at frames 24 and 36. | largest difference 3.0e-8 | yes |
| FX-PEAUX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-011 frame 12: At Bounce with Floor Action None: no bounce, so nothing thrown; FX-PEMIT-028's burst. | largest difference 2.9e-8 | yes |
| FX-PEAUX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-012 frame 18: The particle lives 0.5 seconds, its trail 1 second: at frame 24 the particle is gone and its trail is still fading. | largest difference 2.6e-8 | yes |
| FX-PEAUX-012 frame 24: The particle lives 0.5 seconds, its trail 1 second: at frame 24 the particle is gone and its trail is still fading. | largest difference 2.3e-8 | yes |
| FX-PEAUX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-013 frame 20: Gravity 40 and Aux Velocity 10: the sparks fall as they go. | largest difference 2.7e-8 | yes |
| FX-PEAUX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-014 frame 24: Aux Particles/sec keyed from 0 at frame 0 to 48 at frame 20 on a burst: read at each thrower's birth, so the later ones leave thicker trails. | largest difference 2.9e-8 | yes |
| FX-PEAUX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-015 frame 20: Aux Opacity at Death keyed from 0 at frame 0 to 100 at frame 20: read at the frame drawn. | largest difference 2.2e-8 | yes |
| FX-PEAUX-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-016 frame 10: FX-PEAUX-002 with the layer starting at frame 10: frame 30 the same as its frame 20. | largest difference 0.0e0 | yes |
| FX-PEAUX-016 frame 30: FX-PEAUX-002 with the layer starting at frame 10: frame 30 the same as its frame 20. | largest difference 1.7e-8 | yes |
| FX-PEAUX-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-017 frame 20: FX-PEAUX-004 with the layer moved 3 pixels right. | largest difference 2.6e-8 | yes |
| FX-PEAUX-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-018 frame 20: Random Seed 7 on FX-PEAUX-004: different sparks. | largest difference 2.9e-8 | yes |
| FX-PEAUX-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-019 frame 20: Aux Particles/sec 0: nothing thrown; the particle alone. | largest difference 1.6e-8 | yes |
| FX-PEAUX-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-020 frame 20: Aux Life 0: nothing thrown lives; the particle alone. | largest difference 1.6e-8 | yes |
| FX-PEAUX-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-021 frame 20: The sparking trail through a composition camera 12 pixels in front, the emitter 6 pixels behind the layer: the sparks bigger as they come nearer. | largest difference 2.1e-8 | yes |
| FX-PEAUX-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-022 frame 12: A burst every way, each particle trailing short-lived dots, at frames 6, 12 and 24. | largest difference 3.0e-8 | yes |
| FX-PEAUX-022 frame 24: A burst every way, each particle trailing short-lived dots, at frames 6, 12 and 24. | largest difference 2.5e-8 | yes |
| FX-PEAUX-022 frame 6: A burst every way, each particle trailing short-lived dots, at frames 6, 12 and 24. | largest difference 2.9e-8 | yes |
| FX-PEAUX-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEAUX-023 frame 0: Aux Emit "trail": not one of off, continuously, at_bounce. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-023 frame 4: Aux Emit "trail": not one of off, continuously, at_bounce. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-024 frame 0: Aux Particle Type "sprite". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-024 frame 4: Aux Particle Type "sprite". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-025 frame 0: Aux Set Color "gradient". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-025 frame 4: Aux Set Color "gradient". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-026 frame 0: Aux Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-026 frame 4: Aux Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-027 frame 0: Aux Particles/sec 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-027 frame 4: Aux Particles/sec 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-028 frame 0: Aux Life -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-028 frame 4: Aux Life -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-029 frame 0: Aux Inherit Velocity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-029 frame 4: Aux Inherit Velocity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEAUX-030 frame 0: Aux Opacity at Death 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-030 frame 4: Aux Opacity at Death 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEAUX-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: throwers and thrown, where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PEAUX-002 frame 20: the 12 particles alive (11 thrown), each its number, birth, place and speed | 12 alive, largest difference 1.4e-17 | yes |
| FX-PEAUX-004 frame 20: the 12 particles alive (11 thrown), each its number, birth, place and speed | 12 alive, largest difference 2.2e-16 | yes |
| FX-PEAUX-010 frame 24: the 72 particles alive (64 thrown), each its number, birth, place and speed | 72 alive, largest difference 1.1e-16 | yes |
| FX-PEAUX-012 frame 24: the 12 particles alive (12 thrown), each its number, birth, place and speed | 12 alive, largest difference 1.4e-17 | yes |
| FX-PEAUX-013 frame 20: the 12 particles alive (11 thrown), each its number, birth, place and speed | 12 alive, largest difference 1.1e-16 | yes |
| FX-PEAUX-022 frame 12: the 39 particles alive (27 thrown), each its number, birth, place and speed | 39 alive, largest difference 2.2e-16 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_peaux_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_peaux_010.json is saved with its 71 settings (the 58 before and the second system's 13), aux emit at_bounce | 71 settings; aux emit "at_bounce", aux particles/bounce 4 | yes |
| a file written before D-711, without the second system, reads it as added (off) and is saved without it | as added: true; saved with 58 settings | yes |
| fx_peaux_023.json is refused in a sentence naming aux emit | Particle Emitter's aux emit is one of "off", "continuously", "at_bounce", and this is "trail". | yes |
| fx_peaux_024.json is refused in a sentence naming aux particle type | Particle Emitter's aux particle type is one of "sphere", "glow_sphere", "star", "cloudlet", "streak", and this is "sprite". | yes |
| fx_peaux_025.json is refused in a sentence naming aux set color | Particle Emitter's aux set color is one of "from_main", "own", and this is "gradient". | yes |
| fx_peaux_026.json is refused in a sentence naming aux colour | Particle Emitter's aux colour is written #rrggbb, and this is "#12345". | yes |
| fx_peaux_027.json is refused in a sentence naming aux particles per second | Particle Emitter's aux particles per second runs from 0 to 1000, and this is 1001. | yes |
| fx_peaux_028.json is refused in a sentence naming aux life | Particle Emitter's aux life runs from 0 to 1000, and this is -1. | yes |
| fx_peaux_029.json is refused in a sentence naming aux inherit velocity | Particle Emitter's aux inherit velocity runs from -1000 to 1000, and this is 1001. | yes |
| fx_peaux_030.json is refused in a sentence naming aux opacity at death | Particle Emitter's aux opacity at death runs from 0 to 100, and this is 101. | yes |
| a file with a Particle Emitter whose aux life is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_peaux_004.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_peaux_010.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_peaux_022.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-711 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_comet_trails.png, frame 48: drawn without a warning, the street changed | 6206 pixels changed; warnings [] | yes |
| 2_sparks_at_the_floor.png, frame 48: drawn without a warning, the street changed | 8919 pixels changed; warnings [] | yes |
| 3_fireworks_fading_trails.png, frame 30: drawn without a warning, the street changed | 12472 pixels changed; warnings [] | yes |

## On the card (D-711)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PEAUX-001, fx_peaux_001.json, frames [0, 12, 6] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-002, fx_peaux_002.json, frames [20, 30] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-003, fx_peaux_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-004, fx_peaux_004.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-005, fx_peaux_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-006, fx_peaux_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-007, fx_peaux_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-008, fx_peaux_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-009, fx_peaux_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-010, fx_peaux_010.json, frames [24, 36] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-011, fx_peaux_011.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-012, fx_peaux_012.json, frames [18, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-013, fx_peaux_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-014, fx_peaux_014.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-015, fx_peaux_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-016, fx_peaux_016.json, frames [10, 30] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-017, fx_peaux_017.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-018, fx_peaux_018.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-019, fx_peaux_019.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-020, fx_peaux_020.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-021, fx_peaux_021.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-022, fx_peaux_022.json, frames [12, 24, 6] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-023, fx_peaux_023.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-024, fx_peaux_024.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-025, fx_peaux_025.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-026, fx_peaux_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-027, fx_peaux_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-028, fx_peaux_028.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-029, fx_peaux_029.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEAUX-030, fx_peaux_030.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, trails of glowing dots, added: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, trails of glowing dots, added, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, trails of glowing dots, added, frame 100, Full | largest difference 1 of 255, 94 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, trails of glowing dots, added, frame 239, Full | largest difference 1 of 255, 141 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, trails of glowing dots, added, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, trails of glowing dots, added, frame 100, Draft | largest difference 1 of 255, 540 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, trails of glowing dots, added, frame 239, Draft | largest difference 1 of 255, 465 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks, frame 100, Full | largest difference 1 of 255, 46 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks, frame 239, Full | largest difference 1 of 255, 8 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks, frame 100, Draft | largest difference 1 of 255, 489 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, sparks thrown at a floor, own colour, streaks, frame 239, Draft | largest difference 1 of 255, 448 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |

## Result

173 of 173 checks pass.
