# D-449: Block Load

B-329: `core.block_load` takes After Effects' CC Block Load controls (Completion, Scans, Start Cleared, Bilinear). Every expected pixel is `Fixtures/block_load/expected_block_load.json`, written by `tools/block_load_reference.py` before this code existed and printed in document 25 as FX-BLOCKLOAD-001 to 024. Tolerance 2e-5.

## FX-BLOCKLOAD-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLOCKLOAD-001 frame 0: The settings as they start: Completion 0, Scans 4, Start Cleared on: nothing shows yet. | largest difference 0.0e0 | yes |
| FX-BLOCKLOAD-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-002 frame 0: Completion 100: every pass done, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-003 frame 0: Scans 3, Start Cleared off, Completion 0: the roughest picture, blocks 8 pixels square, as Mosaic size 8 draws it (FX-MOSAIC-009). | largest difference 8.0e-8 | yes |
| FX-BLOCKLOAD-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-004 frame 0: Scans 2, Completion 20: three passes, the first, blocks 4 wide, a little over half under way: its first row of blocks, rows 0 to 3, drawn; below it nothing yet. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-005 frame 0: Scans 2, Completion 50: the second pass, blocks 2 wide, half under way: rows 0 to 3 in blocks 2 wide, rows 4 to 9 still in blocks 4 wide. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-006 frame 0: Scans 2, Start Cleared off, Completion 50: two passes, the first done and the last not begun: all in blocks 2 wide, Mosaic size 2. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-007 frame 0: Scans 2, Start Cleared off, Completion 78: the last pass a little over half way: rows 0 to 4 the drawing as it is, rows 5 to 9 in blocks 2 wide. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-008 frame 0: Scans 1, Start Cleared off, Completion 0: blocks 2 wide. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-009 frame 0: Scans 16, Start Cleared off, Completion 0: one block, the whole drawing's mean (FX-MOSAIC-007). | largest difference 8.0e-8 | yes |
| FX-BLOCKLOAD-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-010 frame 0: Scans 2.7: its whole part, FX-BLOCKLOAD-005. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-011 frame 0: Bilinear, Scans 2, Start Cleared off, Completion 0: the blocks 4 wide blended smoothly into each other instead of flat. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-012 frame 0: Bilinear, Scans 2, Completion 50: FX-BLOCKLOAD-005 blended. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-013 frame 0: Bilinear, Completion 100: the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-014 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, Scans 2: frame 0 nothing; frame 1 the first pass three quarters done, rows 0 to 7 in blocks 4 wide; frame 2 FX-BLOCKLOAD-005; frame 3 the last pass a quarter done, rows 0 and 1 as drawn, the rest in blocks 2 wide; frame 4 the drawing. | largest difference 0.0e0 | yes |
| FX-BLOCKLOAD-014 frame 1: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, Scans 2: frame 0 nothing; frame 1 the first pass three quarters done, rows 0 to 7 in blocks 4 wide; frame 2 FX-BLOCKLOAD-005; frame 3 the last pass a quarter done, rows 0 and 1 as drawn, the rest in blocks 2 wide; frame 4 the drawing. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-014 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, Scans 2: frame 0 nothing; frame 1 the first pass three quarters done, rows 0 to 7 in blocks 4 wide; frame 2 FX-BLOCKLOAD-005; frame 3 the last pass a quarter done, rows 0 and 1 as drawn, the rest in blocks 2 wide; frame 4 the drawing. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-014 frame 3: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, Scans 2: frame 0 nothing; frame 1 the first pass three quarters done, rows 0 to 7 in blocks 4 wide; frame 2 FX-BLOCKLOAD-005; frame 3 the last pass a quarter done, rows 0 and 1 as drawn, the rest in blocks 2 wide; frame 4 the drawing. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-014 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, Scans 2: frame 0 nothing; frame 1 the first pass three quarters done, rows 0 to 7 in blocks 4 wide; frame 2 FX-BLOCKLOAD-005; frame 3 the last pass a quarter done, rows 0 and 1 as drawn, the rest in blocks 2 wide; frame 4 the drawing. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-015 frame 0: Scans keyed from 1 at frame 0 to 3 at frame 4, eased past its end, Start Cleared off, Completion 0: blocks 2 wide at frame 0, then 4 or 8. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-015 frame 2: Scans keyed from 1 at frame 0 to 3 at frame 4, eased past its end, Start Cleared off, Completion 0: blocks 2 wide at frame 0, then 4 or 8. | largest difference 8.0e-8 | yes |
| FX-BLOCKLOAD-015 frame 4: Scans keyed from 1 at frame 0 to 3 at frame 4, eased past its end, Start Cleared off, Completion 0: blocks 2 wide at frame 0, then 4 or 8. | largest difference 8.0e-8 | yes |
| FX-BLOCKLOAD-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-016 frame 0: FX-BLOCKLOAD-005 moved three pixels right: the blocks are laid from the drawing's own corner, so they move with it. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-017 frame 0: After a directional blur (direction 90, length 4) that grows the layer two pixels on every side, Scans 2, Completion 50: the blocks are laid from the drawing's corner, and the pass's rows of blocks are the grown layer's, the two grown rows above the drawing a short row of their own. | largest difference 1.8e-7 | yes |
| FX-BLOCKLOAD-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOCKLOAD-018 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-018 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOCKLOAD-019 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-019 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOCKLOAD-020 frame 0: Scans 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-020 frame 4: Scans 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOCKLOAD-021 frame 0: Scans 17, above 16. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-021 frame 4: Scans 17, above 16. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOCKLOAD-022 frame 0: Start Cleared "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-022 frame 4: Start Cleared "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOCKLOAD-023 frame 0: Bilinear "On", in capitals, not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-023 frame 4: Bilinear "On", in capitals, not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOCKLOAD-024 frame 0: Completion keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-024 frame 4: Completion keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLOCKLOAD-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blockload_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blockload_011.json is saved with its four settings, and not the draft's pixel width | {"bilinear":"on","completion":0,"scans":2,"start_cleared":"off"} | yes |
| fx_blockload_018.json is refused in a sentence naming Block Load and its completion | Block Load's completion runs from 0 to 100, and this is 101. | yes |
| fx_blockload_019.json is refused in a sentence naming Block Load and its completion | Block Load's completion runs from 0 to 100, and this is -1. | yes |
| fx_blockload_020.json is refused in a sentence naming Block Load and its scans | Block Load's scans runs from 1 to 16, and this is 0.5. | yes |
| fx_blockload_021.json is refused in a sentence naming Block Load and its scans | Block Load's scans runs from 1 to 16, and this is 17. | yes |
| fx_blockload_022.json is refused in a sentence naming Block Load and its start cleared | Block Load's start cleared is "on" or "off", and this is "yes". | yes |
| fx_blockload_023.json is refused in a sentence naming Block Load and its bilinear | Block Load's bilinear is "on" or "off", and this is "On". | yes |
| a file with a Block Load whose scans is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Block Load without its completion is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Block Load whose bilinear is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scans 0.5 is refused with a sentence, and nothing changes | Block Load's scans runs from 1 to 16, and this is 0.5. | yes |
| bilinear On (a capital) is refused with a sentence, and nothing changes | Block Load's bilinear is "on" or "off", and this is "On". | yes |
| completion keyed to 150 is refused with a sentence, and nothing changes | Block Load's completion runs from 0 to 100, and this is 150. | yes |
| completion 60, bilinear on is taken | taken | yes |
| scans keyed from 1 to 16 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blockload_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blockload_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blockload_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blockload_014.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blockload_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blockload_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blockload_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_blockload_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Block Load as added (nothing yet): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Block Load as added (nothing yet) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load as added (nothing yet) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load as added (nothing yet) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load as added (nothing yet) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load as added (nothing yet) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load as added (nothing yet) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load completion 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1443945 pixels changed | yes |
| the reference shot, Block Load completion 50 on three layers, frame 0, Full | largest difference 1 of 255, 1336 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load completion 50 on three layers, frame 100, Full | largest difference 1 of 255, 568 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load completion 50 on three layers, frame 239, Full | largest difference 1 of 255, 668 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load completion 50 on three layers, frame 0, Draft | largest difference 1 of 255, 120 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load completion 50 on three layers, frame 100, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load completion 50 on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1771344 pixels changed | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear on three layers, frame 0, Full | largest difference 1 of 255, 910 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear on three layers, frame 100, Full | largest difference 1 of 255, 1012 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear on three layers, frame 239, Full | largest difference 1 of 255, 794 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear on three layers, frame 100, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Block Load start cleared off, scans 6, completion 30, bilinear on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-449 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| as added (Completion 0, Start Cleared on): nothing shows yet, every pixel clear; draws cleanly (no picture: it would be empty) | [], pixels with any cover 0 | yes |
| 2_coarsest.png, Start Cleared off, Completion 0, Scans 4: the street in blocks 16 pixels square, the same picture as Mosaic size 16; draws cleanly | [], the same as Mosaic 16: true | yes |
| 3_half_way.png, Completion 50: the top 136 rows in blocks 4 pixels square (Mosaic 4), the rows below still in blocks 8 square (Mosaic 8); draws cleanly | [], top as Mosaic 4: true, below as Mosaic 8: true | yes |
| 4_half_way_bilinear.png, Completion 50, Bilinear: 3_half_way.png with the blocks blended smoothly, so fewer sudden jumps (more than 24 levels) between neighbours; draws cleanly | [], neighbour pairs differing by more than 24 in red: flat 5285, blended 285 | yes |
| 5_nearly_done.png, Completion 90: the top 135 rows the street as it is, the rows below in blocks 2 pixels square (Mosaic 2); draws cleanly | [], top as the street: true, below as Mosaic 2: true | yes |
| Completion 100: the street byte for byte; draws cleanly | [], the same: true | yes |

## Result

159 of 159 checks pass.
