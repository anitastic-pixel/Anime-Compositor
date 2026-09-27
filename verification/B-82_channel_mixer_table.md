# B-82: channel mixer

D-139, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the sixth of the third batch. Every expected pixel is `Fixtures/channel_mixer/expected_channel_mixer.json`, written by `tools/channel_mixer_reference.py` before this code existed and printed in document 25 as FX-MIXER-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MIXER-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIXER-001 frame 0: The settings as they start, red 100, 0, 0, 0, green 0, 100, 0, 0, blue 0, 0, 100, 0, monochrome off: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-MIXER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-002 frame 0: Red 0, 0, 100, 0 and blue 100, 0, 0, 0, red and blue swapped: the red turns blue, #2828c8, the blue turns orange, #ff6040, the green #50b43c and the skin a pale blue, #bed6f6; the white, the grey and the black stay as they are, as does every grey. | largest difference 1.9e-7 | yes |
| FX-MIXER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-003 frame 0: Red 0, 0, 0, 0: every pixel loses its red: the red turns #002828, the skin #00d6be and the white cyan, #00ffff; the black stays black. | largest difference 1.3e-7 | yes |
| FX-MIXER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-004 frame 0: Red 200, 0, 0, 0: every red doubled and held at the top: the line's red goes from #1e to #3c, the blue's from #40 to #80, and the grey, the red and the skin reach full red; the white and the black stay as they are. | largest difference 1.3e-7 | yes |
| FX-MIXER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-005 frame 0: Green 0, 100, 0, 50, a constant: every shown pixel's green is raised half the scale, held at the top: the black turns #008000 but for rounding (127.5), the grey's green is full, and the white stays white. | largest difference 1.9e-7 | yes |
| FX-MIXER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-006 frame 0: Each row taking its own channel away from a constant of 100, the negative: the white turns black and the black white, the grey #7f7f7f, the red cyan, #37d7d7, and the line a pale #e1e5db. | largest difference 4.7e-8 | yes |
| FX-MIXER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-007 frame 0: Monochrome on, the rows as they start: every channel takes the red row, so every pixel turns the grey of its own red: the red #c8c8c8, the green #3c3c3c, the blue #404040, the skin #f6f6f6; the white, the grey and the black stay as they are. | largest difference 1.9e-7 | yes |
| FX-MIXER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-008 frame 0: Monochrome on, red 30, 59, 11, 0: every pixel turns grey by those weights, the red #585858 (88), the green 133 and the skin 220.96 of 255; the weights add up to 100, so the white, the grey and the black stay as they are. | largest difference 8.5e-8 | yes |
| FX-MIXER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-009 frame 0: FX-MIXER-008 with green 0, 0, 0, 0 and blue 200, -200, 0, 50: with monochrome on the green and blue rows are not used, so the same. | largest difference 8.5e-8 | yes |
| FX-MIXER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-010 frame 0: Green 100, 0, 0, 0, green from red: every pixel's green becomes its red: the red turns yellow, #c8c828, the green #3c3c50, the blue #4040ff and the skin #f6f6be; every grey stays as it is. | largest difference 1.9e-7 | yes |
| FX-MIXER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-011 frame 0: Red 200, -200, 0, 0, the ends of the range: the red is twice the red less the green, held inside the scale: the red swatch keeps a full red, #ff2828, the green and the blue lose all their red, the skin's red is 64 (#40d6be) and every grey loses its red: the white turns cyan, #00ffff. | largest difference 1.3e-7 | yes |
| FX-MIXER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-012 frame 0: Red keyed from 100, 0, 0, 0 at frame 0 to 0, 0, 100, 0 at frame 4 and blue from 0, 0, 100, 0 to 100, 0, 0, 0, linear, one key holding all four numbers: frame 0 untouched, frame 2 red and blue both 50, 0, 50, 0, so the red swatch turns purple, #782878, and every grey stays as it is; frame 4 is FX-MIXER-002. | largest difference 1.9e-7 | yes |
| FX-MIXER-012 frame 2: Red keyed from 100, 0, 0, 0 at frame 0 to 0, 0, 100, 0 at frame 4 and blue from 0, 0, 100, 0 to 100, 0, 0, 0, linear, one key holding all four numbers: frame 0 untouched, frame 2 red and blue both 50, 0, 50, 0, so the red swatch turns purple, #782878, and every grey stays as it is; frame 4 is FX-MIXER-002. | largest difference 1.3e-7 | yes |
| FX-MIXER-012 frame 4: Red keyed from 100, 0, 0, 0 at frame 0 to 0, 0, 100, 0 at frame 4 and blue from 0, 0, 100, 0 to 100, 0, 0, 0, linear, one key holding all four numbers: frame 0 untouched, frame 2 red and blue both 50, 0, 50, 0, so the red swatch turns purple, #782878, and every grey stays as it is; frame 4 is FX-MIXER-002. | largest difference 1.9e-7 | yes |
| FX-MIXER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-013 frame 0: Green eased from 0, 100, 0, 0 at frame 0 to 0, 100, 0, 200 at frame 4 on a curve that overshoots: frame 0 untouched; at frame 2 the constant has gone past 200 (265) and is held there, so frames 2 and 4 alike give every shown pixel a full green. | largest difference 1.9e-7 | yes |
| FX-MIXER-013 frame 2: Green eased from 0, 100, 0, 0 at frame 0 to 0, 100, 0, 200 at frame 4 on a curve that overshoots: frame 0 untouched; at frame 2 the constant has gone past 200 (265) and is held there, so frames 2 and 4 alike give every shown pixel a full green. | largest difference 1.9e-7 | yes |
| FX-MIXER-013 frame 4: Green eased from 0, 100, 0, 0 at frame 0 to 0, 100, 0, 200 at frame 4 on a curve that overshoots: frame 0 untouched; at frame 2 the constant has gone past 200 (265) and is held there, so frames 2 and 4 alike give every shown pixel a full green. | largest difference 1.9e-7 | yes |
| FX-MIXER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-014 frame 0: Red keyed from 50, 0, 0, 0 at frame 0 to 150, 0, 0, 0 at frame 4: frame 0 halves every red, frame 2, the rows as they start, is the drawing untouched, and frame 4 raises every red by half, held at the top. | largest difference 1.3e-7 | yes |
| FX-MIXER-014 frame 2: Red keyed from 50, 0, 0, 0 at frame 0 to 150, 0, 0, 0 at frame 4: frame 0 halves every red, frame 2, the rows as they start, is the drawing untouched, and frame 4 raises every red by half, held at the top. | largest difference 1.9e-7 | yes |
| FX-MIXER-014 frame 4: Red keyed from 50, 0, 0, 0 at frame 0 to 150, 0, 0, 0 at frame 4: frame 0 halves every red, frame 2, the rows as they start, is the drawing untouched, and frame 4 raises every red by half, held at the top. | largest difference 1.3e-7 | yes |
| FX-MIXER-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-015 frame 0: Monochrome on, red keyed from 100, 0, 0, 0 at frame 0 to 30, 59, 11, 0 at frame 4: frame 0 is FX-MIXER-007, frame 4 FX-MIXER-008, and frame 2 lies half way, the weights 65, 29.5, 5.5, 0. | largest difference 1.9e-7 | yes |
| FX-MIXER-015 frame 2: Monochrome on, red keyed from 100, 0, 0, 0 at frame 0 to 30, 59, 11, 0 at frame 4: frame 0 is FX-MIXER-007, frame 4 FX-MIXER-008, and frame 2 lies half way, the weights 65, 29.5, 5.5, 0. | largest difference 1.4e-7 | yes |
| FX-MIXER-015 frame 4: Monochrome on, red keyed from 100, 0, 0, 0 at frame 0 to 30, 59, 11, 0 at frame 4: frame 0 is FX-MIXER-007, frame 4 FX-MIXER-008, and frame 2 lies half way, the weights 65, 29.5, 5.5, 0. | largest difference 8.5e-8 | yes |
| FX-MIXER-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-016 frame 0: FX-MIXER-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-MIXER-016 frame 3: FX-MIXER-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-MIXER-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIXER-017 frame 0: Red 201, 0, 0, 0: a number above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-017 frame 4: Red 201, 0, 0, 0: a number above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIXER-018 frame 0: Green 0, -250, 0, 0: a number below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-018 frame 4: Green 0, -250, 0, 0: a number below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIXER-019 frame 0: Green 0, 100, 0, -201: a constant below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-019 frame 4: Green 0, 100, 0, -201: a constant below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIXER-020 frame 0: Blue written with three numbers, 0, 0, 100: a row is four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-020 frame 4: Blue written with three numbers, 0, 0, 100: a row is four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIXER-021 frame 0: Red keyed to 0, 0, 300, 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-021 frame 4: Red keyed to 0, 0, 300, 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIXER-022 frame 0: Monochrome "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-022 frame 4: Monochrome "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIXER-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | ChannelMixer { red: [0.0, 0.0, 100.0, 0.0], green: [0.0, 100.0, 0.0, 0.0], blue: [100.0, 0.0, 0.0, 0.0], monochrome: "off" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mixer_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mixer_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `blue` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a red row that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red 201, 0, 0, 0 is refused with a sentence, and nothing changes | Channel Mixer's red runs from -200 to 200, and this is 201. | yes |
| green's constant -201 is refused with a sentence, and nothing changes | Channel Mixer's green runs from -200 to 200, and this is -201. | yes |
| blue as three numbers is refused with a sentence, and nothing changes | Channel Mixer's blue row is four numbers, from red, green, blue and a constant, and this has 3. | yes |
| monochrome "yes" is refused with a sentence, and nothing changes | Channel Mixer's monochrome is "off" or "on", and this is "yes". | yes |
| red keyed to 0, 0, 300, 0 is refused with a sentence, and nothing changes | Channel Mixer's red runs from -200 to 200, and this is 300. | yes |
| every number at -200, the bottom, is taken | taken | yes |
| every number at 200 with monochrome on, the top, is taken | taken | yes |
| red keyed from red to blue is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mixer_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mixer_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mixer_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

85 of 85 checks pass.
