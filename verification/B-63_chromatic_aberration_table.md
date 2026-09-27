# B-63: chromatic aberration

D-120, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/chromatic_aberration/expected_chromatic_aberration.json`, written by `tools/chromatic_aberration_reference.py` before this code existed and printed in document 25 as FX-CHROMA-001 to 016. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CHROMA-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHROMA-001 frame 0: The settings as they start, amount 3 about the middle: red fringes outward past the blocks' outer edges, where the red sample alone covers and the pixel takes its covering, and blue fringes inward; the white block's left column, on the layer's edge, loses its blue, whose sample falls outside the layer, and turns yellow, and nothing is drawn beyond the layer. | largest difference 2.5e-7 | yes |
| FX-CHROMA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-CHROMA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-003 frame 0: Amount 6: wider fringes than FX-CHROMA-001; the half-covering edge keeps at least its own covering. | largest difference 2.5e-7 | yes |
| FX-CHROMA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-004 frame 0: Centre 0, 50, the middle of the left edge: the split runs away from there, so red spreads right past both blocks' right sides, and the white block's left column, beside the centre, keeps its blue, whose sample now lands inside the layer. | largest difference 2.5e-7 | yes |
| FX-CHROMA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-005 frame 0: Amount keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 untouched, frame 2 is FX-CHROMA-003, frame 4 splits at 12, past the drawing's half diagonal, 9.43, so its red is turned over as FX-CHROMA-008's is. | largest difference 1.9e-7 | yes |
| FX-CHROMA-005 frame 2: Amount keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 untouched, frame 2 is FX-CHROMA-003, frame 4 splits at 12, past the drawing's half diagonal, 9.43, so its red is turned over as FX-CHROMA-008's is. | largest difference 2.5e-7 | yes |
| FX-CHROMA-005 frame 4: Amount keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 untouched, frame 2 is FX-CHROMA-003, frame 4 splits at 12, past the drawing's half diagonal, 9.43, so its red is turned over as FX-CHROMA-008's is. | largest difference 2.5e-7 | yes |
| FX-CHROMA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-006 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4: frame 0 is FX-CHROMA-001, frame 2 splits about 25, 50, frame 4 is FX-CHROMA-004. | largest difference 2.5e-7 | yes |
| FX-CHROMA-006 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4: frame 0 is FX-CHROMA-001, frame 2 splits about 25, 50, frame 4 is FX-CHROMA-004. | largest difference 2.5e-7 | yes |
| FX-CHROMA-006 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4: frame 0 is FX-CHROMA-001, frame 2 splits about 25, 50, frame 4 is FX-CHROMA-004. | largest difference 2.5e-7 | yes |
| FX-CHROMA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-007 frame 0: FX-CHROMA-001 moved three pixels right: the same, moved; the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 2.5e-7 | yes |
| FX-CHROMA-007 frame 3: FX-CHROMA-001 moved three pixels right: the same, moved; the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 2.5e-7 | yes |
| FX-CHROMA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-008 frame 0: Amount 100, the most: k is past 10, so the red picture is turned over through the centre and shrunk nearly tenfold and the blue one grown nearly twelvefold, and on a drawing this small both samples fall off it for every pixel: each keeps only its own green, at its own covering, as in FX-CHROMA-009. | largest difference 5.5e-8 | yes |
| FX-CHROMA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-009 frame 0: Centre -1000, -1000, far off the top left: both samples fall off the layer for every pixel, so each pixel keeps only its own green, at its own covering. | largest difference 5.5e-8 | yes |
| FX-CHROMA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-010 frame 0: Amount 1: fringes a pixel or less wide at the blocks' edges; the insides of both blocks, whose samples all land inside them, keep their colour. | largest difference 2.5e-7 | yes |
| FX-CHROMA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-011 frame 0: Centre 100, 100, the bottom right corner: the red picture is drawn larger away from that corner, up and left, so red reaches rows 0 and 1 above the blocks; the blue picture is drawn smaller toward it, so blue reaches row 8 below the white block; and the white block's left column loses its blue. | largest difference 2.5e-7 | yes |
| FX-CHROMA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMA-012 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-012 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHROMA-013 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-013 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHROMA-014 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-014 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHROMA-015 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-015 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHROMA-016 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-016 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHROMA-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: a fringe past the layer's edge is cut | 0 | yes |
| a half-size draft preview halves the amount, a distance, and leaves the centre, a share of the drawing | ChromaticAberration { amount: 3.0, center: [30.0, 70.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chroma_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chroma_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `center` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Chromatic Aberration's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Chromatic Aberration's amount runs from 0 to 100, and this is -1. | yes |
| centre 1001, 50 is refused with a sentence, and nothing changes | Chromatic Aberration's center runs from -1000 to 1000, and this is 1001. | yes |
| centre 50, -1001 is refused with a sentence, and nothing changes | Chromatic Aberration's center runs from -1000 to 1000, and this is -1001. | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Chromatic Aberration's amount runs from 0 to 100, and this is 150. | yes |
| amount 100 and centre 1000, 1000, the tops, is taken | taken | yes |
| amount 0 and centre -1000, -1000, the bottoms, is taken | taken | yes |
| centre keyed from 50, 50 to 0, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chroma_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chroma_007.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

68 of 68 checks pass.
