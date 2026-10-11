# D-506: 3D Glasses

B-386: `core.three_d_glasses`, after After Effects' 3D Glasses (Perspective): a left and a right view, each another layer of the composition named by D-189's layer setting (on the card through `map_texture`) or the layer itself, made into one stereo picture. Adobe names the controls but publishes no formula and no defaults we found, so the rule and the settings when added are ours, stated in `tools/three_d_glasses_reference.py`: Scene Convergence slides the left view half its distance right and the right view half left, Vertical Alignment half up and half down; the red-green and red-blue views put the right view's brightness in red and the left's in green (and blue); Balanced Colored Red Blue keeps the right's red and the left's green and blue; Balance draws the two views towards their mean before that. Every expected pixel is `Fixtures/three_d_glasses/expected_three_d_glasses.json`, written by the tool before this code existed and printed in document 25 as FX-GLASSES-001 to 037. Tolerance 2e-5.

## FX-GLASSES-001 to 037 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLASSES-001 frame 0: As added: both views the layer itself, balanced colored red blue, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-GLASSES-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-002 frame 0: The stereo pair as red blue: the right view's grey in red, the left's in green and blue; the orange square red-fringed on its right, cyan on its left. | largest difference 7.3e-8 | yes |
| FX-GLASSES-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-003 frame 0: The pair as red green: the left view's grey in green only, blue empty. | largest difference 7.3e-8 | yes |
| FX-GLASSES-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-004 frame 0: The pair as balanced red blue at 0.75: the fringes a quarter softer than FX-GLASSES-002's. | largest difference 7.3e-8 | yes |
| FX-GLASSES-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-005 frame 0: The pair as balanced red green at 0.75. | largest difference 7.3e-8 | yes |
| FX-GLASSES-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-006 frame 0: The pair as balanced colored red blue at 0.75: the colours kept where the views agree, red and cyan fringes where the square differs. | largest difference 9.7e-8 | yes |
| FX-GLASSES-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-007 frame 0: The pair side by side: the left view squeezed into the left half, the right view into the right half. | largest difference 9.7e-8 | yes |
| FX-GLASSES-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-008 frame 0: The pair over under: the left view squeezed into the top half, the right into the bottom. | largest difference 9.7e-8 | yes |
| FX-GLASSES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-009 frame 0: The pair interlaced: even rows from the left view, odd rows from the right. | largest difference 9.7e-8 | yes |
| FX-GLASSES-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-010 frame 0: Red blue with scene convergence 2: the left view 1 pixel right, the right view 1 pixel left, so the squares lie 0 apart and line up. | largest difference 7.3e-8 | yes |
| FX-GLASSES-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-011 frame 0: Scene convergence 3: each view moved a pixel and a half, read between pixels. | largest difference 7.3e-8 | yes |
| FX-GLASSES-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-012 frame 0: Vertical alignment 2: the left view 1 pixel up, the right 1 pixel down. | largest difference 7.3e-8 | yes |
| FX-GLASSES-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-013 frame 0: In per cent: convergence 12.5 of 16 is 2 and alignment 20 of 10 is 2, so this is convergence 2 and alignment 2 in pixels. | largest difference 7.3e-8 | yes |
| FX-GLASSES-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-014 frame 0: FX-GLASSES-013 written in pixels. | largest difference 7.3e-8 | yes |
| FX-GLASSES-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-015 frame 0: FX-GLASSES-002 with left and right swapped. | largest difference 7.3e-8 | yes |
| FX-GLASSES-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-016 frame 0: Balanced colored at balance 0: the two views made one, the mean of the two, no depth. | largest difference 9.7e-8 | yes |
| FX-GLASSES-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-017 frame 0: Balanced colored at balance 2: the views pushed apart, the fringes stronger, colour held at 0. | largest difference 2.0e-7 | yes |
| FX-GLASSES-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-018 frame 0: Convergence keyed from 0 at frame 0 to 4 at frame 4: frame 0 is FX-GLASSES-002, frame 2 FX-GLASSES-010. | largest difference 7.3e-8 | yes |
| FX-GLASSES-018 frame 2: Convergence keyed from 0 at frame 0 to 4 at frame 4: frame 0 is FX-GLASSES-002, frame 2 FX-GLASSES-010. | largest difference 7.3e-8 | yes |
| FX-GLASSES-018 frame 4: Convergence keyed from 0 at frame 0 to 4 at frame 4: frame 0 is FX-GLASSES-002, frame 2 FX-GLASSES-010. | largest difference 7.3e-8 | yes |
| FX-GLASSES-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-019 frame 0: A left view that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GLASSES-019 frame 4: A left view that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GLASSES-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-GLASSES-020 frame 0: The right view with its in point at frame 4: before it the right view is empty, so red is empty; at frame 4 it is FX-GLASSES-002. | largest difference 7.3e-8 | yes |
| FX-GLASSES-020 frame 4: The right view with its in point at frame 4: before it the right view is empty, so red is empty; at frame 4 it is FX-GLASSES-002. | largest difference 7.3e-8 | yes |
| FX-GLASSES-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-021 frame 0: FX-GLASSES-006 on the holder moved 2 right and 1 down: the same picture moved, since the views lie on the layer. | largest difference 9.7e-8 | yes |
| FX-GLASSES-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-022 frame 0: The effect on an adjustment layer above the holder, the pair named: the views lie on the frame, the composition's size, so this is FX-GLASSES-006. | largest difference 9.7e-8 | yes |
| FX-GLASSES-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-023 frame 0: Only the left view named, the right the layer itself, red blue. | largest difference 1.1e-7 | yes |
| FX-GLASSES-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-024 frame 0: The left view's middle 8 by 6 as the left view, centred: it lies where it was cut, clear around it. | largest difference 7.3e-8 | yes |
| FX-GLASSES-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-025 frame 0: Side by side with convergence 5 and alignment 3: both unused, so this is FX-GLASSES-007. | largest difference 9.7e-8 | yes |
| FX-GLASSES-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-026 frame 0: The layer itself as both views, red blue, convergence 2: the shot as a grey anaglyph, its edges fringed. | largest difference 1.1e-7 | yes |
| FX-GLASSES-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASSES-027 frame 0: Scene convergence 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-027 frame 4: Scene convergence 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-028 frame 0: Scene convergence -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-028 frame 4: Scene convergence -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-029 frame 0: Vertical alignment 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-029 frame 4: Vertical alignment 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-030 frame 0: Balance 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-030 frame 4: Balance 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-031 frame 0: Balance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-031 frame 4: Balance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-032 frame 0: A 3D view written "anaglyph". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-032 frame 4: A 3D view written "anaglyph". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-033 frame 0: Units written "inches". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-033 frame 4: Units written "inches". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-034 frame 0: Swap written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-034 frame 4: Swap written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-035 frame 0: A left view written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-035 frame 4: A left view written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-036 frame 0: A right view written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-036 frame 4: A right view written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLASSES-037 frame 0: Balance keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-037 frame 4: Balance keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASSES-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the shot, each with 3D Glasses whose left view is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glasses_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glasses_002.json is saved with its eight settings, the views as written, red blue | {"balance":0.75,"left_view":"left","right_view":"right","scene_convergence":0,"swap_left_right":"off","three_d_view":"red_blue","units":"pixels","vertical_alignment":0} | yes |
| fx_glasses_027.json is refused in a sentence naming scene convergence | 3D Glasses's scene convergence runs from -1000 to 1000, and this is 1001. | yes |
| fx_glasses_028.json is refused in a sentence naming scene convergence | 3D Glasses's scene convergence runs from -1000 to 1000, and this is -1001. | yes |
| fx_glasses_029.json is refused in a sentence naming vertical alignment | 3D Glasses's vertical alignment runs from -1000 to 1000, and this is 1001. | yes |
| fx_glasses_030.json is refused in a sentence naming balance | 3D Glasses's balance runs from 0 to 10, and this is 11. | yes |
| fx_glasses_031.json is refused in a sentence naming balance | 3D Glasses's balance runs from 0 to 10, and this is -1. | yes |
| fx_glasses_032.json is refused in a sentence naming 3D view | 3D Glasses's 3D view is one of "side_by_side", "over_under", "interlace", "red_green", "red_blue", "balanced_red_green", "balanced_red_blue", "balanced_colored_red_blue", and this is "anaglyph". | yes |
| fx_glasses_033.json is refused in a sentence naming units | 3D Glasses's units are "pixels" or "percent", and this is "inches". | yes |
| fx_glasses_034.json is refused in a sentence naming swap left-right | 3D Glasses's swap left-right is "off" or "on", and this is "yes". | yes |
| fx_glasses_035.json is refused in a sentence naming left view | 3D Glasses's left view is the name of a layer of this composition, and this is 3. | yes |
| fx_glasses_036.json is refused in a sentence naming right view | 3D Glasses's right view is the name of a layer of this composition, and this is 3. | yes |
| a file with a 3D Glasses whose convergence is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a 3D Glasses without its convergence is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a 3D Glasses without its right view is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scene convergence 1001 is refused with a sentence, and nothing changes | 3D Glasses's scene convergence runs from -1000 to 1000, and this is 1001. | yes |
| 3D view "anaglyph" is refused with a sentence, and nothing changes | 3D Glasses's 3D view is one of "side_by_side", "over_under", "interlace", "red_green", "red_blue", "balanced_red_green", "balanced_red_blue", "balanced_colored_red_blue", and this is "anaglyph". | yes |
| units "inches" is refused with a sentence, and nothing changes | 3D Glasses's units are "pixels" or "percent", and this is "inches". | yes |
| balance keyed to 20 is refused with a sentence, and nothing changes | 3D Glasses's balance runs from 0 to 10, and this is 20. | yes |
| balanced red green at convergence 2 is taken | taken | yes |
| vertical alignment keyed from 0 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview halves the convergence and alignment in pixels, and keeps them in per cent (B-386b) | ThreeDGlasses { left_view: String("left"), right_view: String("right"), scene_convergence: 4.0, vertical_alignment: -2.0, units: "pixels", swap_left_right: "off", three_d_view: "red_blue", balance: 0.75, left_map: None, right_map: None } / ThreeDGlasses { left_view: String("left"), right_view: String("right"), scene_convergence: 8.0, vertical_alignment: -4.0, units: "percent", swap_left_right: "off", three_d_view: "red_blue", balance: 0.75, left_map: None, right_map: None } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glasses_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glasses_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glasses_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_glasses_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue on three layers, frame 0, Full | largest difference 1 of 255, 1159 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue on three layers, frame 100, Full | largest difference 1 of 255, 1420 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue on three layers, frame 239, Full | largest difference 1 of 255, 1186 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue on three layers, frame 100, Draft | largest difference 1 of 255, 86 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself against layer 4, red blue on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped on three layers, frame 0, Full | largest difference 1 of 255, 614 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped on three layers, frame 100, Full | largest difference 1 of 255, 1143 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped on three layers, frame 239, Full | largest difference 1 of 255, 740 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped on three layers, frame 0, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped on three layers, frame 100, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped on three layers, frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1385525 pixels changed | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7 on three layers, frame 0, Full | largest difference 1 of 255, 2388 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7 on three layers, frame 100, Full | largest difference 1 of 255, 449 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7 on three layers, frame 239, Full | largest difference 1 of 255, 867 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7 on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7 on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses the layer itself, interlaced, convergence 7 on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, side by side: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2041771 pixels changed | yes |
| the reference shot, 3D Glasses against layer 4, side by side on three layers, frame 0, Full | largest difference 1 of 255, 560 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, side by side on three layers, frame 100, Full | largest difference 1 of 255, 4964 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, side by side on three layers, frame 239, Full | largest difference 1 of 255, 1027 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, side by side on three layers, frame 0, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, side by side on three layers, frame 100, Draft | largest difference 1 of 255, 160 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, side by side on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, over under: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2060629 pixels changed | yes |
| the reference shot, 3D Glasses against layer 4, over under on three layers, frame 0, Full | largest difference 1 of 255, 2455 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, over under on three layers, frame 100, Full | largest difference 1 of 255, 161 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, over under on three layers, frame 239, Full | largest difference 1 of 255, 452 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, over under on three layers, frame 0, Draft | largest difference 1 of 255, 100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, over under on three layers, frame 100, Draft | largest difference 1 of 255, 6 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 3D Glasses against layer 4, over under on three layers, frame 239, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-506 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_red_blue.png, red blue as the two eyes stand: the street in grey with red and cyan fringes where the eyes disagree (watch it with red-cyan glasses); draws cleanly | [], 15444 of 129600 pixels coloured | yes |
| 4_lined_up.png, the same at convergence -6: the eyes agree, so between the 3 columns at each edge (which only one eye sees) the picture is plain grey; draws cleanly | [], 127980 of 127980 pixels grey | yes |
| 5_balanced_colored.png, as added but with the right eye named: the street in its own colours, fringed where the eyes disagree; draws cleanly | [], 15444 pixels differ from the left eye | yes |
| 6_side_by_side.png, side by side: the left eye squeezed into the left half, the right eye into the right half, which is the left half 3 pixels on; draws cleanly | [], 63720 of 63720 pixels match | yes |
| a right view not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

238 of 238 checks pass.
