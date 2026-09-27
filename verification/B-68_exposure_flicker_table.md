# B-68: exposure flicker

D-125, accepted by the owner on 2026-09-26, the third of the second batch of ten. Every expected pixel is `Fixtures/exposure_flicker/expected_exposure_flicker.json`, written by `tools/exposure_flicker_reference.py` before this code existed and printed in document 25 as FX-FLICKER-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-FLICKER-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FLICKER-001 frame 0: The settings as they start, amount 0.25, hold 1, seed 0: every frame has its own brightness, every colour channel of every pixel times the same factor 2^(0.25 u), between 2^-0.25 and 2^0.25; with seed 0, frames 0 and 1 are brighter and frames 2, 3 and 4 darker. The covering is kept, the soft edge keeps its half covering, and the empty pixels stay empty. | largest difference 2.5e-7 | yes |
| FX-FLICKER-001 frame 1: The settings as they start, amount 0.25, hold 1, seed 0: every frame has its own brightness, every colour channel of every pixel times the same factor 2^(0.25 u), between 2^-0.25 and 2^0.25; with seed 0, frames 0 and 1 are brighter and frames 2, 3 and 4 darker. The covering is kept, the soft edge keeps its half covering, and the empty pixels stay empty. | largest difference 1.4e-7 | yes |
| FX-FLICKER-001 frame 2: The settings as they start, amount 0.25, hold 1, seed 0: every frame has its own brightness, every colour channel of every pixel times the same factor 2^(0.25 u), between 2^-0.25 and 2^0.25; with seed 0, frames 0 and 1 are brighter and frames 2, 3 and 4 darker. The covering is kept, the soft edge keeps its half covering, and the empty pixels stay empty. | largest difference 1.7e-7 | yes |
| FX-FLICKER-001 frame 3: The settings as they start, amount 0.25, hold 1, seed 0: every frame has its own brightness, every colour channel of every pixel times the same factor 2^(0.25 u), between 2^-0.25 and 2^0.25; with seed 0, frames 0 and 1 are brighter and frames 2, 3 and 4 darker. The covering is kept, the soft edge keeps its half covering, and the empty pixels stay empty. | largest difference 1.7e-7 | yes |
| FX-FLICKER-001 frame 4: The settings as they start, amount 0.25, hold 1, seed 0: every frame has its own brightness, every colour channel of every pixel times the same factor 2^(0.25 u), between 2^-0.25 and 2^0.25; with seed 0, frames 0 and 1 are brighter and frames 2, 3 and 4 darker. The covering is kept, the soft edge keeps its half covering, and the empty pixels stay empty. | largest difference 1.6e-7 | yes |
| FX-FLICKER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-002 frame 0: Amount 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-FLICKER-002 frame 2: Amount 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-FLICKER-002 frame 4: Amount 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-FLICKER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-003 frame 0: Amount 1: the same flicker four times as many stops, so each frame's factor is FX-FLICKER-001's to the fourth power. | largest difference 1.8e-7 | yes |
| FX-FLICKER-003 frame 1: Amount 1: the same flicker four times as many stops, so each frame's factor is FX-FLICKER-001's to the fourth power. | largest difference 3.3e-7 | yes |
| FX-FLICKER-003 frame 2: Amount 1: the same flicker four times as many stops, so each frame's factor is FX-FLICKER-001's to the fourth power. | largest difference 1.4e-7 | yes |
| FX-FLICKER-003 frame 3: Amount 1: the same flicker four times as many stops, so each frame's factor is FX-FLICKER-001's to the fourth power. | largest difference 9.6e-8 | yes |
| FX-FLICKER-003 frame 4: Amount 1: the same flicker four times as many stops, so each frame's factor is FX-FLICKER-001's to the fourth power. | largest difference 1.4e-7 | yes |
| FX-FLICKER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-004 frame 0: Amount 4, the top of its range: frame 1 is brightened by about 2.07 stops, so the white patch goes past its covering, as Exposure's rule does not clamp, and frame 3 is darkened by about 3.44 stops, near black; the black patch stays black. | largest difference 2.9e-7 | yes |
| FX-FLICKER-004 frame 1: Amount 4, the top of its range: frame 1 is brightened by about 2.07 stops, so the white patch goes past its covering, as Exposure's rule does not clamp, and frame 3 is darkened by about 3.44 stops, near black; the black patch stays black. | largest difference 6.1e-7 | yes |
| FX-FLICKER-004 frame 2: Amount 4, the top of its range: frame 1 is brightened by about 2.07 stops, so the white patch goes past its covering, as Exposure's rule does not clamp, and frame 3 is darkened by about 3.44 stops, near black; the black patch stays black. | largest difference 1.7e-7 | yes |
| FX-FLICKER-004 frame 3: Amount 4, the top of its range: frame 1 is brightened by about 2.07 stops, so the white patch goes past its covering, as Exposure's rule does not clamp, and frame 3 is darkened by about 3.44 stops, near black; the black patch stays black. | largest difference 3.0e-8 | yes |
| FX-FLICKER-004 frame 4: Amount 4, the top of its range: frame 1 is brightened by about 2.07 stops, so the white patch goes past its covering, as Exposure's rule does not clamp, and frame 3 is darkened by about 3.44 stops, near black; the black patch stays black. | largest difference 3.4e-8 | yes |
| FX-FLICKER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-005 frame 0: Hold 2: each brightness lasts two frames, so frames 0 and 1 are FX-FLICKER-001's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 2.5e-7 | yes |
| FX-FLICKER-005 frame 1: Hold 2: each brightness lasts two frames, so frames 0 and 1 are FX-FLICKER-001's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 2.5e-7 | yes |
| FX-FLICKER-005 frame 2: Hold 2: each brightness lasts two frames, so frames 0 and 1 are FX-FLICKER-001's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 1.4e-7 | yes |
| FX-FLICKER-005 frame 3: Hold 2: each brightness lasts two frames, so frames 0 and 1 are FX-FLICKER-001's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 1.4e-7 | yes |
| FX-FLICKER-005 frame 4: Hold 2: each brightness lasts two frames, so frames 0 and 1 are FX-FLICKER-001's frame 0, frames 2 and 3 its frame 1, and frame 4 its frame 2. | largest difference 1.7e-7 | yes |
| FX-FLICKER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-006 frame 0: Hold 3.7: the hold counts as its whole part, 3, so frames 0 to 2 are FX-FLICKER-001's frame 0 and frames 3 and 4 its frame 1. | largest difference 2.5e-7 | yes |
| FX-FLICKER-006 frame 1: Hold 3.7: the hold counts as its whole part, 3, so frames 0 to 2 are FX-FLICKER-001's frame 0 and frames 3 and 4 its frame 1. | largest difference 2.5e-7 | yes |
| FX-FLICKER-006 frame 2: Hold 3.7: the hold counts as its whole part, 3, so frames 0 to 2 are FX-FLICKER-001's frame 0 and frames 3 and 4 its frame 1. | largest difference 2.5e-7 | yes |
| FX-FLICKER-006 frame 3: Hold 3.7: the hold counts as its whole part, 3, so frames 0 to 2 are FX-FLICKER-001's frame 0 and frames 3 and 4 its frame 1. | largest difference 1.4e-7 | yes |
| FX-FLICKER-006 frame 4: Hold 3.7: the hold counts as its whole part, 3, so frames 0 to 2 are FX-FLICKER-001's frame 0 and frames 3 and 4 its frame 1. | largest difference 1.4e-7 | yes |
| FX-FLICKER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-007 frame 0: Hold 100, the top of its range: all five frames are FX-FLICKER-001's frame 0; the brightness is steady. | largest difference 2.5e-7 | yes |
| FX-FLICKER-007 frame 1: Hold 100, the top of its range: all five frames are FX-FLICKER-001's frame 0; the brightness is steady. | largest difference 2.5e-7 | yes |
| FX-FLICKER-007 frame 2: Hold 100, the top of its range: all five frames are FX-FLICKER-001's frame 0; the brightness is steady. | largest difference 2.5e-7 | yes |
| FX-FLICKER-007 frame 3: Hold 100, the top of its range: all five frames are FX-FLICKER-001's frame 0; the brightness is steady. | largest difference 2.5e-7 | yes |
| FX-FLICKER-007 frame 4: Hold 100, the top of its range: all five frames are FX-FLICKER-001's frame 0; the brightness is steady. | largest difference 2.5e-7 | yes |
| FX-FLICKER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-008 frame 0: Seed 7: a different flicker from FX-FLICKER-001's seed 0; frame 0 is brighter, frames 1 and 2 darker, frames 3 and 4 brighter. | largest difference 2.2e-7 | yes |
| FX-FLICKER-008 frame 1: Seed 7: a different flicker from FX-FLICKER-001's seed 0; frame 0 is brighter, frames 1 and 2 darker, frames 3 and 4 brighter. | largest difference 2.1e-7 | yes |
| FX-FLICKER-008 frame 2: Seed 7: a different flicker from FX-FLICKER-001's seed 0; frame 0 is brighter, frames 1 and 2 darker, frames 3 and 4 brighter. | largest difference 1.8e-7 | yes |
| FX-FLICKER-008 frame 3: Seed 7: a different flicker from FX-FLICKER-001's seed 0; frame 0 is brighter, frames 1 and 2 darker, frames 3 and 4 brighter. | largest difference 2.2e-7 | yes |
| FX-FLICKER-008 frame 4: Seed 7: a different flicker from FX-FLICKER-001's seed 0; frame 0 is brighter, frames 1 and 2 darker, frames 3 and 4 brighter. | largest difference 1.3e-7 | yes |
| FX-FLICKER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-009 frame 0: Seed 7.9: the seed counts as its whole part, so this is FX-FLICKER-008. | largest difference 2.2e-7 | yes |
| FX-FLICKER-009 frame 2: Seed 7.9: the seed counts as its whole part, so this is FX-FLICKER-008. | largest difference 1.8e-7 | yes |
| FX-FLICKER-009 frame 4: Seed 7.9: the seed counts as its whole part, so this is FX-FLICKER-008. | largest difference 1.3e-7 | yes |
| FX-FLICKER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-010 frame 0: Seed 100000, the top of its range: a flicker of its own. | largest difference 2.3e-7 | yes |
| FX-FLICKER-010 frame 1: Seed 100000, the top of its range: a flicker of its own. | largest difference 1.5e-7 | yes |
| FX-FLICKER-010 frame 2: Seed 100000, the top of its range: a flicker of its own. | largest difference 2.9e-7 | yes |
| FX-FLICKER-010 frame 3: Seed 100000, the top of its range: a flicker of its own. | largest difference 2.9e-7 | yes |
| FX-FLICKER-010 frame 4: Seed 100000, the top of its range: a flicker of its own. | largest difference 1.8e-7 | yes |
| FX-FLICKER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-011 frame 0: Amount keyed from 0 at frame 0 to 1 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 0.5 on its own frame, and frame 4 is FX-FLICKER-003's frame 4. | largest difference 1.9e-7 | yes |
| FX-FLICKER-011 frame 2: Amount keyed from 0 at frame 0 to 1 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 0.5 on its own frame, and frame 4 is FX-FLICKER-003's frame 4. | largest difference 2.0e-7 | yes |
| FX-FLICKER-011 frame 4: Amount keyed from 0 at frame 0 to 1 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 0.5 on its own frame, and frame 4 is FX-FLICKER-003's frame 4. | largest difference 1.4e-7 | yes |
| FX-FLICKER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-012 frame 0: Amount keyed from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frames 2 and 3 it would pass 4, is held at 4, and is FX-FLICKER-004's; frame 4, at 4, is FX-FLICKER-004's too. | largest difference 1.9e-7 | yes |
| FX-FLICKER-012 frame 2: Amount keyed from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frames 2 and 3 it would pass 4, is held at 4, and is FX-FLICKER-004's; frame 4, at 4, is FX-FLICKER-004's too. | largest difference 1.7e-7 | yes |
| FX-FLICKER-012 frame 3: Amount keyed from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frames 2 and 3 it would pass 4, is held at 4, and is FX-FLICKER-004's; frame 4, at 4, is FX-FLICKER-004's too. | largest difference 3.0e-8 | yes |
| FX-FLICKER-012 frame 4: Amount keyed from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frames 2 and 3 it would pass 4, is held at 4, and is FX-FLICKER-004's; frame 4, at 4, is FX-FLICKER-004's too. | largest difference 3.4e-8 | yes |
| FX-FLICKER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-013 frame 0: Hold keyed from 1 at frame 0 to 2 at frame 4, linear: at frames 2 and 3 the hold is 1.5 and 1.75, which count as 1, so they are FX-FLICKER-001's frames 2 and 3; at frame 4 the hold is 2, so frame 4 takes the brightness of frame 2, as FX-FLICKER-005's frame 4 does. | largest difference 2.5e-7 | yes |
| FX-FLICKER-013 frame 2: Hold keyed from 1 at frame 0 to 2 at frame 4, linear: at frames 2 and 3 the hold is 1.5 and 1.75, which count as 1, so they are FX-FLICKER-001's frames 2 and 3; at frame 4 the hold is 2, so frame 4 takes the brightness of frame 2, as FX-FLICKER-005's frame 4 does. | largest difference 1.7e-7 | yes |
| FX-FLICKER-013 frame 3: Hold keyed from 1 at frame 0 to 2 at frame 4, linear: at frames 2 and 3 the hold is 1.5 and 1.75, which count as 1, so they are FX-FLICKER-001's frames 2 and 3; at frame 4 the hold is 2, so frame 4 takes the brightness of frame 2, as FX-FLICKER-005's frame 4 does. | largest difference 1.7e-7 | yes |
| FX-FLICKER-013 frame 4: Hold keyed from 1 at frame 0 to 2 at frame 4, linear: at frames 2 and 3 the hold is 1.5 and 1.75, which count as 1, so they are FX-FLICKER-001's frames 2 and 3; at frame 4 the hold is 2, so frame 4 takes the brightness of frame 2, as FX-FLICKER-005's frame 4 does. | largest difference 1.7e-7 | yes |
| FX-FLICKER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-014 frame 0: Seed keyed from 0 at frame 0 to 7.5 at frame 4: frame 0 is FX-FLICKER-001's, and frame 4, at 7.5, counts as 7 and is FX-FLICKER-008's frame 4. | largest difference 2.5e-7 | yes |
| FX-FLICKER-014 frame 4: Seed keyed from 0 at frame 0 to 7.5 at frame 4: frame 0 is FX-FLICKER-001's, and frame 4, at 7.5, counts as 7 and is FX-FLICKER-008's frame 4. | largest difference 1.3e-7 | yes |
| FX-FLICKER-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-015 frame 0: FX-FLICKER-001 moved three pixels right: the flicker belongs to the frame, not the place, so each frame is FX-FLICKER-001's, moved. | largest difference 2.5e-7 | yes |
| FX-FLICKER-015 frame 1: FX-FLICKER-001 moved three pixels right: the flicker belongs to the frame, not the place, so each frame is FX-FLICKER-001's, moved. | largest difference 1.3e-7 | yes |
| FX-FLICKER-015 frame 2: FX-FLICKER-001 moved three pixels right: the flicker belongs to the frame, not the place, so each frame is FX-FLICKER-001's, moved. | largest difference 1.7e-7 | yes |
| FX-FLICKER-015 frame 3: FX-FLICKER-001 moved three pixels right: the flicker belongs to the frame, not the place, so each frame is FX-FLICKER-001's, moved. | largest difference 1.7e-7 | yes |
| FX-FLICKER-015 frame 4: FX-FLICKER-001 moved three pixels right: the flicker belongs to the frame, not the place, so each frame is FX-FLICKER-001's, moved. | largest difference 1.6e-7 | yes |
| FX-FLICKER-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLICKER-016 frame 0: Amount 5, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-016 frame 4: Amount 5, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLICKER-017 frame 0: Amount -0.25, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-017 frame 4: Amount -0.25, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLICKER-018 frame 0: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-018 frame 4: Hold 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLICKER-019 frame 0: Hold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-019 frame 4: Hold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLICKER-020 frame 0: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-020 frame 4: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLICKER-021 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-021 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLICKER-022 frame 0: Amount keyed to 6 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-022 frame 4: Amount keyed to 6 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FLICKER-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | ExposureFlicker { amount: 1.0, hold: 3.0, seed: 7.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flicker_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flicker_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `amount` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a hold that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 4.5 is refused with a sentence, and nothing changes | Exposure Flicker's amount runs from 0 to 4, and this is 4.5. | yes |
| amount -0.1 is refused with a sentence, and nothing changes | Exposure Flicker's amount runs from 0 to 4, and this is -0.1. | yes |
| hold 0.5 is refused with a sentence, and nothing changes | Exposure Flicker's hold runs from 1 to 100, and this is 0.5. | yes |
| hold 101 is refused with a sentence, and nothing changes | Exposure Flicker's hold runs from 1 to 100, and this is 101. | yes |
| seed -1 is refused with a sentence, and nothing changes | Exposure Flicker's seed runs from 0 to 100000, and this is -1. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Exposure Flicker's seed runs from 0 to 100000, and this is 100001. | yes |
| amount keyed to 6 is refused with a sentence, and nothing changes | Exposure Flicker's amount runs from 0 to 4, and this is 6. | yes |
| amount 4, hold 100 and seed 100000, the tops, is taken | taken | yes |
| amount 0, hold 1 and seed 0, the bottoms, is taken | taken | yes |
| hold keyed from 1 to 2 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flicker_001.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flicker_015.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

130 of 130 checks pass.
