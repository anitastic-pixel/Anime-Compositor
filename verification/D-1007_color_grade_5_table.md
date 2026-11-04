# D-1007: Color Grade, the fifth unit

B-887: HSL Secondary's Show Key as Color/Gray and Color/Black (the colour where keyed, over grey or black elsewhere). Every expected pixel is `Fixtures/color_grade_5/expected_color_grade_5.json`, written by `tools/color_grade_5_reference.py` before this code existed and printed in document 25 as FX-GRADE5-001 to 009.

## FX-GRADE5-001 to 009 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADE5-001 frame 0: The red key (hue 0, range 20) shown Color/Gray: the reds in their own colour, everything else flat 50% grey. | largest difference 1.2e-7 | yes |
| FX-GRADE5-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-002 frame 0: The same key shown Color/Black: the reds in their own colour, everything else black. | largest difference 1.2e-7 | yes |
| FX-GRADE5-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-003 frame 0: The red key with hue softness 30, Color/Gray: the warm tones part colour, part grey, as much colour as the key takes. | largest difference 5.8e-7 | yes |
| FX-GRADE5-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-004 frame 0: The red key inverted, Color/Black: everything but the reds in colour, the reds black. | largest difference 1.9e-7 | yes |
| FX-GRADE5-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-005 frame 0: Color/Gray with a correction set (secondary saturation 0) and a dark vignette: neither shows, only the colours going in and the key. | largest difference 1.2e-7 | yes |
| FX-GRADE5-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-006 frame 0: The red key with key blur 20, Color/Black: the key softened as D-571's White/Black shows it, the colour fading out with it. | largest difference 3.0e-8 | yes |
| FX-GRADE5-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-007 frame 0: A skin key (hue 25, range 10, softness 20; saturation 20 to 80, lightness 30 to 90, softness 10), Color/Gray, after a warmer white balance (temperature 30): the colours going in are the warmer ones. | largest difference 3.5e-6 | yes |
| FX-GRADE5-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-008 frame 0: The red key shown White/Black ("mask", D-398's word, unchanged): D-571's own picture. | largest difference 3.0e-8 | yes |
| FX-GRADE5-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADE5-009 frame 0: Show Key "color", not one of the four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE5-009 frame 4: Show Key "color", not one of the four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADE5-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade5_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grade5_009.json, key view "color", is refused in a sentence naming the key view | Color Grade's key view is "off", "mask", "color_gray" or "color_black", and this is "color". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| key view "Color/Gray" is refused with a sentence, and nothing changes | Color Grade's key view is "off", "mask", "color_gray" or "color_black", and this is "Color/Gray". | yes |
| key view color_black is taken | taken | yes |
| key view mask is taken | taken | yes |
| key view off is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade5_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade5_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade5_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grade5_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grade5_001.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_002.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_003.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_004.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_005.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_006.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_007.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_008.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_grade5_009.json, frames 0 to 2 at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |

## Pictures: in `verification/D-1007 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_white_black.png, White/Black, D-398's: the key as grey; draws cleanly | [] | yes |
| 3_color_gray.png, Color/Gray: the warm colours as they are, the rest mid grey; draws cleanly | [] | yes |
| 4_color_black.png, Color/Black: the warm colours as they are, the rest black; draws cleanly | [] | yes |
| where the key is 0, Color/Gray is mid grey and Color/Black black; where it is whole, both are the street's own colour | 113580 pixels outside, 16020 inside; grey true, black true, the street's own true; Color/Gray differs from White/Black in 129600 pixels | yes |

## Result

51 of 51 checks pass.
