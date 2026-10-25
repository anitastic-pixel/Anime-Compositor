# D-610: Particle Space

B-490, the particle engine's first effect (P0-6), after the purpose of CycoreFX's CC Particle World (Simulation): particles born from a producer, thrown by one of eleven animations, pulled by gravity, wind and turbulence, landing on a floor, and drawn in full 3D as lines, faded or shaded spheres, discs or bubbles, through the composition camera when the file has one and the effect's own camera when not. Nothing is carried from frame to frame: every frame works each particle out from its birth (document 20). CycoreFX publishes no formula and no settings when added, so the rules and those settings are ours, stated in `tools/particle_space_reference.py`. Every expected pixel is `Fixtures/particle_space/expected_particle_space.json` and every expected particle `Fixtures/particle_space/particles_particle_space.json`, written by the tool before this code existed and printed in document 25 as FX-PSPACE-001 to 071. Tolerances 2e-5 and 1e-7.

## FX-PSPACE-001 to 071 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PSPACE-001 frame 0: The settings as they start, at frames 0, 12 and 24: nothing at frame 0 (the first particle is born 0.06 of a frame after it), then 200 a second thrown out every way from the middle as faint yellow-to-red streaks, falling a little. | largest difference 0.0e0 | yes |
| FX-PSPACE-001 frame 12: The settings as they start, at frames 0, 12 and 24: nothing at frame 0 (the first particle is born 0.06 of a frame after it), then 200 a second thrown out every way from the middle as faint yellow-to-red streaks, falling a little. | largest difference 3.0e-8 | yes |
| FX-PSPACE-001 frame 24: The settings as they start, at frames 0, 12 and 24: nothing at frame 0 (the first particle is born 0.06 of a frame after it), then 200 a second thrown out every way from the middle as faint yellow-to-red streaks, falling a little. | largest difference 3.0e-8 | yes |
| FX-PSPACE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-002 frame 20: One disc, still, seen close: a round dot about 4.6 pixels across, its colour a fifth of the way from yellow to red, fully covering at the centre (Max Opacity 100). | largest difference 2.9e-8 | yes |
| FX-PSPACE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-003 frame 20: The same particle as a Faded Sphere: soft, fading to nothing at its edge. | largest difference 2.5e-8 | yes |
| FX-PSPACE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-004 frame 20: As a Shaded Sphere: lit from the upper right and in front, darker at its lower left. | largest difference 2.8e-8 | yes |
| FX-PSPACE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-005 frame 20: As a Bubble: a lit rim with a nearly clear middle. | largest difference 1.3e-8 | yes |
| FX-PSPACE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-006 frame 20: A Disc with Volume Shade 60: lit, but less than the Shaded Sphere. | largest difference 2.8e-8 | yes |
| FX-PSPACE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-007 frame 20: A Shaded Sphere lit from 200 degrees and low (height 10): its bright side moves to the bottom. | largest difference 2.8e-8 | yes |
| FX-PSPACE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-008 frame 20: Death Size 2: the disc a fifth of the way from size 10 to 2, so smaller than FX-PSPACE-002. | largest difference 1.9e-8 | yes |
| FX-PSPACE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-009 frame 20: A tiny disc, size 1 (under a pixel): drawn 1.4 pixels across and fainter by its true size. | largest difference 1.8e-9 | yes |
| FX-PSPACE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-010 frame 20: Size Variation 80 on a spread of still discs: each its own size. | largest difference 2.9e-8 | yes |
| FX-PSPACE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-011 frame 20: Birth and death colours blue to green. | largest difference 2.2e-8 | yes |
| FX-PSPACE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-012 frame 20: Origin to Death, the producer over the yellow column: the disc starts the drawing's yellow under it and goes towards red. | largest difference 5.5e-8 | yes |
| FX-PSPACE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-013 frame 20: Birth to Origin, over the blue column. | largest difference 2.7e-8 | yes |
| FX-PSPACE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-014 frame 20: Origin Constant, over the clear first column: black. | largest difference 1.1e-8 | yes |
| FX-PSPACE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-015 frame 20: Origin Constant over the half-covered last column: its yellow, not darkened by the half covering. | largest difference 5.4e-8 | yes |
| FX-PSPACE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-016 frame 20: Custom with white, yellow, orange, red and near-black: at a fifth of its life, between the first two (white and yellow). | largest difference 2.8e-8 | yes |
| FX-PSPACE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-017 frame 20: Two overlapping discs, Composite: the nearer one over the farther. | largest difference 2.9e-8 | yes |
| FX-PSPACE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-018 frame 20: The two, Screen: brighter where they overlap, neither over the other. | largest difference 2.8e-8 | yes |
| FX-PSPACE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-019 frame 20: The two, Add: the overlap summed. | largest difference 5.6e-8 | yes |
| FX-PSPACE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-020 frame 20: The two with Composite With Original on: over the drawing. | largest difference 1.9e-7 | yes |
| FX-PSPACE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-021 frame 20: Depth Cue Fade, distance 3: the particle 2 units away fades to a third. | largest difference 9.9e-9 | yes |
| FX-PSPACE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-022 frame 20: Depth Cue Fog towards blue, distance 3: two thirds of the way to blue. | largest difference 2.0e-8 | yes |
| FX-PSPACE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-023 frame 20: A spread of still shaded spheres at many depths: nearer ones larger and over farther ones. | largest difference 2.9e-8 | yes |
| FX-PSPACE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-024 frame 12: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.9e-8 | yes |
| FX-PSPACE-024 frame 24: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.9e-8 | yes |
| FX-PSPACE-024 frame 40: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.9e-8 | yes |
| FX-PSPACE-024 frame 6: A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 3.0e-8 | yes |
| FX-PSPACE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-025 frame 12: Resistance 3: the burst slows and stays tighter. | largest difference 2.9e-8 | yes |
| FX-PSPACE-025 frame 24: Resistance 3: the burst slows and stays tighter. | largest difference 2.9e-8 | yes |
| FX-PSPACE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-026 frame 12: Direction Axis, Extra Angle 30: a jet upwards within 15 degrees. | largest difference 2.7e-8 | yes |
| FX-PSPACE-026 frame 24: Direction Axis, Extra Angle 30: a jet upwards within 15 degrees. | largest difference 3.0e-8 | yes |
| FX-PSPACE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-027 frame 12: Cone Axis, 60 degrees: a hollow cone upwards. | largest difference 3.0e-8 | yes |
| FX-PSPACE-027 frame 24: Cone Axis, 60 degrees: a hollow cone upwards. | largest difference 3.0e-8 | yes |
| FX-PSPACE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-028 frame 12: Viscous: Explosive held back hard. | largest difference 2.9e-8 | yes |
| FX-PSPACE-028 frame 24: Viscous: Explosive held back hard. | largest difference 2.9e-8 | yes |
| FX-PSPACE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-029 frame 12: Twirl: swirling round the upright axis through the producer. | largest difference 2.9e-8 | yes |
| FX-PSPACE-029 frame 24: Twirl: swirling round the upright axis through the producer. | largest difference 2.9e-8 | yes |
| FX-PSPACE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-030 frame 12: Twirly: each particle swirling by its own amount. | largest difference 2.9e-8 | yes |
| FX-PSPACE-030 frame 24: Twirly: each particle swirling by its own amount. | largest difference 3.0e-8 | yes |
| FX-PSPACE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-031 frame 12: Vortex: swirling and pulled in to the axis. | largest difference 3.0e-8 | yes |
| FX-PSPACE-031 frame 24: Vortex: swirling and pulled in to the axis. | largest difference 2.9e-8 | yes |
| FX-PSPACE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-032 frame 12: Fire: rising against gravity, narrowing and flickering. | largest difference 1.0e-8 | yes |
| FX-PSPACE-032 frame 24: Fire: rising against gravity, narrowing and flickering. | largest difference 7.4e-9 | yes |
| FX-PSPACE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-033 frame 12: Jet Sideways: out flat round the axis. | largest difference 2.8e-8 | yes |
| FX-PSPACE-033 frame 24: Jet Sideways: out flat round the axis. | largest difference 2.9e-8 | yes |
| FX-PSPACE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-034 frame 12: Fractal Omni: Explosive in the turbulence. | largest difference 1.4e-8 | yes |
| FX-PSPACE-034 frame 24: Fractal Omni: Explosive in the turbulence. | largest difference 1.3e-8 | yes |
| FX-PSPACE-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-035 frame 12: Fractal Uni: Direction Axis in the turbulence. | largest difference 6.6e-9 | yes |
| FX-PSPACE-035 frame 24: Fractal Uni: Direction Axis in the turbulence. | largest difference 7.3e-9 | yes |
| FX-PSPACE-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-036 frame 12: Turbulence 3, Scale 0.5, Evolution 2 on Explosive. | largest difference 8.4e-9 | yes |
| FX-PSPACE-036 frame 24: Turbulence 3, Scale 0.5, Evolution 2 on Explosive. | largest difference 1.3e-8 | yes |
| FX-PSPACE-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-037 frame 12: Wind 1.5 to the right: the burst drifts right. | largest difference 3.0e-8 | yes |
| FX-PSPACE-037 frame 24: Wind 1.5 to the right: the burst drifts right. | largest difference 2.9e-8 | yes |
| FX-PSPACE-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-038 frame 12: Gravity Vector to the right: falling sideways. | largest difference 2.9e-8 | yes |
| FX-PSPACE-038 frame 24: Gravity Vector to the right: falling sideways. | largest difference 3.0e-8 | yes |
| FX-PSPACE-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-039 frame 12: Inherit Velocity 100 with the producer keyed across: the particles carry its motion. | largest difference 2.8e-8 | yes |
| FX-PSPACE-039 frame 24: Inherit Velocity 100 with the producer keyed across: the particles carry its motion. | largest difference 2.9e-8 | yes |
| FX-PSPACE-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-040 frame 12: Floor Bounce at 0.1, Bounciness 60, Random 30, Spread 50: the burst lands and hops. | largest difference 3.0e-8 | yes |
| FX-PSPACE-040 frame 24: Floor Bounce at 0.1, Bounciness 60, Random 30, Spread 50: the burst lands and hops. | largest difference 2.9e-8 | yes |
| FX-PSPACE-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-041 frame 12: Floor Ice: the burst lands and slides. | largest difference 2.7e-8 | yes |
| FX-PSPACE-041 frame 24: Floor Ice: the burst lands and slides. | largest difference 3.0e-8 | yes |
| FX-PSPACE-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-042 frame 12: Floor Glue: the burst lands and stays. | largest difference 3.0e-8 | yes |
| FX-PSPACE-042 frame 24: Floor Glue: the burst lands and stays. | largest difference 2.8e-8 | yes |
| FX-PSPACE-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-043 frame 20: The effect camera turned 60 degrees about Y and 20 about X: the spread seen from the side and above. | largest difference 2.8e-8 | yes |
| FX-PSPACE-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-044 frame 20: The effect camera 1 unit away with a 100 degree lens. | largest difference 2.8e-8 | yes |
| FX-PSPACE-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-045 frame 20: The composition's own camera, as added: the spread drawn through it, nearer particles larger and spread wider. | largest difference 3.0e-8 | yes |
| FX-PSPACE-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-046 frame 20: The composition camera moved 3 pixels left: the layer moves right on screen, and the particles off its plane move by more or less than it. | largest difference 2.9e-8 | yes |
| FX-PSPACE-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-047 frame 20: The composition camera dollied in with its lens widened to match (depth -12, zoom 12): the layer's plane unchanged, the depth stronger. | largest difference 3.0e-8 | yes |
| FX-PSPACE-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-048 frame 20: FX-PSPACE-045 with the layer moved 3 pixels right. | largest difference 2.9e-8 | yes |
| FX-PSPACE-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-049 frame 20: The camera dollied to 2 pixels from the plane: the particles nearer than it, behind the camera, are not drawn (4 of the 17). | largest difference 2.9e-8 | yes |
| FX-PSPACE-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-050 frame 10: Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start slowly and quicken. | largest difference 2.7e-8 | yes |
| FX-PSPACE-050 frame 20: Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start slowly and quicken. | largest difference 2.8e-8 | yes |
| FX-PSPACE-050 frame 30: Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start slowly and quicken. | largest difference 2.8e-8 | yes |
| FX-PSPACE-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-051 frame 10: Velocity keyed from 0.2 to 1 over frames 0 to 20: each particle keeps the speed it was born with. | largest difference 2.8e-8 | yes |
| FX-PSPACE-051 frame 20: Velocity keyed from 0.2 to 1 over frames 0 to 20: each particle keeps the speed it was born with. | largest difference 3.0e-8 | yes |
| FX-PSPACE-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-052 frame 12: Gravity keyed 0 to 3 over frames 0 to 24: every particle feels it when it changes. | largest difference 3.0e-8 | yes |
| FX-PSPACE-052 frame 24: Gravity keyed 0 to 3 over frames 0 to 24: every particle feels it when it changes. | largest difference 2.9e-8 | yes |
| FX-PSPACE-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-053 frame 10: FX-PSPACE-024 with the layer starting at frame 10: nothing before, and frame 22 the same as FX-PSPACE-024's frame 12. | largest difference 0.0e0 | yes |
| FX-PSPACE-053 frame 22: FX-PSPACE-024 with the layer starting at frame 10: nothing before, and frame 22 the same as FX-PSPACE-024's frame 12. | largest difference 2.9e-8 | yes |
| FX-PSPACE-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-054 frame 12: FX-PSPACE-024 with the layer moved 3 pixels right. | largest difference 2.9e-8 | yes |
| FX-PSPACE-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-055 frame 12: Random Seed 7: a different burst. | largest difference 3.0e-8 | yes |
| FX-PSPACE-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-056 frame 12: Birth Rate 0: nothing. | largest difference 0.0e0 | yes |
| FX-PSPACE-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-057 frame 12: Longevity 0: nothing. | largest difference 0.0e0 | yes |
| FX-PSPACE-057: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-058 frame 12: Lines, larger: the streak's length follows Birth Size. | largest difference 3.0e-8 | yes |
| FX-PSPACE-058: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-059 frame 0: Birth Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-059 frame 4: Birth Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-060 frame 0: Birth Rate 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-060 frame 4: Birth Rate 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-061 frame 0: Longevity 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-061 frame 4: Longevity 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-062 frame 0: Turbulence Scale 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-062 frame 4: Turbulence Scale 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-063 frame 0: Camera FOV 171, above 170. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-063 frame 4: Camera FOV 171, above 170. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-063: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-064 frame 0: Animation "spiral": not one of the eleven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-064 frame 4: Animation "spiral": not one of the eleven. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-064: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-065 frame 0: Animation "Explosive": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-065 frame 4: Animation "Explosive": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-065: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-066 frame 0: Particle Type "star": not one of the five (stars wait for D-612). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-066 frame 4: Particle Type "star": not one of the five (stars wait for D-612). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-066: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-067 frame 0: Floor Action "slide". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-067 frame 4: Floor Action "slide". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-067: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-068 frame 0: Birth Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-068 frame 4: Birth Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-068: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-069 frame 0: Composite With Original "yes": only "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-069 frame 4: Composite With Original "yes": only "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-069: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-070 frame 0: Transfer Mode "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-070 frame 4: Transfer Mode "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PSPACE-070: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PSPACE-071 frame 20: Custom as added, from the birth colour to the death colour: at a fifth of its life, close to FX-PSPACE-002 but not equal (the stops are spaced in 8-bit steps, the mix in linear light). | largest difference 2.8e-8 | yes |
| FX-PSPACE-071: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-072 frame 11: Opacity Map Fade In And Out (D-837), the one disc: at frame 11, 0.037 of its life, its centre 0.32 covered; at frame 20, 0.224 of its life, 0.96. | largest difference 1.5e-8 | yes |
| FX-PSPACE-072 frame 20: Opacity Map Fade In And Out (D-837), the one disc: at frame 11, 0.037 of its life, its centre 0.32 covered; at frame 20, 0.224 of its life, 0.96. | largest difference 2.7e-8 | yes |
| FX-PSPACE-072: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PSPACE-073 frame 12: Fade In And Out on the burst: the newest and oldest faint, the middle-aged as FX-PSPACE-024 has them. | largest difference 2.7e-8 | yes |
| FX-PSPACE-073: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PSPACE-024 frame 12: the 50 particles alive, each its number, birth, place and speed | 50 alive, largest difference 1.8e-15 | yes |
| FX-PSPACE-025 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-029 frame 12: the 50 particles alive, each its number, birth, place and speed | 50 alive, largest difference 1.8e-15 | yes |
| FX-PSPACE-031 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-032 frame 24: the 6 particles alive, each its number, birth, place and speed | 6 alive, largest difference 0.0e0 | yes |
| FX-PSPACE-036 frame 24: the 6 particles alive, each its number, birth, place and speed | 6 alive, largest difference 2.2e-16 | yes |
| FX-PSPACE-039 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-040 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-041 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-042 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-050 frame 30: the 42 particles alive, each its number, birth, place and speed | 42 alive, largest difference 3.6e-15 | yes |
| FX-PSPACE-052 frame 24: the 100 particles alive, each its number, birth, place and speed | 100 alive, largest difference 3.6e-15 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pspace_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_047.json is saved with its 59 settings, 44 numbers, 7 words and 8 colours, and no clock, camera or history | 59 settings; birth rate 0.2, particle type "shaded_sphere", camera FOV 20 | yes |
| fx_pspace_059.json is refused in a sentence naming birth rate | Particle Space's birth rate runs from 0 to 100, and this is -1. | yes |
| fx_pspace_060.json is refused in a sentence naming birth rate | Particle Space's birth rate runs from 0 to 100, and this is 101. | yes |
| fx_pspace_061.json is refused in a sentence naming longevity | Particle Space's longevity runs from 0 to 30, and this is 31. | yes |
| fx_pspace_062.json is refused in a sentence naming turbulence scale | Particle Space's turbulence scale runs from 0.01 to 10, and this is 0. | yes |
| fx_pspace_063.json is refused in a sentence naming camera fov | Particle Space's camera fov runs from 1 to 170, and this is 171. | yes |
| fx_pspace_064.json is refused in a sentence naming animation | Particle Space's animation is one of "explosive", "direction_axis", "cone_axis", "viscous", "twirl", "twirly", "vortex", "fire", "jet_sideways", "fractal_omni", "fractal_uni", and this is "spiral". | yes |
| fx_pspace_065.json is refused in a sentence naming animation | Particle Space's animation is one of "explosive", "direction_axis", "cone_axis", "viscous", "twirl", "twirly", "vortex", "fire", "jet_sideways", "fractal_omni", "fractal_uni", and this is "Explosive". | yes |
| fx_pspace_066.json is refused in a sentence naming particle type | Particle Space's particle type is one of "line", "faded_sphere", "shaded_sphere", "disc", "bubble", "textured_square", "textured_disc", "textured_faded_disc", and this is "star". | yes |
| fx_pspace_067.json is refused in a sentence naming floor action | Particle Space's floor action is one of "none", "ice", "glue", "bounce", and this is "slide". | yes |
| fx_pspace_068.json is refused in a sentence naming birth colour | Particle Space's birth colour is written #rrggbb, and this is "#12345". | yes |
| fx_pspace_069.json is refused in a sentence naming composite on original | Particle Space's composite on original is one of "off", "on", and this is "yes". | yes |
| fx_pspace_070.json is refused in a sentence naming transfer mode | Particle Space's transfer mode is one of "composite", "screen", "add", and this is "multiply". | yes |
| a file with a Particle Space whose birth rate is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Space whose animation is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Space without its depth cue colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pspace_024.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pspace_023.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pspace_046.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pspace_058.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-610)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pspace_001.json, frames [0, 12, 24] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_002.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_004.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_010.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_011.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_012.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_014.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_016.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_017.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_018.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_019.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_020.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_021.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_022.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_023.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_024.json, frames [12, 24, 40, 6] at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_025.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_026.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_027.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_028.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_029.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_030.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_031.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_032.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_033.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_034.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_035.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_036.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_037.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_038.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_039.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_040.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_041.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_042.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_043.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_044.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_045.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_046.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_047.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_048.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_049.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_050.json, frames [10, 20, 30] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_051.json, frames [10, 20] at Full and Draft | largest difference 1 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_052.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_053.json, frames [10, 22] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_054.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_055.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_056.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_057.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_058.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_059.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_060.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_061.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_062.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_063.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_064.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_065.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_066.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_067.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_068.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_069.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_070.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_071.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_072.json, frames [11, 20] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_pspace_073.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Space as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Particle Space as added on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space as added on three layers, frame 100, Full | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space as added on three layers, frame 239, Full | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space as added on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space as added on three layers, frame 100, Draft | largest difference 1 of 255, 6 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space as added on three layers, frame 239, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 10524 pixels changed | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing on three layers, frame 100, Full | largest difference 1 of 255, 1414 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing on three layers, frame 239, Full | largest difference 1 of 255, 1754 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space shaded spheres in a vortex, fog, over the drawing on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 9105 pixels changed | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4 on three layers, frame 239, Full | largest difference 1 of 255, 1708 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4 on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Space bubbles bouncing on the floor, screened, birth rate keyed 0 to 4 on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-610 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 2_fountain.png, frame 48: drawn without a warning, the street changed | 5258 pixels changed; warnings [] | yes |
| 3_fire.png, frame 48: drawn without a warning, the street changed | 5405 pixels changed; warnings [] | yes |
| 4_vortex.png, frame 72: drawn without a warning, the street changed | 18977 pixels changed; warnings [] | yes |
| 5_glue_floor.png, frame 72: drawn without a warning, the street changed | 19066 pixels changed; warnings [] | yes |
| 6_composition_camera_dollied_in.png, frame 48: drawn without a warning, the street changed | 71795 pixels changed; warnings [] | yes |

## Result

390 of 390 checks pass.
