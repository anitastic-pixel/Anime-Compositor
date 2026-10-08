# B-229: object and material ids, named channels and Blender's files

D-349 (EFFECTS.md P0-5, part 2): the object and material ids and any named channel a render saves in an EXR file, read by ID Key (after After Effects' ID Matte) and Pass Extract, and Blender's multilayer files taken as a picture. Every expected pixel is `Fixtures/depth_channel/expected_depth_channel.json`, written by `tools/depth_channel_reference.py` before this code existed and printed in document 25 as FX-DEPTH-037 to 064. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DEPTH-037 to 064 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DEPTH-037 frame 0: ID Key as added on the Blender-style file: object id 0, the top-left block of 3 by 3 kept, the rest clear; the colour is ViewLayer.Combined. | largest difference 0.0e0 | yes |
| FX-DEPTH-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-038 frame 0: Object id 4: the middle block of the lower row kept. | largest difference 0.0e0 | yes |
| FX-DEPTH-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-039 frame 0: Object id 4, Invert on: everything but that block. | largest difference 0.0e0 | yes |
| FX-DEPTH-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-040 frame 0: Material id 2: its diagonals, with the pixel at 2.4 (5, 2) and not the one at 2.6 (5, 3). | largest difference 0.0e0 | yes |
| FX-DEPTH-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-041 frame 0: Object id 4, Feather 1.5: the block's edge soft, its edge pixels held at the picture's border. | largest difference 4.9e-8 | yes |
| FX-DEPTH-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-042 frame 0: Object id 4, Feather 1.5, Invert on: the soft hole. | largest difference 7.2e-8 | yes |
| FX-DEPTH-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-043 frame 0: A bare channel named ObjectID (0 to 3 by column): id 3, the columns 3 and 7. | largest difference 0.0e0 | yes |
| FX-DEPTH-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-044 frame 0: Material from materialID.R (7 or 8 by row), not its G (100) or B (200): id 8, the odd rows. | largest difference 0.0e0 | yes |
| FX-DEPTH-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-045 frame 0: ID Key on a file with no ids: nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-045: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-046 frame 0: ID Key on a PNG drawing: nothing changes, the warning. | largest difference 1.8e-7 | yes |
| FX-DEPTH-046: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-047 frame 0: ID keyed from 0 at frame 0 to 5 at frame 4: frame 0 is FX-DEPTH-037, frame 4 the bottom-right block. | largest difference 0.0e0 | yes |
| FX-DEPTH-047 frame 4: ID keyed from 0 at frame 0 to 5 at frame 4: frame 0 is FX-DEPTH-037, frame 4 the bottom-right block. | largest difference 0.0e0 | yes |
| FX-DEPTH-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-048 frame 0: An Exposure of +1 before ID Key, Feather 1: the matte from the ids, the colour the brighter one. | largest difference 1.3e-7 | yes |
| FX-DEPTH-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-049 frame 0: The layer moved 2 right and 1 down: FX-DEPTH-038 moved. | largest difference 0.0e0 | yes |
| FX-DEPTH-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-050 frame 0: Pass Extract of the object id, Black 0, White 5: the blocks as six greys. | largest difference 2.4e-8 | yes |
| FX-DEPTH-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-051 frame 0: Pass Extract of the material id, Black 0, White 3, Clamp off. | largest difference 2.0e-8 | yes |
| FX-DEPTH-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-052 frame 0: Pass Extract of the channel named ViewLayer.Mist.Z: Blender's mist as grey. | largest difference 0.0e0 | yes |
| FX-DEPTH-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-053 frame 0: Three named channels, ViewLayer.Normal.X, .Y, .Z, Black -1, White 1: the same picture as FX-DEPTH-054. | largest difference 0.0e0 | yes |
| FX-DEPTH-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-054 frame 0: The normals of the Blender-style file found by their layer's last word, Normal. | largest difference 0.0e0 | yes |
| FX-DEPTH-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-055 frame 0: Its depth, ViewLayer.Depth.Z, found the same way: FX-DEPTH-002's ramp. | largest difference 2.0e-8 | yes |
| FX-DEPTH-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-056 frame 0: Depth Key on it, Depth 6: FX-DEPTH-023, the colour from ViewLayer.Combined. | largest difference 0.0e0 | yes |
| FX-DEPTH-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEPTH-057 frame 0: A named channel the file lacks (Mist.Z; the name must be whole and exact): nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-057: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-058 frame 0: Two names, neither one nor three: nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-058: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-059 frame 0: No name at all: nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-059: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-060 frame 0: Pass Extract of the object id from a file with none: nothing changes, the warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-060: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-DEPTH-061 frame 0: ID Key's aux_channel written "uv". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-061 frame 4: ID Key's aux_channel written "uv". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-062 frame 0: ID Key's id -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-062 frame 4: ID Key's id -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-063 frame 0: ID Key's feather 101, past 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-063 frame 4: ID Key's feather 101, past 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-063: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEPTH-064 frame 0: ID Key's invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-064 frame 4: ID Key's invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-DEPTH-064: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_depth_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_depth_038.json: the ids or channel read for a frame are never saved | {"aux_channel":"object_id","feather":0,"id":4,"invert":"off"} | yes |
| fx_depth_052.json: the ids or channel read for a frame are never saved | {"black_point":0,"channel":"ViewLayer.Mist.Z","clamp":"on","invert":"off","pass":"named","white_point":1} | yes |
| fx_depth_050.json: a Pass Extract with no channel names writes none | {"black_point":0,"clamp":"on","invert":"off","pass":"object_id","white_point":5} | yes |
| fx_depth_061.json is refused in a sentence naming it | ID Key's channel is "object_id" or "material_id", and this is "uv". | yes |
| fx_depth_062.json is refused in a sentence naming it | ID Key's id runs from 0 to 1000000, and this is -1. | yes |
| fx_depth_063.json is refused in a sentence naming it | ID Key's feather runs from 0 to 100, and this is 101. | yes |
| fx_depth_064.json is refused in a sentence naming it | ID Key's invert is "off" or "on", and this is "yes". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| channel "uv" is refused with a sentence, and nothing changes | ID Key's channel is "object_id" or "material_id", and this is "uv". | yes |
| feather 101 is refused with a sentence, and nothing changes | ID Key's feather runs from 0 to 100, and this is 101. | yes |
| the material ids, is taken | taken | yes |
| id 2, feather 1.5, invert on, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_depth_038.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_depth_041.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_depth_049.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_depth_053.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_depth_037.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_037.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_037.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_037.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_038.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_038.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_038.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_038.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_039.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_039.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_039.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_039.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_040.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_040.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_040.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_040.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_041.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_041.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_041.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_041.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_042.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_042.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_042.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_042.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_043.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_043.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_043.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_043.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_044.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_044.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_044.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_044.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_045.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_045.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_045.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_045.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_046.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_046.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_046.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_046.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_047.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_047.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_047.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_047.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_048.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_048.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_048.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_048.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_depth_049.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_049.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_049.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_049.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_050.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_050.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_050.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_050.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_051.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_051.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_051.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_051.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_052.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_052.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_052.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_052.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_053.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_053.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_053.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_053.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_054.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_054.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_054.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_054.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_055.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_055.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_055.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_055.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_056.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_056.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_056.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_056.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_depth_057.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_057.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_057.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_057.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_058.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_058.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_058.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_058.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_059.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_059.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_059.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_059.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_060.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_060.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_060.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_depth_060.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| the sample, ID Key, object 1 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 1 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 1 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 1 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 4 (the floor), feather 3 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 4 (the floor), feather 3 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 4 (the floor), feather 3 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 4 (the floor), feather 3 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 2, inverted, feather 5 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 2, inverted, feather 5 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 2, inverted, feather 5 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 2, inverted, feather 5 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, material 3, feather 1.5 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, material 3, feather 1.5 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, material 3, feather 1.5 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, material 3, feather 1.5 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 3, feather 3, after a Gaussian Blur of 6 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 3, feather 3, after a Gaussian Blur of 6 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 3, feather 3, after a Gaussian Blur of 6 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, ID Key, object 3, feather 3, after a Gaussian Blur of 6 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, object ids 0 to 4 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, object ids 0 to 4 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, object ids 0 to 4 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, object ids 0 to 4 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, material ids 0 to 4 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, material ids 0 to 4 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, material ids 0 to 4 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, material ids 0 to 4 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, ViewLayer.Mist.Z frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, ViewLayer.Mist.Z frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, ViewLayer.Mist.Z frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, ViewLayer.Mist.Z frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Pass Extract, depth frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 0 level(s) | yes |

## Pictures: the sample render in Blender's layout, in `verification/D-349 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the Blender-style file with no effect: the same three balls on a floor as D-348's before.png, its colour taken from ViewLayer.Combined | warnings "" | yes |
| object_ids.png, Pass Extract, Object ID, Black 0, White 4: the sky black, the three balls three greys, the floor white | warnings "" | yes |
| material_ids.png, Pass Extract, Material ID, Black 0, White 4: the floor's squares two greys, the red and blue balls a very light grey, the yellow ball white | warnings "" | yes |
| mist.png, Pass Extract, Named Channel ViewLayer.Mist.Z: near dark, far light, the sky white | warnings "" | yes |
| key_object_2.png, ID Key, Object ID 2: one ball alone, everything else clear | warnings "" | yes |
| key_object_2_inverted_soft.png, ID Key, Object ID 2, Feather 3, Invert: that ball cut out of the picture, its edge soft | warnings "" | yes |
| key_material_3.png, ID Key, Material ID 3: the red and blue balls together (they share a material), the rest clear | warnings "" | yes |

## Result

245 of 245 checks pass.
