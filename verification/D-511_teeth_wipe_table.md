# D-511: Teeth Wipe

B-391, after CycoreFX's CC Jaws (Transition): the layer split in two along a toothed cut, the two parts moving apart. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/teeth_wipe_reference.py`: a cut through the Center at the Direction, its teeth Width apart and Height tall (spikes, trapezoids or blocks); the part before the cut moves back across it and the part after moves on, each read between pixels, until at completion 100 both have left the drawing. Every expected pixel is `Fixtures/teeth_wipe/expected_teeth_wipe.json`, written by the tool before this code existed and printed in document 25 as FX-TEETH-001 to 027. Tolerance 2e-5.

## FX-TEETH-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TEETH-001 frame 0: The settings as they start: completion 0, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-TEETH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-002 frame 0: Completion 100: both parts gone from the drawing, every pixel clear. | largest difference 0.0e0 | yes |
| FX-TEETH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-003 frame 0: Completion 20, spikes 3 high and 4 wide: the drawing parted across its middle along a zigzag, the top half moved up, the bottom half down. | largest difference 1.2e-7 | yes |
| FX-TEETH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-004 frame 0: Trapezoids 3 high and 7 wide, completion 20: flat-topped teeth. | largest difference 1.2e-7 | yes |
| FX-TEETH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-005 frame 0: FX-TEETH-003 with blocks: square teeth. | largest difference 1.2e-7 | yes |
| FX-TEETH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-006 frame 0: FX-TEETH-003 with direction 90: the cut runs top to bottom, the halves move left and right. | largest difference 2.0e-7 | yes |
| FX-TEETH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-007 frame 0: FX-TEETH-003 with direction 30: the cut turned about the centre. | largest difference 1.4e-7 | yes |
| FX-TEETH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-008 frame 0: Center [25, 30], completion 20: the cut moved up and to the left, the teeth counted from the new centre. | largest difference 1.2e-7 | yes |
| FX-TEETH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-009 frame 0: Height 0, completion 20: a straight cut, no teeth. | largest difference 1.3e-7 | yes |
| FX-TEETH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-010 frame 0: Completion 7.3: a move of part of a pixel, each part read between pixels. | largest difference 1.6e-7 | yes |
| FX-TEETH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-011 frame 0: Spikes 5 high and 2.5 wide, completion 15: tall narrow teeth. | largest difference 1.2e-7 | yes |
| FX-TEETH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-012 frame 0: Completion keyed from 0 at frame 0 to 97 at frame 4, linear: frame 2 is completion 48.5; at frame 4 every pixel is in the gap. | largest difference 1.9e-7 | yes |
| FX-TEETH-012 frame 2: Completion keyed from 0 at frame 0 to 97 at frame 4, linear: frame 2 is completion 48.5; at frame 4 every pixel is in the gap. | largest difference 1.5e-7 | yes |
| FX-TEETH-012 frame 4: Completion keyed from 0 at frame 0 to 97 at frame 4, linear: frame 2 is completion 48.5; at frame 4 every pixel is in the gap. | largest difference 0.0e0 | yes |
| FX-TEETH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-013 frame 0: Direction keyed from 0 at frame 0 to 60 at frame 4, linear, completion 20: frame 2 is direction 30. | largest difference 1.2e-7 | yes |
| FX-TEETH-013 frame 2: Direction keyed from 0 at frame 0 to 60 at frame 4, linear, completion 20: frame 2 is direction 30. | largest difference 1.4e-7 | yes |
| FX-TEETH-013 frame 4: Direction keyed from 0 at frame 0 to 60 at frame 4, linear, completion 20: frame 2 is direction 30. | largest difference 1.5e-7 | yes |
| FX-TEETH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-014 frame 0: Height keyed from 0 at frame 0 to 6 at frame 4, linear, completion 20, blocks. | largest difference 1.3e-7 | yes |
| FX-TEETH-014 frame 4: Height keyed from 0 at frame 0 to 6 at frame 4, linear, completion 20, blocks. | largest difference 1.4e-7 | yes |
| FX-TEETH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-015 frame 0: FX-TEETH-003 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-TEETH-015 frame 3: FX-TEETH-003 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-TEETH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-016 frame 0: Direction -135, trapezoids 2 high and 3 wide, completion 35, Center [60, 40]. | largest difference 1.4e-7 | yes |
| FX-TEETH-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TEETH-017 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-017 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-018 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-018 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-019 frame 0: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-019 frame 4: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-020 frame 0: Direction -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-020 frame 4: Direction -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-021 frame 0: Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-021 frame 4: Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-022 frame 0: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-022 frame 4: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-023 frame 0: Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-023 frame 4: Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-024 frame 0: Width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-024 frame 4: Width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-025 frame 0: Shape "Spikes": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-025 frame 4: Shape "Spikes": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-026 frame 0: Shape "robo_jaw": not a shape of Teeth Wipe. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-026 frame 4: Shape "robo_jaw": not a shape of Teeth Wipe. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TEETH-027 frame 0: Width keyed to 2000 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-027 frame 4: Width keyed to 2000 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TEETH-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_teeth_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_teeth_016.json is saved with its six settings, trapezoids, direction -135, 2 high and 3 wide, Center [60, 40] | {"center":[60,40],"completion":35,"direction":-135,"height":2,"shape":"trapezoids","width":3} | yes |
| fx_teeth_017.json is refused in a sentence naming completion | Teeth Wipe's completion runs from 0 to 100, and this is 101. | yes |
| fx_teeth_018.json is refused in a sentence naming completion | Teeth Wipe's completion runs from 0 to 100, and this is -1. | yes |
| fx_teeth_019.json is refused in a sentence naming center | Teeth Wipe's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_teeth_020.json is refused in a sentence naming direction | Teeth Wipe's direction runs from -3600 to 3600, and this is -3601. | yes |
| fx_teeth_021.json is refused in a sentence naming height | Teeth Wipe's height runs from 0 to 1000, and this is -1. | yes |
| fx_teeth_022.json is refused in a sentence naming height | Teeth Wipe's height runs from 0 to 1000, and this is 1001. | yes |
| fx_teeth_023.json is refused in a sentence naming width | Teeth Wipe's width runs from 1 to 1000, and this is 0.5. | yes |
| fx_teeth_024.json is refused in a sentence naming width | Teeth Wipe's width runs from 1 to 1000, and this is 1001. | yes |
| fx_teeth_025.json is refused in a sentence naming shape | Teeth Wipe's shape is "spikes", "trapezoids" or "blocks", and this is "Spikes". | yes |
| fx_teeth_026.json is refused in a sentence naming shape | Teeth Wipe's shape is "spikes", "trapezoids" or "blocks", and this is "robo_jaw". | yes |
| a file with a Teeth Wipe whose height is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Teeth Wipe whose Center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-511)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_teeth_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_teeth_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073501 pixels changed | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 627 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 181 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 50 (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70]: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70] on three layers, frame 0, Full | largest difference 1 of 255, 237 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70] on three layers, frame 100, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70] on three layers, frame 239, Full | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70] on three layers, frame 0, Draft | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70] on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 40, blocks, direction 30, Center [20, 70] on three layers, frame 239, Draft | largest difference 1 of 255, 16 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2053301 pixels changed | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide on three layers, frame 0, Full | largest difference 1 of 255, 2833 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide on three layers, frame 100, Full | largest difference 1 of 255, 477 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide on three layers, frame 239, Full | largest difference 1 of 255, 1076 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide on three layers, frame 100, Draft | largest difference 1 of 255, 15 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion 30, trapezoids 80 high and 25 wide on three layers, frame 239, Draft | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073339 pixels changed | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90 on three layers, frame 0, Full | largest difference 1 of 255, 3180 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90 on three layers, frame 100, Full | largest difference 1 of 255, 104 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90 on three layers, frame 0, Draft | largest difference 1 of 255, 108 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90 on three layers, frame 100, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Teeth Wipe completion keyed 10 to 90 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-511 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_spikes_parted.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 2) as the street's at (2, 64) | 127408 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [143, 181, 222, 255]; warnings [] | yes |
| 2_trapezoids_parted.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 267) as the street's at (2, 205) | 127316 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 267) [180, 90, 70, 255]; warnings [] | yes |
| 3_blocks_upright.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 135) as the street's at (106, 135) | 77925 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 135) [170, 195, 215, 255]; warnings [] | yes |
| 4_tall_narrow_spikes.png: drawn without a warning, the pixel at (240, 135) gone, the one at (2, 2) as the street's at (2, 72) | 128468 pixels changed; at (240, 135) [0, 0, 0, 0]; at (2, 2) [146, 183, 222, 255]; warnings [] | yes |
| 5_done.png: drawn without a warning, the pixel at (2, 2) gone, every pixel gone | 129600 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [0, 0, 0, 0]; warnings [] | yes |

## Result

171 of 171 checks pass.
