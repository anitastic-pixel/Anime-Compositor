# B-400: CC Wide Time

D-520, under the owner's /loop request of 2026-10-11. Every expected pixel is `Fixtures/cc_wide_time/expected_cc_wide_time.json`, written by `tools/cc_wide_time_reference.py` before this code existed and printed in document 25 as FX-WIDE-001 to 019. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-WIDE-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WIDE-001 frame 0: As added: three frames before and three after, seven averaged. At frame 3 drawings 1 to 4 and drawing 5 three times; at frame 0 the three frames before the layer are empty but still counted, so the ball is faint; at frame 6 frame 8 is past the out point. | largest difference 3.0e-8 | yes |
| FX-WIDE-001 frame 3: As added: three frames before and three after, seven averaged. At frame 3 drawings 1 to 4 and drawing 5 three times; at frame 0 the three frames before the layer are empty but still counted, so the ball is faint; at frame 6 frame 8 is past the out point. | largest difference 6.3e-8 | yes |
| FX-WIDE-001 frame 6: As added: three frames before and three after, seven averaged. At frame 3 drawings 1 to 4 and drawing 5 three times; at frame 0 the three frames before the layer are empty but still counted, so the ball is faint; at frame 6 frame 8 is past the out point. | largest difference 7.4e-8 | yes |
| FX-WIDE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-002 frame 0: No steps either way: the layer exactly as it is. | largest difference 9.1e-8 | yes |
| FX-WIDE-002 frame 3: No steps either way: the layer exactly as it is. | largest difference 9.1e-8 | yes |
| FX-WIDE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-003 frame 1: Two forward steps only: frames 1, 2 and 3 averaged at frame 1. | largest difference 8.1e-8 | yes |
| FX-WIDE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-004 frame 4: Two backward steps only: frames 2, 3 and 4 averaged at frame 4. | largest difference 8.1e-8 | yes |
| FX-WIDE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-005 frame 2: One step each way, the layer's in point at frame 2: at frame 2 the frame before is outside the layer and counts empty. | largest difference 8.1e-8 | yes |
| FX-WIDE-005 frame 3: One step each way, the layer's in point at frame 2: at frame 2 the frame before is outside the layer and counts empty. | largest difference 8.1e-8 | yes |
| FX-WIDE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-006 frame 2: One step each way, drawings on twos: at frame 2 drawings 1, 2 and 2; at frame 3 drawings 2, 2 and 3. | largest difference 1.5e-7 | yes |
| FX-WIDE-006 frame 3: One step each way, drawings on twos: at frame 2 drawings 1, 2 and 2; at frame 3 drawings 2, 2 and 3. | largest difference 1.5e-7 | yes |
| FX-WIDE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-007 frame 0: Forward steps keyed from 0 at frame 0 to 4 at frame 4, no backward steps: 2 at frame 2. | largest difference 9.1e-8 | yes |
| FX-WIDE-007 frame 2: Forward steps keyed from 0 at frame 0 to 4 at frame 4, no backward steps: 2 at frame 2. | largest difference 8.1e-8 | yes |
| FX-WIDE-007 frame 4: Forward steps keyed from 0 at frame 0 to 4 at frame 4, no backward steps: 2 at frame 2. | largest difference 8.9e-8 | yes |
| FX-WIDE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-008 frame 3: Steps 1.5 forward and 1.9 back count as one each. | largest difference 8.1e-8 | yes |
| FX-WIDE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-009 frame 3: An Exposure of +1 before CC Wide Time is not seen: it reads the drawing, not what came before it. FX-WIDE-001. | largest difference 6.3e-8 | yes |
| FX-WIDE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-010 frame 3: An Exposure of -1 after CC Wide Time darkens the smear too. | largest difference 3.1e-8 | yes |
| FX-WIDE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-011 frame 3: A mask keeping columns 0 to 6: every frame is masked. | largest difference 3.0e-8 | yes |
| FX-WIDE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-012 frame 3: The holder moved 2 right and 3 down: FX-WIDE-001 moved, since the frames lie on the layer. | largest difference 6.3e-8 | yes |
| FX-WIDE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-013 frame 3: The effect on an adjustment layer above the holder: nothing changes. | largest difference 9.1e-8 | yes |
| FX-WIDE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-014 frame 3: Native Motion Blur on, with no composition inside: FX-WIDE-001. | largest difference 6.3e-8 | yes |
| FX-WIDE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-015 frame 4: Ten steps each way, past both ends of the layer: 21 frames counted, eight of them drawn. | largest difference 2.1e-8 | yes |
| FX-WIDE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDE-016 frame 0: Forward steps 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WIDE-016 frame 4: Forward steps 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-WIDE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDE-017 frame 0: Backward steps -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WIDE-017 frame 4: Backward steps -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-WIDE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDE-018 frame 0: Backward steps keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WIDE-018 frame 4: Backward steps keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-WIDE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDE-019 frame 0: Native Motion Blur written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WIDE-019 frame 4: Native Motion Blur written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-WIDE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| CC Wide Time never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is, since none is a distance | CcWideTime { forward_steps: 5.0, backward_steps: 2.0, native_motion_blur: "on", picture: None } | yes |

## Native Motion Blur, on a composition inside the holder

| Check | The build's answer | Matches |
| --- | --- | --- |
| with the inner composition's motion blur enabled, Off and On draw frame 1 differently | largest difference 0.12500001 | yes |
| Off draws frame 1 exactly as On does with the inner composition's blur switched off | byte-identical | yes |
| with no blur inside, Off and On draw frame 1 the same | byte-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wide_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wide_011.json, its mask in the first form, is saved as one mask of the same four          corners in the current form, and the saved file draws frame 3 the same | 1 mask(s), frame 3 byte-identical | yes |
| the averaged frames it lays are never saved | None | yes |
| a file with no `native_motion_blur` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `forward_steps` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a step count that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Native Motion Blur that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Forward Steps 101 is refused with a sentence, and nothing changes | CC Wide Time's forward steps runs from 0 to 100, and this is 101. | yes |
| Backward Steps -1 is refused with a sentence, and nothing changes | CC Wide Time's backward steps runs from 0 to 100, and this is -1. | yes |
| Native Motion Blur "maybe" is refused with a sentence, and nothing changes | CC Wide Time's native motion blur is "off" or "on", and this is "maybe". | yes |
| Backward Steps keyed to 150 at frame 4 is refused with a sentence, and nothing changes | CC Wide Time's backward steps runs from 0 to 100, and this is 150. | yes |
| Five forward, none back, Native Motion Blur on is taken | taken | yes |
| Forward Steps keyed from 0 at frame 0 to 6 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a ball bouncing on ones over a night-blue solid, in `verification/D-520 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, frame 16 with no effect, draws cleanly, the ball in columns 282 to 330 | [], columns 282 to 329 | yes |
| 2_as_added.png, three steps each way, as added: drawings 14 to 20 averaged, draws cleanly in columns 234 to 378 | [], columns 234 to 377 | yes |
| 3_behind.png, six steps back only: a smear behind the ball, to drawing 11, draws cleanly in columns 186 to 330 | [], columns 186 to 329 | yes |
| 4_ahead.png, four steps forward only: a smear ahead, to drawing 21, draws cleanly in columns 282 to 394 | [], columns 282 to 393 | yes |
| 5_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size: the smear in columns 58 to 94 (the last row, half covered by the solid, left out) | [], 120 wide, columns 58 to 94 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wide_001.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wide_006.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wide_011.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wide_012.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

94 of 94 checks pass.
