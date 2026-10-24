# B-56: hue/saturation

D-113, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation.json`, written by `tools/hue_saturation_reference.py` before this code existed and printed in document 25 as FX-HUESAT-001 to 036. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-HUESAT-001 to 036 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HUESAT-001 frame 0: All three at 0, the defaults: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-HUESAT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-002 frame 0: Hue +120: red turns green, green blue, and blue red; magenta, at 300, goes past 360 to 60, yellow; the grey and the empty pixels stay as they are, and the soft edge keeps its half covering. | largest difference 1.9e-7 | yes |
| FX-HUESAT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-003 frame 0: Hue -120: red turns blue, green red, and blue green. | largest difference 1.9e-7 | yes |
| FX-HUESAT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-004 frame 0: Hue -30: red, at 0, goes below 0 to 330, a pink-red #ff0080. | largest difference 2.4e-7 | yes |
| FX-HUESAT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-005 frame 0: Hue +90: magenta, at 300, goes past 360 to 30, orange #ff8000. | largest difference 2.4e-7 | yes |
| FX-HUESAT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-006 frame 0: Hue 180: every colour turns to its opposite, red to cyan; the grey stays. | largest difference 2.6e-7 | yes |
| FX-HUESAT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-007 frame 0: Saturation -100: every pixel turns grey at its own lightness, red, green, blue and magenta all to mid grey; the grey stays. | largest difference 1.3e-7 | yes |
| FX-HUESAT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-008 frame 0: Saturation +100: the line's saturation doubles, and the skin's, at 0.76, is held at 1, fully vivid, at the same hue and lightness; the pure primaries and magenta, already full, and the grey stay. | largest difference 2.3e-7 | yes |
| FX-HUESAT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-009 frame 0: Saturation -50: every colour half way to grey. | largest difference 1.6e-7 | yes |
| FX-HUESAT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-010 frame 0: Lightness +100: every shown pixel turns white at its own covering, the soft edge white at half; the empty pixels stay empty. | largest difference 3.0e-8 | yes |
| FX-HUESAT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-011 frame 0: Lightness -100: every shown pixel turns black at its own covering. | largest difference 3.0e-8 | yes |
| FX-HUESAT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-012 frame 0: Lightness +50: red's lightness goes from 0.5 half way to 1, to #ff8080. | largest difference 9.0e-8 | yes |
| FX-HUESAT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-013 frame 0: Lightness -50: red's lightness goes from 0.5 half way to 0, to #800000. | largest difference 4.2e-7 | yes |
| FX-HUESAT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-014 frame 0: Hue 60, saturation -50, lightness 20 together: red turns a pale, soft yellow; the grey is lightened only. | largest difference 2.0e-7 | yes |
| FX-HUESAT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-015 frame 0: Hue keyed from -180 at frame 0 to 180 at frame 4, linear: frames 0 and 4 are both FX-HUESAT-006, half a turn either way; frame 2, at 0, is the drawing untouched. | largest difference 2.6e-7 | yes |
| FX-HUESAT-015 frame 2: Hue keyed from -180 at frame 0 to 180 at frame 4, linear: frames 0 and 4 are both FX-HUESAT-006, half a turn either way; frame 2, at 0, is the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-HUESAT-015 frame 4: Hue keyed from -180 at frame 0 to 180 at frame 4, linear: frames 0 and 4 are both FX-HUESAT-006, half a turn either way; frame 2, at 0, is the drawing untouched. | largest difference 2.6e-7 | yes |
| FX-HUESAT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-016 frame 0: Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is FX-HUESAT-007, frame 2 the drawing, frame 4 FX-HUESAT-008. | largest difference 1.3e-7 | yes |
| FX-HUESAT-016 frame 2: Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is FX-HUESAT-007, frame 2 the drawing, frame 4 FX-HUESAT-008. | largest difference 1.9e-7 | yes |
| FX-HUESAT-016 frame 4: Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is FX-HUESAT-007, frame 2 the drawing, frame 4 FX-HUESAT-008. | largest difference 2.3e-7 | yes |
| FX-HUESAT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-017 frame 0: FX-HUESAT-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-HUESAT-017 frame 3: FX-HUESAT-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-HUESAT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-018 frame 0: Hue 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-018 frame 4: Hue 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-019 frame 0: Hue -181, below -180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-019 frame 4: Hue -181, below -180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-020 frame 0: Saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-020 frame 4: Saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-021 frame 0: Lightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-021 frame 4: Lightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-022 frame 0: Hue keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-022 frame 4: Hue keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-023 frame 0: Lightness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-023 frame 4: Lightness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HUESAT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HUESAT-024 frame 0: The wheel, greens hue +90, saturation -40, lightness +10: green, at 120, turns to 210, a sky blue, a little greyer and lighter; the yellow-green at 90 and the spring green at 150 take half of each; red, cyan, blue, magenta, the grey and the pale cyan stay as they are. | largest difference 1.5e-7 | yes |
| FX-HUESAT-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-025 frame 0: The wheel, cyans hue -120, saturation +50, lightness -20: cyan, at 180, turns to 60, a darker yellow, and so does the half-covered cyan at its half covering; the pale cyan turns to a pale yellow, more vivid and a little darker; the spring green at 150 and the azure at 210 take half; red, green, blue and the grey stay. | largest difference 2.3e-7 | yes |
| FX-HUESAT-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HUESAT-026 frame 0: The wheel, magentas hue +60, saturation -100, lightness +30: magenta, at 300, turns grey and lighter; the violet at 270 and the rose at 330 lose half their colour, turned half way on; red, green, cyan, blue and the grey stay. | largest difference 1.5e-7 | yes |
| FX-HUESAT-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
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

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is regraded where it is | 0 | yes |
| a half-size draft preview changes nothing: hue and saturation are colours, not distances | HueSaturation { hue: 60.0, saturation: -50.0, lightness: 20.0, ranges: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], colorize: "off", colorize_hue: 0.0, colorize_saturation: 25.0, colorize_lightness: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_huesat_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_huesat_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `saturation` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a lightness written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue 181 is refused with a sentence, and nothing changes | Hue/Saturation's hue runs from -180 to 180, and this is 181. | yes |
| hue -181 is refused with a sentence, and nothing changes | Hue/Saturation's hue runs from -180 to 180, and this is -181. | yes |
| saturation 101 is refused with a sentence, and nothing changes | Hue/Saturation's saturation runs from -100 to 100, and this is 101. | yes |
| saturation -101 is refused with a sentence, and nothing changes | Hue/Saturation's saturation runs from -100 to 100, and this is -101. | yes |
| lightness 101 is refused with a sentence, and nothing changes | Hue/Saturation's lightness runs from -100 to 100, and this is 101. | yes |
| lightness -101 is refused with a sentence, and nothing changes | Hue/Saturation's lightness runs from -100 to 100, and this is -101. | yes |
| hue keyed to 200 is refused with a sentence, and nothing changes | Hue/Saturation's hue runs from -180 to 180, and this is 200. | yes |
| hue 180, saturation and lightness 100, the tops of the ranges, is taken | taken | yes |
| hue -180, saturation and lightness -100, the bottoms, is taken | taken | yes |
| hue keyed from -180 to 180 is taken | taken | yes |
| lightness keyed from 0 to 50 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_huesat_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_huesat_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

120 of 120 checks pass.
