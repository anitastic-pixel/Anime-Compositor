# D-616: Particle Grid

B-496, the particle engine's fifth effect (P0-6, PLUGINS.md's parked pick), after what Trapcode Form is for: particles that stay put, in a grid of points or on layered spheres in 3D, turned and moved as one, each point nudged its own way, the whole twisted and rippled by a noise field that can drift and change, coloured, sized and pushed towards the camera by other layers; drawn as soft or glowing spheres, stars or faded clouds over a clear layer, through the composition camera when there is one. Nothing is carried from frame to frame: the drift is the sum of the Flow settings over the frames from the in point, each worked from the settings alone (document 20). The points are worked out on the processor and drawn on the card too (D-618), within 1 level of the processor. Red Giant publishes no formula and no settings when added, so the rules, those settings and the name are ours, stated in `tools/particle_grid_reference.py`. Every expected pixel is `Fixtures/particle_grid/expected_particle_grid.json` and every expected point `Fixtures/particle_grid/particles_particle_grid.json`, written by the tool before this code existed and printed in document 25 as FX-PGRID-001 to 060. Tolerances 2e-5 and 1e-7.

## FX-PGRID-001 to 060 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PGRID-001 frame 0: The settings as added, at frames 0 and 12: a grid 41 by 31 points, 20 pixels apart, so only its middle point lands on this 16 by 10 layer, a white soft dot 4 pixels across in the middle; still; the layer's own picture is gone. | largest difference 1.9e-8 | yes |
| FX-PGRID-001 frame 12: The settings as added, at frames 0 and 12: a grid 41 by 31 points, 20 pixels apart, so only its middle point lands on this 16 by 10 layer, a white soft dot 4 pixels across in the middle; still; the layer's own picture is gone. | largest difference 1.9e-8 | yes |
| FX-PGRID-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-002 frame 0: A small grid: 4 by 2 white dots, 2 pixels across. | largest difference 1.9e-9 | yes |
| FX-PGRID-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-003 frame 0: A finer grid: 5 by 3 dots 1.5 pixels across, 3 pixels apart. | largest difference 1.2e-8 | yes |
| FX-PGRID-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-004 frame 0: Sphere Feather 0 on big dots: hard edges. | largest difference 2.7e-8 | yes |
| FX-PGRID-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-005 frame 0: Glow Spheres on the big dots: solid cores and faint glows. | largest difference 2.3e-8 | yes |
| FX-PGRID-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-006 frame 0: Stars: 2 by 1 points, 6 pixels across, five points up. | largest difference 2.4e-8 | yes |
| FX-PGRID-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-007 frame 0: Cloudlets on the big dots: soft to the edge. | largest difference 1.8e-8 | yes |
| FX-PGRID-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-008 frame 0: Size 1 on the small grid: drawn 1.4 pixels across, fainter by the true size. | largest difference 4.9e-9 | yes |
| FX-PGRID-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-009 frame 0: Size Random 80 on the fine grid: each dot its own size. | largest difference 3.0e-8 | yes |
| FX-PGRID-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-010 frame 0: Opacity 60: every dot 60 per cent. | largest difference 2.4e-8 | yes |
| FX-PGRID-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-011 frame 0: Opacity Random 80 on the fine grid: each dot its own opacity. | largest difference 1.8e-8 | yes |
| FX-PGRID-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-012 frame 0: Color orange. | largest difference 1.9e-9 | yes |
| FX-PGRID-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-013 frame 0: Big overlapping blue dots, Normal: later points over earlier. | largest difference 2.3e-8 | yes |
| FX-PGRID-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-014 frame 0: The same, Add: brighter where they overlap. | largest difference 5.7e-8 | yes |
| FX-PGRID-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-015 frame 0: The same, Screen. | largest difference 2.3e-8 | yes |
| FX-PGRID-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-016 frame 0: The fine grid turned 30 degrees about Z. | largest difference 2.7e-8 | yes |
| FX-PGRID-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-017 frame 0: The fine grid turned 60 degrees about Y: narrower, the right side farther and smaller. | largest difference 2.8e-8 | yes |
| FX-PGRID-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-018 frame 0: The deep grid tipped 50 degrees about X: its three layers seen from above. | largest difference 2.9e-8 | yes |
| FX-PGRID-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-019 frame 0: The deep grid straight on: the far layers smaller and drawn nearer the middle, the front layer over them. | largest difference 2.7e-8 | yes |
| FX-PGRID-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-020 frame 0: The small grid moved up and left, and 10 pixels back. | largest difference 2.7e-8 | yes |
| FX-PGRID-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-021 frame 0: Sphere Layered: one shell, 8 points round on 5 rings, 8 pixels across. | largest difference 2.9e-8 | yes |
| FX-PGRID-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-022 frame 0: Two shells: an inner ball half the size inside. | largest difference 2.9e-8 | yes |
| FX-PGRID-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-023 frame 0: The ball turning: Form Rotation Y keyed from 0 at frame 0 to 80 at frame 24, at frames 0, 12 and 24. | largest difference 3.0e-8 | yes |
| FX-PGRID-023 frame 12: The ball turning: Form Rotation Y keyed from 0 at frame 0 to 80 at frame 24, at frames 0, 12 and 24. | largest difference 2.9e-8 | yes |
| FX-PGRID-023 frame 24: The ball turning: Form Rotation Y keyed from 0 at frame 0 to 80 at frame 24, at frames 0, 12 and 24. | largest difference 2.8e-8 | yes |
| FX-PGRID-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-024 frame 0: Disperse 1.5: each dot nudged its own way, the grid loosened. | largest difference 2.7e-8 | yes |
| FX-PGRID-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-025 frame 0: Disperse 1.5, Random Seed 7: nudged differently. | largest difference 2.9e-8 | yes |
| FX-PGRID-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-026 frame 0: Twist 120: the top row turned one way about the up line, the bottom row the other, so the rows narrow and widen. | largest difference 2.9e-8 | yes |
| FX-PGRID-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-027 frame 0: Fractal displace 2 at scale 6: the grid rippled. | largest difference 2.3e-8 | yes |
| FX-PGRID-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-028 frame 0: The ripple drifting: Flow X 12 pixels a second, at frames 0, 12 and 24; the field slides through the grid. | largest difference 2.3e-8 | yes |
| FX-PGRID-028 frame 12: The ripple drifting: Flow X 12 pixels a second, at frames 0, 12 and 24; the field slides through the grid. | largest difference 2.8e-8 | yes |
| FX-PGRID-028 frame 24: The ripple drifting: Flow X 12 pixels a second, at frames 0, 12 and 24; the field slides through the grid. | largest difference 2.9e-8 | yes |
| FX-PGRID-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-029 frame 0: The ripple changing: Flow Evolution 1 a second, at frames 0 and 12. | largest difference 2.3e-8 | yes |
| FX-PGRID-029 frame 12: The ripple changing: Flow Evolution 1 a second, at frames 0 and 12. | largest difference 2.9e-8 | yes |
| FX-PGRID-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-030 frame 0: Evolution 3: a different ripple, still. | largest difference 2.9e-8 | yes |
| FX-PGRID-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-031 frame 0: Affect Size 150 with no displacement: the field grows and shrinks the dots in place. | largest difference 2.9e-8 | yes |
| FX-PGRID-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-032 frame 0: Affect Opacity 100: the field fades dots in place. | largest difference 2.8e-8 | yes |
| FX-PGRID-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-033 frame 12: Flow Y keyed from 0 at frame 0 to 24 at frame 24: the drift speeds up, by frame 24 moved 12 pixels' worth, at frames 12 and 24. | largest difference 3.0e-8 | yes |
| FX-PGRID-033 frame 24: Flow Y keyed from 0 at frame 0 to 24 at frame 24: the drift speeds up, by frame 24 moved 12 pixels' worth, at frames 12 and 24. | largest difference 2.9e-8 | yes |
| FX-PGRID-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-034 frame 0: Color Map the ramp: the dots red to the right, green lower down. | largest difference 1.9e-8 | yes |
| FX-PGRID-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-035 frame 0: Color Map half blue, half clear: only the left half's dots, blue. | largest difference 1.3e-8 | yes |
| FX-PGRID-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-036 frame 0: Size Map the spot: big dots in the middle, none at the edges. | largest difference 9.8e-9 | yes |
| FX-PGRID-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-037 frame 0: Displace Map the spot, Map Displace 6: the middle pushed towards the camera, larger and spread out. | largest difference 2.9e-8 | yes |
| FX-PGRID-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-038 frame 0: Map Displace -6: the middle pushed away, smaller and drawn in. | largest difference 3.0e-8 | yes |
| FX-PGRID-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-039 frame 0: All three maps at once on the tipped grid. | largest difference 2.6e-8 | yes |
| FX-PGRID-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-040 frame 0: The deep grid through a composition camera 12 pixels in front: the far layers much smaller, the near ones larger. | largest difference 2.5e-8 | yes |
| FX-PGRID-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-041 frame 10: FX-PGRID-028 with the layer starting at frame 10: frame 10 as its frame 0, frame 22 as its frame 12. | largest difference 2.3e-8 | yes |
| FX-PGRID-041 frame 22: FX-PGRID-028 with the layer starting at frame 10: frame 10 as its frame 0, frame 22 as its frame 12. | largest difference 2.8e-8 | yes |
| FX-PGRID-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-042 frame 0: FX-PGRID-027 with the layer moved 3 pixels right. | largest difference 2.3e-8 | yes |
| FX-PGRID-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-043 frame 0: FX-PGRID-027 with Random Seed 7: a different ripple. | largest difference 3.0e-8 | yes |
| FX-PGRID-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-044 frame 21: Particles In X keyed from 2 at frame 0 to 6 at frame 24, whole points only: 3 across at frame 9, 5 at frame 21. | largest difference 1.2e-8 | yes |
| FX-PGRID-044 frame 9: Particles In X keyed from 2 at frame 0 to 6 at frame 24, whole points only: 3 across at frame 9, 5 at frame 21. | largest difference 1.2e-8 | yes |
| FX-PGRID-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-045 frame 0: Size 0: nothing. | largest difference 0.0e0 | yes |
| FX-PGRID-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-046 frame 0: Opacity 0: nothing. | largest difference 0.0e0 | yes |
| FX-PGRID-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-047 frame 0: Form Size 0: all eight dots on one spot in the middle, piled up. | largest difference 2.9e-8 | yes |
| FX-PGRID-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-048 frame 0: One point: a single dot in the middle. | largest difference 1.9e-8 | yes |
| FX-PGRID-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-049 frame 12: A glowing grid coloured by the ramp, rippling and added. | largest difference 2.9e-8 | yes |
| FX-PGRID-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PGRID-050 frame 0: Particles In X 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-050 frame 4: Particles In X 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-051 frame 0: Particles In Z 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-051 frame 4: Particles In Z 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-052 frame 0: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-052 frame 4: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-053 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-053 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-054 frame 0: Fractal Scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-054 frame 4: Fractal Scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-055 frame 0: Base Form "text": not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-055 frame 4: Base Form "text": not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-056 frame 0: Base Form "Box_Grid": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-056 frame 4: Base Form "Box_Grid": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-057 frame 0: Particle Type "streak": a grid does not move, so no streaks. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-057 frame 4: Particle Type "streak": a grid does not move, so no streaks. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-058 frame 0: Transfer Mode "lighten". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-058 frame 4: Transfer Mode "lighten". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-059 frame 0: Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-059 frame 4: Color "#12345": not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PGRID-060 frame 0: Flow Evolution 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-060 frame 4: Flow Evolution 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PGRID-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The points themselves: where each is, its size and its opacity

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PGRID-016 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-018 frame 0: the 45 points drawn, each its number, place, size and opacity | 45 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-021 frame 0: the 40 points drawn, each its number, place, size and opacity | 40 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-022 frame 0: the 80 points drawn, each its number, place, size and opacity | 80 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-023 frame 12: the 40 points drawn, each its number, place, size and opacity | 40 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-024 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 5.6e-17 | yes |
| FX-PGRID-026 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-028 frame 24: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-029 frame 12: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 5.6e-17 | yes |
| FX-PGRID-031 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 4.4e-16 | yes |
| FX-PGRID-032 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 1.1e-16 | yes |
| FX-PGRID-033 frame 24: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-034 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-036 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.2e-16 | yes |
| FX-PGRID-037 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.8e-17 | yes |
| FX-PGRID-039 frame 0: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 2.2e-16 | yes |
| FX-PGRID-041 frame 22: the 15 points drawn, each its number, place, size and opacity | 15 drawn, largest difference 5.6e-17 | yes |
| FX-PGRID-044 frame 9: the 9 points drawn, each its number, place, size and opacity | 9 drawn, largest difference 0.0e0 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pgrid_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pgrid_049.json is saved with its 36 settings (28 numbers, the centre, 3 words, 3 layers and the colour) and no maps, clock, camera or history | 36 settings; form center [50,50], particle type "glow_sphere", transfer mode "add", color map "ramp" | yes |
| fx_pgrid_050.json is refused in a sentence naming particles in x | Particle Grid's particles in x runs from 1 to 1000, and this is 0. | yes |
| fx_pgrid_051.json is refused in a sentence naming particles in z | Particle Grid's particles in z runs from 1 to 100, and this is 101. | yes |
| fx_pgrid_052.json is refused in a sentence naming size | Particle Grid's size runs from 0 to 10000, and this is -1. | yes |
| fx_pgrid_053.json is refused in a sentence naming opacity | Particle Grid's opacity runs from 0 to 100, and this is 101. | yes |
| fx_pgrid_054.json is refused in a sentence naming fractal scale | Particle Grid's fractal scale runs from 1 to 100000, and this is 0. | yes |
| fx_pgrid_055.json is refused in a sentence naming base form | Particle Grid's base form is one of "box_grid", "sphere_layered", and this is "text". | yes |
| fx_pgrid_056.json is refused in a sentence naming base form | Particle Grid's base form is one of "box_grid", "sphere_layered", and this is "Box_Grid". | yes |
| fx_pgrid_057.json is refused in a sentence naming particle type | Particle Grid's particle type is one of "sphere", "glow_sphere", "star", "cloudlet", and this is "streak". | yes |
| fx_pgrid_058.json is refused in a sentence naming transfer mode | Particle Grid's transfer mode is one of "normal", "add", "screen", and this is "lighten". | yes |
| fx_pgrid_059.json is refused in a sentence naming colour | Particle Grid's colour is written #rrggbb, and this is "#12345". | yes |
| fx_pgrid_060.json is refused in a sentence naming flow evolution | Particle Grid's flow evolution runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Particle Grid whose size is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Grid whose base form is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Particle Grid without its displace map is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pgrid_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pgrid_034.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pgrid_039.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pgrid_040.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures (verification/D-616 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png, frame 0: drawn without a warning, the street changed | 4992 pixels changed; warnings [] | yes |
| 2_rippling_field.png, frame 48: drawn without a warning, the street changed | 12161 pixels changed; warnings [] | yes |
| 3_layered_ball_turning.png, frame 0: drawn without a warning, the street changed | 23110 pixels changed; warnings [] | yes |
| 4_coloured_by_the_street.png, frame 0: drawn without a warning, the street changed | 14926 pixels changed; warnings [] | yes |
| 5_pushed_by_the_street.png, frame 0: drawn without a warning, the street changed | 33221 pixels changed; warnings [] | yes |
| 6_twisted_stars.png, frame 0: drawn without a warning, the street changed | 6940 pixels changed; warnings [] | yes |
| 7_cloudlets.png, frame 0: drawn without a warning, the street changed | 88374 pixels changed; warnings [] | yes |

