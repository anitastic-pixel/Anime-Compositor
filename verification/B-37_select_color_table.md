# B-37: select colour

D-93, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/select_color/expected_select_color.json`, written by `tools/select_color_reference.py` before this code existed and printed in document 25 as FX-SELECT-001 to 016. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SELECT-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SELECT-001 frame 0: The line chosen, keep chosen: only the box's line and its half-covering edge are left, the edge still half covering; the skin and the trace line become transparent. | largest difference 3.0e-8 | yes |
| FX-SELECT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-002 frame 0: The line chosen, keep others: the line and its edge become transparent, and the skin and the trace line are left. | largest difference 1.9e-7 | yes |
| FX-SELECT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-003 frame 0: No colour chosen, keep chosen: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SELECT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-004 frame 0: No colour chosen, keep others: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SELECT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-005 frame 0: The line and the trace line chosen, keep chosen: both lines are left, and only the skin goes. | largest difference 7.1e-8 | yes |
| FX-SELECT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-006 frame 0: FX-SELECT-001 with the colour written in capitals: the same. | largest difference 3.0e-8 | yes |
| FX-SELECT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-007 frame 0: #28242e chosen, 10 above the line on every channel, keep chosen, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1 choose nothing and so keep nothing, and the frame is empty; frame 2, at exactly 10, is FX-SELECT-001, and so is frame 4. | largest difference 0.0e0 | yes |
| FX-SELECT-007 frame 1: #28242e chosen, 10 above the line on every channel, keep chosen, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1 choose nothing and so keep nothing, and the frame is empty; frame 2, at exactly 10, is FX-SELECT-001, and so is frame 4. | largest difference 0.0e0 | yes |
| FX-SELECT-007 frame 2: #28242e chosen, 10 above the line on every channel, keep chosen, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1 choose nothing and so keep nothing, and the frame is empty; frame 2, at exactly 10, is FX-SELECT-001, and so is frame 4. | largest difference 3.0e-8 | yes |
| FX-SELECT-007 frame 4: #28242e chosen, 10 above the line on every channel, keep chosen, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1 choose nothing and so keep nothing, and the frame is empty; frame 2, at exactly 10, is FX-SELECT-001, and so is frame 4. | largest difference 3.0e-8 | yes |
| FX-SELECT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-008 frame 0: FX-SELECT-001 moved three pixels right: the same, moved. | largest difference 3.0e-8 | yes |
| FX-SELECT-008 frame 3: FX-SELECT-001 moved three pixels right: the same, moved. | largest difference 3.0e-8 | yes |
| FX-SELECT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-009 frame 0: Tolerance 255, keep others: every pixel that shows is chosen, so the frame is empty. | largest difference 0.0e0 | yes |
| FX-SELECT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELECT-010 frame 0: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-010 frame 4: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELECT-011 frame 0: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-011 frame 4: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELECT-012 frame 0: Tolerance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-012 frame 4: Tolerance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELECT-013 frame 0: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-013 frame 4: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELECT-014 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-014 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELECT-015 frame 0: Keep "both", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-015 frame 4: Keep "both", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELECT-016 frame 0: Keep "Chosen", in a capital, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-016 frame 4: Keep "Chosen", in a capital, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELECT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is kept or cleared where it is | 0 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_select_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_select_006.json, its colour written in capitals, is saved in small letters | ["#1e1a24"] | yes |
| fx_select_016.json's keep "Chosen" is saved as written, not corrected | "Chosen" | yes |
| a file with no `keep` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a keep that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tolerance 256 is refused with a sentence, and nothing changes | Select Colour's tolerance runs from 0 to 255, and this is 256. | yes |
| tolerance -1 is refused with a sentence, and nothing changes | Select Colour's tolerance runs from 0 to 255, and this is -1. | yes |
| nine colours is refused with a sentence, and nothing changes | Select Colour takes up to eight colours, and this has 9. | yes |
| the colour "#12345" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12345". | yes |
| keep "both" is refused with a sentence, and nothing changes | Select Colour keeps "chosen" or "others", and this is "both". | yes |
| keep "Others" is refused with a sentence, and nothing changes | Select Colour keeps "chosen" or "others", and this is "Others". | yes |
| tolerance keyed to 300 is refused with a sentence, and nothing changes | Select Colour's tolerance runs from 0 to 255, and this is 300. | yes |
| tolerance 255 with eight colours, keep others, the top of the ranges, is taken | taken | yes |
| tolerance 0 with no colour, the bottom, is taken | taken | yes |
| tolerance keyed from 0 to 20 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_select_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_select_008.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

70 of 70 checks pass.
