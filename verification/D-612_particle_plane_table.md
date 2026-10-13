# D-612: Particle Plane

B-492, the particle engine's flat emitter (P0-6), after the purpose of CycoreFX's CC Particle Systems II (Simulation): Particle Space's engine (D-610) with everything in the layer's own plane, particles born from a producer, thrown by one of eleven animations, pulled down by gravity and slowed by resistance, drawn straight onto the layer as lines, faded or shaded spheres, discs, bubbles or stars, faded by an Opacity Map. Nothing is carried from frame to frame: every frame works each particle out from its birth (document 20). CycoreFX publishes no formula and no settings when added, so the rules, those settings and the name are ours, stated in `tools/particle_plane_reference.py`. Every expected pixel is `Fixtures/particle_plane/expected_particle_plane.json` and every expected particle `Fixtures/particle_plane/particles_particle_plane.json`, written by the tool before this code existed and printed in document 25 as FX-PPLANE-001 to 058. Tolerances 2e-5 and 1e-7.

## FX-PPLANE-001 to 058 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PPLANE-001 frame 0: The settings as they start, at frames 0, 12 and 24: nothing at frame 0, then 200 a second thrown out every way in the layer's plane as faint yellow-to-red streaks, falling a little. | largest difference 0.0e0 | yes |
| FX-PPLANE-001 frame 12: The settings as they start, at frames 0, 12 and 24: nothing at frame 0, then 200 a second thrown out every way in the layer's plane as faint yellow-to-red streaks, falling a little. | largest difference 3.0e-8 | yes |
| FX-PPLANE-001 frame 24: The settings as they start, at frames 0, 12 and 24: nothing at frame 0, then 200 a second thrown out every way in the layer's plane as faint yellow-to-red streaks, falling a little. | largest difference 3.0e-8 | yes |
| FX-PPLANE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-002 frame 20: One disc, still, in the middle: a round dot 8 pixels across, a fifth of the way from yellow to red, fully covering. | largest difference 2.5e-8 | yes |
| FX-PPLANE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-003 frame 20: The same particle as a Faded Sphere: soft to its edge. | largest difference 2.7e-8 | yes |
| FX-PPLANE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-004 frame 20: As a Shaded Sphere: lit from the upper right and in front. | largest difference 3.0e-8 | yes |
| FX-PPLANE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-005 frame 20: As a Bubble: a lit rim, a nearly clear middle. | largest difference 2.9e-8 | yes |
| FX-PPLANE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-006 frame 20: As a Star: five points up, 8 pixels from tip to tip across the top. | largest difference 2.8e-8 | yes |
| FX-PPLANE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-007 frame 20: A still Line: no movement, so a round dot as wide as the size, the same as FX-PPLANE-002's disc. | largest difference 2.5e-8 | yes |
| FX-PPLANE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-008 frame 20: A tiny star, size 3 (under a pixel's radius): drawn 1.4 pixels across and fainter by its true size. | largest difference 3.8e-10 | yes |
| FX-PPLANE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-009 frame 20: Death Size 10: the disc a fifth of the way from 50 to 10, smaller. | largest difference 2.6e-8 | yes |
| FX-PPLANE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-010 frame 20: Size Variation 80 on a spread of still discs: each its own size. | largest difference 3.0e-8 | yes |
| FX-PPLANE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-011 frame 20: Opacity Map Fade In: at a fifth of its life, a fifth covering. | largest difference 7.4e-9 | yes |
| FX-PPLANE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-012 frame 20: Fade Out: four fifths covering. | largest difference 2.6e-8 | yes |
| FX-PPLANE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-013 frame 20: Fade In and Out: two fifths covering. | largest difference 1.5e-8 | yes |
| FX-PPLANE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-014 frame 20: Custom opacity 0, 100, 40, 100, 0: at a fifth of its life, between the first two, about 80 per cent. | largest difference 3.0e-8 | yes |
| FX-PPLANE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-015 frame 20: Custom opacity keyed: the second stop from 100 at frame 0 to 20 at frame 20, read at the frame drawn. | largest difference 1.3e-8 | yes |
| FX-PPLANE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-016 frame 20: Birth and death colours blue to green. | largest difference 3.0e-8 | yes |
| FX-PPLANE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-017 frame 20: Origin to Death over the yellow column: from the drawing's yellow towards red. | largest difference 5.5e-8 | yes |
| FX-PPLANE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-018 frame 20: Birth to Origin over the blue column. | largest difference 2.7e-8 | yes |
| FX-PPLANE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-019 frame 20: Origin Constant over the clear first column: black. | largest difference 1.2e-8 | yes |
| FX-PPLANE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-020 frame 20: Custom colours white, yellow, orange, red, near-black: at a fifth of its life, between white and yellow. | largest difference 1.2e-8 | yes |
| FX-PPLANE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-021 frame 20: Two overlapping discs, Composite: the newer over the older. | largest difference 2.9e-8 | yes |
| FX-PPLANE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-022 frame 20: The two, Screen. | largest difference 2.7e-8 | yes |
| FX-PPLANE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-023 frame 20: The two, Add. | largest difference 5.6e-8 | yes |
| FX-PPLANE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-024 frame 20: Two overlapping stars, Composite. | largest difference 3.0e-8 | yes |
| FX-PPLANE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-025 frame 12: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 3.0e-8 | yes |
| FX-PPLANE-025 frame 24: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 3.0e-8 | yes |
| FX-PPLANE-025 frame 40: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 3.0e-8 | yes |
| FX-PPLANE-025 frame 6: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.8e-8 | yes |
| FX-PPLANE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-026 frame 12: Resistance 3: the burst slows and stays tighter. | largest difference 2.9e-8 | yes |
| FX-PPLANE-026 frame 24: Resistance 3: the burst slows and stays tighter. | largest difference 3.0e-8 | yes |
| FX-PPLANE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-027 frame 12: Direction, Extra Angle 30: a jet upwards within 15 degrees. | largest difference 2.9e-8 | yes |
| FX-PPLANE-027 frame 24: Direction, Extra Angle 30: a jet upwards within 15 degrees. | largest difference 2.8e-8 | yes |
| FX-PPLANE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-028 frame 12: Direction 90, Extra Angle 30: the jet to the right. | largest difference 3.0e-8 | yes |
| FX-PPLANE-028 frame 24: Direction 90, Extra Angle 30: the jet to the right. | largest difference 2.9e-8 | yes |
| FX-PPLANE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-029 frame 12: Cone, 60 degrees: two jets, 30 degrees either side of up. | largest difference 2.9e-8 | yes |
| FX-PPLANE-029 frame 24: Cone, 60 degrees: two jets, 30 degrees either side of up. | largest difference 2.9e-8 | yes |
| FX-PPLANE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-030 frame 12: Viscous: Explosive held back hard. | largest difference 2.9e-8 | yes |
| FX-PPLANE-030 frame 24: Viscous: Explosive held back hard. | largest difference 3.0e-8 | yes |
| FX-PPLANE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-031 frame 12: Twirl: swirling round the producer, clockwise on screen. | largest difference 2.9e-8 | yes |
| FX-PPLANE-031 frame 24: Twirl: swirling round the producer, clockwise on screen. | largest difference 3.0e-8 | yes |
| FX-PPLANE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-032 frame 12: Twirly: each particle swirling by its own amount. | largest difference 3.0e-8 | yes |
| FX-PPLANE-032 frame 24: Twirly: each particle swirling by its own amount. | largest difference 3.0e-8 | yes |
| FX-PPLANE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-033 frame 12: Vortex: swirling and pulled in. | largest difference 3.0e-8 | yes |
| FX-PPLANE-033 frame 24: Vortex: swirling and pulled in. | largest difference 3.0e-8 | yes |
| FX-PPLANE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-034 frame 12: Fire: rising against gravity, narrowing and flickering. | largest difference 2.9e-8 | yes |
| FX-PPLANE-034 frame 24: Fire: rising against gravity, narrowing and flickering. | largest difference 3.0e-8 | yes |
| FX-PPLANE-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-035 frame 12: Jet Sideways: out to the left and right of up. | largest difference 3.0e-8 | yes |
| FX-PPLANE-035 frame 24: Jet Sideways: out to the left and right of up. | largest difference 3.0e-8 | yes |
| FX-PPLANE-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-036 frame 12: Fractal Omni: Explosive in the turbulence. | largest difference 2.9e-8 | yes |
| FX-PPLANE-036 frame 24: Fractal Omni: Explosive in the turbulence. | largest difference 2.9e-8 | yes |
| FX-PPLANE-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-037 frame 12: Fractal Uni: Direction in the turbulence. | largest difference 3.0e-8 | yes |
| FX-PPLANE-037 frame 24: Fractal Uni: Direction in the turbulence. | largest difference 3.0e-8 | yes |
| FX-PPLANE-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-038 frame 12: Inherit Velocity 100 with the producer keyed across: the particles carry its motion. | largest difference 2.9e-8 | yes |
| FX-PPLANE-038 frame 24: Inherit Velocity 100 with the producer keyed across: the particles carry its motion. | largest difference 2.8e-8 | yes |
| FX-PPLANE-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-039 frame 12: Direction keyed from 0 to 180 over frames 0 to 24 on Direction, Extra Angle 20: the jet turns from up through right to down. | largest difference 2.8e-8 | yes |
| FX-PPLANE-039 frame 24: Direction keyed from 0 to 180 over frames 0 to 24 on Direction, Extra Angle 20: the jet turns from up through right to down. | largest difference 2.9e-8 | yes |
| FX-PPLANE-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-040 frame 10: Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start slowly and quicken. | largest difference 3.0e-8 | yes |
| FX-PPLANE-040 frame 20: Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start slowly and quicken. | largest difference 2.9e-8 | yes |
| FX-PPLANE-040 frame 30: Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start slowly and quicken. | largest difference 3.0e-8 | yes |
| FX-PPLANE-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-041 frame 12: Gravity keyed 0 to 3 over frames 0 to 24. | largest difference 3.0e-8 | yes |
| FX-PPLANE-041 frame 24: Gravity keyed 0 to 3 over frames 0 to 24. | largest difference 3.0e-8 | yes |
| FX-PPLANE-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-042 frame 10: FX-PPLANE-025 with the layer starting at frame 10: nothing before, frame 22 the same as FX-PPLANE-025's frame 12. | largest difference 0.0e0 | yes |
| FX-PPLANE-042 frame 22: FX-PPLANE-025 with the layer starting at frame 10: nothing before, frame 22 the same as FX-PPLANE-025's frame 12. | largest difference 3.0e-8 | yes |
| FX-PPLANE-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-043 frame 12: FX-PPLANE-025 with the layer moved 3 pixels right. | largest difference 3.0e-8 | yes |
| FX-PPLANE-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-044 frame 12: Random Seed 7: a different burst. | largest difference 3.0e-8 | yes |
| FX-PPLANE-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-045 frame 12: Birth Rate 0: nothing. | largest difference 0.0e0 | yes |
| FX-PPLANE-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-046 frame 12: Longevity 0: nothing. | largest difference 0.0e0 | yes |
| FX-PPLANE-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-047 frame 12: Lines, larger: the streak's length follows the size. | largest difference 3.0e-8 | yes |
| FX-PPLANE-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-048 frame 12: A burst of stars, faded out over life, added. | largest difference 6.5e-7 | yes |
| FX-PPLANE-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PPLANE-049 frame 0: Birth Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-049 frame 4: Birth Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-050 frame 0: Longevity 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-050 frame 4: Longevity 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-051 frame 0: Birth Size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-051 frame 4: Birth Size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-052 frame 0: Custom Opacity 3 at 120, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-052 frame 4: Custom Opacity 3 at 120, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-053 frame 0: Animation "direction_axis": Particle Space's word, not the plane's. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-053 frame 4: Animation "direction_axis": Particle Space's word, not the plane's. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-054 frame 0: Animation "Explosive": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-054 frame 4: Animation "Explosive": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-055 frame 0: Particle Type "cube": not one of the plane's kinds (D-714, the owner, 2026-10-13: was "textured_square", which the plane now draws). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-055 frame 4: Particle Type "cube": not one of the plane's kinds (D-714, the owner, 2026-10-13: was "textured_square", which the plane now draws). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-056 frame 0: Opacity Map "linear". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-056 frame 4: Opacity Map "linear". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-057 frame 0: Birth Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-057 frame 4: Birth Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PPLANE-058 frame 0: Transfer Mode "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-058 frame 4: Transfer Mode "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PPLANE-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PPLANE-025 frame 12: the 50 particles alive, each its number, birth, place and speed, and none off the plane | 50 alive, largest difference 1.8e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-026 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-028 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-029 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-031 frame 12: the 50 particles alive, each its number, birth, place and speed, and none off the plane | 50 alive, largest difference 1.8e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-033 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-034 frame 24: the 6 particles alive, each its number, birth, place and speed, and none off the plane | 6 alive, largest difference 1.7e-18, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-035 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-037 frame 24: the 6 particles alive, each its number, birth, place and speed, and none off the plane | 6 alive, largest difference 2.8e-17, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-038 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-039 frame 24: the 100 particles alive, each its number, birth, place and speed, and none off the plane | 100 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |
| FX-PPLANE-040 frame 30: the 42 particles alive, each its number, birth, place and speed, and none off the plane | 42 alive, largest difference 3.6e-15, all at depth 0 moving in the plane: true | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pplane_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pplane_048.json is saved with its 35 settings, 23 numbers, 5 words and 7 colours, and no clock or history | 35 settings; particle type "star", opacity map "fade_out", transfer mode "add" | yes |
| fx_pplane_049.json is refused in a sentence naming birth rate | Particle Plane's birth rate runs from 0 to 100, and this is -1. | yes |
| fx_pplane_050.json is refused in a sentence naming longevity | Particle Plane's longevity runs from 0 to 30, and this is 31. | yes |
| fx_pplane_051.json is refused in a sentence naming birth size | Particle Plane's birth size runs from 0 to 100, and this is 101. | yes |
| fx_pplane_052.json is refused in a sentence naming custom opacity 3 | Particle Plane's custom opacity 3 runs from 0 to 100, and this is 120. | yes |
| fx_pplane_053.json is refused in a sentence naming animation | Particle Plane's animation is one of "explosive", "direction", "cone", "viscous", "twirl", "twirly", "vortex", "fire", "jet_sideways", "fractal_omni", "fractal_uni", and this is "direction_axis". | yes |
| fx_pplane_054.json is refused in a sentence naming animation | Particle Plane's animation is one of "explosive", "direction", "cone", "viscous", "twirl", "twirly", "vortex", "fire", "jet_sideways", "fractal_omni", "fractal_uni", and this is "Explosive". | yes |
| fx_pplane_055.json is refused in a sentence naming particle type | Particle Plane's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "star", "textured_disc", "textured_faded_disc", "textured_square", and this is "cube". | yes |
| fx_pplane_056.json is refused in a sentence naming opacity map | Particle Plane's opacity map is one of "constant", "fade_in", "fade_out", "fade_in_and_out", "custom", and this is "linear". | yes |
| fx_pplane_057.json is refused in a sentence naming birth colour | Particle Plane's birth colour is written #rrggbb, and this is "#12345". | yes |
| fx_pplane_058.json is refused in a sentence naming transfer mode | Particle Plane's transfer mode is one of "composite", "screen", "add", and this is "multiply". | yes |
| a file with a Particle Plane whose birth rate is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Plane whose particle type is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Plane without its fifth custom colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pplane_025.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pplane_006.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pplane_048.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pplane_034.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-612)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pplane_001.json, frames [0, 12, 24] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_002.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_004.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_010.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_011.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_012.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_014.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_016.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_017.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_018.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_019.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_020.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_021.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_022.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_023.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_024.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_025.json, frames [12, 24, 40, 6] at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_026.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_027.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_028.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_029.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_030.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_031.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_032.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_033.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_034.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_035.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_036.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_037.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_038.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_039.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_040.json, frames [10, 20, 30] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_041.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_042.json, frames [10, 22] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_043.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_044.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_045.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_046.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_047.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_048.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_049.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_050.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_051.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_052.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_053.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_054.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_055.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_056.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_057.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pplane_058.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Plane as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Plane as added on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane as added on three layers, frame 100, Full | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane as added on three layers, frame 239, Full | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane as added on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane as added on three layers, frame 100, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane as added on three layers, frame 239, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened on three layers, frame 100, Full | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened on three layers, frame 239, Full | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened on three layers, frame 100, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane stars in a vortex, faded in and out, screened on three layers, frame 239, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4 on three layers, frame 239, Full | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Plane bubbles from a fire, custom colours, birth rate keyed 0 to 4 on three layers, frame 239, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-612 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 3802 pixels changed; warnings [] | yes |
| 2_fountain.png, frame 48: drawn without a warning, the street changed | 10428 pixels changed; warnings [] | yes |
| 3_star_burst.png, frame 30: drawn without a warning, the street changed | 22855 pixels changed; warnings [] | yes |
| 4_vortex.png, frame 72: drawn without a warning, the street changed | 26222 pixels changed; warnings [] | yes |
| 5_fire.png, frame 48: drawn without a warning, the street changed | 20601 pixels changed; warnings [] | yes |
| 6_jet_sideways_lines.png, frame 36: drawn without a warning, the street changed | 15117 pixels changed; warnings [] | yes |

## Result

322 of 322 checks pass.
