# B-35: line recolour

D-91, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/recolor/expected_recolor.json`, written by `tools/recolor_reference.py` before this code existed and printed in document 25 as FX-RECOLOR-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RECOLOR-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RECOLOR-001 frame 0: The line chosen, new colour #ff0000: the box's line and its half-covering edge turn red, the edge still half covering; the skin and the trace line are untouched. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-002 frame 0: No colour chosen: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-003 frame 0: The line chosen one step off in blue, tolerance 0: nothing is chosen, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-004 frame 0: The same at tolerance 1: the line is chosen again, and this is FX-RECOLOR-001. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-005 frame 0: The line and the trace line chosen, new colour #3060ff: both turn blue. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-006 frame 0: FX-RECOLOR-001 with both colours written in capitals: the same. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-007 frame 0: #28242e chosen, 10 above the line on every channel, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1, at 0 and 5, choose nothing; frame 2, at exactly 10, is FX-RECOLOR-001, and so is frame 4. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-007 frame 1: #28242e chosen, 10 above the line on every channel, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1, at 0 and 5, choose nothing; frame 2, at exactly 10, is FX-RECOLOR-001, and so is frame 4. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-007 frame 2: #28242e chosen, 10 above the line on every channel, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1, at 0 and 5, choose nothing; frame 2, at exactly 10, is FX-RECOLOR-001, and so is frame 4. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-007 frame 4: #28242e chosen, 10 above the line on every channel, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1, at 0 and 5, choose nothing; frame 2, at exactly 10, is FX-RECOLOR-001, and so is frame 4. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-008 frame 0: FX-RECOLOR-001 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-008 frame 3: FX-RECOLOR-001 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-009 frame 0: The new colour is the line's own: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-010 frame 0: The new colour is the skin's: the line turns to skin and vanishes into it, and its edge is skin at half covering. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-011 frame 0: Tolerance 255: every pixel that shows is chosen and turns red; the empty ones stay empty. | largest difference 3.0e-8 | yes |
| FX-RECOLOR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RECOLOR-012 frame 0: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-012 frame 4: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RECOLOR-013 frame 0: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-013 frame 4: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RECOLOR-014 frame 0: Tolerance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-014 frame 4: Tolerance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RECOLOR-015 frame 0: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-015 frame 4: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RECOLOR-016 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-016 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RECOLOR-017 frame 0: A new colour written "red". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-017 frame 4: A new colour written "red". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RECOLOR-018 frame 0: A new colour left empty, which this effect does not allow. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-018 frame 4: A new colour left empty, which this effect does not allow. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RECOLOR-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is recoloured where it is | 0 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_recolor_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_recolor_006.json, written in capitals, is saved in small letters | ["#1e1a24"] and "#ff0000" | yes |
| a file with no `new_color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a new colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a tolerance that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tolerance 256 is refused with a sentence, and nothing changes | Line Recolour's tolerance runs from 0 to 255, and this is 256. | yes |
| tolerance -1 is refused with a sentence, and nothing changes | Line Recolour's tolerance runs from 0 to 255, and this is -1. | yes |
| nine colours is refused with a sentence, and nothing changes | Line Recolour takes up to eight colours, and this has 9. | yes |
| the colour "#12345" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12345". | yes |
| the new colour "red" is refused with a sentence, and nothing changes | Line Recolour's new colour is written # and six hexadecimal digits, such as #ff4000, and this is "red". | yes |
| an empty new colour is refused with a sentence, and nothing changes | Line Recolour's new colour is written # and six hexadecimal digits, such as #ff4000, and this is "". | yes |
| tolerance keyed to 300 is refused with a sentence, and nothing changes | Line Recolour's tolerance runs from 0 to 255, and this is 300. | yes |
| tolerance 255 with eight colours, the top of the ranges, is taken | taken | yes |
| tolerance 0 with no colour, the bottom, is taken | taken | yes |
| the new colour in capitals "#FF4000" is taken | taken | yes |
| tolerance keyed from 0 to 20 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_recolor_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_recolor_008.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

75 of 75 checks pass.
