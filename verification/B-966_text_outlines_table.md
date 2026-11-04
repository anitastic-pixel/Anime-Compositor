# B-966: Create Shapes from Text and Create Masks from Text (D-1086)

Written by `tests/b966_text_outlines.rs`. Each case against `Fixtures/text_outlines/expected_text_outlines.json` from `tools/text_outlines_reference.py`: every character's contours, point by point and handle by handle, which are holes, and the shapes and masks the two commands make.

## The outlines

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TXO-001: Plain words: one group per letter, no group for the space; T, x and t have one contour each, e two (its eye a hole). Characters with an outline | Textext | yes |
| FX-TXO-001: every contour's points and handles | at most 5.71e-14 px out | yes |
| FX-TXO-001: which contours are holes | 2 holes | yes |
| FX-TXO-001: each character's colour | at most 0.00e0 out | yes |
| FX-TXO-001: Create Masks from Text's masks, names, modes and points in order | T add, e add, e subtract, x add, t add, e add, e subtract, x add, t add | yes |
| FX-TXO-001: Create Shapes from Text's groups and paths (each group a Merge Paths, Merge) | 7 groups, paths [1, 2, 1, 1, 2, 1, 1] | yes |
| FX-TXO-001: the shape layer draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000021 | yes |
| FX-TXO-001: the masked solid draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000021 | yes |
| FX-TXO-002: Holes: O and 8 and B, each outer contour Add and each inside Subtract. Characters with an outline | O8B | yes |
| FX-TXO-002: every contour's points and handles | at most 2.84e-14 px out | yes |
| FX-TXO-002: which contours are holes | 5 holes | yes |
| FX-TXO-002: each character's colour | at most 0.00e0 out | yes |
| FX-TXO-002: Create Masks from Text's masks, names, modes and points in order | O add, O subtract, 8 add, 8 subtract, 8 subtract, B add, B subtract, B subtract | yes |
| FX-TXO-002: Create Shapes from Text's groups and paths (each group a Merge Paths, Merge) | 3 groups, paths [2, 3, 3] | yes |
| FX-TXO-002: the shape layer draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000009 | yes |
| FX-TXO-002: the masked solid draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000009 | yes |
| FX-TXO-003: Centred, two lines: the second line a line lower, each centred on the place. Characters with an outline | ABCD | yes |
| FX-TXO-003: every contour's points and handles | at most 1.14e-13 px out | yes |
| FX-TXO-003: which contours are holes | 4 holes | yes |
| FX-TXO-003: each character's colour | at most 0.00e0 out | yes |
| FX-TXO-003: Create Masks from Text's masks, names, modes and points in order | A add, A subtract, B add, B subtract, B subtract, C add, D add, D subtract | yes |
| FX-TXO-003: Create Shapes from Text's groups and paths (each group a Merge Paths, Merge) | 4 groups, paths [2, 3, 1, 2] | yes |
| FX-TXO-003: the shape layer draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000010 | yes |
| FX-TXO-003: the masked solid draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000010 | yes |
| FX-TXO-004: A Text Animator as it is on the frame: Rotation 30, Position 10, -20 and a red fill on every letter; the outlines are where the letters are drawn. Characters with an outline | Spin | yes |
| FX-TXO-004: every contour's points and handles | at most 2.84e-14 px out | yes |
| FX-TXO-004: which contours are holes | 1 holes | yes |
| FX-TXO-004: each character's colour | at most 0.00e0 out | yes |
| FX-TXO-004: Create Masks from Text's masks, names, modes and points in order | S add, p add, p subtract, i add, i add, n add | yes |
| FX-TXO-004: Create Shapes from Text's groups and paths (each group a Merge Paths, Merge) | 4 groups, paths [1, 2, 2, 1] | yes |
| FX-TXO-004: the shape layer draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000013 | yes |
| FX-TXO-004: the masked solid draws where the words were drawn | 0.000% of pixels differ by more than half, mean 0.000013 | yes |

## The project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_txo_020.json, Create Shapes from Text: "Title Outlines" in front, frames 2 to 20, position Vec2(700.0, 380.0), rotation Scalar(10.0); Title off | order (back to front) ["art", "outlines"] | yes |
| fx_txo_020.json: groups H, i, O with 1, 2 and 2 paths, every one filled light blue | ["H", "i", "O"] [1, 2, 2] | yes |
| fx_txo_020.json, Create Shapes from Text: frame 2 looks as it did | 0.000% of pixels differ by more than half, mean 0.000007 | yes |
| fx_txo_020.json, Create Shapes from Text: one undo puts it back as it was | as it was | yes |
| fx_txo_020.json, Create Shapes from Text: saved and opened again, it draws the same | the same | yes |
| fx_txo_020.json, Create Masks from Text: "Title Outlines" in front, frames 2 to 20, position Vec2(700.0, 380.0), rotation Scalar(10.0); Title off | order (back to front) ["art", "outlines"] | yes |
| fx_txo_020.json: the masks H add, i add, i add, O add, O subtract; the solid in the text's colour, the composition's size | [{"mode":"add","name":"H"},{"mode":"add","name":"i"},{"mode":"add","name":"i"},{"mode":"add","name":"O"},{"mode":"subtract","name":"O"}] | yes |
| fx_txo_020.json, Create Masks from Text: frame 2 looks as it did | 0.000% of pixels differ by more than half, mean 0.000007 | yes |
| fx_txo_020.json, Create Masks from Text: one undo puts it back as it was | as it was | yes |
| fx_txo_020.json, Create Masks from Text: saved and opened again, it draws the same | the same | yes |

## Refused

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_txo_021.json, Create Shapes from Text: words that are only spaces are refused | Title's words have no outline (only spaces), so nothing was made. | yes |
| fx_txo_021.json, Create Masks from Text: words that are only spaces are refused | Title's words have no outline (only spaces), so nothing was made. | yes |
| A layer that is not text is refused | Red Solid is not a text layer, so it has no outlines to make shapes from. | yes |

## Result

45 of 45 checks pass.
