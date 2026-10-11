# D-516: Stretch Wipe

B-396, after CycoreFX's CC Scale Wipe (Transition): everything past a line through the Center stretched Scale times along the Direction, so the line's own colours draw out in streaks and moving the Center stretches the layer off the screen or onto it. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/stretch_wipe_reference.py`. Every expected pixel is `Fixtures/stretch_wipe/expected_stretch_wipe.json`, written by the tool before this code existed and printed in document 25 as FX-STRETCH-001 to 017. Tolerance 2e-5.

## FX-STRETCH-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STRETCH-001 frame 0: The settings as they start: scale 100, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-STRETCH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-002 frame 0: Scale 200: the right half stretched twice as wide, from the middle. | largest difference 1.6e-7 | yes |
| FX-STRETCH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-003 frame 0: Scale 400, direction 90: the bottom half stretched four times down. | largest difference 1.9e-7 | yes |
| FX-STRETCH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-004 frame 0: Scale 100000: the right half a streak of the middle column. | largest difference 1.4e-7 | yes |
| FX-STRETCH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-005 frame 0: Scale 300, direction 30: the stretch turned. | largest difference 1.9e-7 | yes |
| FX-STRETCH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-006 frame 0: Center [25, 30], scale 250: stretched from a line further left. | largest difference 1.2e-7 | yes |
| FX-STRETCH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-007 frame 0: Center [-20, 50], scale 150: the line off the drawing to the left, the whole drawing stretched, its left edge pulled in from outside it. | largest difference 1.2e-7 | yes |
| FX-STRETCH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-008 frame 0: Direction 180, scale 300: the left half stretched to the left. | largest difference 1.9e-7 | yes |
| FX-STRETCH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-009 frame 0: Scale keyed from 100 at frame 0 to 500 at frame 4, linear: frame 2 is scale 300. | largest difference 1.9e-7 | yes |
| FX-STRETCH-009 frame 2: Scale keyed from 100 at frame 0 to 500 at frame 4, linear: frame 2 is scale 300. | largest difference 1.9e-7 | yes |
| FX-STRETCH-009 frame 4: Scale keyed from 100 at frame 0 to 500 at frame 4, linear: frame 2 is scale 300. | largest difference 1.4e-7 | yes |
| FX-STRETCH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-010 frame 0: Direction keyed from 0 at frame 0 to 90 at frame 4, linear, scale 300: frame 2 is direction 45. | largest difference 1.9e-7 | yes |
| FX-STRETCH-010 frame 2: Direction keyed from 0 at frame 0 to 90 at frame 4, linear, scale 300: frame 2 is direction 45. | largest difference 1.9e-7 | yes |
| FX-STRETCH-010 frame 4: Direction keyed from 0 at frame 0 to 90 at frame 4, linear, scale 300: frame 2 is direction 45. | largest difference 1.9e-7 | yes |
| FX-STRETCH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-011 frame 0: FX-STRETCH-002 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-STRETCH-011 frame 3: FX-STRETCH-002 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-STRETCH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-012 frame 0: Direction -135, Center [60, 40], scale 175. | largest difference 1.4e-7 | yes |
| FX-STRETCH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRETCH-013 frame 0: Scale 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-013 frame 4: Scale 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRETCH-014 frame 0: Scale 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-014 frame 4: Scale 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRETCH-015 frame 0: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-015 frame 4: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRETCH-016 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-016 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRETCH-017 frame 0: Scale keyed to 200000 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-017 frame 4: Scale keyed to 200000 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRETCH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stretch_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stretch_012.json is saved with its three settings, direction -135, Center [60, 40], scale 175 | {"center":[60,40],"direction":-135,"scale":175} | yes |
| fx_stretch_013.json is refused in a sentence naming scale | Stretch Wipe's scale runs from 100 to 100000, and this is 99. | yes |
| fx_stretch_014.json is refused in a sentence naming scale | Stretch Wipe's scale runs from 100 to 100000, and this is 100001. | yes |
| fx_stretch_015.json is refused in a sentence naming center | Stretch Wipe's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_stretch_016.json is refused in a sentence naming direction | Stretch Wipe's direction runs from -3600 to 3600, and this is 3601. | yes |
| a file with a Stretch Wipe whose scale is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Stretch Wipe whose Center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-516)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stretch_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stretch_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1010053 pixels changed | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 1766 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 1507 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 300 (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1030649 pixels changed | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90 on three layers, frame 0, Full | largest difference 1 of 255, 3265 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90 on three layers, frame 100, Full | largest difference 1 of 255, 1265 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90 on three layers, frame 239, Full | largest difference 1 of 255, 1481 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90 on three layers, frame 0, Draft | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90 on three layers, frame 100, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 1000, direction 90 on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40]: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1105937 pixels changed | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40] on three layers, frame 0, Full | largest difference 1 of 255, 3211 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40] on three layers, frame 100, Full | largest difference 1 of 255, 907 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40] on three layers, frame 239, Full | largest difference 1 of 255, 1474 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40] on three layers, frame 0, Draft | largest difference 1 of 255, 126 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40] on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale 175, direction -135, Center [60, 40] on three layers, frame 239, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1017207 pixels changed | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000 on three layers, frame 0, Full | largest difference 1 of 255, 2304 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000 on three layers, frame 100, Full | largest difference 1 of 255, 1421 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000 on three layers, frame 239, Full | largest difference 1 of 255, 1530 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000 on three layers, frame 0, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000 on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stretch Wipe scale keyed 150 to 5000 on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-516 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_stretched.png: drawn without a warning, pixels changed, the one at (100, 135) behind the line as the street's | 17512 pixels changed; at (100, 135) [170, 195, 215, 255]; warnings [] | yes |
| 2_down.png: drawn without a warning, pixels changed, the one at (240, 2) behind the line as the street's | 48468 pixels changed; at (240, 2) [120, 170, 229, 255]; warnings [] | yes |
| 3_turned.png: drawn without a warning, pixels changed, the one at (470, 268) behind the line as the street's | 59600 pixels changed; at (470, 268) [60, 60, 66, 255]; warnings [] | yes |
| 4_off_the_left.png: drawn without a warning, every pixel stretched | 46080 pixels changed; at (0, 0) [0, 0, 0, 0]; warnings [] | yes |
| 5_streaks.png: drawn without a warning, scale 100000: along every row the right half lies between the street's two middle columns, within 1 level | 24960 pixels changed; every row from x 241 to 479 between columns 239 and 240: true; warnings [] | yes |

## Result

118 of 118 checks pass.
