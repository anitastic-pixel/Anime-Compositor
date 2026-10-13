# D-615: Particle Emitter

B-495, the particle engine's fourth effect (P0-6, PLUGINS.md pick #20), after what Trapcode Particular is for: Particle Space's engine (D-610) with particles born from a point, a box, a ball or a grid of points in 3D, thrown out every way, one way, two ways, round a disc, or straight out or in, each with its own life, size and opacity changing over that life, coloured once or along a gradient, pushed by gravity, wind and swirling air, and bouncing, sliding or sticking on a floor; drawn as soft or glowing spheres, stars, faded clouds or streaks over a clear layer, through the composition camera when there is one. Nothing is carried from frame to frame: every frame works each particle out from its birth (document 20). The particles are worked out on the processor and drawn on the card too (D-617), within 1 level of the processor. Red Giant publishes no formula and no settings when added, so the rules, those settings and the name are ours, stated in `tools/particle_emitter_reference.py`. Every expected pixel is `Fixtures/particle_emitter/expected_particle_emitter.json` and every expected particle `Fixtures/particle_emitter/particles_particle_emitter.json`, written by the tool before this code existed and printed in document 25 as FX-PEMIT-001 to 063. Tolerances 2e-5 and 1e-7.

## FX-PEMIT-001 to 063 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PEMIT-001 frame 0: The settings as added, at frames 0, 6 and 12: nothing at frame 0, then 100 white soft dots a second thrown every way at about 100 pixels a second, so fast most are out of the 16 by 10 layer within frames; the layer's own picture is gone. | largest difference 0.0e0 | yes |
| FX-PEMIT-001 frame 12: The settings as added, at frames 0, 6 and 12: nothing at frame 0, then 100 white soft dots a second thrown every way at about 100 pixels a second, so fast most are out of the 16 by 10 layer within frames; the layer's own picture is gone. | largest difference 2.6e-8 | yes |
| FX-PEMIT-001 frame 6: The settings as added, at frames 0, 6 and 12: nothing at frame 0, then 100 white soft dots a second thrown every way at about 100 pixels a second, so fast most are out of the 16 by 10 layer within frames; the layer's own picture is gone. | largest difference 2.9e-8 | yes |
| FX-PEMIT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-002 frame 20: One particle, still, in the middle: a white Sphere 8 pixels across, feathered by half its radius. | largest difference 2.4e-8 | yes |
| FX-PEMIT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-003 frame 20: Sphere Feather 0: a hard-edged disc. | largest difference 1.2e-8 | yes |
| FX-PEMIT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-004 frame 20: Sphere Feather 100: soft from the middle out. | largest difference 2.6e-8 | yes |
| FX-PEMIT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-005 frame 20: A Glow Sphere: a solid core half as wide and a faint glow round it. | largest difference 1.0e-8 | yes |
| FX-PEMIT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-006 frame 20: A Star: five points up. | largest difference 2.8e-8 | yes |
| FX-PEMIT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-007 frame 20: A Cloudlet: soft all the way to its edge. | largest difference 4.9e-17 | yes |
| FX-PEMIT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-008 frame 20: A still Streak: no movement, so a round dot as wide as the size. | largest difference 1.2e-8 | yes |
| FX-PEMIT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-009 frame 20: A tiny sphere, size 1: drawn 1.4 pixels across and fainter by its true size. | largest difference 6.8e-9 | yes |
| FX-PEMIT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-010 frame 20: Size Over Life 100, 30, 100, 100, 100: at a fifth of its life it is shrinking towards the second level, about 37 per cent of its size. | largest difference 1.1e-8 | yes |
| FX-PEMIT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-011 frame 20: Opacity Over Life 0, 100, 100, 100, 100: fading in, about 90 per cent at a fifth of its life. | largest difference 1.8e-8 | yes |
| FX-PEMIT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-012 frame 20: Opacity 50: half covering. | largest difference 1.2e-8 | yes |
| FX-PEMIT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-013 frame 20: Opacity Over Life keyed: the second level from 100 at frame 0 to 10 at frame 20, read at the frame drawn. | largest difference 3.5e-9 | yes |
| FX-PEMIT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-014 frame 20: Opacity Random 80 on a spread of still dots: each its own opacity. | largest difference 2.5e-8 | yes |
| FX-PEMIT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-015 frame 20: Size Random 80 on the spread: each its own size. | largest difference 2.6e-8 | yes |
| FX-PEMIT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-016 frame 20: Life Random 90 on the spread: some gone early, at frames 20 and 40. | largest difference 2.9e-8 | yes |
| FX-PEMIT-016 frame 40: Life Random 90 on the spread: some gone early, at frames 20 and 40. | largest difference 3.0e-8 | yes |
| FX-PEMIT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-017 frame 20: At Birth colour blue. | largest difference 2.4e-8 | yes |
| FX-PEMIT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-018 frame 20: Over Life: at a fifth of its life, between white and yellow on the gradient. | largest difference 2.4e-8 | yes |
| FX-PEMIT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-019 frame 20: Random From Gradient on the spread: each dot a colour from white to dark red. | largest difference 2.9e-8 | yes |
| FX-PEMIT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-020 frame 20: Color Random 70 on the spread: each dot its own brightness. | largest difference 2.9e-8 | yes |
| FX-PEMIT-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-021 frame 20: A Sphere emitter 14 wide, 8 high and 30 deep: still dots in a ball, the far ones smaller and nearer the middle. | largest difference 2.5e-8 | yes |
| FX-PEMIT-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-022 frame 20: A Grid emitter 4 by 2 over 12 by 6 pixels: a dot a frame, filling the grid point by point in rows. | largest difference 9.5e-9 | yes |
| FX-PEMIT-022 frame 6: A Grid emitter 4 by 2 over 12 by 6 pixels: a dot a frame, filling the grid point by point in rows. | largest difference 1.2e-8 | yes |
| FX-PEMIT-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-023 frame 20: The grid turned 30 degrees about Z. | largest difference 2.7e-8 | yes |
| FX-PEMIT-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-024 frame 20: The particle 11 pixels farther away and up the left: smaller, drawn nearer the middle. | largest difference 2.3e-8 | yes |
| FX-PEMIT-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-025 frame 20: Two overlapping blue dots, Normal: the newer over the older. | largest difference 2.9e-8 | yes |
| FX-PEMIT-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-026 frame 20: The two, Add: brighter where they overlap. | largest difference 5.1e-8 | yes |
| FX-PEMIT-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-027 frame 20: The two, Screen. | largest difference 2.9e-8 | yes |
| FX-PEMIT-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-028 frame 12: A burst every way, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.9e-8 | yes |
| FX-PEMIT-028 frame 24: A burst every way, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.2e-8 | yes |
| FX-PEMIT-028 frame 40: A burst every way, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.7e-8 | yes |
| FX-PEMIT-028 frame 6: A burst every way, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the same as after frames 0 to 39. | largest difference 2.4e-8 | yes |
| FX-PEMIT-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-029 frame 12: Directional, spread 10: a jet upwards within 18 degrees. | largest difference 2.9e-8 | yes |
| FX-PEMIT-029 frame 24: Directional, spread 10: a jet upwards within 18 degrees. | largest difference 2.9e-8 | yes |
| FX-PEMIT-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-030 frame 12: The jet turned 90 degrees about Z: to the right. | largest difference 2.9e-8 | yes |
| FX-PEMIT-030 frame 24: The jet turned 90 degrees about Z: to the right. | largest difference 2.8e-8 | yes |
| FX-PEMIT-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-031 frame 12: Bi-Directional, spread 10: up and down at once. | largest difference 2.8e-8 | yes |
| FX-PEMIT-031 frame 24: Bi-Directional, spread 10: up and down at once. | largest difference 2.7e-8 | yes |
| FX-PEMIT-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-032 frame 12: Disc, the emitter turned 90 degrees about X so the axis faces us: a ring spreading out on the screen. | largest difference 2.6e-8 | yes |
| FX-PEMIT-032 frame 24: Disc, the emitter turned 90 degrees about X so the axis faces us: a ring spreading out on the screen. | largest difference 2.9e-8 | yes |
| FX-PEMIT-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-033 frame 12: Outwards from a Sphere emitter 6 across: away from its middle. | largest difference 2.9e-8 | yes |
| FX-PEMIT-033 frame 24: Outwards from a Sphere emitter 6 across: away from its middle. | largest difference 2.6e-8 | yes |
| FX-PEMIT-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-034 frame 12: Inwards into a Sphere emitter 14 across, slowly: towards its middle. | largest difference 2.4e-8 | yes |
| FX-PEMIT-034 frame 24: Inwards into a Sphere emitter 14 across, slowly: towards its middle. | largest difference 2.8e-8 | yes |
| FX-PEMIT-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-035 frame 12: Gravity 60 on the jet: it arcs back down. | largest difference 3.0e-8 | yes |
| FX-PEMIT-035 frame 24: Gravity 60 on the jet: it arcs back down. | largest difference 2.9e-8 | yes |
| FX-PEMIT-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-036 frame 12: Air Resistance 3: the burst slows and stays tight. | largest difference 2.6e-8 | yes |
| FX-PEMIT-036 frame 24: Air Resistance 3: the burst slows and stays tight. | largest difference 2.9e-8 | yes |
| FX-PEMIT-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-037 frame 12: Wind X 40 on the jet: it bends to the right. | largest difference 2.1e-8 | yes |
| FX-PEMIT-037 frame 24: Wind X 40 on the jet: it bends to the right. | largest difference 2.7e-8 | yes |
| FX-PEMIT-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-038 frame 12: Turbulence 400 at scale 4 on the jet: it wanders. | largest difference 2.4e-8 | yes |
| FX-PEMIT-038 frame 24: Turbulence 400 at scale 4 on the jet: it wanders. | largest difference 2.6e-8 | yes |
| FX-PEMIT-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-039 frame 12: Bounce on a floor at 80 per cent: a fountain from the top falls and bounces. | largest difference 2.9e-8 | yes |
| FX-PEMIT-039 frame 24: Bounce on a floor at 80 per cent: a fountain from the top falls and bounces. | largest difference 2.8e-8 | yes |
| FX-PEMIT-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-040 frame 24: Slide: they land and slide along the floor. | largest difference 3.0e-8 | yes |
| FX-PEMIT-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-041 frame 24: Stick: they land and stay. | largest difference 3.0e-8 | yes |
| FX-PEMIT-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-042 frame 12: Velocity From Motion 100 with the emitter keyed across: the particles carry its motion. | largest difference 2.6e-8 | yes |
| FX-PEMIT-042 frame 24: Velocity From Motion 100 with the emitter keyed across: the particles carry its motion. | largest difference 2.6e-8 | yes |
| FX-PEMIT-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-043 frame 10: Particles Per Second keyed from 0 at frame 0 to 110 at frame 20: births start slowly and quicken. | largest difference 2.6e-8 | yes |
| FX-PEMIT-043 frame 20: Particles Per Second keyed from 0 at frame 0 to 110 at frame 20: births start slowly and quicken. | largest difference 3.0e-8 | yes |
| FX-PEMIT-043 frame 30: Particles Per Second keyed from 0 at frame 0 to 110 at frame 20: births start slowly and quicken. | largest difference 2.9e-8 | yes |
| FX-PEMIT-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-044 frame 20: The ball of FX-PEMIT-021 through a composition camera 12 pixels in front: the far dots much smaller, the near ones larger. | largest difference 2.8e-8 | yes |
| FX-PEMIT-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-045 frame 10: FX-PEMIT-028 with the layer starting at frame 10: nothing before, frame 22 the same as FX-PEMIT-028's frame 12. | largest difference 0.0e0 | yes |
| FX-PEMIT-045 frame 22: FX-PEMIT-028 with the layer starting at frame 10: nothing before, frame 22 the same as FX-PEMIT-028's frame 12. | largest difference 2.9e-8 | yes |
| FX-PEMIT-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-046 frame 12: FX-PEMIT-028 with the layer moved 3 pixels right. | largest difference 2.9e-8 | yes |
| FX-PEMIT-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-047 frame 12: Random Seed 7: a different burst. | largest difference 2.9e-8 | yes |
| FX-PEMIT-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-048 frame 12: Particles Per Second 0: nothing. | largest difference 0.0e0 | yes |
| FX-PEMIT-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-049 frame 12: Life 0: nothing. | largest difference 0.0e0 | yes |
| FX-PEMIT-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-050 frame 12: Streaks on the burst: each a short line back along its last frame. | largest difference 3.0e-8 | yes |
| FX-PEMIT-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-051 frame 12: A burst of glowing sparks: Over Life colours, shrinking and fading, added. | largest difference 5.8e-8 | yes |
| FX-PEMIT-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PEMIT-052 frame 0: Particles Per Second -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-052 frame 4: Particles Per Second -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-053 frame 0: Life 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-053 frame 4: Life 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-054 frame 0: Size Over Life 3 at 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-054 frame 4: Size Over Life 3 at 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-055 frame 0: Opacity Over Life 2 at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-055 frame 4: Opacity Over Life 2 at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-056 frame 0: Emitter Type "light": not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-056 frame 4: Emitter Type "light": not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-057 frame 0: Direction "Uniform": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-057 frame 4: Direction "Uniform": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-058 frame 0: Particle Type "sprite": not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-058 frame 4: Particle Type "sprite": not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-059 frame 0: Set Color "from_layer". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-059 frame 4: Set Color "from_layer". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-060 frame 0: Transfer Mode "lighten". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-060 frame 4: Transfer Mode "lighten". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-061 frame 0: Floor Action "collide". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-061 frame 4: Floor Action "collide". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-062 frame 0: Gradient Color 2 "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-062 frame 4: Gradient Color 2 "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PEMIT-063 frame 0: Grid X 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-063 frame 4: Grid X 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PEMIT-063: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The particles themselves: where each is and how fast it goes

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PEMIT-016 frame 20: the 9 particles alive, each its number, birth, place and speed | 9 alive, largest difference 3.5e-18 | yes |
| FX-PEMIT-021 frame 20: the 10 particles alive, each its number, birth, place and speed | 10 alive, largest difference 2.8e-17 | yes |
| FX-PEMIT-022 frame 20: the 20 particles alive, each its number, birth, place and speed | 20 alive, largest difference 0.0e0 | yes |
| FX-PEMIT-023 frame 20: the 20 particles alive, each its number, birth, place and speed | 20 alive, largest difference 2.8e-17 | yes |
| FX-PEMIT-028 frame 12: the 12 particles alive, each its number, birth, place and speed | 12 alive, largest difference 2.2e-16 | yes |
| FX-PEMIT-029 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 4.4e-16 | yes |
| FX-PEMIT-030 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 4.4e-16 | yes |
| FX-PEMIT-031 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 4.4e-16 | yes |
| FX-PEMIT-032 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 2.2e-16 | yes |
| FX-PEMIT-033 frame 12: the 12 particles alive, each its number, birth, place and speed | 12 alive, largest difference 1.1e-16 | yes |
| FX-PEMIT-034 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 5.6e-17 | yes |
| FX-PEMIT-035 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 2.2e-16 | yes |
| FX-PEMIT-036 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 1.1e-16 | yes |
| FX-PEMIT-037 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 4.4e-16 | yes |
| FX-PEMIT-038 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 4.4e-16 | yes |
| FX-PEMIT-039 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 2.2e-16 | yes |
| FX-PEMIT-040 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 5.6e-17 | yes |
| FX-PEMIT-041 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 5.6e-17 | yes |
| FX-PEMIT-042 frame 24: the 24 particles alive, each its number, birth, place and speed | 24 alive, largest difference 1.1e-16 | yes |
| FX-PEMIT-043 frame 30: the 92 particles alive, each its number, birth, place and speed | 92 alive, largest difference 3.6e-15 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pemit_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pemit_051.json is saved with its 58 settings (45 numbers, the position, 6 words and 6 colours) and no clock, camera or history | 58 settings; emitter position [50,50], particle type "glow_sphere", set color "over_life", transfer mode "add" | yes |
| fx_pemit_052.json is refused in a sentence naming particles per second | Particle Emitter's particles per second runs from 0 to 100000, and this is -1. | yes |
| fx_pemit_053.json is refused in a sentence naming life | Particle Emitter's life runs from 0 to 1000, and this is 1001. | yes |
| fx_pemit_054.json is refused in a sentence naming size over life 3 | Particle Emitter's size over life 3 runs from 0 to 1000, and this is 1001. | yes |
| fx_pemit_055.json is refused in a sentence naming opacity over life 2 | Particle Emitter's opacity over life 2 runs from 0 to 100, and this is 101. | yes |
| fx_pemit_056.json is refused in a sentence naming emitter type | Particle Emitter's emitter type is one of "point", "box", "sphere", "grid", and this is "light". | yes |
| fx_pemit_057.json is refused in a sentence naming direction | Particle Emitter's direction is one of "uniform", "directional", "bi_directional", "disc", "outwards", "inwards", and this is "Uniform". | yes |
| fx_pemit_058.json is refused in a sentence naming particle type | Particle Emitter's particle type is one of "sphere", "glow_sphere", "star", "cloudlet", "streak", and this is "sprite". | yes |
| fx_pemit_059.json is refused in a sentence naming set color | Particle Emitter's set color is one of "at_birth", "over_life", "random_from_gradient", and this is "from_layer". | yes |
| fx_pemit_060.json is refused in a sentence naming transfer mode | Particle Emitter's transfer mode is one of "normal", "add", "screen", and this is "lighten". | yes |
| fx_pemit_061.json is refused in a sentence naming floor action | Particle Emitter's floor action is one of "none", "bounce", "slide", "stick", and this is "collide". | yes |
| fx_pemit_062.json is refused in a sentence naming gradient colour 2 | Particle Emitter's gradient colour 2 is written #rrggbb, and this is "#12345". | yes |
| fx_pemit_063.json is refused in a sentence naming grid x | Particle Emitter's grid x runs from 1 to 1000, and this is 0. | yes |
| a file with a Particle Emitter whose velocity is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Emitter whose particle type is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Emitter without its fifth gradient colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pemit_016.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pemit_022.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pemit_039.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pemit_044.json frame 20 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-615 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 1818 pixels changed; warnings [] | yes |
| 2_fountain.png, frame 48: drawn without a warning, the street changed | 7874 pixels changed; warnings [] | yes |
| 3_sparks_on_a_floor.png, frame 48: drawn without a warning, the street changed | 6058 pixels changed; warnings [] | yes |
| 4_snow.png, frame 72: drawn without a warning, the street changed | 3101 pixels changed; warnings [] | yes |
| 5_streaks_outwards.png, frame 24: drawn without a warning, the street changed | 10310 pixels changed; warnings [] | yes |
| 6_grid_of_cloudlets.png, frame 36: drawn without a warning, the street changed | 32186 pixels changed; warnings [] | yes |
| 7_stars_turned.png, frame 48: drawn without a warning, the street changed | 3758 pixels changed; warnings [] | yes |

