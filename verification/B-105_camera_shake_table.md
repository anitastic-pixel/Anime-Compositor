# B-105: camera shake

D-162, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-ninth of the third batch. Every expected pixel is `Fixtures/camera_shake/expected_camera_shake.json`, written by `tools/camera_shake_reference.py` before this code existed and printed in document 25 as FX-SHAKE-001 to 021. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SHAKE-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHAKE-001 frame 0: The settings as they start: amount 10, rotation 0, hold 1, seed 0. Each frame the drawing jolts to a new place, up to 10 pixels each way, larger than the drawing itself: frame 0 moves it 0.57 pixels left and 2.96 down, frame 1 9.51 right and 9.94 up, out of the frame, so frame 1 is empty, frame 2 6.04 left and 1.57 up, frame 3 1.54 right and 0.47 down, and frame 4 8.05 left and 6.89 down. | largest difference 1.9e-7 | yes |
| FX-SHAKE-001 frame 1: The settings as they start: amount 10, rotation 0, hold 1, seed 0. Each frame the drawing jolts to a new place, up to 10 pixels each way, larger than the drawing itself: frame 0 moves it 0.57 pixels left and 2.96 down, frame 1 9.51 right and 9.94 up, out of the frame, so frame 1 is empty, frame 2 6.04 left and 1.57 up, frame 3 1.54 right and 0.47 down, and frame 4 8.05 left and 6.89 down. | largest difference 0.0e0 | yes |
| FX-SHAKE-001 frame 2: The settings as they start: amount 10, rotation 0, hold 1, seed 0. Each frame the drawing jolts to a new place, up to 10 pixels each way, larger than the drawing itself: frame 0 moves it 0.57 pixels left and 2.96 down, frame 1 9.51 right and 9.94 up, out of the frame, so frame 1 is empty, frame 2 6.04 left and 1.57 up, frame 3 1.54 right and 0.47 down, and frame 4 8.05 left and 6.89 down. | largest difference 2.5e-7 | yes |
| FX-SHAKE-001 frame 3: The settings as they start: amount 10, rotation 0, hold 1, seed 0. Each frame the drawing jolts to a new place, up to 10 pixels each way, larger than the drawing itself: frame 0 moves it 0.57 pixels left and 2.96 down, frame 1 9.51 right and 9.94 up, out of the frame, so frame 1 is empty, frame 2 6.04 left and 1.57 up, frame 3 1.54 right and 0.47 down, and frame 4 8.05 left and 6.89 down. | largest difference 2.5e-7 | yes |
| FX-SHAKE-001 frame 4: The settings as they start: amount 10, rotation 0, hold 1, seed 0. Each frame the drawing jolts to a new place, up to 10 pixels each way, larger than the drawing itself: frame 0 moves it 0.57 pixels left and 2.96 down, frame 1 9.51 right and 9.94 up, out of the frame, so frame 1 is empty, frame 2 6.04 left and 1.57 up, frame 3 1.54 right and 0.47 down, and frame 4 8.05 left and 6.89 down. | largest difference 1.7e-7 | yes |
| FX-SHAKE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-002 frame 0: Amount 0 and rotation 0: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-SHAKE-002 frame 2: Amount 0 and rotation 0: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-SHAKE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-003 frame 0: Amount 2: the same jolts at a fifth of the size, so the drawing stays in the frame and each frame shows it moved a little differently, by a fifth of FX-SHAKE-001's move: frame 0 by 0.11 left and 0.59 down, frame 1 by 1.90 right and 1.99 up. | largest difference 2.5e-7 | yes |
| FX-SHAKE-003 frame 1: Amount 2: the same jolts at a fifth of the size, so the drawing stays in the frame and each frame shows it moved a little differently, by a fifth of FX-SHAKE-001's move: frame 0 by 0.11 left and 0.59 down, frame 1 by 1.90 right and 1.99 up. | largest difference 1.9e-7 | yes |
| FX-SHAKE-003 frame 2: Amount 2: the same jolts at a fifth of the size, so the drawing stays in the frame and each frame shows it moved a little differently, by a fifth of FX-SHAKE-001's move: frame 0 by 0.11 left and 0.59 down, frame 1 by 1.90 right and 1.99 up. | largest difference 1.9e-7 | yes |
| FX-SHAKE-003 frame 3: Amount 2: the same jolts at a fifth of the size, so the drawing stays in the frame and each frame shows it moved a little differently, by a fifth of FX-SHAKE-001's move: frame 0 by 0.11 left and 0.59 down, frame 1 by 1.90 right and 1.99 up. | largest difference 1.6e-7 | yes |
| FX-SHAKE-003 frame 4: Amount 2: the same jolts at a fifth of the size, so the drawing stays in the frame and each frame shows it moved a little differently, by a fifth of FX-SHAKE-001's move: frame 0 by 0.11 left and 0.59 down, frame 1 by 1.90 right and 1.99 up. | largest difference 2.5e-7 | yes |
| FX-SHAKE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-004 frame 0: Amount 2, hold 2: each jolt is kept for two frames, a shake on twos: frames 0 and 1 are FX-SHAKE-003's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 2.5e-7 | yes |
| FX-SHAKE-004 frame 1: Amount 2, hold 2: each jolt is kept for two frames, a shake on twos: frames 0 and 1 are FX-SHAKE-003's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 2.5e-7 | yes |
| FX-SHAKE-004 frame 2: Amount 2, hold 2: each jolt is kept for two frames, a shake on twos: frames 0 and 1 are FX-SHAKE-003's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 1.9e-7 | yes |
| FX-SHAKE-004 frame 3: Amount 2, hold 2: each jolt is kept for two frames, a shake on twos: frames 0 and 1 are FX-SHAKE-003's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 1.9e-7 | yes |
| FX-SHAKE-004 frame 4: Amount 2, hold 2: each jolt is kept for two frames, a shake on twos: frames 0 and 1 are FX-SHAKE-003's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 1.9e-7 | yes |
| FX-SHAKE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-005 frame 0: Amount 2, hold 5: one jolt kept for all five frames, each FX-SHAKE-003's frame 0. | largest difference 2.5e-7 | yes |
| FX-SHAKE-005 frame 2: Amount 2, hold 5: one jolt kept for all five frames, each FX-SHAKE-003's frame 0. | largest difference 2.5e-7 | yes |
| FX-SHAKE-005 frame 4: Amount 2, hold 5: one jolt kept for all five frames, each FX-SHAKE-003's frame 0. | largest difference 2.5e-7 | yes |
| FX-SHAKE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-006 frame 0: Amount 2, hold 1.9, floored to 1: FX-SHAKE-003 exactly. | largest difference 2.5e-7 | yes |
| FX-SHAKE-006 frame 2: Amount 2, hold 1.9, floored to 1: FX-SHAKE-003 exactly. | largest difference 1.9e-7 | yes |
| FX-SHAKE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-007 frame 0: Amount 2, seed 1: another shake, frame 0 moving the drawing 1.18 pixels left and 1.72 up where FX-SHAKE-003's frame 0 moves it down. | largest difference 2.1e-7 | yes |
| FX-SHAKE-007 frame 2: Amount 2, seed 1: another shake, frame 0 moving the drawing 1.18 pixels left and 1.72 up where FX-SHAKE-003's frame 0 moves it down. | largest difference 2.4e-7 | yes |
| FX-SHAKE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-008 frame 0: Amount 2, seed 1.6, floored to 1: FX-SHAKE-007 exactly. | largest difference 2.1e-7 | yes |
| FX-SHAKE-008 frame 2: Amount 2, seed 1.6, floored to 1: FX-SHAKE-007 exactly. | largest difference 2.4e-7 | yes |
| FX-SHAKE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-009 frame 0: Amount 0, rotation 10: the drawing is not moved, only turned about its centre, frame 0 by 9.61 degrees anticlockwise, so the blue band tilts up to the right, and frame 2 by 3.47 degrees clockwise, so it tilts down to the right. | largest difference 2.5e-7 | yes |
| FX-SHAKE-009 frame 2: Amount 0, rotation 10: the drawing is not moved, only turned about its centre, frame 0 by 9.61 degrees anticlockwise, so the blue band tilts up to the right, and frame 2 by 3.47 degrees clockwise, so it tilts down to the right. | largest difference 2.5e-7 | yes |
| FX-SHAKE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-010 frame 0: Amount 2, rotation 10: moved as FX-SHAKE-003 and turned as FX-SHAKE-009, different from both. | largest difference 2.5e-7 | yes |
| FX-SHAKE-010 frame 2: Amount 2, rotation 10: moved as FX-SHAKE-003 and turned as FX-SHAKE-009, different from both. | largest difference 2.5e-7 | yes |
| FX-SHAKE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-011 frame 0: Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 2, FX-SHAKE-003's frame 2, and frame 4 amount 4. | largest difference 1.9e-7 | yes |
| FX-SHAKE-011 frame 2: Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 2, FX-SHAKE-003's frame 2, and frame 4 amount 4. | largest difference 1.9e-7 | yes |
| FX-SHAKE-011 frame 4: Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 2, FX-SHAKE-003's frame 2, and frame 4 amount 4. | largest difference 2.5e-7 | yes |
| FX-SHAKE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-012 frame 0: Amount 0, rotation keyed from 0 at frame 0 to 45 at frame 4, eased past its end (about 60 at frame 2): frame 0 is the drawing, and frame 2 is held at 45, so it is rotation 45's frame 2. | largest difference 1.9e-7 | yes |
| FX-SHAKE-012 frame 2: Amount 0, rotation keyed from 0 at frame 0 to 45 at frame 4, eased past its end (about 60 at frame 2): frame 0 is the drawing, and frame 2 is held at 45, so it is rotation 45's frame 2. | largest difference 2.5e-7 | yes |
| FX-SHAKE-012 frame 4: Amount 0, rotation keyed from 0 at frame 0 to 45 at frame 4, eased past its end (about 60 at frame 2): frame 0 is the drawing, and frame 2 is held at 45, so it is rotation 45's frame 2. | largest difference 2.5e-7 | yes |
| FX-SHAKE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-013 frame 0: Amount 3, seed 7, moved three pixels right: frame 0 moves the drawing 2.73 pixels left and 0.25 down, into the grown pixels, and the three columns left of the drawing show them in place; every other column is the unmoved layer's, three columns on. | largest difference 1.7e-7 | yes |
| FX-SHAKE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-014 frame 0: Amount 0, rotation 45, moved three pixels right: frame 0 turns the drawing 43.25 degrees anticlockwise, and its top-left corner swings down and out past its left edge into the grown pixels, shown in rows 5 to 7 of the columns left of the drawing. | largest difference 2.5e-7 | yes |
| FX-SHAKE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAKE-015 frame 0: Amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-015 frame 4: Amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAKE-016 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-016 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAKE-017 frame 0: Rotation 46, above 45. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-017 frame 4: Rotation 46, above 45. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAKE-018 frame 0: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-018 frame 4: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAKE-019 frame 0: Hold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-019 frame 4: Hold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAKE-020 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-020 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAKE-021 frame 0: Amount keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-021 frame 4: Amount keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHAKE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it declares no fixed growth to the card, which never runs it: its turn's reach depends on the size the drawing reaches it at, and is counted as the stack runs, as FX-SHAKE-013 and 014 show | 0 | yes |
| a half-size draft preview halves the amount, 10 to 5, and nothing else | CameraShake { amount: 5.0, rotation: 0.0, hold: 1.0, seed: 0.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shake_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shake_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `hold` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an amount that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 1001 is refused with a sentence, and nothing changes | Camera Shake's amount runs from 0 to 1000, and this is 1001. | yes |
| amount -1 is refused with a sentence, and nothing changes | Camera Shake's amount runs from 0 to 1000, and this is -1. | yes |
| rotation 46 is refused with a sentence, and nothing changes | Camera Shake's rotation runs from 0 to 45, and this is 46. | yes |
| hold 0 is refused with a sentence, and nothing changes | Camera Shake's hold runs from 1 to 100, and this is 0. | yes |
| hold 101 is refused with a sentence, and nothing changes | Camera Shake's hold runs from 1 to 100, and this is 101. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Camera Shake's seed runs from 0 to 100000, and this is 100001. | yes |
| amount keyed to 1500 is refused with a sentence, and nothing changes | Camera Shake's amount runs from 0 to 1000, and this is 1500. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| amount keyed from 0 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shake_003.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shake_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shake_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

104 of 104 checks pass.
