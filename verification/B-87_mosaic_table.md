# B-87: mosaic

D-144, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the eleventh of the third batch. Every expected pixel is `Fixtures/mosaic/expected_mosaic.json`, written by `tools/mosaic_reference.py` before this code existed and printed in document 25 as FX-MOSAIC-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MOSAIC-001 to 020 (document 25)

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

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the size, 10 to 5 | Mosaic { size: 5.0 } | yes |
| a quarter-size draft of size 1.5 holds the size at 1, which changes nothing, rather than dropping the effect | Mosaic { size: 1.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mosaic_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mosaic_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `size` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a size that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 0.5 is refused with a sentence, and nothing changes | Mosaic's size runs from 1 to 1000, and this is 0.5. | yes |
| size 1001 is refused with a sentence, and nothing changes | Mosaic's size runs from 1 to 1000, and this is 1001. | yes |
| size keyed to 0 is refused with a sentence, and nothing changes | Mosaic's size runs from 1 to 1000, and this is 0. | yes |
| size 1, the bottom, is taken | taken | yes |
| size 1000, the top, is taken | taken | yes |
| size keyed from 2 to 6 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mosaic_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mosaic_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

76 of 76 checks pass.
