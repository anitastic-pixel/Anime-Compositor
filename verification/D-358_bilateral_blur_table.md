# D-358: Bilateral Blur

B-237, after After Effects' Bilateral Blur: each pixel mixes with the pixels round it, weighed by how near they are and how alike, so grain and flat colour are smoothed and strong edges kept. Every expected pixel is `Fixtures/bilateral_blur/expected_bilateral_blur.json`, written by `tools/bilateral_blur_reference.py` before this code existed and printed in document 25 as FX-BILAT-001 to 017. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BILAT-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BILAT-001 frame 0: Bilateral Blur as it starts, Radius 5, Threshold 20, Colorize on: the grain, 8 levels either side of the skin, is smoothed toward the skin, while the line and the dark specks, far more than 20 from the skin, stay as they are to well within a level; the white speck, 41 and 65 from the skin in green and blue, stays far lighter than the skin. The hole stays a hole and the half-covered column keeps its half covering. | largest difference 2.1e-7 | yes |
| FX-BILAT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-002 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BILAT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-003 frame 0: Threshold 0: each pixel counts only itself, so the drawing is unchanged. | largest difference 1.9e-7 | yes |
| FX-BILAT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-004 frame 0: Colorize off: the same smoothing worked on luminance alone, and every pixel that shows is grey, red, green and blue equal; the line stays dark and the outline is kept. | largest difference 1.3e-7 | yes |
| FX-BILAT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-005 frame 0: Colorize off with Radius 0: no smoothing, each pixel only its own luminance, grey. | largest difference 1.1e-7 | yes |
| FX-BILAT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-006 frame 0: Threshold 255: likeness hardly matters, a soft blur that stays inside the drawing: the line and the specks are blurred into the skin, and the hole, the empty column and the empty row stay empty. | largest difference 2.0e-7 | yes |
| FX-BILAT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-007 frame 0: Radius 1.5, a disc of nine: a lighter smoothing of the grain than FX-BILAT-001. | largest difference 2.0e-7 | yes |
| FX-BILAT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-008 frame 0: Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 1.9e-7 | yes |
| FX-BILAT-008 frame 2: Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-008 frame 4: Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.9e-7 | yes |
| FX-BILAT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-009 frame 0: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 1.9e-7 | yes |
| FX-BILAT-009 frame 2: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-009 frame 4: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-010 frame 0: Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end (about 64 at frame 2): frame 2 is held at 50, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-BILAT-010 frame 2: Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end (about 64 at frame 2): frame 2 is held at 50, the same as frame 4. | largest difference 3.2e-7 | yes |
| FX-BILAT-010 frame 4: Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end (about 64 at frame 2): frame 2 is held at 50, the same as frame 4. | largest difference 3.2e-7 | yes |
| FX-BILAT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-011 frame 0: Bilateral Blur as it starts, the layer moved three pixels right: FX-BILAT-001 moved with it. | largest difference 2.1e-7 | yes |
| FX-BILAT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-012 frame 0: Bilateral Blur, Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-012 frame 4: Bilateral Blur, Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-013 frame 0: Bilateral Blur, Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-013 frame 4: Bilateral Blur, Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-014 frame 0: Bilateral Blur, Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-014 frame 4: Bilateral Blur, Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-015 frame 0: Bilateral Blur, Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-015 frame 4: Bilateral Blur, Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-016 frame 0: Bilateral Blur, Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-016 frame 4: Bilateral Blur, Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-017 frame 0: Bilateral Blur, Colorize "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-017 frame 4: Bilateral Blur, Colorize "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview halves the radius, 5 to 2.5, and keeps the threshold, which is colour, not distance, and the word | BilateralBlur { radius: 2.5, threshold: 20.0, colorize: "off" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bilat_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_017.json is refused in a sentence | Bilateral Blur's colorize is "off" or "on", and this is "sometimes". | yes |
| a file with a Bilateral Blur with no `colorize` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bilateral Blur with no `threshold` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bilateral Blur whose colorize is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 50.5 is refused with a sentence, and nothing changes | Bilateral Blur's radius runs from 0 to 50, and this is 50.5. | yes |
| threshold 255.5 is refused with a sentence, and nothing changes | Bilateral Blur's threshold runs from 0 to 255, and this is 255.5. | yes |
| colorize "On", written with a capital is refused with a sentence, and nothing changes | Bilateral Blur's colorize is "off" or "on", and this is "On". | yes |
| radius keyed to 60 is refused with a sentence, and nothing changes | Bilateral Blur's radius runs from 0 to 50, and this is 60. | yes |
| radius 50, threshold 255, colorize off, is taken | taken | yes |
| threshold keyed from 0 to 40 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bilat_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bilat_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bilat_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bilat_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bilat_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bilat_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bilat_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bilat_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bilat_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_bilat_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 7 of 10 frames; the same warnings: true | yes |
| fx_bilat_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_bilat_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_bilat_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bilat_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bilat_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1348088 pixels changed | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) on three layers, frame 0, Full | largest difference 1 of 255, 928 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) on three layers, frame 100, Full | largest difference 1 of 255, 1095 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) on three layers, frame 239, Full | largest difference 1 of 255, 832 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) on three layers, frame 0, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2003654 pixels changed | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off on three layers, frame 0, Full | largest difference 1 of 255, 1154 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off on three layers, frame 100, Full | largest difference 1 of 255, 1099 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off on three layers, frame 239, Full | largest difference 1 of 255, 1163 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off on three layers, frame 0, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 8, Threshold 30, Colorize off on three layers, frame 239, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1692324 pixels changed | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on on three layers, frame 0, Full | largest difference 1 of 255, 1045 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on on three layers, frame 100, Full | largest difference 1 of 255, 1151 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on on three layers, frame 239, Full | largest difference 1 of 255, 985 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on on three layers, frame 0, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on on three layers, frame 100, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur Radius 16, Threshold 60, Colorize on on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a scanned face smoothed, in `verification/D-358 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the face with no effect: grainy skin, specks, pinholes; draws cleanly | [], grain 20.7 | yes |
| bilateral_default.png, Bilateral Blur as it starts, Radius 5, Threshold 20, Colorize on: the grain less than half what it was, the line, the eyes and the one-pixel hair kept to within a level, as the skin is far too unlike them to be mixed in, and every pixel's covering as it was, pinholes still clear; draws cleanly | [], grain 2.0 from 20.7, line moved at most 0 levels | yes |
| bilateral_12_30.png, Radius 12, Threshold 30: smoother skin than at the defaults, while the line, 150 to 220 levels from the skin, still stays within 2 levels; draws cleanly | [], grain 0.9, line moved at most 0 levels | yes |
| bilateral_12_60.png, Radius 12, Threshold 60: so high a threshold lets a little of the skin into the line, and the one-pixel hair, outnumbered by the skin round it, is lightened by more than 20 levels: the edge-keeping gives way as Threshold rises; draws cleanly | [], grain 0.5, hair lightened by up to 75 levels | yes |
| bilateral_colorize_off.png, Colorize off: the same smoothing worked on brightness alone, and every pixel that shows grey, red, green and blue equal; covering as it was; draws cleanly | [], every pixel grey: true | yes |

## Result

123 of 123 checks pass.
