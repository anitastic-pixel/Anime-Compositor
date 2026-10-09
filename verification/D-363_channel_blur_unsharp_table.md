# D-363: Channel Blur and Unsharp Mask, checked against fixtures

Channel Blur (D-312/D-313) and Unsharp Mask's Threshold (D-317) had hand-worked tests only. After Effects' Channel Blur has Red, Green, Blue and Alpha Blurriness, Repeat Edge Pixels and Blur Dimensions, all built; its Unsharp Mask has Amount, Radius and Threshold, all built. Every expected pixel is `Fixtures/channel_blur/expected_channel_blur.json` (`tools/channel_blur_reference.py`) or `Fixtures/sharpen/expected_sharpen_threshold.json` (`tools/unsharp_threshold_reference.py`), independent double-precision workings, printed in document 25 as FX-CHBLUR-001 to 014 and FX-SHARPEN-019 to 030. Tolerance 2e-5.

## FX-CHBLUR-001 to 014 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHBLUR-001 frame 0: Red Blurriness 3, the rest 0: only red is blurred, and laid back inside the drawing's own covering, so green, blue and every covering are the drawing's exactly and the empty pixels stay empty. Red softens across the join of orange and green, the orange side losing red and the green side gaining it; a square of one colour, as the half-covered blue one, has no other red within reach and stays as it is, its edge against the clear being no edge to a blur divided by its own covering. | largest difference 1.0e-7 | yes |
| FX-CHBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-002 frame 0: Alpha Blurriness 3, the rest 0: the covering spreads as Gaussian Blur spreads it, and every pixel the drawing covered keeps its own straight colour; past the drawing the spread takes the colour the blurred covering carries, green beside green and blue beside blue, never a dark rim. | largest difference 5.8e-8 | yes |
| FX-CHBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-003 frame 0: All four 3: Gaussian Blur at sigma 3, every channel blurred together. | largest difference 5.2e-8 | yes |
| FX-CHBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-004 frame 0: Red 2, green 0, blue 5, alpha 1: each colour spread by its own amount and laid inside a covering spread by one pixel. Where orange meets green, red and blue soften across the join, blue the widest, so the orange takes a little blue and loses some red, while green stays as sharp as drawn. | largest difference 1.5e-7 | yes |
| FX-CHBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-005 frame 0: FX-CHBLUR-004 with Repeat Edge Pixels: past the left edge the orange square's own pixels are read, so its left column keeps its covering and its orange; the layer does not grow. | largest difference 1.4e-7 | yes |
| FX-CHBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-006 frame 0: FX-CHBLUR-004 with Blur Dimensions horizontal: spread across only, the rows above and below the squares still empty. | largest difference 8.8e-8 | yes |
| FX-CHBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-007 frame 0: Red 4 and alpha 2 with Blur Dimensions vertical and Repeat Edge Pixels: spread down only, the columns between the green and blue squares still empty. | largest difference 7.2e-8 | yes |
| FX-CHBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-008 frame 0: All four 0, as it starts: the drawing untouched. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-009 frame 0: Green 1.5 and alpha 0.5: a part of a pixel, each Gaussian cut at three of its sigmas, 5 pixels and 2. | largest difference 1.5e-7 | yes |
| FX-CHBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-010 frame 0: Red Blurriness keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 untouched, frame 2 red 3, FX-CHBLUR-001, and frame 4 red 6. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-010 frame 2: Red Blurriness keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 untouched, frame 2 red 3, FX-CHBLUR-001, and frame 4 red 6. | largest difference 1.0e-7 | yes |
| FX-CHBLUR-010 frame 4: Red Blurriness keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 untouched, frame 2 red 3, FX-CHBLUR-001, and frame 4 red 6. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-011 frame 0: Red Blurriness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-011 frame 4: Red Blurriness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHBLUR-012 frame 0: Alpha Blurriness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-012 frame 4: Alpha Blurriness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHBLUR-013 frame 0: Edges "Repeat": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-013 frame 4: Edges "Repeat": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHBLUR-014 frame 0: Blur Dimensions "diagonal", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-014 frame 4: Blur Dimensions "diagonal", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Channel Blur: the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chblur_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Saved with its four blurrinesses, edges and dimensions | {"alpha_blurriness":2,"blue_blurriness":0,"dimensions":"vertical","edges":"repeat","green_blurriness":0,"red_blurriness":4} | yes |

