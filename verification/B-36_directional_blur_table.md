# B-36: directional blur

D-92, accepted by the owner on 2026-09-25, read by D-98's lines since B-42 the same day. Every expected pixel is `Fixtures/directional_blur/expected_directional_blur.json`, written by `tools/directional_blur_reference.py` before this code existed and printed in document 25 as FX-DIRBLUR-001 to 015. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DIRBLUR-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DIRBLUR-001 frame 0: Direction 0, length 4: every edge streaks up and down, two pixels each way and faintly a third, and left and right stay sharp. | largest difference 1.4e-7 | yes |
| FX-DIRBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-002 frame 0: Direction 90, length 4: the same streak left and right, and up and down stay sharp. | largest difference 1.8e-7 | yes |
| FX-DIRBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-003 frame 0: Direction 270, the opposite way: the streak runs both ways, so this is FX-DIRBLUR-002. | largest difference 1.8e-7 | yes |
| FX-DIRBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-004 frame 0: Direction 45, length 6: a diagonal streak, up and right and down and left, between pixels. | largest difference 1.6e-7 | yes |
| FX-DIRBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-005 frame 0: Length 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-006 frame 0: Length 1, direction 90: each column mixes itself and the two beside it at a quarter, a half and a quarter, so the block's left edge column is three quarters covered and the column outside it one quarter. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-007 frame 0: Length 2.5, direction 90: a length that is not a whole number, the ends of the average softened. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-008 frame 0: Direction 3600, ten turns: this is FX-DIRBLUR-001. | largest difference 1.4e-7 | yes |
| FX-DIRBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-009 frame 0: Direction 90, length 6, moved three pixels right: the line on the drawing's left edge streaks three pixels past it into the grown border. | largest difference 1.5e-7 | yes |
| FX-DIRBLUR-009 frame 3: Direction 90, length 6, moved three pixels right: the line on the drawing's left edge streaks three pixels past it into the grown border. | largest difference 1.5e-7 | yes |
| FX-DIRBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-010 frame 0: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-010 frame 2: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8. | largest difference 1.8e-7 | yes |
| FX-DIRBLUR-010 frame 4: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8. | largest difference 1.4e-7 | yes |
| FX-DIRBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-011 frame 0: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002. | largest difference 1.4e-7 | yes |
| FX-DIRBLUR-011 frame 2: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-011 frame 4: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002. | largest difference 1.8e-7 | yes |
| FX-DIRBLUR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-012 frame 0: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-012 frame 4: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRBLUR-013 frame 0: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-013 frame 4: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRBLUR-014 frame 0: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-014 frame 4: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRBLUR-015 frame 0: Length keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-015 frame 4: Length keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| length 0 grows the drawing's bounds by 0, half, rounded up | 0 | yes |
| length 1 grows the drawing's bounds by 1, half, rounded up | 1 | yes |
| length 4 grows the drawing's bounds by 2, half, rounded up | 2 | yes |
| length 5 grows the drawing's bounds by 3, half, rounded up | 3 | yes |
| length 500 grows the drawing's bounds by 250, half, rounded up | 250 | yes |
| a half-size draft preview halves the length and keeps the direction | DirectionalBlur { direction: 90.0, length: 4.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dirblur_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dirblur_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `length` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a direction that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| length 501 is refused with a sentence, and nothing changes | Directional Blur's length runs from 0 to 500, and this is 501. | yes |
| length -1 is refused with a sentence, and nothing changes | Directional Blur's length runs from 0 to 500, and this is -1. | yes |
| direction 3601 is refused with a sentence, and nothing changes | Directional Blur's direction runs from -3600 to 3600, and this is 3601. | yes |
| direction -3601 is refused with a sentence, and nothing changes | Directional Blur's direction runs from -3600 to 3600, and this is -3601. | yes |
| length keyed to 600 is refused with a sentence, and nothing changes | Directional Blur's length runs from 0 to 500, and this is 600. | yes |
| length 500 and direction 3600, the top of the ranges, is taken | taken | yes |
| length 0 and direction -3600, the bottom, is taken | taken | yes |
| direction keyed from 0 to 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dirblur_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirblur_009.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

66 of 66 checks pass.
