# B-810: Minimax

D-930, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/minimax/expected_minimax.json`, written by `tools/minimax_reference.py` before this code existed and printed in document 25 as FX-MINIMAX-001 to 020. The reference searches every window in full; the build runs a row pass then a column pass. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MINIMAX-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MINIMAX-001 frame 0: The settings as they start: Maximum, radius 0, Color, Horizontal & Vertical: nothing changes. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-002 frame 0: Maximum, radius 1, Color: each colour channel becomes the largest of the 3 by 3 square's showing pixels; the covering is untouched, so the white box's black pixel turns white and the shapes keep their outline. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-003 frame 0: Minimum, radius 1, Color: the smallest of each channel; the white box's black pixel spreads to a 3 by 3 black square. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-004 frame 0: Maximum, radius 2, Alpha and Color: every shape grows two pixels each way in a square, the new pixels taking the largest showing colour (F's Max, w12b video 1 at 6:25). | largest difference 1.3e-7 | yes |
| FX-MINIMAX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-005 frame 0: Minimum, radius 1, Alpha: every shape's covering shrinks a pixel each side, the lone yellow pixel and the half-covered column go, and the block on the left loses its pixels along the layer's edges too, since past the edge counts as clear. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-006 frame 0: FX-MINIMAX-005 with Don't Shrink Edges on: the same, but the block keeps its pixels along the layer's left, top and bottom edges. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-007 frame 0: Minimum Then Maximum, radius 1, Alpha and Color: the lone yellow pixel and the half-covered column are taken away and the big shapes come back to their size. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-008 frame 0: Maximum Then Minimum, radius 1, Alpha and Color: the white box's black pixel is filled in and the shapes come back to their size. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-009 frame 0: Maximum, radius 2, Red, Just Horizontal: only red changes, the largest of five along the row. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-010 frame 0: Minimum, radius 1, Green, Just Vertical: only green changes, the smallest of three down the column. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-011 frame 0: Maximum, radius 1, Blue: only blue changes, so the blue column's blue spills into its showing neighbours. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-012 frame 0: Radius keyed from 0 at frame 0 to 4 at frame 4, linear, Maximum, Alpha and Color: the shapes grow a pixel a frame. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-012 frame 2: Radius keyed from 0 at frame 0 to 4 at frame 4, linear, Maximum, Alpha and Color: the shapes grow a pixel a frame. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-012 frame 4: Radius keyed from 0 at frame 0 to 4 at frame 4, linear, Maximum, Alpha and Color: the shapes grow a pixel a frame. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-013 frame 0: FX-MINIMAX-005 with the layer moved three pixels right: the same, moved; the layer's own edges still count. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-014 frame 0: Radius 1.5 is used as 2 (whole pixels, half up), radius 2.4 as 2: both frames equal FX-MINIMAX-004's. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-014 frame 4: Radius 1.5 is used as 2 (whole pixels, half up), radius 2.4 as 2: both frames equal FX-MINIMAX-004's. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MINIMAX-015 frame 0: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-015 frame 4: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MINIMAX-016 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-016 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MINIMAX-017 frame 0: Operation "max": not one of the four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-017 frame 4: Operation "max": not one of the four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MINIMAX-018 frame 0: Channel "rgb": not one of the six words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-018 frame 4: Channel "rgb": not one of the six words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MINIMAX-019 frame 0: Direction "Horizontal": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-019 frame 4: Direction "Horizontal": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MINIMAX-020 frame 0: Don't Shrink Edges "yes": not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-020 frame 4: Don't Shrink Edges "yes": not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-MINIMAX-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: the layer keeps its size | 0 | yes |
| a half-size draft preview halves the radius, 4 to 2 | Minimax { operation: "maximum", radius: 2.0, channel: "color", direction: "both", dont_shrink_edges: "off" } | yes |

## Wrong words are refused (what each warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_minimax_017.json: the warning names the operation | Minimax's operation is "maximum", "minimum", "min_then_max" or "max_then_min", and this is "max". | yes |
| fx_minimax_018.json: the warning names the channel | Minimax's channel is "color", "alpha_and_color", "alpha", "red", "green" or "blue", and this is "rgb". | yes |
| fx_minimax_019.json: the warning names the direction | Minimax's direction is "both", "horizontal" or "vertical", and this is "Horizontal". | yes |
| fx_minimax_020.json: the warning names the don't shrink edges | Minimax's don't shrink edges is "off" or "on", and this is "yes". | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_minimax_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_minimax_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_minimax_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_minimax_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_minimax_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_minimax_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_minimax_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_minimax_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_minimax_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_minimax_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

65 of 65 checks pass.
