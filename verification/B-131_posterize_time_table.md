# B-131: posterize time

D-196, accepted by the owner on 2026-09-28 ("take everything"). Every expected pixel is `Fixtures/posterize_time/expected_posterize_time.json`, written by `tools/posterize_time_reference.py` before this code existed and printed in document 25 as FX-PTIME-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-PTIME-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PTIME-001 frame 0: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 1.1e-7 | yes |
| FX-PTIME-001 frame 1: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 1.1e-7 | yes |
| FX-PTIME-001 frame 2: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 9.1e-8 | yes |
| FX-PTIME-001 frame 3: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 9.1e-8 | yes |
| FX-PTIME-001 frame 4: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 9.1e-8 | yes |
| FX-PTIME-001 frame 5: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 9.1e-8 | yes |
| FX-PTIME-001 frame 6: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 1.1e-7 | yes |
| FX-PTIME-001 frame 7: As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7. | largest difference 1.1e-7 | yes |
| FX-PTIME-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-002 frame 1: 24 a second, the composition's own rate: nothing is held. | largest difference 9.1e-8 | yes |
| FX-PTIME-002 frame 3: 24 a second, the composition's own rate: nothing is held. | largest difference 9.1e-8 | yes |
| FX-PTIME-002 frame 5: 24 a second, the composition's own rate: nothing is held. | largest difference 9.1e-8 | yes |
| FX-PTIME-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-003 frame 3: 30 a second, above the composition's rate: nothing is held. | largest difference 9.1e-8 | yes |
| FX-PTIME-003 frame 5: 30 a second, above the composition's rate: nothing is held. | largest difference 9.1e-8 | yes |
| FX-PTIME-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-004 frame 1: 8 a second: on threes. Frames 1 and 2 show frame 0's drawing, 3 to 5 frame 3's. | largest difference 1.1e-7 | yes |
| FX-PTIME-004 frame 2: 8 a second: on threes. Frames 1 and 2 show frame 0's drawing, 3 to 5 frame 3's. | largest difference 1.1e-7 | yes |
| FX-PTIME-004 frame 3: 8 a second: on threes. Frames 1 and 2 show frame 0's drawing, 3 to 5 frame 3's. | largest difference 9.1e-8 | yes |
| FX-PTIME-004 frame 4: 8 a second: on threes. Frames 1 and 2 show frame 0's drawing, 3 to 5 frame 3's. | largest difference 9.1e-8 | yes |
| FX-PTIME-004 frame 5: 8 a second: on threes. Frames 1 and 2 show frame 0's drawing, 3 to 5 frame 3's. | largest difference 9.1e-8 | yes |
| FX-PTIME-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-005 frame 2: 10 a second: uneven holds, frames 0 to 2, then 3 and 4, then 5 on. | largest difference 1.1e-7 | yes |
| FX-PTIME-005 frame 3: 10 a second: uneven holds, frames 0 to 2, then 3 and 4, then 5 on. | largest difference 9.1e-8 | yes |
| FX-PTIME-005 frame 4: 10 a second: uneven holds, frames 0 to 2, then 3 and 4, then 5 on. | largest difference 9.1e-8 | yes |
| FX-PTIME-005 frame 5: 10 a second: uneven holds, frames 0 to 2, then 3 and 4, then 5 on. | largest difference 9.1e-8 | yes |
| FX-PTIME-005 frame 6: 10 a second: uneven holds, frames 0 to 2, then 3 and 4, then 5 on. | largest difference 9.1e-8 | yes |
| FX-PTIME-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-006 frame 3: 6 a second: on fours. Frame 3 shows frame 0's drawing, 4 and 7 frame 4's. | largest difference 1.1e-7 | yes |
| FX-PTIME-006 frame 4: 6 a second: on fours. Frame 3 shows frame 0's drawing, 4 and 7 frame 4's. | largest difference 9.1e-8 | yes |
| FX-PTIME-006 frame 7: 6 a second: on fours. Frame 3 shows frame 0's drawing, 4 and 7 frame 4's. | largest difference 9.1e-8 | yes |
| FX-PTIME-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-007 frame 2: The layer's in point at frame 3, 12 a second: at frame 2 there is no layer; at frame 3 its first drawing, not a frame before it; frames 4 and 5 its second. | largest difference 0.0e0 | yes |
| FX-PTIME-007 frame 3: The layer's in point at frame 3, 12 a second: at frame 2 there is no layer; at frame 3 its first drawing, not a frame before it; frames 4 and 5 its second. | largest difference 1.1e-7 | yes |
| FX-PTIME-007 frame 4: The layer's in point at frame 3, 12 a second: at frame 2 there is no layer; at frame 3 its first drawing, not a frame before it; frames 4 and 5 its second. | largest difference 9.1e-8 | yes |
| FX-PTIME-007 frame 5: The layer's in point at frame 3, 12 a second: at frame 2 there is no layer; at frame 3 its first drawing, not a frame before it; frames 4 and 5 its second. | largest difference 9.1e-8 | yes |
| FX-PTIME-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-008 frame 2: The position keyed from (0, 0) at frame 0 to (4, 0) at frame 4: the drawing holds and the layer still moves. Frame 3 shows drawing 3 moved 3 right. | largest difference 9.1e-8 | yes |
| FX-PTIME-008 frame 3: The position keyed from (0, 0) at frame 0 to (4, 0) at frame 4: the drawing holds and the layer still moves. Frame 3 shows drawing 3 moved 3 right. | largest difference 9.1e-8 | yes |
| FX-PTIME-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-009 frame 3: An Exposure before it, keyed from 0 at frame 0 to +2 at frame 4: held with the drawing. Frame 3 is drawing 3 at +1. | largest difference 1.8e-7 | yes |
| FX-PTIME-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-010 frame 3: The same Exposure after it: held too, FX-PTIME-009. | largest difference 1.8e-7 | yes |
| FX-PTIME-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-011 frame 3: The effect on an adjustment layer above the holder: nothing changes. | largest difference 9.1e-8 | yes |
| FX-PTIME-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-012 frame 4: Two, 12 then 8: frame 4 is held at 4 by the first and 3 by the second; frames 4 and 5 show frame 3's drawing, frame 6 its own. | largest difference 9.1e-8 | yes |
| FX-PTIME-012 frame 5: Two, 12 then 8: frame 4 is held at 4 by the first and 3 by the second; frames 4 and 5 show frame 3's drawing, frame 6 its own. | largest difference 9.1e-8 | yes |
| FX-PTIME-012 frame 6: Two, 12 then 8: frame 4 is held at 4 by the first and 3 by the second; frames 4 and 5 show frame 3's drawing, frame 6 its own. | largest difference 1.1e-7 | yes |
| FX-PTIME-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-013 frame 2: An Echo after it, one echo one frame back, Add: frame 3 is frame 2's drawing and frame 1's added, the same as frame 2. | largest difference 1.4e-7 | yes |
| FX-PTIME-013 frame 3: An Echo after it, one echo one frame back, Add: frame 3 is frame 2's drawing and frame 1's added, the same as frame 2. | largest difference 1.4e-7 | yes |
| FX-PTIME-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-014 frame 3: Switched off: nothing is held. | largest difference 9.1e-8 | yes |
| FX-PTIME-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-015 frame 3: The rate keyed, 24 held until frame 4 and 8 from there: frames 0 to 3 on ones, then frames 4 and 5 show frame 3's drawing and frame 6 its own. | largest difference 9.1e-8 | yes |
| FX-PTIME-015 frame 4: The rate keyed, 24 held until frame 4 and 8 from there: frames 0 to 3 on ones, then frames 4 and 5 show frame 3's drawing and frame 6 its own. | largest difference 9.1e-8 | yes |
| FX-PTIME-015 frame 5: The rate keyed, 24 held until frame 4 and 8 from there: frames 0 to 3 on ones, then frames 4 and 5 show frame 3's drawing and frame 6 its own. | largest difference 9.1e-8 | yes |
| FX-PTIME-015 frame 6: The rate keyed, 24 held until frame 4 and 8 from there: frames 0 to 3 on ones, then frames 4 and 5 show frame 3's drawing and frame 6 its own. | largest difference 1.1e-7 | yes |
| FX-PTIME-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-016 frame 3: On a composition layer showing a composition of the same drawings: frame 3 shows the inner composition's frame 2. | largest difference 9.1e-8 | yes |
| FX-PTIME-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-017 frame 2: 12 a second in a composition of 30: frames 0 to 2 show frame 0's drawing, 3 and 4 frame 3's, 5 to 7 frame 5's. | largest difference 1.1e-7 | yes |
| FX-PTIME-017 frame 3: 12 a second in a composition of 30: frames 0 to 2 show frame 0's drawing, 3 and 4 frame 3's, 5 to 7 frame 5's. | largest difference 9.1e-8 | yes |
| FX-PTIME-017 frame 4: 12 a second in a composition of 30: frames 0 to 2 show frame 0's drawing, 3 and 4 frame 3's, 5 to 7 frame 5's. | largest difference 9.1e-8 | yes |
| FX-PTIME-017 frame 5: 12 a second in a composition of 30: frames 0 to 2 show frame 0's drawing, 3 and 4 frame 3's, 5 to 7 frame 5's. | largest difference 9.1e-8 | yes |
| FX-PTIME-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PTIME-018 frame 1: Frame rate 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-PTIME-018 frame 3: Frame rate 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-PTIME-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PTIME-019 frame 1: Frame rate 100, above 99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-PTIME-019 frame 3: Frame rate 100, above 99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-PTIME-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PTIME-020 frame 1: Frame rate keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-PTIME-020 frame 3: Frame rate keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-PTIME-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| holding never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves the frame rate as it is, since it is not a distance | PosterizeTime { frame_rate: 8.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ptime_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ptime_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `frame_rate` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a frame rate that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Frame Rate 0.05 is refused with a sentence, and nothing changes | Posterize Time's frame rate runs from 0.1 to 99, and this is 0.05. | yes |
| Frame Rate 100 is refused with a sentence, and nothing changes | Posterize Time's frame rate runs from 0.1 to 99, and this is 100. | yes |
| Frame Rate keyed to 120 at frame 4 is refused with a sentence, and nothing changes | Posterize Time's frame rate runs from 0.1 to 99, and this is 120. | yes |
| Frame Rate 8 is taken | taken | yes |
| Frame Rate keyed from 24 at frame 0 to 8 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a ball bouncing on ones over a night-blue solid, in `verification/B-131 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| strip.png, frames 12 to 19 left to right: on ones with no effect, then Posterize Time 12, then 8, top to bottom; every frame draws cleanly | 24 frames, nothing said: true | yes |
| at 12, frames 12 to 19 show frames 12, 12, 14, 14, 16, 16, 18 and 18 byte for byte | [12, 12, 14, 14, 16, 16, 18, 18] | yes |
| at 8, frames 12 to 19 show frames 12, 12, 12, 15, 15, 15, 18 and 18 byte for byte | [12, 12, 12, 15, 15, 15, 18, 18] | yes |
| at Draft, 12 a second: frame 13 is frame 12 byte for byte, and frame 14 is not | 120 wide; 13 = 12, 14 ≠ 12 | yes |
| animated Noise after it holds with the drawing: frame 13 is frame 12 byte for byte and frame 14 is not, where the Noise alone changes every frame | 13 = 12, 14 ≠ 12, alone 13 ≠ 12 | yes |
| with drawing 13's file gone, frame 13, which holds frame 12 and so drawing 13, reports the missing file against frame 13, not frame 12 | frame 13 ["MEDIA_MISSING"], frame 12 [] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ptime_001.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ptime_008.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ptime_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ptime_016.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

116 of 116 checks pass.
