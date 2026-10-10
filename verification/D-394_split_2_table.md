# D-394: Split 2

B-273, after CycoreFX's CC Split 2: Split (D-393) under a second name, the side of the tear on your left as you walk from Point A to Point B opened by Split 1 and the side on your right by Split 2. The owner chose "Second name, one engine (Recommended)" on 2026-10-09. The formulas are this program's own. Every expected pixel is `Fixtures/split_2/expected_split_2.json`, written by `tools/split2_reference.py` before this code existed and printed in document 25 as FX-SPLIT2-001 to 019. Tolerance 2e-5.

## FX-SPLIT2-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPLIT2-001 frame 0: The settings as they start: from 25, 50 to 75, 50, both sides 50: Split's picture as it starts (FX-SPLIT-001). | largest difference 1.9e-7 | yes |
| FX-SPLIT2-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-002 frame 0: Both sides 4: Split's picture with split 4 (FX-SPLIT-002). | largest difference 2.5e-7 | yes |
| FX-SPLIT2-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-003 frame 0: Split 1 4, Split 2 0: only the upper side opens, the rows above the line pushed up and squeezed; every row below the line untouched. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-004 frame 0: Split 1 0, Split 2 4: only the lower side opens; every row above the line untouched. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-005 frame 0: Both sides 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-006 frame 0: Both points at 50, 50: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-007 frame 0: From 50, 10 to 50, 90, Split 1 4, Split 2 0: walking down the screen your left is the right of the picture, so only the right half opens. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-008 frame 0: From 10, 10 to 90, 90, Split 1 3, Split 2 6: torn along the diagonal, the lower left side opened twice as far. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-009 frame 0: Split 1 keyed from 0 at frame 0 to 8 at frame 4, linear, Split 2 4: frame 0 FX-SPLIT2-004, frame 2 FX-SPLIT2-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-009 frame 2: Split 1 keyed from 0 at frame 0 to 8 at frame 4, linear, Split 2 4: frame 0 FX-SPLIT2-004, frame 2 FX-SPLIT2-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-009 frame 4: Split 1 keyed from 0 at frame 0 to 8 at frame 4, linear, Split 2 4: frame 0 FX-SPLIT2-004, frame 2 FX-SPLIT2-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-010 frame 0: Split 2 keyed from 0 at frame 0 to 8 at frame 4, Split 1 0: frame 0 the drawing, frame 2 FX-SPLIT2-004. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-010 frame 2: Split 2 keyed from 0 at frame 0 to 8 at frame 4, Split 1 0: frame 0 the drawing, frame 2 FX-SPLIT2-004. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-010 frame 4: Split 2 keyed from 0 at frame 0 to 8 at frame 4, Split 1 0: frame 0 the drawing, frame 2 FX-SPLIT2-004. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-011 frame 0: FX-SPLIT2-003 moved three pixels right: the same, moved; nothing grows. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-012 frame 0: Split 2 eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, Split 1 0: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-012 frame 2: Split 2 eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, Split 1 0: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-012 frame 4: Split 2 eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, Split 1 0: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-013 frame 0: FX-SPLIT2-003 with the points swapped: the sides swap, so the lower side opens, FX-SPLIT2-004's picture. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-014 frame 0: From -50, 50 to 150, 50, Split 1 6, Split 2 2: the points off the drawing, the tear right across it, opened 3 up and 1 down at the middle. | largest difference 2.5e-7 | yes |
| FX-SPLIT2-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT2-015 frame 0: Split 1 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-015 frame 4: Split 1 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT2-016 frame 0: Split 2 -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-016 frame 4: Split 2 -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT2-017 frame 0: Point A at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-017 frame 4: Point A at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT2-018 frame 0: Point B at 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-018 frame 4: Point B at 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT2-019 frame 0: Split 2 keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-019 frame 4: Split 2 keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT2-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Split draws exactly as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| Split's street picture 2_as_added.png, drawn again, is the committed D-393 file byte for byte | 14006 bytes now, 14006 committed | yes |
| Split's street picture 3_wide.png, drawn again, is the committed D-393 file byte for byte | 20497 bytes now, 20497 committed | yes |
| Split's street picture 4_diagonal.png, drawn again, is the committed D-393 file byte for byte | 24860 bytes now, 24860 committed | yes |
| Split 2 with equal sides, as they start, both 50, on the street is Split's picture byte for byte | 0 pixels differ | yes |
| Split 2 with equal sides, both 150, on the street is Split's picture byte for byte | 0 pixels differ | yes |
| Split 2 with equal sides, corner to corner, both 40, on the street is Split's picture byte for byte | 0 pixels differ | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_split2_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split2_008.json is saved with its two points and both splits | {"point_a":[10,10],"point_b":[90,90],"split_1":3,"split_2":6} | yes |
| fx_split2_015.json is refused in a sentence | Split 2's split 1 runs from 0 to 1000, and this is 1001. | yes |
| fx_split2_016.json is refused in a sentence | Split 2's split 2 runs from 0 to 1000, and this is -1. | yes |
| fx_split2_017.json is refused in a sentence | Split 2's point a runs from -1000 to 1000, and this is 1001. | yes |
| fx_split2_018.json is refused in a sentence | Split 2's point b runs from -1000 to 1000, and this is -1001. | yes |
| a file with a Split 2 with no `split_2` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Split 2 with Split's one `split` instead is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| split 1 1000.5 is refused with a sentence, and nothing changes | Split 2's split 1 runs from 0 to 1000, and this is 1000.5. | yes |
| split 2 -0.5 is refused with a sentence, and nothing changes | Split 2's split 2 runs from 0 to 1000, and this is -0.5. | yes |
| split 2 keyed to -10 is refused with a sentence, and nothing changes | Split 2's split 2 runs from 0 to 1000, and this is -10. | yes |
| from 20, 30 to 80, 70, split 1 35, split 2 10 is taken | taken | yes |
| split 1 keyed from 0 to 200 is taken | taken | yes |
| point b keyed from 75, 50 to 75, 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_split2_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split2_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split2_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split2_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split2_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split2_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_split2_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split2_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split2_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_split2_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_split2_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split2_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split2_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split2_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split2_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split2_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 734555 pixels changed | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50) on three layers, frame 0, Full | largest difference 1 of 255, 2007 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50) on three layers, frame 100, Full | largest difference 1 of 255, 781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50) on three layers, frame 239, Full | largest difference 1 of 255, 808 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50) on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50) on three layers, frame 100, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 as it starts (25, 50 to 75, 50, both 50) on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1712086 pixels changed | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40 on three layers, frame 0, Full | largest difference 1 of 255, 1271 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40 on three layers, frame 100, Full | largest difference 1 of 255, 852 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40 on three layers, frame 239, Full | largest difference 1 of 255, 588 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40 on three layers, frame 0, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40 on three layers, frame 100, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 down a diagonal, split 1 200, split 2 40 on three layers, frame 239, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 983514 pixels changed | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30 on three layers, frame 0, Full | largest difference 1 of 255, 2581 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30 on three layers, frame 100, Full | largest difference 1 of 255, 1117 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30 on three layers, frame 239, Full | largest difference 1 of 255, 1312 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30 on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30 on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split 2 past both edges, split 1 0, split 2 30 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-394 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: both sides 50, Split's eye-shaped gap; draws cleanly | [], 40414 pixels changed | yes |
| 3_upper_only.png, split 1 150, split 2 0: only the upper half pushed up, the lower half untouched; draws cleanly | [], 31044 pixels changed | yes |
| 4_lower_wide.png, split 1 20, split 2 200: a little up, a lot down; draws cleanly | [], 46949 pixels changed | yes |
| 5_diagonal.png, corner to corner, split 2 60: only the lower-right side opens; draws cleanly | [], 36565 pixels changed | yes |

## Result

139 of 139 checks pass.
