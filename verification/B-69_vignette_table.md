# B-69: vignette

D-126, accepted by the owner on 2026-09-26, the fourth of the second batch of ten. Every expected pixel is `Fixtures/vignette/expected_vignette.json`, written by `tools/vignette_reference.py` before this code existed and printed in document 25 as FX-VIGNETTE-001 to 025. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-VIGNETTE-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-VIGNETTE-001 frame 0: The settings as they start: amount 50, #000000, size 100, roundness 0, softness 50, about the middle. The corners are darkened most, the bottom right pixel 0.465 of the way to black; the fade reaches in only to the ellipse half the size, so the middle, columns 3 to 12 of rows 3 to 6 among it, is untouched; nothing is lightened, and the empty corner stays empty. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-002 frame 0: Amount 100: the same fade twice as strong, each pixel twice as far toward black as in FX-VIGNETTE-001. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-003 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-004 frame 0: Colour #6450a0, a violet: FX-VIGNETTE-001's fade toward violet, so the line along row 7 is lightened at its ends and the skin darkened. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-005 frame 0: Size 50: the ellipse half the size, so the fade runs from a quarter of the way out to half way, and every pixel past half way, among them the top and bottom rows and the two outer columns on each side, takes the full amount, half way to black; the four middle pixels are untouched. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-006 frame 0: Size 200: the fade starts at the corners' own distance and beyond, so nothing in the drawing is reached: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-007 frame 0: Softness 0, size 60: a hard-edged ellipse, each pixel either untouched or half way to black, nothing between. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-008 frame 0: Softness 100: the fade starts at the centre itself, so every pixel that shows is darkened, the four middle pixels least, and more toward the edges. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-009 frame 0: Roundness 100: a circle rather than an ellipse the drawing's shape, so pixels equally far from the middle are darkened alike: (3, 1) and (4, 0), each 5.70 pixels from it, where roundness 0 darkens (4, 0) about twice as much; the sides are darkened more than in FX-VIGNETTE-001, and the middle of the top row less. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-010 frame 0: Centre 0, 0, the top left corner: the vignette is about that corner, so the top left pixels are untouched and the bottom right is half way to black. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-011 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-VIGNETTE-001 and frame 4 FX-VIGNETTE-002. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-011 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-VIGNETTE-001 and frame 4 FX-VIGNETTE-002. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-011 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-VIGNETTE-001 and frame 4 FX-VIGNETTE-002. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-012 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-VIGNETTE-001, frame 2 is about 25, 25 and frame 4 is FX-VIGNETTE-010. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-012 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-VIGNETTE-001, frame 2 is about 25, 25 and frame 4 is FX-VIGNETTE-010. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-012 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-VIGNETTE-001, frame 2 is about 25, 25 and frame 4 is FX-VIGNETTE-010. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-013 frame 0: Size keyed from 200 at frame 0 to 50 at frame 4, linear: the vignette closes in, frame 0 the drawing, frame 2 size 125 and frame 4 FX-VIGNETTE-005. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-013 frame 2: Size keyed from 200 at frame 0 to 50 at frame 4, linear: the vignette closes in, frame 0 the drawing, frame 2 size 125 and frame 4 FX-VIGNETTE-005. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-013 frame 4: Size keyed from 200 at frame 0 to 50 at frame 4, linear: the vignette closes in, frame 0 the drawing, frame 2 size 125 and frame 4 FX-VIGNETTE-005. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-014 frame 0: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-VIGNETTE-002; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-014 frame 2: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-VIGNETTE-002; frame 0 is the drawing. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-015 frame 0: Size 1, amount 100: every pixel is past the tiny ellipse, so every pixel that shows turns black at its own covering, the soft edge black at half covering, and the empty corner stays empty. | largest difference 3.0e-8 | yes |
| FX-VIGNETTE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-016 frame 0: FX-VIGNETTE-004 with the colour written in capitals, #6450A0: the same. | largest difference 2.1e-7 | yes |
| FX-VIGNETTE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-017 frame 0: FX-VIGNETTE-001 moved three pixels right: the same, moved; the vignette moves with the drawing. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-017 frame 3: FX-VIGNETTE-001 moved three pixels right: the same, moved; the vignette moves with the drawing. | largest difference 2.0e-7 | yes |
| FX-VIGNETTE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-018 frame 0: A directional blur, direction 90 and length 4, then the vignette as it starts: the blur grew the layer two pixels on every side, and the ellipse is still the drawing's own, about its own middle. | largest difference 2.2e-7 | yes |
| FX-VIGNETTE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIGNETTE-019 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-019 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIGNETTE-020 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-020 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIGNETTE-021 frame 0: Roundness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-021 frame 4: Roundness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIGNETTE-022 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-022 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIGNETTE-023 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-023 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIGNETTE-024 frame 0: Size keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-024 frame 4: Size keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIGNETTE-025 frame 0: Colour "black", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-025 frame 4: Colour "black", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIGNETTE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: the size and the centre are shares of the drawing | Vignette { amount: 70.0, color: "#6450a0", size: 80.0, roundness: 40.0, softness: 30.0, center: [40.0, 60.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vignette_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vignette_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `center` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a size that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Vignette's amount runs from 0 to 100, and this is 101. | yes |
| size 0.5 is refused with a sentence, and nothing changes | Vignette's size runs from 1 to 200, and this is 0.5. | yes |
| size 201 is refused with a sentence, and nothing changes | Vignette's size runs from 1 to 200, and this is 201. | yes |
| roundness -1 is refused with a sentence, and nothing changes | Vignette's roundness runs from 0 to 100, and this is -1. | yes |
| softness 101 is refused with a sentence, and nothing changes | Vignette's softness runs from 0 to 100, and this is 101. | yes |
| centre 1001, 50 is refused with a sentence, and nothing changes | Vignette's center runs from -1000 to 1000, and this is 1001. | yes |
| colour "black" is refused with a sentence, and nothing changes | Vignette's colour is written #rrggbb, and this is "black". | yes |
| size keyed to 250 is refused with a sentence, and nothing changes | Vignette's size runs from 1 to 200, and this is 250. | yes |
| amount 100, size 200, roundness 100, softness 100 and centre 1000, 1000, the tops, is taken | taken | yes |
| amount 0, size 1, roundness 0, softness 0 and centre -1000, -1000, the bottoms, is taken | taken | yes |
| softness keyed from 50 to 0 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vignette_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vignette_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

95 of 95 checks pass.
