# B-965: Path Options, text on a path (D-1085)

Written by `tests/b965_text_path.rs`. Each case's characters against `Fixtures/text_path/expected_text_path.json` from `tools/text_path_reference.py`: where each one's anchor lands and how far it is turned.

## Where each character lands

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TXP-001: Left, First Margin 20, on an open path with a corner: the letters past the corner climb its second piece. | 12 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-001: the frame has ink at every letter | yes | yes |
| FX-TXP-002: Center on the open path. | 6 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-002: the frame has ink at every letter | yes | yes |
| FX-TXP-003: Right, Last Margin 30, on the open path. | 5 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-003: the frame has ink at every letter | yes | yes |
| FX-TXP-004: Force Alignment, margins 10 and 10: the word spreads over the whole path. | 5 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-004: the frame has ink at every letter | yes | yes |
| FX-TXP-005: Perpendicular To Path off: the letters follow the path but stay upright. | 7 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-005: the frame has ink at every letter | yes | yes |
| FX-TXP-006: An open path's ends go on straight: First Margin -80 starts before it, and the last letters run past its end. | 14 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-006: the frame has ink at every letter | yes | yes |
| FX-TXP-007: A closed square, First Margin 1500 of its 1600: the word wraps round its first corner. | 5 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-007: the frame has ink at every letter | yes | yes |
| FX-TXP-008: Reverse Path on the closed square: same first corner, walked the other way. | 7 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-008: the frame has ink at every letter | yes | yes |
| FX-TXP-009: The tutorial's circle (tQ5wqg10x_I at 2:30): an ellipse mask, centred words, First Margin 73.6: inside the ring, just right of the bottom. | 13 characters, at most 0.0014 px and 0.1645 degrees out | yes |
| FX-TXP-009: the frame has ink at every letter | yes | yes |
| FX-TXP-010: The tutorial's circle at 2:40: Reverse Path on, First Margin 1146: outside the ring, just right of the top. | 13 characters, at most 0.0024 px and 0.1685 degrees out | yes |
| FX-TXP-010: the frame has ink at every letter | yes | yes |
| FX-TXP-011: A text animator on a path: Position 15, -20 and Rotation 30 on every letter; x moves along the path, y lifts off it. | 6 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-011: the frame has ink at every letter | yes | yes |
| FX-TXP-012: Two lines: each starts at the path's start, the second a line lower. | 6 characters, at most 0.0000 px and 0.0000 degrees out | yes |
| FX-TXP-012: the frame has ink at every letter | yes | yes |

## Projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_txp_020.json frame 0: First Margin keyed, 0 along | at most 0.0000 px and 0.0000 degrees out | yes |
| fx_txp_020.json frame 10: First Margin keyed, 500 along | at most 0.0000 px and 0.0000 degrees out | yes |
| fx_txp_020.json frame 5: First Margin keyed, 250 along | at most 0.0000 px and 0.0000 degrees out | yes |
| fx_txp_020.json: frames 0 and 5 are drawn differently | they differ | yes |
| fx_txp_021.json: mask 3 on a layer with one is said on the frame | EFFECT_PATH_MISSING | yes |
| fx_txp_021.json: its words are drawn where they are without a path | the same | yes |
| A project from before Path Options opens and draws as before | the same | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_txp_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_txp_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Saved: the six settings, never the mask's pieces | {"first_margin":0,"force_alignment":"off","last_margin":0,"path":3,"perpendicular":"on","reverse":"off"} | yes |
| a file with a First Margin written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no Last Margin is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| Reverse Path written "maybe" is said on opening | EFFECT_PARAMETER_INVALID | yes |

## Result

37 of 37 checks pass.