## On the card (D-618)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PGRID-001, fx_pgrid_001.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-002, fx_pgrid_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-003, fx_pgrid_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-004, fx_pgrid_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-005, fx_pgrid_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-006, fx_pgrid_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-007, fx_pgrid_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-008, fx_pgrid_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-009, fx_pgrid_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-010, fx_pgrid_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-011, fx_pgrid_011.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-012, fx_pgrid_012.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-013, fx_pgrid_013.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-014, fx_pgrid_014.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-015, fx_pgrid_015.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-016, fx_pgrid_016.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-017, fx_pgrid_017.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-018, fx_pgrid_018.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-019, fx_pgrid_019.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-020, fx_pgrid_020.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-021, fx_pgrid_021.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-022, fx_pgrid_022.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-023, fx_pgrid_023.json, frames [0, 12, 24] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-024, fx_pgrid_024.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-025, fx_pgrid_025.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-026, fx_pgrid_026.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-027, fx_pgrid_027.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-028, fx_pgrid_028.json, frames [0, 12, 24] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-029, fx_pgrid_029.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-030, fx_pgrid_030.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-031, fx_pgrid_031.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-032, fx_pgrid_032.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-033, fx_pgrid_033.json, frames [12, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-034, fx_pgrid_034.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-035, fx_pgrid_035.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-036, fx_pgrid_036.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-037, fx_pgrid_037.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-038, fx_pgrid_038.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-039, fx_pgrid_039.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-040, fx_pgrid_040.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-041, fx_pgrid_041.json, frames [10, 22] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-042, fx_pgrid_042.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-043, fx_pgrid_043.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-044, fx_pgrid_044.json, frames [21, 9] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-045, fx_pgrid_045.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-046, fx_pgrid_046.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-047, fx_pgrid_047.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-048, fx_pgrid_048.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-049, fx_pgrid_049.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-050, fx_pgrid_050.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-051, fx_pgrid_051.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-052, fx_pgrid_052.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-053, fx_pgrid_053.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-054, fx_pgrid_054.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-055, fx_pgrid_055.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-056, fx_pgrid_056.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-057, fx_pgrid_057.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-058, fx_pgrid_058.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-059, fx_pgrid_059.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-PGRID-060, fx_pgrid_060.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Particle Grid as added: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, Particle Grid as added, frame 0, Full | largest difference 1 of 255, 10 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Grid as added, frame 100, Full | largest difference 1 of 255, 46 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Grid as added, frame 239, Full | largest difference 1 of 255, 11 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Grid as added, frame 0, Draft | largest difference 1 of 255, 514 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Grid as added, frame 100, Draft | largest difference 1 of 255, 491 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Particle Grid as added, frame 239, Draft | largest difference 1 of 255, 436 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing points rippling and drifting, added: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, glowing points rippling and drifting, added, frame 0, Full | largest difference 1 of 255, 434 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing points rippling and drifting, added, frame 100, Full | largest difference 1 of 255, 578 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing points rippling and drifting, added, frame 239, Full | largest difference 1 of 255, 428 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing points rippling and drifting, added, frame 0, Draft | largest difference 1 of 255, 527 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing points rippling and drifting, added, frame 100, Draft | largest difference 1 of 255, 479 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, glowing points rippling and drifting, added, frame 239, Draft | largest difference 1 of 255, 444 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars on a layered sphere, screened: the processor's frame 100 differs from the shot without it | 1967427 pixels changed | yes |
| the reference shot, stars on a layered sphere, screened, frame 0, Full | largest difference 1 of 255, 427 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars on a layered sphere, screened, frame 100, Full | largest difference 1 of 255, 508 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars on a layered sphere, screened, frame 239, Full | largest difference 1 of 255, 393 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars on a layered sphere, screened, frame 0, Draft | largest difference 1 of 255, 553 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars on a layered sphere, screened, frame 100, Draft | largest difference 1 of 255, 445 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, stars on a layered sphere, screened, frame 239, Draft | largest difference 1 of 255, 479 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer: the processor's frame 100 differs from the shot without it | 1967431 pixels changed | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer, frame 0, Full | largest difference 1 of 255, 61 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer, frame 100, Full | largest difference 1 of 255, 75 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer, frame 0, Draft | largest difference 1 of 255, 534 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer, frame 100, Draft | largest difference 1 of 255, 480 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, cloudlets tipped, coloured and pushed by a layer, frame 239, Draft | largest difference 1 of 255, 448 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |

## Result

332 of 332 checks pass.
