# B-58: drop shadow

D-115, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/drop_shadow/expected_drop_shadow.json`, written by `tools/drop_shadow_reference.py` before this code existed and printed in document 25 as FX-SHADOW-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SHADOW-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHADOW-001 frame 0: The defaults: black at 50 per cent, direction 135, distance 5, softness 0. The card's shadow falls down and to the right, 3.54 pixels each way, spread by the bilinear sample over the pixels it falls between; where it lies wholly under the card's shape it is black at half covering. The card itself, fully covered, is unchanged, and the shadow's lower part falls below the frame. | largest difference 1.9e-7 | yes |
| FX-SHADOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-002 frame 0: Direction 0, distance 2: the shadow sits exactly two pixels above the card, black at half covering in rows 1 and 2; the soft pixel's shadow, at (10, 2), is half its covering of 128/255. | largest difference 1.9e-7 | yes |
| FX-SHADOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-003 frame 0: Direction 90, distance 2: exactly two pixels right. The soft pixel, with the card's shadow behind it, takes the shadow through its uncovered share: its covering goes from 0.502 to 0.751, its colour unchanged, as the shadow is black. | largest difference 1.9e-7 | yes |
| FX-SHADOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-004 frame 0: Direction 180, distance 2: exactly two pixels below, in rows 7 and 8, and the soft pixel's, at (10, 6), is half its covering. | largest difference 1.9e-7 | yes |
| FX-SHADOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-005 frame 0: Direction 270, distance 2: exactly two pixels left, in columns 3 and 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-006 frame 0: Direction -270 is a quarter turn clockwise from up, the same as 90: FX-SHADOW-003 exactly. | largest difference 1.9e-7 | yes |
| FX-SHADOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-007 frame 0: Distance 0, softness 0: the shadow sits exactly behind the card, so it shows only through the soft pixel, whose covering goes from 0.502 to 0.627; nothing else changes and the layer does not grow. | largest difference 1.9e-7 | yes |
| FX-SHADOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-008 frame 0: Distance 0, softness 3: the shadow, blurred at sigma 1, spreads three pixels out from the card on every side, darkest next to it and fading outward; column 1 and beyond stay empty. | largest difference 1.9e-7 | yes |
| FX-SHADOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-009 frame 0: Softness 6 at the default direction and distance: the shadow is FX-SHADOW-001's, blurred at sigma 2, so its darkest pixel is lighter than half covering and it reaches further. | largest difference 1.9e-7 | yes |
| FX-SHADOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-010 frame 0: Opacity 0: no shadow; the frame is the drawing, untouched, though the layer still grows. | largest difference 1.9e-7 | yes |
| FX-SHADOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-011 frame 0: Colour #2040a0, opacity 100, distance 2: a blue shadow at full covering where it lies wholly under the card's shape, its straight colour exactly #2040a0. | largest difference 1.9e-7 | yes |
| FX-SHADOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-012 frame 0: FX-SHADOW-011 with the colour written in capitals: the same. | largest difference 1.9e-7 | yes |
| FX-SHADOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-013 frame 0: Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four pixels right. | largest difference 1.9e-7 | yes |
| FX-SHADOW-013 frame 2: Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four pixels right. | largest difference 1.9e-7 | yes |
| FX-SHADOW-013 frame 4: Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four pixels right. | largest difference 1.9e-7 | yes |
| FX-SHADOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-014 frame 0: Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and frame 4 is FX-SHADOW-004. | largest difference 1.9e-7 | yes |
| FX-SHADOW-014 frame 2: Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and frame 4 is FX-SHADOW-004. | largest difference 1.9e-7 | yes |
| FX-SHADOW-014 frame 4: Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and frame 4 is FX-SHADOW-004. | largest difference 1.9e-7 | yes |
| FX-SHADOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-015 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-015 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-015 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-016 frame 0: FX-SHADOW-001 moved three pixels right: the same, moved, and the shadow's right end, which would fall in columns 16 and 17, is cut off by the frame's edge. | largest difference 1.9e-7 | yes |
| FX-SHADOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-017 frame 0: Direction 0, distance 5: the shadow of the card's two lower rows shows in rows 0 and 1; the rest falls above the frame and is cut off, and row 2 stays empty. | largest difference 1.9e-7 | yes |
| FX-SHADOW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-018 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-018 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-019 frame 0: Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-019 frame 4: Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-020 frame 0: Softness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-020 frame 4: Softness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-021 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-021 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-022 frame 0: A colour written "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-022 frame 4: A colour written "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-023 frame 0: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-023 frame 4: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| the defaults, distance 5, grow the drawing by 5 on every side | 5 | yes |
| distance 2.5 and softness 6 grow it by 3, the distance rounded up, and 6, the blur's reach | 9 | yes |
| distance 0, softness 0 does not grow it | 0 | yes |
| a half-size draft preview halves the distance and the softness, and nothing else | DropShadow { color: "#000000", opacity: 50.0, direction: 135.0, distance: 2.5, softness: 3.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shadow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_012.json, its colour in capitals, is saved with it in small letters | "#2040a0" | yes |
| a file with no `softness` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a distance written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| opacity 101 is refused with a sentence, and nothing changes | Drop Shadow's opacity runs from 0 to 100, and this is 101. | yes |
| opacity -1 is refused with a sentence, and nothing changes | Drop Shadow's opacity runs from 0 to 100, and this is -1. | yes |
| direction 3601 is refused with a sentence, and nothing changes | Drop Shadow's direction runs from -3600 to 3600, and this is 3601. | yes |
| direction -3601 is refused with a sentence, and nothing changes | Drop Shadow's direction runs from -3600 to 3600, and this is -3601. | yes |
| distance -1 is refused with a sentence, and nothing changes | Drop Shadow's distance runs from 0 to 1000, and this is -1. | yes |
| distance 1001 is refused with a sentence, and nothing changes | Drop Shadow's distance runs from 0 to 1000, and this is 1001. | yes |
| softness 501 is refused with a sentence, and nothing changes | Drop Shadow's softness runs from 0 to 500, and this is 501. | yes |
| colour "black" is refused with a sentence, and nothing changes | Drop Shadow's colour is written #rrggbb, and this is "black". | yes |
| opacity keyed to 150 is refused with a sentence, and nothing changes | Drop Shadow's opacity runs from 0 to 100, and this is 150. | yes |
| opacity 100, distance 1000 and softness 500, the tops, is taken | taken | yes |
| direction -3600, the bottom, is taken | taken | yes |
| distance keyed from 0 to 4 is taken | taken | yes |
| direction keyed from 0 to 180 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shadow_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shadow_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

92 of 92 checks pass.
