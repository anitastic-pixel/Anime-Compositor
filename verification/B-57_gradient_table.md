# B-57: gradient

D-114, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/gradient/expected_gradient.json`, written by `tools/gradient_reference.py` before this code existed and printed in document 25 as FX-GRAD-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-GRAD-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRAD-001 frame 0: The settings as they start: linear from the top middle to the bottom middle, white at 0 per cent to #6450a0 at 50, multiply. Row 1, the line along the top, is barely touched, and each row down is darker and more violet, the bottom most; every pixel in a row is changed alike, and the empty pixels stay empty. | largest difference 2.0e-7 | yes |
| FX-GRAD-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-002 frame 0: Blend normal, both opacities 100: the cel is painted over with the gradient itself, white fading down to #6450a0, the line and the shadow gone under it, each pixel at its own covering. | largest difference 3.0e-8 | yes |
| FX-GRAD-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-003 frame 0: Blend screen, white to #6450a0, both opacities 100: every pixel that shows is lightened, and none darkened. | largest difference 1.3e-7 | yes |
| FX-GRAD-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-004 frame 0: Blend add, #000000 to #6450a0, 0 to 50 per cent: the violet is added, nothing at the top and most at the bottom; no channel passes the pixel's covering. | largest difference 2.1e-7 | yes |
| FX-GRAD-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-005 frame 0: Radial, from the middle, 50, 50, to the middle of the right edge, 100, 50, blend normal at 100: the colour runs in rings about the middle, and everything eight pixels or further from it is #6450a0. | largest difference 3.0e-8 | yes |
| FX-GRAD-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-006 frame 0: Start and end the same point, 50, 50: t is 1 everywhere, so, blend normal at 100, every pixel that shows is #6450a0 at its own covering. | largest difference 3.0e-8 | yes |
| FX-GRAD-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-007 frame 0: Linear, left to right, 0, 50 to 100, 50, blend normal at 100: the colour runs across the columns, every pixel in a column alike. | largest difference 3.9e-8 | yes |
| FX-GRAD-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-008 frame 0: Both points outside the drawing, 50, -100 and 50, 200, blend normal at 100: the drawing sees only the middle third of the gradient, t from 11.5/30 in row 1, the line along its top, to 18.5/30 in row 8. | largest difference 3.1e-8 | yes |
| FX-GRAD-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-009 frame 0: Both points past the bottom, 50, 150 and 50, 200, blend normal at 100: every pixel is before the start, t is 0, and the cel is all white. | largest difference 3.0e-8 | yes |
| FX-GRAD-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-010 frame 0: The start point keyed from 50, 0 at frame 0 to 50, 100 at frame 4, the end 50, 100, blend normal at 100, linear: frame 0 is FX-GRAD-002; frame 2 starts at the middle row, so rows 0 to 4 are white; frame 4 has start and end the same, and is FX-GRAD-006. | largest difference 3.0e-8 | yes |
| FX-GRAD-010 frame 2: The start point keyed from 50, 0 at frame 0 to 50, 100 at frame 4, the end 50, 100, blend normal at 100, linear: frame 0 is FX-GRAD-002; frame 2 starts at the middle row, so rows 0 to 4 are white; frame 4 has start and end the same, and is FX-GRAD-006. | largest difference 3.9e-8 | yes |
| FX-GRAD-010 frame 4: The start point keyed from 50, 0 at frame 0 to 50, 100 at frame 4, the end 50, 100, blend normal at 100, linear: frame 0 is FX-GRAD-002; frame 2 starts at the middle row, so rows 0 to 4 are white; frame 4 has start and end the same, and is FX-GRAD-006. | largest difference 3.0e-8 | yes |
| FX-GRAD-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-011 frame 0: Start opacity keyed from 0 at frame 0 to 100 at frame 4, end opacity from 50 to 0: frame 0 is FX-GRAD-001, frame 2 is 50 to 25, frame 4 is 100 to 0, the top now most touched and the bottom row barely. | largest difference 2.0e-7 | yes |
| FX-GRAD-011 frame 2: Start opacity keyed from 0 at frame 0 to 100 at frame 4, end opacity from 50 to 0: frame 0 is FX-GRAD-001, frame 2 is 50 to 25, frame 4 is 100 to 0, the top now most touched and the bottom row barely. | largest difference 1.8e-7 | yes |
| FX-GRAD-011 frame 4: Start opacity keyed from 0 at frame 0 to 100 at frame 4, end opacity from 50 to 0: frame 0 is FX-GRAD-001, frame 2 is 50 to 25, frame 4 is 100 to 0, the top now most touched and the bottom row barely. | largest difference 1.5e-7 | yes |
| FX-GRAD-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-012 frame 0: Both opacities 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-GRAD-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-013 frame 0: White to white, blend normal at 100: every pixel that shows turns white at its own covering, the half-covering edge white at half covering, and the empty pixels stay empty. | largest difference 3.0e-8 | yes |
| FX-GRAD-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-014 frame 0: FX-GRAD-001 with its colours written in capitals: the same. | largest difference 2.0e-7 | yes |
| FX-GRAD-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-015 frame 0: FX-GRAD-001 moved three pixels right: the same, moved; the gradient moves with the drawing. | largest difference 2.0e-7 | yes |
| FX-GRAD-015 frame 3: FX-GRAD-001 moved three pixels right: the same, moved; the gradient moves with the drawing. | largest difference 2.0e-7 | yes |
| FX-GRAD-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-016 frame 0: Start opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-016 frame 4: Start opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-017 frame 0: End opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-017 frame 4: End opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-018 frame 0: End opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-018 frame 4: End opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-019 frame 0: Start point 1001, 0, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-019 frame 4: Start point 1001, 0, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-020 frame 0: Shape "conic", which is not a shape. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-020 frame 4: Shape "conic", which is not a shape. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-021 frame 0: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-021 frame 4: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-022 frame 0: A start colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-022 frame 4: A start colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is coloured where it is | 0 | yes |
| a half-size draft preview changes nothing: the points are shares of the drawing, not distances | Gradient { shape: "linear", start: [50.0, 0.0], end: [50.0, 100.0], start_color: "#ffffff", end_color: "#6450a0", start_opacity: 0.0, end_opacity: 50.0, blend: "multiply" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grad_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_014.json, its colours in capitals, is saved with them in small letters | "#ffffff" | yes |
| a file with no `blend` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a start point of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| start opacity 101 is refused with a sentence, and nothing changes | Gradient's start opacity runs from 0 to 100, and this is 101. | yes |
| end opacity -1 is refused with a sentence, and nothing changes | Gradient's end opacity runs from 0 to 100, and this is -1. | yes |
| start point 1001, 0 is refused with a sentence, and nothing changes | Gradient's start runs from -1000 to 1000, and this is 1001. | yes |
| end point 50, -1001 is refused with a sentence, and nothing changes | Gradient's end runs from -1000 to 1000, and this is -1001. | yes |
| shape "conic" is refused with a sentence, and nothing changes | Gradient's shape is "linear" or "radial", and this is "conic". | yes |
| blend "overlay" is refused with a sentence, and nothing changes | Gradient's blend is "normal", "multiply", "screen" or "add", and this is "overlay". | yes |
| start colour "#12345" is refused with a sentence, and nothing changes | Gradient's start colour is written #rrggbb, and this is "#12345". | yes |
| end colour "violet" is refused with a sentence, and nothing changes | Gradient's end colour is written #rrggbb, and this is "violet". | yes |
| end opacity keyed to 150 is refused with a sentence, and nothing changes | Gradient's end opacity runs from 0 to 100, and this is 150. | yes |
| both opacities 100, the top, is taken | taken | yes |
| start point -1000, 1000, the corner of the range, is taken | taken | yes |
| radial, screen is taken | taken | yes |
| the start point keyed from 50, 0 to 50, 100 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grad_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grad_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

87 of 87 checks pass.
