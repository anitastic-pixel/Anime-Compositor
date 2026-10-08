# B-228: depth and normals from an EXR file

D-348 (EFFECTS.md P0-5): the depth and normals a render saves in an EXR file, read by Pass Extract (after After Effects' 3D Channel Extract) and Depth Key (after Depth Matte). Every expected pixel is `Fixtures/depth_channel/expected_depth_channel.json`, written by `tools/depth_channel_reference.py` before this code existed and printed in document 25 as FX-DEPTH-001 to 036. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DEPTH-001 to 036 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DEPTH-001 frame 0: Pass Extract as added: the depth, Black Point 0, White Point 1, Clamp on: every depth past 1 is white, the nearest pixel (0.5) mid grey; the picture is opaque, its clear corner too. | largest difference 0.0e0 | yes |
| FX-DEPTH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-002 frame 0: Black 0, White 12: the depth as a grey ramp, near dark, far light. | largest difference 2.0e-8 | yes |
| FX-DEPTH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-003 frame 0: Black 12, White 0: the other way round, near light. | largest difference 2.0e-8 | yes |
| FX-DEPTH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-004 frame 0: Black 0, White 12, Invert on: FX-DEPTH-002 turned over, as 003. | largest difference 2.0e-8 | yes |
| FX-DEPTH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-005 frame 0: Black 10, White 14, Clamp off, the other names' file (depth 1 to 20): values below 0 and above 1 are kept. | largest difference 0.0e0 | yes |
| FX-DEPTH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-006 frame 0: Black and White both 6: a cut, white from depth 6 on, black nearer; the pixel exactly at 6 is white. | largest difference 0.0e0 | yes |
| FX-DEPTH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-007 frame 0: The normals, Black -1, White 1: each direction as a colour, x red, y green, z blue, mid grey for 0. | largest difference 0.0e0 | yes |
| FX-DEPTH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-008 frame 0: The normals as added, Black 0, White 1, Clamp on: what points left or down is held at 0. | largest difference 0.0e0 | yes |
| FX-DEPTH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-009 frame 0: The data window only columns 2 to 6 of rows 1 to 4: outside it the depth is 0, black, as the colour there is clear. | largest difference 2.0e-8 | yes |
| FX-DEPTH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-010 frame 0: Depth named depth.Z: found. | largest difference 2.8e-8 | yes |
| FX-DEPTH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-011 frame 0: Normals named normal.R, normal.G, normal.B: found. | largest difference 0.0e0 | yes |
| FX-DEPTH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-012 frame 0: Infinities and not-a-number in the depth: far infinity white, near infinity black, not-a-number read as 0, black; the warning every frame. | largest difference 2.0e-8 | yes |
| FX-DEPTH-012: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-DEPTH-013 frame 0: A file with no depth: nothing changes, with the warning every frame. | largest difference 0.0e0 | yes |
| FX-DEPTH-013: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-014 frame 0: Asked for normals from a file with depth only: nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-014: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-015 frame 0: On a PNG drawing: no passes, nothing changes, the warning. | largest difference 1.8e-7 | yes |
| FX-DEPTH-015: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-016 frame 0: On a solid: no file, nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-016: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-017 frame 0: On an adjustment layer above the EXR: no file of its own, nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-017: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-018 frame 0: An Exposure of +1 before it is not seen: FX-DEPTH-002. | largest difference 2.0e-8 | yes |
| FX-DEPTH-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-019 frame 0: An Exposure of -1 after it darkens the ramp. | largest difference 9.9e-9 | yes |
| FX-DEPTH-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-020 frame 0: The layer moved 2 right and 1 down: FX-DEPTH-002 moved. | largest difference 2.0e-8 | yes |
| FX-DEPTH-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-021 frame 0: White Point keyed from 1 at frame 0 to 12 at frame 4: frame 0 is FX-DEPTH-001, frame 4 FX-DEPTH-002. | largest difference 0.0e0 | yes |
| FX-DEPTH-021 frame 4: White Point keyed from 1 at frame 0 to 12 at frame 4: frame 0 is FX-DEPTH-001, frame 4 FX-DEPTH-002. | largest difference 2.0e-8 | yes |
| FX-DEPTH-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-022 frame 0: Depth Key as added: Depth 0, no feather: every depth is at least 0, so nothing changes. | largest difference 0.0e0 | yes |
| FX-DEPTH-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-023 frame 0: Depth 6: everything nearer than 6 taken out; the pixel exactly at 6 kept. | largest difference 0.0e0 | yes |
| FX-DEPTH-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-024 frame 0: Depth 6, Invert on: everything from 6 on taken out instead. | largest difference 0.0e0 | yes |
| FX-DEPTH-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-025 frame 0: Depth 6, Feather 4: a soft edge from 4 to 8. | largest difference 0.0e0 | yes |
| FX-DEPTH-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-026 frame 0: Depth 6, Feather 4, Invert on: the soft edge the other way. | largest difference 1.4e-17 | yes |
| FX-DEPTH-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-027 frame 0: Depth 6 on the file with infinities: far infinity kept, near infinity and not-a-number (0) out; the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-027: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-DEPTH-028 frame 0: Depth 5 on the small data window: inside it the near pixels go; outside it the depth is 0, out, but already clear. | largest difference 0.0e0 | yes |
| FX-DEPTH-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-029 frame 0: An Exposure of +1 before it: the kept pixels are the brighter ones. | largest difference 0.0e0 | yes |
| FX-DEPTH-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-030 frame 0: Depth Key on a file with no depth: nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-030: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-031 frame 0: Depth keyed from 0 at frame 0 to 12.5 at frame 4: frame 4 has only the far corner left. | largest difference 0.0e0 | yes |
| FX-DEPTH-031 frame 4: Depth keyed from 0 at frame 0 to 12.5 at frame 4: frame 4 has only the far corner left. | largest difference 0.0e0 | yes |
| FX-DEPTH-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-032 frame 0: Pass Extract's pass written "uv". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-032 frame 4: Pass Extract's pass written "uv". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-033 frame 0: Pass Extract's clamp written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-033 frame 4: Pass Extract's clamp written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-034 frame 0: Black Point 2,000,000, past 1,000,000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-034 frame 4: Black Point 2,000,000, past 1,000,000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-035 frame 0: Depth Key's feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-035 frame 4: Depth Key's feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-036 frame 0: Depth Key's invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-036 frame 4: Depth Key's invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_depth_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_001.json: the pass read for a frame is never saved | {"black_point":0,"clamp":"on","invert":"off","pass":"depth","white_point":1} | yes |
| fx_depth_022.json: the pass read for a frame is never saved | {"depth":0,"feather":0,"invert":"off"} | yes |
| fx_depth_032.json is refused in a sentence naming it | Pass Extract's pass is "depth" or "normals", and this is "uv". | yes |
| fx_depth_033.json is refused in a sentence naming it | Pass Extract's clamp is "off" or "on", and this is "yes". | yes |
| fx_depth_034.json is refused in a sentence naming it | Pass Extract's black point runs from -1000000 to 1000000, and this is 2000000. | yes |
| fx_depth_035.json is refused in a sentence naming it | Depth Key's feather runs from 0 to 1000000, and this is -1. | yes |
| fx_depth_036.json is refused in a sentence naming it | Depth Key's invert is "off" or "on", and this is "yes". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| pass "uv" is refused with a sentence, and nothing changes | Pass Extract's pass is "depth" or "normals", and this is "uv". | yes |
| white point 2,000,000 is refused with a sentence, and nothing changes | Pass Extract's white point runs from -1000000 to 1000000, and this is 2000000. | yes |
| the normals, is taken | taken | yes |
| black point -1, invert on, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_depth_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_depth_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_depth_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_depth_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## A blur before Pass Extract grows the drawing; the pass still lies where it was

| Check | The build's answer | Matches |
| --- | --- | --- |
| the sample's depth, with and without a Gaussian Blur of 6 before it | byte-identical, warnings "" and "" | yes |
| the sample's normals, with and without a Gaussian Blur of 6 before it | byte-identical, warnings "" and "" | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_depth_001.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_001.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_001.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_001.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_002.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_002.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_002.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_002.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_003.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_003.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_003.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_004.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_004.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_004.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_004.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_005.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_005.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_005.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_005.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_006.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_006.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_006.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_006.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_007.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_007.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_008.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_008.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_008.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_009.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_009.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_009.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_009.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_010.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_010.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_010.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_010.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_011.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_011.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_012.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_012.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_012.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_013.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_013.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_013.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_013.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_014.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_014.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_015.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_015.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_015.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_015.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_016.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_016.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_016.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_016.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_017.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_017.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_018.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_018.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_018.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_018.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_019.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_019.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_020.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_020.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_020.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_020.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_021.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_021.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_021.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_021.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_022.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_022.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_022.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_022.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_023.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_023.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_023.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_023.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_024.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_024.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_024.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_024.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_025.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_025.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_025.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_025.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_026.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_026.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_026.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_026.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_027.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_027.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_027.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_027.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_depth_028.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_028.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_028.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_028.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_029.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_029.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_029.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_029.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_030.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_030.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_030.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_030.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_031.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_031.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_031.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_031.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 2 to 16 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth 16 to 2, clamp off frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals -1 to 1 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, normals inverted, a cut at 0 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth after a Gaussian Blur of 6 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 1, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 2, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 3, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 1, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 2, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 3, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 10, inverted, no feather frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 0, Full | largest 1 level(s), 1 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 1, Full | largest 1 level(s), 1 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 2, Full | largest 1 level(s), 1 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 3, Full | largest 1 level(s), 1 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 4, Full | largest 1 level(s), 1 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 0, Draft | largest 1 level(s), 22 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 1, Draft | largest 1 level(s), 22 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 2, Draft | largest 1 level(s), 22 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 3, Draft | largest 1 level(s), 22 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Depth Key at 8, feather 3, after a Gaussian Blur of 6 frame 4, Draft | largest 1 level(s), 22 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: the sample render, in `verification/D-348 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the render's colour, as it is drawn without an effect: three balls on a floor, the sky clear (white in a viewer that shows clear as white) | warnings "" | yes |
| depth.png, Pass Extract, depth, Black Point 2, White Point 16: near dark, far light, the sky white | warnings "" | yes |
| normals.png, Pass Extract, normals, Black -1, White 1: what faces right reddish, up greenish, the camera bluish; the floor all one pale green, the sky (no surface) mid grey | warnings "" | yes |
| key_near_out.png, Depth Key at 8, Feather 3: the red ball in front and the near floor taken out (clear), softly | warnings "" | yes |
| key_far_out.png, Depth Key at 8, Feather 3, Invert: only the red ball and the near floor left | warnings "" | yes |

## Result

343 of 343 checks pass.
