# D-513: Glow Wipe

B-393, after CycoreFX's CC Light Wipe (Transition): the layer wiped away from the Center behind a band of light. The CycoreFX manual names the controls but publishes no formula and no settings when added (beyond Color from Source off), so the rule and those settings are ours, stated in `tools/glow_wipe_reference.py`: an edge moving out from the Center by the shape's distance (two walls, a circle or a square, the walls and the square turned by the Direction), the layer gone inside it (outside it, reversed), and a band of light a tenth of the range wide around it, the Color or the layer's own colour, added on top. Every expected pixel is `Fixtures/glow_wipe/expected_glow_wipe.json`, written by the tool before this code existed and printed in document 25 as FX-GLOW-001 to 028. Tolerance 2e-5.

## FX-GLOW-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLOW-001 frame 0: The settings as they start: completion 0, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-GLOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-002 frame 0: Completion 100: every pixel gone, and no light left. | largest difference 0.0e0 | yes |
| FX-GLOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-003 frame 0: Radial, completion 40: a hole from the centre, a white glow around its edge on the layer and inside the hole. | largest difference 1.9e-7 | yes |
| FX-GLOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-004 frame 0: Doors, completion 40: two walls of light moving left and right from the centre. | largest difference 1.4e-7 | yes |
| FX-GLOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-005 frame 0: Rectangular, completion 40: a growing square of light. | largest difference 1.9e-7 | yes |
| FX-GLOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-006 frame 0: Rectangular, direction 30, completion 40: the square turned. | largest difference 1.9e-7 | yes |
| FX-GLOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-007 frame 0: Doors, direction 90, completion 40: the walls move up and down. | largest difference 1.9e-7 | yes |
| FX-GLOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-008 frame 0: Radial, completion 40, Color from Source on: the glow is the layer's own colour, and none over the transparent columns. | largest difference 2.2e-7 | yes |
| FX-GLOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-009 frame 0: Radial, completion 40, colour #ff8000, intensity 250: a strong orange glow. | largest difference 2.3e-7 | yes |
| FX-GLOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-010 frame 0: Radial, completion 40, intensity 0: the hole alone, no light. | largest difference 1.9e-7 | yes |
| FX-GLOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-011 frame 0: Radial, completion 40, reversed: the wipe from the outside in, the middle kept. | largest difference 1.6e-7 | yes |
| FX-GLOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-012 frame 0: Radial, Center [25, 30], completion 30: the hole moved up and to the left. | largest difference 1.9e-7 | yes |
| FX-GLOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-013 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is completion 50, frame 4 all clear. | largest difference 1.9e-7 | yes |
| FX-GLOW-013 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is completion 50, frame 4 all clear. | largest difference 1.6e-7 | yes |
| FX-GLOW-013 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is completion 50, frame 4 all clear. | largest difference 0.0e0 | yes |
| FX-GLOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-014 frame 0: Intensity keyed from 0 at frame 0 to 400 at frame 4, linear, completion 40: frame 2 is intensity 200. | largest difference 1.9e-7 | yes |
| FX-GLOW-014 frame 2: Intensity keyed from 0 at frame 0 to 400 at frame 4, linear, completion 40: frame 2 is intensity 200. | largest difference 1.9e-7 | yes |
| FX-GLOW-014 frame 4: Intensity keyed from 0 at frame 0 to 400 at frame 4, linear, completion 40: frame 2 is intensity 200. | largest difference 1.9e-7 | yes |
| FX-GLOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-015 frame 0: FX-GLOW-003 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-GLOW-015 frame 3: FX-GLOW-003 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-GLOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-016 frame 0: Doors, direction -135, reversed, Color from Source on, Center [60, 40], completion 55. | largest difference 2.1e-7 | yes |
| FX-GLOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-017 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-017 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-018 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-018 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-019 frame 0: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-019 frame 4: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-020 frame 0: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-020 frame 4: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-021 frame 0: Intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-021 frame 4: Intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-022 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-022 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-023 frame 0: Shape "Radial": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-023 frame 4: Shape "Radial": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-024 frame 0: Shape "round": not a shape of Glow Wipe. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-024 frame 4: Shape "round": not a shape of Glow Wipe. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-025 frame 0: Color from Source "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-025 frame 4: Color from Source "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-026 frame 0: Color "#ff80": not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-026 frame 4: Color "#ff80": not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-027 frame 0: Reverse "true": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-027 frame 4: Reverse "true": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-028 frame 0: Intensity keyed to 2000 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-028 frame 4: Intensity keyed to 2000 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GLOW-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_016.json is saved with its eight settings, doors, Color from Source on, reversed, direction -135, Center [60, 40], completion 55 | {"center":[60,40],"color":"#ffffff","color_from_source":"on","completion":55,"direction":-135,"intensity":100,"reverse":"on","shape":"doors"} | yes |
| fx_glow_017.json is refused in a sentence naming completion | Glow Wipe's completion runs from 0 to 100, and this is 101. | yes |
| fx_glow_018.json is refused in a sentence naming completion | Glow Wipe's completion runs from 0 to 100, and this is -1. | yes |
| fx_glow_019.json is refused in a sentence naming center | Glow Wipe's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_glow_020.json is refused in a sentence naming intensity | Glow Wipe's intensity runs from 0 to 1000, and this is -1. | yes |
| fx_glow_021.json is refused in a sentence naming intensity | Glow Wipe's intensity runs from 0 to 1000, and this is 1001. | yes |
| fx_glow_022.json is refused in a sentence naming direction | Glow Wipe's direction runs from -3600 to 3600, and this is 3601. | yes |
| fx_glow_023.json is refused in a sentence naming shape | Glow Wipe's shape is "doors", "radial" or "rectangular", and this is "Radial". | yes |
| fx_glow_024.json is refused in a sentence naming shape | Glow Wipe's shape is "doors", "radial" or "rectangular", and this is "round". | yes |
| fx_glow_025.json is refused in a sentence naming color from source | Glow Wipe's color from source is "off" or "on", and this is "yes". | yes |
| fx_glow_026.json is refused in a sentence naming colour | Glow Wipe's colour is written #rrggbb, and this is "#ff80". | yes |
| fx_glow_027.json is refused in a sentence naming reverse | Glow Wipe's reverse is "off" or "on", and this is "true". | yes |
| a file with a Glow Wipe whose intensity is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Glow Wipe whose Center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-513)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glow_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1246689 pixels changed | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 50 (as added otherwise) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1318405 pixels changed | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250 on three layers, frame 0, Full | largest difference 1 of 255, 16 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250 on three layers, frame 100, Full | largest difference 1 of 255, 110 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250 on three layers, frame 239, Full | largest difference 1 of 255, 125 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250 on three layers, frame 0, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250 on three layers, frame 100, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 40, doors, direction 30, #ff8000 at 250 on three layers, frame 239, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70]: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 595978 pixels changed | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70] on three layers, frame 0, Full | largest difference 1 of 255, 3782 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70] on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70] on three layers, frame 239, Full | largest difference 1 of 255, 1773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70] on three layers, frame 0, Draft | largest difference 1 of 255, 153 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70] on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion 30, rectangular, reversed, Color from Source, Center [20, 70] on three layers, frame 239, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1018796 pixels changed | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90 on three layers, frame 0, Full | largest difference 1 of 255, 2794 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90 on three layers, frame 100, Full | largest difference 1 of 255, 332 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90 on three layers, frame 0, Draft | largest difference 1 of 255, 104 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90 on three layers, frame 100, Draft | largest difference 1 of 255, 17 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glow Wipe completion keyed 10 to 90 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-513 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_circle.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 2) as the street's | 54825 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [120, 170, 229, 255]; warnings [] | yes |
| 2_doors.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 135) as the street's | 62100 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 135) [180, 90, 70, 255]; warnings [] | yes |
| 3_square_turned.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 2) as the street's | 65046 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [120, 170, 229, 255]; warnings [] | yes |
| 5_reversed_own_colour.png: drawn without a warning, the pixel at (2, 2) gone, the one at (240, 135) as the street's | 66047 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [90, 140, 170, 255]; warnings [] | yes |
| 6_done.png: drawn without a warning, the pixel at (2, 2) gone, every pixel gone | 129600 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [0, 0, 0, 0]; warnings [] | yes |
| 4_orange_light.png: drawn without a warning, the middle gone, the pixel at (344, 135) on the edge lit orange: covered, red full, green less, no blue | 54838 pixels changed; at (344, 135) [255, 210, 0, 255]; at (240, 135) [0, 0, 0, 0]; warnings [] | yes |

## Result

177 of 177 checks pass.
