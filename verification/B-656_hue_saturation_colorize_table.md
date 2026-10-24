# B-656: Hue/Saturation's Colorize

D-776, accepted on 2026-10-24. Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation.json`, written by `tools/hue_saturation_reference.py` before this code existed and printed in document 25 as FX-HUESAT-027 to 036. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-HUESAT-027 to 036 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HUESAT-027 frame 0: Colorize on at After Effects' defaults, hue 0, saturation 25, lightness 0 (Jake In Motion, 2:24): every shown colour turns a dull red at its own HSL lightness, red, green, blue and magenta alike at 0.5; the skin a pale pink, the line a dark one; the soft edge keeps its half covering and the empty pixels stay empty. | largest difference 1.9e-7 | yes |
| FX-HUESAT-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-028 frame 0: Colorize on, hue 180, saturation 70, lightness -4 (Jake In Motion, 2:42): every colour a strong cyan, each a little darker than its own lightness. | largest difference 2.0e-7 | yes |
| FX-HUESAT-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-029 frame 0: Colorize on, hue 80, saturation 25 (Jake In Motion, 2:30), with the master hue 120, saturation -50 and lightness 30 and the greens moved too: the master and the ranges are not used, so this is Colorize alone. | largest difference 1.9e-7 | yes |
| FX-HUESAT-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-030 frame 0: Colorize on, hue 240, saturation 100, lightness +50: a full blue, every lightness half way to white; red's 0.5 goes to 0.75, #8080ff. | largest difference 1.3e-7 | yes |
| FX-HUESAT-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-031 frame 0: Colorize on, its hue keyed from 0 at frame 0 to 360 at frame 4, linear, saturation 100: frames 0 and 4 the same full red tint, frame 2 cyan. | largest difference 2.3e-7 | yes |
| FX-HUESAT-031 frame 2: Colorize on, its hue keyed from 0 at frame 0 to 360 at frame 4, linear, saturation 100: frames 0 and 4 the same full red tint, frame 2 cyan. | largest difference 2.3e-7 | yes |
| FX-HUESAT-031 frame 4: Colorize on, its hue keyed from 0 at frame 0 to 360 at frame 4, linear, saturation 100: frames 0 and 4 the same full red tint, frame 2 cyan. | largest difference 2.3e-7 | yes |
| FX-HUESAT-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-032 frame 0: Colorize written "off" with its hue 200 and saturation 90, and the master hue +120: Colorize is not used, so this is FX-HUESAT-002. | largest difference 1.9e-7 | yes |
| FX-HUESAT-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-033 frame 0: Colorize "yes", neither "on" nor "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-033 frame 4: Colorize "yes", neither "on" nor "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-034 frame 0: Colorize saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-034 frame 4: Colorize saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-035 frame 0: Colorize lightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-035 frame 4: Colorize lightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-036 frame 0: Colorize hue 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-036 frame 4: Colorize hue 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_huesat_001.json, a file from before Colorize, opens with it off and is saved without its words | {"hue":0,"lightness":0,"saturation":0} | yes |
| fx_huesat_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a colorize hue written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| colorize "yes" is refused with a sentence, and nothing changes | Hue/Saturation's colorize is "on" or "off", and this is "yes". | yes |
| colorize hue 361 is refused with a sentence, and nothing changes | Hue/Saturation's colorize hue runs from 0 to 360, and this is 361. | yes |
| colorize hue -1 is refused with a sentence, and nothing changes | Hue/Saturation's colorize hue runs from 0 to 360, and this is -1. | yes |
| colorize saturation 101 is refused with a sentence, and nothing changes | Hue/Saturation's colorize saturation runs from 0 to 100, and this is 101. | yes |
| colorize saturation -1 is refused with a sentence, and nothing changes | Hue/Saturation's colorize saturation runs from 0 to 100, and this is -1. | yes |
| colorize lightness 101 is refused with a sentence, and nothing changes | Hue/Saturation's colorize lightness runs from -100 to 100, and this is 101. | yes |
| colorize lightness -101 is refused with a sentence, and nothing changes | Hue/Saturation's colorize lightness runs from -100 to 100, and this is -101. | yes |
| colorize hue keyed to 400 is refused with a sentence, and nothing changes | Hue/Saturation's colorize hue runs from 0 to 360, and this is 400. | yes |
| colorize on, hue 360, saturation 100 and lightness 100, the tops of the ranges, is taken | taken | yes |
| colorize on, hue 0, saturation 0 and lightness -100, the bottoms, is taken | taken | yes |
| colorize hue keyed from 0 to 360 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_huesat_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_huesat_031.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

52 of 52 checks pass.
