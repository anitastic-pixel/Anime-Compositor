# B-119: line blur

D-183, accepted by the owner on 2026-09-28. Every expected pixel is `Fixtures/line_blur/expected_line_blur.json`, written by `tools/line_blur_reference.py` before this code existed and printed in document 25 as FX-LBLUR-001 to 016. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LBLUR-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LBLUR-001 frame 0: The bar, length 3: a clean straight line has nothing to smooth, and every pixel stays as it was, to within rounding. | largest difference 2.2e-9 | yes |
| FX-LBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-002 frame 0: The stepped diagonal, length 4: the steps soften into a slope. The empty pixel in the crook of each step takes some ink, the step pixels give some up, and nothing more than one pixel from the line changes much. | largest difference 2.4e-8 | yes |
| FX-LBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-003 frame 0: The face, length 3: the box's straight sides, the flat skin and the red line's middle stay as they are; the red line's ends taper into the skin, and the box's inside corners round off, the skin in each taking some of the line. | largest difference 2.0e-7 | yes |
| FX-LBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-004 frame 0: FX-LBLUR-003 at strength 50: every pixel halfway between the drawing and FX-LBLUR-003. | largest difference 2.1e-7 | yes |
| FX-LBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-005 frame 0: FX-LBLUR-003 with lines only: each pixel changes by FX-LBLUR-003's change times its own ink, so the empty outside stays empty and the skin moves less than the lines. | largest difference 2.2e-7 | yes |
| FX-LBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-006 frame 0: Length 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-007 frame 0: The stepped diagonal with the length keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 softens it less, and frame 4 is FX-LBLUR-002. | largest difference 2.2e-9 | yes |
| FX-LBLUR-007 frame 2: The stepped diagonal with the length keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 softens it less, and frame 4 is FX-LBLUR-002. | largest difference 2.9e-8 | yes |
| FX-LBLUR-007 frame 4: The stepped diagonal with the length keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 softens it less, and frame 4 is FX-LBLUR-002. | largest difference 2.4e-8 | yes |
| FX-LBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-008 frame 0: FX-LBLUR-002 moved three pixels right: the same, moved. | largest difference 2.4e-8 | yes |
| FX-LBLUR-008 frame 3: FX-LBLUR-002 moved three pixels right: the same, moved. | largest difference 2.4e-8 | yes |
| FX-LBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-009 frame 0: Strength 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-010 frame 0: The bar, length 50, the most: each side of the blur stops where the line ends, so the bar is still as it was, to within rounding. | largest difference 2.2e-9 | yes |
| FX-LBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LBLUR-011 frame 0: Length 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-011 frame 4: Length 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LBLUR-012 frame 0: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-012 frame 4: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LBLUR-013 frame 0: Length keyed to 60 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-013 frame 4: Length keyed to 60 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LBLUR-014 frame 0: Strength 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-014 frame 4: Strength 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LBLUR-015 frame 0: Strength -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-015 frame 4: Strength -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LBLUR-016 frame 0: Lines only written "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-016 frame 4: Lines only written "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LBLUR-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| length 0 at strength 100 grows the drawing's bounds by 0 | 0 | yes |
| length 0.5 at strength 100 grows the drawing's bounds by 3 | 3 | yes |
| length 4 at strength 0 grows the drawing's bounds by 3 | 3 | yes |
| length 50 at strength 100 grows the drawing's bounds by 3 | 3 | yes |
| a half-size draft preview halves the length and keeps the strength and lines only | LineBlur { length: 4.0, strength: 60.0, lines_only: "on" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lblur_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lblur_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lblur_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lblur_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lblur_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lblur_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `lines_only` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a length that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| length 51 is refused with a sentence, and nothing changes | Line Blur's length runs from 0 to 50, and this is 51. | yes |
| length -1 is refused with a sentence, and nothing changes | Line Blur's length runs from 0 to 50, and this is -1. | yes |
| strength 101 is refused with a sentence, and nothing changes | Line Blur's strength runs from 0 to 100, and this is 101. | yes |
| lines only "yes" is refused with a sentence, and nothing changes | Line Blur's lines only is "off" or "on", and this is "yes". | yes |
| length keyed to 60 is refused with a sentence, and nothing changes | Line Blur's length runs from 0 to 50, and this is 60. | yes |
| length 50 at strength 100, the top of the ranges, is taken | taken | yes |
| length 0 at strength 0, the bottom, is taken | taken | yes |
| strength keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a jagged ink drawing, in `verification/B-119 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| length_4.png, the settings it is added with, draws cleanly, changes the strokes and leaves the empty paper white | [], 1653 of 24000 pixels changed | yes |
| length_12.png, a longer pass, draws cleanly, changes the strokes and leaves the empty paper white | [], 2595 of 24000 pixels changed | yes |
| lines_only.png, lines only on, draws cleanly, changes the strokes and leaves the empty paper white | [], 765 of 24000 pixels changed | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lblur_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lblur_008.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

69 of 69 checks pass.
