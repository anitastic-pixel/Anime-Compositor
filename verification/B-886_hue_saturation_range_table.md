# B-886: Hue/Saturation's Channel Range

D-1006, accepted on 2026-10-27. Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation_range.json`, written by `tools/hue_saturation_range_reference.py` before this code existed and printed in document 25 as FX-HUESAT-037 to 047. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance.

## FX-HUESAT-037 to 047 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HUESAT-037 frame 0: All six Channel Ranges written at After Effects' own (reds 315, 345, 15, 45 and so on round the wheel) with the greens hue +90, saturation -40, lightness +10: the same as FX-HUESAT-024, which leaves them out. | largest difference 1.5e-7 | yes |
| FX-HUESAT-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-038 frame 0: Reds widened to 275, 320, 35, 85, saturation -100: red (0), orange (30) and rose (330) turn fully grey; magenta (300) and yellow (60) lose half their colour; yellow-green (90) to violet (270) stay. | largest difference 1.5e-7 | yes |
| FX-HUESAT-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-039 frame 0: Yellows narrowed to 50, 55, 65, 70, saturation -100: only yellow (60) turns grey; orange (30) and yellow-green (90), half-taken at the default, now stay. | largest difference 1.5e-7 | yes |
| FX-HUESAT-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-040 frame 0: Blues 180, 240, 240, 300, a full range of one hue, lightness -50: blue (240) darkens fully; azure (210) and violet (270) half; cyan (180) and magenta (300) stay. | largest difference 1.5e-7 | yes |
| FX-HUESAT-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-041 frame 0: Greens 85, 85, 155, 155, no fade, hue +60: yellow-green (90), green (120) and spring green (150) all turn 60 degrees; yellow (60) and cyan (180) stay. | largest difference 1.5e-7 | yes |
| FX-HUESAT-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-042 frame 0: Reds widened to 240, 300, 60, 120 over the yellows at their own, both saturation -50: yellow (60) is fully in both, so it takes -100 and turns grey; overlapping ranges add. | largest difference 1.5e-7 | yes |
| FX-HUESAT-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-043 frame 0: Magentas dragged round to 330, 0, 35, 55, saturation -100: red (0) and orange (30) turn grey; magenta (300), its own colour, stays. | largest difference 1.5e-7 | yes |
| FX-HUESAT-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-044 frame 0: Reds 45, 15, 345, 315, the handles backwards, more than one turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-044 frame 4: Reds 45, 15, 345, 315, the handles backwards, more than one turn. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-045 frame 0: A greens handle at 361, past 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-045 frame 4: A greens handle at 361, past 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-046 frame 0: A cyans handle at -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-046 frame 4: A cyans handle at -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-047 frame 0: Blues with three handles, not four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-047 frame 4: Blues with three handles, not four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HUESAT-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_huesat_024.json, a file from before the handles, opens at After Effects' own and is saved without them | {"greens_hsl":[90,-40,10],"hue":0,"lightness":0,"saturation":0} | yes |
| fx_huesat_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a handle written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| reds handles backwards, more than a turn is refused with a sentence, and nothing changes | Hue/Saturation's reds_range handles go round the wheel in order within one turn, and these go further. | yes |
| a reds handle at 361 is refused with a sentence, and nothing changes | Hue/Saturation's reds_range handles are 0 to 360 degrees, and one is 361. | yes |
| a reds handle at -1 is refused with a sentence, and nothing changes | Hue/Saturation's reds_range handles are 0 to 360 degrees, and one is -1. | yes |
| reds with three handles is refused with a sentence, and nothing changes | Hue/Saturation's reds_range is four hue angles, and it has 3. | yes |
| reds with five handles is refused with a sentence, and nothing changes | Hue/Saturation's reds_range is four hue angles, and it has 5. | yes |
| reds widened to 275, 320, 35, 85 is taken | taken | yes |
| reds as one hue, 0, 0, 0, 0 is taken | taken | yes |
| reds at 0 and 360, the same place is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_huesat_038.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_huesat_042.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

46 of 46 checks pass.
