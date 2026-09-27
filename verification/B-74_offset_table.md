# B-74: offset

D-131, accepted by the owner on 2026-09-26, the ninth of the second batch of ten. Every expected pixel is `Fixtures/offset/expected_offset.json`, written by `tools/offset_reference.py` before this code existed and printed in document 25 as FX-OFFSET-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-OFFSET-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-OFFSET-001 frame 0: The settings as they start, shift 0, 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-OFFSET-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-002 frame 0: Shift 3, 0: every pixel moves exactly three pixels right, and the three columns that leave the right edge come back in at the left, so the red dot is at (2, 0) and the soft edge down column 1. | largest difference 1.9e-7 | yes |
| FX-OFFSET-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-003 frame 0: Shift 0, 2: every pixel moves exactly two pixels down, and the skin block's two lowest rows come back in at the top, in rows 0 and 1; the red dot is at (15, 2). | largest difference 1.9e-7 | yes |
| FX-OFFSET-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-004 frame 0: Shift -3, -2, left and up: every pixel moves exactly; the white block's left three columns come back in at the right, in columns 13 to 15, and its top row at the bottom, in row 9; the red dot is at (12, 8). | largest difference 1.9e-7 | yes |
| FX-OFFSET-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-005 frame 0: Shift 8, 5, half the drawing each way: the four quarters change places corner to corner, so the red dot, from the top right corner, is at (7, 5) in the middle; the white block, from the left edge, is in columns 8 to 12 and rows 6 to 9; and the skin block, from the bottom right, is in columns 1 to 5 and rows 0 to 4. | largest difference 1.9e-7 | yes |
| FX-OFFSET-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-006 frame 0: Shift 16, 10, one whole width and height: every pixel comes back to its own place, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-OFFSET-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-007 frame 0: Shift 35, -23, more than twice round each way: exactly the same as shift 3, -3, as 35 is two widths and 3, and -23 is three heights up and 7 down again. | largest difference 1.9e-7 | yes |
| FX-OFFSET-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-008 frame 0: Shift 100000, -100000, the most each way: 100000 is a whole number of widths (6250) and of heights (10000), so the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-OFFSET-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-009 frame 0: Shift -99999, 99999: thousands of times round, and exactly the same as shift 1, -1. | largest difference 1.9e-7 | yes |
| FX-OFFSET-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-010 frame 0: Shift 0.5, 0, half a pixel right: every pixel is the even mix of itself and the one to its left, column 0 mixing with column 15 round the wrap, so the red dot shows half-covering at (0, 0) and (15, 0), and the soft edge spreads to column 15 at about a quarter covering. | largest difference 1.9e-7 | yes |
| FX-OFFSET-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-011 frame 0: Shift -2.25, 1.75, a part of a pixel both ways: every pixel is a mix of four, with weights 9/16, 3/16, 3/16 and 1/16, wrapped; nothing is lost and nothing gained, each channel's total over the frame is the drawing's. | largest difference 1.9e-7 | yes |
| FX-OFFSET-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-012 frame 0: Shift -31.5, 20.25, part-pixel shifts nearly twice round: exactly the same as shift 0.5, 0.25. | largest difference 1.9e-7 | yes |
| FX-OFFSET-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-013 frame 0: Shift keyed from 0, 0 at frame 0 to 8, 4 at frame 4, linear: frame 0 untouched, frame 2 is shift 4, 2, and frame 4 is shift 8, 4, each moved exactly. | largest difference 1.9e-7 | yes |
| FX-OFFSET-013 frame 2: Shift keyed from 0, 0 at frame 0 to 8, 4 at frame 4, linear: frame 0 untouched, frame 2 is shift 4, 2, and frame 4 is shift 8, 4, each moved exactly. | largest difference 1.9e-7 | yes |
| FX-OFFSET-013 frame 4: Shift keyed from 0, 0 at frame 0 to 8, 4 at frame 4, linear: frame 0 untouched, frame 2 is shift 4, 2, and frame 4 is shift 8, 4, each moved exactly. | largest difference 1.9e-7 | yes |
| FX-OFFSET-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-014 frame 0: Shift keyed from 0, 0 at frame 0 to 3, 1 at frame 4, linear: frame 2 falls between whole pixels, at 1.5, 0.5, and is the plain case of that shift, softened by the mix: the red dot is spread over four pixels, a quarter covering at (1, 0). | largest difference 1.9e-7 | yes |
| FX-OFFSET-014 frame 2: Shift keyed from 0, 0 at frame 0 to 3, 1 at frame 4, linear: frame 2 falls between whole pixels, at 1.5, 0.5, and is the plain case of that shift, softened by the mix: the red dot is spread over four pixels, a quarter covering at (1, 0). | largest difference 1.9e-7 | yes |
| FX-OFFSET-014 frame 4: Shift keyed from 0, 0 at frame 0 to 3, 1 at frame 4, linear: frame 2 falls between whole pixels, at 1.5, 0.5, and is the plain case of that shift, softened by the mix: the red dot is spread over four pixels, a quarter covering at (1, 0). | largest difference 1.9e-7 | yes |
| FX-OFFSET-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-015 frame 0: Shift keyed from 0, 0 at frame 0 to 100000, 0 at frame 4, eased past its end (132500 at frame 2): frame 2 is held at 100000, a whole number of widths, so it is the drawing untouched, as frame 4 is; unheld it would have slid four pixels. | largest difference 1.9e-7 | yes |
| FX-OFFSET-015 frame 2: Shift keyed from 0, 0 at frame 0 to 100000, 0 at frame 4, eased past its end (132500 at frame 2): frame 2 is held at 100000, a whole number of widths, so it is the drawing untouched, as frame 4 is; unheld it would have slid four pixels. | largest difference 1.9e-7 | yes |
| FX-OFFSET-015 frame 4: Shift keyed from 0, 0 at frame 0 to 100000, 0 at frame 4, eased past its end (132500 at frame 2): frame 2 is held at 100000, a whole number of widths, so it is the drawing untouched, as frame 4 is; unheld it would have slid four pixels. | largest difference 1.9e-7 | yes |
| FX-OFFSET-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-016 frame 0: Shift 3, 0, the layer moved three pixels right: FX-OFFSET-002 moved; the wrap stays inside the layer's own pixels, so the three columns left of it stay empty. | largest difference 1.9e-7 | yes |
| FX-OFFSET-016 frame 3: Shift 3, 0, the layer moved three pixels right: FX-OFFSET-002 moved; the wrap stays inside the layer's own pixels, so the three columns left of it stay empty. | largest difference 1.9e-7 | yes |
| FX-OFFSET-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-017 frame 0: A drop shadow two pixels right, then shift 16, 0: the shadow grew the layer two pixels on every side, to 20 by 14, and the wrap goes round that, so 16 is not a whole width: it is shift -4 on the grown layer, and the shadow and the drawing slide four pixels left. | largest difference 1.9e-7 | yes |
| FX-OFFSET-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-018 frame 0: A drop shadow two pixels right, then shift 20, 14, the grown layer's whole width and height: the drawing with its shadow, as the drop shadow alone draws it. | largest difference 1.9e-7 | yes |
| FX-OFFSET-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OFFSET-019 frame 0: Shift 100001, 0: x above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-019 frame 4: Shift 100001, 0: x above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OFFSET-020 frame 0: Shift 0, -100001: y below -100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-020 frame 4: Shift 0, -100001: y below -100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OFFSET-021 frame 0: Shift 100000.5, 0: x past 100000 by half a pixel. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-021 frame 4: Shift 100000.5, 0: x past 100000 by half a pixel. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OFFSET-022 frame 0: Shift keyed to 150000, 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-022 frame 4: Shift keyed to 150000, 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OFFSET-023 frame 0: Shift keyed to 0, -100001 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-023 frame 4: Shift keyed to 0, -100001 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OFFSET-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: what leaves one edge comes in at the other | 0 | yes |
| a half-size draft preview slides by half: shift 40, -12 becomes 20, -6 | Offset { shift: [20.0, -6.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_offset_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_offset_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `shift` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a shift of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shift 100001, 0 is refused with a sentence, and nothing changes | Offset's shift runs from -100000 to 100000, and this is 100001. | yes |
| shift 0, -100001 is refused with a sentence, and nothing changes | Offset's shift runs from -100000 to 100000, and this is -100001. | yes |
| shift 100000.5, 0 is refused with a sentence, and nothing changes | Offset's shift runs from -100000 to 100000, and this is 100000.5. | yes |
| shift keyed to 150000, 0 is refused with a sentence, and nothing changes | Offset's shift runs from -100000 to 100000, and this is 150000. | yes |
| shift 100000, -100000, the most, is taken | taken | yes |
| shift -2.25, 1.75, parts of a pixel, is taken | taken | yes |
| shift keyed from 0, 0 to 8, 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_offset_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_offset_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

82 of 82 checks pass.
