# B-633: Mosaic gains After Effects' Horizontal and Vertical Blocks and Sharp Colors

D-754, accepted on 2026-10-23 by the owner's message "Proceed, if an effect is more like a inspiration/modified version of a AE effect, only improve it." Mosaic is inspired by After Effects' (D-144), so nothing it drew before changes: size mode and its fixtures stay, and a file that names no mode is size mode. Mode count splits the drawing's own box evenly into Horizontal by Vertical blocks; Sharp Colors gives each block its middle pixel instead of the mean. Every expected pixel is `Fixtures/mosaic/expected_mosaic.json`, written by `tools/mosaic_reference.py` before this code and printed in document 25 as FX-MOSAIC-001 to 037. The build's frame is compared sample by sample against the catalogue's tolerance of 2e-5.

## FX-MOSAIC-001 to 037 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MOSAIC-001 frame 0: The settings as they start, size 10: two blocks across, columns 0 to 9 and a block cut short at the right edge, columns 10 to 15, each the full height and one flat colour, the mean of its pixels; both are partly empty, so both come out partly covered, the left 63.5 of its 100 pixels' worth, the right 33 of its 60, and the frame keeps each channel's total. | largest difference 9.2e-8 | yes |
| FX-MOSAIC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-002 frame 0: Size 1, the least: one-pixel blocks, and the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-003 frame 0: Size 2: blocks of two by two, eight across and five down; the soft edge down column 3 is averaged with the empty column 2 beside it, to a quarter covering. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-004 frame 0: Size 4: blocks of four by four, the bottom row of blocks cut short to rows 8 and 9; a block all of skin, columns 4 to 7 and rows 0 to 3, stays skin; the empty top right block stays empty; the top left block, empty but for four soft pixels, is skin at a quarter of their half covering. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-005 frame 0: Size 3: 16 and 10 are each one more than a multiple of 3, so column 15 and row 9 are blocks one pixel wide, and the corner pixel (15, 9), a block of its own, is exactly the drawing's white. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-006 frame 0: Size 2.5, part of a pixel: the blocks are three and two pixels wide by turns, columns 0 to 2, 3 and 4, 5 to 7, 8 and 9, 10 to 12, 13 and 14, and 15 alone; rows 0 to 2, 3 and 4, 5 to 7, 8 and 9. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-007 frame 0: Size 16, the drawing's width: one block, the whole frame, every pixel the drawing's mean colour at its mean covering. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-008 frame 0: Size 1000, the most: the same single block as size 16, FX-MOSAIC-007. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-009 frame 0: Size 8: two blocks across, columns 0 to 7 and 8 to 15, and two down, rows 0 to 7 and a short row of blocks, rows 8 and 9, which are the white strip's and its empty start, so the bottom left block is white at half covering and the bottom right white. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-010 frame 0: Size keyed from 2 at frame 0 to 6 at frame 4, linear: frame 0 is FX-MOSAIC-003, frame 2 is size 4, FX-MOSAIC-004, and frame 4 size 6. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-010 frame 2: Size keyed from 2 at frame 0 to 6 at frame 4, linear: frame 0 is FX-MOSAIC-003, frame 2 is size 4, FX-MOSAIC-004, and frame 4 size 6. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-010 frame 4: Size keyed from 2 at frame 0 to 6 at frame 4, linear: frame 0 is FX-MOSAIC-003, frame 2 is size 4, FX-MOSAIC-004, and frame 4 size 6. | largest difference 2.0e-7 | yes |
| FX-MOSAIC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-011 frame 0: Size keyed from 1 at frame 0 to 1000 at frame 4, eased past its end: at frame 2 it would pass 1000, is held at 1000, and is FX-MOSAIC-008, as frame 4 is; frame 0 is the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-011 frame 2: Size keyed from 1 at frame 0 to 1000 at frame 4, eased past its end: at frame 2 it would pass 1000, is held at 1000, and is FX-MOSAIC-008, as frame 4 is; frame 0 is the drawing untouched. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-011 frame 4: Size keyed from 1 at frame 0 to 1000 at frame 4, eased past its end: at frame 2 it would pass 1000, is held at 1000, and is FX-MOSAIC-008, as frame 4 is; frame 0 is the drawing untouched. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-012 frame 0: Size 4, the layer moved three pixels right: FX-MOSAIC-004 moved; the blocks are laid from the drawing's own corner, so they move with it, and the three columns left of it stay empty. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-012 frame 3: Size 4, the layer moved three pixels right: FX-MOSAIC-004 moved; the blocks are laid from the drawing's own corner, so they move with it, and the three columns left of it stay empty. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-013 frame 0: A directional blur, direction 90 and length 4, then size 4: the blur grew the layer two pixels on every side, and the blocks are still laid from the drawing's own corner, not the grown layer's, so the blocks in the frame are FX-MOSAIC-004's, averaged from the blurred pixels, and the bottom row of blocks, rows 8 to 11, takes in the two grown rows below the frame, which the sideways blur left empty, so it comes out half as covered as rows 8 and 9 are. | largest difference 1.8e-7 | yes |
| FX-MOSAIC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-014 frame 0: Size 10.5: blocks columns 0 to 10 and 11 to 15, rows 0 to 9 one block: the part of a pixel moves the block's edge past column 10. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-015 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-015 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-016 frame 0: Size 0.5, below 1 by half a pixel. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-016 frame 4: Size 0.5, below 1 by half a pixel. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-017 frame 0: Size -10, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-017 frame 4: Size -10, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-018 frame 0: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-018 frame 4: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-019 frame 0: Size keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-019 frame 4: Size keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-020 frame 0: Size keyed to 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-020 frame 4: Size keyed to 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-021 frame 0: Mode count, 10 by 10, Sharp Colors off, as a Mosaic starts when it is added: the drawing's 16 columns split evenly into 10 blocks, column x in block floor(10x / 16), so they are 2, 2, 1, 2, 1, 2, 2, 1, 2 and 1 columns wide, and its 10 rows into 10 blocks one row tall; each block the mean of its pixels. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-022 frame 0: Mode count, 4 by 2: blocks four columns wide and five rows tall, the drawing split evenly with nothing left over, where size mode's square blocks leave a strip. | largest difference 1.7e-7 | yes |
| FX-MOSAIC-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-023 frame 0: Mode count, 3 by 3: 16 and 10 do not divide by 3, so the blocks are columns 0 to 5, 6 to 10 and 11 to 15, and rows 0 to 3, 4 to 6 and 7 to 9, as near even as whole pixels allow. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-024 frame 0: Mode count, 1 by 1: one block, the whole drawing, the same as FX-MOSAIC-007. | largest difference 8.0e-8 | yes |
| FX-MOSAIC-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-025 frame 0: Mode count, 16 by 10, one block a pixel: the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-026 frame 0: Size 4 with Sharp Colors on: FX-MOSAIC-004's blocks, each taking the pixel at its middle, column and row start + 2, rather than the mean, so every block is one of the drawing's own colours; the bottom row of blocks, rows 8 and 9, cut short, takes row 9. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-027 frame 0: Mode count, 4 by 2, Sharp Colors on: FX-MOSAIC-022's blocks, each the pixel at its middle, columns 2, 6, 10 and 14 and rows 2 and 7. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-028 frame 0: Mode count, 4 by 2, the layer moved three pixels right: FX-MOSAIC-022 moved; the blocks split the drawing's own box, so they move with it. | largest difference 1.7e-7 | yes |
| FX-MOSAIC-028 frame 3: Mode count, 4 by 2, the layer moved three pixels right: FX-MOSAIC-022 moved; the blocks split the drawing's own box, so they move with it. | largest difference 1.7e-7 | yes |
| FX-MOSAIC-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-029 frame 0: A directional blur, direction 90 and length 4, then mode count, 4 by 2: the blur grew the layer two pixels on every side, and the counts split the drawing's own box, not the grown one, so the blocks in the frame are FX-MOSAIC-022's, averaged from the blurred pixels; the grown rows above and below are blocks of their own, which the sideways blur left empty. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-030 frame 0: Mode count, Horizontal Blocks keyed from 2 at frame 0 to 6 at frame 4, linear, Vertical Blocks 1: frame 0 is two blocks across, frame 2 four (FX-MOSAIC-022 across, one block down) and frame 4 six. | largest difference 5.2e-8 | yes |
| FX-MOSAIC-030 frame 2: Mode count, Horizontal Blocks keyed from 2 at frame 0 to 6 at frame 4, linear, Vertical Blocks 1: frame 0 is two blocks across, frame 2 four (FX-MOSAIC-022 across, one block down) and frame 4 six. | largest difference 1.3e-7 | yes |
| FX-MOSAIC-030 frame 4: Mode count, Horizontal Blocks keyed from 2 at frame 0 to 6 at frame 4, linear, Vertical Blocks 1: frame 0 is two blocks across, frame 2 four (FX-MOSAIC-022 across, one block down) and frame 4 six. | largest difference 1.3e-7 | yes |
| FX-MOSAIC-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-031 frame 0: Mode count, 2.6 by 3.4: a count is taken to the nearest whole number, so this is 3 by 3, FX-MOSAIC-023. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-032 frame 0: Size 4 with the mode written as size and Sharp Colors off: the same as FX-MOSAIC-004, which names neither, as every file from before D-754 is read. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MOSAIC-033 frame 0: Mode "grid", not one of size and count. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-033 frame 4: Mode "grid", not one of size and count. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-034 frame 0: Sharp Colors "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-034 frame 4: Sharp Colors "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-035 frame 0: Mode count, Horizontal Blocks 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-035 frame 4: Mode count, Horizontal Blocks 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-036 frame 0: Mode count, Vertical Blocks 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-036 frame 4: Mode count, Vertical Blocks 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MOSAIC-037 frame 0: Mode size, Horizontal Blocks keyed to 0.5 at frame 4: out of range even where size mode does not use it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-037 frame 4: Mode size, Horizontal Blocks keyed to 0.5 at frame 4: out of range even where size mode does not use it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MOSAIC-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A project saved before D-754

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mosaic_001.json, written before D-754 with only a size, opens in size mode, Sharp Colors off | EffectInstance { instance_id: Id("fx-0-0"), enabled: true, effect: Mosaic { size: 10.0, mode: "size", horizontal: 10.0, vertical: 10.0, sharp_colors: "off" }, tracks: {}, mix: 100.0, record: FileRecord(Some(Object {"enabled": Bool(true), "instance_id": String("fx-0-0"), "parameters": Object {"size": Number(10)}, "type_id": String("core.mosaic")})) } | yes |
| saving it again writes only the size | {"size":10} | yes |
| a file that wrote mode size and Sharp Colors off keeps them when saved | {"horizontal":10,"mode":"size","sharp_colors":"off","size":4,"vertical":10} | yes |
| a half-size draft halves the size but keeps the counts, so it shows the same blocks | Mosaic { size: 5.0, mode: "count", horizontal: 10.0, vertical: 4.0, sharp_colors: "off" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mosaic_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a mode that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with Horizontal Blocks that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| mode "grid" is refused with a sentence, and nothing changes | Mosaic's mode is "size" or "count", and this is "grid". | yes |
| mode "Count" is refused with a sentence, and nothing changes | Mosaic's mode is "size" or "count", and this is "Count". | yes |
| Sharp Colors "yes" is refused with a sentence, and nothing changes | Mosaic's sharp colors is "on" or "off", and this is "yes". | yes |
| Horizontal Blocks 0 is refused with a sentence, and nothing changes | Mosaic's horizontal runs from 1 to 1000, and this is 0. | yes |
| Vertical Blocks 1001 is refused with a sentence, and nothing changes | Mosaic's vertical runs from 1 to 1000, and this is 1001. | yes |
| Horizontal Blocks keyed to 0.5 is refused with a sentence, and nothing changes | Mosaic's horizontal runs from 1 to 1000, and this is 0.5. | yes |
| mode count, 10 by 10, is taken | taken | yes |
| 1 by 1, the bottom, is taken | taken | yes |
| 1000 by 1000, the top, is taken | taken | yes |
| Sharp Colors on, is taken | taken | yes |
| mode size, is taken | taken | yes |
| Vertical Blocks keyed from 2 to 6 is taken | taken | yes |
| undo 6 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mosaic_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_029.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MOSAIC-001, fx_mosaic_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-002, fx_mosaic_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-003, fx_mosaic_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-004, fx_mosaic_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-005, fx_mosaic_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-006, fx_mosaic_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-007, fx_mosaic_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-008, fx_mosaic_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-009, fx_mosaic_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-010, fx_mosaic_010.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-011, fx_mosaic_011.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-012, fx_mosaic_012.json, frames [0, 3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-013, fx_mosaic_013.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-014, fx_mosaic_014.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-015, fx_mosaic_015.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-016, fx_mosaic_016.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-017, fx_mosaic_017.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-018, fx_mosaic_018.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-019, fx_mosaic_019.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-020, fx_mosaic_020.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-021, fx_mosaic_021.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-022, fx_mosaic_022.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-023, fx_mosaic_023.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-024, fx_mosaic_024.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-025, fx_mosaic_025.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-026, fx_mosaic_026.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-027, fx_mosaic_027.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-028, fx_mosaic_028.json, frames [0, 3] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-029, fx_mosaic_029.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-030, fx_mosaic_030.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-031, fx_mosaic_031.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-032, fx_mosaic_032.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 1 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-033, fx_mosaic_033.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-034, fx_mosaic_034.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-035, fx_mosaic_035.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-036, fx_mosaic_036.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MOSAIC-037, fx_mosaic_037.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Result

166 of 166 checks pass.
