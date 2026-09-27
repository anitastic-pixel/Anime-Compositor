# B-84: leave colour

D-141, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the eighth of the third batch. Every expected pixel is `Fixtures/leave_color/expected_leave_color.json`, written by `tools/leave_color_reference.py` before this code existed and printed in document 25 as FX-LEAVE-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LEAVE-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LEAVE-001 frame 0: The settings as they start: #ff0000, tolerance 15, softness 10, amount 100. The red, the pink across the wheel's seam (15 degrees the other way), the skin's shadow and the skin, all within 27 degrees of red, are kept exactly; the orange, 36 degrees off, half way across the soft band, is half drained; the yellow, green, teal, blue, violet, magenta and the line turn fully grey, each to its own brightness. The red at half covering is kept; the blue at half covering turns the same grey as the blue, at its own covering; the empty pixels stay empty. | largest difference 1.9e-7 | yes |
| FX-LEAVE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LEAVE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-003 frame 0: Amount 50: every drained colour goes half way, on the encoded scale, to its grey, and the orange a quarter of the way; the kept colours stay. | largest difference 1.9e-7 | yes |
| FX-LEAVE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-004 frame 0: Tolerance 5, softness 0, a hard edge 9 degrees round red: only the red and the soft red are kept; the pink and the skin's shadow, 15 degrees off, turn fully grey, as does everything else. | largest difference 9.9e-8 | yes |
| FX-LEAVE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-005 frame 0: Tolerance 40, softness 0, 72 degrees each way: the warm half of the wheel is kept, red, pink, shadow, skin, orange, yellow and magenta (60 degrees off); green, teal, blue, violet (82.5 off) and the line (96 off) turn fully grey. | largest difference 1.9e-7 | yes |
| FX-LEAVE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-006 frame 0: Softness 0 at the start's tolerance 15: the skin, 25.7 degrees off, is still kept, but the orange, 36 off, now turns fully grey; the rest is FX-LEAVE-001. | largest difference 1.9e-7 | yes |
| FX-LEAVE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-007 frame 0: Softness 30, a wide band from 27 to 81 degrees: the orange keeps five sixths of its colour, the yellow and the magenta, both 60 degrees off, the same part, 7/18; the violet, 82.5 off, and everything further turn fully grey. | largest difference 1.9e-7 | yes |
| FX-LEAVE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-008 frame 0: Colour #4060ff, the blue: the blue and the soft blue are kept; the line, 34 degrees off, keeps about three fifths of its colour; the violet, 48 off, the teal, 50 off, and all the warm colours turn fully grey. | largest difference 9.9e-8 | yes |
| FX-LEAVE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-009 frame 0: Colour #808080, a grey, has no hue: nothing is kept, and every pixel that shows turns fully grey, each to its own brightness. | largest difference 9.9e-8 | yes |
| FX-LEAVE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-010 frame 0: FX-LEAVE-008 with its colour written in capitals, #4060FF: the same. | largest difference 9.9e-8 | yes |
| FX-LEAVE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-011 frame 0: Colour #ff9900, the orange, keeping the skin tones: the orange, the skin, the shadow and the yellow, all within 27 degrees, are kept; the red, 36 off, is half drained; the pink and the rest turn fully grey. | largest difference 1.9e-7 | yes |
| FX-LEAVE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-012 frame 0: Tolerance 100: every hue is at most 180 degrees off, so every colour is kept and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-LEAVE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-013 frame 0: Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 keeps the red whole, the pink and the shadow a sixth, and drains the rest; frame 2, at 50, keeps the violet whole and the line two thirds, the green, teal and blue turned grey; frame 4 is FX-LEAVE-012, the drawing untouched. | largest difference 1.2e-7 | yes |
| FX-LEAVE-013 frame 2: Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 keeps the red whole, the pink and the shadow a sixth, and drains the rest; frame 2, at 50, keeps the violet whole and the line two thirds, the green, teal and blue turned grey; frame 4 is FX-LEAVE-012, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-LEAVE-013 frame 4: Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 keeps the red whole, the pink and the shadow a sixth, and drains the rest; frame 2, at 50, keeps the violet whole and the line two thirds, the green, teal and blue turned grey; frame 4 is FX-LEAVE-012, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-LEAVE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-014 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-LEAVE-003, frame 4 is FX-LEAVE-001. | largest difference 1.9e-7 | yes |
| FX-LEAVE-014 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-LEAVE-003, frame 4 is FX-LEAVE-001. | largest difference 1.9e-7 | yes |
| FX-LEAVE-014 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-LEAVE-003, frame 4 is FX-LEAVE-001. | largest difference 1.9e-7 | yes |
| FX-LEAVE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-015 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 is held at 100 and is FX-LEAVE-001, as frame 4 is. | largest difference 1.9e-7 | yes |
| FX-LEAVE-015 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 is held at 100 and is FX-LEAVE-001, as frame 4 is. | largest difference 1.9e-7 | yes |
| FX-LEAVE-015 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 is held at 100 and is FX-LEAVE-001, as frame 4 is. | largest difference 1.9e-7 | yes |
| FX-LEAVE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-016 frame 0: FX-LEAVE-001 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-LEAVE-016 frame 3: FX-LEAVE-001 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-LEAVE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEAVE-017 frame 0: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-017 frame 4: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEAVE-018 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-018 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEAVE-019 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-019 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEAVE-020 frame 0: Tolerance keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-020 frame 4: Tolerance keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEAVE-021 frame 0: A colour written "#ff00", two digits short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-021 frame 4: A colour written "#ff00", two digits short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEAVE-022 frame 0: A colour written "red", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-022 frame 4: A colour written "red", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEAVE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | LeaveColor { color: "#ff0000", tolerance: 15.0, softness: 10.0, amount: 100.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_leave_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_leave_010.json's colour, written #4060FF, is saved in small letters | "#4060ff" | yes |
| a file with no `color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an amount that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tolerance 101 is refused with a sentence, and nothing changes | Leave Color's tolerance runs from 0 to 100, and this is 101. | yes |
| softness -1 is refused with a sentence, and nothing changes | Leave Color's softness runs from 0 to 100, and this is -1. | yes |
| amount 101 is refused with a sentence, and nothing changes | Leave Color's amount runs from 0 to 100, and this is 101. | yes |
| the colour "red" is refused with a sentence, and nothing changes | Leave Color's colour is written #rrggbb, and this is "red". | yes |
| the colour "#ff00" is refused with a sentence, and nothing changes | Leave Color's colour is written #rrggbb, and this is "#ff00". | yes |
| tolerance keyed to 150 is refused with a sentence, and nothing changes | Leave Color's tolerance runs from 0 to 100, and this is 150. | yes |
| every number at 0, the bottom, is taken | taken | yes |
| every number at 100, the top, is taken | taken | yes |
| amount keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_leave_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_leave_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_leave_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

86 of 86 checks pass.
