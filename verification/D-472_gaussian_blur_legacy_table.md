# D-472: Gaussian Blur (Legacy)

B-352: `core.gaussian_blur_legacy` takes After Effects' Gaussian Blur (Legacy) controls (Blurriness, Blur Dimensions) and draws them with Gaussian Blur's engine in Blurriness, its edges always transparent (D-321; D-383 and D-394, both names, one engine). Every expected pixel is `Fixtures/gaussian_blur_legacy/expected_gaussian_blur_legacy.json`, written by `tools/gaussian_blur_legacy_reference.py` before this code existed and printed in document 25 as FX-GAUSSLEG-001 to 011. Tolerance 2e-5.

## FX-GAUSSLEG-001 to 011 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GAUSSLEG-001 frame 0: Blurriness 10: Gaussian Blur's Blurriness 10, a Gaussian of sigma 3 reaching 20 pixels, FX-BLURRY-001's frame. | largest difference 2.9e-8 | yes |
| FX-GAUSSLEG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-002 frame 0: Blurriness 4: sigma 1.2, reaching 8 pixels; column 11, eight right of the square, is lit and column 12 is clear. | largest difference 7.4e-8 | yes |
| FX-GAUSSLEG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-003 frame 0: Blurriness 10, Blur Dimensions horizontal: spread across only. | largest difference 2.9e-8 | yes |
| FX-GAUSSLEG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-004 frame 0: Blurriness 10, Blur Dimensions vertical: spread down only. | largest difference 2.9e-8 | yes |
| FX-GAUSSLEG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-005 frame 0: Blurriness 0: the drawing untouched. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-006 frame 0: Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is Blurriness 10, FX-GAUSSLEG-001's picture. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-006 frame 2: Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is Blurriness 10, FX-GAUSSLEG-001's picture. | largest difference 2.9e-8 | yes |
| FX-GAUSSLEG-006 frame 4: Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is Blurriness 10, FX-GAUSSLEG-001's picture. | largest difference 9.1e-9 | yes |
| FX-GAUSSLEG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-007 frame 0: A Float composition: Blurriness 4, then Exposure +6. The light falls away smoothly for eight pixels right of the square and no further. | largest difference 4.7e-6 | yes |
| FX-GAUSSLEG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GAUSSLEG-008 frame 0: Blurriness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-008 frame 4: Blurriness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-008: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GAUSSLEG-009 frame 0: Blurriness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-009 frame 4: Blurriness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GAUSSLEG-010 frame 0: Blur Dimensions "Both": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-010 frame 4: Blur Dimensions "Both": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GAUSSLEG-011 frame 0: Blurriness keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-011 frame 4: Blurriness keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-GAUSSLEG-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## One engine

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GAUSSLEG-001's drawing with Gaussian Blur, Blurriness 10 in Blurriness, in its place draws FX-GAUSSLEG-001's frame (Blurriness 10) | largest difference 2.9e-8 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gaussleg_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gaussleg_003.json is saved with its blurriness and dimensions, and no sigma, units or edges of its own | {"blurriness":10,"dimensions":"horizontal"} | yes |
| fx_gaussleg_008.json is refused in a sentence naming Gaussian Blur (Legacy) and its blurriness | Gaussian Blur (Legacy)'s blurriness runs from 0 to 500, and this is -1. | yes |
| fx_gaussleg_009.json is refused in a sentence naming Gaussian Blur (Legacy) and its blurriness | Gaussian Blur (Legacy)'s blurriness runs from 0 to 500, and this is 501. | yes |
| fx_gaussleg_010.json is refused in a sentence naming Gaussian Blur (Legacy) and its dimensions | Gaussian Blur (Legacy)'s dimensions are "both", "horizontal" or "vertical", and this is "Both". | yes |
| a file with a Gaussian Blur (Legacy) whose blurriness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Gaussian Blur (Legacy) whose dimensions is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blurriness 501 is refused with a sentence, and nothing changes | Gaussian Blur (Legacy)'s blurriness runs from 0 to 500, and this is 501. | yes |
| blurriness -1 is refused with a sentence, and nothing changes | Gaussian Blur (Legacy)'s blurriness runs from 0 to 500, and this is -1. | yes |
| Blur Dimensions "Both", written with a capital is refused with a sentence, and nothing changes | Gaussian Blur (Legacy)'s dimensions are "both", "horizontal" or "vertical", and this is "Both". | yes |
| blurriness keyed to 600 is refused with a sentence, and nothing changes | Gaussian Blur (Legacy)'s blurriness runs from 0 to 500, and this is 600. | yes |
| blurriness 20, vertical is taken | taken | yes |
| blurriness keyed from 0 to 500 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## A half-size draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| A half-size draft halves the blurriness | GaussianBlurLegacy { blurriness: 10.0, dimensions: "both" } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gaussleg_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gaussleg_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gaussleg_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gaussleg_006.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gaussleg_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: true; the same warnings: false | yes |
| fx_gaussleg_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gaussleg_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1497920 pixels changed | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10 on three layers, frame 0, Full | largest difference 1 of 255, 21794 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10 on three layers, frame 100, Full | largest difference 1 of 255, 22108 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10 on three layers, frame 239, Full | largest difference 1 of 255, 21704 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10 on three layers, frame 0, Draft | largest difference 1 of 255, 1536 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10 on three layers, frame 100, Draft | largest difference 1 of 255, 1536 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 10 on three layers, frame 239, Draft | largest difference 1 of 255, 1551 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2002693 pixels changed | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60 on three layers, frame 0, Full | largest difference 1 of 255, 22641 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60 on three layers, frame 100, Full | largest difference 1 of 255, 22159 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60 on three layers, frame 239, Full | largest difference 1 of 255, 22206 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60 on three layers, frame 0, Draft | largest difference 1 of 255, 934 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60 on three layers, frame 100, Draft | largest difference 1 of 255, 857 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gaussian Blur (Legacy) Blurriness 60 on three layers, frame 239, Draft | largest difference 1 of 255, 883 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-472 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_blurriness_20.png, Gaussian Blur (Legacy) Blurriness 20, and 3_gaussian_blur_20.png, Gaussian Blur at Blurriness 20: the same soft street, byte for byte; both draw cleanly | [] [], changed true, the same true | yes |
| 4_horizontal_40.png, Blurriness 40, Blur Dimensions horizontal: smeared sideways only, so neighbours across differ far less than in the street while neighbours down differ about as much; draws cleanly | [], across 46839 against 236569, down 304736 against 305580 | yes |

## Result

82 of 82 checks pass.
