# D-531: CC Environment

B-411, after CycoreFX's CC Environment (Perspective): an environment picture seen round a fixed eye, filling the layer's shape, in its own fake 3D. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/cc_environment_reference.py`: the ray through each pixel, turned by Roll, Tilt and Horizontal Pan, reads a Spherical Map (latitude and longitude), a Light Probe (angular) or a Vertical Cross; Filter Environment averages four rays a pixel on a Spherical Map; the colour is laid on the layer's alpha. The composition camera is not used. Every expected pixel is `Fixtures/cc_environment/expected_cc_environment.json`, written by the tool before this code existed and printed in document 25 as FX-ENV-001 to 030. Tolerance 2e-5.

## FX-ENV-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ENV-001 frame 0: As added: no environment named, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-ENV-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-002 frame 0: The spherical map, field of view 90: the middle of the map ahead, red rising to the right, green to the bottom; the clear corner stays clear. | largest difference 1.3e-7 | yes |
| FX-ENV-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-003 frame 0: FX-ENV-002 with Filter Environment off: one ray a pixel. | largest difference 1.1e-7 | yes |
| FX-ENV-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-004 frame 0: Horizontal Pan 90: the view turned a quarter right, so the map three quarters across lies ahead. | largest difference 1.1e-7 | yes |
| FX-ENV-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-005 frame 0: Horizontal Pan 180: looking behind, across the map's left and right edges, which meet with no seam. | largest difference 1.3e-7 | yes |
| FX-ENV-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-006 frame 0: Tilt 45: looking up, the map's upper rows. | largest difference 1.7e-7 | yes |
| FX-ENV-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-007 frame 0: Tilt 80: looking almost straight up, near the map's top row, which the rows are held to. | largest difference 1.7e-7 | yes |
| FX-ENV-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-008 frame 0: Roll 30: the view turned about the way it looks, the map's rows tilted. | largest difference 1.5e-7 | yes |
| FX-ENV-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-009 frame 0: Field of view 20, as added otherwise: a narrow view, the map enlarged. | largest difference 1.0e-7 | yes |
| FX-ENV-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-010 frame 0: The probe, field of view 90: the probe's middle ahead. | largest difference 1.3e-7 | yes |
| FX-ENV-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-011 frame 0: The probe, Horizontal Pan 150: looking nearly behind, toward the rim. | largest difference 1.4e-7 | yes |
| FX-ENV-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-012 frame 0: The probe with Filter Environment off: the filter is for Spherical only, so this is FX-ENV-010. | largest difference 1.3e-7 | yes |
| FX-ENV-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-013 frame 0: The vertical cross, field of view 90: the green face ahead, edges of red, blue, white and brown at the sides, top and bottom. | largest difference 9.8e-8 | yes |
| FX-ENV-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-014 frame 0: The cross, Horizontal Pan 90: the blue face ahead. | largest difference 1.7e-7 | yes |
| FX-ENV-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-015 frame 0: The cross, Tilt 90: the white face ahead. | largest difference 1.7e-7 | yes |
| FX-ENV-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-016 frame 0: The cross, Horizontal Pan 180: the yellow face behind, its darker pixel turned to the upper left, as the cross keeps it upside down. | largest difference 1.2e-7 | yes |
| FX-ENV-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-017 frame 0: Horizontal Pan keyed from 0 at frame 0 to 360 at frame 4: frame 0 is FX-ENV-002, frame 2 is FX-ENV-005, and frame 4 FX-ENV-002 again. | largest difference 1.3e-7 | yes |
| FX-ENV-017 frame 2: Horizontal Pan keyed from 0 at frame 0 to 360 at frame 4: frame 0 is FX-ENV-002, frame 2 is FX-ENV-005, and frame 4 FX-ENV-002 again. | largest difference 1.3e-7 | yes |
| FX-ENV-017 frame 4: Horizontal Pan keyed from 0 at frame 0 to 360 at frame 4: frame 0 is FX-ENV-002, frame 2 is FX-ENV-005, and frame 4 FX-ENV-002 again. | largest difference 1.3e-7 | yes |
| FX-ENV-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-018 frame 0: FX-ENV-002 on the holder moved 3 right and 1 down: the same, moved. | largest difference 1.3e-7 | yes |
| FX-ENV-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-019 frame 0: An environment layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-ENV-019 frame 4: An environment layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-ENV-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-ENV-020 frame 0: The environment with its in point at frame 4: before it there is no picture, so the layer is clear; at frame 4 it is FX-ENV-002. | largest difference 0.0e0 | yes |
| FX-ENV-020 frame 4: The environment with its in point at frame 4: before it there is no picture, so the layer is clear; at frame 4 it is FX-ENV-002. | largest difference 1.3e-7 | yes |
| FX-ENV-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-021 frame 0: Field of view 150 with Roll -20 and Tilt -30: wide, looking down, the map's lower rows. | largest difference 1.7e-7 | yes |
| FX-ENV-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENV-022 frame 0: Mapping "Spherical": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-022 frame 4: Mapping "Spherical": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-023 frame 0: Filter Environment written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-023 frame 4: Filter Environment written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-024 frame 0: Horizontal Pan 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-024 frame 4: Horizontal Pan 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-025 frame 0: Tilt -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-025 frame 4: Tilt -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-026 frame 0: Roll 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-026 frame 4: Roll 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-027 frame 0: Field of view 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-027 frame 4: Field of view 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-028 frame 0: Field of view 180, above 179. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-028 frame 4: Field of view 180, above 179. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-029 frame 0: An environment written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-029 frame 4: An environment written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENV-030 frame 0: Field of view keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-030 frame 4: Field of view keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-ENV-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview leaves every setting as it is, since none is a distance | CcEnvironment { environment: String("sphere"), mapping: "spherical", horizontal_pan: 0.0, tilt: 0.0, roll: 0.0, field_of_view: 90.0, filter_environment: "on", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_env_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_env_013.json is saved with its seven settings, the cross layer, mapping vertical_cross, field of view 90 | {"environment":"cross","field_of_view":90,"filter_environment":"on","horizontal_pan":0,"mapping":"vertical_cross","roll":0,"tilt":0} | yes |
| fx_env_029.json, its environment written as the number 3, is saved as the number 3 | 3 | yes |
| the environment picture the effect reads is never saved | None | yes |
| fx_env_022.json is refused in a sentence naming mapping | CC Environment's mapping is one of "spherical", "probe", "vertical_cross", and this is "Spherical". | yes |
| fx_env_023.json is refused in a sentence naming filter environment | CC Environment's filter environment is "on" or "off", and this is "yes". | yes |
| fx_env_024.json is refused in a sentence naming horizontal pan | CC Environment's horizontal pan runs from -3600 to 3600, and this is 3601. | yes |
| fx_env_025.json is refused in a sentence naming tilt | CC Environment's tilt runs from -3600 to 3600, and this is -3601. | yes |
| fx_env_026.json is refused in a sentence naming roll | CC Environment's roll runs from -3600 to 3600, and this is 3601. | yes |
| fx_env_027.json is refused in a sentence naming field of view | CC Environment's field of view runs from 1 to 179, and this is 0. | yes |
| fx_env_028.json is refused in a sentence naming field of view | CC Environment's field of view runs from 1 to 179, and this is 180. | yes |
| fx_env_029.json is refused in a sentence naming environment | CC Environment's environment is the name of a layer of this composition, and this is 3. | yes |
| a file with a CC Environment whose field of view is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Environment with no mapping is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Environment with no environment is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-531)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_env_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_env_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment spherical, the background layer-4 the map, field of view 90 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment probe, turned and tilted, unfiltered on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right on three layers, frame 0, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right on three layers, frame 239, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment vertical cross, looking up and to the right on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Environment Horizontal Pan keyed one whole turn on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-531 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_street_all_round.png: drawn without a warning, the street changed, the middle drawn | 126123 pixels changed; middle [125, 161, 187, 255]; warnings [] | yes |
| 2_turned_behind.png: drawn without a warning, the street changed, the middle drawn | 122895 pixels changed; middle [177, 138, 142, 255]; warnings [] | yes |
| 3_looking_up.png: drawn without a warning, the street changed, the middle drawn | 127892 pixels changed; middle [144, 182, 222, 255]; warnings [] | yes |
| 4_wide_looking_down.png: drawn without a warning, the street changed, the middle drawn | 111049 pixels changed; middle [92, 141, 171, 255]; warnings [] | yes |
| 5_cross_corner.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [40, 80, 220, 255]; warnings [] | yes |
| 6_probe.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [129, 131, 168, 255]; warnings [] | yes |

## Result

182 of 182 checks pass.
