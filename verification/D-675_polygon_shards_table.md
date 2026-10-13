# D-675: Polygon Shards

B-554, after CycoreFX's CC Pixel Polly (Simulation): the layer cut along a grid into triangles or squares from the start time, each thrown away from the force centre in 3D, falling and spinning, seen through the composition camera or the effect's own eye, drawn far to near; before the start time the layer as it is. Nothing is carried from frame to frame: every shard's flight is worked from the start time with every setting as it is at the frame drawn (document 20). CycoreFX publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/polygon_shards_reference.py`. Every expected pixel is `Fixtures/polygon_shards/expected_polygon_shards.json`, written by the tool before this code existed and printed in document 25 as FX-POLYGON-SHARDS-001 to 036. Tolerance 2e-5.

## FX-POLYGON-SHARDS-001 to 036 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-POLYGON-SHARDS-001 frame 0: As added: a grid of 20 pixels, three cells by two, each cut into two triangles thrown out from the middle and towards the eye, falling and spinning; frame 0 the layer as it is. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-001 frame 12: As added: a grid of 20 pixels, three cells by two, each cut into two triangles thrown out from the middle and towards the eye, falling and spinning; frame 0 the layer as it is. | largest difference 1.6e-7 | yes |
| FX-POLYGON-SHARDS-001 frame 24: As added: a grid of 20 pixels, three cells by two, each cut into two triangles thrown out from the middle and towards the eye, falling and spinning; frame 0 the layer as it is. | largest difference 1.6e-7 | yes |
| FX-POLYGON-SHARDS-001 frame 6: As added: a grid of 20 pixels, three cells by two, each cut into two triangles thrown out from the middle and towards the eye, falling and spinning; frame 0 the layer as it is. | largest difference 1.7e-7 | yes |
| FX-POLYGON-SHARDS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-002 frame 12: A close look: a grid of 8 pixels, 48 triangles; frame 3 just cracked apart, frame 12 flying. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-002 frame 3: A close look: a grid of 8 pixels, 48 triangles; frame 3 just cracked apart, frame 12 flying. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-003 frame 0: Object Polygon: each triangle its own flat colour. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-003 frame 6: Object Polygon: each triangle its own flat colour. | largest difference 1.3e-7 | yes |
| FX-POLYGON-SHARDS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-004 frame 6: Object Textured Square: squares carrying the picture. | largest difference 1.7e-7 | yes |
| FX-POLYGON-SHARDS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-005 frame 6: Object Square: squares, each its own flat colour. | largest difference 1.2e-7 | yes |
| FX-POLYGON-SHARDS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-006 frame 12: Force 0: the shards only fall and spin. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-007 frame 12: Gravity 0, Force 100: thrown out slowly, not falling. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-008 frame 12: Gravity -1: the shards float up. | largest difference 1.6e-7 | yes |
| FX-POLYGON-SHARDS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-009 frame 12: Spinning 0: the shards fly flat, facing the eye. | largest difference 1.4e-7 | yes |
| FX-POLYGON-SHARDS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-010 frame 6: Spinning 720: the shards tumble fast. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-011 frame 6: The force centre at the top left corner: thrown down and right. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-012 frame 6: Direction Randomness 0: each thrown straight out from the centre, 45 degrees towards the eye. | largest difference 1.7e-7 | yes |
| FX-POLYGON-SHARDS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-013 frame 6: Direction Randomness 100: thrown every which way, from flat across the layer to straight at the eye. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-014 frame 12: Speed Randomness 100: speeds from none to full. | largest difference 1.6e-7 | yes |
| FX-POLYGON-SHARDS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-015 frame 12: Depth Sort off: drawn in grid order, later shards over earlier ones whatever their depth. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-016 frame 18: Start Time 0.5 seconds: frame 6 the layer as it is, frame 18 FX-POLYGON-SHARDS-002's frame 6. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-016 frame 6: Start Time 0.5 seconds: frame 6 the layer as it is, frame 18 FX-POLYGON-SHARDS-002's frame 6. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-017 frame 6: The picture with its right half clear: the right half's shards are clear. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-018 frame 6: The half picture, Object Polygon: the right half's triangles clear, the left half's flat. | largest difference 1.2e-7 | yes |
| FX-POLYGON-SHARDS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-019 frame 6: A grid of 7 pixels, which fits neither side: the last column and row of cells cut at the layer's edge. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-020 frame 12: Force keyed from 0 at frame 0 to 600 at frame 24: frame 12 is Force 300 at frame 12, the whole flight from the key's value then. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-021 frame 12: FX-POLYGON-SHARDS-002 moved 3 pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-022 frame 18: FX-POLYGON-SHARDS-002 with the layer starting at frame 6: frame 9 is 002's frame 3 and frame 18 its 12. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-022 frame 9: FX-POLYGON-SHARDS-002 with the layer starting at frame 6: frame 9 is 002's frame 3 and frame 18 its 12. | largest difference 1.8e-7 | yes |
| FX-POLYGON-SHARDS-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-023 frame 12: Through a composition camera 100 pixels away over the middle: the shards coming at the eye grow less than through the effect's own eye 67 pixels away. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-024 frame 12: The composition camera 6 pixels to the right: the frame moved 6 pixels left, the shards seen from the side. | largest difference 1.6e-7 | yes |
| FX-POLYGON-SHARDS-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-025 frame 12: FX-POLYGON-SHARDS-002 at 30 frames a second: frame 12 is two fifths of a second in, not half. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLYGON-SHARDS-026 frame 0: Force -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-026 frame 4: Force -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-027 frame 0: Gravity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-027 frame 4: Gravity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-028 frame 0: Spinning 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-028 frame 4: Spinning 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-029 frame 0: Force Center X 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-029 frame 4: Force Center X 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-030 frame 0: Direction Randomness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-030 frame 4: Direction Randomness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-031 frame 0: Speed Randomness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-031 frame 4: Speed Randomness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-032 frame 0: Grid Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-032 frame 4: Grid Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-033 frame 0: Start Time -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-033 frame 4: Start Time -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-034 frame 0: An object written "triangle". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-034 frame 4: An object written "triangle". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-035 frame 0: A depth sort written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-035 frame 4: A depth sort written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLYGON-SHARDS-036 frame 0: Force keyed to 2000 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-036 frame 4: Force keyed to 2000 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-POLYGON-SHARDS-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polygon_shards_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polygon_shards_023.json is saved with its 11 settings, nine numbers and two words, and no clock or camera | 11 settings; grid spacing 8, force 300, object "textured_polygon", depth sort "on" | yes |
| fx_polygon_shards_026.json is refused in a sentence naming force | Polygon Shards's force runs from 0 to 1000, and this is -1. | yes |
| fx_polygon_shards_027.json is refused in a sentence naming gravity | Polygon Shards's gravity runs from -100 to 100, and this is 101. | yes |
| fx_polygon_shards_028.json is refused in a sentence naming spinning | Polygon Shards's spinning runs from -3600 to 3600, and this is 3601. | yes |
| fx_polygon_shards_029.json is refused in a sentence naming force center x | Polygon Shards's force center x runs from -1000 to 1000, and this is 1001. | yes |
| fx_polygon_shards_030.json is refused in a sentence naming direction randomness | Polygon Shards's direction randomness runs from 0 to 100, and this is 101. | yes |
| fx_polygon_shards_031.json is refused in a sentence naming speed randomness | Polygon Shards's speed randomness runs from 0 to 100, and this is -1. | yes |
| fx_polygon_shards_032.json is refused in a sentence naming grid spacing | Polygon Shards's grid spacing runs from 2 to 1000, and this is 1. | yes |
| fx_polygon_shards_033.json is refused in a sentence naming start time | Polygon Shards's start time runs from 0 to 3600, and this is -1. | yes |
| fx_polygon_shards_034.json is refused in a sentence naming object | Polygon Shards's object is polygon, textured_polygon, square, textured_square, and this is "triangle". | yes |
| fx_polygon_shards_035.json is refused in a sentence naming depth sort | Polygon Shards's depth sort is off, on, and this is "yes". | yes |
| fx_polygon_shards_036.json is refused in a sentence naming force | Polygon Shards's force runs from 0 to 1000, and this is 2000. | yes |
| a file with a Polygon Shards whose force is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Polygon Shards whose object is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Polygon Shards without its depth sort is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polygon_shards_002.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polygon_shards_013.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polygon_shards_019.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polygon_shards_024.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-675)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polygon_shards_001.json, frames [0, 12, 24, 6] at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_002.json, frames [12, 3] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_003.json, frames [0, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_004.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_005.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_006.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_007.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_008.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_009.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_010.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_011.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_012.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_013.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_014.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_015.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_016.json, frames [18, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_017.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_018.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_019.json, frames [6] at Full and Draft | largest difference 1 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_020.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_021.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_022.json, frames [18, 9] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_023.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_024.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_025.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_026.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_027.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_028.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_029.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_030.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_031.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_032.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_033.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_034.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_035.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_polygon_shards_036.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Polygon Shards as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Polygon Shards as added on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards as added on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards as added on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards as added on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards as added on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards as added on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards small flat triangles tumbling from the top left on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Polygon Shards force keyed 0 to 600, textured squares in grid order on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-675 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 129590 pixels changed; warnings [] | yes |
| 2_just_cracked_frame_3.png, frame 3: drawn without a warning, the street changed | 96192 pixels changed; warnings [] | yes |
| 3_flat_squares.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 4_from_the_top_left.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 5_small_tumbling_pieces.png, frame 24: drawn without a warning, the street changed | 129499 pixels changed; warnings [] | yes |
| 6_depth_sort_off.png, frame 24: drawn without a warning, the street changed | 129496 pixels changed; warnings [] | yes |

## Result

208 of 208 checks pass.