## On the card (D-617)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PEMIT-001, fx_pemit_001.json, frames [0, 12, 6] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-002, fx_pemit_002.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-003, fx_pemit_003.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-004, fx_pemit_004.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-005, fx_pemit_005.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-006, fx_pemit_006.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-007, fx_pemit_007.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-008, fx_pemit_008.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-009, fx_pemit_009.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-010, fx_pemit_010.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-011, fx_pemit_011.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-012, fx_pemit_012.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-013, fx_pemit_013.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-014, fx_pemit_014.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-015, fx_pemit_015.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-016, fx_pemit_016.json, frames [20, 40] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-017, fx_pemit_017.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-018, fx_pemit_018.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-019, fx_pemit_019.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-020, fx_pemit_020.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-021, fx_pemit_021.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-022, fx_pemit_022.json, frames [20, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-023, fx_pemit_023.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-024, fx_pemit_024.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-025, fx_pemit_025.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-026, fx_pemit_026.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-027, fx_pemit_027.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-028, fx_pemit_028.json, frames [12, 24, 40, 6] at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-029, fx_pemit_029.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-030, fx_pemit_030.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-031, fx_pemit_031.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-032, fx_pemit_032.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-033, fx_pemit_033.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-034, fx_pemit_034.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-035, fx_pemit_035.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-036, fx_pemit_036.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-037, fx_pemit_037.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-038, fx_pemit_038.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-039, fx_pemit_039.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-040, fx_pemit_040.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-041, fx_pemit_041.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-042, fx_pemit_042.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-043, fx_pemit_043.json, frames [10, 20, 30] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-044, fx_pemit_044.json, frames [20] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-045, fx_pemit_045.json, frames [10, 22] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-046, fx_pemit_046.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-047, fx_pemit_047.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-048, fx_pemit_048.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-049, fx_pemit_049.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-050, fx_pemit_050.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-051, fx_pemit_051.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-052, fx_pemit_052.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-053, fx_pemit_053.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-054, fx_pemit_054.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-055, fx_pemit_055.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-056, fx_pemit_056.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-057, fx_pemit_057.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-058, fx_pemit_058.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-059, fx_pemit_059.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-060, fx_pemit_060.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-061, fx_pemit_061.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-062, fx_pemit_062.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PEMIT-063, fx_pemit_063.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Emitter as added: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, Particle Emitter as added, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Emitter as added, frame 100, Full | largest difference 1 of 255, 12 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Emitter as added, frame 239, Full | largest difference 1 of 255, 19 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Emitter as added, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Emitter as added, frame 100, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Emitter as added, frame 239, Draft | largest difference 1 of 255, 453 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing spheres added, turbulence: the processor's frame 100 differs from the shot without it | 1967426 pixels changed | yes |
| the reference shot, glowing spheres added, turbulence, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing spheres added, turbulence, frame 100, Full | largest difference 1 of 255, 178 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing spheres added, turbulence, frame 239, Full | largest difference 1 of 255, 260 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing spheres added, turbulence, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing spheres added, turbulence, frame 100, Draft | largest difference 1 of 255, 521 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing spheres added, turbulence, frame 239, Draft | largest difference 1 of 255, 450 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars screened from a box: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, stars screened from a box, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars screened from a box, frame 100, Full | largest difference 1 of 255, 117 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars screened from a box, frame 239, Full | largest difference 1 of 255, 107 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars screened from a box, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars screened from a box, frame 100, Draft | largest difference 1 of 255, 514 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars screened from a box, frame 239, Draft | largest difference 1 of 255, 455 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets bouncing on a floor: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, cloudlets bouncing on a floor, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets bouncing on a floor, frame 100, Full | largest difference 1 of 255, 466 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets bouncing on a floor, frame 239, Full | largest difference 1 of 255, 107 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets bouncing on a floor, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets bouncing on a floor, frame 100, Draft | largest difference 1 of 255, 459 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets bouncing on a floor, frame 239, Draft | largest difference 1 of 255, 454 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, streaks from a ball: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, streaks from a ball, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, streaks from a ball, frame 100, Full | largest difference 1 of 255, 119 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, streaks from a ball, frame 239, Full | largest difference 1 of 255, 103 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, streaks from a ball, frame 0, Draft | largest difference 1 of 255, 525 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, streaks from a ball, frame 100, Draft | largest difference 1 of 255, 494 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, streaks from a ball, frame 239, Draft | largest difference 1 of 255, 450 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |

## Result

368 of 368 checks pass.
