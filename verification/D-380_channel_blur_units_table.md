# D-380: Channel Blur in After Effects' Blurriness

The owner chose option (c) on 2026-10-09 ("c"): Channel Blur gains Units exactly as Gaussian Blur did under D-321. Blurriness (After Effects), which a new Channel Blur takes, reads each of Red, Green, Blue and Alpha Blurriness as After Effects does, sigma 0.3 times it, its kernel reaching 6.5 sigmas; Sigma (older projects), what a file without units means, is document 21's rule as before, so a saved project draws as it did. Every expected pixel is `Fixtures/channel_blur/expected_channel_blur_blurriness.json` (FX-CHBLUR-015 to 024) or `expected_channel_blur.json` (FX-CHBLUR-001 to 014, unchanged), written by `tools/channel_blur_reference.py` before the build had the units. Tolerance 2e-5.

## FX-CHBLUR-001 to 014, the old file, unchanged (document 25)

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

## FX-CHBLUR-015 to 024, Blurriness (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHBLUR-015 frame 0: Units Blurriness (After Effects), Red Blurriness 10, the rest 0: red at sigma 3, reaching 20 pixels. As FX-CHBLUR-001, only red is blurred and laid back inside the drawing's own covering, so green, blue and every covering are the drawing's exactly. The red is FX-CHBLUR-001's within a billionth: divided by its own covering's blur, the longer tail cancels. | largest difference 1.1e-7 | yes |
| FX-CHBLUR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-016 frame 0: Units Blurriness, red 6, green 0, blue 15, alpha 3 (sigmas 1.8, 0, 4.5 and 0.9): each colour spread by its own amount and laid inside a covering spread by about a pixel; green stays as sharp as drawn. | largest difference 1.5e-7 | yes |
| FX-CHBLUR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-017 frame 0: FX-CHBLUR-016 with Repeat Edge Pixels: the orange square's own pixels are read past the left edge, so its left column keeps its covering; the layer does not grow. | largest difference 1.8e-7 | yes |
| FX-CHBLUR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-018 frame 0: FX-CHBLUR-016 with Blur Dimensions horizontal: spread across only, the rows above and below the squares still empty. | largest difference 1.1e-7 | yes |
| FX-CHBLUR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-019 frame 0: Units Blurriness, red 12 and alpha 6 with Blur Dimensions vertical and Repeat Edge Pixels: spread down only, the columns between the green and blue squares still empty. | largest difference 5.2e-8 | yes |
| FX-CHBLUR-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-020 frame 0: Units Blurriness, all four 10: Gaussian Blur in Blurriness 10 (D-321) exactly, so the two blurs agree at the same number. Ten pixels right of the blue square the covering is still lit, past where sigma 3 cut at three sigmas (FX-CHBLUR-003) leaves it clear. | largest difference 4.1e-8 | yes |
| FX-CHBLUR-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-021 frame 0: Units written "sigma" (Sigma, older projects), Red Blurriness 3: FX-CHBLUR-001 exactly, the file with no units. | largest difference 1.0e-7 | yes |
| FX-CHBLUR-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-022 frame 0: Units Blurriness, Red Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 untouched, frame 2 red 10, FX-CHBLUR-015, and frame 4 red 20. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-022 frame 2: Units Blurriness, Red Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 untouched, frame 2 red 10, FX-CHBLUR-015, and frame 4 red 20. | largest difference 1.1e-7 | yes |
| FX-CHBLUR-022 frame 4: Units Blurriness, Red Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 untouched, frame 2 red 10, FX-CHBLUR-015, and frame 4 red 20. | largest difference 1.7e-7 | yes |
| FX-CHBLUR-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHBLUR-023 frame 0: Units "Blurriness": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-023 frame 4: Units "Blurriness": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHBLUR-024 frame 0: Units "pixels", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-024 frame 4: Units "pixels", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 8.2e-8 | yes |
| FX-CHBLUR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chblur_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chblur_001.json, a Channel Blur with no units as before D-380, is saved without units | {"alpha_blurriness":0,"blue_blurriness":0,"dimensions":"both","green_blurriness":0,"red_blurriness":3} | yes |
| A Channel Blur in Blurriness is saved as units: blurriness | {"alpha_blurriness":0,"blue_blurriness":0,"dimensions":"both","green_blurriness":0,"red_blurriness":10,"units":"blurriness"} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| units "pixels" is refused with a sentence, and nothing changes | Channel Blur's units are "sigma" or "blurriness", and this is "pixels". | yes |
| units "Blurriness", written with a capital is refused with a sentence, and nothing changes | Channel Blur's units are "sigma" or "blurriness", and this is "Blurriness". | yes |
| FX-CHBLUR-001 set to Blurriness 6/0/15/3, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-CHBLUR-001 set to Blurriness 6/0/15/3 by the command draws FX-CHBLUR-016's frame | largest difference 1.5e-7 | yes |
| FX-CHBLUR-015 with Red keyed 0 to 20 by the command draws FX-CHBLUR-022's frame 2 | largest difference 1.1e-7 | yes |

## The preview card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chblur_001.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_001.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_015.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_015.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_016.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_016.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_017.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_017.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_018.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_018.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_019.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_019.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_020.json frame 0 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_020.json frame 0 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_022.json frame 2 at Full: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_chblur_022.json frame 2 at Draft: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |

## Pictures: the street, in `verification/D-380 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| built_1_sigma_red_20.png, Red Blurriness 20 in an old file (Sigma): red smeared far past the houses; draws cleanly | [] | yes |
| built_2_blurriness_red_20.png, Red Blurriness 20 in Blurriness, as After Effects reads it (sigma 6): a red fringe; draws cleanly | [] | yes |
| built_3_sigma_all_10.png, all four at 10 in Sigma: very soft; draws cleanly | [] | yes |
| built_4_blurriness_all_10.png, all four at 10 in Blurriness: softened, the windows still there; draws cleanly | [] | yes |
| built_5_gaussian_blurriness_10.png, Gaussian Blur at Blurriness 10, for comparison; draws cleanly | [] | yes |
| Channel Blur with all four at Blurriness 10 is Gaussian Blur at Blurriness 10, within 1 level | largest difference 0 of 255 | yes |
| Blurriness 20 is not Sigma 20 (the old file keeps the old look) | largest difference 53 of 255 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chblur_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chblur_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chblur_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chblur_022.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

100 of 100 checks pass.
