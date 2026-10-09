# D-387: Fisheye

B-266, after CycoreFX's CC Lens: the layer seen through a round lens Size per cent of half its diagonal across, bulging like a glass ball for a positive Convergence and pinched like a dish for a negative one; outside the circle is left clear, with a soft one-pixel rim. The formulas are this program's own. Every expected pixel is `Fixtures/fisheye/expected_fisheye.json`, written by `tools/fisheye_reference.py` before this code existed and printed in document 25 as FX-FISHEYE-001 to 020. Tolerance 2e-5.

## FX-FISHEYE-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FISHEYE-001 frame 0: The settings as they start: the lens in the middle, its radius a quarter of the diagonal, 4.7 pixels here, convergence 50: the stripes swell out from the middle inside a circle, and the rest is transparent. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-002 frame 0: Size 100, convergence 0: the lens reaches the corners, every pixel covered: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-003 frame 0: Size 50, convergence 0: the drawing cut to the circle, its rim soft to a pixel. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-004 frame 0: Size 100, convergence 100: the whole drawing laid round a ball, the middle grown by half again and the rim squeezed. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-005 frame 0: Size 100, convergence -100: the other way, the middle drawn in and the rim stretched. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-006 frame 0: Size 100, convergence 50: half way to the ball. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-007 frame 0: Size 30 at the left quarter: a small lens over the left stripes, the rest transparent. | largest difference 1.7e-7 | yes |
| FX-FISHEYE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-008 frame 0: Size 0: nothing drawn. | largest difference 0.0e0 | yes |
| FX-FISHEYE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-009 frame 0: Size keyed from 0 at frame 0 to 100 at frame 4, linear: the lens opens from nothing; frame 2 FX-FISHEYE-001. | largest difference 0.0e0 | yes |
| FX-FISHEYE-009 frame 2: Size keyed from 0 at frame 0 to 100 at frame 4, linear: the lens opens from nothing; frame 2 FX-FISHEYE-001. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-009 frame 4: Size keyed from 0 at frame 0 to 100 at frame 4, linear: the lens opens from nothing; frame 2 FX-FISHEYE-001. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-010 frame 0: Convergence keyed from -100 at frame 0 to 100 at frame 4, size 100: frame 0 FX-FISHEYE-005, frame 2 the drawing, frame 4 FX-FISHEYE-004. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-010 frame 2: Convergence keyed from -100 at frame 0 to 100 at frame 4, size 100: frame 0 FX-FISHEYE-005, frame 2 the drawing, frame 4 FX-FISHEYE-004. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-010 frame 4: Convergence keyed from -100 at frame 0 to 100 at frame 4, size 100: frame 0 FX-FISHEYE-005, frame 2 the drawing, frame 4 FX-FISHEYE-004. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-011 frame 0: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the lens slides across. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-011 frame 2: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the lens slides across. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-011 frame 4: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the lens slides across. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-012 frame 0: FX-FISHEYE-001 moved three pixels right: the same, moved; nothing grows. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-013 frame 0: Size eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 0.0e0 | yes |
| FX-FISHEYE-013 frame 2: Size eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-013 frame 4: Size eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: at frame 2 it would pass 1000 and is held there. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-014 frame 0: Size 1000, convergence 100: a lens ten times the drawing, so only its middle shows here: the drawing grown by about pi / 2 about the centre. | largest difference 2.5e-7 | yes |
| FX-FISHEYE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FISHEYE-015 frame 0: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-015 frame 4: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FISHEYE-016 frame 0: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-016 frame 4: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FISHEYE-017 frame 0: Convergence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-017 frame 4: Convergence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FISHEYE-018 frame 0: Convergence -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-018 frame 4: Convergence -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FISHEYE-019 frame 0: Centre at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-019 frame 4: Centre at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FISHEYE-020 frame 0: Convergence keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-020 frame 4: Convergence keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FISHEYE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fisheye_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fisheye_007.json is saved with its centre, size and convergence | {"center":[25,50],"convergence":50,"size":30} | yes |
| fx_fisheye_015.json is refused in a sentence | Fisheye's size runs from 0 to 1000, and this is 1001. | yes |
| fx_fisheye_016.json is refused in a sentence | Fisheye's size runs from 0 to 1000, and this is -1. | yes |
| fx_fisheye_017.json is refused in a sentence | Fisheye's convergence runs from -100 to 100, and this is 101. | yes |
| fx_fisheye_018.json is refused in a sentence | Fisheye's convergence runs from -100 to 100, and this is -101. | yes |
| fx_fisheye_019.json is refused in a sentence | Fisheye's center runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Fisheye with no `size` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fisheye whose center is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 1000.5 is refused with a sentence, and nothing changes | Fisheye's size runs from 0 to 1000, and this is 1000.5. | yes |
| size -0.5 is refused with a sentence, and nothing changes | Fisheye's size runs from 0 to 1000, and this is -0.5. | yes |
| convergence 100.5 is refused with a sentence, and nothing changes | Fisheye's convergence runs from -100 to 100, and this is 100.5. | yes |
| center 1000.5, 50 is refused with a sentence, and nothing changes | Fisheye's center runs from -1000 to 1000, and this is 1000.5. | yes |
| convergence keyed to -150 is refused with a sentence, and nothing changes | Fisheye's convergence runs from -100 to 100, and this is -150. | yes |
| centre 40, 60, size 80, convergence -70 is taken | taken | yes |
| size keyed from 10 to 120 is taken | taken | yes |
| center keyed from 30, 50 to 70, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fisheye_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fisheye_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fisheye_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fisheye_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fisheye_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fisheye_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fisheye_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fisheye_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fisheye_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fisheye_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fisheye_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fisheye_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fisheye_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2015490 pixels changed | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50) on three layers, frame 0, Full | largest difference 1 of 255, 2123 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50) on three layers, frame 100, Full | largest difference 1 of 255, 945 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50) on three layers, frame 239, Full | largest difference 1 of 255, 873 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50) on three layers, frame 0, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50) on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye as it starts (size 50, convergence 50) on three layers, frame 239, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2040431 pixels changed | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100 on three layers, frame 0, Full | largest difference 1 of 255, 1424 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100 on three layers, frame 100, Full | largest difference 1 of 255, 985 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100 on three layers, frame 239, Full | largest difference 1 of 255, 1111 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100 on three layers, frame 0, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100 on three layers, frame 100, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a full ball, size 100, convergence 100 on three layers, frame 239, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2020134 pixels changed | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100 on three layers, frame 0, Full | largest difference 1 of 255, 4417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100 on three layers, frame 100, Full | largest difference 1 of 255, 1077 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100 on three layers, frame 239, Full | largest difference 1 of 255, 3066 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100 on three layers, frame 0, Draft | largest difference 1 of 255, 105 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100 on three layers, frame 100, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a dish at the left, size 70, convergence -100 on three layers, frame 239, Draft | largest difference 1 of 255, 103 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1949140 pixels changed | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fisheye a small flat lens off the frame's edge on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-387 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (size 50, convergence 50): the middle of the street bulges in a round lens, the corners left clear; draws cleanly | [], 111802 pixels changed | yes |
| 3_ball_full.png, size 120, convergence 100: the whole street wrapped round a glass ball; draws cleanly | [], 111579 pixels changed | yes |
| 4_dish.png, size 100, convergence -100: the middle of the street pinched small, the rim stretched; draws cleanly | [], 117169 pixels changed | yes |

## Result

149 of 149 checks pass.
