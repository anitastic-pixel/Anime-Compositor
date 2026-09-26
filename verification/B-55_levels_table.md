# B-55: levels

D-112, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/levels/expected_levels.json`, written by `tools/levels_reference.py` before this code existed and printed in document 25 as FX-LEVELS-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LEVELS-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LEVELS-001 frame 0: Every setting its default, 0, 255, gamma 1, 0, 255: the drawing, within the tolerance. | largest difference 1.9e-7 | yes |
| FX-LEVELS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-002 frame 0: Input black 64: the ramp's greys at or below 64 become black, and every colour darkens toward it; white stays. | largest difference 2.2e-7 | yes |
| FX-LEVELS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-003 frame 0: Input white 192: the ramp's greys at or above 192 become white, and every colour lightens toward it; black stays. | largest difference 2.5e-7 | yes |
| FX-LEVELS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-004 frame 0: Gamma 2: the middle lightens, the ramp's 128 to about 180; black and white stay. | largest difference 1.1e-7 | yes |
| FX-LEVELS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-005 frame 0: Gamma 0.5: the middle darkens, the ramp's 128 to about 64; black and white stay. | largest difference 3.6e-7 | yes |
| FX-LEVELS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-006 frame 0: Output black 64: black becomes 64, every colour rises toward white a little; white stays. | largest difference 1.6e-7 | yes |
| FX-LEVELS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-007 frame 0: Output white 192: white becomes 192, every colour falls toward it; black stays. | largest difference 9.2e-8 | yes |
| FX-LEVELS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-008 frame 0: All five at once: input 32 to 224, gamma 1.5, output 16 to 240. | largest difference 1.1e-7 | yes |
| FX-LEVELS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-009 frame 0: Input black 255 and input white 0, the white below the black: every colour is inverted; the empty pixels stay empty. | largest difference 5.2e-8 | yes |
| FX-LEVELS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-010 frame 0: Output black 255 and output white 0: every colour is inverted, the same as FX-LEVELS-009. | largest difference 5.2e-8 | yes |
| FX-LEVELS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-011 frame 0: Input black and white both 120: a threshold; each channel at or above 120 becomes 255 and each below becomes 0. | largest difference 3.0e-8 | yes |
| FX-LEVELS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-012 frame 0: Output black and white both 255: every pixel that shows becomes white at its own covering, the soft ones still soft; the empty pixels stay empty. | largest difference 3.0e-8 | yes |
| FX-LEVELS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-013 frame 0: Gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is gamma 2, FX-LEVELS-004, and frame 4 is gamma 3. | largest difference 1.9e-7 | yes |
| FX-LEVELS-013 frame 2: Gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is gamma 2, FX-LEVELS-004, and frame 4 is gamma 3. | largest difference 1.1e-7 | yes |
| FX-LEVELS-013 frame 4: Gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is gamma 2, FX-LEVELS-004, and frame 4 is gamma 3. | largest difference 1.0e-7 | yes |
| FX-LEVELS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-014 frame 0: Gamma keyed from 1 at frame 0 to 10 at frame 4 on an ease that runs past its end: frame 2 works out above 10 and is held at 10, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-LEVELS-014 frame 2: Gamma keyed from 1 at frame 0 to 10 at frame 4 on an ease that runs past its end: frame 2 works out above 10 and is held at 10, the same as frame 4. | largest difference 6.1e-8 | yes |
| FX-LEVELS-014 frame 4: Gamma keyed from 1 at frame 0 to 10 at frame 4 on an ease that runs past its end: frame 2 works out above 10 and is held at 10, the same as frame 4. | largest difference 6.1e-8 | yes |
| FX-LEVELS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-015 frame 0: Output white keyed from 255 at frame 0 to 128 at frame 4, linear: white falls to 191.5 at frame 2 and to 128 at frame 4. | largest difference 1.9e-7 | yes |
| FX-LEVELS-015 frame 2: Output white keyed from 255 at frame 0 to 128 at frame 4, linear: white falls to 191.5 at frame 2 and to 128 at frame 4. | largest difference 1.0e-7 | yes |
| FX-LEVELS-015 frame 4: Output white keyed from 255 at frame 0 to 128 at frame 4, linear: white falls to 191.5 at frame 2 and to 128 at frame 4. | largest difference 4.1e-8 | yes |
| FX-LEVELS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-016 frame 0: Gamma 0.1, the least: everything but white falls almost to black. | largest difference 8.8e-7 | yes |
| FX-LEVELS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-017 frame 0: FX-LEVELS-008 moved three pixels right: the same, moved. | largest difference 1.1e-7 | yes |
| FX-LEVELS-017 frame 3: FX-LEVELS-008 moved three pixels right: the same, moved. | largest difference 1.1e-7 | yes |
| FX-LEVELS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LEVELS-018 frame 0: Input black 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-018 frame 4: Input black 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEVELS-019 frame 0: Input white -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-019 frame 4: Input white -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEVELS-020 frame 0: Gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-020 frame 4: Gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEVELS-021 frame 0: Gamma 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-021 frame 4: Gamma 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEVELS-022 frame 0: Output black -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-022 frame 4: Output black -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEVELS-023 frame 0: Output white 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-023 frame 4: Output white 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LEVELS-024 frame 0: Input black keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-024 frame 4: Input black keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LEVELS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is regraded where it is | 0 | yes |
| a half-size draft preview changes nothing: levels are colours, not distances | Levels { input_black: 32.0, input_white: 224.0, gamma: 1.5, output_black: 16.0, output_white: 240.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_levels_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_levels_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `gamma` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an output white written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| input black 256 is refused with a sentence, and nothing changes | Levels's input black runs from 0 to 255, and this is 256. | yes |
| input white -1 is refused with a sentence, and nothing changes | Levels's input white runs from 0 to 255, and this is -1. | yes |
| gamma 0.05 is refused with a sentence, and nothing changes | Levels's gamma runs from 0.1 to 10, and this is 0.05. | yes |
| gamma 11 is refused with a sentence, and nothing changes | Levels's gamma runs from 0.1 to 10, and this is 11. | yes |
| output black -1 is refused with a sentence, and nothing changes | Levels's output black runs from 0 to 255, and this is -1. | yes |
| output white 256 is refused with a sentence, and nothing changes | Levels's output white runs from 0 to 255, and this is 256. | yes |
| input black keyed to 300 is refused with a sentence, and nothing changes | Levels's input black runs from 0 to 255, and this is 300. | yes |
| input 255 to 0 and output 255 to 0 with gamma 10, the ends of the ranges, is taken | taken | yes |
| input black and white both 120, a threshold, is taken | taken | yes |
| gamma keyed from 1 to 3 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_levels_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_levels_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

88 of 88 checks pass.
