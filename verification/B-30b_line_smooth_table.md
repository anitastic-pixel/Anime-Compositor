# B-30b: line smoothing

D-86, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/smooth/expected_smooth.json`, written by `tools/smooth_reference.py` before this code existed and printed in document 25 as FX-SMOOTH-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SMOOTH-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SMOOTH-001 frame 0: One step between white and black, softness 50: row 4 turns into an even slope across the whole row, and no other row changes. | largest difference 2.5e-7 | yes |
| FX-SMOOTH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-002 frame 0: The same at softness 0: the drawing, untouched. | largest difference 0.0e0 | yes |
| FX-SMOOTH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-003 frame 0: The same at softness 100: the slope falls half as fast, so it is still part of the way down at both ends of the row; row 4 only. | largest difference 9.1e-8 | yes |
| FX-SMOOTH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-004 frame 0: A one-pixel black line stepping down every three pixels: each step is softened, and a pixel two rows from the line is untouched. | largest difference 2.5e-7 | yes |
| FX-SMOOTH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-005 frame 0: A black box six by four: straight edges and corners of 4 or more, so nothing changes. | largest difference 0.0e0 | yes |
| FX-SMOOTH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-006 frame 0: A black box three by three: its corners are shorter than 4 and are rounded. | largest difference 3.4e-8 | yes |
| FX-SMOOTH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-007 frame 0: A red line on blue: every pixel is red, blue or a mix of the two, never darker. | largest difference 2.5e-7 | yes |
| FX-SMOOTH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-008 frame 0: A black line on nothing: the softened pixels are black, partly covering, with no grey or white fringe. | largest difference 4.6e-8 | yes |
| FX-SMOOTH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-009 frame 0: A red line on nothing: every pixel that shows is the line's red. | largest difference 2.2e-7 | yes |
| FX-SMOOTH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-010 frame 0: Two skin colours six apart, threshold 10: one colour to the rule, so nothing changes. | largest difference 1.8e-7 | yes |
| FX-SMOOTH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-011 frame 0: The same at threshold 0: two colours, and their step is softened. | largest difference 2.2e-7 | yes |
| FX-SMOOTH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-012 frame 0: Softness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SMOOTH-002, frame 2 is FX-SMOOTH-001, frame 4 is FX-SMOOTH-003. | largest difference 0.0e0 | yes |
| FX-SMOOTH-012 frame 2: Softness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SMOOTH-002, frame 2 is FX-SMOOTH-001, frame 4 is FX-SMOOTH-003. | largest difference 2.5e-7 | yes |
| FX-SMOOTH-012 frame 4: Softness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SMOOTH-002, frame 2 is FX-SMOOTH-001, frame 4 is FX-SMOOTH-003. | largest difference 9.1e-8 | yes |
| FX-SMOOTH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-013 frame 0: FX-SMOOTH-001 moved two pixels right: the same smoothed drawing, moved; the smoothing is done on the drawing's own pixels before it is moved. | largest difference 2.5e-7 | yes |
| FX-SMOOTH-013 frame 3: FX-SMOOTH-001 moved two pixels right: the same smoothed drawing, moved; the smoothing is done on the drawing's own pixels before it is moved. | largest difference 2.5e-7 | yes |
| FX-SMOOTH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMOOTH-020 frame 0: Softness 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SMOOTH-020 frame 4: Softness 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SMOOTH-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTH-021 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SMOOTH-021 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SMOOTH-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMOOTH-022 frame 0: Softness keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SMOOTH-022 frame 4: Softness keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-SMOOTH-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smooth_001.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_smooth_012.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_smooth_020.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_smooth_022.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| softness 101 is refused with a sentence, and nothing changes | Line smoothing's softness runs from 0 to 100 and its threshold from 0 to 255, and these are 101 and 10. | yes |
| softness -1 is refused with a sentence, and nothing changes | Line smoothing's softness runs from 0 to 100 and its threshold from 0 to 255, and these are -1 and 10. | yes |
| threshold 256 is refused with a sentence, and nothing changes | Line smoothing's softness runs from 0 to 100 and its threshold from 0 to 255, and these are 50 and 256. | yes |
| threshold -1 is refused with a sentence, and nothing changes | Line smoothing's softness runs from 0 to 100 and its threshold from 0 to 255, and these are 50 and -1. | yes |
| softness keyed to 120 is refused with a sentence, and nothing changes | Line smoothing's softness runs from 0 to 100 and its threshold from 0 to 255, and these are 120 and 10. | yes |
| softness 100 and threshold 255, the ends of the ranges, are taken | taken | yes |
| softness 0 and threshold 0, the ends of the ranges, are taken | taken | yes |
| undo twice: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smooth_004.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smooth_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## The packaged build carries OpenToonz's licence

| Check | The build's answer | Matches |
| --- | --- | --- |
| `tools/package.ps1` puts `docs/third_party/OpenToonz-LICENSE.txt` in the package | it does | yes |
| the port and the licence file carry OpenToonz's copyright notice | both | yes |

## Result

54 of 54 checks pass.
