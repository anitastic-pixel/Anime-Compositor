# D-386: Griddler

B-265, after CycoreFX's CC Griddler: the layer cut into square tiles Tile Size per cent of its width, the picture inside each scaled across and down and turned about the tile's own centre; Cut Tiles keeps each tile's picture inside its own square. The formulas are this program's own. Every expected pixel is `Fixtures/griddler/expected_griddler.json`, written by `tools/griddler_reference.py` before this code existed and printed in document 25 as FX-GRIDDLER-001 to 022. Tolerance 2e-5.

## FX-GRIDDLER-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRIDDLER-001 frame 0: The settings as they start: tiles a tenth of the width, 1.6 pixels here, each its own piece at 80 per cent, cut: a fine grid of gaps through the drawing. | largest difference 2.5e-7 | yes |
| FX-GRIDDLER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-002 frame 0: Scales 100, rotation 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-003 frame 0: Tiles 4 pixels, scales 50, cut: each tile's own piece at half size in its middle 2 pixels, the ring round it transparent. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-004 frame 0: FX-GRIDDLER-003 with Cut Tiles off: each tile filled with the 8 pixels round its centre at half size. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-005 frame 0: Tiles 4 pixels, scales 100, rotation 45, cut: each tile's piece turned an eighth, its corners cut off at the tile's edges and the tile's own corners empty. | largest difference 2.5e-7 | yes |
| FX-GRIDDLER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-006 frame 0: FX-GRIDDLER-005 with Cut Tiles off: the tile's corners filled from the picture round it, turned with it. | largest difference 2.5e-7 | yes |
| FX-GRIDDLER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-007 frame 0: Tiles 4 pixels, horizontal scale -100: each tile's piece turned over left to right, so each stripe pair swaps within its tile. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-008 frame 0: Tiles 4 pixels, vertical scale -100: each tile turned over top to bottom; the band in rows 4 and 5 moves to rows 6 and 7. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-009 frame 0: Tiles 4 pixels, horizontal scale 200: each tile's middle 2 columns stretched across it. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-010 frame 0: Tiles 8 pixels, rotation 90: each tile turned a quarter about its centre, its pixels landing on pixels, so the stripes lie across. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-011 frame 0: Horizontal scale 0: nothing drawn. | largest difference 0.0e0 | yes |
| FX-GRIDDLER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-012 frame 0: Rotation keyed from 0 at frame 0 to 90 at frame 4, linear, tiles 8 pixels: frame 0 the drawing, frame 4 FX-GRIDDLER-010. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-012 frame 2: Rotation keyed from 0 at frame 0 to 90 at frame 4, linear, tiles 8 pixels: frame 0 the drawing, frame 4 FX-GRIDDLER-010. | largest difference 2.5e-7 | yes |
| FX-GRIDDLER-012 frame 4: Rotation keyed from 0 at frame 0 to 90 at frame 4, linear, tiles 8 pixels: frame 0 the drawing, frame 4 FX-GRIDDLER-010. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-013 frame 0: Tile size keyed from 25 at frame 0 to 50 at frame 4, scales 50: the tiles grow. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-013 frame 2: Tile size keyed from 25 at frame 0 to 50 at frame 4, scales 50: the tiles grow. | largest difference 1.3e-7 | yes |
| FX-GRIDDLER-013 frame 4: Tile size keyed from 25 at frame 0 to 50 at frame 4, scales 50: the tiles grow. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-014 frame 0: FX-GRIDDLER-003 moved three pixels right: the same, moved; the tiles go with the layer, and the three columns left of it stay empty. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-015 frame 0: Horizontal scale eased from 100 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-015 frame 2: Horizontal scale eased from 100 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.8e-7 | yes |
| FX-GRIDDLER-015 frame 4: Horizontal scale eased from 100 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.8e-7 | yes |
| FX-GRIDDLER-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRIDDLER-016 frame 0: Horizontal scale 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-016 frame 4: Horizontal scale 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRIDDLER-017 frame 0: Vertical scale -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-017 frame 4: Vertical scale -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRIDDLER-018 frame 0: Tile size 0, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-018 frame 4: Tile size 0, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRIDDLER-019 frame 0: Tile size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-019 frame 4: Tile size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRIDDLER-020 frame 0: Rotation 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-020 frame 4: Rotation 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRIDDLER-021 frame 0: Cut Tiles written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-021 frame 4: Cut Tiles written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRIDDLER-022 frame 0: Tile size keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-022 frame 4: Tile size keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRIDDLER-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_griddler_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_griddler_006.json is saved with its scales, tile size, rotation and cut tiles | {"cut_tiles":"off","horizontal_scale":100,"rotation":45,"tile_size":25,"vertical_scale":100} | yes |
| fx_griddler_016.json is refused in a sentence | Griddler's horizontal scale runs from -1000 to 1000, and this is 1001. | yes |
| fx_griddler_017.json is refused in a sentence | Griddler's vertical scale runs from -1000 to 1000, and this is -1001. | yes |
| fx_griddler_018.json is refused in a sentence | Griddler's tile size runs from 0.1 to 100, and this is 0. | yes |
| fx_griddler_019.json is refused in a sentence | Griddler's tile size runs from 0.1 to 100, and this is 101. | yes |
| fx_griddler_020.json is refused in a sentence | Griddler's rotation runs from -3600 to 3600, and this is 3601. | yes |
| fx_griddler_021.json is refused in a sentence | Griddler's cut tiles is "on" or "off", and this is "yes". | yes |
| a file with a Griddler with no `tile_size` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Griddler whose rotation is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| horizontal scale 1000.5 is refused with a sentence, and nothing changes | Griddler's horizontal scale runs from -1000 to 1000, and this is 1000.5. | yes |
| tile size 0.05 is refused with a sentence, and nothing changes | Griddler's tile size runs from 0.1 to 100, and this is 0.05. | yes |
| rotation -3600.5 is refused with a sentence, and nothing changes | Griddler's rotation runs from -3600 to 3600, and this is -3600.5. | yes |
| cut tiles "On", written with a capital is refused with a sentence, and nothing changes | Griddler's cut tiles is "on" or "off", and this is "On". | yes |
| vertical scale keyed to 1500 is refused with a sentence, and nothing changes | Griddler's vertical scale runs from -1000 to 1000, and this is 1500. | yes |
| 60 across, -120 down, tiles of 20, turned 30, uncut is taken | taken | yes |
| rotation keyed from 0 to 90 is taken | taken | yes |
| tile size keyed from 10 to 40 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_griddler_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_griddler_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_griddler_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_griddler_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_griddler_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_griddler_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_griddler_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_griddler_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_griddler_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_griddler_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_griddler_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1895984 pixels changed | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent) on three layers, frame 0, Full | largest difference 1 of 255, 1290 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent) on three layers, frame 100, Full | largest difference 1 of 255, 730 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent) on three layers, frame 239, Full | largest difference 1 of 255, 715 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent) on three layers, frame 0, Draft | largest difference 1 of 255, 28 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent) on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler as it starts (tiles of 10, 80 per cent) on three layers, frame 239, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1833022 pixels changed | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut on three layers, frame 0, Full | largest difference 1 of 255, 2199 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut on three layers, frame 100, Full | largest difference 1 of 255, 953 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut on three layers, frame 239, Full | largest difference 1 of 255, 852 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut on three layers, frame 0, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut on three layers, frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 5 turned 30, uncut on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2043874 pixels changed | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down on three layers, frame 0, Full | largest difference 1 of 255, 2677 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down on three layers, frame 100, Full | largest difference 1 of 255, 5207 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down on three layers, frame 239, Full | largest difference 1 of 255, 2647 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down on three layers, frame 0, Draft | largest difference 1 of 255, 99 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down on three layers, frame 100, Draft | largest difference 1 of 255, 213 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 20, turned over across, 150 down on three layers, frame 239, Draft | largest difference 1 of 255, 119 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2059114 pixels changed | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200 on three layers, frame 0, Full | largest difference 1 of 255, 589 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200 on three layers, frame 100, Full | largest difference 1 of 255, 142 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200 on three layers, frame 239, Full | largest difference 1 of 255, 117 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200 on three layers, frame 0, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200 on three layers, frame 100, Draft | largest difference 1 of 255, 6 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Griddler tiles of 12.5 at 40 per cent, turned -200 on three layers, frame 239, Draft | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-386 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (tiles of 10, 80 per cent): the street in a grid of shrunken tiles with clear seams; draws cleanly | [], 87278 pixels changed | yes |
| 3_scale_100.png, 100 per cent, unturned: nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 4_turned_45_uncut.png, tiles of 20 turned 45, uncut: every tile holds its piece of the street turned on the slant; draws cleanly | [], 100049 pixels changed | yes |
| 5_mirrored_across.png, tiles of 25 turned over across: each tile's piece of the street mirrored left to right; draws cleanly | [], 36336 pixels changed | yes |

## Result

158 of 158 checks pass.
