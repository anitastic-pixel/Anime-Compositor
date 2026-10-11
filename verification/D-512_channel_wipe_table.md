# D-512: Channel Wipe

B-392: `core.channel_wipe`, after CycoreFX's CC Image Wipe (Transition), under our own name: the layer wiped away in the order of a chosen channel of a gradient layer, the lowest values first. By D-383 and D-394, as D-402 Tritone, it is a second effect over Gradient Wipe's engine (D-194), which takes the extra settings (Property, Blur, the layer itself as the gradient); Gradient Wipe is that engine with luminance, no blur and nothing wiped when no layer is named, and draws as before: its own fixtures and card comparisons are run again below, unchanged. CycoreFX names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/channel_wipe_reference.py`: the gradient (the layer itself when none is chosen) is read in the chosen property (red, green, blue, alpha, luminance, lightness, hue or saturation), blurred at sigma Blur / 2 with the outer pixels repeated, and Gradient Wipe's completion and softness rule keeps or clears each pixel. Every expected pixel is `Fixtures/channel_wipe/expected_channel_wipe.json`, written by the tool before this code existed and printed in document 25 as FX-CWIPE-001 to 033. Tolerance 2e-5. CWIPE-002 to 006 are Gradient Wipe's own rendered numbers (FX-GWIPE-016, 003, 004, 008, 012), which shows the one engine.

## FX-CWIPE-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CWIPE-001 frame 0: As added: the layer itself as the gradient, completion 0, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-CWIPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-002 frame 0: The layer itself, completion 50, softness 0, luminance: its red and blue squares, darker than half, go, its cream ones stay, and the clear corner stays clear; Gradient Wipe's FX-GWIPE-016 exactly. | largest difference 9.7e-8 | yes |
| FX-CWIPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-003 frame 0: The grey ramp, completion 50, softness 0, luminance: the eight dark columns gone; Gradient Wipe's FX-GWIPE-003 exactly. | largest difference 1.5e-7 | yes |
| FX-CWIPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-004 frame 0: The ramp, completion 50, softness 100: kept by the ramp's brightness; FX-GWIPE-004 exactly. | largest difference 1.9e-7 | yes |
| FX-CWIPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-005 frame 0: FX-CWIPE-003 with Invert on: the light columns go; FX-GWIPE-008 exactly. | largest difference 1.5e-7 | yes |
| FX-CWIPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-006 frame 0: A white map clear at the top and more covering down each row, completion 50, softness 0, luminance: a clear map reads 0, so rows 0 and 1 go; FX-GWIPE-012 exactly. | largest difference 1.5e-7 | yes |
| FX-CWIPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-007 frame 0: The same map, alpha: rows 0 to 4, less than half covered, go, rows 5 to 9 stay. | largest difference 1.5e-7 | yes |
| FX-CWIPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-008 frame 0: The painted map, red (40 + 8 y of 255 down each row), completion 30, softness 0: rows 0 to 4 gone, 5 to 9 kept. | largest difference 1.5e-7 | yes |
| FX-CWIPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-009 frame 0: The painted map, green (30 + 14 x across each row), completion 50, softness 0: columns 0 to 6 gone, 7 to 15 kept. | largest difference 1.5e-7 | yes |
| FX-CWIPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-010 frame 0: The painted map, blue, completion 50, softness 20: falling to the right, so the right side fades. | largest difference 4.4e-7 | yes |
| FX-CWIPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-011 frame 0: The painted map, lightness, completion 50, softness 30. | largest difference 2.4e-7 | yes |
| FX-CWIPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-012 frame 0: The painted map, hue, completion 50, softness 10. | largest difference 4.5e-7 | yes |
| FX-CWIPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-013 frame 0: The painted map, saturation, completion 60, softness 20. | largest difference 5.9e-7 | yes |
| FX-CWIPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-014 frame 0: The ramp blurred, blur 4, completion 50, softness 0: an even slope is not changed by the blur inside, so the cut is FX-CWIPE-003's. | largest difference 1.5e-7 | yes |
| FX-CWIPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-015 frame 0: The ramp blurred, blur 6, completion 10, softness 0: the outer pixels repeated, the dark end is lifted, so fewer columns go than without blur. | largest difference 1.5e-7 | yes |
| FX-CWIPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-016 frame 0: The checker stretched to 16 by 10, blur 3, completion 50, softness 20: soft squares. | largest difference 3.2e-7 | yes |
| FX-CWIPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-017 frame 0: The 4 by 2 checker centred, alpha, completion 50, softness 0: outside it the map is clear, 0, so only its eight pixels are kept. | largest difference 1.5e-7 | yes |
| FX-CWIPE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-018 frame 0: Completion 100: every pixel clear. | largest difference 0.0e0 | yes |
| FX-CWIPE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-019 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp, softness 0: frame 0 whole, frame 2 FX-CWIPE-003, frame 4 clear. | largest difference 1.5e-7 | yes |
| FX-CWIPE-019 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp, softness 0: frame 0 whole, frame 2 FX-CWIPE-003, frame 4 clear. | largest difference 1.5e-7 | yes |
| FX-CWIPE-019 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp, softness 0: frame 0 whole, frame 2 FX-CWIPE-003, frame 4 clear. | largest difference 0.0e0 | yes |
| FX-CWIPE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-020 frame 0: Blur keyed from 0 at frame 0 to 12 at frame 4, the ramp at 10, softness 0: frame 2 FX-CWIPE-015. | largest difference 1.5e-7 | yes |
| FX-CWIPE-020 frame 2: Blur keyed from 0 at frame 0 to 12 at frame 4, the ramp at 10, softness 0: frame 2 FX-CWIPE-015. | largest difference 1.5e-7 | yes |
| FX-CWIPE-020 frame 4: Blur keyed from 0 at frame 0 to 12 at frame 4, the ramp at 10, softness 0: frame 2 FX-CWIPE-015. | largest difference 1.5e-7 | yes |
| FX-CWIPE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-021 frame 0: A layer that is not in the composition, `gone`, completion 50: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-CWIPE-021 frame 4: A layer that is not in the composition, `gone`, completion 50: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-CWIPE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CWIPE-022 frame 0: A white solid whose in point is frame 4, alpha, completion 50: before it the map is empty, alpha 0, so at frame 0 everything is gone; at frame 4 nothing is. | largest difference 0.0e0 | yes |
| FX-CWIPE-022 frame 4: A white solid whose in point is frame 4, alpha, completion 50: before it the map is empty, alpha 0, so at frame 0 everything is gone; at frame 4 nothing is. | largest difference 1.5e-7 | yes |
| FX-CWIPE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-023 frame 0: FX-CWIPE-009 on the holder moved 2 right and 1 down: the same picture moved, since the map lies on the layer. | largest difference 1.5e-7 | yes |
| FX-CWIPE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-024 frame 0: The effect on an adjustment layer above the holder, the painted map, green: the map lies on the frame, the holder's own rectangle here, so this is FX-CWIPE-009. | largest difference 1.5e-7 | yes |
| FX-CWIPE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CWIPE-025 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-025 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-026 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-026 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-027 frame 0: Blur 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-027 frame 4: Blur 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-028 frame 0: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-028 frame 4: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-029 frame 0: A property written "value". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-029 frame 4: A property written "value". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-030 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-030 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-031 frame 0: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-031 frame 4: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-032 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-032 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CWIPE-033 frame 0: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-033 frame 4: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CWIPE-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_mixed.json: `a`'s Channel Wipe names `b`, and `b`'s Gradient Wipe names `a`: refused the same. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Channel Wipe whose layer is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cwipe_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cwipe_016.json is saved with its seven settings, the layer as written, blur 3 | {"blur":3,"completion":50,"fit":"stretch","invert":"off","layer":"card","property":"luminance","softness":20} | yes |
| the map the effect reads is never saved | None | yes |
| fx_cwipe_025.json is refused in a sentence naming completion | Channel Wipe's completion runs from 0 to 100, and this is 101. | yes |
| fx_cwipe_026.json is refused in a sentence naming softness | Channel Wipe's softness runs from 0 to 100, and this is -1. | yes |
| fx_cwipe_027.json is refused in a sentence naming blur | Channel Wipe's blur runs from 0 to 100, and this is 101. | yes |
| fx_cwipe_028.json is refused in a sentence naming blur | Channel Wipe's blur runs from 0 to 100, and this is -1. | yes |
| fx_cwipe_029.json is refused in a sentence naming property | Channel Wipe's property is "red", "green", "blue", "alpha", "luminance", "lightness", "hue" or "saturation", and this is "value". | yes |
| fx_cwipe_030.json is refused in a sentence naming fit | Channel Wipe's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| fx_cwipe_031.json is refused in a sentence naming invert | Channel Wipe's invert is "off" or "on", and this is "yes". | yes |
| fx_cwipe_032.json is refused in a sentence naming layer | Channel Wipe's layer is the name of a layer of this composition, and this is 3. | yes |
| a file with a Channel Wipe whose completion is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Channel Wipe without its completion is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Channel Wipe without its property is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion 101 is refused with a sentence, and nothing changes | Channel Wipe's completion runs from 0 to 100, and this is 101. | yes |
| blur 120 is refused with a sentence, and nothing changes | Channel Wipe's blur runs from 0 to 100, and this is 120. | yes |
| property "value" is refused with a sentence, and nothing changes | Channel Wipe's property is "red", "green", "blue", "alpha", "luminance", "lightness", "hue" or "saturation", and this is "value". | yes |
| blur keyed to 600 is refused with a sentence, and nothing changes | Channel Wipe's blur runs from 0 to 100, and this is 600. | yes |
| the layer itself read in hue, blur 6 is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview halves the blur and keeps the completion and the softness | ChannelWipe { layer: String("paint"), fit: "stretch", completion: 30.0, softness: 30.0, property: "red", blur: 4.0, invert: "off", map: None } | yes |
| a wipe never grows the drawing's bounds | 0 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cwipe_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_020.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cwipe_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cwipe_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cwipe_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 525824 pixels changed | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 930 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 1553 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 925 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 187 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 140 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself, completion 40 (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 170 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 in hue, completion 50, softness 20, blur 6 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 160801 pixels changed | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe layer 4 centred in alpha, completion 60, inverted on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 579981 pixels changed | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20 on three layers, frame 0, Full | largest difference 1 of 255, 961 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20 on three layers, frame 100, Full | largest difference 1 of 255, 1236 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20 on three layers, frame 239, Full | largest difference 1 of 255, 919 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20 on three layers, frame 0, Draft | largest difference 1 of 255, 22 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20 on three layers, frame 100, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20 on three layers, frame 239, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe completion 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Channel Wipe completion 100 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe completion 100 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe completion 100 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe completion 100 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe completion 100 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Channel Wipe completion 100 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Gradient Wipe unchanged: FX-GWIPE-001 to 028 (document 25), as B-129 runs them

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GWIPE-001 frame 0: As added: no layer named and completion 0, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-GWIPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-002 frame 0: No layer named, completion 50: with no map nothing changes. | largest difference 1.5e-7 | yes |
| FX-GWIPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-003 frame 0: The grey ramp as the map, completion 50, softness 0: its eight dark columns, 0 to 7, are wiped away and the eight light ones kept whole. The ramp layer is moved, scaled and switched off, none of which the map reads. | largest difference 1.5e-7 | yes |
| FX-GWIPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-004 frame 0: The ramp, completion 50, softness 100: each pixel kept by the ramp's brightness there, column 0 gone, column 15 whole, the rest between. | largest difference 1.9e-7 | yes |
| FX-GWIPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-005 frame 0: The ramp, completion 0, softness 50: nothing changes. | largest difference 1.5e-7 | yes |
| FX-GWIPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-006 frame 0: The ramp, completion 100, softness 50: every pixel clear. | largest difference 0.0e0 | yes |
| FX-GWIPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-007 frame 0: The ramp, completion 100, softness 0: every pixel clear, the white column too. | largest difference 0.0e0 | yes |
| FX-GWIPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-008 frame 0: FX-GWIPE-003 with Invert on: the eight light columns go, the dark ones stay. | largest difference 1.5e-7 | yes |
| FX-GWIPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-009 frame 0: The ramp, completion 30, softness 20: columns 0 to 2 gone, 6 and on whole, 3 to 5 fading in between. | largest difference 2.9e-7 | yes |
| FX-GWIPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-010 frame 0: A white solid, completion 99: nothing is dark enough to go. | largest difference 1.5e-7 | yes |
| FX-GWIPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-011 frame 0: A black solid, completion 1: everything goes at once. | largest difference 0.0e0 | yes |
| FX-GWIPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-012 frame 0: A white map clear at the top and more covering down each row, completion 50: a clear map reads as black, so rows 0 and 1 go and the rest stay. | largest difference 1.5e-7 | yes |
| FX-GWIPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-013 frame 0: A 4 by 2 checker of white and black, centred, completion 50: outside it the map is clear, black, so only its four white pixels are kept. | largest difference 1.5e-7 | yes |
| FX-GWIPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-014 frame 0: The checker tiled: kept and gone in a checker over the whole layer. | largest difference 1.5e-7 | yes |
| FX-GWIPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-015 frame 0: The checker stretched to 16 by 10, completion 50, softness 30: soft squares. | largest difference 1.5e-7 | yes |
| FX-GWIPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-016 frame 0: The holder names itself, completion 50: its red and blue squares, darker than half, go, its cream ones stay, and the clear corner stays clear. | largest difference 9.7e-8 | yes |
| FX-GWIPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-017 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 whole, frame 2 as FX-GWIPE-003, frame 4 clear. | largest difference 1.5e-7 | yes |
| FX-GWIPE-017 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 whole, frame 2 as FX-GWIPE-003, frame 4 clear. | largest difference 1.5e-7 | yes |
| FX-GWIPE-017 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 whole, frame 2 as FX-GWIPE-003, frame 4 clear. | largest difference 0.0e0 | yes |
| FX-GWIPE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-018 frame 0: Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004. | largest difference 1.5e-7 | yes |
| FX-GWIPE-018 frame 2: Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004. | largest difference 2.2e-7 | yes |
| FX-GWIPE-018 frame 4: Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004. | largest difference 1.9e-7 | yes |
| FX-GWIPE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-019 frame 0: A layer that is not in the composition, `gone`, completion 50: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GWIPE-019 frame 4: A layer that is not in the composition, `gone`, completion 50: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GWIPE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-GWIPE-020 frame 0: A white solid whose in point is frame 4, completion 50: before it the map is empty, which reads as black, so at frame 0 everything is gone; at frame 4 nothing is. | largest difference 0.0e0 | yes |
| FX-GWIPE-020 frame 4: A white solid whose in point is frame 4, completion 50: before it the map is empty, which reads as black, so at frame 0 everything is gone; at frame 4 nothing is. | largest difference 1.5e-7 | yes |
| FX-GWIPE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-021 frame 0: FX-GWIPE-003 on the holder moved 2 right and 1 down: the same picture moved, since the map lies on the layer. | largest difference 1.5e-7 | yes |
| FX-GWIPE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-022 frame 0: The effect on an adjustment layer above the holder, the ramp as its map: the map lies on the frame, which here is the holder's own rectangle, so this is FX-GWIPE-003. | largest difference 1.5e-7 | yes |
| FX-GWIPE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-023 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-023 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-024 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-024 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-025 frame 0: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-025 frame 4: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-026 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-026 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-027 frame 0: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-027 frame 4: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-028 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-028 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| cycle_mixed.json: `a`'s Gradient Wipe names `b`, and `b`'s Displacement Map names `a`: a circle through two kinds of effect, refused the same. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Gradient Wipe naming the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## Gradient Wipe unchanged on the card: its files and B-226's four reference settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gwipe_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gwipe_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1912799 pixels changed | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0 on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 50, softness 0 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 160801 pixels changed | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted on three layers, frame 0, Full | largest difference 1 of 255, 2822 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted on three layers, frame 100, Full | largest difference 1 of 255, 20053 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted on three layers, frame 239, Full | largest difference 1 of 255, 3496 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted on three layers, frame 0, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted on three layers, frame 100, Draft | largest difference 1 of 255, 541 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 stretched, completion 30, softness 25, inverted on three layers, frame 239, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100 on three layers, frame 0, Full | largest difference 1 of 255, 5058 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100 on three layers, frame 100, Full | largest difference 1 of 255, 1829 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100 on three layers, frame 239, Full | largest difference 1 of 255, 2783 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100 on three layers, frame 0, Draft | largest difference 1 of 255, 165 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100 on three layers, frame 100, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 centred, completion 70, softness 100 on three layers, frame 239, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1912799 pixels changed | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10 on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gradient Wipe layer 4 tiled, completion 45, softness 10 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-512 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_itself_luminance_half.png, the street read in its own luminance at completion 50, softness 0: the dark half of the street clear, the light half exactly as it was, nothing between; draws cleanly | [], 91380 pixels kept, 38220 clear, of 129600 | yes |
| 4_itself_hue_soft.png, the same read in hue, softness 20: other parts go first (reds and oranges before blues), soft edges between; draws cleanly | [], 120564 pixels differ from 3 | yes |
| 5_itself_luminance_blur.png, luminance at completion 50 with blur 20: the cleared shapes rounded into soft blobs, not following every detail; draws cleanly | [], 22866 pixels differ from the same without blur | yes |
| 6_other_layer.png, the opposite-colours layer as the gradient, luminance, completion 50: the light parts of the street go first; byte for byte Gradient Wipe's picture with the same settings, one engine; draws cleanly | [], 0 pixels differ from Gradient Wipe's, 82920 clear | yes |
| completion 100: every pixel clear; draws cleanly | [], 129600 of 129600 clear | yes |
| a gradient layer not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

353 of 353 checks pass.
