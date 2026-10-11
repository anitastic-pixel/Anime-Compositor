# D-464: Repeat Tile

B-344: `core.repeat_tile` takes After Effects' CC RepeTile controls (Expand Right, Expand Left, Expand Down, Expand Up, Tiling, Blend Borders). Every expected pixel is `Fixtures/repeat_tile/expected_repeat_tile.json`, written by `tools/repeat_tile_reference.py` before this code existed and printed in document 25 as FX-REPEAT-001 to 030. Tolerance 2e-5.

## FX-REPEAT-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-REPEAT-001 frame 0: The settings as they start: Unfold, nothing grown, no blending. The picture as it was, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-REPEAT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-002 frame 0: Repeat: the picture copied as it is, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-003 frame 0: Checker Flip H: every other copy, as a chequerboard, flipped across, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-004 frame 0: Checker Flip V: every other copy flipped down, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-005 frame 0: Unfold: copies flipped across in odd columns and down in odd rows, so they meet edge to matching edge, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-006 frame 0: Checker 180: every other copy turned half round, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-007 frame 0: Checker Flip 45: every other copy flipped on its diagonal, squeezed to the tile's shape, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-008 frame 0: Checker 90 CW: every other copy turned a quarter clockwise, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-009 frame 0: Checker 90 CCW: every other copy turned a quarter anticlockwise, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-010 frame 0: Rosette: each square of four copies turned a quarter on round it, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-011 frame 0: Random: each copy one of the eight ways, chosen by its place, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-012 frame 0: None: the room grows but stays clear, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-013 frame 0: Turn CW: each copy a quarter turn clockwise on from the one before it, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-014 frame 0: Turn CCW: each copy a quarter turn anticlockwise on, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-015 frame 0: Twist: turning clockwise across and anticlockwise down, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-016 frame 0: Slide: the edge pixels drawn on outwards, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-017 frame 0: Brick: copies as they are, every other row moved half a tile across, grown 5 across and 3 down each way. | largest difference 1.9e-7 | yes |
| FX-REPEAT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-018 frame 0: Repeat, grown 4 right, 1 left, 2 up and none down: the layer grows 4 both ways across and 2 both ways down, the room past each expansion clear. | largest difference 1.9e-7 | yes |
| FX-REPEAT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-019 frame 0: Repeat, nothing grown, Blend Borders 50: the picture's own edges mixed with its middle, ready to repeat without a seam. | largest difference 1.9e-7 | yes |
| FX-REPEAT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-020 frame 0: Repeat grown all round, Blend Borders 100: the copies meet with no seam. | largest difference 2.0e-7 | yes |
| FX-REPEAT-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-021 frame 0: Unfold grown all round, Blend Borders 100: Unfold ignores the blending and is FX-REPEAT-005. | largest difference 1.9e-7 | yes |
| FX-REPEAT-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-022 frame 0: Expand Right keyed from 0 at frame 0 to 6 at frame 4, linear, Repeat: frame 1 is 1.5, rounded up to 2; frame 2 is 3. | largest difference 1.9e-7 | yes |
| FX-REPEAT-022 frame 1: Expand Right keyed from 0 at frame 0 to 6 at frame 4, linear, Repeat: frame 1 is 1.5, rounded up to 2; frame 2 is 3. | largest difference 1.9e-7 | yes |
| FX-REPEAT-022 frame 2: Expand Right keyed from 0 at frame 0 to 6 at frame 4, linear, Repeat: frame 1 is 1.5, rounded up to 2; frame 2 is 3. | largest difference 1.9e-7 | yes |
| FX-REPEAT-022 frame 4: Expand Right keyed from 0 at frame 0 to 6 at frame 4, linear, Repeat: frame 1 is 1.5, rounded up to 2; frame 2 is 3. | largest difference 1.9e-7 | yes |
| FX-REPEAT-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-023 frame 0: Blend Borders keyed from 50 at frame 0 to 100 at frame 4, eased past its end, Repeat grown all round: frame 2 would pass 100, is held at 100, and is FX-REPEAT-020. | largest difference 1.9e-7 | yes |
| FX-REPEAT-023 frame 2: Blend Borders keyed from 50 at frame 0 to 100 at frame 4, eased past its end, Repeat grown all round: frame 2 would pass 100, is held at 100, and is FX-REPEAT-020. | largest difference 2.0e-7 | yes |
| FX-REPEAT-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-024 frame 0: FX-REPEAT-002 moved three pixels right: the copies move with the drawing and the three columns left bare stay empty. | largest difference 1.9e-7 | yes |
| FX-REPEAT-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-025 frame 0: After a drop shadow that grows the layer to 10 by 8, Checker Flip H grown 2 each way: the copies are of the grown layer, shadow and all. | largest difference 1.9e-7 | yes |
| FX-REPEAT-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-REPEAT-026 frame 0: Expand Right -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-026 frame 4: Expand Right -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REPEAT-027 frame 0: Expand Up 4001, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-027 frame 4: Expand Up 4001, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REPEAT-028 frame 0: Tiling "wallpaper", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-028 frame 4: Tiling "wallpaper", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REPEAT-029 frame 0: Blend Borders 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-029 frame 4: Blend Borders 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-REPEAT-030 frame 0: Expand Left keyed to 5000 at frame 4, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-030 frame 4: Expand Left keyed to 5000 at frame 4, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-REPEAT-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_repeat_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_repeat_018.json is saved with its six settings | {"blend_borders":0,"expand_down":0,"expand_left":1,"expand_right":4,"expand_up":2,"tiling":"repeat"} | yes |
| fx_repeat_026.json is refused in a sentence naming Repeat Tile and its expand right | Repeat Tile's expand right runs from 0 to 4000, and this is -1. | yes |
| fx_repeat_027.json is refused in a sentence naming Repeat Tile and its expand up | Repeat Tile's expand up runs from 0 to 4000, and this is 4001. | yes |
| fx_repeat_028.json is refused in a sentence naming Repeat Tile and its tiling | Repeat Tile's tiling is one of repeat, checker_flip_h, checker_flip_v, unfold, checker_180, checker_flip_45, checker_90_cw, checker_90_ccw, rosette, random, none, turn_cw, turn_ccw, twist, slide, brick, and this is "wallpaper". | yes |
| fx_repeat_029.json is refused in a sentence naming Repeat Tile and its blend borders | Repeat Tile's blend borders runs from 0 to 100, and this is 101. | yes |
| a file with a Repeat Tile whose expand right is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Repeat Tile without its blend borders is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Repeat Tile whose tiling is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| expand down 4001 is refused with a sentence, and nothing changes | Repeat Tile's expand down runs from 0 to 4000, and this is 4001. | yes |
| blend borders 101 is refused with a sentence, and nothing changes | Repeat Tile's blend borders runs from 0 to 100, and this is 101. | yes |
| tiling wallpaper is refused with a sentence, and nothing changes | Repeat Tile's tiling is one of repeat, checker_flip_h, checker_flip_v, unfold, checker_180, checker_flip_45, checker_90_cw, checker_90_ccw, rosette, random, none, turn_cw, turn_ccw, twist, slide, brick, and this is "wallpaper". | yes |
| expand left keyed to 5000 is refused with a sentence, and nothing changes | Repeat Tile's expand left runs from 0 to 4000, and this is 5000. | yes |
| Brick, grown 12 right, 3 left, 7 down and half a pixel up, blend 40 is taken | taken | yes |
| blend borders keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_repeat_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_repeat_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_repeat_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_repeat_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_repeat_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_repeat_022.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_repeat_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_repeat_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_repeat_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1047884 pixels changed | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30 on three layers, frame 0, Full | largest difference 1 of 255, 3800 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30 on three layers, frame 100, Full | largest difference 1 of 255, 1441 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30 on three layers, frame 239, Full | largest difference 1 of 255, 1794 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Checker Flip H, grown 200 right and 100 down, blend 30 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1318192 pixels changed | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40 on three layers, frame 0, Full | largest difference 1 of 255, 3796 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40 on three layers, frame 100, Full | largest difference 1 of 255, 1436 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40 on three layers, frame 239, Full | largest difference 1 of 255, 1788 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40 on three layers, frame 0, Draft | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40 on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Brick, grown 150 all round, blend 40 on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073374 pixels changed | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100 on three layers, frame 0, Full | largest difference 1 of 255, 1030 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100 on three layers, frame 100, Full | largest difference 1 of 255, 982 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100 on three layers, frame 239, Full | largest difference 1 of 255, 1026 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100 on three layers, frame 0, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100 on three layers, frame 100, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Random, grown 80 left and up, blend 100 on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1735099 pixels changed | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60 on three layers, frame 0, Full | largest difference 1 of 255, 4573 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60 on three layers, frame 100, Full | largest difference 1 of 255, 1377 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60 on three layers, frame 239, Full | largest difference 1 of 255, 2468 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60 on three layers, frame 0, Draft | largest difference 1 of 255, 158 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Repeat Tile Repeat, nothing grown, blend 60 on three layers, frame 239, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-464 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street in the middle of a composition twice its size, the rest clear; draws cleanly | [], covered 129600 of 518400 | yes |
| as added (Unfold, nothing grown, no blending): the picture as it was; draws cleanly | [], the same true | yes |
| 2_repeat.png, Repeat, grown half the street each way: the whole composition covered, the street kept in the middle, and the picture the same one street to the right everywhere; draws cleanly | [], covered 518400 of 518400, middle kept true, the same a street to the right at 259200 of 259200 | yes |
| 3_unfold.png, Unfold: the copies mirrored about the street's edges, so the right copy is the street flipped, whole composition covered; draws cleanly | [], mirrored true, covered 518400 of 518400 | yes |
| 4_brick.png, Brick: the rows above and below the street pushed half a street across, whole composition covered; draws cleanly | [], pushed true, not 2_repeat.png true, covered 518400 of 518400 | yes |
| 5_repeat_blend_100.png, Repeat with Blend Borders 100: not 2_repeat.png, still the same one street to the right everywhere, whole composition covered; draws cleanly | [], different true, the same a street to the right at 259200 of 259200, covered 518400 of 518400 | yes |
| 6_unfold_right_only.png, Unfold grown half a street to the right only: the street and half a mirrored street beside it, nothing on the left; draws cleanly | [], covered 194400 (want 194400), left clear true | yes |
| 7_random.png and 8_twist.png: neither is 2_repeat.png nor each other, whole composition covered; draw cleanly | [] [], different true true true, covered 518400 518400 | yes |

## Result

187 of 187 checks pass.
