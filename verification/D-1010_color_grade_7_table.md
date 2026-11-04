# D-1010: Color Grade, the seventh unit

B-890: HSL Secondary's Sharpen, D-637's sharpen mixed in by the secondary's key after its correction. Every expected pixel is `Fixtures/color_grade_7/expected_color_grade_7.json`, written by `tools/color_grade_7_reference.py` before this code existed and printed in document 25 as FX-GRADE7-001 to 010.

## FX-GRADE7-001 to 010 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE7-001 frame 0: Secondary sharpen 100 on a key of the reds: the reds' edges harder, the rest as drawn. | largest difference 1.9e-7 | yes |
| FX-GRADE7-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-002 frame 0: Secondary sharpen -100 on the reds: the reds softened. | largest difference 1.9e-7 | yes |
| FX-GRADE7-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-003 frame 0: Secondary sharpen 100 with the key as added (every colour): the Creative sharpen 100's picture, FX-GRADE4-002. | largest difference 2.5e-7 | yes |
| FX-GRADE7-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-004 frame 0: The reds' key inverted: every colour but the reds sharpened. | largest difference 2.5e-7 | yes |
| FX-GRADE7-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-005 frame 0: With a correction (secondary saturation 0) and a vignette: the reds grey and sharpened, then the corners darkened. | largest difference 1.6e-7 | yes |
| FX-GRADE7-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-006 frame 0: Keyed from -100 at frame 0 to 100 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE7-006 frame 2: Keyed from -100 at frame 0 to 100 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE7-006 frame 4: Keyed from -100 at frame 0 to 100 at frame 4. | largest difference 1.9e-7 | yes |
| FX-GRADE7-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-007 frame 0: HSL Secondary switched off: the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE7-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-008 frame 0: Show Key White/Black: the key, unchanged by the sharpen. | largest difference 3.0e-8 | yes |
| FX-GRADE7-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-009 frame 0: A D-1009 file without the setting, the reds made grey: D-1009's own picture. | largest difference 1.9e-7 | yes |
| FX-GRADE7-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE7-010 frame 0: Secondary sharpen 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE7-010 frame 4: Secondary sharpen 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE7-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade7_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade7_010.json, secondary sharpen 101, is refused in a sentence naming the setting | Color Grade's secondary sharpen runs from -100 to 100, and this is 101. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| secondary sharpen -101 is refused with a sentence, and nothing changes | Color Grade's secondary sharpen runs from -100 to 100, and this is -101. | yes |
| secondary sharpen -100 is taken | taken | yes |
| secondary sharpen 0 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade7_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade7_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade7_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade7_006.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade7_001.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_002.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_003.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_004.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_005.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_006.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_007.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_008.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_009.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade7_010.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |

## Pictures: in `verification/D-1010 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| the street with no effect draws cleanly | [] | yes |
| 2_sharper.png, Secondary Sharpen 100: the warm buildings' edges harder; draws cleanly | [] | yes |
| 3_softer.png, Secondary Sharpen -100: the warm buildings softened; draws cleanly | [] | yes |
| where the key is 0 both pictures are the street's own colour; inside it the sharpened one differs | 113580 pixels outside the key, unchanged: true; 9098 pixels inside changed; sharper against softer: 9098 pixels | yes |

## Result

55 of 55 checks pass.
