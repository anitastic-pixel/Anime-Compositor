# B-471: CC Rainfall

D-582 (b), accepted by the owner on 2026-10-12: "go with your recommendations on all six". Every expected pixel is `Fixtures/cc_rainfall/expected_cc_rainfall.json`, written by `tools/cc_rainfall_reference.py` before this code existed and printed in document 25 as FX-RAIN3D-001 to 037. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. `core.rain` (D-163) is a separate effect and is not changed.

## FX-RAIN3D-001 to 037 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAIN3D-001 frame 0: As added: 5000 drops, size 3, scene depth 5000, speed 4000, wind 0, variation 0, spread 15, colour #b4bec8, opacity 25, Lighten, composited with the night. In a picture 48 wide the volume is a hundred times deeper than the focal length, so most drops are far, faint and fine; the few near ones are long pale streaks. | largest difference 9.0e-9 | yes |
| FX-RAIN3D-001 frame 2: As added: 5000 drops, size 3, scene depth 5000, speed 4000, wind 0, variation 0, spread 15, colour #b4bec8, opacity 25, Lighten, composited with the night. In a picture 48 wide the volume is a hundred times deeper than the focal length, so most drops are far, faint and fine; the few near ones are long pale streaks. | largest difference 1.6e-8 | yes |
| FX-RAIN3D-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-002 frame 0: One drop at the front, size 2, falling 240 pixels a second, 10 a frame at 24, a streak 5 long, opacity 100: frame 1 is frame 0's streak 10 pixels lower, frame 4 has it 40 lower, wrapped back to the top. | largest difference 2.2e-8 | yes |
| FX-RAIN3D-002 frame 1: One drop at the front, size 2, falling 240 pixels a second, 10 a frame at 24, a streak 5 long, opacity 100: frame 1 is frame 0's streak 10 pixels lower, frame 4 has it 40 lower, wrapped back to the top. | largest difference 2.2e-8 | yes |
| FX-RAIN3D-002 frame 4: One drop at the front, size 2, falling 240 pixels a second, 10 a frame at 24, a streak 5 long, opacity 100: frame 1 is frame 0's streak 10 pixels lower, frame 4 has it 40 lower, wrapped back to the top. | largest difference 1.7e-8 | yes |
| FX-RAIN3D-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-003 frame 1: The same drop, size 0.5: a hairline at half strength. | largest difference 8.1e-9 | yes |
| FX-RAIN3D-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-004 frame 1: Size 0: no drop is drawn, so the night is untouched. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-005 frame 1: Drops 0: the night, untouched. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-006 frame 0: Wind 120 pixels a second: the drop drifts 5 pixels right a frame as it falls 10, its streak slanting. | largest difference 2.5e-8 | yes |
| FX-RAIN3D-006 frame 1: Wind 120 pixels a second: the drop drifts 5 pixels right a frame as it falls 10, its streak slanting. | largest difference 2.5e-8 | yes |
| FX-RAIN3D-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-007 frame 0: Six drops, wind 120, variation 50: each its own wind, between 60 and 180. | largest difference 2.9e-8 | yes |
| FX-RAIN3D-007 frame 2: Six drops, wind 120, variation 50: each its own wind, between 60 and 180. | largest difference 2.9e-8 | yes |
| FX-RAIN3D-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-008 frame 1: Six drops, spread 60: each tilted its own way, up to 30 degrees either side of straight down. | largest difference 2.9e-8 | yes |
| FX-RAIN3D-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-009 frame 1: Six drops in a volume 48 deep, the focal length: the far ones down to half size, half speed, half strength, and laid first. | largest difference 2.6e-8 | yes |
| FX-RAIN3D-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-010 frame 1: Six drops, ground level 50: nothing below row 16. | largest difference 3.0e-8 | yes |
| FX-RAIN3D-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-011 frame 0: Six drops, speed 0: each a round dot, still, so frame 4 is frame 0. | largest difference 2.6e-8 | yes |
| FX-RAIN3D-011 frame 4: Six drops, speed 0: each a round dot, still, so frame 4 is frame 0. | largest difference 2.6e-8 | yes |
| FX-RAIN3D-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-012 frame 1: Six drops, seed 7: drops of their own. | largest difference 2.8e-8 | yes |
| FX-RAIN3D-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-013 frame 1: Seed 7.9 counts as 7: FX-RAIN3D-012. | largest difference 2.8e-8 | yes |
| FX-RAIN3D-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-014 frame 1: Six drops at opacity 40. | largest difference 1.5e-8 | yes |
| FX-RAIN3D-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-015 frame 1: Six dark drops (#202020) on the pale wall, Composite: they darken it. | largest difference 1.0e-7 | yes |
| FX-RAIN3D-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-016 frame 1: The same with Lighten: rain never darkens, so the wall is untouched. | largest difference 7.6e-8 | yes |
| FX-RAIN3D-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-017 frame 1: Six drops, Composite With Original off: the rain alone, on clear. | largest difference 2.8e-8 | yes |
| FX-RAIN3D-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-018 frame 1: Six drops on the night with its right half clear: the rain falls over the clear half too. | largest difference 2.8e-8 | yes |
| FX-RAIN3D-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-019 frame 2: FX-RAIN3D-007 at 30 frames a second: on frame 2 the drops have fallen for a fifteenth of a second, not a twelfth, and their streaks are a fifth shorter. | largest difference 2.9e-8 | yes |
| FX-RAIN3D-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-020 frame 0: Drops keyed from 0 at frame 0 to 12 at frame 4: six on frame 2, FX-RAIN3D-007's six, the same drops. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-020 frame 2: Drops keyed from 0 at frame 0 to 12 at frame 4: six on frame 2, FX-RAIN3D-007's six, the same drops. | largest difference 2.9e-8 | yes |
| FX-RAIN3D-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-021 frame 0: Forty drops with everything: depth 96, wind -60 with variation 80, spread 30, size 3, opacity 70, ground level 90, seed 3. | largest difference 3.1e-8 | yes |
| FX-RAIN3D-021 frame 3: Forty drops with everything: depth 96, wind -60 with variation 80, spread 30, size 3, opacity 70, ground level 90, seed 3. | largest difference 2.9e-8 | yes |
| FX-RAIN3D-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-022 frame 1: FX-RAIN3D-012 with its colour written in capitals, #C8D8FF: the same. | largest difference 2.8e-8 | yes |
| FX-RAIN3D-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN3D-023 frame 0: Drops -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-023 frame 4: Drops -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-024 frame 0: Drops 50001, above 50000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-024 frame 4: Drops 50001, above 50000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-025 frame 0: Size 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-025 frame 4: Size 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-026 frame 0: Scene depth -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-026 frame 4: Scene depth -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-027 frame 0: Speed 20001, above 20000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-027 frame 4: Speed 20001, above 20000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-028 frame 0: Wind 5001, above 5000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-028 frame 4: Wind 5001, above 5000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-029 frame 0: Variation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-029 frame 4: Variation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-030 frame 0: Spread 91, above 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-030 frame 4: Spread 91, above 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-031 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-031 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-032 frame 0: Ground level 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-032 frame 4: Ground level 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-033 frame 0: Random seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-033 frame 4: Random seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-034 frame 0: Transfer mode written "screen". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-034 frame 4: Transfer mode written "screen". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-035 frame 0: Composite With Original written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-035 frame 4: Composite With Original written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-036 frame 0: A colour written "#c8d8f", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-036 frame 4: A colour written "#c8d8f", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN3D-037 frame 0: Speed keyed to 30000 at frame 4, above 20000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-037 frame 4: Speed keyed to 30000 at frame 4, above 20000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN3D-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| CC Rainfall never grows the layer's bounds | 0 | yes |
| a half-size draft preview halves Size, Scene Depth, Speed and Wind, the distances, and nothing else | CcRainfall { drops: 5000.0, size: 1.5, scene_depth: 2500.0, speed: 2000.0, wind: 0.0, wind_variation: 0.0, spread: 15.0, color: "#b4bec8", opacity: 25.0, transfer_mode: "lighten", composite_with_original: "on", ground_level: 100.0, random_seed: 0.0, clock: [0.0, 0.0] } | yes |

## Where the drops are

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAIN3D-017 (the rain alone) with a Gaussian Blur of 6 before it, which grows the layer: the drops stay where they were in the drawing (within 1e-6) | largest difference 0e0 | yes |
| FX-RAIN3D-002 with the layer's source 3 frames on: frame 1 is the same, since the rain falls by the composition's frame | largest difference 0e0 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rain3d_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain3d_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the clock the effect falls by is never saved | None | yes |
| fx_rain3d_022.json, its colour written in capitals, is saved in small letters, as Rain's is | "#c8d8ff" | yes |
| a file with no `random_seed` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a drop count that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a transfer mode that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Drops 50001 is refused with a sentence, and nothing changes | CC Rainfall's drops runs from 0 to 50000, and this is 50001. | yes |
| Size -1 is refused with a sentence, and nothing changes | CC Rainfall's size runs from 0 to 20, and this is -1. | yes |
| Spread 91 is refused with a sentence, and nothing changes | CC Rainfall's spread runs from 0 to 90, and this is 91. | yes |
| Transfer Mode "screen" is refused with a sentence, and nothing changes | CC Rainfall's transfer mode is "composite" or "lighten", and this is "screen". | yes |
| Composite With Original "maybe" is refused with a sentence, and nothing changes | CC Rainfall's composite with original is "off" or "on", and this is "maybe". | yes |
| Color "blue" is refused with a sentence, and nothing changes | CC Rainfall's colour is written #rrggbb, and this is "blue". | yes |
| Wind keyed to 6000 at frame 4 is refused with a sentence, and nothing changes | CC Rainfall's wind runs from -5000 to 5000, and this is 6000. | yes |
| Composite, alone, wind -300 with variation 40, Color #ffffff is taken | taken | yes |
| Drops keyed from 0 at frame 0 to 20000 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: rain over a street, in `verification/D-582 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, frame 2, the street with no effect: drawn without a warning | []; 0 pixels differ from the street | yes |
| 2_as_added.png, frame 2, as added: 5000 fine pale drops through a deep volume, Lighten, so nothing darker: drawn without a warning | []; 71074 pixels differ from the street | yes |
| 3_storm.png, frame 2, a storm: more, bigger, faster drops blown to the right: drawn without a warning | []; 111187 pixels differ from the street | yes |
| 4_near_drops.png, frame 2, a few big near drops, each its own tilt: drawn without a warning | []; 31699 pixels differ from the street | yes |
| 5_ground_level_70.png, frame 2, Ground Level 70: no rain on the road: drawn without a warning | []; 76722 pixels differ from the street | yes |
| 6_dark_rain_composite.png, frame 2, dark drops with Composite: they darken the street: drawn without a warning | []; 76065 pixels differ from the street | yes |
| 7_rain_alone.png, frame 2, the rain alone, without the street: drawn without a warning | []; 129600 pixels differ from the street | yes |

## Played frame after frame with the viewer's cache (D-528)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rain3d_021.json, frames 0 to 4 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it (the drawing is still, the rain is not) | 5 of 5 byte-identical; 122880 bytes kept for effects | yes |
| fx_rain3d_002.json, frames 0 to 4 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it (the drawing is still, the rain is not) | 5 of 5 byte-identical; 122880 bytes kept for effects | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rain3d_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rain3d_009.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rain3d_018.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rain3d_021.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

166 of 166 checks pass.
