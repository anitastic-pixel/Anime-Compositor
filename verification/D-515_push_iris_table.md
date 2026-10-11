# D-515: Push Iris

B-395, after CycoreFX's CC Radial ScaleWipe (Transition): a round hole opens at the Center and pushes the picture out of its way, its middle stretched round the hole's edge; reversed, the outside closes in like an iris and the picture is pulled toward the middle. The CycoreFX manual says only "radial wipe where edge folds" and names the controls, so the rule and the settings when added are ours, stated in `tools/push_iris_reference.py`. Every expected pixel is `Fixtures/push_iris/expected_push_iris.json`, written by the tool before this code existed and printed in document 25 as FX-PUSH-001 to 017. Tolerance 2e-5.

## FX-PUSH-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PUSH-001 frame 0: The settings as they start: completion 0, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-PUSH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-002 frame 0: Completion 100: every pixel clear. | largest difference 0.0e0 | yes |
| FX-PUSH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-003 frame 0: Completion 30: a hole in the middle, the picture pushed out round it, its middle stretched round the hole's edge. | largest difference 1.5e-7 | yes |
| FX-PUSH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-004 frame 0: Completion 60: a bigger hole, more of the picture pushed off. | largest difference 1.1e-7 | yes |
| FX-PUSH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-005 frame 0: Reversed, completion 30: the outside closed in, the picture pulled toward the middle. | largest difference 1.1e-7 | yes |
| FX-PUSH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-006 frame 0: Center [25, 30], completion 30: the hole moved up and to the left. | largest difference 1.4e-7 | yes |
| FX-PUSH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-007 frame 0: Completion 7.3: a small hole, every pixel read between pixels. | largest difference 1.4e-7 | yes |
| FX-PUSH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-008 frame 0: Center [120, 50], off the drawing to the right, completion 40: the hole's edge sweeps in from the right. | largest difference 1.5e-7 | yes |
| FX-PUSH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-009 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is completion 50, frame 4 all clear. | largest difference 1.9e-7 | yes |
| FX-PUSH-009 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is completion 50, frame 4 all clear. | largest difference 1.4e-7 | yes |
| FX-PUSH-009 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is completion 50, frame 4 all clear. | largest difference 0.0e0 | yes |
| FX-PUSH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-010 frame 0: FX-PUSH-003 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-PUSH-010 frame 3: FX-PUSH-003 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-PUSH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-011 frame 0: Reversed, completion 65, Center [60, 40]. | largest difference 3.7e-8 | yes |
| FX-PUSH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PUSH-012 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-012 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PUSH-013 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-013 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PUSH-014 frame 0: Center x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-014 frame 4: Center x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PUSH-015 frame 0: Reverse "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-015 frame 4: Reverse "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PUSH-016 frame 0: Reverse "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-016 frame 4: Reverse "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PUSH-017 frame 0: Completion keyed to 150 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-017 frame 4: Completion keyed to 150 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PUSH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_push_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_push_011.json is saved with its three settings, reversed, completion 65, Center [60, 40] | {"center":[60,40],"completion":65,"reverse":"on"} | yes |
| fx_push_012.json is refused in a sentence naming completion | Push Iris's completion runs from 0 to 100, and this is 101. | yes |
| fx_push_013.json is refused in a sentence naming completion | Push Iris's completion runs from 0 to 100, and this is -1. | yes |
| fx_push_014.json is refused in a sentence naming center | Push Iris's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_push_015.json is refused in a sentence naming reverse | Push Iris's reverse is "off" or "on", and this is "yes". | yes |
| fx_push_016.json is refused in a sentence naming reverse | Push Iris's reverse is "off" or "on", and this is "On". | yes |
| a file with a Push Iris whose completion is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Push Iris whose Center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-515)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_push_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_push_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Push Iris completion 30 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069827 pixels changed | yes |
| the reference shot, Push Iris completion 30 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 30 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 318 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 30 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 30 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 30 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 22 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 30 (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60]: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073590 pixels changed | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60] on three layers, frame 0, Full | largest difference 1 of 255, 328 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60] on three layers, frame 100, Full | largest difference 1 of 255, 438 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60] on three layers, frame 239, Full | largest difference 1 of 255, 248 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60] on three layers, frame 0, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60] on three layers, frame 100, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 50, reversed, Center [30, 60] on three layers, frame 239, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 70, Center [120, 50]: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Push Iris completion 70, Center [120, 50] on three layers, frame 0, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 70, Center [120, 50] on three layers, frame 100, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 70, Center [120, 50] on three layers, frame 239, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 70, Center [120, 50] on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 70, Center [120, 50] on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion 70, Center [120, 50] on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion keyed 10 to 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2071662 pixels changed | yes |
| the reference shot, Push Iris completion keyed 10 to 90 on three layers, frame 0, Full | largest difference 1 of 255, 932 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion keyed 10 to 90 on three layers, frame 100, Full | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion keyed 10 to 90 on three layers, frame 239, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion keyed 10 to 90 on three layers, frame 0, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion keyed 10 to 90 on three layers, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Push Iris completion keyed 10 to 90 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-515 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_hole.png: drawn without a warning, the pixel at (240, 135) clear, the one at (2, 2) covered | 125979 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [135, 177, 225, 255]; warnings [] | yes |
| 2_bigger_hole.png: drawn without a warning, the pixel at (240, 135) clear, the one at (2, 2) covered | 128990 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [150, 185, 220, 255]; warnings [] | yes |
| 3_iris_closing.png: drawn without a warning, the pixel at (2, 2) clear, the one at (240, 135) covered | 127314 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [200, 180, 120, 255]; warnings [] | yes |
| 4_off_centre.png: drawn without a warning, the pixel at (96, 189) clear, the one at (470, 2) covered | 121563 pixels changed; at (96, 189) [0, 0, 0, 0]; at (470, 2) [148, 184, 221, 255]; warnings [] | yes |
| 5_done.png: drawn without a warning, the pixel at (2, 2) clear, every pixel clear | 129600 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [0, 0, 0, 0]; warnings [] | yes |

## Result

118 of 118 checks pass.
