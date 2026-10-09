# D-361: Spin & Zoom Blur

B-240, after CycoreFX's CC Radial Blur: each pixel averaged along a circle round a centre or a line through it, six ways: Straight, Fading and Centered Zoom, Rotate, Scratch and Rotate Fading, with a Quality setting for how many points it reads. Every expected pixel is `Fixtures/spin_zoom_blur/expected_spin_zoom_blur.json`, written by `tools/spin_zoom_blur_reference.py` before this code existed and printed in document 25 as FX-SPINZOOM-001 to 022. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SPINZOOM-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPINZOOM-001 frame 0: Straight zoom 30 about the middle: every edge streaks outward, evenly; Radial Blur's zoom 30, FX-RADIAL-002, exactly. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-002 frame 0: Fading zoom 30: the same streaks, fading as they run out, so the drawing itself stays stronger. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-003 frame 0: Centered zoom 30: streaks both outward and inward, half as long each way. | largest difference 2.4e-7 | yes |
| FX-SPINZOOM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-004 frame 0: Rotate 30: every edge streaks clockwise only, so the line down the left edge streaks upward past its top and not past its foot. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-005 frame 0: Rotate -30: anticlockwise, the line streaking past its foot instead. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-006 frame 0: Scratch 30: both ways evenly; Radial Blur's spin 30, FX-RADIAL-001, exactly. | largest difference 2.1e-7 | yes |
| FX-SPINZOOM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-007 frame 0: Rotate fading 30: clockwise, fading as it goes. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-008 frame 0: Straight zoom 60, quality 10: a fifth of the points, so the streaks break into separate copies. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-009 frame 0: Rotate 30, quality 100: twice the points, smoother than FX-SPINZOOM-004. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-010 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-011 frame 0: Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as a wheel spinning up. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-011 frame 2: Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as a wheel spinning up. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-011 frame 4: Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as a wheel spinning up. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-012 frame 0: Rotate 30 about the top left corner, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of it. | largest difference 1.8e-7 | yes |
| FX-SPINZOOM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-013 frame 0: Straight zoom -30: the points run outward from the pixel, so the streaks run inward, toward the centre. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-014 frame 0: A directional blur, direction 90 and length 4, then rotate fading 30 about 0, 0: the grown layer is read, the centre still the drawing's corner. | largest difference 1.6e-7 | yes |
| FX-SPINZOOM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-015 frame 0: Rotate 360 about -1000, -1000: every path is longer than 255 pixels, so each takes the most points, 256. | largest difference 1.5e-9 | yes |
| FX-SPINZOOM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-016 frame 0: Amount 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-016 frame 4: Amount 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-017 frame 0: Amount -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-017 frame 4: Amount -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-018 frame 0: Quality 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-018 frame 4: Quality 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-019 frame 0: Quality 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-019 frame 4: Quality 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-020 frame 0: Type "spin", which is Radial Blur's word, not one of these. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-020 frame 4: Type "spin", which is Radial Blur's word, not one of these. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-021 frame 0: Type "Rotate": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-021 frame 4: Type "Rotate": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-022 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-022 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth, as Radial Blur | 0 | yes |
| a half-size draft preview keeps every setting: the amount is degrees or a share of the distance, the centre a share of the size, and the quality points a pixel, so the paths halve with the picture | SpinZoomBlur { kind: "centered_zoom", amount: 30.0, quality: 80.0, center: [40.0, 60.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spinzoom_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spinzoom_001.json, which leaves out the quality, is saved without it: 50 is the default | {"amount":30,"center":[50,50],"type":"straight_zoom"} | yes |
| fx_spinzoom_020.json is refused in a sentence | Spin & Zoom Blur's type is "straight_zoom", "fading_zoom", "centered_zoom", "rotate", "scratch" or "rotate_fading", and this is "spin". | yes |
| a file with a Spin & Zoom Blur with no `type` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Spin & Zoom Blur whose centre is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 360.5 is refused with a sentence, and nothing changes | Spin & Zoom Blur's amount runs from -360 to 360, and this is 360.5. | yes |
| quality 0.5 is refused with a sentence, and nothing changes | Spin & Zoom Blur's quality runs from 1 to 100, and this is 0.5. | yes |
| type "Scratch", written with a capital is refused with a sentence, and nothing changes | Spin & Zoom Blur's type is "straight_zoom", "fading_zoom", "centered_zoom", "rotate", "scratch" or "rotate_fading", and this is "Scratch". | yes |
| centre 1000.5, 50 is refused with a sentence, and nothing changes | Spin & Zoom Blur's center runs from -1000 to 1000, and this is 1000.5. | yes |
| amount keyed to 400 is refused with a sentence, and nothing changes | Spin & Zoom Blur's amount runs from -360 to 360, and this is 400. | yes |
| type rotate, amount 45, quality 80, centre 40, 60, is taken | taken | yes |
| centre keyed from 0, 50 to 100, 50 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spinzoom_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spinzoom_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spinzoom_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spinzoom_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spinzoom_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spinzoom_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spinzoom_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1924431 pixels changed | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle) on three layers, frame 0, Full | largest difference 1 of 255, 22967 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle) on three layers, frame 100, Full | largest difference 1 of 255, 22803 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle) on three layers, frame 239, Full | largest difference 1 of 255, 22560 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 1155 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur as added (Straight Zoom 10, quality 50, about the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 1121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2050815 pixels changed | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40 on three layers, frame 0, Full | largest difference 1 of 255, 20499 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40 on three layers, frame 100, Full | largest difference 1 of 255, 19981 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40 on three layers, frame 239, Full | largest difference 1 of 255, 20081 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40 on three layers, frame 0, Draft | largest difference 1 of 255, 1131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40 on three layers, frame 100, Draft | largest difference 1 of 255, 1079 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Rotate 30, quality 80, about 30, 40 on three layers, frame 239, Draft | largest difference 1 of 255, 1145 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2053778 pixels changed | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20 on three layers, frame 0, Full | largest difference 1 of 255, 21800 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20 on three layers, frame 100, Full | largest difference 1 of 255, 21039 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20 on three layers, frame 239, Full | largest difference 1 of 255, 21128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20 on three layers, frame 0, Draft | largest difference 1 of 255, 1718 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20 on three layers, frame 100, Draft | largest difference 1 of 255, 1704 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spin & Zoom Blur Fading Zoom -40, quality 20 on three layers, frame 239, Draft | largest difference 1 of 255, 1690 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a car wheel spun and zoomed, in `verification/D-361 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the wheel with no effect; draws cleanly | [], 0 pixels smeared | yes |
| rotate_20.png, Rotate 20 about the middle, a wheel turning: the spokes smeared round into a blur, the hub, where every path is nothing, and the tyre, the same all the way round, as they were; draws cleanly | [], 1472 pixels inside the tyre smeared; hub 50 was 50, tyre 28 was 28 | yes |
| straight_zoom_30.png, Straight Zoom 30, rushing toward the wheel: the tyre's inner edge smeared inward, while a spoke, lying along the streaks, stays as it was 20 pixels out; draws cleanly | [], the tyre's inner edge 202 was 28; spoke 50 was 50 | yes |
| fading_zoom_30.png, Fading Zoom 30: the same streaks fading as they run, so the tyre's inner edge moves less than with Straight Zoom; draws cleanly | [], the tyre's inner edge 194, Straight Zoom's 202, before 28 | yes |
| scratch_20.png, Scratch 20: the turn spread both ways from each pixel, as Radial Blur's spin; the spokes smear as with Rotate, but stay centred where they were; draws cleanly | [], 1422 pixels inside the tyre smeared | yes |

## Result

142 of 142 checks pass.
