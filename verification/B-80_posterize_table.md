# B-80: posterize

D-137, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the fourth of the third batch. Every expected pixel is `Fixtures/posterize/expected_posterize.json`, written by `tools/posterize_reference.py` before this code existed and printed in document 25 as FX-POSTER-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-POSTER-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-POSTER-001 frame 0: Levels 6, as it starts: every channel of every pixel that shows lands on one of six steps, 0, 51, 102, 153, 204 or 255 (encoded, of 255); the grey ramp turns to six flat greys, the greys 42 and 43 either side of a step landing on 0 and 51, 127 and 128 on 102 and 153, 212 and 213 on 204 and 255; black and white stay; a pixel at half covering takes the same steps as its colour at full covering, at its own covering; the empty pixels stay empty. | largest difference 3.0e-8 | yes |
| FX-POSTER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-002 frame 0: Levels 2, the fewest: every channel is either 0 or full, so every pixel is one of eight colours; the grey 127 turns black and 128 white, the skin and its shadow white, the line black, the red #ff0000 and the blue #0000ff. | largest difference 3.0e-8 | yes |
| FX-POSTER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-003 frame 0: Levels 3: steps 0, 127.5 and 255; the greys 85 and 170 sit exactly on the steps and are taken up to 127.5 and 255, as the rule's 1e-4 decides, while 64 falls to 0, 128 lands on 127.5 and 212 on 255. | largest difference 3.0e-8 | yes |
| FX-POSTER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-004 frame 0: Levels 16: sixteen steps 17 apart; every channel lands on a step within one step of its own value, and the ramps show their bands close together. | largest difference 3.0e-8 | yes |
| FX-POSTER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-005 frame 0: Levels 256, the most: every 8-bit channel is already on a step, so the drawing is untouched, exactly. | largest difference 3.0e-8 | yes |
| FX-POSTER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-006 frame 0: Levels 255: 255 steps, one fewer than an 8-bit value has, so every channel between 0 and 255 is lifted a little, by at most one 8-bit step (v becomes 255 v / 254); black and white stay. | largest difference 3.0e-8 | yes |
| FX-POSTER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-007 frame 0: Levels 6.9: only the whole part counts, so it is FX-POSTER-001. | largest difference 3.0e-8 | yes |
| FX-POSTER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-008 frame 0: Levels keyed from 2 at frame 0 to 10 at frame 4, linear: frame 0 is FX-POSTER-002, frame 2, at 6, is FX-POSTER-001, and frame 4 has ten steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-008 frame 2: Levels keyed from 2 at frame 0 to 10 at frame 4, linear: frame 0 is FX-POSTER-002, frame 2, at 6, is FX-POSTER-001, and frame 4 has ten steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-008 frame 4: Levels keyed from 2 at frame 0 to 10 at frame 4, linear: frame 0 is FX-POSTER-002, frame 2, at 6, is FX-POSTER-001, and frame 4 has ten steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-009 frame 0: Levels keyed from 4 at frame 0 to 7 at frame 4, linear, every frame: 4.75 at frame 1 counts as 4, the same as frame 0; 5.5 at frame 2 counts as 5; 6.25 at frame 3 counts as 6, FX-POSTER-001; frame 4 has seven steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-009 frame 1: Levels keyed from 4 at frame 0 to 7 at frame 4, linear, every frame: 4.75 at frame 1 counts as 4, the same as frame 0; 5.5 at frame 2 counts as 5; 6.25 at frame 3 counts as 6, FX-POSTER-001; frame 4 has seven steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-009 frame 2: Levels keyed from 4 at frame 0 to 7 at frame 4, linear, every frame: 4.75 at frame 1 counts as 4, the same as frame 0; 5.5 at frame 2 counts as 5; 6.25 at frame 3 counts as 6, FX-POSTER-001; frame 4 has seven steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-009 frame 3: Levels keyed from 4 at frame 0 to 7 at frame 4, linear, every frame: 4.75 at frame 1 counts as 4, the same as frame 0; 5.5 at frame 2 counts as 5; 6.25 at frame 3 counts as 6, FX-POSTER-001; frame 4 has seven steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-009 frame 4: Levels keyed from 4 at frame 0 to 7 at frame 4, linear, every frame: 4.75 at frame 1 counts as 4, the same as frame 0; 5.5 at frame 2 counts as 5; 6.25 at frame 3 counts as 6, FX-POSTER-001; frame 4 has seven steps. | largest difference 3.0e-8 | yes |
| FX-POSTER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-010 frame 0: Levels eased from 256 at frame 0 to 2 at frame 4 on a curve that overshoots: frame 0 is the drawing; at frame 2 the levels have gone past 2 (to -80.55) and are held at 2, so frames 2 and 4 are both FX-POSTER-002. | largest difference 3.0e-8 | yes |
| FX-POSTER-010 frame 2: Levels eased from 256 at frame 0 to 2 at frame 4 on a curve that overshoots: frame 0 is the drawing; at frame 2 the levels have gone past 2 (to -80.55) and are held at 2, so frames 2 and 4 are both FX-POSTER-002. | largest difference 3.0e-8 | yes |
| FX-POSTER-010 frame 4: Levels eased from 256 at frame 0 to 2 at frame 4 on a curve that overshoots: frame 0 is the drawing; at frame 2 the levels have gone past 2 (to -80.55) and are held at 2, so frames 2 and 4 are both FX-POSTER-002. | largest difference 3.0e-8 | yes |
| FX-POSTER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-011 frame 0: Levels 3 held (a hold key) at frame 0, then 16 at frame 4: frames 0 and 2 are FX-POSTER-003 and frame 4 is FX-POSTER-004. | largest difference 3.0e-8 | yes |
| FX-POSTER-011 frame 2: Levels 3 held (a hold key) at frame 0, then 16 at frame 4: frames 0 and 2 are FX-POSTER-003 and frame 4 is FX-POSTER-004. | largest difference 3.0e-8 | yes |
| FX-POSTER-011 frame 4: Levels 3 held (a hold key) at frame 0, then 16 at frame 4: frames 0 and 2 are FX-POSTER-003 and frame 4 is FX-POSTER-004. | largest difference 3.0e-8 | yes |
| FX-POSTER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-012 frame 0: FX-POSTER-001 moved three pixels right: the same, moved. | largest difference 1.8e-8 | yes |
| FX-POSTER-012 frame 3: FX-POSTER-001 moved three pixels right: the same, moved. | largest difference 1.8e-8 | yes |
| FX-POSTER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POSTER-013 frame 0: Levels 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-013 frame 4: Levels 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POSTER-014 frame 0: Levels 257, above 256. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-014 frame 4: Levels 257, above 256. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POSTER-015 frame 0: Levels 256.5, above 256: the range is on the number as written, before its whole part is taken. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-015 frame 4: Levels 256.5, above 256: the range is on the number as written, before its whole part is taken. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POSTER-016 frame 0: Levels -6, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-016 frame 4: Levels -6, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POSTER-017 frame 0: Levels keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-017 frame 4: Levels keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POSTER-018 frame 0: Levels keyed from 0 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-018 frame 4: Levels keyed from 0 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POSTER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | Posterize { levels: 6.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_poster_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_poster_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `levels` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with levels that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| levels 1 is refused with a sentence, and nothing changes | Posterize's levels runs from 2 to 256, and this is 1. | yes |
| levels 257 is refused with a sentence, and nothing changes | Posterize's levels runs from 2 to 256, and this is 257. | yes |
| levels 256.5 is refused with a sentence, and nothing changes | Posterize's levels runs from 2 to 256, and this is 256.5. | yes |
| levels keyed to 300 is refused with a sentence, and nothing changes | Posterize's levels runs from 2 to 256, and this is 300. | yes |
| levels 2, the bottom, is taken | taken | yes |
| levels 256, the top, is taken | taken | yes |
| levels keyed from 2 to 10 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_poster_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_poster_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_poster_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

78 of 78 checks pass.
