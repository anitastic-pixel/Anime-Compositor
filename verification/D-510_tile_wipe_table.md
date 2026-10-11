# D-510: Tile Wipe

B-390, after CycoreFX's CC Grid Wipe (Transition): the layer cut into square tiles that shrink away. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/tile_wipe_reference.py`: Tiles squares across the drawing's width, the grid turned by Rotation about the Center; each tile shrinks about its own centre, the tiles nearest the Center (round, square or the two doors' distance) first, each taking a quarter of the transition; Border softens each shrinking tile's edge. Every expected pixel is `Fixtures/tile_wipe/expected_tile_wipe.json`, written by the tool before this code existed and printed in document 25 as FX-TILE-001 to 028. Tolerance 2e-5.

## FX-TILE-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TILE-001 frame 0: The settings as they start: completion 0, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-TILE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-002 frame 0: Completion 100: every tile gone, every pixel clear. | largest difference 0.0e0 | yes |
| FX-TILE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-003 frame 0: Completion 50, tiles 4 (tiles 4 pixels wide), radial: the tiles round the middle gone, the next ring shrinking, the corners whole. | largest difference 1.9e-7 | yes |
| FX-TILE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-004 frame 0: FX-TILE-003 with shape rectangle: the tiles go in square rings. | largest difference 1.1e-7 | yes |
| FX-TILE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-005 frame 0: FX-TILE-003 with shape doors: the tiles go in columns from the middle out, splitting the layer in two. | largest difference 1.1e-7 | yes |
| FX-TILE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-006 frame 0: FX-TILE-003 reversed: the corners go first, the middle stays. | largest difference 1.2e-7 | yes |
| FX-TILE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-007 frame 0: FX-TILE-003 with border 2: each shrinking tile's edge soft over 2 pixels. | largest difference 1.7e-7 | yes |
| FX-TILE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-008 frame 0: FX-TILE-003 with rotation 30: the whole grid turned, not each tile. | largest difference 1.3e-7 | yes |
| FX-TILE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-009 frame 0: Center [0, 0], completion 50, tiles 4: the wipe starts from the top-left corner. | largest difference 1.9e-7 | yes |
| FX-TILE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-010 frame 0: Tiles 1, completion 10: one tile the drawing's width, four tenths of the way gone, shrunk about the middle. | largest difference 1.9e-7 | yes |
| FX-TILE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-011 frame 0: Tiles 2.5, completion 60, rectangle: tile 6.4 pixels wide, so the tiles need not fit the drawing. | largest difference 1.9e-7 | yes |
| FX-TILE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-012 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, tiles 4, border 1: frame 2 is completion 50. | largest difference 1.9e-7 | yes |
| FX-TILE-012 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, tiles 4, border 1: frame 2 is completion 50. | largest difference 1.7e-7 | yes |
| FX-TILE-012 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, tiles 4, border 1: frame 2 is completion 50. | largest difference 0.0e0 | yes |
| FX-TILE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-013 frame 0: Rotation keyed from 0 at frame 0 to 40 at frame 4, linear, completion 50, tiles 4: frame 2 is rotation 20. | largest difference 1.9e-7 | yes |
| FX-TILE-013 frame 2: Rotation keyed from 0 at frame 0 to 40 at frame 4, linear, completion 50, tiles 4: frame 2 is rotation 20. | largest difference 1.2e-7 | yes |
| FX-TILE-013 frame 4: Rotation keyed from 0 at frame 0 to 40 at frame 4, linear, completion 50, tiles 4: frame 2 is rotation 20. | largest difference 1.3e-7 | yes |
| FX-TILE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-014 frame 0: Center keyed from [25, 50] at frame 0 to [75, 50] at frame 4, linear, completion 40, tiles 4, rectangle. | largest difference 1.9e-7 | yes |
| FX-TILE-014 frame 4: Center keyed from [25, 50] at frame 0 to [75, 50] at frame 4, linear, completion 40, tiles 4, rectangle. | largest difference 1.2e-7 | yes |
| FX-TILE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-015 frame 0: FX-TILE-007 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-TILE-015 frame 3: FX-TILE-007 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-TILE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-016 frame 0: Doors reversed, completion 70, tiles 8, border 0.5: the outer columns gone, a soft middle left. | largest difference 1.2e-7 | yes |
| FX-TILE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-017 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-017 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-018 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-018 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-019 frame 0: Center x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-019 frame 4: Center x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-020 frame 0: Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-020 frame 4: Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-021 frame 0: Border -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-021 frame 4: Border -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-022 frame 0: Border 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-022 frame 4: Border 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-023 frame 0: Tiles 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-023 frame 4: Tiles 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-024 frame 0: Tiles 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-024 frame 4: Tiles 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-025 frame 0: Shape "Radial": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-025 frame 4: Shape "Radial": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-026 frame 0: Shape "star": not a shape of Tile Wipe. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-026 frame 4: Shape "star": not a shape of Tile Wipe. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-027 frame 0: Reverse "yes": "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-027 frame 4: Reverse "yes": "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-028 frame 0: Completion keyed to 200 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-028 frame 4: Completion keyed to 200 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tile_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_016.json is saved with its seven settings, doors, reversed, border 0.5, 8 tiles | {"border":0.5,"center":[50,50],"completion":70,"reverse":"on","rotation":0,"shape":"doors","tiles":8} | yes |
| fx_tile_017.json is refused in a sentence naming completion | Tile Wipe's completion runs from 0 to 100, and this is 101. | yes |
| fx_tile_018.json is refused in a sentence naming completion | Tile Wipe's completion runs from 0 to 100, and this is -1. | yes |
| fx_tile_019.json is refused in a sentence naming center | Tile Wipe's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_tile_020.json is refused in a sentence naming rotation | Tile Wipe's rotation runs from -3600 to 3600, and this is 3601. | yes |
| fx_tile_021.json is refused in a sentence naming border | Tile Wipe's border runs from 0 to 100, and this is -1. | yes |
| fx_tile_022.json is refused in a sentence naming border | Tile Wipe's border runs from 0 to 100, and this is 101. | yes |
| fx_tile_023.json is refused in a sentence naming tiles | Tile Wipe's tiles runs from 1 to 200, and this is 0.5. | yes |
| fx_tile_024.json is refused in a sentence naming tiles | Tile Wipe's tiles runs from 1 to 200, and this is 201. | yes |
| fx_tile_025.json is refused in a sentence naming shape | Tile Wipe's shape is "doors", "radial" or "rectangle", and this is "Radial". | yes |
| fx_tile_026.json is refused in a sentence naming shape | Tile Wipe's shape is "doors", "radial" or "rectangle", and this is "star". | yes |
| fx_tile_027.json is refused in a sentence naming reverse | Tile Wipe's reverse is "off" or "on", and this is "yes". | yes |
| a file with a Tile Wipe whose tiles is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Tile Wipe whose Center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-510)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tile_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tile_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1108724 pixels changed | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 50 (as added otherwise) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1059744 pixels changed | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 40, rectangle, reversed, 6 tiles, border 3 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70]: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1806462 pixels changed | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70] on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70] on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70] on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70] on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70] on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion 60, doors, turned 30, Center [20, 70] on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 838208 pixels changed | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90 on three layers, frame 0, Full | largest difference 1 of 255, 3235 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90 on three layers, frame 100, Full | largest difference 1 of 255, 100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90 on three layers, frame 0, Draft | largest difference 1 of 255, 114 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90 on three layers, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tile Wipe completion keyed 10 to 90 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-510 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_radial_half.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 2) as the street's | 69464 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [120, 170, 229, 255]; warnings [] | yes |
| 2_rectangle_reversed.png: drawn without a warning, the pixel at (2, 2) gone, the one at (240, 135) as the street's | 62200 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [90, 140, 170, 255]; warnings [] | yes |
| 3_doors_soft_border.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 135) as the street's | 76464 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 135) [180, 90, 70, 255]; warnings [] | yes |
| 4_many_tiles_turned.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 2) as the street's | 69154 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [120, 170, 229, 255]; warnings [] | yes |
| 5_done.png: drawn without a warning, the pixel at (2, 2) gone, every pixel gone | 129600 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [0, 0, 0, 0]; warnings [] | yes |

## Result

177 of 177 checks pass.
