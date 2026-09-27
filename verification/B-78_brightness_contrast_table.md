# B-78: brightness and contrast

D-135, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the second of the third batch. Every expected pixel is `Fixtures/brightness_contrast/expected_brightness_contrast.json`, written by `tools/brightness_contrast_reference.py` before this code existed and printed in document 25 as FX-BRICON-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BRICON-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BRICON-001 frame 0: Brightness 0 and contrast 0, the settings as they start: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BRICON-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-002 frame 0: Brightness 50: every channel of every shown pixel is lifted 50 levels of 255 as it is written: black turns #323232, the grey #b2b2b2; the white, already full, is held there and stays white, and the skin's red and green are held at full while its blue rises to 240. | largest difference 1.6e-7 | yes |
| FX-BRICON-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-003 frame 0: Brightness -50: every channel lowered 50 levels: white turns #cdcdcd, the grey #4e4e4e; black, and the line's channels, all below 50, are held at 0, so both turn black. | largest difference 1.5e-7 | yes |
| FX-BRICON-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-004 frame 0: Brightness 150, the most: black turns #969696, and every channel at or above 105 goes to full, so the grey, the light grey, the skin, the shaded skin and the pink all turn white; the blue keeps a little of its red (58 + 150 = 208). | largest difference 5.4e-8 | yes |
| FX-BRICON-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-005 frame 0: Brightness -150, the least: white turns #696969 (105), and every channel at or below 150 goes to 0, so the tones up to the grey turn black and the red keeps only some of its red (214 - 150 = 64). | largest difference 5.8e-8 | yes |
| FX-BRICON-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-006 frame 0: Contrast 50: the tones pushed apart from half by 1 / 0.505, about 1.98: the dark grey darkens, the light grey lightens, the grey, a hair above half, moves a hair up, black and white stay, and the colours grow stronger. | largest difference 3.3e-7 | yes |
| FX-BRICON-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-007 frame 0: Contrast -50: the tones drawn half way to half brightness: black turns #404040 but for rounding (63.75), white #bfbfbf (191.25), and the colours grow duller. | largest difference 6.4e-8 | yes |
| FX-BRICON-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-008 frame 0: Contrast -100, the least: every shown pixel, whatever its colour, turns flat mid grey, #808080 but for rounding (127.5), at its own covering. | largest difference 3.0e-8 | yes |
| FX-BRICON-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-009 frame 0: Contrast 100, the most: the tones pushed apart 100 times: every channel below half goes to 0 and every one above to full, so the colours turn the pure colours they lean to, the blue #0000ff and the skin #ffffff; only the grey, half a level above half, stays between, at 0.5 + 0.5 / 255 * 100 = 0.696. | largest difference 4.6e-6 | yes |
| FX-BRICON-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-010 frame 0: Brightness 30 and contrast 40: contrast first, about half, then brightness added: the grey turns 0.5 + (0.5 / 255) / 0.604 + 30 / 255 of its scale. | largest difference 2.4e-7 | yes |
| FX-BRICON-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-011 frame 0: Brightness -100 and contrast -100: every shown pixel turns the one flat dark grey 0.5 - 100 / 255 of the scale (27.5 levels). | largest difference 3.0e-8 | yes |
| FX-BRICON-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-012 frame 0: Brightness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-BRICON-002 (brightness 50), frame 4 brightness 100. | largest difference 1.9e-7 | yes |
| FX-BRICON-012 frame 2: Brightness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-BRICON-002 (brightness 50), frame 4 brightness 100. | largest difference 1.6e-7 | yes |
| FX-BRICON-012 frame 4: Brightness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-BRICON-002 (brightness 50), frame 4 brightness 100. | largest difference 1.6e-7 | yes |
| FX-BRICON-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-013 frame 0: Contrast keyed from -100 at frame 0 to 100 at frame 4: frame 0 FX-BRICON-008, frame 2, both at 0, the drawing untouched, frame 4 FX-BRICON-009. | largest difference 3.0e-8 | yes |
| FX-BRICON-013 frame 2: Contrast keyed from -100 at frame 0 to 100 at frame 4: frame 0 FX-BRICON-008, frame 2, both at 0, the drawing untouched, frame 4 FX-BRICON-009. | largest difference 1.9e-7 | yes |
| FX-BRICON-013 frame 4: Contrast keyed from -100 at frame 0 to 100 at frame 4: frame 0 FX-BRICON-008, frame 2, both at 0, the drawing untouched, frame 4 FX-BRICON-009. | largest difference 4.6e-6 | yes |
| FX-BRICON-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-014 frame 0: Brightness eased from 0 at frame 0 to 150 at frame 4 on a curve that overshoots: at frame 2 it has gone past 150 and is held there, so frames 2 and 4 are both FX-BRICON-004. | largest difference 1.9e-7 | yes |
| FX-BRICON-014 frame 2: Brightness eased from 0 at frame 0 to 150 at frame 4 on a curve that overshoots: at frame 2 it has gone past 150 and is held there, so frames 2 and 4 are both FX-BRICON-004. | largest difference 5.4e-8 | yes |
| FX-BRICON-014 frame 4: Brightness eased from 0 at frame 0 to 150 at frame 4 on a curve that overshoots: at frame 2 it has gone past 150 and is held there, so frames 2 and 4 are both FX-BRICON-004. | largest difference 5.4e-8 | yes |
| FX-BRICON-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-015 frame 0: Brightness held at 50 from frame 0 and keyed to -50 at frame 3: frames 0 and 2 are FX-BRICON-002, frame 4 FX-BRICON-003. | largest difference 1.6e-7 | yes |
| FX-BRICON-015 frame 2: Brightness held at 50 from frame 0 and keyed to -50 at frame 3: frames 0 and 2 are FX-BRICON-002, frame 4 FX-BRICON-003. | largest difference 1.6e-7 | yes |
| FX-BRICON-015 frame 4: Brightness held at 50 from frame 0 and keyed to -50 at frame 3: frames 0 and 2 are FX-BRICON-002, frame 4 FX-BRICON-003. | largest difference 1.5e-7 | yes |
| FX-BRICON-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-016 frame 0: FX-BRICON-010 moved three pixels right: the same, moved. | largest difference 2.4e-7 | yes |
| FX-BRICON-016 frame 3: FX-BRICON-010 moved three pixels right: the same, moved. | largest difference 2.4e-7 | yes |
| FX-BRICON-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRICON-017 frame 0: Brightness 151, above 150. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-017 frame 4: Brightness 151, above 150. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRICON-018 frame 0: Brightness -151, below -150. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-018 frame 4: Brightness -151, below -150. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRICON-019 frame 0: Contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-019 frame 4: Contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRICON-020 frame 0: Contrast -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-020 frame 4: Contrast -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRICON-021 frame 0: Contrast keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-021 frame 4: Contrast keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRICON-022 frame 0: Brightness keyed from -160 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-022 frame 4: Brightness keyed from -160 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRICON-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | BrightnessContrast { brightness: 30.0, contrast: 40.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bricon_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bricon_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `contrast` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a brightness that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| brightness 151 is refused with a sentence, and nothing changes | Brightness & Contrast's brightness runs from -150 to 150, and this is 151. | yes |
| brightness -151 is refused with a sentence, and nothing changes | Brightness & Contrast's brightness runs from -150 to 150, and this is -151. | yes |
| contrast 101 is refused with a sentence, and nothing changes | Brightness & Contrast's contrast runs from -100 to 100, and this is 101. | yes |
| contrast -101 is refused with a sentence, and nothing changes | Brightness & Contrast's contrast runs from -100 to 100, and this is -101. | yes |
| contrast keyed to 120 is refused with a sentence, and nothing changes | Brightness & Contrast's contrast runs from -100 to 100, and this is 120. | yes |
| brightness -150 and contrast 100, the ends, is taken | taken | yes |
| brightness 150 and contrast -100, the other ends, is taken | taken | yes |
| brightness keyed from 0 to 150 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bricon_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bricon_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bricon_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

85 of 85 checks pass.
