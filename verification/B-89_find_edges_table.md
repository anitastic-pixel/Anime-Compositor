# B-89: find edges

D-146, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the thirteenth of the third batch. Every expected pixel is `Fixtures/find_edges/expected_find_edges.json`, written by `tools/find_edges_reference.py` before this code existed and printed in document 25 as FX-FINDEDGES-001 to 019. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-FINDEDGES-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FINDEDGES-001 frame 0: Invert off, amount 100, the settings as they start: dark lines on white. Where the picture is flat, inside the skin and the blue, the pixel turns pure white; the drawing's own outline against the empty rows and column turns black; the line, one pixel wide in column 6, comes out as two black lines either side of it, its own middle white in rows 2 and 3, where the skin either side matches (the rule looks at a pixel's neighbours, not the pixel), and grey lower down, where the shade begins on one side; the skin meets its shade in a grey edge; the blue beside the red trace barely darkens; every shown pixel turns a grey, at its own covering, the soft edge keeping its covering, and the empty pixels stay empty. | largest difference 9.9e-8 | yes |
| FX-FINDEDGES-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-003 frame 0: Invert on, amount 100: light lines on black, FX-FINDEDGES-001 turned round: every shown pixel's grey is 1 minus FX-FINDEDGES-001's, so the flat skin and blue turn black and the lines and the outline white or nearly white. | largest difference 1.6e-7 | yes |
| FX-FINDEDGES-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-004 frame 0: Amount 50: every shown pixel goes half way from its own colour to FX-FINDEDGES-001's grey, in encoded values: the flat skin half way to white, the line half way to black. | largest difference 1.2e-7 | yes |
| FX-FINDEDGES-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-005 frame 0: Invert on, amount 50: half way from each pixel's colour to FX-FINDEDGES-003's grey. | largest difference 1.7e-7 | yes |
| FX-FINDEDGES-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-006 frame 0: Amount 25: a quarter of the way to FX-FINDEDGES-001's grey; the drawing's colours still show. | largest difference 1.7e-7 | yes |
| FX-FINDEDGES-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-007 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-FINDEDGES-004 and frame 4 FX-FINDEDGES-001. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-007 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-FINDEDGES-004 and frame 4 FX-FINDEDGES-001. | largest difference 1.2e-7 | yes |
| FX-FINDEDGES-007 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 FX-FINDEDGES-004 and frame 4 FX-FINDEDGES-001. | largest difference 9.9e-8 | yes |
| FX-FINDEDGES-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-008 frame 0: Invert on, amount keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 FX-FINDEDGES-003, frame 2 FX-FINDEDGES-005 and frame 4 untouched. | largest difference 1.6e-7 | yes |
| FX-FINDEDGES-008 frame 2: Invert on, amount keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 FX-FINDEDGES-003, frame 2 FX-FINDEDGES-005 and frame 4 untouched. | largest difference 1.7e-7 | yes |
| FX-FINDEDGES-008 frame 4: Invert on, amount keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 FX-FINDEDGES-003, frame 2 FX-FINDEDGES-005 and frame 4 untouched. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-009 frame 0: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-FINDEDGES-001, as is frame 4; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-009 frame 2: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-FINDEDGES-001, as is frame 4; frame 0 is the drawing. | largest difference 9.9e-8 | yes |
| FX-FINDEDGES-009 frame 4: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-FINDEDGES-001, as is frame 4; frame 0 is the drawing. | largest difference 9.9e-8 | yes |
| FX-FINDEDGES-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-010 frame 0: Invert on, amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-011 frame 0: A flat skin card that fills the whole drawing, invert off: nothing changes from one pixel to the next, and past the drawing's border its border pixels repeat, so the border is no edge: every pixel turns pure white. | largest difference 0.0e0 | yes |
| FX-FINDEDGES-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-012 frame 0: The same card, invert on: every pixel turns pure black. | largest difference 8.6e-18 | yes |
| FX-FINDEDGES-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-013 frame 0: FX-FINDEDGES-001 moved three pixels right: the same, moved; the three columns on the left, outside the layer, stay empty. | largest difference 9.9e-8 | yes |
| FX-FINDEDGES-013 frame 3: FX-FINDEDGES-001 moved three pixels right: the same, moved; the three columns on the left, outside the layer, stay empty. | largest difference 9.9e-8 | yes |
| FX-FINDEDGES-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-014 frame 0: FX-FINDEDGES-003 moved three pixels right: the same, moved; the columns outside the layer stay empty, not black. | largest difference 1.6e-7 | yes |
| FX-FINDEDGES-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FINDEDGES-015 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-015 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FINDEDGES-016 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-016 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FINDEDGES-017 frame 0: Invert "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-017 frame 4: Invert "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FINDEDGES-018 frame 0: Invert "ON", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-018 frame 4: Invert "ON", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FINDEDGES-019 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-019 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FINDEDGES-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | FindEdges { invert: "off", amount: 100.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_findedges_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_findedges_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `invert` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an amount that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Find Edges's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Find Edges's amount runs from 0 to 100, and this is -1. | yes |
| invert "yes" is refused with a sentence, and nothing changes | Find Edges's invert is "off" or "on", and this is "yes". | yes |
| invert "ON" is refused with a sentence, and nothing changes | Find Edges's invert is "off" or "on", and this is "ON". | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Find Edges's amount runs from 0 to 100, and this is 150. | yes |
| amount 0, the bottom, is taken | taken | yes |
| amount 100, the top, is taken | taken | yes |
| amount keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_findedges_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_findedges_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_findedges_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

75 of 75 checks pass.
