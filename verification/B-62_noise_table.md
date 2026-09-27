# B-62: noise

D-119, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/noise/expected_noise.json`, written by `tools/noise_reference.py` before this code existed and printed in document 25 as FX-NOISE-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-NOISE-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NOISE-001 frame 0: The defaults, amount 10, mono, seed 0, animated: every shown pixel's three channels move together by up to 0.05 through the sRGB curve, and the grain is different on frames 0, 2 and 4; the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 2.1e-7 | yes |
| FX-NOISE-001 frame 2: The defaults, amount 10, mono, seed 0, animated: every shown pixel's three channels move together by up to 0.05 through the sRGB curve, and the grain is different on frames 0, 2 and 4; the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 2.2e-7 | yes |
| FX-NOISE-001 frame 4: The defaults, amount 10, mono, seed 0, animated: every shown pixel's three channels move together by up to 0.05 through the sRGB curve, and the grain is different on frames 0, 2 and 4; the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 2.2e-7 | yes |
| FX-NOISE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-002 frame 0: Animate off: the grain is the same on frames 0, 2 and 4, and is FX-NOISE-001's frame 0. | largest difference 2.1e-7 | yes |
| FX-NOISE-002 frame 2: Animate off: the grain is the same on frames 0, 2 and 4, and is FX-NOISE-001's frame 0. | largest difference 2.1e-7 | yes |
| FX-NOISE-002 frame 4: Animate off: the grain is the same on frames 0, 2 and 4, and is FX-NOISE-001's frame 0. | largest difference 2.1e-7 | yes |
| FX-NOISE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-003 frame 0: Mode color: the three channels move each their own way; the grey card turns speckled with colour. | largest difference 2.1e-7 | yes |
| FX-NOISE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-004 frame 0: Seed 7: a different grain from FX-NOISE-002's seed 0. | largest difference 2.2e-7 | yes |
| FX-NOISE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-005 frame 0: Seed 3. | largest difference 2.1e-7 | yes |
| FX-NOISE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-006 frame 0: Seed 3.7: the seed counts as its whole part, so this is FX-NOISE-005. | largest difference 2.1e-7 | yes |
| FX-NOISE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-007 frame 0: Seed 100000, the top of its range: a grain of its own. | largest difference 2.2e-7 | yes |
| FX-NOISE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-008 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NOISE-008 frame 2: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NOISE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-009 frame 0: Amount 100, color: channels move by up to 0.5; the white patch can only darken, so where the grain would lighten it the channel stays exactly white, and the black patch stays exactly black where it would darken. | largest difference 2.0e-7 | yes |
| FX-NOISE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-010 frame 0: Amount keyed from 0 at frame 0 to 40 at frame 4, animate off: frame 0 is the drawing; frame 4's grain is frame 2's, twice as strong through the sRGB curve. | largest difference 1.9e-7 | yes |
| FX-NOISE-010 frame 2: Amount keyed from 0 at frame 0 to 40 at frame 4, animate off: frame 0 is the drawing; frame 4's grain is frame 2's, twice as strong through the sRGB curve. | largest difference 2.2e-7 | yes |
| FX-NOISE-010 frame 4: Amount keyed from 0 at frame 0 to 40 at frame 4, animate off: frame 0 is the drawing; frame 4's grain is frame 2's, twice as strong through the sRGB curve. | largest difference 2.1e-7 | yes |
| FX-NOISE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-011 frame 0: Seed keyed from 0 at frame 0 to 3.5 at frame 4, animate off: frame 0 is FX-NOISE-002, and frame 4, at 3.5, counts as 3 and is FX-NOISE-005. | largest difference 2.1e-7 | yes |
| FX-NOISE-011 frame 4: Seed keyed from 0 at frame 0 to 3.5 at frame 4, animate off: frame 0 is FX-NOISE-002, and frame 4, at 3.5, counts as 3 and is FX-NOISE-005. | largest difference 2.1e-7 | yes |
| FX-NOISE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-012 frame 0: FX-NOISE-001 moved three pixels right: the grain is worked in the drawing's own space, so it moves with the drawing, frame by frame; each frame is FX-NOISE-001's, moved. | largest difference 2.1e-7 | yes |
| FX-NOISE-012 frame 2: FX-NOISE-001 moved three pixels right: the grain is worked in the drawing's own space, so it moves with the drawing, frame by frame; each frame is FX-NOISE-001's, moved. | largest difference 2.2e-7 | yes |
| FX-NOISE-012 frame 4: FX-NOISE-001 moved three pixels right: the grain is worked in the drawing's own space, so it moves with the drawing, frame by frame; each frame is FX-NOISE-001's, moved. | largest difference 2.2e-7 | yes |
| FX-NOISE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISE-013 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-013 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISE-014 frame 0: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-014 frame 4: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISE-015 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-015 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISE-016 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-016 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISE-017 frame 0: Mode "Mono", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-017 frame 4: Mode "Mono", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISE-018 frame 0: Animate "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-018 frame 4: Animate "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: only pixels that show change | 0 | yes |
| a half-size draft preview changes none of its settings: the grain is a pixel wide | Noise { amount: 100.0, mode: "color", seed: 7.0, animate: "on", frame: 0 } | yes |

## The grain's frame

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noise_001.json: animated, the grain at frame 3 is frame 3's | 3 | yes |
| fx_noise_002.json: not animated, the grain at frame 3 is frame 0's | 0 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noise_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noise_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `animate` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an amount written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Noise's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Noise's amount runs from 0 to 100, and this is -1. | yes |
| seed -1 is refused with a sentence, and nothing changes | Noise's seed runs from 0 to 100000, and this is -1. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Noise's seed runs from 0 to 100000, and this is 100001. | yes |
| mode "Mono" is refused with a sentence, and nothing changes | Noise's mode is "mono" or "color", and this is "Mono". | yes |
| animate "yes" is refused with a sentence, and nothing changes | Noise's animate is "on" or "off", and this is "yes". | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Noise's amount runs from 0 to 100, and this is 150. | yes |
| amount 100 and seed 100000, the tops, in colour and still, is taken | taken | yes |
| amount 0 and seed 0, the bottoms, is taken | taken | yes |
| seed keyed from 0 to 3.5 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noise_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noise_012.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## Result

83 of 83 checks pass.
