# D-700: Liquify's other brushes, freeze areas and earlier strokes

B-570: `core.liquify` takes After Effects' remaining Liquify brushes (Turbulence, Reflection, Clone, Reconstruction with Revert, Displace, Amplitwist and Affine), Freeze Area and Thaw Area, Turbulent Jitter, Clone Offset, Reconstruction Mode, a Freeze Area Mask, and the strokes brushed before the last, kept with the effect. Every expected pixel is `Fixtures/liquify_brushes/expected_liquify_brushes.json`, written by `tools/liquify_brushes_reference.py` before this code existed and printed in document 25 as FX-LIQUIFY-BRUSHES-001 to 028. Tolerance 2e-5. D-605's own fixtures stand unchanged (B-484).

## FX-LIQUIFY-BRUSHES-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LIQUIFY-BRUSHES-001 frame 0: Turbulence, brush 8, pressure 50, jitter 70, stroked across the middle: the stripes there scrambled smoothly. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-002 frame 0: FX-LIQUIFY-BRUSHES-001 at jitter 20: scrambled less. | largest difference 2.6e-7 | yes |
| FX-LIQUIFY-BRUSHES-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-003 frame 0: Reflection, brush 8, pressure 100, stroked from the top left to the bottom right: below the line, the stripes above it mirrored. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-004 frame 0: A Warp across the middle first, then Clone, brush 6, pressure 100, one dab near the top with Clone Offset 3 pixels down: the push copied up there. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-005 frame 0: A Warp across the middle first, then Reconstruction (Revert), brush 6, pressure 70, one dab in the middle: the push mostly undone there. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-006 frame 0: A Twirl in the middle and a Shift Pixels down it first, then Reconstruction (Displace) brushed from above the middle to the right: the displacement where it starts carried along. | largest difference 2.7e-7 | yes |
| FX-LIQUIFY-BRUSHES-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-007 frame 0: FX-LIQUIFY-BRUSHES-006 with Amplitwist: the turn and swell there carried along too. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-008 frame 0: FX-LIQUIFY-BRUSHES-006 with Affine: the whole local distortion, slant and all. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-009 frame 0: Freeze Area, brush 6, pressure 100, one dab in the middle, then FX-LIQUIFY-002's Warp: the push held back in the middle. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-010 frame 0: FX-LIQUIFY-BRUSHES-009 with a Thaw Area at pressure 50 over the freeze before the Warp: held back half as much. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-011 frame 0: FX-LIQUIFY-002's Warp with Freeze Area Mask 1, a box from (4, 2) to (12, 8): the drawing well inside it does not move. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-012 frame 0: FX-LIQUIFY-BRUSHES-011 with the layer moved three pixels right: the mask is the layer's, so all of it moves with it. | largest difference 2.2e-7 | yes |
| FX-LIQUIFY-BRUSHES-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-013 frame 0: Two strokes: a Warp across the middle, then Pucker, brush 10, pressure 100, one dab in the middle: both drawn. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-014 frame 0: FX-LIQUIFY-002's Warp as an earlier stroke and no stroke after it: FX-LIQUIFY-002's frame. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-015 frame 0: Turbulence with the jitter keyed from 10 at frame 0 to 100 at frame 4, linear: scrambled more as it goes. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-015 frame 2: Turbulence with the jitter keyed from 10 at frame 0 to 100 at frame 4, linear: scrambled more as it goes. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-015 frame 4: Turbulence with the jitter keyed from 10 at frame 0 to 100 at frame 4, linear: scrambled more as it goes. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-016 frame 0: FX-LIQUIFY-BRUSHES-004 with the Clone Offset keyed from 3 pixels down at frame 0 to 3 pixels up at frame 4: frame 0 FX-LIQUIFY-BRUSHES-004's, then copied from above instead. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-016 frame 4: FX-LIQUIFY-BRUSHES-004 with the Clone Offset keyed from 3 pixels down at frame 0 to 3 pixels up at frame 4: frame 0 FX-LIQUIFY-BRUSHES-004's, then copied from above instead. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-BRUSHES-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-BRUSHES-017 frame 0: Freeze Area Mask 2, of one. Nothing to freeze by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-017 frame 4: Freeze Area Mask 2, of one. Nothing to freeze by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-017: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-LIQUIFY-BRUSHES-018 frame 0: Tool "smudge", not one of the twelve. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-018 frame 4: Tool "smudge", not one of the twelve. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-019 frame 0: Turbulent Jitter 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-019 frame 4: Turbulent Jitter 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-020 frame 0: Reconstruction Mode "smooth", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-020 frame 4: Reconstruction Mode "smooth", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-021 frame 0: An earlier stroke with tool number 12, past the last. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-021 frame 4: An earlier stroke with tool number 12, past the last. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-022 frame 0: An earlier stroke with half a point over. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-022 frame 4: An earlier stroke with half a point over. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-023 frame 0: An earlier stroke with a point 300 per cent across. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-023 frame 4: An earlier stroke with a point 300 per cent across. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-024 frame 0: An earlier stroke with brush size 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-024 frame 4: An earlier stroke with brush size 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-025 frame 0: An earlier stroke with no points. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-025 frame 4: An earlier stroke with no points. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-026 frame 0: Freeze Area Mask 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-026 frame 4: Freeze Area Mask 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-027 frame 0: Clone Offset 10001 pixels across, past 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-027 frame 4: Clone Offset 10001 pixels across, past 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-BRUSHES-028 frame 0: An earlier stroke with reconstruction mode number 1.5, not whole. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-028 frame 4: An earlier stroke with reconstruction mode number 1.5, not whole. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-BRUSHES-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_liquify_brushes_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_brushes_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| D-605's fx_liquify_002.json, from before D-700, is saved with its six settings and none of the new ones | {"brush_pressure":50,"brush_size":8,"distortion_mesh_offset":[0,0],"distortion_percentage":100,"stroke":[25,50,75,50],"tool":"warp"} | yes |
| fx_liquify_brushes_013.json is saved with its earlier stroke as a list of lists | [[0,8,50,70,0,0,0,25,50,75,50]] | yes |
| fx_liquify_brushes_018.json is refused in a sentence naming Liquify and its tool | Liquify's tool is warp, twirl_clockwise, twirl_counterclockwise, pucker, bloat, shift_pixels, turbulence, reflection, clone, reconstruction, freeze_area or thaw_area, and this is "smudge". | yes |
| fx_liquify_brushes_019.json is refused in a sentence naming Liquify and its turbulent jitter | Liquify's turbulent jitter runs from 1 to 100, and this is 0. | yes |
| fx_liquify_brushes_020.json is refused in a sentence naming Liquify and its reconstruction mode | Liquify's reconstruction mode is revert, displace, amplitwist or affine, and this is "smooth". | yes |
| fx_liquify_brushes_021.json is refused in a sentence naming Liquify and its tool number | Liquify's earlier stroke 1 has tool number 12, and the tools are numbered 0 to 11. | yes |
| fx_liquify_brushes_022.json is refused in a sentence naming Liquify and its numbers | Liquify's earlier stroke 1 is its tool, brush size, brush pressure, turbulent jitter, clone offset x and y and reconstruction mode, then x then y for one point or more, and this has 10 numbers. | yes |
| fx_liquify_brushes_023.json is refused in a sentence naming Liquify and its point | Liquify's earlier stroke 1 has a point at 300 per cent, and they are -100 to 200. | yes |
| fx_liquify_brushes_024.json is refused in a sentence naming Liquify and its brush size | Liquify's earlier stroke 1 has brush size 0, and it is 1 to 1000. | yes |
| fx_liquify_brushes_025.json is refused in a sentence naming Liquify and its numbers | Liquify's earlier stroke 1 is its tool, brush size, brush pressure, turbulent jitter, clone offset x and y and reconstruction mode, then x then y for one point or more, and this has 7 numbers. | yes |
| fx_liquify_brushes_026.json is refused in a sentence naming Liquify and its freeze area mask | Liquify's freeze area mask runs from 0 to 1000, and this is 1001. | yes |
| fx_liquify_brushes_027.json is refused in a sentence naming Liquify and its clone offset | Liquify's clone offset runs from -10000 to 10000, and this is 10001. | yes |
| fx_liquify_brushes_028.json is refused in a sentence naming Liquify and its reconstruction mode number | Liquify's earlier stroke 1 has reconstruction mode number 1.5, and the modes are numbered 0 to 3. | yes |
| a file with a Liquify whose earlier strokes are one flat list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Liquify whose earlier stroke holds a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Liquify whose clone offset is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tool freeze (Freeze Area is freeze_area) is refused with a sentence, and nothing changes | Liquify's tool is warp, twirl_clockwise, twirl_counterclockwise, pucker, bloat, shift_pixels, turbulence, reflection, clone, reconstruction, freeze_area or thaw_area, and this is "freeze". | yes |
| reconstruction mode smooth is refused with a sentence, and nothing changes | Liquify's reconstruction mode is revert, displace, amplitwist or affine, and this is "smooth". | yes |
| an earlier stroke with tool number 12 is refused with a sentence, and nothing changes | Liquify's earlier stroke 1 has tool number 12, and the tools are numbered 0 to 11. | yes |
| turbulent jitter keyed to end at 0 is refused with a sentence, and nothing changes | Liquify's turbulent jitter runs from 1 to 100, and this is 0. | yes |
| a Freeze Area dab added as an earlier stroke, the last stroke Bloat is taken | taken | yes |
| clone offset keyed from (0, 0) to (5, -5) is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_liquify_brushes_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_brushes_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_brushes_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_brushes_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_brushes_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LIQUIFY-BRUSHES-001, fx_liquify_brushes_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-002, fx_liquify_brushes_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-003, fx_liquify_brushes_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-004, fx_liquify_brushes_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-005, fx_liquify_brushes_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-006, fx_liquify_brushes_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-007, fx_liquify_brushes_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-008, fx_liquify_brushes_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-009, fx_liquify_brushes_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-010, fx_liquify_brushes_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-011, fx_liquify_brushes_011.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-012, fx_liquify_brushes_012.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-013, fx_liquify_brushes_013.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-014, fx_liquify_brushes_014.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-015, fx_liquify_brushes_015.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-016, fx_liquify_brushes_016.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-017, fx_liquify_brushes_017.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-018, fx_liquify_brushes_018.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-019, fx_liquify_brushes_019.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-020, fx_liquify_brushes_020.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-021, fx_liquify_brushes_021.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-022, fx_liquify_brushes_022.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-023, fx_liquify_brushes_023.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-024, fx_liquify_brushes_024.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-025, fx_liquify_brushes_025.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-026, fx_liquify_brushes_026.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-027, fx_liquify_brushes_027.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-LIQUIFY-BRUSHES-028, fx_liquify_brushes_028.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle: the processor's frame 100 differs from the shot without it | 265142 pixels changed | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle, frame 0, Full | largest difference 1 of 255, 1944 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle, frame 100, Full | largest difference 1 of 255, 958 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle, frame 239, Full | largest difference 1 of 255, 838 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle, frame 100, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Turbulence, brush 300, stroked across the middle, frame 239, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask: the processor's frame 100 differs from the shot without it | 271009 pixels changed | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask, frame 0, Full | largest difference 1 of 255, 2309 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask, frame 100, Full | largest difference 1 of 255, 1044 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask, frame 239, Full | largest difference 1 of 255, 1384 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask, frame 100, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp across the middle with a box about the middle as its freeze area mask, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it: the processor's frame 100 differs from the shot without it | 310528 pixels changed | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it, frame 0, Full | largest difference 1 of 255, 2048 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it, frame 100, Full | largest difference 1 of 255, 984 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it, frame 239, Full | largest difference 1 of 255, 795 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it, frame 0, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it, frame 100, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it, frame 239, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-700 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 3_turbulence.png, Turbulence, brush 120, pressure 100, across the middle: the street under the stroke scrambled smoothly; differs from 1_before.png; draws cleanly | [], 22917 pixels differ from 1_before.png | yes |
| 4_reflection.png, Reflection, brush 160, pressure 100, across the middle going right: the street above the line mirrored below it; differs from 1_before.png; draws cleanly | [], 22205 pixels differ from 1_before.png | yes |
| 5_clone.png, 2_warp.png's push, then Clone, brush 100, pressure 100, one dab low on the street with Clone Offset 80 up: the push copied down there; differs from 2_warp.png; draws cleanly | [], 1147 pixels differ from 2_warp.png | yes |
| 6_revert.png, 2_warp.png's push, then Reconstruction (Revert), brush 100, pressure 100, one dab in the middle: the push undone there; differs from 2_warp.png; draws cleanly | [], 2338 pixels differ from 2_warp.png | yes |
| 7_freeze_then_warp.png, Freeze Area, brush 100, pressure 100, one dab in the middle, then 2_warp.png's push: the middle held still; differs from 2_warp.png; draws cleanly | [], 2338 pixels differ from 2_warp.png | yes |
| 8_freeze_mask.png, 2_warp.png's push with Freeze Area Mask 1, a box about the middle: the middle held still inside the box; differs from 2_warp.png; draws cleanly | [], 4675 pixels differ from 2_warp.png | yes |
| 8_freeze_mask.png: the pixel at the middle of the box is the street's, where 2_warp.png moved it | street [90, 140, 170, 255], frozen [90, 140, 170, 255], pushed [170, 195, 215, 255] | yes |
| Freeze Area Mask 2 of one: the street as it is, and EFFECT_PATH_MISSING said | ["EFFECT_PATH_MISSING Layer art's Liquify has no mask 2 that is on with two points or more, so it liquifies nothing."] | yes |

## Result

185 of 185 checks pass.
