# B-79: black and white

D-136, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the third of the third batch. Every expected pixel is `Fixtures/black_white/expected_black_white.json`, written by `tools/black_white_reference.py` before this code existed and printed in document 25 as FX-BW-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BW-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BW-001 frame 0: The settings as they start, 40, 60, 40, 60, 20, 80: every pixel that shows turns grey. Each pure colour becomes the grey of its own number: red and green #666666 (102), yellow and cyan #999999 (153), blue #333333 (51), magenta #cccccc (204). Grey, white and black stay exactly as they are. The skin becomes 217.2 on the 8-bit scale, the pink 166, the leaf 128, the teal and the sky 115.2, the line 30.4, the trace red 104 and the shadow 176; the half-covered red and skin are the red's and the skin's greys at half covering. | largest difference 1.2e-7 | yes |
| FX-BW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-002 frame 0: All six at 100: each pixel becomes its largest channel, so every pure colour turns white and the skin, the pink and the trace red take their red. | largest difference 1.9e-7 | yes |
| FX-BW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-003 frame 0: All six at 0: each pixel becomes its smallest channel, so every pure colour turns black, the skin takes its blue and the sky its red. | largest difference 1.3e-7 | yes |
| FX-BW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-004 frame 0: Reds 300, the most: red, and every colour led by red (the skin, the pink, the trace red, the shadow), is lifted to white, held there; yellow and magenta, where red only ties for the lead, and every colour led by green or blue, are as in FX-BW-001. | largest difference 4.6e-8 | yes |
| FX-BW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-005 frame 0: Reds -200, the least: red, the trace red and the pink fall below 0 and are held at black; the skin and the shadow darken; the rest as in FX-BW-001. | largest difference 5.3e-8 | yes |
| FX-BW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-006 frame 0: Yellows 300: yellow turns white, and the colours whose two largest channels are red and green (the skin, the leaf, the shadow) lighten; red and green, with nothing between their largest and smallest, stay as in FX-BW-001. | largest difference 2.1e-7 | yes |
| FX-BW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-007 frame 0: Greens -200: green turns black and the leaf and the teal, led by green, darken; the rest as in FX-BW-001. | largest difference 1.2e-7 | yes |
| FX-BW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-008 frame 0: Cyans 0: cyan turns black, its smallest channel, and the teal and the sky, whose two largest are green and blue, darken; the rest as in FX-BW-001. | largest difference 1.2e-7 | yes |
| FX-BW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-009 frame 0: Blues 300: blue turns white, and the line and the sky, led by blue, lighten; the rest as in FX-BW-001. | largest difference 1.2e-7 | yes |
| FX-BW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-010 frame 0: Magentas -200: magenta turns black, and the pink and the line, whose two largest are red and blue, darken; the rest as in FX-BW-001. | largest difference 1.2e-7 | yes |
| FX-BW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-011 frame 0: A red filter, 120, 110, -10, -50, -50, 120, as a red glass in front of a black and white camera: the warm colours (red, yellow, magenta, the skin) come out lighter than at the start and the cool ones (cyan, blue, the sky) darker; blue and cyan fall below 0 and are held at black. | largest difference 2.2e-7 | yes |
| FX-BW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-012 frame 0: Reds keyed from 40 at frame 0 to 300 at frame 4, linear: frame 0 is FX-BW-001, frame 2 reds 170, frame 4 FX-BW-004. | largest difference 1.2e-7 | yes |
| FX-BW-012 frame 2: Reds keyed from 40 at frame 0 to 300 at frame 4, linear: frame 0 is FX-BW-001, frame 2 reds 170, frame 4 FX-BW-004. | largest difference 1.8e-7 | yes |
| FX-BW-012 frame 4: Reds keyed from 40 at frame 0 to 300 at frame 4, linear: frame 0 is FX-BW-001, frame 2 reds 170, frame 4 FX-BW-004. | largest difference 4.6e-8 | yes |
| FX-BW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-013 frame 0: Blues keyed from -200 at frame 0 to 300 at frame 4: frame 0 blue is black, frame 2 blues 50, a grey #808080 but for rounding (127.5), frame 4 blue is white. | largest difference 1.2e-7 | yes |
| FX-BW-013 frame 2: Blues keyed from -200 at frame 0 to 300 at frame 4: frame 0 blue is black, frame 2 blues 50, a grey #808080 but for rounding (127.5), frame 4 blue is white. | largest difference 1.2e-7 | yes |
| FX-BW-013 frame 4: Blues keyed from -200 at frame 0 to 300 at frame 4: frame 0 blue is black, frame 2 blues 50, a grey #808080 but for rounding (127.5), frame 4 blue is white. | largest difference 1.2e-7 | yes |
| FX-BW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-014 frame 0: Magentas eased from 80 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 the number has gone past 300 and is held there, so frames 2 and 4 are both magentas 300, magenta white. | largest difference 1.2e-7 | yes |
| FX-BW-014 frame 2: Magentas eased from 80 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 the number has gone past 300 and is held there, so frames 2 and 4 are both magentas 300, magenta white. | largest difference 1.2e-7 | yes |
| FX-BW-014 frame 4: Magentas eased from 80 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 the number has gone past 300 and is held there, so frames 2 and 4 are both magentas 300, magenta white. | largest difference 1.2e-7 | yes |
| FX-BW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-015 frame 0: Greens eased from 40 at frame 0 to -200 at frame 4 on the same curve: at frame 2 the number has gone past -200 and is held there, so frames 2 and 4 are both FX-BW-007. | largest difference 1.2e-7 | yes |
| FX-BW-015 frame 2: Greens eased from 40 at frame 0 to -200 at frame 4 on the same curve: at frame 2 the number has gone past -200 and is held there, so frames 2 and 4 are both FX-BW-007. | largest difference 1.2e-7 | yes |
| FX-BW-015 frame 4: Greens eased from 40 at frame 0 to -200 at frame 4 on the same curve: at frame 2 the number has gone past -200 and is held there, so frames 2 and 4 are both FX-BW-007. | largest difference 1.2e-7 | yes |
| FX-BW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-016 frame 0: FX-BW-011 moved three pixels right: the same, moved. | largest difference 2.2e-7 | yes |
| FX-BW-016 frame 3: FX-BW-011 moved three pixels right: the same, moved. | largest difference 2.2e-7 | yes |
| FX-BW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BW-017 frame 0: Reds 301: above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-017 frame 4: Reds 301: above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BW-018 frame 0: Yellows -201: below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-018 frame 4: Yellows -201: below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BW-019 frame 0: Greens 350: above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-019 frame 4: Greens 350: above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BW-020 frame 0: Cyans -200.5: below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-020 frame 4: Cyans -200.5: below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BW-021 frame 0: Blues 1000: above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-021 frame 4: Blues 1000: above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BW-022 frame 0: Magentas keyed to -250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-022 frame 4: Magentas keyed to -250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | BlackWhite { reds: 40.0, yellows: 60.0, greens: 40.0, cyans: 60.0, blues: 20.0, magentas: 80.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bw_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bw_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `magentas` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a reds that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| reds 301 is refused with a sentence, and nothing changes | Black & White's reds runs from -200 to 300, and this is 301. | yes |
| yellows -201 is refused with a sentence, and nothing changes | Black & White's yellows runs from -200 to 300, and this is -201. | yes |
| blues 1000 is refused with a sentence, and nothing changes | Black & White's blues runs from -200 to 300, and this is 1000. | yes |
| magentas keyed to -250 is refused with a sentence, and nothing changes | Black & White's magentas runs from -200 to 300, and this is -250. | yes |
| all six at -200, the bottom, is taken | taken | yes |
| all six at 300, the top, is taken | taken | yes |
| cyans keyed from 60 to -200 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bw_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bw_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bw_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

84 of 84 checks pass.
