# D-471: Fast Blur (Legacy)

B-351: `core.fast_blur_legacy` takes After Effects' Fast Blur (Legacy) controls (Blurriness, Blur Dimensions, Repeat Edge Pixels) and draws them with Fast Box Blur's engine at radius 0.37 times Blurriness and 3 iterations (Maschwitz's match; D-383 and D-394, both names, one engine). Every expected pixel is `Fixtures/fast_blur_legacy/expected_fast_blur_legacy.json`, written by `tools/fast_blur_legacy_reference.py` before this code existed and printed in document 25 as FX-FASTLEG-001 to 013. Tolerance 2e-5.

## FX-FASTLEG-001 to 013 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FASTLEG-001 frame 0: Blurriness 10: Fast Box Blur at radius 3.7 and 3 iterations, reaching 12 pixels; column 15, twelve right of the square, is lit and column 16 is clear. | largest difference 1.2e-8 | yes |
| FX-FASTLEG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-002 frame 0: Blurriness 5: radius 1.85, the box of 3 pixels and 0.85 of the next on each side, laid on three times, reaching 6 pixels. | largest difference 3.4e-8 | yes |
| FX-FASTLEG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-003 frame 0: FX-FASTLEG-001 with Repeat Edge Pixels: the square's orange is read past the left edge, and the layer does not grow. | largest difference 3.0e-8 | yes |
| FX-FASTLEG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-004 frame 0: FX-FASTLEG-001 with Blur Dimensions horizontal: spread across only. | largest difference 2.1e-8 | yes |
| FX-FASTLEG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-005 frame 0: FX-FASTLEG-001 with Blur Dimensions vertical: spread down only. | largest difference 2.1e-8 | yes |
| FX-FASTLEG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-006 frame 0: Blurriness 0: the drawing untouched. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-007 frame 0: Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is Blurriness 10, FX-FASTLEG-001's picture. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-007 frame 2: Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is Blurriness 10, FX-FASTLEG-001's picture. | largest difference 1.2e-8 | yes |
| FX-FASTLEG-007 frame 4: Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is Blurriness 10, FX-FASTLEG-001's picture. | largest difference 3.1e-9 | yes |
| FX-FASTLEG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-008 frame 0: A Float composition: Blurriness 5, then Exposure +6. The light falls away to nothing 6 pixels right of the square and no further. | largest difference 2.2e-6 | yes |
| FX-FASTLEG-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTLEG-009 frame 0: Blurriness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-009 frame 4: Blurriness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTLEG-010 frame 0: Blurriness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-010 frame 4: Blurriness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTLEG-011 frame 0: Blur Dimensions "Both": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-011 frame 4: Blur Dimensions "Both": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTLEG-012 frame 0: Repeat Edge Pixels "on": the word is "transparent" or "repeat". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-012 frame 4: Repeat Edge Pixels "on": the word is "transparent" or "repeat". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTLEG-013 frame 0: Blurriness keyed to 1200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-013 frame 4: Blurriness keyed to 1200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FASTLEG-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## One engine

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FASTLEG-001's drawing with Fast Box Blur, radius 3.7 and 3 iterations, in its place draws FX-FASTLEG-001's frame (Blurriness 10) | largest difference 1.2e-8 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastleg_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastleg_004.json is saved with its blurriness and dimensions, and no radius or iterations of its own | {"blurriness":10,"dimensions":"horizontal"} | yes |
| fx_fastleg_009.json is refused in a sentence naming Fast Blur (Legacy) and its blurriness | Fast Blur (Legacy)'s blurriness runs from 0 to 1000, and this is -1. | yes |
| fx_fastleg_010.json is refused in a sentence naming Fast Blur (Legacy) and its blurriness | Fast Blur (Legacy)'s blurriness runs from 0 to 1000, and this is 1001. | yes |
| fx_fastleg_011.json is refused in a sentence naming Fast Blur (Legacy) and its dimensions | Fast Blur (Legacy)'s dimensions are "both", "horizontal" or "vertical", and this is "Both". | yes |
| fx_fastleg_012.json is refused in a sentence naming Fast Blur (Legacy) and its edges | Fast Blur (Legacy)'s edges are "transparent" or "repeat", and this is "on". | yes |
| a file with a Fast Blur (Legacy) whose blurriness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fast Blur (Legacy) whose dimensions is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blurriness 1001 is refused with a sentence, and nothing changes | Fast Blur (Legacy)'s blurriness runs from 0 to 1000, and this is 1001. | yes |
| blurriness -1 is refused with a sentence, and nothing changes | Fast Blur (Legacy)'s blurriness runs from 0 to 1000, and this is -1. | yes |
| Blur Dimensions "Both", written with a capital is refused with a sentence, and nothing changes | Fast Blur (Legacy)'s dimensions are "both", "horizontal" or "vertical", and this is "Both". | yes |
| edges "on" is refused with a sentence, and nothing changes | Fast Blur (Legacy)'s edges are "transparent" or "repeat", and this is "on". | yes |
| blurriness keyed to 1200 is refused with a sentence, and nothing changes | Fast Blur (Legacy)'s blurriness runs from 0 to 1000, and this is 1200. | yes |
| blurriness 20, edges repeated, horizontal is taken | taken | yes |
| blurriness keyed from 0 to 1000 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## A half-size draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| A half-size draft halves the blurriness | FastBlurLegacy { blurriness: 10.0, edges: "transparent", dimensions: "both" } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastleg_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastleg_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastleg_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastleg_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastleg_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: true; the same warnings: false | yes |
| fx_fastleg_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_fastleg_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1609722 pixels changed | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10 on three layers, frame 0, Full | largest difference 1 of 255, 913 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10 on three layers, frame 100, Full | largest difference 1 of 255, 1038 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10 on three layers, frame 239, Full | largest difference 1 of 255, 809 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10 on three layers, frame 0, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10 on three layers, frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 10 on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1753397 pixels changed | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal on three layers, frame 0, Full | largest difference 1 of 255, 917 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal on three layers, frame 100, Full | largest difference 1 of 255, 1021 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal on three layers, frame 239, Full | largest difference 1 of 255, 794 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal on three layers, frame 100, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Blur (Legacy) Blurriness 60, Repeat Edge Pixels, horizontal on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-471 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_blurriness_20.png, Fast Blur (Legacy) Blurriness 20, and 3_fast_box_radius_7_4.png, Fast Box Blur radius 7.4 with 3 iterations: the same soft street, byte for byte; both draw cleanly | [] [], changed true, the same true | yes |
| 4_horizontal_40.png, Blurriness 40, Blur Dimensions horizontal: smeared sideways only, so neighbours across differ far less than in the street while neighbours down differ about as much; draws cleanly | [], across 35058 against 236569, down 301838 against 305580 | yes |

## Result

92 of 92 checks pass.
