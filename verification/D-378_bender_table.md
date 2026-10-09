# D-378: Bender

B-257, after CycoreFX's CC Bender: the layer pushed sideways from an axis between Base and Top, the push growing along the axis in one of four shapes: a bend (straight on past the Top), a smooth sway in the middle (Marilyn), a sharp kink (Sharp) or a smooth step (Boxer). The formulas are this program's own. Every expected pixel is `Fixtures/bender/expected_bender.json`, written by `tools/bender_reference.py` before this code existed and printed in document 25 as FX-BENDER-001 to 023. Tolerance 2e-5.

## FX-BENDER-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BENDER-001 frame 0: The settings as they start: Amount 20, Bend, the axis from the middle of the bottom edge to the middle of the top. Each row is pushed right by 20 times the square of its height up the drawing, so the bottom rows barely move and the top ones are pushed clean off the drawing: a hard bend. | largest difference 2.5e-7 | yes |
| FX-BENDER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BENDER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-003 frame 0: Bend, Amount 4: row 1, the stripes' top row, pushed 2.89 pixels right, the band's rows 1.21 and 0.81, and row 8 0.09: the stripes curve over to the right. | largest difference 2.0e-7 | yes |
| FX-BENDER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-004 frame 0: Bend, Amount -4: the same push to the left. | largest difference 2.2e-7 | yes |
| FX-BENDER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-005 frame 0: Marilyn, Amount 4: the middle rows swell 4 pixels right and the top and bottom rows hardly move, a smooth bulge. | largest difference 2.5e-7 | yes |
| FX-BENDER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-006 frame 0: Sharp, Amount 4: the same swell as a triangle, its point in the middle. | largest difference 1.9e-7 | yes |
| FX-BENDER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-007 frame 0: Boxer, Amount 4: a smooth S from the bottom row to the top, the top pushed most nearly 4 pixels. | largest difference 2.5e-7 | yes |
| FX-BENDER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-008 frame 0: Adjust To Distance on, Amount 40: 40 per cent of the 10-pixel axis, 4 pixels: FX-BENDER-003 exactly. | largest difference 2.0e-7 | yes |
| FX-BENDER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-009 frame 0: Bend, Amount 2, the axis from row 7 up to row 3: rows 7 to 9 below the Base stay put, the rows to the Top curve, and the rows past it, 1 and 2, carry on straight, slanting further, 2 (2 s - 1) pixels. | largest difference 2.1e-7 | yes |
| FX-BENDER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-010 frame 0: Marilyn, Amount 2, the same short axis: only rows 3 to 6 swell; the rest is untouched. | largest difference 2.5e-7 | yes |
| FX-BENDER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-011 frame 0: Boxer, Amount 3, the axis lying across from the middle of the left edge to the middle of the right: n points down, so the columns are pushed down, the right ones 3 pixels. | largest difference 1.9e-7 | yes |
| FX-BENDER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-012 frame 0: Top and Base the same point: no axis, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-BENDER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-013 frame 0: Amount keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDER-003. | largest difference 1.9e-7 | yes |
| FX-BENDER-013 frame 2: Amount keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDER-003. | largest difference 2.0e-7 | yes |
| FX-BENDER-013 frame 4: Amount keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDER-003. | largest difference 2.5e-7 | yes |
| FX-BENDER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-014 frame 0: Bend, Amount 3, the Top keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the axis shortens and the same push bends harder. | largest difference 2.5e-7 | yes |
| FX-BENDER-014 frame 2: Bend, Amount 3, the Top keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the axis shortens and the same push bends harder. | largest difference 2.5e-7 | yes |
| FX-BENDER-014 frame 4: Bend, Amount 3, the Top keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the axis shortens and the same push bends harder. | largest difference 1.9e-7 | yes |
| FX-BENDER-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-015 frame 0: FX-BENDER-005 moved three pixels right: the same, moved; nothing grows, and the three columns left of the drawing stay empty. | largest difference 2.5e-7 | yes |
| FX-BENDER-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-016 frame 0: Sharp, Amount 0.5: half a pixel at most, a blend of neighbouring stripes rather than a move. | largest difference 1.9e-7 | yes |
| FX-BENDER-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-017 frame 0: Amount eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-BENDER-017 frame 2: Amount eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 0.0e0 | yes |
| FX-BENDER-017 frame 4: Amount eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 0.0e0 | yes |
| FX-BENDER-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDER-018 frame 0: Amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-018 frame 4: Amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDER-019 frame 0: Amount -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-019 frame 4: Amount -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDER-020 frame 0: A style written "Bend", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-020 frame 4: A style written "Bend", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDER-021 frame 0: Adjust To Distance written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-021 frame 4: Adjust To Distance written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDER-022 frame 0: Top 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-022 frame 4: Top 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDER-023 frame 0: Amount keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-023 frame 4: Amount keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDER-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bender_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bender_011.json is saved with its amount, style, adjust to distance, top and base | {"adjust_to_distance":"off","amount":3,"base":[0,50],"style":"boxer","top":[100,50]} | yes |
| fx_bender_018.json is refused in a sentence | Bender's amount runs from -1000 to 1000, and this is 1001. | yes |
| fx_bender_019.json is refused in a sentence | Bender's amount runs from -1000 to 1000, and this is -1001. | yes |
| fx_bender_020.json is refused in a sentence | Bender's style is bend, marilyn, sharp or boxer, and this is "Bend". | yes |
| fx_bender_021.json is refused in a sentence | Bender's adjust to distance is "on" or "off", and this is "yes". | yes |
| fx_bender_022.json is refused in a sentence | Bender's top runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Bender with no `style` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bender whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 1000.5 is refused with a sentence, and nothing changes | Bender's amount runs from -1000 to 1000, and this is 1000.5. | yes |
| base 50, -1000.5 is refused with a sentence, and nothing changes | Bender's base runs from -1000 to 1000, and this is -1000.5. | yes |
| style "wave" is refused with a sentence, and nothing changes | Bender's style is bend, marilyn, sharp or boxer, and this is "wave". | yes |
| adjust to distance "On", written with a capital is refused with a sentence, and nothing changes | Bender's adjust to distance is "on" or "off", and this is "On". | yes |
| amount keyed to 2000 is refused with a sentence, and nothing changes | Bender's amount runs from -1000 to 1000, and this is 2000. | yes |
| marilyn 35 per cent of a slanted axis is taken | taken | yes |
| amount keyed from 0 to 8 is taken | taken | yes |
| top keyed from 50, 0 to 50, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bender_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bender_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bender_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bender_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bender_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bender_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bender_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_bender_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bender_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_bender_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bender_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Bender as it starts (bend 20, up the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1308499 pixels changed | yes |
| the reference shot, Bender as it starts (bend 20, up the middle) on three layers, frame 0, Full | largest difference 1 of 255, 2477 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender as it starts (bend 20, up the middle) on three layers, frame 100, Full | largest difference 1 of 255, 869 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender as it starts (bend 20, up the middle) on three layers, frame 239, Full | largest difference 1 of 255, 1045 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender as it starts (bend 20, up the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender as it starts (bend 20, up the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender as it starts (bend 20, up the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender marilyn -60 on the middle half: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 844924 pixels changed | yes |
| the reference shot, Bender marilyn -60 on the middle half on three layers, frame 0, Full | largest difference 1 of 255, 3027 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender marilyn -60 on the middle half on three layers, frame 100, Full | largest difference 1 of 255, 930 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender marilyn -60 on the middle half on three layers, frame 239, Full | largest difference 1 of 255, 1465 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender marilyn -60 on the middle half on three layers, frame 0, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender marilyn -60 on the middle half on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender marilyn -60 on the middle half on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1555915 pixels changed | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis on three layers, frame 0, Full | largest difference 1 of 255, 1006 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis on three layers, frame 100, Full | largest difference 1 of 255, 790 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis on three layers, frame 239, Full | largest difference 1 of 255, 878 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis on three layers, frame 0, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis on three layers, frame 100, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender sharp 15 per cent of a slanted axis on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender boxer 40 across, left to right: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1569685 pixels changed | yes |
| the reference shot, Bender boxer 40 across, left to right on three layers, frame 0, Full | largest difference 1 of 255, 2198 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender boxer 40 across, left to right on three layers, frame 100, Full | largest difference 1 of 255, 956 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender boxer 40 across, left to right on three layers, frame 239, Full | largest difference 1 of 255, 938 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender boxer 40 across, left to right on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender boxer 40 across, left to right on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bender boxer 40 across, left to right on three layers, frame 239, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-378 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (bend 20 up the middle): the upper street swept 20 pixels right, the bottom row still; draws cleanly | [], 10899 pixels changed | yes |
| 3_amount_0.png, amount 0: nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 4_marilyn_40.png, marilyn 40: the middle of the street bulges 40 pixels right, top and bottom still; draws cleanly | [], 33940 pixels changed | yes |
| 5_boxer_across.png, boxer -30 across: the street steps smoothly up 30 pixels from left to right; draws cleanly | [], 83719 pixels changed | yes |

## Result

160 of 160 checks pass.
