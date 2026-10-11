# D-517: Twister

B-397: `core.twister`, after CycoreFX's CC Twister (Transition), under our own name: the layer twists over round a line through the Center, along the Axis, a strip at a time like a ribbon, showing the Backside (D-189's layer setting, centred on the drawing; nothing when none is chosen) where a strip has turned over. CycoreFX names the controls but publishes no formula and no defaults, so the rule and the settings when added are ours, stated in `tools/twister_reference.py`: across the drawing a strip at t (0 where the twist starts, 1 at the far side) is turned by pi clamp(2 Completion / 100 - t, 0, 1); each point reads the picture squeezed toward the line by the cosine of that turn, the front while it faces the viewer and the back after, and Shading multiplies the colour by that cosine's size. Every expected pixel is `Fixtures/twister/expected_twister.json`, written by the tool before this code existed and printed in document 25 as FX-TWIST-001 to 030. Tolerance 2e-5.

## FX-TWIST-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TWIST-001 frame 0: As added: completion 0, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-TWIST-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-002 frame 0: Completion 100 with nothing on the back: every pixel clear. | largest difference 0.0e0 | yes |
| FX-TWIST-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-003 frame 0: Completion 100 with the paint on the back: the paint exactly, shading on but every strip flat again. | largest difference 1.5e-7 | yes |
| FX-TWIST-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-004 frame 0: Completion 25, the paint on the back, shading off: the left half twisting (the far left nearly edge on), the right half untouched. | largest difference 2.1e-7 | yes |
| FX-TWIST-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-005 frame 0: FX-TWIST-004 with shading on: the twisting strips darker the more they have turned. | largest difference 1.9e-7 | yes |
| FX-TWIST-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-006 frame 0: Completion 50, the paint on the back: the left half shows the paint, the right half the holder, squeezed toward the middle row where they turn. | largest difference 1.9e-7 | yes |
| FX-TWIST-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-007 frame 0: Completion 60, axis 90: the twist runs downward, the top rows over first. | largest difference 1.4e-7 | yes |
| FX-TWIST-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-008 frame 0: Completion 45, axis 30, Center at 40, 60: a slanted twist off the middle. | largest difference 2.2e-7 | yes |
| FX-TWIST-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-009 frame 0: Completion 70, axis -135, shading off: the twist from the bottom right. | largest difference 1.5e-7 | yes |
| FX-TWIST-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-010 frame 0: Completion 50, nothing on the back: where the paint was, clear. | largest difference 1.9e-7 | yes |
| FX-TWIST-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-011 frame 0: The holder on its own back, completion 50: the layer twists into itself. | largest difference 1.9e-7 | yes |
| FX-TWIST-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-012 frame 0: The paint's middle 8 by 6 on the back, centred: it lies where it was cut, clear round it. | largest difference 1.9e-7 | yes |
| FX-TWIST-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-013 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, frame 2 FX-TWIST-006, frame 4 the paint. | largest difference 1.5e-7 | yes |
| FX-TWIST-013 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, frame 2 FX-TWIST-006, frame 4 the paint. | largest difference 1.9e-7 | yes |
| FX-TWIST-013 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, frame 2 FX-TWIST-006, frame 4 the paint. | largest difference 1.5e-7 | yes |
| FX-TWIST-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-014 frame 0: Axis keyed from 0 at frame 0 to 90 at frame 4, completion 60: frame 4 is FX-TWIST-007. | largest difference 1.5e-7 | yes |
| FX-TWIST-014 frame 2: Axis keyed from 0 at frame 0 to 90 at frame 4, completion 60: frame 4 is FX-TWIST-007. | largest difference 1.7e-7 | yes |
| FX-TWIST-014 frame 4: Axis keyed from 0 at frame 0 to 90 at frame 4, completion 60: frame 4 is FX-TWIST-007. | largest difference 1.4e-7 | yes |
| FX-TWIST-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-015 frame 0: Center keyed from 50, 50 at frame 0 to 40, 60 at frame 4, completion 45, axis 30: frame 4 is FX-TWIST-008. | largest difference 1.8e-7 | yes |
| FX-TWIST-015 frame 4: Center keyed from 50, 50 at frame 0 to 40, 60 at frame 4, completion 45, axis 30: frame 4 is FX-TWIST-008. | largest difference 2.2e-7 | yes |
| FX-TWIST-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-016 frame 0: A back layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-TWIST-016 frame 4: A back layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-TWIST-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-TWIST-017 frame 0: The paint with its in point at frame 4 on the back: before it the back is clear; at frame 4 FX-TWIST-006. | largest difference 1.9e-7 | yes |
| FX-TWIST-017 frame 4: The paint with its in point at frame 4 on the back: before it the back is clear; at frame 4 FX-TWIST-006. | largest difference 1.9e-7 | yes |
| FX-TWIST-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-018 frame 0: FX-TWIST-006 on the holder moved 2 right and 1 down: the same picture moved, since the back lies on the layer. | largest difference 1.9e-7 | yes |
| FX-TWIST-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-019 frame 0: The effect on an adjustment layer above the holder: the back lies on the frame, which here is the holder's own rectangle, so this is FX-TWIST-006. | largest difference 1.9e-7 | yes |
| FX-TWIST-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-020 frame 0: Completion 50, Center at -200, 50: the line far off to the left, so the twist reaches across the drawing at once, every strip partly turned. | largest difference 2.4e-7 | yes |
| FX-TWIST-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-021 frame 0: Axis 360: the same as axis 0, FX-TWIST-006. | largest difference 1.9e-7 | yes |
| FX-TWIST-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIST-022 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-022 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-023 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-023 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-024 frame 0: Axis 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-024 frame 4: Axis 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-025 frame 0: Axis -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-025 frame 4: Axis -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-026 frame 0: Center x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-026 frame 4: Center x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-027 frame 0: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-027 frame 4: Center y -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-028 frame 0: Shading "maybe", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-028 frame 4: Shading "maybe", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-029 frame 0: A back layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-029 frame 4: A back layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIST-030 frame 0: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-030 frame 4: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TWIST-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Twister whose back layer is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twist_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twist_008.json is saved with its five settings, the back as written | {"axis":30,"backside":"paint","center":[40,60],"completion":45,"shading":"on"} | yes |
| fx_twist_022.json is refused in a sentence naming completion | Twister's completion runs from 0 to 100, and this is 101. | yes |
| fx_twist_023.json is refused in a sentence naming completion | Twister's completion runs from 0 to 100, and this is -1. | yes |
| fx_twist_024.json is refused in a sentence naming axis | Twister's axis runs from -3600 to 3600, and this is 3601. | yes |
| fx_twist_025.json is refused in a sentence naming axis | Twister's axis runs from -3600 to 3600, and this is -3601. | yes |
| fx_twist_026.json is refused in a sentence naming center | Twister's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_twist_027.json is refused in a sentence naming center | Twister's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_twist_028.json is refused in a sentence naming shading | Twister's shading is "on" or "off", and this is "maybe". | yes |
| fx_twist_029.json is refused in a sentence naming backside | Twister's backside is the name of a layer of this composition, and this is 3. | yes |
| a file with a Twister whose completion is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Twister without its completion is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Twister without its backside is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Twister whose Center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion 101 is refused with a sentence, and nothing changes | Twister's completion runs from 0 to 100, and this is 101. | yes |
| shading "maybe" is refused with a sentence, and nothing changes | Twister's shading is "on" or "off", and this is "maybe". | yes |
| axis keyed to 4000 is refused with a sentence, and nothing changes | Twister's axis runs from -3600 to 3600, and this is 4000. | yes |
| completion 70 with nothing on the back is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview changes no setting (none is a distance) | Twister { completion: 45.0, backside: String("paint"), shading: "on", center: [40.0, 60.0], axis: 30.0, map: None } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twist_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twist_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twist_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_twist_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1604789 pixels changed | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 1158 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 654 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 724 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 40, layer 4 on the back (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2064325 pixels changed | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back on three layers, frame 0, Full | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back on three layers, frame 100, Full | largest difference 1 of 255, 17 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back on three layers, frame 239, Full | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back on three layers, frame 0, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back on three layers, frame 100, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 60, axis 90, shading off, nothing on the back on three layers, frame 239, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 851687 pixels changed | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back on three layers, frame 0, Full | largest difference 1 of 255, 3725 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back on three layers, frame 100, Full | largest difference 1 of 255, 1367 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back on three layers, frame 239, Full | largest difference 1 of 255, 1646 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back on three layers, frame 0, Draft | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 30, axis -30, Center [30, 60], layer 4 on the back on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 100, layer 4 on the back: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Twister completion 100, layer 4 on the back on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 100, layer 4 on the back on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 100, layer 4 on the back on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 100, layer 4 on the back on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 100, layer 4 on the back on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Twister completion 100, layer 4 on the back on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-517 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_begun.png, completion 25: the left half twisting toward edge on and darkening, squeezed toward the middle row; the right half untouched; draws cleanly | [], pixels changed left 60176 right 0 | yes |
| 4_half_over.png, completion 50: the left half turned over to the opposite colours, the right half starting to turn, both squeezed toward the middle row near the centre line; draws cleanly | [], pixels changed left 64800 right 60147 | yes |
| 5_down_unshaded.png, completion 50, axis 90, shading off: the twist runs downward, the top half turned over, no darkening; draws cleanly | [], 128396 pixels differ from 4 | yes |
| 6_done.png, completion 100: turned over entirely, the opposite colours; draws cleanly | [], largest difference 0 of 255 from the back, 0 pixels | yes |
| a backside not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

203 of 203 checks pass.
