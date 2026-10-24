# D-565: Energy Stroke's extras

B-445, docs/effects/EFFECTS.md row 53 (P0-22): Start and End Roundness shape each end of the core, from round (100, as added and as D-508 drew) to flat (0); Noise Along Path off reads the wobble's noise on the layer, so it stays put while the beam moves through it; Alpha Mode lets every mask of the layer that is on, whatever its mode, cut the core, the glow or both, turned over by Invert Masks. The core from two points and the size at each end were D-508's. The rules are ours, stated in `tools/energy_stroke_extras_reference.py`. Every expected pixel is `Fixtures/energy_stroke_extras/expected_energy_stroke_extras.json`, written by the tool before this code existed and printed in document 25 as FX-ENERGYX-001 to 033. Tolerance 2e-5. Open masks stay a gap (document 19). The masks' cut runs on the processor, which holds the masks; the ends and the noise on the layer run on the card too.

## FX-ENERGYX-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ENERGYX-001 frame 0: A core 6 across from (4, 5) to (12, 5), no glow, Start and End Roundness 100 written out: D-508's round ends, the frame without them. | largest difference 2.3e-8 | yes |
| FX-ENERGYX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-002 frame 0: Start and End Roundness 0: both ends flat, cut square across at x 4 and x 12. | largest difference 0.0e0 | yes |
| FX-ENERGYX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-003 frame 0: Start Roundness 0, End Roundness 100: flat at the left, round at the right. | largest difference 2.3e-8 | yes |
| FX-ENERGYX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-004 frame 0: Both 50: each end half an ellipse, half as long as the round end. | largest difference 5.2e-9 | yes |
| FX-ENERGYX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-005 frame 0: Both 0 with a glow, spread 6: the core's ends flat, the glow still round past them. | largest difference 1.3e-8 | yes |
| FX-ENERGYX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-006 frame 0: End Roundness keyed from 0 at frame 0 to 100 at frame 4, linear: the right end rounds out, frames 0, 2 and 4. | largest difference 2.3e-8 | yes |
| FX-ENERGYX-006 frame 2: End Roundness keyed from 0 at frame 0 to 100 at frame 4, linear: the right end rounds out, frames 0, 2 and 4. | largest difference 2.3e-8 | yes |
| FX-ENERGYX-006 frame 4: End Roundness keyed from 0 at frame 0 to 100 at frame 4, linear: the right end rounds out, frames 0, 2 and 4. | largest difference 2.3e-8 | yes |
| FX-ENERGYX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-007 frame 0: The box mask as the path, drawn whole, both 0: a closed path drawn whole has no ends, the frame with them round. | largest difference 1.1e-8 | yes |
| FX-ENERGYX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-008 frame 0: The box, End offset 50, both 0: half way round, flat at its top-left and bottom-right corners' ends. | largest difference 1.1e-8 | yes |
| FX-ENERGYX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-009 frame 0: Both 0, End size 50: the right end flat and half as wide. | largest difference 0.0e0 | yes |
| FX-ENERGYX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-010 frame 0: Both 0 with distortion 1 at scale 3: the ends flat across the wobbled beam's first and last bit. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-011 frame 0: The open roof shape, Start Roundness 0, End Roundness 25, core 4. | largest difference 1.1e-8 | yes |
| FX-ENERGYX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-012 frame 0: Noise Along Path on, written out, with distortion 1 at scale 3: D-508's wobble, the frame without the setting. | largest difference 3.0e-8 | yes |
| FX-ENERGYX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-013 frame 0: Noise Along Path off: the noise read on the layer, a different wobble. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-014 frame 0: Off on the box, distortion 1.5 at scale 4, speed 0: still joined round the box, the same at frames 0 and 3. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-014 frame 3: Off on the box, distortion 1.5 at scale 4, speed 0: still joined round the box, the same at frames 0 and 3. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-015 frame 0: Off on the box, speed 50: the wobble moves, frames 0, 1 and 2. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-015 frame 1: Off on the box, speed 50: the wobble moves, frames 0, 1 and 2. | largest difference 3.0e-8 | yes |
| FX-ENERGYX-015 frame 2: Off on the box, speed 50: the wobble moves, frames 0, 1 and 2. | largest difference 3.0e-8 | yes |
| FX-ENERGYX-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-016 frame 2: Off, End offset keyed from 0 at frame 0 to 100 at frame 4: the beam draws on through a wobble that stays put, frames 2 and 4. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-016 frame 4: Off, End offset keyed from 0 at frame 0 to 100 at frame 4: the beam draws on through a wobble that stays put, frames 2 and 4. | largest difference 2.9e-8 | yes |
| FX-ENERGYX-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-017 frame 0: Alpha Mode masks, the small box (5, 4) to (9, 6) as a mask of mode None: the core and glow only inside it. | largest difference 0.0e0 | yes |
| FX-ENERGYX-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-018 frame 0: Alpha Mode core: the core only inside the small box, the glow everywhere. | largest difference 2.5e-8 | yes |
| FX-ENERGYX-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-019 frame 0: Alpha Mode glow: the glow only inside, the core all along. | largest difference 1.9e-9 | yes |
| FX-ENERGYX-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-020 frame 0: Alpha Mode masks, Invert Masks on: everything but the inside. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-021 frame 0: Alpha Mode off with the small box: nothing cut, the frame without it. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-022 frame 0: Alpha Mode masks, the small box feathered 2: a soft cut. | largest difference 8.8e-8 | yes |
| FX-ENERGYX-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-023 frame 0: Two masks, the left six columns in mode Add and the right five in mode None: both cut alike, the mode not read; the Add mask also cuts the drawing, as a mask always does, and the light is added over it. | largest difference 5.1e-8 | yes |
| FX-ENERGYX-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-024 frame 0: The small box inverted as a mask (its own Inverted): kept outside it. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-025 frame 0: The box as both the path and the cut: the beam along its edge, half of it outside, the core and glow cut at the box's edge. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-026 frame 0: Alpha Mode masks, composite add, the small box: the drawing beneath is not cut, only the light. | largest difference 5.1e-8 | yes |
| FX-ENERGYX-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-027 frame 0: Alpha Mode masks with no mask. No mask to cut by: the beam is drawn uncut, the effect is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-027 frame 4: Alpha Mode masks with no mask. No mask to cut by: the beam is drawn uncut, the effect is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-027: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ENERGYX-028 frame 0: Alpha Mode core with the one mask switched off. No mask to cut by: the beam is drawn uncut, the effect is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-028 frame 4: Alpha Mode core with the one mask switched off. No mask to cut by: the beam is drawn uncut, the effect is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-8 | yes |
| FX-ENERGYX-028: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ENERGYX-029 frame 0: Start roundness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-029 frame 4: Start roundness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGYX-030 frame 0: End roundness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-030 frame 4: End roundness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGYX-031 frame 0: Noise Along Path "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-031 frame 4: Noise Along Path "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGYX-032 frame 0: Alpha Mode "enable", not "off", "masks", "core" or "glow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-032 frame 4: Alpha Mode "enable", not "off", "masks", "core" or "glow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGYX-033 frame 0: Invert Masks "1", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-033 frame 4: Invert Masks "1", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGYX-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_energyx_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energyx_020.json is saved with its twenty-five settings, Alpha Mode masks, Invert Masks on | {"alpha_mode":"masks","composite":"transparent","core_end":[75,50],"core_size":3,"core_softness":30,"core_start":[25,50],"distortion_amount":0,"distortion_scale":60,"distortion_speed":20,"end_offset":100,"end_roundness":100,"end_size":100,"evolution":0,"flicker_amount":0,"flicker_speed":20,"glow_color":"#3080ff","glow_intensity":100,"glow_spread":6,"invert_masks":"on","noise_along_path":"on","random_seed":0,"source":"points","start_offset":0,"start_roundness":100,"start_size":100} | yes |
| D-508's fx_energy_001.json opened and saved holds what it held, none of the five new settings written | the same | yes |
| D-508's fx_energy_009.json opened and saved holds what it held, none of the five new settings written | the same | yes |
| D-508's fx_energy_032.json opened and saved holds what it held, none of the five new settings written | the same | yes |
| a Start Roundness of 30 and Alpha Mode glow are valid |  | yes |
| fx_energyx_029.json is refused in a sentence naming start roundness | Energy Stroke's start roundness runs from 0 to 100, and this is -1. | yes |
| fx_energyx_030.json is refused in a sentence naming end roundness | Energy Stroke's end roundness runs from 0 to 100, and this is 101. | yes |
| fx_energyx_031.json is refused in a sentence naming noise along path | Energy Stroke's noise along path is "on" or "off", and this is "yes". | yes |
| fx_energyx_032.json is refused in a sentence naming alpha mode | Energy Stroke's alpha mode is "off", "masks", "core" or "glow", and this is "enable". | yes |
| fx_energyx_033.json is refused in a sentence naming invert masks | Energy Stroke's invert masks is "on" or "off", and this is "1". | yes |
| a file with an Energy Stroke whose Start Roundness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-565)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_energyx_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energyx_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 91622 pixels changed | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40 on three layers, frame 0, Full | largest difference 1 of 255, 2100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40 on three layers, frame 100, Full | largest difference 1 of 255, 1098 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40 on three layers, frame 239, Full | largest difference 1 of 255, 1267 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40 on three layers, frame 0, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40 on three layers, frame 100, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke core 30, Start Roundness 0, End Roundness 40 on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 96001 pixels changed | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40 on three layers, frame 0, Full | largest difference 1 of 255, 1992 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40 on three layers, frame 100, Full | largest difference 1 of 255, 1154 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40 on three layers, frame 239, Full | largest difference 1 of 255, 1325 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40 on three layers, frame 0, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40 on three layers, frame 100, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Noise Along Path off, distortion 20 at scale 80, speed 40 on three layers, frame 239, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 91622 pixels changed | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer on three layers, frame 0, Full | largest difference 1 of 255, 5232 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer on three layers, frame 100, Full | largest difference 1 of 255, 5583 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer on three layers, frame 239, Full | largest difference 1 of 255, 5036 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer on three layers, frame 0, Draft | largest difference 1 of 255, 16756 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer on three layers, frame 100, Draft | largest difference 1 of 255, 16670 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode core, a mask of mode None on each layer on three layers, frame 239, Draft | largest difference 1 of 255, 16944 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 91622 pixels changed | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner on three layers, frame 0, Full | largest difference 1 of 255, 5292 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner on three layers, frame 100, Full | largest difference 1 of 255, 5622 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner on three layers, frame 239, Full | largest difference 1 of 255, 5125 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner on three layers, frame 0, Draft | largest difference 1 of 255, 16765 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner on three layers, frame 100, Draft | largest difference 1 of 255, 16677 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke Alpha Mode masks, Invert Masks on, a small mask in the corner on three layers, frame 239, Draft | largest difference 1 of 255, 16950 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-565 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_round_ends_as_added.png: drawn without a warning, the street changed, lit at (72, 135), unlike the picture before | 25405 pixels changed; at (72, 135) [255, 255, 255, 255]; 25405 pixels unlike the picture before; warnings [] | yes |
| 2_flat_ends.png: drawn without a warning, the street changed, lit at (72, 135), unlike the picture before | 25405 pixels changed; at (72, 135) [255, 255, 255, 255]; 172 pixels unlike the picture before; warnings [] | yes |
| 3_flat_start_pointed_end.png: drawn without a warning, the street changed, lit at (240, 135), unlike the picture before | 23752 pixels changed; at (240, 135) [255, 255, 255, 255]; 18046 pixels unlike the picture before; warnings [] | yes |
| 4_noise_along_path.png: drawn without a warning, the street changed, lit at (240, 135), unlike the picture before | 26340 pixels changed; at (240, 135) [255, 255, 255, 255]; 24894 pixels unlike the picture before; warnings [] | yes |
| 5_noise_on_the_layer.png: drawn without a warning, the street changed, lit at (240, 135), unlike the picture before | 25179 pixels changed; at (240, 135) [255, 255, 255, 255]; 25582 pixels unlike the picture before; warnings [] | yes |
| 6_masks_cut_the_core.png: drawn without a warning, the street changed, lit at (240, 135), unlike the picture before | 25405 pixels changed; at (240, 135) [255, 255, 255, 255]; 20277 pixels unlike the picture before; warnings [] | yes |
| 7_masks_cut_both_inverted.png: drawn without a warning, the street changed, lit at (100, 135), unlike the picture before | 14844 pixels changed; at (100, 135) [255, 255, 255, 255]; 13197 pixels unlike the picture before; warnings [] | yes |

## Result

191 of 191 checks pass.
