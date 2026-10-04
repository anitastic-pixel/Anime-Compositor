# B-97: motion tile

D-154, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-first of the third batch. Every expected pixel is `Fixtures/motion_tile/expected_motion_tile.json`, written by `tools/motion_tile_reference.py` before this code existed and printed in document 25 as FX-TILE-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TILE-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TILE-001 frame 0: The settings as they start, output width 100, output height 100, mirror off: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-TILE-001 frame 4: The settings as they start, output width 100, output height 100, mirror off: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-TILE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-002 frame 0: Mirror on at 100, 100: there is only the one tile, the drawing itself, so it is untouched. | largest difference 1.9e-7 | yes |
| FX-TILE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-003 frame 0: Output width 200: the layer grows by 3 on the left and the right, 6 (1 - 1) / 2 rounded up, and is 12 wide: columns 2 to 4 repeat the drawing's columns 3 to 5, the right part of the tile to its left, and columns 11 to 13 its columns 0 to 2, so the red dot is repeated at (12, 4); rows 3 to 6 only, nothing above or below. | largest difference 1.9e-7 | yes |
| FX-TILE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-004 frame 0: Output height 200: the layer grows by 2 above and below and is 8 tall: rows 1 and 2 repeat the drawing's rows 2 and 3, and rows 7 and 8 its rows 0 and 1, so the red dot is repeated at (6, 8); columns 5 to 10 only. | largest difference 1.9e-7 | yes |
| FX-TILE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-005 frame 0: Output width and height 200: a layer 12 by 8, columns 2 to 13 and rows 1 to 8, every pixel of it a pixel of the drawing, the tiles repeated across, down and at the corners; the rest of the frame stays empty. | largest difference 1.9e-7 | yes |
| FX-TILE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-006 frame 0: FX-TILE-005 with mirror on: the tiles left and right of the drawing are turned over left to right and those above and below top to bottom, so each edge meets its own reflection: column 4 is the drawing's column 0 and column 11 its column 5, row 2 is its row 0 and row 7 its row 3, and the red dot is repeated at (3, 4), (6, 1) and (3, 1). | largest difference 1.9e-7 | yes |
| FX-TILE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-007 frame 0: Output width and height 300: the layer grows by 6 and by 4 and is 18 by 12, more than the frame, so the whole frame is tiled: every pixel is the drawing's pixel at column (x - 5) mod 6 and row (y - 3) mod 4. | largest difference 1.9e-7 | yes |
| FX-TILE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-008 frame 0: FX-TILE-007 with mirror on: the whole frame tiled with every other tile turned over, a pattern that repeats every 12 columns and 8 rows with no seam. | largest difference 1.9e-7 | yes |
| FX-TILE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-009 frame 0: Output width and height 1000, the most: the layer grows by 27 and by 18 and is 60 by 40, far larger than the frame, and the tiles keep their places against the drawing, so the frame is FX-TILE-007's exactly. | largest difference 1.9e-7 | yes |
| FX-TILE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-010 frame 0: Output width and height 150: the growth is rounded up to whole pixels, 1.5 to 2 on the left and the right and exactly 1 above and below, so the layer is 10 by 6, columns 3 to 12 and rows 2 to 7, not exactly 150 per cent. | largest difference 1.9e-7 | yes |
| FX-TILE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-011 frame 0: Output width 110 and height 125: the least growth, 0.3 and 0.5 rounded up to 1, one pixel on every side, each the pixel from the opposite edge of the drawing: column 4 is its column 5 and row 2 its row 3. | largest difference 1.9e-7 | yes |
| FX-TILE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-012 frame 0: FX-TILE-011 with mirror on: each added pixel is the drawing's own edge pixel beside it, as a held edge is: column 4 is its column 0, column 11 its column 5, row 2 its row 0 and row 7 its row 3. | largest difference 1.9e-7 | yes |
| FX-TILE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-013 frame 0: Output width keyed from 100 at frame 0 to 300 at frame 4, linear: frame 0 is the drawing, frame 2 is width 200, FX-TILE-003, and frame 4 is width 300, a band of tiles across the whole frame in rows 3 to 6. | largest difference 1.9e-7 | yes |
| FX-TILE-013 frame 2: Output width keyed from 100 at frame 0 to 300 at frame 4, linear: frame 0 is the drawing, frame 2 is width 200, FX-TILE-003, and frame 4 is width 300, a band of tiles across the whole frame in rows 3 to 6. | largest difference 1.9e-7 | yes |
| FX-TILE-013 frame 4: Output width keyed from 100 at frame 0 to 300 at frame 4, linear: frame 0 is the drawing, frame 2 is width 200, FX-TILE-003, and frame 4 is width 300, a band of tiles across the whole frame in rows 3 to 6. | largest difference 1.9e-7 | yes |
| FX-TILE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-014 frame 0: Output height keyed from 100 at frame 0 to 1000 at frame 4, eased past its end (1292.5 at frame 2): frame 2 is held at 1000, so frames 2 and 4 are the same, a column of tiles down the whole frame in columns 5 to 10. | largest difference 1.9e-7 | yes |
| FX-TILE-014 frame 2: Output height keyed from 100 at frame 0 to 1000 at frame 4, eased past its end (1292.5 at frame 2): frame 2 is held at 1000, so frames 2 and 4 are the same, a column of tiles down the whole frame in columns 5 to 10. | largest difference 1.9e-7 | yes |
| FX-TILE-014 frame 4: Output height keyed from 100 at frame 0 to 1000 at frame 4, eased past its end (1292.5 at frame 2): frame 2 is held at 1000, so frames 2 and 4 are the same, a column of tiles down the whole frame in columns 5 to 10. | largest difference 1.9e-7 | yes |
| FX-TILE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-015 frame 0: FX-TILE-005 with the layer moved three pixels right: the tiles move with it, columns 5 to 15 are FX-TILE-005's columns 2 to 12, its column 13 is past the frame's edge, and columns 0 to 4 stay empty. | largest difference 1.9e-7 | yes |
| FX-TILE-015 frame 3: FX-TILE-005 with the layer moved three pixels right: the tiles move with it, columns 5 to 15 are FX-TILE-005's columns 2 to 12, its column 13 is past the frame's edge, and columns 0 to 4 stay empty. | largest difference 1.9e-7 | yes |
| FX-TILE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-016 frame 0: A drop shadow two pixels right, then output width and height 150: the shadow grew the layer by 2 on every side to 10 by 8, and the tiles are that grown layer's, shadow and all: it grows by 3 across (2.5 rounded up) and 2 down to 16 by 12, filling the frame. | largest difference 1.9e-7 | yes |
| FX-TILE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILE-017 frame 0: Output width 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-017 frame 4: Output width 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-018 frame 0: Output width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-018 frame 4: Output width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-019 frame 0: Output height 99.5, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-019 frame 4: Output height 99.5, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-020 frame 0: Output height 1000.5, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-020 frame 4: Output height 1000.5, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-021 frame 0: Output width keyed to 1200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-021 frame 4: Output width keyed to 1200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-022 frame 0: Mirror "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-022 frame 4: Mirror "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILE-023 frame 0: Mirror "On": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-023 frame 4: Mirror "On": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it declares no fixed growth to the card, which never runs it: its growth depends on the size the drawing reaches it at, and is counted as the stack runs, as FX-TILE-003 to 016 show | 0 | yes |
| a half-size draft preview changes nothing: the sizes are shares of the drawing | MotionTile { output_width: 300.0, output_height: 150.0, mirror: "on", tile_center: [50.0, 50.0], tile_width: 100.0, tile_height: 100.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tile_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tile_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `mirror` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a mirror that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| output width 99 is refused with a sentence, and nothing changes | Motion Tile's output width runs from 100 to 1000, and this is 99. | yes |
| output width 1001 is refused with a sentence, and nothing changes | Motion Tile's output width runs from 100 to 1000, and this is 1001. | yes |
| output height 99.5 is refused with a sentence, and nothing changes | Motion Tile's output height runs from 100 to 1000, and this is 99.5. | yes |
| output height 1000.5 is refused with a sentence, and nothing changes | Motion Tile's output height runs from 100 to 1000, and this is 1000.5. | yes |
| mirror "yes" is refused with a sentence, and nothing changes | Motion Tile's mirror is "off" or "on", and this is "yes". | yes |
| mirror "On" is refused with a sentence, and nothing changes | Motion Tile's mirror is "off" or "on", and this is "On". | yes |
| output width keyed to 1200 is refused with a sentence, and nothing changes | Motion Tile's output width runs from 100 to 1000, and this is 1200. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| output width keyed from 100 to 300 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tile_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tile_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tile_015.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tile_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

91 of 91 checks pass.
