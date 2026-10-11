# D-509: Glass Melt

B-389: `core.glass_melt`, after CycoreFX's CC Glass Wipe (Transition), under our own name: the layer melts away into the Layer to Reveal (D-189's layer setting, nothing when none is chosen), the darkest parts of the Gradient Layer first (the layer itself when none is chosen), the melting edge bending what it shows. CycoreFX names the controls but publishes no formula and no defaults, so the rule and the settings when added are ours, stated in `tools/glass_melt_reference.py`: the bump is the gradient's brightness times its covering, blurred at sigma Softness / 2 with the outer pixels repeated; its slope times 1.25 max(Softness, 1) times Displacement Amount is the bend; a band a tenth of the bump wide sweeps from below 0 to above 1 as Completion goes from 0 to 100, and inside it the layer, bent forward, fades into the reveal, bent back. Every expected pixel is `Fixtures/glass_melt/expected_glass_melt.json`, written by the tool before this code existed and printed in document 25 as FX-GMELT-001 to 033. Tolerance 2e-5.

## FX-GMELT-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GMELT-001 frame 0: As added: completion 0, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-GMELT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-002 frame 0: Completion 100 with nothing to reveal: every pixel clear. | largest difference 0.0e0 | yes |
| FX-GMELT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-003 frame 0: Completion 100 revealing the paint: the paint exactly. | largest difference 1.5e-7 | yes |
| FX-GMELT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-004 frame 0: The grey ramp as the gradient, completion 50, softness 0, no bend: the dark half gone to the paint, the light half kept, the middle column part of each. | largest difference 8.3e-7 | yes |
| FX-GMELT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-005 frame 0: FX-GMELT-004 with displacement 20: the ramp's even slope bends the melting column sideways. | largest difference 8.3e-7 | yes |
| FX-GMELT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-006 frame 0: The ramp, completion 50, softness 4, displacement 10: the blur rounds the ramp's ends, so the edge columns bend less. | largest difference 8.3e-7 | yes |
| FX-GMELT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-007 frame 0: The holder as its own gradient (as added), completion 40, softness 2, displacement 8: its dark red and blue squares melt first, bending round their edges. | largest difference 5.4e-7 | yes |
| FX-GMELT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-008 frame 0: The dome as the gradient, completion 50, softness 2, displacement 6: the dark rim goes first, the bright middle stays, bent outward. | largest difference 9.2e-7 | yes |
| FX-GMELT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-009 frame 0: FX-GMELT-008 with displacement -6: bent the other way. | largest difference 8.5e-7 | yes |
| FX-GMELT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-010 frame 0: The dome, completion 50, nothing revealed: the gone part clear. | largest difference 1.5e-6 | yes |
| FX-GMELT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-011 frame 0: A white gradient, softness 100, displacement 500: flat, so nothing bends, and at completion 50 nothing is dark enough to go. | largest difference 1.5e-7 | yes |
| FX-GMELT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-012 frame 0: The checker stretched to 16 by 10, completion 50, softness 3, displacement 5: soft squares melt. | largest difference 2.1e-7 | yes |
| FX-GMELT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-013 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, the dome: frame 0 the holder, frame 2 FX-GMELT-008, frame 4 the paint. | largest difference 1.5e-7 | yes |
| FX-GMELT-013 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, the dome: frame 0 the holder, frame 2 FX-GMELT-008, frame 4 the paint. | largest difference 9.2e-7 | yes |
| FX-GMELT-013 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, the dome: frame 0 the holder, frame 2 FX-GMELT-008, frame 4 the paint. | largest difference 1.5e-7 | yes |
| FX-GMELT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-014 frame 0: Displacement keyed from 0 at frame 0 to 12 at frame 4, the dome at 50: frame 2 is FX-GMELT-008. | largest difference 7.5e-7 | yes |
| FX-GMELT-014 frame 2: Displacement keyed from 0 at frame 0 to 12 at frame 4, the dome at 50: frame 2 is FX-GMELT-008. | largest difference 9.2e-7 | yes |
| FX-GMELT-014 frame 4: Displacement keyed from 0 at frame 0 to 12 at frame 4, the dome at 50: frame 2 is FX-GMELT-008. | largest difference 1.7e-6 | yes |
| FX-GMELT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-015 frame 0: Softness keyed from 0 at frame 0 to 4 at frame 4, the ramp: frame 0 FX-GMELT-005's softness, frame 4 a softer ramp. | largest difference 8.3e-7 | yes |
| FX-GMELT-015 frame 4: Softness keyed from 0 at frame 0 to 4 at frame 4, the ramp: frame 0 FX-GMELT-005's softness, frame 4 a softer ramp. | largest difference 8.3e-7 | yes |
| FX-GMELT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-016 frame 0: A reveal layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GMELT-016 frame 4: A reveal layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GMELT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-GMELT-017 frame 0: A gradient layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GMELT-017 frame 4: A gradient layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GMELT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-GMELT-018 frame 0: The paint with its in point at frame 4 as the reveal: before it nothing is revealed, so the gone part is clear; at frame 4 FX-GMELT-008. | largest difference 1.5e-6 | yes |
| FX-GMELT-018 frame 4: The paint with its in point at frame 4 as the reveal: before it nothing is revealed, so the gone part is clear; at frame 4 FX-GMELT-008. | largest difference 9.2e-7 | yes |
| FX-GMELT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-019 frame 0: The paint's middle 8 by 6 as the reveal, centred: it lies where it was cut, clear round it. | largest difference 1.5e-6 | yes |
| FX-GMELT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-020 frame 0: The holder revealing itself, the dome, completion 50: the layer bends into itself. | largest difference 1.1e-6 | yes |
| FX-GMELT-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-021 frame 0: FX-GMELT-008 on the holder moved 2 right and 1 down: the same picture moved, since the maps lie on the layer. | largest difference 9.2e-7 | yes |
| FX-GMELT-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-022 frame 0: The effect on an adjustment layer above the holder: the maps lie on the frame, which here is the holder's own rectangle, so this is FX-GMELT-008. | largest difference 9.2e-7 | yes |
| FX-GMELT-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-023 frame 0: The ramp, completion 25, softness 0, displacement 0: only the darkest columns gone. | largest difference 2.5e-7 | yes |
| FX-GMELT-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-024 frame 0: The ramp, completion 75, softness 0, displacement 0: only the lightest columns kept. | largest difference 7.2e-7 | yes |
| FX-GMELT-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GMELT-025 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-025 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-026 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-026 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-027 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-027 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-028 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-028 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-029 frame 0: Displacement 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-029 frame 4: Displacement 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-030 frame 0: Displacement -501, below -500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-030 frame 4: Displacement -501, below -500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-031 frame 0: A reveal layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-031 frame 4: A reveal layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-032 frame 0: A gradient layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-032 frame 4: A gradient layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GMELT-033 frame 0: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-033 frame 4: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GMELT-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_reveal.json: `a` reveals `b` and `b` reveals `a`: refused the same. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Glass Melt whose gradient layer is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gmelt_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gmelt_008.json is saved with its five settings, the layers as written | {"completion":50,"displacement_amount":6,"gradient_layer":"dome","layer_to_reveal":"paint","softness":2} | yes |
| fx_gmelt_025.json is refused in a sentence naming completion | Glass Melt's completion runs from 0 to 100, and this is 101. | yes |
| fx_gmelt_026.json is refused in a sentence naming completion | Glass Melt's completion runs from 0 to 100, and this is -1. | yes |
| fx_gmelt_027.json is refused in a sentence naming softness | Glass Melt's softness runs from 0 to 100, and this is 101. | yes |
| fx_gmelt_028.json is refused in a sentence naming softness | Glass Melt's softness runs from 0 to 100, and this is -1. | yes |
| fx_gmelt_029.json is refused in a sentence naming displacement | Glass Melt's displacement amount runs from -500 to 500, and this is 501. | yes |
| fx_gmelt_030.json is refused in a sentence naming displacement | Glass Melt's displacement amount runs from -500 to 500, and this is -501. | yes |
| fx_gmelt_031.json is refused in a sentence naming layer to reveal | Glass Melt's layer to reveal is the name of a layer of this composition, and this is 3. | yes |
| fx_gmelt_032.json is refused in a sentence naming gradient layer | Glass Melt's gradient layer is the name of a layer of this composition, and this is 3. | yes |
| a file with a Glass Melt whose completion is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Glass Melt without its completion is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Glass Melt without its gradient layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion 101 is refused with a sentence, and nothing changes | Glass Melt's completion runs from 0 to 100, and this is 101. | yes |
| softness 120 is refused with a sentence, and nothing changes | Glass Melt's softness runs from 0 to 100, and this is 120. | yes |
| displacement keyed to 600 is refused with a sentence, and nothing changes | Glass Melt's displacement amount runs from -500 to 500, and this is 600. | yes |
| completion 70 revealing nothing is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview halves the softness and the displacement and keeps the completion | GlassMelt { completion: 50.0, layer_to_reveal: String("paint"), gradient_layer: String("dome"), softness: 4.0, displacement_amount: -20.0, reveal_map: None, gradient_map: None } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gmelt_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gmelt_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gmelt_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gmelt_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 545173 pixels changed | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4 on three layers, frame 0, Full | largest difference 1 of 255, 1710 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4 on three layers, frame 100, Full | largest difference 1 of 255, 2604 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4 on three layers, frame 239, Full | largest difference 1 of 255, 1941 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4 on three layers, frame 0, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4 on three layers, frame 100, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt the layer itself as the gradient, completion 40, revealing layer 4 on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt layer 4 as the gradient, completion 60, softness 30, displacement -120, revealing nothing on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 215655 pixels changed | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4 on three layers, frame 0, Full | largest difference 1 of 255, 1689 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4 on three layers, frame 100, Full | largest difference 1 of 255, 2420 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4 on three layers, frame 239, Full | largest difference 1 of 255, 1847 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4 on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4 on three layers, frame 100, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 25, softness 0, displacement 0, revealing layer 4 on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glass Melt completion 100, revealing layer 4 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-509 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_half_melted.png, completion 50 revealing the street in its opposite colours: the dark half of the street gone to the opposite colours, the light half kept as it was, bent glassy edges between; draws cleanly | [], 87997 pixels kept, 31035 revealed, of 129600 | yes |
| 4_strong_bend.png, the same with softness 30 and displacement 200: wider, smoother melting edges that bend the street like thick glass; draws cleanly | [], 15982 pixels differ from 3 | yes |
| 5_done.png, completion 100: the street gone entirely, the opposite colours exactly; draws cleanly | [], 0 pixels differ from the reveal | yes |
| a layer to reveal not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

214 of 214 checks pass.
