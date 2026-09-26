# B-61: outline

D-118, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/outline/expected_outline.json`, written by `tools/outline_reference.py` before this code existed and printed in document 25 as FX-OUTLINE-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-OUTLINE-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-OUTLINE-001 frame 0: The defaults: white, width 3, softness 0, opacity 100. A white band three pixels wide round the card, its corners rounded (a pixel three across and one down from a corner is outside it), filling rows 0 to 9; the card itself, fully covered, is unchanged. The soft pixel takes the white through its uncovered share and is fully covered; past it, where only the soft pixel is within reach, as at (13, 4), the band is only as strong as it, 128/255. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-002 frame 0: Width 0: the drawing, untouched, and the layer does not grow. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-003 frame 0: Width 0, softness 10: still nothing. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-004 frame 0: Width 1: a one-pixel band beside the card's four sides, with its four corners empty, since a diagonal step is 1.41 away; the soft pixel is filled to full covering, and the pixel past it, (11, 4), takes the band at 128/255. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-005 frame 0: Width 2.5: steps of (2, 1) count, 2.24 away, and (2, 2) and (3, 0) do not, 2.83 and 3 away; the layer grows by 3, but its outermost ring stays empty. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-006 frame 0: Width 2, softness 3: the band of width 2, blurred at sigma 1, soft at its outer edge and reaching three pixels further. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-007 frame 0: Opacity 40: FX-OUTLINE-001's band at 40 per cent of its covering. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-008 frame 0: Colour #c82828, width 1: a red band; the soft pixel becomes half line and half red, fully covered. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-009 frame 0: FX-OUTLINE-008 with the colour written in capitals: the same. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-010 frame 0: Width keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, untouched; frame 2, at 1, is FX-OUTLINE-004; frame 4 is width 2, where the step (1, 1), 1.41 away, counts, and (2, 1), 2.24 away, does not, so its corners are cut on the slant. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-010 frame 2: Width keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, untouched; frame 2, at 1, is FX-OUTLINE-004; frame 4 is width 2, where the step (1, 1), 1.41 away, counts, and (2, 1), 2.24 away, does not, so its corners are cut on the slant. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-010 frame 4: Width keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, untouched; frame 2, at 1, is FX-OUTLINE-004; frame 4 is width 2, where the step (1, 1), 1.41 away, counts, and (2, 1), 2.24 away, does not, so its corners are cut on the slant. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-011 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is FX-OUTLINE-001, as is frame 4. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-011 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is FX-OUTLINE-001, as is frame 4. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-011 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is FX-OUTLINE-001, as is frame 4. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-012 frame 0: Width 0.5: only the step (0, 0) counts, so the band is the drawing's own covering and shows only through the soft pixel, whose covering goes from 0.502 to 0.752; the layer grows by 1 with nothing in it. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-013 frame 0: Width 5: the band would reach rows -2 to 11 and is cut off by the frame's top and bottom edges; it runs along rows 0 and 9 from column 1 to 13, and reaches column 0 in rows 3 to 6. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-014 frame 0: FX-OUTLINE-001 moved three pixels right: the same, moved, and the band past the soft pixel, which would fall in column 16, is cut off by the frame's edge. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OUTLINE-015 frame 0: Width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-015 frame 4: Width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OUTLINE-016 frame 0: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-016 frame 4: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OUTLINE-017 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-017 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OUTLINE-018 frame 0: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-018 frame 4: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OUTLINE-019 frame 0: A colour written "#gggggg", not hexadecimal. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-019 frame 4: A colour written "#gggggg", not hexadecimal. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OUTLINE-020 frame 0: Width keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-020 frame 4: Width keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OUTLINE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| the defaults, width 3, grow the drawing by 3 on every side | 3 | yes |
| width 2.5 and softness 6 grow it by 3, the width rounded up, and 6, the blur's reach | 9 | yes |
| width 0 does not grow it, whatever the softness | 0 | yes |
| a half-size draft preview halves the width and the softness, and nothing else | Outline { color: "#ffffff", width: 2.0, softness: 1.0, opacity: 100.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_outline_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_outline_009.json, its colour in capitals, is saved with it in small letters | "#c82828" | yes |
| a file with no `opacity` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a width written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 101 is refused with a sentence, and nothing changes | Outline's width runs from 0 to 100, and this is 101. | yes |
| width -1 is refused with a sentence, and nothing changes | Outline's width runs from 0 to 100, and this is -1. | yes |
| softness 101 is refused with a sentence, and nothing changes | Outline's softness runs from 0 to 100, and this is 101. | yes |
| opacity -1 is refused with a sentence, and nothing changes | Outline's opacity runs from 0 to 100, and this is -1. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Outline's opacity runs from 0 to 100, and this is 101. | yes |
| colour "white" is refused with a sentence, and nothing changes | Outline's colour is written #rrggbb, and this is "white". | yes |
| width keyed to 150 is refused with a sentence, and nothing changes | Outline's width runs from 0 to 100, and this is 150. | yes |
| width, softness and opacity 100, the tops, is taken | taken | yes |
| width, softness and opacity 0, the bottoms, is taken | taken | yes |
| width keyed from 0 to 2 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_outline_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_outline_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

81 of 81 checks pass.
