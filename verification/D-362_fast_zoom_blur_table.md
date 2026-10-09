# D-362: Fast Zoom Blur

B-241, after CycoreFX's CC Radial Fast Blur: each pixel reads back along the line toward a centre, the nearer points counting more, so streaks fade as they run out; Brightest keeps only the lightest of them and Darkest only the darkest. Every expected pixel is `Fixtures/fast_zoom_blur/expected_fast_zoom_blur.json`, written by `tools/fast_zoom_blur_reference.py` before this code existed and printed in document 25 as FX-FASTZOOM-001 to 014. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-FASTZOOM-001 to 014 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FASTZOOM-001 frame 0: Standard, amount 50 (the default), about the middle: every edge streaks outward, fading as it runs. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-002 frame 0: Brightest: only light streaks; the skin block streaks outward over the clear pixels round it, the dark line, with only clear pixels behind it, is kept, and nothing grows darker anywhere. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-003 frame 0: Darkest: only dark streaks; the line down the left edge, with clear pixels behind it, fades to a quarter, the skin block, whose streaks run back over itself, is kept, and nothing grows lighter. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-004 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-005 frame 0: Standard, amount 100, the most: each pixel reads all the way back to the centre. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-006 frame 0: Brightest about centre 25, 50, the point (4, 5): the block streaks right, away from it. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-007 frame 0: Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-007 frame 2: Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-007 frame 4: Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-008 frame 0: Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest amount 80, as a light passing along a line of text. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-008 frame 2: Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest amount 80, as a light passing along a line of text. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-008 frame 4: Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest amount 80, as a light passing along a line of text. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-009 frame 0: Standard, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of it. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-010 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-010 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-011 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-011 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-012 frame 0: Zoom "bright", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-012 frame 4: Zoom "bright", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-013 frame 0: Zoom "Standard": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-013 frame 4: Zoom "Standard": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-014 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-014 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth, as Radial Blur | 0 | yes |
| a half-size draft preview keeps every setting: the amount is a share of the distance and the centre a share of the size, so the streaks halve with the picture | FastZoomBlur { amount: 70.0, center: [30.0, 40.0], zoom: "darkest" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastzoom_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fastzoom_001.json, which leaves out the zoom, is saved without it: standard is the default | {"amount":50,"center":[50,50]} | yes |
| fx_fastzoom_012.json is refused in a sentence | Fast Zoom Blur's zoom is "standard", "brightest" or "darkest", and this is "bright". | yes |
| a file with a Fast Zoom Blur with no `amount` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fast Zoom Blur whose zoom is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 100.5 is refused with a sentence, and nothing changes | Fast Zoom Blur's amount runs from 0 to 100, and this is 100.5. | yes |
| amount -0.5 is refused with a sentence, and nothing changes | Fast Zoom Blur's amount runs from 0 to 100, and this is -0.5. | yes |
| zoom "Brightest", written with a capital is refused with a sentence, and nothing changes | Fast Zoom Blur's zoom is "standard", "brightest" or "darkest", and this is "Brightest". | yes |
| centre -1000.5, 50 is refused with a sentence, and nothing changes | Fast Zoom Blur's center runs from -1000 to 1000, and this is -1000.5. | yes |
| amount keyed to 120 is refused with a sentence, and nothing changes | Fast Zoom Blur's amount runs from 0 to 100, and this is 120. | yes |
| amount 70, centre 30, 40, brightest, is taken | taken | yes |
| centre keyed from 0, 50 to 100, 50 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastzoom_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastzoom_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastzoom_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastzoom_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fastzoom_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fastzoom_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fastzoom_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2067383 pixels changed | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle) on three layers, frame 0, Full | largest difference 1 of 255, 20317 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle) on three layers, frame 100, Full | largest difference 1 of 255, 20572 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle) on three layers, frame 239, Full | largest difference 1 of 255, 19831 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 1054 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 1013 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur as added (amount 50, standard, about the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 1015 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1965476 pixels changed | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30 on three layers, frame 0, Full | largest difference 1 of 255, 48344 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30 on three layers, frame 100, Full | largest difference 1 of 255, 48366 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30 on three layers, frame 239, Full | largest difference 1 of 255, 47214 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30 on three layers, frame 0, Draft | largest difference 1 of 255, 4347 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30 on three layers, frame 100, Draft | largest difference 1 of 255, 3063 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 80, brightest, about 20, 30 on three layers, frame 239, Draft | largest difference 1 of 255, 4437 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1966058 pixels changed | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60 on three layers, frame 0, Full | largest difference 1 of 255, 34333 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60 on three layers, frame 100, Full | largest difference 1 of 255, 33606 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60 on three layers, frame 239, Full | largest difference 1 of 255, 33242 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60 on three layers, frame 0, Draft | largest difference 1 of 255, 5307 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60 on three layers, frame 100, Draft | largest difference 1 of 255, 5096 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fast Zoom Blur amount 30, darkest, about 70, 60 on three layers, frame 239, Draft | largest difference 1 of 255, 5484 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: light streaking out of a title, in `verification/D-362 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the title with no effect; draws cleanly | [] | yes |
| standard_50.png, Fast Zoom Blur as it starts, amount 50, standard: the letters streak outward, fading, and the card's blue streaks in over them, so pixels both lighten and darken; draws cleanly | [], 7042 pixels lighter, 1824 darker | yes |
| brightest_40.png, amount 40, brightest, about a light up and left of the title: light rays thrown down and right from every letter over the card, and no pixel darker; draws cleanly | [], 4797 pixels lighter, 0 darker | yes |
| darkest_40.png, amount 40, darkest: the card's blue eats into the letters' edges and slits, away from the centre, and no pixel lighter; draws cleanly | [], 0 pixels lighter, 1800 darker | yes |

## Result

109 of 109 checks pass.
