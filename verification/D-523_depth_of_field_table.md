# B-403: Depth of Field

D-523: Depth of Field, after After Effects' Depth of Field (3D Channel): a render with a depth pass in its EXR file blurred more the further each pixel lies from the focal plane, by D-359's Lens Blur with a blur map. Every expected pixel is `Fixtures/depth_of_field/expected_depth_of_field.json`, written by `tools/depth_of_field_reference.py` before this code existed and printed in document 25 as FX-DOF-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DOF-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DOF-001 frame 0: As added: Focal Plane 0, so every depth is out of focus by its whole distance, fully; the whole picture at the full radius 10, edges repeated. | largest difference 2.9e-8 | yes |
| FX-DOF-001 frame 4: As added: Focal Plane 0, so every depth is out of focus by its whole distance, fully; the whole picture at the full radius 10, edges repeated. | largest difference 2.9e-8 | yes |
| FX-DOF-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-002 frame 0: Focal Plane 6, radius 3: the diagonal at depth 6 sharp, the near top left fully blurred, the far bottom right about half. | largest difference 2.7e-8 | yes |
| FX-DOF-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-003 frame 0: Focal Plane 6, Thickness 4: everything from 4 to 8 sharp. | largest difference 2.9e-8 | yes |
| FX-DOF-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-004 frame 0: Focal Plane 6, Bias 100: out of focus twice as fast. | largest difference 2.9e-8 | yes |
| FX-DOF-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-005 frame 0: Focal Plane 6, Bias 25: half as fast. | largest difference 2.7e-8 | yes |
| FX-DOF-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-006 frame 0: Bias 0: nothing leaves focus, the picture unchanged. | largest difference 0.0e0 | yes |
| FX-DOF-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-007 frame 0: Maximum Radius 0: the picture unchanged. | largest difference 0.0e0 | yes |
| FX-DOF-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-008 frame 0: Maximum Radius 2.5: three levels mixed. | largest difference 2.8e-8 | yes |
| FX-DOF-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-009 frame 0: Focal Plane keyed from 0 at frame 0 to 13 at frame 4: the sharp band moves from near to far. | largest difference 2.7e-8 | yes |
| FX-DOF-009 frame 2: Focal Plane keyed from 0 at frame 0 to 13 at frame 4: the sharp band moves from near to far. | largest difference 2.8e-8 | yes |
| FX-DOF-009 frame 4: Focal Plane keyed from 0 at frame 0 to 13 at frame 4: the sharp band moves from near to far. | largest difference 2.7e-8 | yes |
| FX-DOF-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-010 frame 0: Maximum Radius keyed from 0 at frame 0 to 4 at frame 4. | largest difference 0.0e0 | yes |
| FX-DOF-010 frame 2: Maximum Radius keyed from 0 at frame 0 to 4 at frame 4. | largest difference 2.8e-8 | yes |
| FX-DOF-010 frame 4: Maximum Radius keyed from 0 at frame 0 to 4 at frame 4. | largest difference 2.9e-8 | yes |
| FX-DOF-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-011 frame 0: The depth written negative, Focal Plane -6: FX-DOF-002. | largest difference 2.7e-8 | yes |
| FX-DOF-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-012 frame 0: Thickness 1,000,000: everything in focus, unchanged. | largest difference 0.0e0 | yes |
| FX-DOF-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-013 frame 0: Infinities and not-a-number in the depth: each read as D-348 reads it, all three fully out of focus; the warning every frame. | largest difference 2.7e-8 | yes |
| FX-DOF-013: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-DOF-014 frame 0: Blender's layout, the depth in ViewLayer.Depth.Z and the colour in ViewLayer.Combined: FX-DOF-002. | largest difference 2.7e-8 | yes |
| FX-DOF-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-015 frame 0: A file with no depth: unchanged, the warning every frame. | largest difference 0.0e0 | yes |
| FX-DOF-015: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DOF-016 frame 0: On a PNG drawing: unchanged, the warning. | largest difference 1.8e-7 | yes |
| FX-DOF-016: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DOF-017 frame 0: On a solid: unchanged, the warning. | largest difference 0.0e0 | yes |
| FX-DOF-017: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DOF-018 frame 0: On an adjustment layer above the EXR: no file of its own, unchanged, the warning. | largest difference 0.0e0 | yes |
| FX-DOF-018: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DOF-019 frame 0: The layer moved 2 right and 1 down: FX-DOF-002 moved. | largest difference 2.7e-8 | yes |
| FX-DOF-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-020 frame 0: An Exposure of +1 before it: the brighter picture blurred. | largest difference 4.8e-8 | yes |
| FX-DOF-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-021 frame 0: An Exposure of -1 after it. | largest difference 2.7e-8 | yes |
| FX-DOF-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-022 frame 0: Focal Plane 13, radius 3, Thickness 2: the far corner sharp, the near side fully blurred. | largest difference 2.8e-8 | yes |
| FX-DOF-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DOF-023 frame 0: Maximum Radius 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-023 frame 4: Maximum Radius 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DOF-024 frame 0: Maximum Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-024 frame 4: Maximum Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DOF-025 frame 0: Focal Bias 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-025 frame 4: Focal Bias 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DOF-026 frame 0: Focal Plane Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-026 frame 4: Focal Plane Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DOF-027 frame 0: Focal Plane keyed to 2,000,000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-027 frame 4: Focal Plane keyed to 2,000,000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DOF-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dof_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dof_002.json: the depth read for a frame is never saved | {"focal_bias":50,"focal_plane":6,"focal_plane_thickness":0,"maximum_radius":3} | yes |
| fx_dof_009.json: the depth read for a frame is never saved | {"focal_bias":50,"focal_plane":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":13}]},"focal_plane_thickness":0,"maximum_radius":3} | yes |
| fx_dof_023.json is refused in a sentence naming it | Depth of Field's maximum radius runs from 0 to 200, and this is 201. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Focal Bias 101 is refused with a sentence, and nothing changes | Depth of Field's focal bias runs from 0 to 100, and this is 101. | yes |
| Focal Plane 9, Thickness 2, is taken | taken | yes |
| Maximum Radius 12, Bias 80, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dof_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dof_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dof_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dof_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dof_001.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_001.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_001.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_001.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_002.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_002.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_002.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_002.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_003.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_003.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_003.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_004.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_004.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_004.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_004.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_005.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_005.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_005.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_005.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_006.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_006.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_006.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_006.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_007.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_007.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_008.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_008.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_008.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_009.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_009.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_009.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_009.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_010.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_010.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_010.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_010.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_011.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_011.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_012.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_012.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_012.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_013.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_dof_013.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_dof_013.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_dof_013.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_dof_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_014.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_014.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_015.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_015.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_015.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_015.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_016.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_016.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_016.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_016.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_017.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_017.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_018.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_018.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_018.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_018.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_dof_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_019.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_019.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_020.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_020.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_020.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_020.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_021.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_021.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_021.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_021.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_dof_022.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_022.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_022.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_dof_022.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the red ball (6), radius 8 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the red ball (6), radius 8 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the red ball (6), radius 8 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the red ball (6), radius 8 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball (9), radius 8 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball (9), radius 8 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball (9), radius 8 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball (9), radius 8 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the yellow ball (14), thickness 2, radius 12 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the yellow ball (14), thickness 2, radius 12 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the yellow ball (14), thickness 2, radius 12 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the yellow ball (14), thickness 2, radius 12 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 100, radius 20 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 100, radius 20 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 100, radius 20 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 100, radius 20 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 10, radius 30 frame 0, Full | largest 1 level(s), 2 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 10, radius 30 frame 4, Full | largest 1 level(s), 2 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 10, radius 30 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, focused on the blue ball, bias 10, radius 30 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (focal plane 0, radius 10) frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (focal plane 0, radius 10) frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (focal plane 0, radius 10) frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (focal plane 0, radius 10) frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: the sample render, in `verification/D-523 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the sample with no effect: a red ball near, a blue one further, a yellow one far, on a checked floor | warnings "" | yes |
| focus_red.png, Focal Plane 6, radius 8: the red ball sharp, the blue and yellow balls and the far floor soft | warnings "" | yes |
| focus_blue.png, Focal Plane 9, radius 8: the blue ball sharp, the red ball and the yellow ball soft | warnings "" | yes |
| focus_yellow.png, Focal Plane 14, Thickness 2, radius 12: the yellow ball sharp, the red ball very soft | warnings "" | yes |
| focus_blue_bias_100.png, Focal Plane 9, Bias 100, radius 20: only a thin band of the floor by the blue ball sharp | warnings "" | yes |
| focus_blue_thick.png, Focal Plane 9, Thickness 8, radius 8: everything from depth 5 to 13 sharp, the yellow ball a little soft | warnings "" | yes |

## Result

221 of 221 checks pass.
