# B-147: Extract

D-212, accepted on 2026-09-28 with the After Effects picks (B10). Every expected pixel is `Fixtures/extract/expected_extract.json`, written by `tools/extract_reference.py` before this code existed and printed in document 25 as FX-EXTRACT-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-EXTRACT-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EXTRACT-001 frame 0: As it starts: luminance, black point 0, white point 255, no softness: every pixel is kept, black and white too, and the frame is the drawing. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-002 frame 0: Black point 60: black, blue, the line, red and the soft line, all darker, turn transparent; the dark grey at 64 is kept. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-003 frame 0: White point 190: the light grey, the skin, white and the soft skin, all brighter, turn transparent; green at 182.4 is kept. | largest difference 1.5e-7 | yes |
| FX-EXTRACT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-004 frame 0: Black point 60 and white point 190: only the middle band is kept, from the dark grey to green. | largest difference 1.5e-7 | yes |
| FX-EXTRACT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-005 frame 0: Black point 128: grey 128, exactly on the point, is kept; grey 127 goes. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-006 frame 0: Black point 20 with black softness 100: blue, below 20, goes; the pixels from 20 to 120 fade in, the line faintly, red and the dark grey more; grey 127 and above are kept whole. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-007 frame 0: White softness 60: the pixels from 195 to 255 fade out, the skin at 219 part way, white wholly; the light grey at 192 is kept whole. | largest difference 2.0e-7 | yes |
| FX-EXTRACT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-008 frame 0: Invert on, black point 60: the other side: black, blue, the line, red and the soft line are kept, and every brighter pixel goes. | largest difference 3.0e-8 | yes |
| FX-EXTRACT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-009 frame 0: FX-EXTRACT-006 inverted: each fading pixel keeps what 006 took, so the two add up to the drawing. | largest difference 5.7e-8 | yes |
| FX-EXTRACT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-010 frame 0: Channel red, black point 128: red, the skins and grey 128 are kept, blue, green, the line and grey 127 go, by their red alone. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-011 frame 0: Channel green, black point 128: green, the skins and grey 128 are kept, red and blue go. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-012 frame 0: Channel blue, white point 100: blue, the skins, the greys from 127 and white go; black, red, green, the line and the dark grey are kept. | largest difference 3.0e-8 | yes |
| FX-EXTRACT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-013 frame 0: Channel alpha, black point 200: the two half-covered columns go, every fully covered pixel is kept whatever its colour. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-014 frame 0: Black point 200 above white point 100: nothing is kept. | largest difference 0.0e0 | yes |
| FX-EXTRACT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-015 frame 0: Black point keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-EXTRACT-001, frame 2 at 127.5 keeps grey 128 and loses grey 127, and at frame 4 only white is left. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-015 frame 2: Black point keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-EXTRACT-001, frame 2 at 127.5 keeps grey 128 and loses grey 127, and at frame 4 only white is left. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-015 frame 4: Black point keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-EXTRACT-001, frame 2 at 127.5 keeps grey 128 and loses grey 127, and at frame 4 only white is left. | largest difference 0.0e0 | yes |
| FX-EXTRACT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-016 frame 0: White point eased from 255 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it has gone below 0 and is held there, so frames 2 and 4 keep only black. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-016 frame 2: White point eased from 255 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it has gone below 0 and is held there, so frames 2 and 4 keep only black. | largest difference 0.0e0 | yes |
| FX-EXTRACT-016 frame 4: White point eased from 255 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it has gone below 0 and is held there, so frames 2 and 4 keep only black. | largest difference 0.0e0 | yes |
| FX-EXTRACT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-017 frame 0: FX-EXTRACT-004 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-EXTRACT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXTRACT-018 frame 0: Black point -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-018 frame 4: Black point -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-019 frame 0: Black point 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-019 frame 4: Black point 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-020 frame 0: White point 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-020 frame 4: White point 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-021 frame 0: Black softness 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-021 frame 4: Black softness 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-022 frame 0: White softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-022 frame 4: White softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-023 frame 0: Channel "luma", which is not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-023 frame 4: Channel "luma", which is not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-024 frame 0: Channel "Red", written with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-024 frame 4: Channel "Red", written with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-025 frame 0: Invert "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-025 frame 4: Invert "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXTRACT-026 frame 0: Black point keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-026 frame 4: Black point keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EXTRACT-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-EXTRACT-027, in dispute (D-215, proposed)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EXTRACT-027, in dispute (D-215, proposed): the build refuses fx_extract_027.json as a fault in its shape, as it does a number written as a word in every effect; the case expects it kept with a warning | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | Extract { channel: "green", black_point: 30.0, white_point: 220.0, black_softness: 10.0, white_softness: 20.0, invert: "on" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_extract_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_extract_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `channel` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a white softness that is a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| black point -1 is refused with a sentence, and nothing changes | Extract's black point runs from 0 to 255, and this is -1. | yes |
| white point 256 is refused with a sentence, and nothing changes | Extract's white point runs from 0 to 255, and this is 256. | yes |
| black softness 256 is refused with a sentence, and nothing changes | Extract's black softness runs from 0 to 255, and this is 256. | yes |
| white softness -1 is refused with a sentence, and nothing changes | Extract's white softness runs from 0 to 255, and this is -1. | yes |
| channel "luma" is refused with a sentence, and nothing changes | Extract's channel is "luminance", "red", "green", "blue" or "alpha", and this is "luma". | yes |
| channel "Red", written with a capital is refused with a sentence, and nothing changes | Extract's channel is "luminance", "red", "green", "blue" or "alpha", and this is "Red". | yes |
| invert "yes" is refused with a sentence, and nothing changes | Extract's invert is "off" or "on", and this is "yes". | yes |
| black point keyed to 300 is refused with a sentence, and nothing changes | Extract's black point runs from 0 to 255, and this is 300. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| black point keyed from 0 to 255 is taken | taken | yes |
| white softness keyed from 0 to 255 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_extract_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_extract_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_extract_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_extract_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up scan, in `verification/B-147 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, as it starts: every part of the drawing kept as it was; draws cleanly | [], kept ["paper", "sky", "line", "skin", "scarf", "star"] | yes |
| paper_gone.png, white point 220: the cream paper, brighter, taken out; the line, sky, skin, scarf and star kept; draws cleanly | [], kept ["sky", "line", "skin", "scarf", "star"], gone ["paper"] | yes |
| paper_soft.png, white point 220 with white softness 40: the paper gone, the line, sky and scarf whole, and the skin and star, within 40 of the point, faded part way; draws cleanly | [], kept ["sky", "line", "scarf"], gone ["paper"] | yes |
| line_gone.png, black point 40: the dark line and mouth taken out, everything else kept; draws cleanly | [], kept ["paper", "sky", "skin", "scarf", "star"], gone ["line"] | yes |
| line_only.png, black point 40 inverted: only the line and mouth kept; draws cleanly | [], kept ["line"], gone ["paper", "sky", "skin", "scarf", "star"] | yes |
| middle.png, black point 60 to white point 200: only the middle tones, the sky and the scarf, kept; draws cleanly | [], kept ["sky", "scarf"], gone ["paper", "line", "skin", "star"] | yes |
| red_channel.png, channel red, white point 100: what has little red, the line and the sky, kept; the paper, skin, scarf and star, all red enough, taken out; draws cleanly | [], kept ["sky", "line"], gone ["paper", "skin", "scarf", "star"] | yes |
| in paper_soft.png the skin, nearer the white point, is fainter than the star, and both show a little | skin covering 6, star covering 88 | yes |

## Result

121 of 121 checks pass.
