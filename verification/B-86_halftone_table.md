# B-86: halftone

D-143, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the tenth of the third batch. Every expected pixel is `Fixtures/halftone/expected_halftone.json`, written by `tools/halftone_reference.py` before this code existed and printed in document 25 as FX-HALFTONE-001 to 025. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-HALFTONE-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HALFTONE-001 frame 0: The settings as they start: size 8, angle 45, ink #000000, paper #ffffff, amount 100. Every pixel that shows turns black or white at its own covering, the soft edge at half covering; the dots grow with the darkness: the skin keeps a few small black dots, 4 of its 56 pixels right of the soft edge, the shadow larger ones, 13 of its 30, the line is black but for one pixel at a cell's corner, (6, 6), and along the ramp white is all paper and black all ink; the empty corner stays empty. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-002 frame 0: Amount 50: each pixel half way from its own colour to the black or white FX-HALFTONE-001 gives it, in linear values. | largest difference 1.1e-7 | yes |
| FX-HALFTONE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-003 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-004 frame 0: Size 4: cells half as wide, so four times as many dots, each half as wide. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-005 frame 0: Size 2: the finest screen, a dot every two pixels. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-006 frame 0: Angle 0: the cells square to the frame, so the screen repeats every 8 pixels across: in each band, pixel (x, y) and pixel (x + 8, y) are decided alike. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-007 frame 0: Angle 90: the square screen turned a quarter onto itself, the same as angle 0, FX-HALFTONE-006. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-008 frame 0: Ink #1e1a24, the line's colour, on paper #f6d6be, the skin's: the same dots as FX-HALFTONE-001 in sepia, each ink pixel the line's colour and each paper pixel the skin's, so the skin's paper pixels are unchanged. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-009 frame 0: FX-HALFTONE-008 with the colours written in capitals, #1E1A24 and #F6D6BE: the same. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-010 frame 0: Ink #ffffff on paper #000000, the two swapped: every pixel that shows is the opposite of FX-HALFTONE-001's, white dots on black. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-011 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-HALFTONE-002 and frame 4 FX-HALFTONE-001. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-011 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-HALFTONE-002 and frame 4 FX-HALFTONE-001. | largest difference 1.1e-7 | yes |
| FX-HALFTONE-011 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-HALFTONE-002 and frame 4 FX-HALFTONE-001. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-012 frame 0: Size keyed from 4 at frame 0 to 8 at frame 4, linear: the dots swell, frame 0 FX-HALFTONE-004, frame 2 size 6 and frame 4 FX-HALFTONE-001. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-012 frame 2: Size keyed from 4 at frame 0 to 8 at frame 4, linear: the dots swell, frame 0 FX-HALFTONE-004, frame 2 size 6 and frame 4 FX-HALFTONE-001. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-012 frame 4: Size keyed from 4 at frame 0 to 8 at frame 4, linear: the dots swell, frame 0 FX-HALFTONE-004, frame 2 size 6 and frame 4 FX-HALFTONE-001. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-013 frame 0: Angle keyed from 0 at frame 0 to 90 at frame 4, linear: the screen turns, frame 0 FX-HALFTONE-006, frame 2 angle 45, FX-HALFTONE-001, and frame 4 FX-HALFTONE-007. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-013 frame 2: Angle keyed from 0 at frame 0 to 90 at frame 4, linear: the screen turns, frame 0 FX-HALFTONE-006, frame 2 angle 45, FX-HALFTONE-001, and frame 4 FX-HALFTONE-007. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-013 frame 4: Angle keyed from 0 at frame 0 to 90 at frame 4, linear: the screen turns, frame 0 FX-HALFTONE-006, frame 2 angle 45, FX-HALFTONE-001, and frame 4 FX-HALFTONE-007. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-014 frame 0: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-HALFTONE-001; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-014 frame 2: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-HALFTONE-001; frame 0 is the drawing. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-015 frame 0: FX-HALFTONE-001 moved three pixels right: the same, moved; the screen moves with the drawing. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-015 frame 3: FX-HALFTONE-001 moved three pixels right: the same, moved; the screen moves with the drawing. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-016 frame 0: A directional blur, direction 90 and length 4, then the halftone as it starts: the blur grew the layer two pixels on every side, and the screen is still the drawing's own, fixed to its own top left corner, not the grown layer's, and the blurred pixels, the empty corner now partly covered among them, are screened at their own covering. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-017 frame 0: Angle 405, a whole turn past 45: the same as FX-HALFTONE-001. | largest difference 3.0e-8 | yes |
| FX-HALFTONE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALFTONE-018 frame 0: Size 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-018 frame 4: Size 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-019 frame 0: Size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-019 frame 4: Size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-020 frame 0: Angle 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-020 frame 4: Angle 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-021 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-021 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-022 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-022 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-023 frame 0: Ink "black", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-023 frame 4: Ink "black", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-024 frame 0: Paper "#fffff", five digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-024 frame 4: Paper "#fffff", five digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HALFTONE-025 frame 0: Size keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-025 frame 4: Size keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HALFTONE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the size, 8 to 4, and nothing else | Halftone { size: 4.0, angle: 45.0, ink: "#000000", paper: "#ffffff", amount: 100.0 } | yes |
| a quarter-size draft of size 3 holds the size at its smallest, 2, rather than dropping the effect | Halftone { size: 2.0, angle: 45.0, ink: "#000000", paper: "#ffffff", amount: 100.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_halftone_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halftone_009.json's colours, written #1E1A24 and #F6D6BE, are saved in small letters | "#1e1a24" and "#f6d6be" | yes |
| a file with no `paper` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a size that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 1 is refused with a sentence, and nothing changes | Halftone's size runs from 2 to 200, and this is 1. | yes |
| size 201 is refused with a sentence, and nothing changes | Halftone's size runs from 2 to 200, and this is 201. | yes |
| angle 3601 is refused with a sentence, and nothing changes | Halftone's angle runs from -3600 to 3600, and this is 3601. | yes |
| amount 101 is refused with a sentence, and nothing changes | Halftone's amount runs from 0 to 100, and this is 101. | yes |
| the ink "black" is refused with a sentence, and nothing changes | Halftone's ink is written #rrggbb, and this is "black". | yes |
| the paper "#fffff" is refused with a sentence, and nothing changes | Halftone's paper is written #rrggbb, and this is "#fffff". | yes |
| size keyed to 250 is refused with a sentence, and nothing changes | Halftone's size runs from 2 to 200, and this is 250. | yes |
| size 2, angle -3600 and amount 0, the bottoms, is taken | taken | yes |
| size 200, angle 3600 and amount 100, the tops, is taken | taken | yes |
| angle keyed from 0 to 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_halftone_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_halftone_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_halftone_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

97 of 97 checks pass.
