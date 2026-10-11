# D-465: Scatter

B-345: `core.scatter` takes After Effects' Scatter controls (Scatter Amount, Grain, Randomize Every Frame). Every expected pixel is `Fixtures/scatter/expected_scatter.json`, written by `tools/scatter_reference.py` before this code existed and printed in document 25 as FX-SCATTER-001 to 019. Tolerance 2e-5.

## FX-SCATTER-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SCATTER-001 frame 0: The settings as they start: Scatter Amount 10, both ways, the same jumble every frame. On a 6 by 4 drawing most pixels read from outside it and are clear. | largest difference 0.0e0 | yes |
| FX-SCATTER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-002 frame 0: Scatter Amount 1, both ways: each pixel from one of its nine neighbours or itself. | largest difference 1.9e-7 | yes |
| FX-SCATTER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-003 frame 0: Scatter Amount 2, both ways. | largest difference 1.9e-7 | yes |
| FX-SCATTER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-004 frame 0: Scatter Amount 2, Grain Horizontal: every pixel stays in its own row. | largest difference 1.9e-7 | yes |
| FX-SCATTER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-005 frame 0: Scatter Amount 2, Grain Vertical: every pixel stays in its own column. | largest difference 1.9e-7 | yes |
| FX-SCATTER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-006 frame 0: Scatter Amount 0: the picture as it was. | largest difference 1.9e-7 | yes |
| FX-SCATTER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-007 frame 0: Scatter Amount 0.4, which rounds to no reach: the picture as it was. | largest difference 1.9e-7 | yes |
| FX-SCATTER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-008 frame 0: Scatter Amount 0.5, which rounds to a reach of 1: FX-SCATTER-002. | largest difference 1.9e-7 | yes |
| FX-SCATTER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-009 frame 0: Scatter Amount 2, Randomize Every Frame on: a new jumble each frame; frame 0 is FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-009 frame 1: Scatter Amount 2, Randomize Every Frame on: a new jumble each frame; frame 0 is FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-009 frame 2: Scatter Amount 2, Randomize Every Frame on: a new jumble each frame; frame 0 is FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-010 frame 0: Scatter Amount 2, Randomize Every Frame off: frame 3 is frame 0. | largest difference 1.9e-7 | yes |
| FX-SCATTER-010 frame 3: Scatter Amount 2, Randomize Every Frame off: frame 3 is frame 0. | largest difference 1.9e-7 | yes |
| FX-SCATTER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-011 frame 0: Scatter Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 1 is 1, frame 2 is 2 and FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-011 frame 1: Scatter Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 1 is 1, frame 2 is 2 and FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-011 frame 2: Scatter Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 1 is 1, frame 2 is 2 and FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-011 frame 4: Scatter Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 1 is 1, frame 2 is 2 and FX-SCATTER-003. | largest difference 1.9e-7 | yes |
| FX-SCATTER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-012 frame 0: Scatter Amount keyed from 100 at frame 0 to 127 at frame 4, eased past its end: frame 2 would pass 127, is held at 127, and is frame 4. | largest difference 0.0e0 | yes |
| FX-SCATTER-012 frame 2: Scatter Amount keyed from 100 at frame 0 to 127 at frame 4, eased past its end: frame 2 would pass 127, is held at 127, and is frame 4. | largest difference 0.0e0 | yes |
| FX-SCATTER-012 frame 4: Scatter Amount keyed from 100 at frame 0 to 127 at frame 4, eased past its end: frame 2 would pass 127, is held at 127, and is frame 4. | largest difference 0.0e0 | yes |
| FX-SCATTER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-013 frame 0: FX-SCATTER-003 moved three pixels right: the jumble moves with the drawing. | largest difference 1.9e-7 | yes |
| FX-SCATTER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-014 frame 0: After a drop shadow that grows the layer to 10 by 8, Scatter Amount 1: the shadow is jumbled with the drawing. | largest difference 1.9e-7 | yes |
| FX-SCATTER-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATTER-015 frame 0: Scatter Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-015 frame 4: Scatter Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATTER-016 frame 0: Scatter Amount 128, above 127. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-016 frame 4: Scatter Amount 128, above 127. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATTER-017 frame 0: Grain "diagonal", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-017 frame 4: Grain "diagonal", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATTER-018 frame 0: Randomize Every Frame "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-018 frame 4: Randomize Every Frame "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATTER-019 frame 0: Scatter Amount keyed to 200 at frame 4, above 127. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-019 frame 4: Scatter Amount keyed to 200 at frame 4, above 127. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCATTER-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scatter_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatter_009.json is saved with its three settings, and not the frame | {"grain":"both","randomize":"on","scatter_amount":2} | yes |
| fx_scatter_015.json is refused in a sentence naming Scatter and its scatter amount | Scatter's scatter amount runs from 0 to 127, and this is -1. | yes |
| fx_scatter_016.json is refused in a sentence naming Scatter and its scatter amount | Scatter's scatter amount runs from 0 to 127, and this is 128. | yes |
| fx_scatter_017.json is refused in a sentence naming Scatter and its grain | Scatter's grain is one of both, horizontal, vertical, and this is "diagonal". | yes |
| fx_scatter_018.json is refused in a sentence naming Scatter and its randomize every frame | Scatter's randomize every frame is "on" or "off", and this is "sometimes". | yes |
| a file with a Scatter whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scatter without its randomize is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scatter whose grain is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scatter amount 128 is refused with a sentence, and nothing changes | Scatter's scatter amount runs from 0 to 127, and this is 128. | yes |
| grain diagonal is refused with a sentence, and nothing changes | Scatter's grain is one of both, horizontal, vertical, and this is "diagonal". | yes |
| randomize sometimes is refused with a sentence, and nothing changes | Scatter's randomize every frame is "on" or "off", and this is "sometimes". | yes |
| scatter amount keyed to 200 is refused with a sentence, and nothing changes | Scatter's scatter amount runs from 0 to 127, and this is 200. | yes |
| amount 7.5, vertical, a new jumble each frame is taken | taken | yes |
| scatter amount keyed from 0 to 127 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scatter_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatter_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatter_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatter_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatter_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scatter_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 7 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatter_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Scatter as added, amount 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1699055 pixels changed | yes |
| the reference shot, Scatter as added, amount 10 on three layers, frame 0, Full | largest difference 1 of 255, 3837 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter as added, amount 10 on three layers, frame 100, Full | largest difference 1 of 255, 1392 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter as added, amount 10 on three layers, frame 239, Full | largest difference 1 of 255, 1776 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter as added, amount 10 on three layers, frame 0, Draft | largest difference 1 of 255, 141 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter as added, amount 10 on three layers, frame 100, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter as added, amount 10 on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 40, Horizontal: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1644262 pixels changed | yes |
| the reference shot, Scatter amount 40, Horizontal on three layers, frame 0, Full | largest difference 1 of 255, 3970 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 40, Horizontal on three layers, frame 100, Full | largest difference 1 of 255, 1405 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 40, Horizontal on three layers, frame 239, Full | largest difference 1 of 255, 1831 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 40, Horizontal on three layers, frame 0, Draft | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 40, Horizontal on three layers, frame 100, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 40, Horizontal on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 127, Vertical: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1935245 pixels changed | yes |
| the reference shot, Scatter amount 127, Vertical on three layers, frame 0, Full | largest difference 1 of 255, 4529 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 127, Vertical on three layers, frame 100, Full | largest difference 1 of 255, 1373 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 127, Vertical on three layers, frame 239, Full | largest difference 1 of 255, 1728 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 127, Vertical on three layers, frame 0, Draft | largest difference 1 of 255, 173 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 127, Vertical on three layers, frame 100, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 127, Vertical on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1837274 pixels changed | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3 on three layers, frame 0, Full | largest difference 1 of 255, 3944 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3 on three layers, frame 100, Full | largest difference 1 of 255, 1390 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3 on three layers, frame 239, Full | largest difference 1 of 255, 1802 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3 on three layers, frame 0, Draft | largest difference 1 of 255, 151 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3 on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatter amount 25, Randomize Every Frame on, frame 3 on three layers, frame 239, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-465 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street filling the frame; draws cleanly | [], covered 129600 of 129600 | yes |
| 2_as_added.png, Scatter as added (amount 10): the street jumbled, every pixel the street's own from within 10 pixels; draws cleanly | [], unmoved 50206 of 129600, from within 10 true | yes |
| 3_amount_40.png, amount 40: jumbled further, not 2_as_added.png, the frame's edge frayed (some clear pixels along it, about 11 in 100 of the frame for a reach of 40 on 480 by 270, so more than 85 in 100 covered); draws cleanly | [], different true, covered 114759 of 129600 | yes |
| 4_horizontal_40.png, amount 40, Grain Horizontal: every pixel from its own row, within 40 across; draws cleanly | [], from its row true | yes |
| 5_vertical_40.png, amount 40, Grain Vertical: every pixel from its own column, within 40 down; draws cleanly | [], from its column true | yes |
| 6_randomize_frame_3.png: with Randomize Every Frame off, frame 3 is frame 0; on, frame 3 is a new jumble and frame 0 is 2_as_added.png; draw cleanly | [] [], off the same true, on different true, on frame 0 as added true | yes |

## Result

143 of 143 checks pass.
