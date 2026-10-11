# D-551: CC Ball Action

B-431, after CycoreFX's CC Ball Action (Simulation): the layer as a grid of lit balls, each the colour under its cell's centre, turned about a rotation axis, twisted ball by ball by a twist property and scattered, in a fake 3D with its own eye (D-530's 50 mm lens on a 36 mm frame) and its own light from the upper left (P0-12: no comp camera or 3D layers). Only completely opaque pixels make balls. The CycoreFX manual names the controls but publishes no formula, no lighting and no settings when added, so the rule and those settings are ours, stated in `tools/cc_ball_action_reference.py`. Every expected pixel is `Fixtures/cc_ball_action/expected_cc_ball_action.json`, written by the tool before this code existed and printed in document 25 as FX-BALL-001 to 045. Tolerance 2e-5.

## FX-BALL-001 to 045 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BALL-001 frame 0: The settings as they start: grid spacing 8, so two cells across and one down, each a ball 8 pixels wide of the colour under its centre (yellow on the left, the warm shadow on the right), lit from the upper left, with transparent round them. | largest difference 6.7e-8 | yes |
| FX-BALL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-002 frame 0: Grid spacing 4: four balls across and two down, touching. | largest difference 1.2e-7 | yes |
| FX-BALL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-003 frame 0: Grid spacing 3: five across and three down, half a pixel in from each edge. | largest difference 1.1e-7 | yes |
| FX-BALL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-004 frame 0: Grid spacing 5: three across and two down; the left cell's centre falls on a covered column. | largest difference 8.2e-8 | yes |
| FX-BALL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-005 frame 0: Grid spacing 2, the least: a ball on every other column and row, seven across and five down, the half-covered last column making none. | largest difference 9.2e-8 | yes |
| FX-BALL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-006 frame 0: Ball size 50: the balls half as wide as their cells, with gaps between. | largest difference 9.2e-8 | yes |
| FX-BALL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-007 frame 0: Ball size 160: the balls overlap, each later one in its row and each lower row in front at the same depth. | largest difference 1.4e-7 | yes |
| FX-BALL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-008 frame 0: Ball size 0: no balls, the layer transparent. | largest difference 0.0e0 | yes |
| FX-BALL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-009 frame 0: Rotation 40 about X: the grid tips back at the top, its upper row farther and smaller, its lower row nearer and larger. | largest difference 1.4e-7 | yes |
| FX-BALL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-010 frame 0: Rotation 60 about Y: the grid swings, its left side nearer and larger, its right farther, the columns closing up. | largest difference 1.4e-7 | yes |
| FX-BALL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-011 frame 0: Rotation 30 about Z: the grid turns flat in the picture. | largest difference 1.4e-7 | yes |
| FX-BALL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-012 frame 0: Rotation 35 about XY. | largest difference 1.3e-7 | yes |
| FX-BALL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-013 frame 0: Rotation 35 about XZ. | largest difference 1.3e-7 | yes |
| FX-BALL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-014 frame 0: Rotation 35 about YZ. | largest difference 1.1e-7 | yes |
| FX-BALL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-015 frame 0: Rotation 25 about XYZ. | largest difference 1.5e-7 | yes |
| FX-BALL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-016 frame 0: Rotation 8 about X15Z: 8 degrees about X and 120 about Z. | largest difference 1.3e-7 | yes |
| FX-BALL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-017 frame 0: Rotation 6 about XY15Z: 6 degrees about X and Y and 90 about Z. | largest difference 1.2e-7 | yes |
| FX-BALL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-018 frame 0: Rotation 90 about Y: the grid edge on, every ball in one column, the nearest in front. | largest difference 6.6e-8 | yes |
| FX-BALL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-019 frame 0: Twist X Axis, 80 degrees about Y: each column turned by its place across, the left ones one way and the right ones the other. | largest difference 1.1e-7 | yes |
| FX-BALL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-020 frame 0: Twist Y Axis, 80 degrees about X: each row turned by its place down. | largest difference 1.4e-7 | yes |
| FX-BALL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-021 frame 0: Twist Center-X: the columns farther from the middle turned more. | largest difference 1.1e-7 | yes |
| FX-BALL-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-022 frame 0: Twist Center-Y, about X: the rows farther from the middle turned more. | largest difference 1.5e-7 | yes |
| FX-BALL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-023 frame 0: Twist Radius: the balls near the middle turned most. | largest difference 1.4e-7 | yes |
| FX-BALL-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-024 frame 0: Twist Random: each ball turned by its own fixed amount. | largest difference 1.4e-7 | yes |
| FX-BALL-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-025 frame 0: Twist Red: the balls with least red turned most. | largest difference 7.0e-7 | yes |
| FX-BALL-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-026 frame 0: Twist Green. | largest difference 7.0e-7 | yes |
| FX-BALL-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-027 frame 0: Twist Blue. | largest difference 7.0e-7 | yes |
| FX-BALL-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-028 frame 0: Twist Brightness: the darkest balls turned most. | largest difference 7.0e-7 | yes |
| FX-BALL-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-029 frame 0: Twist Diamond. | largest difference 1.3e-7 | yes |
| FX-BALL-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-030 frame 0: Twist Rectangle. | largest difference 1.2e-7 | yes |
| FX-BALL-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-031 frame 0: Twist Fast Top: each ball's place turned about X and twice as fast about Y, most near the middle. | largest difference 1.0e-7 | yes |
| FX-BALL-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-032 frame 0: Scatter 3: each ball moved up to 3 pixels its own way, nearer ones larger. | largest difference 1.5e-7 | yes |
| FX-BALL-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-033 frame 0: Scatter 3 with Instability State 120: each ball's scatter turned 120 degrees about its own axis. | largest difference 1.2e-7 | yes |
| FX-BALL-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-034 frame 0: Instability State 120 with Scatter 0: nothing scattered, so the same as FX-BALL-002. | largest difference 1.2e-7 | yes |
| FX-BALL-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-035 frame 0: Rotation keyed from 0 at frame 0 to 80 at frame 4, linear, about X. | largest difference 1.2e-7 | yes |
| FX-BALL-035 frame 2: Rotation keyed from 0 at frame 0 to 80 at frame 4, linear, about X. | largest difference 1.4e-7 | yes |
| FX-BALL-035 frame 4: Rotation keyed from 0 at frame 0 to 80 at frame 4, linear, about X. | largest difference 1.4e-7 | yes |
| FX-BALL-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-036 frame 0: FX-BALL-002 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-BALL-036 frame 3: FX-BALL-002 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-BALL-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-037 frame 0: Rotation 360 about X: once round, the same as FX-BALL-002. | largest difference 1.2e-7 | yes |
| FX-BALL-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALL-038 frame 0: Scatter -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-038 frame 4: Scatter -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-039 frame 0: Scatter 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-039 frame 4: Scatter 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-040 frame 0: Grid spacing 1.5, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-040 frame 4: Grid spacing 1.5, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-041 frame 0: Ball size 401, above 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-041 frame 4: Ball size 401, above 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-042 frame 0: Rotation 36001, above 36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-042 frame 4: Rotation 36001, above 36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-043 frame 0: Rotation axis "w": not one of the nine. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-043 frame 4: Rotation axis "w": not one of the nine. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-044 frame 0: Rotation axis "X": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-044 frame 4: Rotation axis "X": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALL-045 frame 0: Twist property "spiral": not one of the thirteen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-045 frame 4: Twist property "spiral": not one of the thirteen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALL-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ball_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ball_017.json is saved with its eight settings, rotation axis xy15z, rotation 6, grid spacing 4 | {"ball_size":100,"grid_spacing":4,"instability_state":0,"rotation":6,"rotation_axis":"xy15z","scatter":0,"twist_angle":0,"twist_property":"x_axis"} | yes |
| fx_ball_038.json is refused in a sentence naming scatter | CC Ball Action's scatter runs from 0 to 1000, and this is -1. | yes |
| fx_ball_039.json is refused in a sentence naming scatter | CC Ball Action's scatter runs from 0 to 1000, and this is 1001. | yes |
| fx_ball_040.json is refused in a sentence naming grid spacing | CC Ball Action's grid spacing runs from 2 to 200, and this is 1.5. | yes |
| fx_ball_041.json is refused in a sentence naming ball size | CC Ball Action's ball size runs from 0 to 400, and this is 401. | yes |
| fx_ball_042.json is refused in a sentence naming rotation | CC Ball Action's rotation runs from -36000 to 36000, and this is 36001. | yes |
| fx_ball_043.json is refused in a sentence naming rotation axis | CC Ball Action's rotation axis is one of "x", "y", "z", "xy", "xz", "yz", "xyz", "x15z", "xy15z", and this is "w". | yes |
| fx_ball_044.json is refused in a sentence naming rotation axis | CC Ball Action's rotation axis is one of "x", "y", "z", "xy", "xz", "yz", "xyz", "x15z", "xy15z", and this is "X". | yes |
| fx_ball_045.json is refused in a sentence naming twist property | CC Ball Action's twist property is one of "x_axis", "y_axis", "center_x", "center_y", "radius", "random", "red", "green", "blue", "brightness", "diamond", "rectangle", "fast_top", and this is "spiral". | yes |
| a file with a CC Ball Action whose grid spacing is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Ball Action whose rotation axis is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-551)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ball_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ball_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Ball Action as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072193 pixels changed | yes |
| the reference shot, CC Ball Action as added on three layers, frame 0, Full | largest difference 1 of 255, 4413 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action as added on three layers, frame 100, Full | largest difference 1 of 255, 4061 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action as added on three layers, frame 239, Full | largest difference 1 of 255, 4435 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action as added on three layers, frame 0, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action as added on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action as added on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073553 pixels changed | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90 on three layers, frame 0, Full | largest difference 1 of 255, 1227 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90 on three layers, frame 100, Full | largest difference 1 of 255, 878 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90 on three layers, frame 239, Full | largest difference 1 of 255, 1360 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90 on three layers, frame 0, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90 on three layers, frame 100, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action grid spacing 24, rotation 40 about XY, twist radius 90 on three layers, frame 239, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072469 pixels changed | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160 on three layers, frame 0, Full | largest difference 1 of 255, 1258 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160 on three layers, frame 100, Full | largest difference 1 of 255, 1231 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160 on three layers, frame 239, Full | largest difference 1 of 255, 1207 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160 on three layers, frame 0, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160 on three layers, frame 100, Draft | largest difference 1 of 255, 89 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action scatter 60, instability state 120, ball size 160 on three layers, frame 239, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073549 pixels changed | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y on three layers, frame 0, Full | largest difference 1 of 255, 946 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y on three layers, frame 100, Full | largest difference 1 of 255, 1212 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y on three layers, frame 239, Full | largest difference 1 of 255, 1138 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y on three layers, frame 100, Draft | largest difference 1 of 255, 99 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Ball Action fast top 200, rotation keyed 0 to 720 about Y on three layers, frame 239, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-551 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed, balls drawn | 129600 pixels changed; 118800 pixels covered; warnings [] | yes |
| 2_large_balls.png: drawn without a warning, the street changed, balls drawn | 129438 pixels changed; 103680 pixels covered; warnings [] | yes |
| 3_turned_about_y.png: drawn without a warning, the street changed, balls drawn | 129492 pixels changed; 83588 pixels covered; warnings [] | yes |
| 4_twist_radius.png: drawn without a warning, the street changed, balls drawn | 129555 pixels changed; 87130 pixels covered; warnings [] | yes |
| 5_scattered.png: drawn without a warning, the street changed, balls drawn | 129547 pixels changed; 70329 pixels covered; warnings [] | yes |

## Result

235 of 235 checks pass.
