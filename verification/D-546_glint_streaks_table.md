# B-426: Glint Streaks (pick #15)

D-546 (EFFECTS.md row 262): Cross Glare gains Glint Streaks' controls. Ray Colours: one colour, as before; Alternate, every other ray in the Second Colour; or Gradient, the rays going from the Colour to the Second Colour round the star. Shimmer: each ray dims and comes back over Shimmer Period frames, each on its own beat, as the rays of a real glint flicker; Shimmer 0 holds them still. Cross Glare draws as before with one colour and no shimmer, so D-161's 29 fixtures are run again here unchanged. Every expected pixel is `Fixtures/glint_streaks/expected_glint_streaks.json`, written by `tools/glint_streaks_reference.py` before this code existed (FX-GLINT-001 to 020), compared sample by sample against the catalogue's tolerance of 2e-5. The rays are drawn on the card too: the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).

## D-161's FX-GLARE-001 to 029, unchanged: Cross Glare draws as before

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
| fx_glare_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glare_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## FX-GLINT-001 to 020

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLINT-001 frame 0: The four new settings written at their start values (one colour, #80c0ff, shimmer 0, period 24) on Cross Glare's FX-GLARE-005 (angle 45, white): FX-GLARE-005's numbers, so a glare drawn before Glint Streaks draws the same. | largest difference 1.6e-7 | yes |
| FX-GLINT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-002 frame 0: Alternate, orange #ff8000 and blue #0080ff, a plus (angle 0): rays 0 and 2, up and down, are orange; rays 1 and 3, right and left, are blue. The pixel right of the yellow point takes 0.8 squared over 1.2 of its light in blue. | largest difference 1.5e-7 | yes |
| FX-GLINT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-003 frame 0: Gradient from red #ff0000 to blue #0000ff: up is red, right two thirds red and one third blue, down one third red and two thirds blue, left blue, mixed in linear light. | largest difference 1.5e-7 | yes |
| FX-GLINT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-004 frame 0: Alternate with one ray: the ray is ray 0, so it is orange; Cross Glare with points 1, orange. | largest difference 1.5e-7 | yes |
| FX-GLINT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-005 frame 0: Gradient with one ray: the ray is the first colour; FX-GLINT-004 exactly. | largest difference 1.5e-7 | yes |
| FX-GLINT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-006 frame 0: Alternate on eight rays from angle 0: up, right, down and left orange, the four diagonals blue. | largest difference 1.8e-7 | yes |
| FX-GLINT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-007 frame 0: Alternate with the second colour the same as the first: one colour, Cross Glare's own glare. | largest difference 1.5e-7 | yes |
| FX-GLINT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-008 frame 0: FX-GLINT-002 with the second colour written in capitals, #0080FF: the same. | largest difference 1.5e-7 | yes |
| FX-GLINT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-009 frame 0: Shimmer 100, period 8, one colour: on frame 0 ray 0 (up) is at full and the others are dimmed by their own phases; on frame 4, half a period on, ray 0 is gone; frame 8, a whole period on, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-GLINT-009 frame 2: Shimmer 100, period 8, one colour: on frame 0 ray 0 (up) is at full and the others are dimmed by their own phases; on frame 4, half a period on, ray 0 is gone; frame 8, a whole period on, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-GLINT-009 frame 4: Shimmer 100, period 8, one colour: on frame 0 ray 0 (up) is at full and the others are dimmed by their own phases; on frame 4, half a period on, ray 0 is gone; frame 8, a whole period on, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-GLINT-009 frame 8: Shimmer 100, period 8, one colour: on frame 0 ray 0 (up) is at full and the others are dimmed by their own phases; on frame 4, half a period on, ray 0 is gone; frame 8, a whole period on, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-GLINT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-010 frame 0: Shimmer 50, period 8, alternate: each ray breathes between half and full strength in its own colour, never gone. | largest difference 1.5e-7 | yes |
| FX-GLINT-010 frame 3: Shimmer 50, period 8, alternate: each ray breathes between half and full strength in its own colour, never gone. | largest difference 1.5e-7 | yes |
| FX-GLINT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-011 frame 0: Shimmer keyed from 0 at frame 0 to 100 at frame 4, linear, period 8: frame 0 is the plain glare, frame 4 is FX-GLINT-009's frame 4. | largest difference 1.5e-7 | yes |
| FX-GLINT-011 frame 2: Shimmer keyed from 0 at frame 0 to 100 at frame 4, linear, period 8: frame 0 is the plain glare, frame 4 is FX-GLINT-009's frame 4. | largest difference 1.5e-7 | yes |
| FX-GLINT-011 frame 4: Shimmer keyed from 0 at frame 0 to 100 at frame 4, linear, period 8: frame 0 is the plain glare, frame 4 is FX-GLINT-009's frame 4. | largest difference 1.5e-7 | yes |
| FX-GLINT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-012 frame 4: Shimmer 100, period 16: frame 4 is a quarter period on, as frame 2 is at period 8, so it is FX-GLINT-009's frame 2. | largest difference 1.5e-7 | yes |
| FX-GLINT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-013 frame 0: Shimmer 100, period 1: every whole frame is a whole period on, so the rays hold still, each dimmed by its own phase. | largest difference 1.5e-7 | yes |
| FX-GLINT-013 frame 1: Shimmer 100, period 1: every whole frame is a whole period on, so the rays hold still, each dimmed by its own phase. | largest difference 1.5e-7 | yes |
| FX-GLINT-013 frame 5: Shimmer 100, period 1: every whole frame is a whole period on, so the rays hold still, each dimmed by its own phase. | largest difference 1.5e-7 | yes |
| FX-GLINT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-014 frame 0: FX-GLINT-002 moved three pixels right: the layer grew by 4, and the yellow's blue left ray shows in the two grown columns left of the drawing. | largest difference 1.5e-7 | yes |
| FX-GLINT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLINT-015 frame 0: Shimmer 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-015 frame 4: Shimmer 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLINT-016 frame 0: Shimmer period 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-016 frame 4: Shimmer period 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLINT-017 frame 0: Shimmer period 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-017 frame 4: Shimmer period 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLINT-018 frame 0: Ray colours "rainbow", not one of "one", "alternate" or "gradient". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-018 frame 4: Ray colours "rainbow", not one of "one", "alternate" or "gradient". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLINT-019 frame 0: A second colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-019 frame 4: A second colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLINT-020 frame 0: Shimmer keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-020 frame 4: Shimmer keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLINT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glint_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glint_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a shimmer written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with ray colours written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| fx_glint_015.json is refused in a sentence naming it | Cross Glare's shimmer runs from 0 to 100, and this is 101. | yes |
| fx_glint_016.json is refused in a sentence naming it | Cross Glare's shimmer period runs from 1 to 1000, and this is 0.5. | yes |
| fx_glint_017.json is refused in a sentence naming it | Cross Glare's shimmer period runs from 1 to 1000, and this is 1001. | yes |
| fx_glint_018.json is refused in a sentence naming it | Cross Glare's ray colours is "one", "alternate" or "gradient", and this is "rainbow". | yes |
| fx_glint_019.json is refused in a sentence naming it | Cross Glare's second colour is written #rrggbb, and this is "#12345". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shimmer 101 is refused with a sentence, and nothing changes | Cross Glare's shimmer runs from 0 to 100, and this is 101. | yes |
| shimmer period 0.5 is refused with a sentence, and nothing changes | Cross Glare's shimmer period runs from 1 to 1000, and this is 0.5. | yes |
| ray colours "rainbow" is refused with a sentence, and nothing changes | Cross Glare's ray colours is "one", "alternate" or "gradient", and this is "rainbow". | yes |
| second colour "#12345" is refused with a sentence, and nothing changes | Cross Glare's second colour is written #rrggbb, and this is "#12345". | yes |
| shimmer keyed to 120 is refused with a sentence, and nothing changes | Cross Glare's shimmer runs from 0 to 100, and this is 120. | yes |
| ray colours gradient, second colour #ff0000, shimmer 100, period 1000 is taken | taken | yes |
| ray colours alternate, shimmer period 1 is taken | taken | yes |
| shimmer keyed 0 to 100 over frames 0 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |
| shimmer period keyed 24 to 48 on a file without it, saved and opened again: the keys are kept | kept | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glint_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glint_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glint_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glint_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glint_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glint_001.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_002.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_003.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_004.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_005.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_006.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_007.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_008.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_009.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_010.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_011.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_012.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_013.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| fx_glint_014.json, frames 0 to 8 at Full and Draft | largest difference 0 of 255; on the card in 18 of 18 frames; the same warnings: true | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000: the processor's frame 100 differs from the same with one colour and no shimmer, so the comparisons below test the new settings | 760712 pixels changed | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000, on three layers, frame 0, Full | largest difference 1 of 255, 2370 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000, on three layers, frame 100, Full | largest difference 1 of 255, 927 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000, on three layers, frame 239, Full | largest difference 1 of 255, 772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000, on three layers, frame 0, Draft | largest difference 1 of 255, 115 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000, on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, ray colours alternate, second colour #ff8000, on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24: the processor's frame 100 differs from the same with one colour and no shimmer, so the comparisons below test the new settings | 1290519 pixels changed | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24, on three layers, frame 0, Full | largest difference 1 of 255, 2139 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24, on three layers, frame 100, Full | largest difference 1 of 255, 908 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24, on three layers, frame 239, Full | largest difference 1 of 255, 594 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24, on three layers, frame 0, Draft | largest difference 1 of 255, 106 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24, on three layers, frame 100, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24, on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the town, set in a larger frame, in `verification/D-546 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| town_glare.png, Cross Glare (threshold 50, length 30), one colour, white: for comparison | 139746 pixels differ from the town alone; warnings [] | yes |
| town_alternate.png, Ray Colours Alternate, the second colour orange #ff8000: every other ray orange | 59898 pixels differ from one colour and no shimmer; warnings [] | yes |
| town_gradient.png, eight rays, Ray Colours Gradient from red #ff4040 to blue #4060ff: the rays go red to blue round the star | 146432 pixels differ from one colour and no shimmer; warnings [] | yes |
| town_shimmer_0.png, Shimmer 100, period 24, frame 0: some rays bright, some dim | 97787 pixels differ from one colour and no shimmer; warnings [] | yes |
| town_shimmer_6.png, the same at frame 6, a quarter period on: each ray has moved on its own beat | 106552 pixels differ from frame 0; warnings [] | yes |

## Result

203 of 203 checks pass.
