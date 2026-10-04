# B-90: sharpen

D-147, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the fourteenth of the third batch. Every expected pixel is `Fixtures/sharpen/expected_sharpen.json`, written by `tools/sharpen_reference.py` before this code existed and printed in document 25 as FX-SHARPEN-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SHARPEN-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHARPEN-001 frame 0: Amount 100 and radius 1, the settings as they start: each colour is pushed away from the blurred colour around it, so the two sides of every edge inside the box move apart. The line, dark beside the skin, is pushed to black all round the box and down the soft column 7; the skin beside the line grows lighter, its red held at 1; the skin beside the band loses blue, and the band loses red and green and gains blue. Every pixel of the box and of the soft line changes; the patch of skin on the left, with its soft edge, has no other colour within reach of the blur and is left as it is but for rounding, as its outline against the empty pixels is not an edge. Every covering is kept, and the empty pixels stay empty. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-003 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-004 frame 0: Amount 50: each colour pushed half as far from its blur as in FX-SHARPEN-001, wherever neither is held at 0 or 1. | largest difference 2.5e-7 | yes |
| FX-SHARPEN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-005 frame 0: Amount 500: each colour pushed five times as far as in FX-SHARPEN-001, held at 0 and 1, so the halos burn: the skin along the line turns white, the skin beside the band a pale yellow, and the whole band pure blue #0000ff; every pixel moves at least as far as in FX-SHARPEN-001, the same way. | largest difference 8.4e-7 | yes |
| FX-SHARPEN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-006 frame 0: Radius 0.5: a blur about one pixel wide, so each colour is compared only with its nearest neighbours; the left patch is still left as it is but for rounding. | largest difference 3.7e-7 | yes |
| FX-SHARPEN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-007 frame 0: Radius 3: a blur wide enough to reach from the left patch into the box, so the skin of the patch, its blur now darkened by the line, grows lighter too, every pixel of it, most at its soft edge; the covering is kept. | largest difference 3.5e-7 | yes |
| FX-SHARPEN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-008 frame 0: Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 untouched, frame 2 amount 100, FX-SHARPEN-001, and frame 4 amount 200. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-008 frame 2: Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 untouched, frame 2 amount 100, FX-SHARPEN-001, and frame 4 amount 200. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-008 frame 4: Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 untouched, frame 2 amount 100, FX-SHARPEN-001, and frame 4 amount 200. | largest difference 4.3e-7 | yes |
| FX-SHARPEN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-009 frame 0: Radius keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 untouched, frame 2 radius 1, FX-SHARPEN-001, and frame 4 radius 2, whose blur reaches the left patch. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-009 frame 2: Radius keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 untouched, frame 2 radius 1, FX-SHARPEN-001, and frame 4 radius 2, whose blur reaches the left patch. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-009 frame 4: Radius keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 untouched, frame 2 radius 1, FX-SHARPEN-001, and frame 4 radius 2, whose blur reaches the left patch. | largest difference 3.7e-7 | yes |
| FX-SHARPEN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-010 frame 0: Amount eased from 0 at frame 0 to 500 at frame 4 on a curve that overshoots: at frame 2 it has gone past 500 and is held there, so frames 2 and 4 are both FX-SHARPEN-005. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-010 frame 2: Amount eased from 0 at frame 0 to 500 at frame 4 on a curve that overshoots: at frame 2 it has gone past 500 and is held there, so frames 2 and 4 are both FX-SHARPEN-005. | largest difference 8.4e-7 | yes |
| FX-SHARPEN-010 frame 4: Amount eased from 0 at frame 0 to 500 at frame 4 on a curve that overshoots: at frame 2 it has gone past 500 and is held there, so frames 2 and 4 are both FX-SHARPEN-005. | largest difference 8.4e-7 | yes |
| FX-SHARPEN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-011 frame 0: FX-SHARPEN-001 moved three pixels right: the same, moved. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-011 frame 3: FX-SHARPEN-001 moved three pixels right: the same, moved. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-012 frame 0: Amount 250 and radius 1.5, a strong crisp: every pixel of the box pushed at least as far as in FX-SHARPEN-001, and the wider blur just reaches the patch's right side, faintly: its soft edge in column 3 grows up to about two levels lighter and column 2 under a quarter of a level, while columns 0 and 1 are left as they are but for rounding. | largest difference 6.6e-7 | yes |
| FX-SHARPEN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-013 frame 0: Amount 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-013 frame 4: Amount 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-014 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-014 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-015 frame 0: Radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-015 frame 4: Radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-016 frame 0: Radius -0.5, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-016 frame 4: Radius -0.5, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-017 frame 0: Amount keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-017 frame 4: Amount keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-018 frame 0: Radius keyed to -2 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-018 frame 4: Radius keyed to -2 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the radius, 3 to 1.5, and nothing else | Sharpen { amount: 100.0, radius: 1.5, threshold: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sharpen_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `radius` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an amount that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 501 is refused with a sentence, and nothing changes | Sharpen's amount runs from 0 to 500, and this is 501. | yes |
| amount -1 is refused with a sentence, and nothing changes | Sharpen's amount runs from 0 to 500, and this is -1. | yes |
| radius 101 is refused with a sentence, and nothing changes | Sharpen's radius runs from 0 to 100, and this is 101. | yes |
| radius -0.5 is refused with a sentence, and nothing changes | Sharpen's radius runs from 0 to 100, and this is -0.5. | yes |
| amount keyed to 600 is refused with a sentence, and nothing changes | Sharpen's amount runs from 0 to 500, and this is 600. | yes |
| amount 0 and radius 0, the bottoms, is taken | taken | yes |
| amount 500 and radius 100, the tops, is taken | taken | yes |
| radius keyed from 0 to 2 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sharpen_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sharpen_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sharpen_011.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

75 of 75 checks pass.
