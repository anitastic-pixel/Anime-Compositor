# B-39: radial blur

D-95, accepted by the owner on 2026-09-25; FX-RADIAL-012's directional blur is read by D-98's lines since B-42 the same day. Every expected pixel is `Fixtures/radial_blur/expected_radial_blur.json`, written by `tools/radial_blur_reference.py` before this code existed and printed in document 25 as FX-RADIAL-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RADIAL-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RADIAL-001 frame 0: Spin 30 about the middle: every edge smears round the centre, the further out the longer, and the middle of the block stays solid. | largest difference 2.1e-7 | yes |
| FX-RADIAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-002 frame 0: Zoom 30 about the middle: every edge smears along the line from the centre, the further out the longer. | largest difference 2.4e-7 | yes |
| FX-RADIAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-003 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RADIAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-004 frame 0: Spin 30 about the top left corner, centre 0, 0: the smears are arcs about that corner, so the far corner smears most. | largest difference 1.7e-7 | yes |
| FX-RADIAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-005 frame 0: Zoom 30 about centre 25, 50, the point (4, 5): the block, right of it, smears left and right, and the line, left of it, the other way. | largest difference 2.5e-7 | yes |
| FX-RADIAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-006 frame 0: Spin 100, the most: longer arcs than FX-RADIAL-001. | largest difference 2.5e-7 | yes |
| FX-RADIAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-007 frame 0: Zoom 100, the most: samples from half to one and a half times the distance from the centre. | largest difference 1.9e-7 | yes |
| FX-RADIAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-008 frame 0: Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 untouched, frame 2 spins 20, frame 4 spins 40. | largest difference 1.9e-7 | yes |
| FX-RADIAL-008 frame 2: Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 untouched, frame 2 spins 20, frame 4 spins 40. | largest difference 2.5e-7 | yes |
| FX-RADIAL-008 frame 4: Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 untouched, frame 2 spins 20, frame 4 spins 40. | largest difference 2.5e-7 | yes |
| FX-RADIAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-009 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004. | largest difference 2.1e-7 | yes |
| FX-RADIAL-009 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004. | largest difference 1.9e-7 | yes |
| FX-RADIAL-009 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004. | largest difference 1.7e-7 | yes |
| FX-RADIAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-010 frame 0: Spin 30, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.1e-7 | yes |
| FX-RADIAL-010 frame 3: Spin 30, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.1e-7 | yes |
| FX-RADIAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-011 frame 0: Spin 100 about centre -1000, -1000, far off the top left: every pixel's arc is longer than 255 pixels, so each takes the most samples, 256. | largest difference 3.0e-9 | yes |
| FX-RADIAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-012 frame 0: A directional blur, direction 90 and length 4, then spin 30 about centre 0, 0: the directional blur grew the layer two pixels on every side, and the spin still turns about the drawing's own top left corner. | largest difference 1.7e-7 | yes |
| FX-RADIAL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-013 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-013 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-014 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-014 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-015 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-015 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-016 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-016 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-017 frame 0: Type "twist", which is not a type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-017 frame 4: Type "twist", which is not a type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-018 frame 0: Type "Spin": the word is exact, so a capital is not the type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-018 frame 4: Type "Spin": the word is exact, so a capital is not the type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| the most spin, far off the drawing, does not grow its bounds | 0 | yes |
| a half-size draft preview changes nothing: the amount and the centre are shares | RadialBlur { kind: "zoom", amount: 40.0, center: [20.0, 70.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_radial_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `type` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of three numbers is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a keyed centre whose key holds one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Radial Blur's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Radial Blur's amount runs from 0 to 100, and this is -1. | yes |
| centre -1001, 50 is refused with a sentence, and nothing changes | Radial Blur's center runs from -1000 to 1000, and this is -1001. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Radial Blur's center runs from -1000 to 1000, and this is 1001. | yes |
| type "twist" is refused with a sentence, and nothing changes | Radial Blur's type is "spin" or "zoom", and this is "twist". | yes |
| type "Zoom" is refused with a sentence, and nothing changes | Radial Blur's type is "spin" or "zoom", and this is "Zoom". | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Radial Blur's amount runs from 0 to 100, and this is 150. | yes |
| centre keyed to 50, 2000 is refused with a sentence, and nothing changes | Radial Blur's center runs from -1000 to 1000, and this is 2000. | yes |
| zoom 100 about centre 1000, -1000, the tops of the ranges, is taken | taken | yes |
| spin 0 about centre -1000, 1000, the bottoms, is taken | taken | yes |
| centre keyed from 50, 50 to 0, 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_radial_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_radial_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

76 of 76 checks pass.
