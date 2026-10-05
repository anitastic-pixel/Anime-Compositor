# D-327: Fast Box Blur

From P-26: tutorials 2 and 3 blur with After Effects' Fast Box Blur, which the replays stood Gaussian Blur in for. After Effects' manual gives its settings, Blur Radius, Iterations, Blur Dimensions and Repeat Edge Pixels: a box laid on several times, ending hard at iterations times the radius, where a Gaussian's faint tail goes on and an Exposure after it shows the difference. Every expected pixel is `Fixtures/fast_box_blur/expected_fast_box_blur.json`, written by `tools/fast_box_blur_reference.py` before the build had it, printed in document 25 as FX-FASTBOX-001 to 011. Tolerance 2e-5.

## FX-FASTBOX-001 to 011 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FASTBOX-001 frame 0: Radius 4, iterations 3 (After Effects' default): the box laid on three times, reaching 12 pixels; column 15, twelve right of the square, is lit and column 16 is clear. | largest difference 1.3e-8 | yes |
| FX-FASTBOX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-002 frame 0: Radius 4, iterations 1: one plain box of 9 pixels, flat-topped, ending hard 4 pixels right of the square. | largest difference 6.6e-9 | yes |
| FX-FASTBOX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-003 frame 0: Radius 2.5, iterations 1: the box of 5 pixels and half of the next on each side. | largest difference 4.3e-8 | yes |
| FX-FASTBOX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-004 frame 0: FX-FASTBOX-001 with edges repeat: the square's orange is read past the left edge, and the layer does not grow. | largest difference 4.9e-8 | yes |
| FX-FASTBOX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-005 frame 0: FX-FASTBOX-001 with Blur Dimensions horizontal: spread across only. | largest difference 2.3e-8 | yes |
| FX-FASTBOX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-006 frame 0: Radius 0: the drawing untouched. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-007 frame 0: A Float composition: radius 2, iterations 3, then Exposure +6. The light falls away to nothing 6 pixels right of the square and no further, where a Gaussian's tail goes on. | largest difference 2.0e-6 | yes |
| FX-FASTBOX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-008 frame 0: Iterations 2.7: its whole part, 2, counted. | largest difference 1.2e-8 | yes |
| FX-FASTBOX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTBOX-009 frame 0: Iterations 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-009 frame 4: Iterations 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTBOX-010 frame 0: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-010 frame 4: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTBOX-011 frame 0: Blur Dimensions "Both": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-011 frame 4: Blur Dimensions "Both": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTBOX-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The kernel

| Check | The build's answer | Matches |
| --- | --- | --- |
| Radius 4, iterations 3: reaches 12 pixels each side and sums to 1 | 25 taps, sum 1.0000000 | yes |
| Radius 2.5, iterations 1: reaches 3 pixels each side and sums to 1 | 7 taps, sum 1.0000000 | yes |
| Radius 2.5, iterations 3: reaches 9 pixels each side and sums to 1 | 19 taps, sum 1.0000000 | yes |
| Radius 0, iterations 3: reaches 0 pixels each side and sums to 1 | 1 taps, sum 1.0000000 | yes |
| Radius 1, iterations 50: reaches 50 pixels each side and sums to 1 | 101 taps, sum 1.0000000 | yes |
| Radius 500, iterations 1: reaches 500 pixels each side and sums to 1 | 1001 taps, sum 1.0000000 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastbox_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastbox_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastbox_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastbox_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastbox_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastbox_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Saved with its radius, iterations, edges and dimensions | {"edges":"repeat","iterations":3,"radius":4} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| iterations 0 is refused with a sentence, and nothing changes | Fast Box Blur's iterations runs from 1 to 50, and this is 0. | yes |
| radius 501 is refused with a sentence, and nothing changes | Fast Box Blur's radius runs from 0 to 500, and this is 501. | yes |
| Blur Dimensions "Both", written with a capital is refused with a sentence, and nothing changes | Fast Box Blur's dimensions are "both", "horizontal" or "vertical", and this is "Both". | yes |
| FX-FASTBOX-002 set to iterations 3, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-FASTBOX-002 set to iterations 3 by the command draws FX-FASTBOX-001's frame | largest difference 1.3e-8 | yes |

## A half-size draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| A half-size draft halves the radius and keeps the iterations | FastBoxBlur { radius: 4.0, iterations: 3.0, edges: "transparent", dimensions: "both" } | yes |

## The preview

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastbox_001.json: the preview draws the same picture as the CPU, within 1 level of 255 | card, largest difference 0 of 255 | yes |
| fx_fastbox_004.json: the preview draws the same picture as the CPU, within 1 level of 255 | card, largest difference 1 of 255 | yes |
| fx_fastbox_007.json: the preview draws the same picture as the CPU, within 1 level of 255 | CPU, largest difference 0 of 255 | yes |

## Pictures: a white square, in `verification/D-327 pictures/`, over black

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_gaussian_blurriness_20.png, Gaussian Blur, Blurriness 20; draws cleanly | [], 18 pixels right of the square 6 of 255, 24 pixels right 0 | yes |
| 2_fast_box_radius_6.png, Fast Box Blur, radius 6, iterations 3: about as soft; draws cleanly | [], 18 pixels right of the square 1 of 255, 24 pixels right 0 | yes |
| 3_gaussian_then_exposure_17.png, the Gaussian with Exposure +17 after it, Float: its faint tail lit up far out; draws cleanly | [], 18 pixels right of the square 255 of 255, 24 pixels right 255 | yes |
| 4_fast_box_then_exposure_17.png, Fast Box Blur with Exposure +17 after it, Float: ending hard 18 pixels out; draws cleanly | [], 18 pixels right of the square 255 of 255, 24 pixels right 0 | yes |
| With Exposure +17, the Gaussian still shows 24 pixels out; Fast Box Blur is black there | Gaussian 255, Fast Box Blur 0 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastbox_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastbox_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastbox_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

56 of 56 checks pass.