## Channel Blur: commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red 501 is refused with a sentence, and nothing changes | Channel Blur's red blurriness runs from 0 to 500, and this is 501. | yes |
| alpha -1 is refused with a sentence, and nothing changes | Channel Blur's alpha blurriness runs from 0 to 500, and this is -1. | yes |
| Repeat Edge Pixels written "Repeat" is refused with a sentence, and nothing changes | Channel Blur's edges are "transparent" or "repeat", and this is "Repeat". | yes |
| Blur Dimensions "diagonal" is refused with a sentence, and nothing changes | Channel Blur's dimensions are "both", "horizontal" or "vertical", and this is "diagonal". | yes |
| FX-CHBLUR-008 set to red 2, blue 5, alpha 1, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-CHBLUR-008 set to red 2, blue 5, alpha 1 by the command draws FX-CHBLUR-004's frame | largest difference 1.5e-7 | yes |
| FX-CHBLUR-008 with red keyed 0 to 6 over frames 0 to 4 by the command draws FX-CHBLUR-010's frame 2 | largest difference 1.0e-7 | yes |

## Channel Blur: the preview

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chblur_001.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_chblur_004.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_chblur_005.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_chblur_006.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_chblur_007.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_chblur_009.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |

## Channel Blur: the frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chblur_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chblur_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## FX-SHARPEN-019 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHARPEN-019 frame 0: Threshold 0 written in the file: FX-SHARPEN-001, every pixel. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-020 frame 0: Threshold 12: the channels nearer their blur than 12 levels, a few inside the box, keep the drawing's value; every other channel is FX-SHARPEN-001's. | largest difference 2.3e-7 | yes |
| FX-SHARPEN-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-021 frame 0: Threshold 30: more channels kept, the skin inside the box among them; the line is still pushed to black. | largest difference 2.3e-7 | yes |
| FX-SHARPEN-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-022 frame 0: Amount 200, threshold 110: only the hardest edges, the line against the skin, are crisped, twice as hard as FX-SHARPEN-001; the band and its skin keep their colours. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-023 frame 0: Threshold 255: no channel is 255 levels from its blur, so the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-024 frame 0: The grain drawing, threshold 0: the grain is crisped along with the line, every pixel changing. | largest difference 4.0e-7 | yes |
| FX-SHARPEN-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-025 frame 0: The grain drawing, threshold 16: the grain, under 16 levels from its blur, keeps the drawing's values exactly; the line and the skin beside it are crisped as in FX-SHARPEN-024. | largest difference 2.5e-7 | yes |
| FX-SHARPEN-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-026 frame 0: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 FX-SHARPEN-001, frame 2 threshold 20, frame 4 threshold 40, each keeping more. | largest difference 3.1e-7 | yes |
| FX-SHARPEN-026 frame 2: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 FX-SHARPEN-001, frame 2 threshold 20, frame 4 threshold 40, each keeping more. | largest difference 2.3e-7 | yes |
| FX-SHARPEN-026 frame 4: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 FX-SHARPEN-001, frame 2 threshold 20, frame 4 threshold 40, each keeping more. | largest difference 2.3e-7 | yes |
| FX-SHARPEN-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-027 frame 0: The grain drawing, amount 300, radius 2, threshold 10, moved three pixels right: the grain still kept, the line's halo wider and harder. | largest difference 3.6e-7 | yes |
| FX-SHARPEN-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHARPEN-028 frame 0: Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-028 frame 4: Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-029 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-029 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHARPEN-030 frame 0: Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-030 frame 4: Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHARPEN-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Unsharp Mask: the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sharpen_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sharpen_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Saved with its amount, radius and threshold | {"amount":200,"radius":1,"threshold":110} | yes |

## Unsharp Mask: commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| threshold 256 is refused with a sentence, and nothing changes | Sharpen's threshold runs from 0 to 255, and this is 256. | yes |
| threshold -1 is refused with a sentence, and nothing changes | Sharpen's threshold runs from 0 to 255, and this is -1. | yes |
| FX-SHARPEN-019 set to threshold 30, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-SHARPEN-019 set to threshold 30 by the command draws FX-SHARPEN-021's frame | largest difference 2.3e-7 | yes |

## Unsharp Mask: the preview (a threshold keeps it on the processor, D-317)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sharpen_019.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_sharpen_020.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_sharpen_025.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_sharpen_027.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 1 of 255 | yes |

## Unsharp Mask: the frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sharpen_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sharpen_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a street, in `verification/D-363 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_channel_blur_red_6.png, Channel Blur, red 6 only, repeat edges: red fringes, green and blue sharp; draws cleanly | [] | yes |
| 2_channel_blur_blue_6.png, Channel Blur, blue 6 only: blue fringes; draws cleanly | [] | yes |
| 3_channel_blur_all_6.png, Channel Blur, all four 6: a plain blur; draws cleanly | [] | yes |
| 4_unsharp_threshold_0.png, the grainy street, Unsharp Mask amount 300, radius 2, threshold 0: the grain crisped into noise; draws cleanly | [] | yes |
| 5_unsharp_threshold_16.png, the same with threshold 16: the grain left as it was, the windows and road markings crisped; draws cleanly | [] | yes |
| In the sky, threshold 0 pushes the grain apart; threshold 16 leaves it within 1 level | largest change along row 20: threshold 0 18 of 255, threshold 16 0 | yes |

## Result

109 of 109 checks pass.
