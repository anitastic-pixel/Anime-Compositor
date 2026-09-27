# B-103: speed lines

D-160, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-seventh of the third batch. Every expected pixel is `Fixtures/speed_lines/expected_speed_lines.json`, written by `tools/speed_lines_reference.py` before this code existed and printed in document 25 as FX-SPEED-001 to 029. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SPEED-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPEED-001 frame 0: The settings as they start: about the middle, #000000, count 120, thickness 1.5, inner 150, inner jitter 40, angle jitter 50, seed 0, hold 2, opacity 100. Every line starts at least 90 pixels from the centre, further than any pixel of the small drawing: the solid, untouched, at frame 0 and frame 4. | largest difference 0.0e0 | yes |
| FX-SPEED-001 frame 4: The settings as they start: about the middle, #000000, count 120, thickness 1.5, inner 150, inner jitter 40, angle jitter 50, seed 0, hold 2, opacity 100. Every line starts at least 90 pixels from the centre, further than any pixel of the small drawing: the solid, untouched, at frame 0 and frame 4. | largest difference 0.0e0 | yes |
| FX-SPEED-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-002 frame 0: Inner 0, the rest as they start: 120 thin lines reach the centre, and every pixel of the solid is greyed, none fully black. With hold 2, frame 1 is frame 0; frame 2 draws new lines, and frame 4 new again. | largest difference 3.0e-8 | yes |
| FX-SPEED-002 frame 1: Inner 0, the rest as they start: 120 thin lines reach the centre, and every pixel of the solid is greyed, none fully black. With hold 2, frame 1 is frame 0; frame 2 draws new lines, and frame 4 new again. | largest difference 3.0e-8 | yes |
| FX-SPEED-002 frame 2: Inner 0, the rest as they start: 120 thin lines reach the centre, and every pixel of the solid is greyed, none fully black. With hold 2, frame 1 is frame 0; frame 2 draws new lines, and frame 4 new again. | largest difference 3.0e-8 | yes |
| FX-SPEED-002 frame 4: Inner 0, the rest as they start: 120 thin lines reach the centre, and every pixel of the solid is greyed, none fully black. With hold 2, frame 1 is frame 0; frame 2 draws new lines, and frame 4 new again. | largest difference 2.9e-8 | yes |
| FX-SPEED-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-003 frame 0: Count 12, thickness 10, inner 2: twelve wide lines, jittered, rushing in to 2 pixels or so from the centre; frame 1 is frame 0, frames 2 and 4 are new lines. | largest difference 2.9e-8 | yes |
| FX-SPEED-003 frame 1: Count 12, thickness 10, inner 2: twelve wide lines, jittered, rushing in to 2 pixels or so from the centre; frame 1 is frame 0, frames 2 and 4 are new lines. | largest difference 2.9e-8 | yes |
| FX-SPEED-003 frame 2: Count 12, thickness 10, inner 2: twelve wide lines, jittered, rushing in to 2 pixels or so from the centre; frame 1 is frame 0, frames 2 and 4 are new lines. | largest difference 3.0e-8 | yes |
| FX-SPEED-003 frame 4: Count 12, thickness 10, inner 2: twelve wide lines, jittered, rushing in to 2 pixels or so from the centre; frame 1 is frame 0, frames 2 and 4 are new lines. | largest difference 2.9e-8 | yes |
| FX-SPEED-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-004 frame 0: FX-SPEED-003 with inner jitter 0 and angle jitter 0: the lines exactly every 30 degrees, the first straight up, so columns 7 and 8, either side of the lines straight up and straight down, are darkened alike in rows 0 to 2 and 7 to 9, and the pixels nearest the centre, under 1.5 pixels from it, untouched. | largest difference 3.0e-8 | yes |
| FX-SPEED-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-005 frame 0: FX-SPEED-004 with inner 5 and inner jitter 100: each line starts anywhere from the centre to 10 pixels out, so some pixels under 4.5 pixels from the centre are reached, which inner jitter 0 leaves untouched, and some further out are not. | largest difference 2.7e-8 | yes |
| FX-SPEED-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-006 frame 0: FX-SPEED-004 with angle jitter 100: each line moved off its even place by up to half a spacing, 15 degrees either way. | largest difference 2.8e-8 | yes |
| FX-SPEED-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-007 frame 0: FX-SPEED-003 with thickness 30: wedges three times as wide, darkening more of the solid, none less. | largest difference 2.9e-8 | yes |
| FX-SPEED-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-008 frame 0: FX-SPEED-003 with thickness 0: each line a hairline, its softened edge alone showing, so no pixel is more than half way to black. | largest difference 2.9e-8 | yes |
| FX-SPEED-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-009 frame 0: Count 4, thickness 30, inner 0, both jitters 0: four wedges, up, right, down and left, a cross, black at its arms' ends; the diagonal pixels (9, 6) to (11, 8) and (6, 3) to (4, 1), between the arms, untouched, and the four pixels about the centre greyed, where the arms' softened edges meet. | largest difference 2.8e-8 | yes |
| FX-SPEED-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-010 frame 0: Count 1000, thickness 30, inner 0: the lines overlap everywhere, so every pixel 4 or more pixels from the centre is black. | largest difference 1.4e-8 | yes |
| FX-SPEED-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-011 frame 0: FX-SPEED-003 with seed 7.9, which counts as 7: lines of their own, not FX-SPEED-003's. | largest difference 3.0e-8 | yes |
| FX-SPEED-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-012 frame 0: FX-SPEED-003 with hold 1: new lines every frame, so frame 1 is FX-SPEED-003's frame 2 and frame 2 is its frame 4. | largest difference 2.9e-8 | yes |
| FX-SPEED-012 frame 1: FX-SPEED-003 with hold 1: new lines every frame, so frame 1 is FX-SPEED-003's frame 2 and frame 2 is its frame 4. | largest difference 3.0e-8 | yes |
| FX-SPEED-012 frame 2: FX-SPEED-003 with hold 1: new lines every frame, so frame 1 is FX-SPEED-003's frame 2 and frame 2 is its frame 4. | largest difference 2.9e-8 | yes |
| FX-SPEED-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-013 frame 0: FX-SPEED-003 with hold 100: the same lines on every frame, frame 4 is frame 0. | largest difference 2.9e-8 | yes |
| FX-SPEED-013 frame 4: FX-SPEED-003 with hold 100: the same lines on every frame, frame 4 is frame 0. | largest difference 2.9e-8 | yes |
| FX-SPEED-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-014 frame 0: FX-SPEED-003 with opacity 50: each pixel half as far toward black. | largest difference 3.0e-8 | yes |
| FX-SPEED-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-015 frame 0: FX-SPEED-003 in #3a6fd8, a blue: the same lines, blue. | largest difference 2.9e-8 | yes |
| FX-SPEED-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-016 frame 0: FX-SPEED-015 with the colour written in capitals, #3A6FD8: the same. | largest difference 2.9e-8 | yes |
| FX-SPEED-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-017 frame 0: FX-SPEED-003 about the middle of the left edge, centre 0, 50: the lines rush in toward the left edge, and the two pixels at its middle, (0, 4) and (0, 5), inside every line's start, are untouched. | largest difference 2.8e-8 | yes |
| FX-SPEED-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-018 frame 0: FX-SPEED-003 with count keyed from 4 at frame 0 to 20 at frame 4, linear: frame 0 has four lines, frame 1 eight, frame 2, at 12, is FX-SPEED-003's frame 2, and frame 4 has twenty. | largest difference 2.9e-8 | yes |
| FX-SPEED-018 frame 1: FX-SPEED-003 with count keyed from 4 at frame 0 to 20 at frame 4, linear: frame 0 has four lines, frame 1 eight, frame 2, at 12, is FX-SPEED-003's frame 2, and frame 4 has twenty. | largest difference 2.9e-8 | yes |
| FX-SPEED-018 frame 2: FX-SPEED-003 with count keyed from 4 at frame 0 to 20 at frame 4, linear: frame 0 has four lines, frame 1 eight, frame 2, at 12, is FX-SPEED-003's frame 2, and frame 4 has twenty. | largest difference 3.0e-8 | yes |
| FX-SPEED-018 frame 4: FX-SPEED-003 with count keyed from 4 at frame 0 to 20 at frame 4, linear: frame 0 has four lines, frame 1 eight, frame 2, at 12, is FX-SPEED-003's frame 2, and frame 4 has twenty. | largest difference 3.0e-8 | yes |
| FX-SPEED-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-019 frame 0: FX-SPEED-003 with opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the solid; frame 2 is held at 100 and is FX-SPEED-003's frame 2. | largest difference 0.0e0 | yes |
| FX-SPEED-019 frame 2: FX-SPEED-003 with opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the solid; frame 2 is held at 100 and is FX-SPEED-003's frame 2. | largest difference 3.0e-8 | yes |
| FX-SPEED-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-020 frame 0: FX-SPEED-003 moved three pixels right: the lines are worked in the drawing's own space, so they move with it; the three columns left of it are empty. | largest difference 2.9e-8 | yes |
| FX-SPEED-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-021 frame 0: FX-SPEED-003 on Noise's card rather than the solid: the same lines, drawn only inside the covering; the empty pixels stay empty, the soft right edge keeps its half covering, and the black patch stays black. | largest difference 1.9e-7 | yes |
| FX-SPEED-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPEED-022 frame 0: Count 3, below 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-022 frame 4: Count 3, below 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-023 frame 0: Thickness 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-023 frame 4: Thickness 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-024 frame 0: Inner -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-024 frame 4: Inner -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-025 frame 0: Inner jitter 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-025 frame 4: Inner jitter 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-026 frame 0: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-026 frame 4: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-027 frame 0: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-027 frame 4: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-028 frame 0: Opacity keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-028 frame 4: Opacity keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPEED-029 frame 0: Colour "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-029 frame 4: Colour "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SPEED-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the inner edge, 150 to 75, and nothing else | SpeedLines { center: [50.0, 50.0], color: "#000000", count: 120.0, thickness: 1.5, inner: 75.0, inner_jitter: 40.0, angle_jitter: 50.0, seed: 0.0, hold: 2.0, opacity: 100.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_speed_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_speed_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `count` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| count 3 is refused with a sentence, and nothing changes | Speed Lines's count runs from 4 to 1000, and this is 3. | yes |
| thickness 31 is refused with a sentence, and nothing changes | Speed Lines's thickness runs from 0 to 30, and this is 31. | yes |
| inner -1 is refused with a sentence, and nothing changes | Speed Lines's inner runs from 0 to 100000, and this is -1. | yes |
| inner jitter 101 is refused with a sentence, and nothing changes | Speed Lines's inner jitter runs from 0 to 100, and this is 101. | yes |
| angle jitter 101 is refused with a sentence, and nothing changes | Speed Lines's angle jitter runs from 0 to 100, and this is 101. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Speed Lines's seed runs from 0 to 100000, and this is 100001. | yes |
| hold 0 is refused with a sentence, and nothing changes | Speed Lines's hold runs from 1 to 100, and this is 0. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Speed Lines's opacity runs from 0 to 100, and this is 101. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Speed Lines's center runs from -1000 to 1000, and this is 1001. | yes |
| colour "#12345" is refused with a sentence, and nothing changes | Speed Lines's colour is written #rrggbb, and this is "#12345". | yes |
| opacity keyed to 150 is refused with a sentence, and nothing changes | Speed Lines's opacity runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| count keyed from 4 to 20 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_speed_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_speed_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_speed_018.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## Result

116 of 116 checks pass.
