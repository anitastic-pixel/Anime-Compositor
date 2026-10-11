# B-405: IDentifier

D-525: IDentifier, after After Effects' IDentifier (3D Channel, fnord's ProEXR): the object or material ids in a render's EXR file shown as colours, as a luma or alpha matte of one id, or raw. Every expected pixel is `Fixtures/identifier/expected_identifier.json`, written by `tools/identifier_reference.py` before this code existed and printed in document 25 as FX-IDENT-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-IDENT-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-IDENT-001 frame 0: As added: the object ids 0 to 5 as six colours, one to each block of three by three, opaque. | largest difference 2.3e-8 | yes |
| FX-IDENT-001 frame 4: As added: the object ids 0 to 5 as six colours, one to each block of three by three, opaque. | largest difference 2.3e-8 | yes |
| FX-IDENT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-002 frame 0: The material ids as colours: 1 to 3 on the diagonals, 2.4 coloured as 2 and 2.6 as 3. | largest difference 2.3e-8 | yes |
| FX-IDENT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-003 frame 0: Luma matte of object 4: the bottom-middle block white, the rest black. | largest difference 0.0e0 | yes |
| FX-IDENT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-004 frame 0: Alpha matte of object 4: the scene kept in the bottom-middle block, transparent elsewhere. | largest difference 0.0e0 | yes |
| FX-IDENT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-005 frame 0: Raw object ids: greys whose 8-bit values are 0 to 5. | largest difference 3.7e-11 | yes |
| FX-IDENT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-006 frame 0: Raw material ids: 2.4 and 2.6 as they are, not rounded. | largest difference 2.5e-11 | yes |
| FX-IDENT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-007 frame 0: Luma matte of material 2: the 2 diagonal and the 2.4 pixel, not the 2.6. | largest difference 0.0e0 | yes |
| FX-IDENT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-008 frame 0: The id keyed from 0 at frame 0 to 4 at frame 4, luma matte: block 0, 2, then 4. | largest difference 0.0e0 | yes |
| FX-IDENT-008 frame 2: The id keyed from 0 at frame 0 to 4 at frame 4, luma matte: block 0, 2, then 4. | largest difference 0.0e0 | yes |
| FX-IDENT-008 frame 4: The id keyed from 0 at frame 0 to 4 at frame 4, luma matte: block 0, 2, then 4. | largest difference 0.0e0 | yes |
| FX-IDENT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-009 frame 0: A bare ObjectID channel, 0 to 3 by column: four colours. | largest difference 2.3e-8 | yes |
| FX-IDENT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-010 frame 0: materialID.R (7 or 8 by row), not its G (100) or B (200): luma matte of 8, the odd rows. | largest difference 0.0e0 | yes |
| FX-IDENT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-011 frame 0: Ids from 0 to 350 by column, -3, an infinity and not-a-number, raw: white past 255, black at -3, the infinity white and the not-a-number black; the warning every frame. | largest difference 2.6e-8 | yes |
| FX-IDENT-011: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-IDENT-012 frame 0: The same ids as colours: each its own colour, the infinity too. | largest difference 2.3e-8 | yes |
| FX-IDENT-012: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-IDENT-013 frame 0: A file with no ids: unchanged, the warning every frame. | largest difference 0.0e0 | yes |
| FX-IDENT-013: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-IDENT-014 frame 0: On a PNG drawing: unchanged, the warning. | largest difference 1.8e-7 | yes |
| FX-IDENT-014: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-IDENT-015 frame 0: On an adjustment layer above the EXR: no file of its own, unchanged, the warning. | largest difference 0.0e0 | yes |
| FX-IDENT-015: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-IDENT-016 frame 0: The layer moved 2 right and 1 down: FX-IDENT-001 moved. | largest difference 2.3e-8 | yes |
| FX-IDENT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-017 frame 0: An Exposure of +1 before it, alpha matte of object 0: the brighter scene kept in the top-left block. | largest difference 0.0e0 | yes |
| FX-IDENT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-018 frame 0: An Exposure of -1 after it, colours: each colour halved. | largest difference 1.2e-8 | yes |
| FX-IDENT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-019 frame 0: Alpha matte of object 9, which no pixel has: all transparent. | largest difference 0.0e0 | yes |
| FX-IDENT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IDENT-020 frame 0: Output written "matte". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-020 frame 4: Output written "matte". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IDENT-021 frame 0: Aux channel written "depth". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-021 frame 4: Aux channel written "depth". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IDENT-022 frame 0: Id -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-022 frame 4: Id -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IDENT-023 frame 0: Id keyed to 1,000,001 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-023 frame 4: Id keyed to 1,000,001 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-IDENT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ident_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ident_001.json: the ids read for a frame are never saved | {"aux_channel":"object_id","id":0,"output":"colors"} | yes |
| fx_ident_004.json: the ids read for a frame are never saved | {"aux_channel":"object_id","id":4,"output":"alpha_matte"} | yes |
| fx_ident_020.json is refused in a sentence naming it | IDentifier's output is "colors", "luma_matte", "alpha_matte" or "raw", and this is "matte". | yes |
| fx_ident_021.json is refused in a sentence naming it | IDentifier's channel is "object_id" or "material_id", and this is "depth". | yes |
| fx_ident_022.json is refused in a sentence naming it | IDentifier's id runs from 0 to 1000000, and this is -1. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Output "matte" is refused with a sentence, and nothing changes | IDentifier's output is "colors", "luma_matte", "alpha_matte" or "raw", and this is "matte". | yes |
| Luma matte of 4, is taken | taken | yes |
| The material ids, raw, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ident_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ident_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ident_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ident_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ident_001.json frame 0, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_001.json frame 4, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_001.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_001.json frame 4, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_002.json frame 0, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_002.json frame 4, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_002.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_002.json frame 4, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_003.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_003.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_003.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_004.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_004.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_004.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_004.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_005.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_005.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_005.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_005.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_006.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_006.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_006.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_006.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_007.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_007.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_008.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_008.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_008.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_009.json frame 0, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_009.json frame 4, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_009.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_009.json frame 4, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_010.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_010.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_010.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_010.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_011.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_011.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_012.json frame 0, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_012.json frame 4, Full | largest 1 level(s), 48 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_012.json frame 0, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_012.json frame 4, Draft | largest 1 level(s), 4 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_ident_013.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_013.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_013.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_013.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_014.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_014.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_015.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_015.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_015.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_015.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_ident_016.json frame 0, Full | largest 1 level(s), 30 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_016.json frame 4, Full | largest 1 level(s), 30 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_016.json frame 0, Draft | largest 1 level(s), 1 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_016.json frame 4, Draft | largest 1 level(s), 1 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_017.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_017.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_018.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_018.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_018.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_018.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_ident_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_019.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_ident_019.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (object id colours) frame 0, Full | largest 1 level(s), 57600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (object id colours) frame 4, Full | largest 1 level(s), 57600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (object id colours) frame 0, Draft | largest 1 level(s), 3600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, as added (object id colours) frame 4, Draft | largest 1 level(s), 3600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, material id colours frame 0, Full | largest 1 level(s), 57600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, material id colours frame 4, Full | largest 1 level(s), 57600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, material id colours frame 0, Draft | largest 1 level(s), 3600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, material id colours frame 4, Draft | largest 1 level(s), 3600 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, luma matte of object 2 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, luma matte of object 2 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, luma matte of object 2 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, luma matte of object 2 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, alpha matte of material 3 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, alpha matte of material 3 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, alpha matte of material 3 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, alpha matte of material 3 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, raw object ids frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, raw object ids frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, raw object ids frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, raw object ids frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: the sample render, in `verification/D-525 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the sample with no effect: a red ball near, a blue one further, a yellow one far, on a checked floor | warnings "" | yes |
| object_colors.png, as added, the object ids as colours: the sky, the floor and each ball a flat colour of its own, every ball's outline sharp | warnings "" | yes |
| material_colors.png, the material ids as colours: the sky one colour, the floor's two squares in two colours, the red and blue balls sharing one colour, the yellow ball another | warnings "" | yes |
| luma_matte_object_2.png, luma matte of object 2: the blue ball white, everything else black | warnings "" | yes |
| alpha_matte_material_3.png, alpha matte of material 3: the red and blue balls kept in their own colours, everything else transparent | warnings "" | yes |
| raw_objects.png, raw object ids: nearly black, the sky darkest (0) and the floor a little lighter (4); the 8-bit values read the ids | warnings "" | yes |

## Result

192 of 192 checks pass.
