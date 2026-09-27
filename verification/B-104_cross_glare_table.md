# B-104: cross glare

D-161, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-eighth of the third batch. Every expected pixel is `Fixtures/cross_glare/expected_cross_glare.json`, written by `tools/cross_glare_reference.py` before this code existed and printed in document 25 as FX-GLARE-001 to 029. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-GLARE-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLARE-001 frame 0: The settings as they start: threshold 80, length 40, points 4, angle 45, intensity 1, white. The yellow, 98 %, and the soft yellow are bright enough; the brown, 60 %, and the purple give no light. Each bright point is streaked along both diagonals, an X, faint as the light is shared over forty steps, running on past the frame; the soft point's streaks are half as strong. The diagonals run between pixel centres, so each streak is spread a little onto the pixels beside its line, and the yellow even takes a little of its own light; pixels well off the diagonals stay as they were. | largest difference 1.8e-7 | yes |
| FX-GLARE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-002 frame 0: Intensity 0: the drawing, untouched, and nothing grows. | largest difference 1.5e-7 | yes |
| FX-GLARE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-003 frame 0: Length 0.9, floored to 0: no streaks, the drawing untouched. | largest difference 1.5e-7 | yes |
| FX-GLARE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-004 frame 0: Threshold 100: nothing is that bright, so the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-GLARE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-005 frame 0: Length 4: the X four pixels long each way, strongest next to the point: the yellow's up-right streak crosses the purple bar at row 1, and pixels further than four steps along a diagonal stay empty. | largest difference 1.6e-7 | yes |
| FX-GLARE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-006 frame 0: FX-GLARE-005 at threshold 60: the brown, at exactly 60 %, glints too. | largest difference 1.6e-7 | yes |
| FX-GLARE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-007 frame 0: Points 1: one streak from each point, up and to the right, and none the other ways. | largest difference 1.5e-7 | yes |
| FX-GLARE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-008 frame 0: Points 2: a single line through each point, up-right and down-left. | largest difference 1.8e-7 | yes |
| FX-GLARE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-009 frame 0: Angle 0: a plus, the streaks straight up, right, down and left, each landing on whole pixels: the pixel next to the yellow takes 0.8 squared over 1.2 of its light, and the purple three to its right takes 0.4 squared over 1.2 of it. The diagonal neighbours stay empty. | largest difference 1.5e-7 | yes |
| FX-GLARE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-010 frame 0: Angle 90: a quarter turn gives the same four streaks, so it is FX-GLARE-009. | largest difference 1.5e-7 | yes |
| FX-GLARE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-011 frame 0: Angle 405, a whole turn past 45: FX-GLARE-005 exactly. | largest difference 1.6e-7 | yes |
| FX-GLARE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-012 frame 0: Points 6: a six-pointed star, streaks every 60 degrees from 45. | largest difference 1.8e-7 | yes |
| FX-GLARE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-013 frame 0: Points 4.9, floored to 4: FX-GLARE-005 exactly. | largest difference 1.6e-7 | yes |
| FX-GLARE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-014 frame 0: Intensity 2.5: two and a half times FX-GLARE-005's streaks, not cut off at white; the covering stops at full, so the purple and yellow pixels the streaks cross keep a covering of 1. | largest difference 2.6e-7 | yes |
| FX-GLARE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-015 frame 0: Colour #ff8000, orange: FX-GLARE-005's streaks tinted, their red as before, their green a fifth and their blue gone; the covering as FX-GLARE-005's. | largest difference 1.8e-7 | yes |
| FX-GLARE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-016 frame 0: FX-GLARE-015 with its colour written in capitals, #FF8000: the same. | largest difference 1.8e-7 | yes |
| FX-GLARE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-017 frame 0: Colour #000000, black: the streaks add covering but no colour, a dark cross; the solid pixels they cross keep their colour and the empty ones turn black. | largest difference 1.5e-7 | yes |
| FX-GLARE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-018 frame 0: Length keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLARE-005, frame 4 is length 8. | largest difference 1.5e-7 | yes |
| FX-GLARE-018 frame 2: Length keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLARE-005, frame 4 is length 8. | largest difference 1.6e-7 | yes |
| FX-GLARE-018 frame 4: Length keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLARE-005, frame 4 is length 8. | largest difference 1.9e-7 | yes |
| FX-GLARE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-019 frame 0: Length 4, points keyed from 1 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLARE-007, frame 2, at 4.5, is floored to 4 and is FX-GLARE-005, frame 4 is an eight-pointed star. | largest difference 1.5e-7 | yes |
| FX-GLARE-019 frame 2: Length 4, points keyed from 1 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLARE-007, frame 2, at 4.5, is floored to 4 and is FX-GLARE-005, frame 4 is an eight-pointed star. | largest difference 1.6e-7 | yes |
| FX-GLARE-019 frame 4: Length 4, points keyed from 1 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLARE-007, frame 2, at 4.5, is floored to 4 and is FX-GLARE-005, frame 4 is an eight-pointed star. | largest difference 1.6e-7 | yes |
| FX-GLARE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-020 frame 0: Length 4, intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-GLARE-020 frame 2: Length 4, intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 6.6e-7 | yes |
| FX-GLARE-020 frame 4: Length 4, intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 6.6e-7 | yes |
| FX-GLARE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-021 frame 0: FX-GLARE-009 moved three pixels right: the layer grew by 4, and the yellow's left streak, which runs two pixels past the drawing's left edge, shows in the two grown columns left of the drawing; column 0, past the streak's end, stays empty. | largest difference 1.5e-7 | yes |
| FX-GLARE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLARE-022 frame 0: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-022 frame 4: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-023 frame 0: Length 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-023 frame 4: Length 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-024 frame 0: Points 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-024 frame 4: Points 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-025 frame 0: Points 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-025 frame 4: Points 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-026 frame 0: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-026 frame 4: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-027 frame 0: Intensity keyed to 11 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-027 frame 4: Intensity keyed to 11 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-028 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-028 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLARE-029 frame 0: A colour written "white", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-029 frame 4: A colour written "white", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLARE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| length 40.7 grows the drawing's bounds by 40 pixels | 40 | yes |
| length 0.9, taken down to 0, grows them by nothing | 0 | yes |
| intensity 0 grows them by nothing | 0 | yes |
| a half-size draft preview halves the length, 40 to 20, and nothing else | CrossGlare { threshold: 80.0, length: 20.0, points: 4.0, angle: 45.0, intensity: 1.0, color: "#ffffff" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glare_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `points` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a length that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| threshold 101 is refused with a sentence, and nothing changes | Cross Glare's threshold runs from 0 to 100, and this is 101. | yes |
| length 1001 is refused with a sentence, and nothing changes | Cross Glare's length runs from 0 to 1000, and this is 1001. | yes |
| points 0 is refused with a sentence, and nothing changes | Cross Glare's points runs from 1 to 8, and this is 0. | yes |
| points 9 is refused with a sentence, and nothing changes | Cross Glare's points runs from 1 to 8, and this is 9. | yes |
| angle -3601 is refused with a sentence, and nothing changes | Cross Glare's angle runs from -3600 to 3600, and this is -3601. | yes |
| intensity 11 is refused with a sentence, and nothing changes | Cross Glare's intensity runs from 0 to 10, and this is 11. | yes |
| colour "#12345" is refused with a sentence, and nothing changes | Cross Glare's colour is written #rrggbb, and this is "#12345". | yes |
| colour "white" is refused with a sentence, and nothing changes | Cross Glare's colour is written #rrggbb, and this is "white". | yes |
| intensity keyed to 11 is refused with a sentence, and nothing changes | Cross Glare's intensity runs from 0 to 10, and this is 11. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| length keyed from 0 to 8 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glare_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glare_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glare_018.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## Result

108 of 108 checks pass.
