# D-676: Scatterize

B-555, after CycoreFX's CC Scatterize (Simulation): every pixel of the layer that shows becomes a dot, twisted with its column about the middle row by a share of the left and right twists and thrown away from where it was by up to Amount pixels on a picture 1080 high, then laid farthest first onto a clear layer by the transfer mode; dots thrown past the layer's edge are cut there. Nothing moves by itself: a frame is the settings as they are at that frame (document 20). CycoreFX publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/scatterize_reference.py`. Every expected pixel is `Fixtures/scatterize/expected_scatterize.json`, written by the tool before this code existed and printed in document 25 as FX-SCATZ-001 to 023. Tolerance 2e-5.

## FX-SCATZ-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SCATZ-001 frame 3: As added: on a picture 32 pixels high each dot is thrown up to a pixel and a half, a fine dusting. | largest difference 1.6e-7 | yes |
| FX-SCATZ-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-002 frame 3: Amount 300: a cloud of dots thrown up to about 9 pixels. | largest difference 1.5e-7 | yes |
| FX-SCATZ-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-003 frame 3: Amount -300: every dot thrown the opposite way to FX-SCATZ-002. | largest difference 1.7e-7 | yes |
| FX-SCATZ-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-004 frame 3: Amount 0: every dot on its own pixel, the picture untouched. | largest difference 1.5e-7 | yes |
| FX-SCATZ-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-005 frame 3: Right Twist 90, Amount 0: the right edge turned edge-on, the picture squeezed towards the middle row more and more to the right. | largest difference 1.6e-7 | yes |
| FX-SCATZ-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-006 frame 3: Left Twist 180, Amount 0: the left edge turned upside down, the picture squeezed through the middle row a third of the way across. | largest difference 1.8e-7 | yes |
| FX-SCATZ-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-007 frame 3: Right Twist 60, Left Twist -60, Amount 100: the two ends turned opposite ways, dusted. | largest difference 1.7e-7 | yes |
| FX-SCATZ-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-008 frame 3: Screen: the cloud of FX-SCATZ-002, the dots screened. | largest difference 1.5e-7 | yes |
| FX-SCATZ-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-009 frame 3: Add: the dots' colours added, so where they bunch they brighten. | largest difference 5.3e-7 | yes |
| FX-SCATZ-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-010 frame 3: Alpha Add: colours and opacities added. | largest difference 5.3e-7 | yes |
| FX-SCATZ-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-011 frame 3: The picture with its right half clear: only the left half's dots, some thrown across into the clear half. | largest difference 1.4e-7 | yes |
| FX-SCATZ-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-012 frame 0: Amount keyed from 0 at frame 0 to 300 at frame 12: at frame 0 the picture untouched, then the cloud gathering out. | largest difference 1.5e-7 | yes |
| FX-SCATZ-012 frame 12: Amount keyed from 0 at frame 0 to 300 at frame 12: at frame 0 the picture untouched, then the cloud gathering out. | largest difference 1.5e-7 | yes |
| FX-SCATZ-012 frame 6: Amount keyed from 0 at frame 0 to 300 at frame 12: at frame 0 the picture untouched, then the cloud gathering out. | largest difference 1.6e-7 | yes |
| FX-SCATZ-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-013 frame 3: FX-SCATZ-002 moved 3 pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-SCATZ-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-014 frame 3: Both twists 360, Amount 0: a whole turn at every column, the picture untouched. | largest difference 1.5e-7 | yes |
| FX-SCATZ-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-015 frame 3: Right Twist 720, Amount 50: the ribbon turned twice over across the picture. | largest difference 1.6e-7 | yes |
| FX-SCATZ-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-016 frame 3: Amount 3000: most dots thrown past the edge, a thin scatter left. | largest difference 1.6e-7 | yes |
| FX-SCATZ-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-017 frame 3: Right Twist 90 with Add, Amount 0: the dots piled where the right edge is edge-on add up bright. | largest difference 2.3e-6 | yes |
| FX-SCATZ-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCATZ-018 frame 0: Amount 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-018 frame 4: Amount 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATZ-019 frame 0: Amount -10001, below -10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-019 frame 4: Amount -10001, below -10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATZ-020 frame 0: Right Twist 36001, above 36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-020 frame 4: Right Twist 36001, above 36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATZ-021 frame 0: Left Twist -36001, below -36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-021 frame 4: Left Twist -36001, below -36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATZ-022 frame 0: Transfer Mode written "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-022 frame 4: Transfer Mode written "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCATZ-023 frame 0: Amount keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-023 frame 4: Amount keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SCATZ-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scatz_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scatz_007.json is saved with its 4 settings, three numbers and a word | 4 settings; amount 100, right twist 60, left twist -60, transfer mode "composite" | yes |
| fx_scatz_018.json is refused in a sentence naming amount | Scatterize's amount runs from -10000 to 10000, and this is 10001. | yes |
| fx_scatz_019.json is refused in a sentence naming amount | Scatterize's amount runs from -10000 to 10000, and this is -10001. | yes |
| fx_scatz_020.json is refused in a sentence naming right twist | Scatterize's right twist runs from -36000 to 36000, and this is 36001. | yes |
| fx_scatz_021.json is refused in a sentence naming left twist | Scatterize's left twist runs from -36000 to 36000, and this is -36001. | yes |
| fx_scatz_022.json is refused in a sentence naming transfer mode | Scatterize's transfer mode is composite, screen, add, alpha_add, and this is "multiply". | yes |
| fx_scatz_023.json is refused in a sentence naming amount | Scatterize's amount runs from -10000 to 10000, and this is 20000. | yes |
| a file with a Scatterize whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scatterize whose transfer mode is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scatterize without its left twist is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scatz_002.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatz_007.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatz_011.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatz_012.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scatz_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-676)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scatz_001.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_002.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_003.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_004.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_005.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_006.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_007.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_008.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_009.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_010.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_011.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_012.json, frames [0, 12, 6] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_013.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_014.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_015.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_016.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_017.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_018.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_019.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_020.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_021.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_022.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_scatz_023.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Scatterize as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072695 pixels changed | yes |
| the reference shot, Scatterize as added on three layers, frame 0, Full | largest difference 1 of 255, 1376 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize as added on three layers, frame 100, Full | largest difference 1 of 255, 1369 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize as added on three layers, frame 239, Full | largest difference 1 of 255, 1329 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize as added on three layers, frame 0, Draft | largest difference 1 of 255, 138 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize as added on three layers, frame 100, Draft | largest difference 1 of 255, 144 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize as added on three layers, frame 239, Draft | largest difference 1 of 255, 139 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened on three layers, frame 0, Full | largest difference 1 of 255, 1259 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened on three layers, frame 100, Full | largest difference 1 of 255, 1305 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened on three layers, frame 239, Full | largest difference 1 of 255, 1243 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened on three layers, frame 0, Draft | largest difference 1 of 255, 149 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened on three layers, frame 100, Draft | largest difference 1 of 255, 149 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount 300, the right edge turned edge-on, screened on three layers, frame 239, Draft | largest difference 1 of 255, 151 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073449 pixels changed | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add on three layers, frame 0, Full | largest difference 1 of 255, 1134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add on three layers, frame 100, Full | largest difference 1 of 255, 1155 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add on three layers, frame 239, Full | largest difference 1 of 255, 1173 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add on three layers, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add on three layers, frame 100, Draft | largest difference 1 of 255, 137 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scatterize Amount keyed 0 to 600, the ends twisted opposite ways, Alpha Add on three layers, frame 239, Draft | largest difference 1 of 255, 163 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-676 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed | 129395 pixels changed; warnings [] | yes |
| 2_cloud_amount_300.png: drawn without a warning, the street changed | 129563 pixels changed; warnings [] | yes |
| 3_ribbon_twisted_180.png: drawn without a warning, the street changed | 128858 pixels changed; warnings [] | yes |
| 4_right_edge_on_with_add.png: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 5_blown_away_amount_3000.png: drawn without a warning, the street changed | 129599 pixels changed; warnings [] | yes |
| 6_screened_cloud.png: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |

## Result

142 of 142 checks pass.
