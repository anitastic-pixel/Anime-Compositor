# D-564: Shadow/Highlight's Auto Amounts, Temporal Smoothing and Scene Detect

B-444, after After Effects' Shadow/Highlight: with Auto Amounts on, the shadow and highlight amounts come from the picture's own brightness histogram (D-351's statistics; the rule is this program's own and unverified) instead of the sliders; Temporal Smoothing and Scene Detect read the frames either side as Stretch Levels does, and do nothing with Auto Amounts off. Every expected pixel is `Fixtures/shadow_highlight_auto/expected_shadow_highlight_auto.json`, written by `tools/shadow_highlight_auto_reference.py` before this code existed and printed in document 25 as FX-SHAUTO-001 to 014. Tolerance 2e-5. FX-SHAUTO-010 warns TEMPORAL_SMOOTHING_SKIPPED on every frame.

## FX-SHAUTO-001 to 014 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHAUTO-001 frame 0: A file written before D-564, without the three settings, at Shadow/Highlight's starting settings: the sliders' lift, as D-400 drew it. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-001 frame 4: A file written before D-564, without the three settings, at Shadow/Highlight's starting settings: the sliders' lift, as D-400 drew it. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-002 frame 0: The three written at their starting values, off, 0 and off: the same as FX-SHAUTO-001. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-002 frame 4: The three written at their starting values, off, 0 and off: the same as FX-SHAUTO-001. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-003 frame 0: Auto Amounts on: frame 0, the light warm scene, lifted a little by its own lightness; frame 4, the darker cold scene, lifted more. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-003 frame 4: Auto Amounts on: frame 0, the light warm scene, lifted a little by its own lightness; frame 4, the darker cold scene, lifted more. | largest difference 9.3e-8 | yes |
| FX-SHAUTO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-004 frame 0: Auto Amounts on with the sliders at 90 and 90: the sliders are not used, so the same as FX-SHAUTO-003. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-004 frame 4: Auto Amounts on with the sliders at 90 and 90: the sliders are not used, so the same as FX-SHAUTO-003. | largest difference 9.3e-8 | yes |
| FX-SHAUTO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-005 frame 0: Auto Amounts on, midtone contrast 40 and both clips 5: the auto lift, then D-563's steps on the frame's own lightness. | largest difference 2.3e-7 | yes |
| FX-SHAUTO-005 frame 4: Auto Amounts on, midtone contrast 40 and both clips 5: the auto lift, then D-563's steps on the frame's own lightness. | largest difference 1.2e-7 | yes |
| FX-SHAUTO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-006 frame 0: Temporal Smoothing 0.1 and Scene Detect on with Auto Amounts off: they do nothing without it, so the same as FX-SHAUTO-001. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-006 frame 4: Temporal Smoothing 0.1 and Scene Detect on with Auto Amounts off: they do nothing without it, so the same as FX-SHAUTO-001. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-007 frame 0: Auto Amounts on, Temporal Smoothing 0.1 seconds, two frames each side: frame 0 judged by frames 0 to 2 (the two before are outside the layer); frame 3 by 1 to 5, across the cut, so lifted more than by itself; frame 7 by 5 to 7. | largest difference 1.3e-7 | yes |
| FX-SHAUTO-007 frame 3: Auto Amounts on, Temporal Smoothing 0.1 seconds, two frames each side: frame 0 judged by frames 0 to 2 (the two before are outside the layer); frame 3 by 1 to 5, across the cut, so lifted more than by itself; frame 7 by 5 to 7. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-007 frame 7: Auto Amounts on, Temporal Smoothing 0.1 seconds, two frames each side: frame 0 judged by frames 0 to 2 (the two before are outside the layer); frame 3 by 1 to 5, across the cut, so lifted more than by itself; frame 7 by 5 to 7. | largest difference 1.3e-7 | yes |
| FX-SHAUTO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-008 frame 3: FX-SHAUTO-007 with Scene Detect: frame 3 judged by 1 to 3 only and frame 4 by 4 to 6 only, each side of the cut. | largest difference 1.3e-7 | yes |
| FX-SHAUTO-008 frame 4: FX-SHAUTO-007 with Scene Detect: frame 3 judged by 1 to 3 only and frame 4 by 4 to 6 only, each side of the cut. | largest difference 7.9e-8 | yes |
| FX-SHAUTO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-009 frame 0: Auto Amounts on, on an adjustment layer above the holder: the frame beneath is the drawing, so the same as FX-SHAUTO-003. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-009 frame 4: Auto Amounts on, on an adjustment layer above the holder: the frame beneath is the drawing, so the same as FX-SHAUTO-003. | largest difference 9.3e-8 | yes |
| FX-SHAUTO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHAUTO-010 frame 0: Auto Amounts on with Temporal Smoothing 0.1 on an adjustment layer: the frames beneath at other times are not to hand, so each frame is judged by itself, as FX-SHAUTO-003, with a warning each frame. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-010 frame 3: Auto Amounts on with Temporal Smoothing 0.1 on an adjustment layer: the frames beneath at other times are not to hand, so each frame is judged by itself, as FX-SHAUTO-003, with a warning each frame. | largest difference 1.6e-7 | yes |
| FX-SHAUTO-010: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-SHAUTO-011 frame 0: Auto Amounts written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-011 frame 4: Auto Amounts written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SHAUTO-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAUTO-012 frame 0: Scene Detect written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-012 frame 4: Scene Detect written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SHAUTO-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAUTO-013 frame 0: Temporal Smoothing 11 seconds, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-013 frame 4: Temporal Smoothing 11 seconds, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SHAUTO-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHAUTO-014 frame 0: Temporal Smoothing -1 seconds, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-SHAUTO-014 frame 4: Temporal Smoothing -1 seconds, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-SHAUTO-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shauto_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shauto_001.json, written without the three settings, is saved without them (D-304's way: only if moved, keyed or already in the file) | {"black_clip":0,"color_correction":20,"highlight_amount":0,"highlight_radius":30,"highlight_tonal_width":50,"midtone_contrast":0,"shadow_amount":50,"shadow_radius":30,"shadow_tonal_width":50,"white_clip":0} | yes |
| fx_shauto_008.json is saved with its thirteen settings, and never the statistics a smoothing added up | {"auto_amounts":"on","black_clip":0,"color_correction":20,"highlight_amount":0,"highlight_radius":30,"highlight_tonal_width":50,"midtone_contrast":0,"scene_detect":"on","shadow_amount":50,"shadow_radius":30,"shadow_tonal_width":50,"temporal_smoothing":0.1,"white_clip":0} | yes |
| fx_shauto_011.json is refused in a sentence naming it | Shadow/Highlight's auto amounts and scene detect are "off" or "on", and this is "yes". | yes |
| fx_shauto_012.json is refused in a sentence naming it | Shadow/Highlight's auto amounts and scene detect are "off" or "on", and this is "maybe". | yes |
| fx_shauto_013.json is refused in a sentence naming it | Shadow/Highlight's temporal smoothing runs from 0 to 10, and this is 11. | yes |
| fx_shauto_014.json is refused in a sentence naming it | Shadow/Highlight's temporal smoothing runs from 0 to 10, and this is -1. | yes |
| a file with a Shadow/Highlight whose auto amounts is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shadow/Highlight whose temporal smoothing is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| auto amounts "yes" is refused with a sentence, and nothing changes | Shadow/Highlight's auto amounts and scene detect are "off" or "on", and this is "yes". | yes |
| scene detect "maybe" is refused with a sentence, and nothing changes | Shadow/Highlight's auto amounts and scene detect are "off" or "on", and this is "maybe". | yes |
| temporal smoothing 11 is refused with a sentence, and nothing changes | Shadow/Highlight's temporal smoothing runs from 0 to 10, and this is 11. | yes |
| temporal smoothing keyed to -1 is refused with a sentence, and nothing changes | Shadow/Highlight's temporal smoothing runs from 0 to 10, and this is -1. | yes |
| auto amounts on, temporal smoothing 0.1, scene detect on is taken | taken | yes |
| temporal smoothing keyed from 0 to 0.2 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shauto_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shauto_003.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shauto_005.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shauto_007.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shauto_008.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shauto_010.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shauto_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: true; the same warnings: true | yes |
| fx_shauto_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: true; the same warnings: true | yes |
| fx_shauto_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shauto_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 393693 pixels changed | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card on three layers, frame 0, Full | largest difference 1 of 255, 1623 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card on three layers, frame 100, Full | largest difference 1 of 255, 1548 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card on three layers, frame 239, Full | largest difference 1 of 255, 1551 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card on three layers, frame 0, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1473950 pixels changed | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor on three layers, frame 0, Full | largest difference 1 of 255, 47145 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor on three layers, frame 100, Full | largest difference 1 of 255, 47779 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor on three layers, frame 239, Full | largest difference 1 of 255, 46176 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor on three layers, frame 0, Draft | largest difference 1 of 255, 4811 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor on three layers, frame 100, Draft | largest difference 1 of 255, 4290 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on, on the processor on three layers, frame 239, Draft | largest difference 1 of 255, 4678 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1473950 pixels changed | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor on three layers, frame 0, Full | largest difference 1 of 255, 47145 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor on three layers, frame 100, Full | largest difference 1 of 255, 47779 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor on three layers, frame 239, Full | largest difference 1 of 255, 46176 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor on three layers, frame 0, Draft | largest difference 1 of 255, 4779 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor on three layers, frame 100, Draft | largest difference 1 of 255, 4298 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1, on the processor on three layers, frame 239, Draft | largest difference 1 of 255, 4632 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-564 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, 2_as_added.png and 3_auto.png: the street, Shadow/Highlight as added (the sliders' shadow 50), and with Auto Amounts on (the slider at 90 not used): the auto lift is its own; all draw cleanly | [] [] []; mean brightness 144.61, 146.01, 142.88 | yes |
| 4_dark_before.png and 5_dark_auto.png: the street at half its brightness, and with Auto Amounts on: the darker picture is lifted more than the street was (whose auto highlight amount pulls its lightest parts down a little); both draw cleanly | [] []; mean brightness raised 7.76 on the dark street against -1.73 on the street | yes |

## Result

115 of 115 checks pass.
