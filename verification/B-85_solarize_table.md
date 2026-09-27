# B-85: solarize

D-142, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the ninth of the third batch. Every expected pixel is `Fixtures/solarize/expected_solarize.json`, written by `tools/solarize_reference.py` before this code existed and printed in document 25 as FX-SOLAR-001 to 016. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SOLAR-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SOLAR-001 frame 0: Threshold 128, as it starts: every channel at 128 or above is turned over. The grey #808080, on the threshold, turns #7f7f7f, while #7f7f7f beside it, one below, stays; the light grey turns #3f3f3f and the white black; the skin turns a deep blue #092941 (red, green and blue all above 128); the red trace keeps its green and blue and its red falls to 55; the night blue keeps its red and green and its blue falls to 101; black, the line and the dark grey, all below, stay as they are. | largest difference 3.3e-8 | yes |
| FX-SOLAR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-002 frame 0: Threshold 0: every channel of every shown pixel is turned over, as Invert does: black turns white, white black, the skin #092941. | largest difference 4.7e-8 | yes |
| FX-SOLAR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-003 frame 0: Threshold 255: only a channel already at full is turned over, so the white turns black and every other pixel stays as it is. | largest difference 1.9e-7 | yes |
| FX-SOLAR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-004 frame 0: Threshold 1: every channel above 0 is turned over, so black, all at 0, stays black, while every other shown pixel is inverted as in FX-SOLAR-002. | largest difference 4.7e-8 | yes |
| FX-SOLAR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-005 frame 0: Threshold 64: the dark grey, on the threshold, turns #bfbfbf; the line, below it, stays; the night blue keeps its red 58 and turns over its green and blue, to #3aa565. | largest difference 3.3e-8 | yes |
| FX-SOLAR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-006 frame 0: Threshold 129: the grey #808080 and its soft edge, now one below, stay as they are; the light grey, the skin and the white are turned over as in FX-SOLAR-001. | largest difference 3.3e-8 | yes |
| FX-SOLAR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-007 frame 0: Threshold 200: the skin turns over its red and green and keeps its blue 190, to #0929be, a blue-purple; the red trace's red, on the threshold, falls to 55; the light grey, 192, stays; the white turns black. | largest difference 1.3e-7 | yes |
| FX-SOLAR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-008 frame 0: Threshold 127.5, between two 8-bit values: the same as 128, FX-SOLAR-001, as no written value lies between them. | largest difference 3.3e-8 | yes |
| FX-SOLAR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-009 frame 0: Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-SOLAR-002, frame 2 (127.5) FX-SOLAR-001 and frame 4 FX-SOLAR-003. | largest difference 4.7e-8 | yes |
| FX-SOLAR-009 frame 2: Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-SOLAR-002, frame 2 (127.5) FX-SOLAR-001 and frame 4 FX-SOLAR-003. | largest difference 3.3e-8 | yes |
| FX-SOLAR-009 frame 4: Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-SOLAR-002, frame 2 (127.5) FX-SOLAR-001 and frame 4 FX-SOLAR-003. | largest difference 1.9e-7 | yes |
| FX-SOLAR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-010 frame 0: Threshold keyed 64 at frame 0, held, then 200 at frame 4: frames 0 and 2 are FX-SOLAR-005 and frame 4 FX-SOLAR-007. | largest difference 3.3e-8 | yes |
| FX-SOLAR-010 frame 2: Threshold keyed 64 at frame 0, held, then 200 at frame 4: frames 0 and 2 are FX-SOLAR-005 and frame 4 FX-SOLAR-007. | largest difference 3.3e-8 | yes |
| FX-SOLAR-010 frame 4: Threshold keyed 64 at frame 0, held, then 200 at frame 4: frames 0 and 2 are FX-SOLAR-005 and frame 4 FX-SOLAR-007. | largest difference 1.3e-7 | yes |
| FX-SOLAR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-011 frame 0: Threshold eased from 128 at frame 0 to 255 at frame 4 on a curve that overshoots: at frame 2 it has gone past 255 and is held there, so frames 2 and 4 are both FX-SOLAR-003. | largest difference 3.3e-8 | yes |
| FX-SOLAR-011 frame 2: Threshold eased from 128 at frame 0 to 255 at frame 4 on a curve that overshoots: at frame 2 it has gone past 255 and is held there, so frames 2 and 4 are both FX-SOLAR-003. | largest difference 1.9e-7 | yes |
| FX-SOLAR-011 frame 4: Threshold eased from 128 at frame 0 to 255 at frame 4 on a curve that overshoots: at frame 2 it has gone past 255 and is held there, so frames 2 and 4 are both FX-SOLAR-003. | largest difference 1.9e-7 | yes |
| FX-SOLAR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-012 frame 0: FX-SOLAR-001 moved three pixels right: the same, moved. | largest difference 3.3e-8 | yes |
| FX-SOLAR-012 frame 3: FX-SOLAR-001 moved three pixels right: the same, moved. | largest difference 3.3e-8 | yes |
| FX-SOLAR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLAR-013 frame 0: Threshold -1: below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-013 frame 4: Threshold -1: below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLAR-014 frame 0: Threshold 255.5: above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-014 frame 4: Threshold 255.5: above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLAR-015 frame 0: Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-015 frame 4: Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLAR-016 frame 0: Threshold keyed from -50 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-016 frame 4: Threshold keyed from -50 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLAR-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | Solarize { threshold: 128.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_solar_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solar_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `threshold` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a threshold that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| threshold -1 is refused with a sentence, and nothing changes | Solarize's threshold runs from 0 to 255, and this is -1. | yes |
| threshold 255.5 is refused with a sentence, and nothing changes | Solarize's threshold runs from 0 to 255, and this is 255.5. | yes |
| threshold keyed to 300 is refused with a sentence, and nothing changes | Solarize's threshold runs from 0 to 255, and this is 300. | yes |
| threshold 0, the bottom, is taken | taken | yes |
| threshold 255, the top, is taken | taken | yes |
| threshold keyed from 0 to 255 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_solar_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solar_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solar_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

67 of 67 checks pass.
