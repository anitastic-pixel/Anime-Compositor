# B-717: Particle Space fades its particles in and out (D-837)

CC Particle World's Opacity Map, as added, fades each particle in at birth and out at death (five tutorials from five creators; the shape read off two of them at 18 points: life 0, 0.025, 0.05, 0.075, 0.1, 0.15, 0.2, 0.25, 0.3 at opacity 0, 0.24, 0.41, 0.55, 0.67, 0.84, 0.94, 0.98, 1, and mirrored). Ours drew every particle at Max Opacity all its life. Particle Space now has an Opacity Map, Fade In and Out when added and Constant for a file without it, on the processor and the card. The fixtures' numbers are the reference's.

## The fixtures, at the reference's numbers: FX-PSPACE-001 to 071 unmoved, 072 and 073 new

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

## The one disc's centre through frames 10 to 21: the map at its age

| Check | The build's answer | Matches |
| --- | --- | --- |
| frame 10, 0.016 of its life: 0.154 covered | 0.1538 | yes |
| frame 11, 0.037 of its life: 0.321 covered | 0.3206 | yes |
| frame 12, 0.058 of its life: 0.453 covered | 0.4531 | yes |
| frame 13, 0.079 of its life: 0.567 covered | 0.5669 | yes |
| frame 14, 0.099 of its life: 0.667 covered | 0.6669 | yes |
| frame 15, 0.120 of its life: 0.739 covered | 0.7387 | yes |
| frame 17, 0.162 of its life: 0.864 covered | 0.8637 | yes |
| frame 19, 0.204 of its life: 0.943 covered | 0.9428 | yes |
| frame 20, 0.224 of its life: 0.959 covered | 0.9595 | yes |
| frame 21, 0.245 of its life: 0.976 covered | 0.9762 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pspace_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pspace_072.json is saved with its Opacity Map | opacity_map "fade_in_and_out" | yes |
| A Particle Space added from the panel starts at Fade In and Out | fade_in_and_out | yes |
| Opacity Map "fade_in" is refused in a sentence naming opacity map | Particle Space's opacity map is one of "constant", "fade_in_and_out", and this is "fade_in". | yes |

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pspace_002.json, written before D-837 without an Opacity Map, opens with no warning and Constant | opacity map constant; warnings []; the file holds opacity_map: false | yes |
| and saves back holding what it held, no Opacity Map added | the same | yes |
| fx_pspace_024.json, written before D-837 without an Opacity Map, opens with no warning and Constant | opacity map constant; warnings []; the file holds opacity_map: false | yes |
| and saves back holding what it held, no Opacity Map added | the same | yes |
| fx_pspace_047.json, written before D-837 without an Opacity Map, opens with no warning and Constant | opacity map constant; warnings []; the file holds opacity_map: false | yes |
| and saves back holding what it held, no Opacity Map added | the same | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pspace_072.json, frames [11, 20] at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_pspace_073.json, frames [12] at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_pspace_024.json, frames [12] at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Result

211 of 211 checks pass.
