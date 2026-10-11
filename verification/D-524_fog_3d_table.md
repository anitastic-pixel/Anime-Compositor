# B-404: Fog 3D

D-524: Fog 3D, after After Effects' Fog 3D (3D Channel): a render with a depth pass in its EXR file laid over with fog that thickens with distance, thinned where a grey gradient layer is dark. Every expected pixel is `Fixtures/fog_3d/expected_fog_3d.json`, written by `tools/fog_3d_reference.py` before this code existed and printed in document 25 as FX-FOG-001 to 030. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-FOG-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FOG-001 frame 0: As added: white fog from depth 0 to 10, density 50, opacity 100, Foggy Background on: the near top left lightly fogged, everything past depth 10 white; the clear pixel filled with white fog, the half-covering one half. | largest difference 4.5e-8 | yes |
| FX-FOG-001 frame 4: As added: white fog from depth 0 to 10, density 50, opacity 100, Foggy Background on: the near top left lightly fogged, everything past depth 10 white; the clear pixel filled with white fog, the half-covering one half. | largest difference 4.5e-8 | yes |
| FX-FOG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-002 frame 0: Density 0: the fog grows evenly from depth 0 to 10. | largest difference 3.6e-8 | yes |
| FX-FOG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-003 frame 0: Density 100: thick almost at once. | largest difference 4.7e-8 | yes |
| FX-FOG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-004 frame 0: Start 4, End 12, density 0: nothing nearer than 4 fogged, the far corner past 12 all fog. | largest difference 5.6e-17 | yes |
| FX-FOG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-005 frame 0: Start and End both 6: a cut, no fog nearer than 6, all fog from 6 on. | largest difference 0.0e0 | yes |
| FX-FOG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-006 frame 0: Opacity 50: half of FX-FOG-002's fog, the clear pixel half fog. | largest difference 3.6e-8 | yes |
| FX-FOG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-007 frame 0: Opacity 0: nothing changes. | largest difference 0.0e0 | yes |
| FX-FOG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-008 frame 0: Foggy Background off: the clear pixel stays clear and the half-covering one keeps its covering. | largest difference 4.5e-8 | yes |
| FX-FOG-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-009 frame 0: Fog colour #4060a0, a dull blue, density 0. | largest difference 1.8e-8 | yes |
| FX-FOG-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-010 frame 0: The ramp as the gradient, contribution 100, density 0: no fog in column 0 (black), the full fog in column 7 (white). | largest difference 1.0e-7 | yes |
| FX-FOG-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-011 frame 0: The ramp, contribution 50: column 0 half fogged. | largest difference 6.5e-8 | yes |
| FX-FOG-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-012 frame 0: The ramp, contribution 0: FX-FOG-002. | largest difference 3.6e-8 | yes |
| FX-FOG-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-013 frame 0: A gradient layer named that is not in the composition: nothing changes, with D-189's warning. | largest difference 0.0e0 | yes |
| FX-FOG-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-FOG-014 frame 0: End keyed from 1 at frame 0 to 20 at frame 4: the fog draws back. | largest difference 2.3e-8 | yes |
| FX-FOG-014 frame 2: End keyed from 1 at frame 0 to 20 at frame 4: the fog draws back. | largest difference 4.6e-8 | yes |
| FX-FOG-014 frame 4: End keyed from 1 at frame 0 to 20 at frame 4: the fog draws back. | largest difference 4.5e-8 | yes |
| FX-FOG-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-015 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4. | largest difference 0.0e0 | yes |
| FX-FOG-015 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4. | largest difference 3.7e-8 | yes |
| FX-FOG-015 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4. | largest difference 4.5e-8 | yes |
| FX-FOG-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-016 frame 0: Start 10, End 2: the other way round, fog thickest nearest. | largest difference 1.1e-16 | yes |
| FX-FOG-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-017 frame 0: Infinities and not-a-number in the depth: each read as D-348 reads it; the warning every frame. | largest difference 4.5e-8 | yes |
| FX-FOG-017: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-FOG-018 frame 0: Blender's layout, the depth in ViewLayer.Depth.Z: FX-FOG-001. | largest difference 4.5e-8 | yes |
| FX-FOG-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-019 frame 0: A file with no depth: unchanged, the warning every frame. | largest difference 0.0e0 | yes |
| FX-FOG-019: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-FOG-020 frame 0: On a PNG drawing: unchanged, the warning. | largest difference 1.8e-7 | yes |
| FX-FOG-020: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-FOG-021 frame 0: On an adjustment layer above the EXR: no file of its own, unchanged, the warning. | largest difference 0.0e0 | yes |
| FX-FOG-021: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-FOG-022 frame 0: The layer moved 2 right and 1 down, the ramp as the gradient: FX-FOG-010 moved, the ramp with it. | largest difference 4.9e-8 | yes |
| FX-FOG-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-023 frame 0: An Exposure of +1 before it. | largest difference 5.7e-8 | yes |
| FX-FOG-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-024 frame 0: An Exposure of -1 after it. | largest difference 2.2e-8 | yes |
| FX-FOG-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOG-025 frame 0: Fog Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-025 frame 4: Fog Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOG-026 frame 0: Scattering Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-026 frame 4: Scattering Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOG-027 frame 0: Foggy Background written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-027 frame 4: Foggy Background written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOG-028 frame 0: Fog colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-028 frame 4: Fog colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOG-029 frame 0: Fog End Depth keyed to 2,000,000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-029 frame 4: Fog End Depth keyed to 2,000,000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOG-030 frame 0: Layer Contribution 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-030 frame 4: Layer Contribution 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FOG-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fog_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fog_001.json: the depth and gradient read for a frame are never saved | {"fog_color":"#ffffff","fog_end_depth":10,"fog_opacity":100,"fog_start_depth":0,"foggy_background":"on","gradient_layer":"","layer_contribution":100,"scattering_density":50} | yes |
| fx_fog_010.json: the depth and gradient read for a frame are never saved | {"fog_color":"#ffffff","fog_end_depth":10,"fog_opacity":100,"fog_start_depth":0,"foggy_background":"on","gradient_layer":"grad","layer_contribution":100,"scattering_density":0} | yes |
| fx_fog_025.json is refused in a sentence naming it | Fog 3D's fog opacity runs from 0 to 100, and this is 101. | yes |
| fx_fog_027.json is refused in a sentence naming it | Fog 3D's foggy background is "on" or "off", and this is "yes". | yes |
| fx_fog_028.json is refused in a sentence naming it | Fog 3D's Fog Color is written #rrggbb, and this is "white". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Layer Contribution 101 is refused with a sentence, and nothing changes | Fog 3D's layer contribution runs from 0 to 100, and this is 101. | yes |
| Start 4, End 12, is taken | taken | yes |
| Colour #4060a0, Foggy Background off, the ramp at 50, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fog_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fog_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fog_014.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fog_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fog_001.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_001.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_001.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_001.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_002.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_002.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_002.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_002.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_003.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_003.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_003.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_004.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_004.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_004.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_004.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_005.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_005.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_005.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_005.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_006.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_006.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_006.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_006.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_007.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_007.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_008.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_008.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_008.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_009.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_009.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_009.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_009.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_010.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_010.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_010.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_010.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_011.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_011.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_012.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_012.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_012.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_013.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_LAYER_MISSING" | yes |
| fx_fog_013.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_LAYER_MISSING" | yes |
| fx_fog_013.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_LAYER_MISSING" | yes |
| fx_fog_013.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_LAYER_MISSING" | yes |
| fx_fog_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_014.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_014.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_015.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_015.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_015.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_015.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_016.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_016.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_016.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_016.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_fog_017.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_fog_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_fog_017.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_fog_018.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_018.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_018.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_018.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_019.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_019.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_020.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_020.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_020.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_020.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_021.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_021.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_021.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_021.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_fog_022.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_022.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_022.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_022.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_fog_023.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_023.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_023.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_023.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_024.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_024.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_024.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_fog_024.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, as added (white, 0 to 10, density 50) frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (white, 0 to 10, density 50) frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (white, 0 to 10, density 50) frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (white, 0 to 10, density 50) frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, blue-grey from 5 to 15, density 0 frame 0, Full | largest 1 level(s), 1 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, blue-grey from 5 to 15, density 0 frame 4, Full | largest 1 level(s), 1 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, blue-grey from 5 to 15, density 0 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, blue-grey from 5 to 15, density 0 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, white from 4 to 30, density 100 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, white from 4 to 30, density 100 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, white from 4 to 30, density 100 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, white from 4 to 30, density 100 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, warm from 6 to 14, opacity 60 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, warm from 6 to 14, opacity 60 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, warm from 6 to 14, opacity 60 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, warm from 6 to 14, opacity 60 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, from 14 to 6, the other way round, background off frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, from 14 to 6, the other way round, background off frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, from 14 to 6, the other way round, background off frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, from 14 to 6, the other way round, background off frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: the sample render, in `verification/D-524 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the sample with no effect: a red ball near, a blue one further, a yellow one far, on a checked floor | warnings "" | yes |
| as_added.png, as added, white fog from depth 0 to 10 at density 50: thick almost at once, only the near floor's checks clear, the red ball a faint pink, the blue ball barely seen, the rest white | warnings "" | yes |
| blue_grey_5_to_15.png, blue-grey fog from 5 to 15, even: the red ball almost clear, the yellow ball deep in fog, the horizon blue-grey | warnings "" | yes |
| dense_4_to_30.png, white fog from 4 to 30 at density 100: thick soon after depth 4, every ball faded | warnings "" | yes |
| warm_opacity_60.png, warm fog from 6 to 14 at opacity 60: the far part tinted, never fully hidden | warnings "" | yes |
| reversed.png, white fog from 14 to 6, the other way round: the near floor and the red ball white, the blue ball pale, the yellow ball, the far floor and the sky clear | warnings "" | yes |

## Result

237 of 237 checks pass.
