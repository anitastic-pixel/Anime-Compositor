# D-1009: Color Grade, the sixth unit

B-889: a switch per section (Basic Correction, Creative, Curves, Color Wheels, HSL Secondary, Vignette). A section switched off draws as if its settings were at their start. Every expected pixel is `Fixtures/color_grade_6/expected_color_grade_6.json`, written by `tools/color_grade_6_reference.py` before this code existed and printed in document 25 as FX-GRADE6-001 to 011.

## FX-GRADE6-001 to 011 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE6-001 frame 0: A whole grade, every section on and written: D-1007's own picture. | largest difference 2.7e-7 | yes |
| FX-GRADE6-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-002 frame 0: The same with Basic Correction off: no white balance, tone or saturation; the rest as FX-GRADE6-001. | largest difference 3.1e-7 | yes |
| FX-GRADE6-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-003 frame 0: Creative off: no look, faded film, sharpen, vibrance or tints. | largest difference 2.6e-7 | yes |
| FX-GRADE6-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-004 frame 0: Curves off: the S curve and the hue-versus-hue curve not drawn. | largest difference 2.7e-7 | yes |
| FX-GRADE6-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-005 frame 0: Color Wheels off: the shadows' blue push and the midtones' darkening not drawn. | largest difference 2.6e-7 | yes |
| FX-GRADE6-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-006 frame 0: HSL Secondary off: the reds keep their colour. | largest difference 2.7e-7 | yes |
| FX-GRADE6-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-007 frame 0: Vignette off: the corners no darker. | largest difference 2.7e-7 | yes |
| FX-GRADE6-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-008 frame 0: Every section off: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-GRADE6-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-009 frame 0: Show Key Color/Gray with HSL Secondary off: the view goes with the section, and the vignette is drawn again. | largest difference 1.6e-7 | yes |
| FX-GRADE6-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-010 frame 0: A D-1007 file with no switches: every section on, FX-GRADE6-001's picture. | largest difference 2.7e-7 | yes |
| FX-GRADE6-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE6-011 frame 0: Curves' switch "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE6-011 frame 4: Curves' switch "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE6-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade6_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade6_011.json, curves' switch "yes", is refused in a sentence naming the switch | Color Grade's curves switch is "on" or "off", and this is "yes". | yes |
| fx_grade6_010.json, a file from before with no switches, saves with none written | switches in the saved file: 0; in the file read: 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| curves switch "Off" is refused with a sentence, and nothing changes | Color Grade's curves switch is "on" or "off", and this is "Off". | yes |
| curves switch off is taken | taken | yes |
| curves switch on is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade6_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade6_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade6_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade6_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade6_001.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_002.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_003.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_004.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_005.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_006.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_007.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_008.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_009.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_010.json, frames 0 to 2 at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade6_011.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |

## Pictures: in `verification/D-1009 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| the street with no effect draws cleanly | [] | yes |
| 1_all_on.png, warm, contrasty, dark corners; draws cleanly | [] | yes |
| 2_vignette_off.png, the same with Vignette off: the corners as bright as the middle's grade; draws cleanly | [] | yes |
| 3_basic_off.png, Basic Correction off: no warmth or contrast, the wheel's tint and vignette kept; draws cleanly | [] | yes |
| 4_all_off.png, every section off, is the street exactly as it is | largest difference 0 of 255; [] | yes |
| each switch changes the picture | all on against vignette off: 74209 pixels; against basic off: 129600 | yes |

## Result

60 of 60 checks pass.
