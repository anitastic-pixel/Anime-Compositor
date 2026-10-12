# D-604: Warp

B-483: `core.warp` takes After Effects' Warp controls (Warp Style, Warp Axis, Bend, Horizontal Distortion, Vertical Distortion) and bends the drawing into the style's shape, drawn through D-601's forward mesh. Every expected pixel is `Fixtures/warp/expected_warp.json`, written by `tools/warp_reference.py` before this code existed and printed in document 25 as FX-WARP-001 to 030. Tolerance 2e-5.

## FX-WARP-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WARP-001 frame 0: The settings as they start: Arc, Bend 50: the drawing bent round a circle, its middle line an eighth of a turn, the sides drooping. | largest difference 2.5e-7 | yes |
| FX-WARP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-002 frame 0: Bend 0, both Distortions 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-WARP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-003 frame 0: Arc Lower, Bend 50: the bottom edge bowed down, the top kept. | largest difference 2.5e-7 | yes |
| FX-WARP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-004 frame 0: Arc Upper, Bend 50: the top edge bowed up, the bottom kept. | largest difference 2.5e-7 | yes |
| FX-WARP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-005 frame 0: Arch, Bend 50: lifted in the middle, the sides kept upright, clear below the middle. | largest difference 2.5e-7 | yes |
| FX-WARP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-006 frame 0: Bulge, Bend 50: the top and bottom bowed out together, cut off at the edges. | largest difference 2.5e-7 | yes |
| FX-WARP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-007 frame 0: Shell Lower, Bend 50: the bottom bowed down and narrowed toward it. | largest difference 2.5e-7 | yes |
| FX-WARP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-008 frame 0: Shell Upper, Bend 50: the top bowed up and narrowed toward it. | largest difference 2.5e-7 | yes |
| FX-WARP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-009 frame 0: Flag, Bend 50: one whole wave across, every row alike. | largest difference 1.9e-7 | yes |
| FX-WARP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-010 frame 0: Wave, Bend 50: the wave half a turn on from the top to the bottom. | largest difference 2.5e-7 | yes |
| FX-WARP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-011 frame 0: Fish, Bend 50: thick at one end and thin at the other. | largest difference 2.5e-7 | yes |
| FX-WARP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-012 frame 0: Rise, Bend 50: the right side raised and the left lowered, smoothly. | largest difference 2.5e-7 | yes |
| FX-WARP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-013 frame 0: Fisheye, Bend 50: swollen from the middle, the corners kept. | largest difference 2.5e-7 | yes |
| FX-WARP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-014 frame 0: Inflate, Bend 50: swollen both ways, the corners kept. | largest difference 2.5e-7 | yes |
| FX-WARP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-015 frame 0: Squeeze, Bend 50: drawn in across and swollen up and down. | largest difference 2.5e-7 | yes |
| FX-WARP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-016 frame 0: Twist, Bend 50: turned most in the middle, the corners kept. | largest difference 2.5e-7 | yes |
| FX-WARP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-017 frame 0: Arc at Bend -50: bent the other way, the sides lifted, the mirror image of FX-WARP-001 top for bottom. | largest difference 2.6e-7 | yes |
| FX-WARP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-018 frame 0: Flag with Warp Axis vertical: the wave runs down the drawing, the columns sway. | largest difference 2.5e-7 | yes |
| FX-WARP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-019 frame 0: Bend 0, Horizontal Distortion 60: no bend, the right side taller than the left, cut off where it passes the edges. | largest difference 2.5e-7 | yes |
| FX-WARP-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-020 frame 0: Bend 0, Vertical Distortion -60: the bottom narrower than the top. | largest difference 2.5e-7 | yes |
| FX-WARP-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-021 frame 0: Bulge, Bend 40, with both Distortions at 30. | largest difference 2.5e-7 | yes |
| FX-WARP-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-022 frame 0: Bend keyed from 0 at frame 0 to 100 at frame 4, linear, Twist: frame 0 the drawing, frame 4 a whole Twist, frame 2 between. | largest difference 1.9e-7 | yes |
| FX-WARP-022 frame 2: Bend keyed from 0 at frame 0 to 100 at frame 4, linear, Twist: frame 0 the drawing, frame 4 a whole Twist, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-WARP-022 frame 4: Bend keyed from 0 at frame 0 to 100 at frame 4, linear, Twist: frame 0 the drawing, frame 4 a whole Twist, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-WARP-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-023 frame 0: Bend keyed from 60 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-WARP-023 frame 2: Bend keyed from 60 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-WARP-023 frame 4: Bend keyed from 60 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-WARP-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-024 frame 0: FX-WARP-001 with the layer moved three pixels right: the bent drawing moves with it. | largest difference 2.5e-7 | yes |
| FX-WARP-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-025 frame 0: Twist at Bend 100 with Warp Axis vertical: u and v change places, so the turn runs the other way round from FX-WARP-022's frame 4. | largest difference 2.5e-7 | yes |
| FX-WARP-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARP-026 frame 0: Warp Style "spiral", not one of the fifteen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-026 frame 4: Warp Style "spiral", not one of the fifteen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARP-027 frame 0: Warp Axis "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-027 frame 4: Warp Axis "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARP-028 frame 0: Bend 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-028 frame 4: Bend 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARP-029 frame 0: Horizontal Distortion -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-029 frame 4: Horizontal Distortion -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARP-030 frame 0: Vertical Distortion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-030 frame 4: Vertical Distortion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WARP-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_warp_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warp_001.json is saved with its five settings, the style and axis as words | {"bend":50,"horizontal_distortion":0,"vertical_distortion":0,"warp_axis":"horizontal","warp_style":"arc"} | yes |
| fx_warp_026.json is refused in a sentence naming Warp and its warp style | Warp's warp style is arc, arc_lower, arc_upper, arch, bulge, shell_lower, shell_upper, flag, wave, fish, rise, fisheye, inflate, squeeze or twist, and this is "spiral". | yes |
| fx_warp_027.json is refused in a sentence naming Warp and its warp axis | Warp's warp axis is "horizontal" or "vertical", and this is "diagonal". | yes |
| fx_warp_028.json is refused in a sentence naming Warp and its bend | Warp's bend runs from -100 to 100, and this is 101. | yes |
| fx_warp_029.json is refused in a sentence naming Warp and its horizontal distortion | Warp's horizontal distortion runs from -100 to 100, and this is -101. | yes |
| a file with a Warp whose style is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Warp whose bend is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Warp without its axis is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| warp style spiral is refused with a sentence, and nothing changes | Warp's warp style is arc, arc_lower, arc_upper, arch, bulge, shell_lower, shell_upper, flag, wave, fish, rise, fisheye, inflate, squeeze or twist, and this is "spiral". | yes |
| bend 101 is refused with a sentence, and nothing changes | Warp's bend runs from -100 to 100, and this is 101. | yes |
| vertical distortion keyed to 150 is refused with a sentence, and nothing changes | Warp's vertical distortion runs from -100 to 100, and this is 150. | yes |
| twist, vertical, bend -70 is taken | taken | yes |
| bend keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_warp_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warp_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warp_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warp_022.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warp_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_warp_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warp_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Warp as added, Arc at Bend 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1920344 pixels changed | yes |
| the reference shot, Warp as added, Arc at Bend 50 on three layers, frame 0, Full | largest difference 1 of 255, 1843 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp as added, Arc at Bend 50 on three layers, frame 100, Full | largest difference 1 of 255, 900 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp as added, Arc at Bend 50 on three layers, frame 239, Full | largest difference 1 of 255, 799 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp as added, Arc at Bend 50 on three layers, frame 0, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp as added, Arc at Bend 50 on three layers, frame 100, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp as added, Arc at Bend 50 on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Flag, vertical, Bend -60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2004539 pixels changed | yes |
| the reference shot, Warp Flag, vertical, Bend -60 on three layers, frame 0, Full | largest difference 1 of 255, 1428 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Flag, vertical, Bend -60 on three layers, frame 100, Full | largest difference 1 of 255, 2005 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Flag, vertical, Bend -60 on three layers, frame 239, Full | largest difference 1 of 255, 841 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Flag, vertical, Bend -60 on three layers, frame 0, Draft | largest difference 1 of 255, 84 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Flag, vertical, Bend -60 on three layers, frame 100, Draft | largest difference 1 of 255, 130 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Flag, vertical, Bend -60 on three layers, frame 239, Draft | largest difference 1 of 255, 84 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Twist 80 with both distortions 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069720 pixels changed | yes |
| the reference shot, Warp Twist 80 with both distortions 30 on three layers, frame 0, Full | largest difference 1 of 255, 1828 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Twist 80 with both distortions 30 on three layers, frame 100, Full | largest difference 1 of 255, 3059 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Twist 80 with both distortions 30 on three layers, frame 239, Full | largest difference 1 of 255, 982 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Twist 80 with both distortions 30 on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Twist 80 with both distortions 30 on three layers, frame 100, Draft | largest difference 1 of 255, 126 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Twist 80 with both distortions 30 on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-604 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect, every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| Bend 0 and no distortion: the street byte for byte; draws cleanly | [], 0 pixels differ | yes |
| 2_arc.png, as added (Arc, Bend 50): the street bent round a circle, the bottom corners lifted clear; draws cleanly | [], 85407 pixels differ from the street, 13180 clear, both bottom corners clear: true | yes |
| 3_flag.png, Flag at Bend 50: the street rising then dropping, none clear above it a quarter across and clear three quarters across; draws cleanly | [], clear rows from the top at a quarter and three quarters across: 0, 34 | yes |
| 4_flag_vertical.png, the same Flag on the vertical axis: the wave runs down the street instead, so it differs from 3_flag.png; draws cleanly | [], 110868 pixels differ from 3_flag.png | yes |
| 5_twist.png, Twist at Bend 100: the middle turned round, the corners nearly in place; draws cleanly | [], 121888 pixels differ from the street | yes |
| 6_horizontal_distortion_60.png, Bend 0, Horizontal Distortion 60: the right side taller than the left, so the top left corner clear and the top right not; draws cleanly | [], top left clear: true, top right clear: false | yes |

## Result

176 of 176 checks pass.
