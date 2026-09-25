# B-41: colour key

D-97, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/color_key/expected_color_key.json`, written by `tools/color_key_reference.py` before this code existed and printed in document 25 as FX-KEY-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-KEY-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KEY-001 frame 0: The screen's green chosen, tolerance 0: the screen goes, its half-covering right edge too; the shadow, the spill and the figure stay. | largest difference 1.9e-7 | yes |
| FX-KEY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-002 frame 0: Tolerance 80: the shadow, 77 from the green on its green channel, goes too; the spill, 90 from it on its red, stays. | largest difference 1.9e-7 | yes |
| FX-KEY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-003 frame 0: Tolerance 60, softness 40: the shadow, 17 into the band, keeps 17/40 of its colour and covering, and the spill, 30 into it, keeps 30/40; the figure, far past the band, stays whole. | largest difference 1.9e-7 | yes |
| FX-KEY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-004 frame 0: Match hue, tolerance 10: the shadow, the same hue as the screen, goes with it; the spill, 13 degrees of hue away (18.6 of 255), stays, and so does the figure and its grey button. | largest difference 1.9e-7 | yes |
| FX-KEY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-005 frame 0: Match hue, tolerance 10, softness 20: the spill keeps (18.6 - 10) / 20 of itself. | largest difference 1.9e-7 | yes |
| FX-KEY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-006 frame 0: No colour chosen: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-KEY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-007 frame 0: No colour chosen, match hue, tolerance 255: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-KEY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-008 frame 0: Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frames 0 and 2, tolerance 0 and 50, are FX-KEY-001; frame 4, tolerance 100, takes the shadow and the spill too. | largest difference 1.9e-7 | yes |
| FX-KEY-008 frame 2: Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frames 0 and 2, tolerance 0 and 50, are FX-KEY-001; frame 4, tolerance 100, takes the shadow and the spill too. | largest difference 1.9e-7 | yes |
| FX-KEY-008 frame 4: Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frames 0 and 2, tolerance 0 and 50, are FX-KEY-001; frame 4, tolerance 100, takes the shadow and the spill too. | largest difference 1.9e-7 | yes |
| FX-KEY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-009 frame 0: Tolerance 60, softness keyed from 0 at frame 0 to 100 at frame 4: frame 0 keeps the shadow and the spill whole; frame 4 keeps 17/100 of the shadow and 30/100 of the spill, and the band now reaches the line, 151 from the green, which keeps 91/100, and the button, 128 from it, 68/100. | largest difference 1.9e-7 | yes |
| FX-KEY-009 frame 4: Tolerance 60, softness keyed from 0 at frame 0 to 100 at frame 4: frame 0 keeps the shadow and the spill whole; frame 4 keeps 17/100 of the shadow and 30/100 of the spill, and the band now reaches the line, 151 from the green, which keeps 91/100, and the button, 128 from it, 68/100. | largest difference 1.9e-7 | yes |
| FX-KEY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-010 frame 0: The green and the skin chosen: the screen and the figure's skin go; the line, the button, the spill and the shadow stay. | largest difference 9.8e-8 | yes |
| FX-KEY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-011 frame 0: FX-KEY-001 with the colour written in capitals: the same. | largest difference 1.9e-7 | yes |
| FX-KEY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-012 frame 0: The grey chosen, match rgb: only the button goes. | largest difference 1.9e-7 | yes |
| FX-KEY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-013 frame 0: The grey chosen, match hue, tolerance 254: a grey is 255 from everything, so nothing goes. | largest difference 1.9e-7 | yes |
| FX-KEY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-014 frame 0: The grey chosen, match hue, tolerance 255: everything is within 255, so the frame is empty. | largest difference 0.0e0 | yes |
| FX-KEY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-015 frame 0: FX-KEY-003 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-KEY-015 frame 3: FX-KEY-003 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-KEY-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KEY-016 frame 0: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-016 frame 4: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-017 frame 0: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-017 frame 4: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-018 frame 0: Softness 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-018 frame 4: Softness 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-019 frame 0: Softness keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-019 frame 4: Softness keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-020 frame 0: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-020 frame 4: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-021 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-021 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-022 frame 0: Match "hsv", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-022 frame 4: Match "hsv", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KEY-023 frame 0: Match "RGB", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-023 frame 4: Match "RGB", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KEY-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is kept, faded or cleared where it is | 0 | yes |
| a half-size draft preview changes nothing: tolerance and softness are colour distances | ColorKey { colors: ["#00b140"], tolerance: 20.0, softness: 40.0, match_by: "rgb" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_key_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_key_011.json, its colour written in capitals, is saved in small letters | ["#00b140"] | yes |
| fx_key_023.json's match "RGB" is saved as written, not corrected | "RGB" | yes |
| a file with no `match` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a softness written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tolerance 256 is refused with a sentence, and nothing changes | Colour Key's tolerance runs from 0 to 255, and this is 256. | yes |
| tolerance -1 is refused with a sentence, and nothing changes | Colour Key's tolerance runs from 0 to 255, and this is -1. | yes |
| softness 256 is refused with a sentence, and nothing changes | Colour Key's softness runs from 0 to 255, and this is 256. | yes |
| softness -1 is refused with a sentence, and nothing changes | Colour Key's softness runs from 0 to 255, and this is -1. | yes |
| nine colours is refused with a sentence, and nothing changes | Colour Key takes up to eight colours, and this has 9. | yes |
| the colour "#12345" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12345". | yes |
| match "hsv" is refused with a sentence, and nothing changes | Colour Key matches by "rgb" or "hue", and this is "hsv". | yes |
| match "Hue" is refused with a sentence, and nothing changes | Colour Key matches by "rgb" or "hue", and this is "Hue". | yes |
| softness keyed to 300 is refused with a sentence, and nothing changes | Colour Key's softness runs from 0 to 255, and this is 300. | yes |
| tolerance and softness 255 with eight colours, match hue, the top of the ranges, is taken | taken | yes |
| tolerance and softness 0 with no colour, the bottom, is taken | taken | yes |
| tolerance keyed from 0 to 100 is taken | taken | yes |
| softness keyed from 0 to 100 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_key_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_key_015.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

90 of 90 checks pass.
