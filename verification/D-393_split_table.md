# D-393: Split

B-272, after CycoreFX's CC Split: the layer torn open along the line from Point A to Point B, the gap Split pixels wide at the middle and closing to nothing at the two points, each side squeezed outward so nothing is lost past the tear's reach. The formulas are this program's own. Every expected pixel is `Fixtures/split/expected_split.json`, written by `tools/split_reference.py` before this code existed and printed in document 25 as FX-SPLIT-001 to 017. Tolerance 2e-5.

## FX-SPLIT-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPLIT-001 frame 0: The settings as they start: from 25, 50 to 75, 50, split 50, wider than the drawing: the middle torn wide open, transparent from top to bottom between the points, the ends kept. | largest difference 1.9e-7 | yes |
| FX-SPLIT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-002 frame 0: Split 4: a gap 4 pixels wide at the middle of the middle row, closed at 4, 5 and 12, 5, the band pushed up and down and squeezed into the 4 pixels each side; columns left of 4 and right of 12 untouched. | largest difference 2.5e-7 | yes |
| FX-SPLIT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-003 frame 0: Split 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SPLIT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-004 frame 0: Both points at 50, 50: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SPLIT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-005 frame 0: From 50, 10 to 50, 90, split 4: torn down the middle column, the stripes pushed left and right. | largest difference 2.5e-7 | yes |
| FX-SPLIT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-006 frame 0: From 10, 10 to 90, 90, split 3: torn along the diagonal. | largest difference 2.5e-7 | yes |
| FX-SPLIT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-007 frame 0: Split keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the drawing, frame 2 FX-SPLIT-002. | largest difference 1.9e-7 | yes |
| FX-SPLIT-007 frame 2: Split keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the drawing, frame 2 FX-SPLIT-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT-007 frame 4: Split keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the drawing, frame 2 FX-SPLIT-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-008 frame 0: Point B keyed from 25, 50 at frame 0 to 75, 50 at frame 4, split 4: frame 0 nothing torn, frame 4 FX-SPLIT-002. | largest difference 1.9e-7 | yes |
| FX-SPLIT-008 frame 2: Point B keyed from 25, 50 at frame 0 to 75, 50 at frame 4, split 4: frame 0 nothing torn, frame 4 FX-SPLIT-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT-008 frame 4: Point B keyed from 25, 50 at frame 0 to 75, 50 at frame 4, split 4: frame 0 nothing torn, frame 4 FX-SPLIT-002. | largest difference 2.5e-7 | yes |
| FX-SPLIT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-009 frame 0: FX-SPLIT-002 moved three pixels right: the same, moved; nothing grows. | largest difference 2.5e-7 | yes |
| FX-SPLIT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-010 frame 0: Split eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SPLIT-010 frame 2: Split eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SPLIT-010 frame 4: Split eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 1.9e-7 | yes |
| FX-SPLIT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-011 frame 0: FX-SPLIT-002 with the points swapped: the same picture. | largest difference 2.5e-7 | yes |
| FX-SPLIT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-012 frame 0: From -50, 50 to 150, 50, split 6: the points off the drawing, the tear running right across it, 6 pixels wide at the middle. | largest difference 2.5e-7 | yes |
| FX-SPLIT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPLIT-013 frame 0: Split 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-013 frame 4: Split 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT-014 frame 0: Split -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-014 frame 4: Split -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT-015 frame 0: Point A at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-015 frame 4: Point A at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT-016 frame 0: Point B at 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-016 frame 4: Point B at 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPLIT-017 frame 0: Split keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-017 frame 4: Split keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPLIT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_split_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_split_006.json is saved with its two points and split | {"point_a":[10,10],"point_b":[90,90],"split":3} | yes |
| fx_split_013.json is refused in a sentence | Split's split runs from 0 to 1000, and this is 1001. | yes |
| fx_split_014.json is refused in a sentence | Split's split runs from 0 to 1000, and this is -1. | yes |
| fx_split_015.json is refused in a sentence | Split's point a runs from -1000 to 1000, and this is 1001. | yes |
| fx_split_016.json is refused in a sentence | Split's point b runs from -1000 to 1000, and this is -1001. | yes |
| a file with a Split with no `split` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Split whose point a is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| split 1000.5 is refused with a sentence, and nothing changes | Split's split runs from 0 to 1000, and this is 1000.5. | yes |
| point a at -1000.5, 50 is refused with a sentence, and nothing changes | Split's point a runs from -1000 to 1000, and this is -1000.5. | yes |
| split keyed to -10 is refused with a sentence, and nothing changes | Split's split runs from 0 to 1000, and this is -10. | yes |
| from 20, 30 to 80, 70, split 35 is taken | taken | yes |
| split keyed from 0 to 200 is taken | taken | yes |
| point b keyed from 75, 50 to 75, 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_split_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_split_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_split_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_split_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_split_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_split_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_split_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_split_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 734555 pixels changed | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50) on three layers, frame 0, Full | largest difference 1 of 255, 2007 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50) on three layers, frame 100, Full | largest difference 1 of 255, 781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50) on three layers, frame 239, Full | largest difference 1 of 255, 808 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50) on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50) on three layers, frame 100, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split as it starts (25, 50 to 75, 50, split 50) on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split down a diagonal, split 200: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1835242 pixels changed | yes |
| the reference shot, Split down a diagonal, split 200 on three layers, frame 0, Full | largest difference 1 of 255, 1149 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split down a diagonal, split 200 on three layers, frame 100, Full | largest difference 1 of 255, 779 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split down a diagonal, split 200 on three layers, frame 239, Full | largest difference 1 of 255, 411 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split down a diagonal, split 200 on three layers, frame 0, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split down a diagonal, split 200 on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split down a diagonal, split 200 on three layers, frame 239, Draft | largest difference 1 of 255, 18 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split past both edges, split 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1732228 pixels changed | yes |
| the reference shot, Split past both edges, split 20 on three layers, frame 0, Full | largest difference 1 of 255, 1691 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split past both edges, split 20 on three layers, frame 100, Full | largest difference 1 of 255, 814 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split past both edges, split 20 on three layers, frame 239, Full | largest difference 1 of 255, 741 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split past both edges, split 20 on three layers, frame 0, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split past both edges, split 20 on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Split past both edges, split 20 on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-393 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: an eye-shaped gap across the middle, widest in the centre; draws cleanly | [], 40414 pixels changed | yes |
| 3_wide.png, split 150: the gap opened wide, the two halves squeezed up and down; draws cleanly | [], 53027 pixels changed | yes |
| 4_diagonal.png, corner to corner, split 40: a slanted tear; draws cleanly | [], 84493 pixels changed | yes |

## Result

124 of 124 checks pass.
