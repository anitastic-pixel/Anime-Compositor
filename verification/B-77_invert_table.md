# B-77: invert

D-134, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the first of the third batch. Every expected pixel is `Fixtures/invert/expected_invert.json`, written by `tools/invert_reference.py` before this code existed and printed in document 25 as FX-INVERT-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-INVERT-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-INVERT-001 frame 0: Channel rgb, amount 100, the settings as they start: every colour that shows turns to its opposite, each 8-bit channel v becoming 255 - v: black turns white and white black, the line #1e1a24 a pale #e1e5db, the red #c82828 a cyan #37d7d7, the grey #808080 a hair darker, #7f7f7f, the skin #f6d6be a deep #092941 and the blue #3a6fd8 an ochre #c59027; the soft edge turns #092941 at its own covering, and the empty pixels stay empty. | largest difference 4.8e-8 | yes |
| FX-INVERT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-INVERT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-003 frame 0: Amount 50: every channel of every colour that shows meets the middle, so every shown pixel turns the same mid grey, #808080 but for rounding (127.5), at its own covering. | largest difference 3.0e-8 | yes |
| FX-INVERT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-004 frame 0: Amount 25: each channel goes a quarter of the way to its opposite, e' = e / 2 + 1 / 4: black lifts to #404040 but for rounding (63.75), white falls to #c0c0c0 (191.25), and every colour keeps half its contrast. | largest difference 6.4e-8 | yes |
| FX-INVERT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-005 frame 0: Channel red: only red is inverted, 255 - v, and green and blue are kept exactly: the red #c82828 turns nearly black, #372828, the skin a blue-green #09d6be, black turns red #ff0000 and white cyan #00ffff. | largest difference 1.3e-7 | yes |
| FX-INVERT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-006 frame 0: Channel green: only green is inverted: the skin turns pink #f629be, black green #00ff00 and white magenta #ff00ff. | largest difference 1.9e-7 | yes |
| FX-INVERT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-007 frame 0: Channel blue: only blue is inverted: the blue #3a6fd8 turns olive #3a6f27, black blue #0000ff and white yellow #ffff00. | largest difference 1.9e-7 | yes |
| FX-INVERT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-008 frame 0: Channel alpha, amount 100: the covering is inverted, 1 - a, the colour kept: every pixel that fully showed disappears, every empty pixel inside the layer turns black and fully covering, the half-covered skin keeps its colour at 127/255 and the quarter-covered skin at 191/255. | largest difference 1.2e-7 | yes |
| FX-INVERT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-009 frame 0: Channel alpha, amount 50: every pixel's covering meets the middle, one half exactly, each keeping its own colour; the empty pixels turn black at half covering. | largest difference 9.3e-8 | yes |
| FX-INVERT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-010 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-INVERT-003 and frame 4 FX-INVERT-001. | largest difference 1.9e-7 | yes |
| FX-INVERT-010 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-INVERT-003 and frame 4 FX-INVERT-001. | largest difference 3.0e-8 | yes |
| FX-INVERT-010 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-INVERT-003 and frame 4 FX-INVERT-001. | largest difference 4.8e-8 | yes |
| FX-INVERT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-011 frame 0: Channel alpha, amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-INVERT-009 and frame 4 FX-INVERT-008. | largest difference 1.9e-7 | yes |
| FX-INVERT-011 frame 2: Channel alpha, amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-INVERT-009 and frame 4 FX-INVERT-008. | largest difference 9.3e-8 | yes |
| FX-INVERT-011 frame 4: Channel alpha, amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-INVERT-009 and frame 4 FX-INVERT-008. | largest difference 1.2e-7 | yes |
| FX-INVERT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-012 frame 0: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-INVERT-001, as is frame 4; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-INVERT-012 frame 2: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-INVERT-001, as is frame 4; frame 0 is the drawing. | largest difference 4.8e-8 | yes |
| FX-INVERT-012 frame 4: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-INVERT-001, as is frame 4; frame 0 is the drawing. | largest difference 4.8e-8 | yes |
| FX-INVERT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-013 frame 0: Channel alpha, amount 0: the drawing, untouched; the empty pixels stay empty. | largest difference 1.9e-7 | yes |
| FX-INVERT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-014 frame 0: FX-INVERT-001 moved three pixels right: the same, moved. | largest difference 4.8e-8 | yes |
| FX-INVERT-014 frame 3: FX-INVERT-001 moved three pixels right: the same, moved. | largest difference 4.8e-8 | yes |
| FX-INVERT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-015 frame 0: FX-INVERT-008 moved three pixels right: the same, moved; the three columns on the left, outside the layer, stay empty, as only the layer's own pixels are inverted. | largest difference 0.0e0 | yes |
| FX-INVERT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-INVERT-016 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-016 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-INVERT-017 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-017 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-INVERT-018 frame 0: Channel "luma", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-018 frame 4: Channel "luma", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-INVERT-019 frame 0: Channel "RGB", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-019 frame 4: Channel "RGB", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-INVERT-020 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-020 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-INVERT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | Invert { channel: "rgb", amount: 60.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_invert_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_invert_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `channel` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an amount that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Invert's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Invert's amount runs from 0 to 100, and this is -1. | yes |
| channel "luma" is refused with a sentence, and nothing changes | Invert's channel is "rgb", "red", "green", "blue" or "alpha", and this is "luma". | yes |
| channel "Red", in a capital is refused with a sentence, and nothing changes | Invert's channel is "rgb", "red", "green", "blue" or "alpha", and this is "Red". | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Invert's amount runs from 0 to 100, and this is 150. | yes |
| channel alpha at amount 0, the bottom, is taken | taken | yes |
| channel blue at amount 100, the top, is taken | taken | yes |
| amount keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_invert_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_invert_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_invert_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

78 of 78 checks pass.
