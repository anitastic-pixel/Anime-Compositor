# B-81: threshold

D-138, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the fifth of the third batch. Every expected pixel is `Fixtures/threshold/expected_threshold.json`, written by `tools/threshold_reference.py` before this code existed and printed in document 25 as FX-THRESH-001 to 019. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-THRESH-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-THRESH-001 frame 0: Level 128, the setting as it starts: black, blue, the line, red, the dark grey and grey 127 turn black; grey 128, exactly on the level, turns white, as do the shadow skin, green, the light grey, the skin and white; the soft skin turns white and the soft line black, each at its own half covering. | largest difference 3.0e-8 | yes |
| FX-THRESH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-002 frame 0: Level 0: every shown pixel turns white, black too; the soft edges are white at half covering. | largest difference 3.0e-8 | yes |
| FX-THRESH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-003 frame 0: Level 255: only white stays white; every other shown pixel, the skin and the light grey too, turns black. | largest difference 3.0e-8 | yes |
| FX-THRESH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-004 frame 0: Level 20: only black and blue, the darkest, turn black; the line, darker to the eye than red but brighter than blue, turns white. | largest difference 3.0e-8 | yes |
| FX-THRESH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-005 frame 0: Level 50: red, at luma 54, turns white while blue and the line stay black: a pure red counts as brighter than a pure blue. | largest difference 3.0e-8 | yes |
| FX-THRESH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-006 frame 0: Level 64: the dark grey sits exactly on the level and turns white; red, just below, turns black. | largest difference 3.0e-8 | yes |
| FX-THRESH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-007 frame 0: Level 127: grey 127 is now exactly on the level and turns white, with grey 128. | largest difference 3.0e-8 | yes |
| FX-THRESH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-008 frame 0: Level 128.5, a level between two 8-bit steps: grey 128 turns black, with grey 127. | largest difference 3.0e-8 | yes |
| FX-THRESH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-009 frame 0: Level 172: the shadow skin, at luma 171.3, turns black while the lit skin stays white, the cel split into its light and its shadow. | largest difference 3.0e-8 | yes |
| FX-THRESH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-010 frame 0: Level 185: pure green, at luma 182.4, turns black; the light grey, at 192, stays white. | largest difference 3.0e-8 | yes |
| FX-THRESH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-011 frame 0: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 1: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 2: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 3: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 4: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-012 frame 0: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012 frame 2: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012 frame 3: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012 frame 4: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-013 frame 0: Level eased from 128 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the level has gone below 0 and is held there, so frames 2 and 4 are both FX-THRESH-002. | largest difference 3.0e-8 | yes |
| FX-THRESH-013 frame 2: Level eased from 128 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the level has gone below 0 and is held there, so frames 2 and 4 are both FX-THRESH-002. | largest difference 3.0e-8 | yes |
| FX-THRESH-013 frame 4: Level eased from 128 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the level has gone below 0 and is held there, so frames 2 and 4 are both FX-THRESH-002. | largest difference 3.0e-8 | yes |
| FX-THRESH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-014 frame 0: FX-THRESH-001 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-THRESH-014 frame 3: FX-THRESH-001 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-THRESH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-015 frame 0: Level -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-015 frame 4: Level -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-016 frame 0: Level 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-016 frame 4: Level 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-017 frame 0: Level 255.5, past 255 by half a step. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-017 frame 4: Level 255.5, past 255 by half a step. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-018 frame 0: Level keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-018 frame 4: Level keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-THRESH-019, in dispute (D-164, proposed)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-THRESH-019, in dispute (D-164, proposed): the build refuses fx_thresh_019.json as a fault in its shape, as it does a number written as a word in every effect; the case expects it kept with a warning | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | Threshold { level: 128.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_thresh_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_thresh_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `level` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| level -1 is refused with a sentence, and nothing changes | Threshold's level runs from 0 to 255, and this is -1. | yes |
| level 256 is refused with a sentence, and nothing changes | Threshold's level runs from 0 to 255, and this is 256. | yes |
| level 255.5 is refused with a sentence, and nothing changes | Threshold's level runs from 0 to 255, and this is 255.5. | yes |
| level keyed to 300 is refused with a sentence, and nothing changes | Threshold's level runs from 0 to 255, and this is 300. | yes |
| level 0, the bottom, is taken | taken | yes |
| level 255, the top, is taken | taken | yes |
| level keyed from 0 to 255 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_thresh_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_thresh_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_thresh_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

75 of 75 checks pass.
