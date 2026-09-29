# B-149: Block Dissolve

D-214, accepted on 2026-09-28 with the After Effects picks (B12): After Effects' Block Dissolve in purpose and names, by this program's own rule; its Soft Edges switch is left out. Every expected pixel is `Fixtures/block_dissolve/expected_block_dissolve.json`, written by `tools/block_dissolve_reference.py` before this code existed and printed in document 25 as FX-BDISSOLVE-001 to 019. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BDISSOLVE-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BDISSOLVE-001 frame 0: The settings as they start, completion 0, blocks 1 by 1, feather 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-002 frame 0: Completion 50, blocks 1 by 1: each pixel kept exactly or gone, about half of those shown gone, scattered. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-003 frame 0: Completion 50, blocks 4 by 4: each block of four columns and four rows from the top left is kept exactly or gone whole; the bottom row of blocks is two rows tall, cut by the drawing's edge. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-004 frame 0: Completion 50, blocks 8 wide by 2 tall: flat bricks, each kept or gone whole. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-005 frame 0: Completion 100, blocks 4 by 4, feather 3: every pixel is transparent, the feather too. | largest difference 0.0e0 | yes |
| FX-BDISSOLVE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-006 frame 0: Completion 25, blocks 4 by 4: fewer blocks gone, and every block gone here is gone in FX-BDISSOLVE-003 too. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-007 frame 0: Completion 50, blocks 4 by 4, feather 2: a pixel whose square of 2 lies inside one block is kept exactly or gone, as in FX-BDISSOLVE-003; one next to a block of the other kind is part kept, a soft edge. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-008 frame 0: Completion 50, blocks 4 by 4, feather 40: the square is ten blocks across, so every shown pixel is kept at nearly the same share, the drawing fading almost evenly. | largest difference 1.3e-7 | yes |
| FX-BDISSOLVE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-009 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, blocks 4 by 4, linear: frame 0 untouched, frame 2 is FX-BDISSOLVE-003, frame 4 is empty, and a block once gone never comes back. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-009 frame 1: Completion keyed from 0 at frame 0 to 100 at frame 4, blocks 4 by 4, linear: frame 0 untouched, frame 2 is FX-BDISSOLVE-003, frame 4 is empty, and a block once gone never comes back. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-009 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, blocks 4 by 4, linear: frame 0 untouched, frame 2 is FX-BDISSOLVE-003, frame 4 is empty, and a block once gone never comes back. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-009 frame 3: Completion keyed from 0 at frame 0 to 100 at frame 4, blocks 4 by 4, linear: frame 0 untouched, frame 2 is FX-BDISSOLVE-003, frame 4 is empty, and a block once gone never comes back. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-009 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, blocks 4 by 4, linear: frame 0 untouched, frame 2 is FX-BDISSOLVE-003, frame 4 is empty, and a block once gone never comes back. | largest difference 0.0e0 | yes |
| FX-BDISSOLVE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-010 frame 0: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, blocks 4 by 4: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both every pixel transparent. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-010 frame 2: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, blocks 4 by 4: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both every pixel transparent. | largest difference 0.0e0 | yes |
| FX-BDISSOLVE-010 frame 4: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, blocks 4 by 4: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both every pixel transparent. | largest difference 0.0e0 | yes |
| FX-BDISSOLVE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-011 frame 0: Feather keyed from 0 at frame 0 to 4 at frame 4, completion 50, blocks 4 by 4: frame 0 is FX-BDISSOLVE-003, frame 2 is FX-BDISSOLVE-007, and frame 4 softer still. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-011 frame 2: Feather keyed from 0 at frame 0 to 4 at frame 4, completion 50, blocks 4 by 4: frame 0 is FX-BDISSOLVE-003, frame 2 is FX-BDISSOLVE-007, and frame 4 softer still. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-011 frame 4: Feather keyed from 0 at frame 0 to 4 at frame 4, completion 50, blocks 4 by 4: frame 0 is FX-BDISSOLVE-003, frame 2 is FX-BDISSOLVE-007, and frame 4 softer still. | largest difference 1.5e-7 | yes |
| FX-BDISSOLVE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-012 frame 0: Completion 50, blocks 5 wide by 3 tall, which do not fit the drawing evenly: the bottom row of blocks is one row tall, cut by the drawing's edge. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-013 frame 0: FX-BDISSOLVE-003 moved three pixels right: the blocks are the drawing's own, so it is the same, moved. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BDISSOLVE-014 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-014 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BDISSOLVE-015 frame 0: Completion 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-015 frame 4: Completion 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BDISSOLVE-016 frame 0: Block width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-016 frame 4: Block width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BDISSOLVE-017 frame 0: Block height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-017 frame 4: Block height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BDISSOLVE-018 frame 0: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-018 frame 4: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BDISSOLVE-019 frame 0: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-019 frame 4: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BDISSOLVE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Block Dissolve grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the block width, block height and feather, and leaves the completion | BlockDissolve { completion: 40.0, block_width: 6.0, block_height: 3.0, feather: 1.5 } | yes |
| a half-size draft preview never makes a block less than a pixel | BlockDissolve { completion: 40.0, block_width: 1.0, block_height: 1.0, feather: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bdissolve_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bdissolve_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `feather` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a block width that is a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion -1 is refused with a sentence, and nothing changes | Block Dissolve's completion runs from 0 to 100, and this is -1. | yes |
| completion 100.5 is refused with a sentence, and nothing changes | Block Dissolve's completion runs from 0 to 100, and this is 100.5. | yes |
| block width 0.5 is refused with a sentence, and nothing changes | Block Dissolve's block width runs from 1 to 10000, and this is 0.5. | yes |
| block height 10001 is refused with a sentence, and nothing changes | Block Dissolve's block height runs from 1 to 10000, and this is 10001. | yes |
| feather -1 is refused with a sentence, and nothing changes | Block Dissolve's feather runs from 0 to 10000, and this is -1. | yes |
| completion keyed to 150 is refused with a sentence, and nothing changes | Block Dissolve's completion runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| block width keyed from 1 to 20 is taken | taken | yes |
| feather keyed from 0 to 10 is taken | taken | yes |
| undo 5 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bdissolve_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bdissolve_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bdissolve_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bdissolve_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bdissolve_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bdissolve_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up title card, in `verification/B-149 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| card_start.png, as it starts: the card as it was; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 12096 as they were, 0 gone, 0 part gone (0 recoloured) | yes |
| card_quarter.png, completion 25 in blocks 10 by 10: about a quarter gone, whole blocks; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 9492 as they were, 2604 gone, 0 part gone (0 recoloured) | yes |
| card_half.png, completion 50: about half gone; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 6588 as they were, 5508 gone, 0 part gone (0 recoloured) | yes |
| card_three_quarters.png, completion 75: about three quarters gone; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 2804 as they were, 9292 gone, 0 part gone (0 recoloured) | yes |
| card_bricks.png, completion 50 in blocks 40 wide by 5 tall: long flat bricks, some gone; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 6400 as they were, 5696 gone, 0 part gone (0 recoloured) | yes |
| card_pixels.png, completion 50 in blocks 1 by 1: single pixels, about half gone; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 6066 as they were, 6030 gone, 0 part gone (0 recoloured) | yes |
| card_feather.png, completion 50 in blocks 10 by 10 with a feather of 8: soft-edged blocks, their colour kept; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 2200 as they were, 1280 gone, 8616 part gone (0 recoloured) | yes |
| card_gone.png, completion 100: nothing left; nothing appears where the card is empty; draws cleanly | [], of 12096 shown pixels 0 as they were, 12096 gone, 0 part gone (0 recoloured) | yes |
| every pixel gone at 25 is gone at 50, and every one gone at 50 is gone at 75: a block once gone stays gone | 6508 gone at 25, 9412 at 50, 13196 at 75 | yes |

## Result

103 of 103 checks pass.
