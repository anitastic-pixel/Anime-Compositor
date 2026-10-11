# B-402: Cryptomatte

D-522: Cryptomatte, after After Effects' Cryptomatte (3D Channel): objects or materials picked by name from the Cryptomatte a 3D render saved in the layer's EXR file (Psyop's specification 1.2.0), shown as colours, as a matte, or keyed. Every expected pixel is `Fixtures/cryptomatte/expected_cryptomatte.json`, written by `tools/cryptomatte_reference.py` before this code existed and printed in document 25 as FX-CRYPTO-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CRYPTO-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CRYPTO-001 frame 0: Colors with the layer name empty: the first layer in the file's order, CryptoMaterial (sorted before CryptoObject): two flat colours, column 3 mixed three to one. | largest difference 2.6e-8 | yes |
| FX-CRYPTO-001 frame 4: Colors with the layer name empty: the first layer in the file's order, CryptoMaterial (sorted before CryptoObject): two flat colours, column 3 mixed three to one. | largest difference 2.6e-8 | yes |
| FX-CRYPTO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-002 frame 0: Colors of CryptoObject, nothing picked: four flat colours, the sky black, the edge pixels mixed by coverage; opaque. | largest difference 2.4e-8 | yes |
| FX-CRYPTO-002 frame 4: Colors of CryptoObject, nothing picked: four flat colours, the sky black, the edge pixels mixed by coverage; opaque. | largest difference 2.4e-8 | yes |
| FX-CRYPTO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-003 frame 0: Colors of CryptoObject, ball picked: the ball white. | largest difference 2.5e-8 | yes |
| FX-CRYPTO-003 frame 4: Colors of CryptoObject, ball picked: the ball white. | largest difference 2.5e-8 | yes |
| FX-CRYPTO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-004 frame 0: Matte of ball: white on the ball, 0.6 at (3, 2), 0.3 at (2, 5). | largest difference 0.0e0 | yes |
| FX-CRYPTO-004 frame 4: Matte of ball: white on the ball, 0.6 at (3, 2), 0.3 at (2, 5). | largest difference 0.0e0 | yes |
| FX-CRYPTO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-005 frame 0: Matte of ball and floor together: the coverages added. | largest difference 3.0e-8 | yes |
| FX-CRYPTO-005 frame 4: Matte of ball and floor together: the coverages added. | largest difference 3.0e-8 | yes |
| FX-CRYPTO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-006 frame 0: Keyed on ball: the picture kept on the ball, soft at its edge. | largest difference 2.4e-9 | yes |
| FX-CRYPTO-006 frame 4: Keyed on ball: the picture kept on the ball, soft at its edge. | largest difference 2.4e-9 | yes |
| FX-CRYPTO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-007 frame 0: Keyed on a name with a space and one beyond ASCII (UTF-8): big box and Kugel_ü, extra spaces and an empty name around them. | largest difference 1.5e-8 | yes |
| FX-CRYPTO-007 frame 4: Keyed on a name with a space and one beyond ASCII (UTF-8): big box and Kugel_ü, extra spaces and an empty name around them. | largest difference 1.5e-8 | yes |
| FX-CRYPTO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-008 frame 0: Keyed on a name the file lacks: all transparent. | largest difference 0.0e0 | yes |
| FX-CRYPTO-008 frame 4: Keyed on a name the file lacks: all transparent. | largest difference 0.0e0 | yes |
| FX-CRYPTO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-009 frame 0: Keyed with nothing picked: all transparent. | largest difference 0.0e0 | yes |
| FX-CRYPTO-009 frame 4: Keyed with nothing picked: all transparent. | largest difference 0.0e0 | yes |
| FX-CRYPTO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-010 frame 0: Matte of steel from CryptoMaterial: columns 0 to 2 white, column 3 at 0.75. | largest difference 0.0e0 | yes |
| FX-CRYPTO-010 frame 4: Matte of steel from CryptoMaterial: columns 0 to 2 white, column 3 at 0.75. | largest difference 0.0e0 | yes |
| FX-CRYPTO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-011 frame 0: The layer name in another case finds the same layer: as FX-CRYPTO-004. | largest difference 0.0e0 | yes |
| FX-CRYPTO-011 frame 4: The layer name in another case finds the same layer: as FX-CRYPTO-004. | largest difference 0.0e0 | yes |
| FX-CRYPTO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-012 frame 0: Blender's layout, found by the last dotted word: one level only, so the third and fourth objects in a pixel are not there. | largest difference 0.0e0 | yes |
| FX-CRYPTO-012 frame 4: Blender's layout, found by the last dotted word: one level only, so the third and fourth objects in a pixel are not there. | largest difference 0.0e0 | yes |
| FX-CRYPTO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-013 frame 0: Blender's layout by its whole name, keyed on Kugel_ü. | largest difference 0.0e0 | yes |
| FX-CRYPTO-013 frame 4: Blender's layout by its whole name, keyed on Kugel_ü. | largest difference 0.0e0 | yes |
| FX-CRYPTO-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-014 frame 0: A layer name the file lacks: unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-014 frame 4: A layer name the file lacks: unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-014: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-CRYPTO-015 frame 0: An EXR with no Cryptomatte: unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-015 frame 4: An EXR with no Cryptomatte: unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-015: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-CRYPTO-016 frame 0: On a PNG layer: unchanged, said each frame. | largest difference 1.8e-7 | yes |
| FX-CRYPTO-016 frame 4: On a PNG layer: unchanged, said each frame. | largest difference 1.8e-7 | yes |
| FX-CRYPTO-016: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-CRYPTO-017 frame 0: On a solid: unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-017 frame 4: On a solid: unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-017: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-CRYPTO-018 frame 0: One rank (no blue or alpha), a lower-case type name, a coverage that is not a number (read as 0, said each frame): rock on the even columns. | largest difference 0.0e0 | yes |
| FX-CRYPTO-018 frame 4: One rank (no blue or alpha), a lower-case type name, a coverage that is not a number (read as 0, said each frame): rock on the even columns. | largest difference 0.0e0 | yes |
| FX-CRYPTO-018: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-CRYPTO-019 frame 0: The layer moved 2 right and 1 down: the matte moves with it. | largest difference 2.4e-9 | yes |
| FX-CRYPTO-019 frame 4: The layer moved 2 right and 1 down: the matte moves with it. | largest difference 2.4e-9 | yes |
| FX-CRYPTO-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-020 frame 0: After an Exposure of -1: the darker picture keyed. | largest difference 1.5e-10 | yes |
| FX-CRYPTO-020 frame 4: After an Exposure of -1: the darker picture keyed. | largest difference 1.5e-10 | yes |
| FX-CRYPTO-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-021 frame 0: On an adjustment layer above: no file of its own, so unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-021 frame 4: On an adjustment layer above: no file of its own, so unchanged, said each frame. | largest difference 0.0e0 | yes |
| FX-CRYPTO-021: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-CRYPTO-022 frame 0: Colors with the floor and Kugel_ü picked, then an Exposure of +1. | largest difference 5.9e-8 | yes |
| FX-CRYPTO-022 frame 4: Colors with the floor and Kugel_ü picked, then an Exposure of +1. | largest difference 5.9e-8 | yes |
| FX-CRYPTO-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRYPTO-023 frame 0: Cryptomatte's output written "preview". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-CRYPTO-023 frame 4: Cryptomatte's output written "preview". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-CRYPTO-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The name hash

| Check | The build's answer | Matches |
| --- | --- | --- |
| "torus" hashes to 4053562365, the specification's own example | 4053562365 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_crypto_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crypto_004.json: the matte read for a frame is never saved, the names kept as typed | {"layer":"CryptoObject","output":"matte","selection":"ball"} | yes |
| fx_crypto_007.json: the matte read for a frame is never saved, the names kept as typed | {"layer":"CryptoObject","output":"keyed","selection":"  big box ,, Kugel_ü "} | yes |
| fx_crypto_023.json is refused in a sentence naming it | Cryptomatte's output is "colors", "matte" or "keyed", and this is "preview". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| output "preview" is refused with a sentence, and nothing changes | Cryptomatte's output is "colors", "matte" or "keyed", and this is "preview". | yes |
| CryptoMaterial, steel, keyed, is taken | taken | yes |
| the colours, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_crypto_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crypto_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crypto_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crypto_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_crypto_001.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_001.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_001.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_001.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_002.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_002.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_002.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_002.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_003.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_003.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_003.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_004.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_004.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_004.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_004.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_005.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_005.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_005.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_005.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_006.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_006.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_006.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_006.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_007.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_007.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_008.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_008.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_008.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_009.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_009.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_009.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_009.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_010.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_010.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_010.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_010.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_011.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_011.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_012.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_012.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_012.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_013.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_013.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_013.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_013.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_014.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_014.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_015.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_015.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_015.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_015.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_016.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_016.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_016.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_016.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_017.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_017.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_018.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_crypto_018.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_crypto_018.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_crypto_018.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_crypto_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_019.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_019.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_crypto_020.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_020.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_020.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_020.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_021.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_021.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_021.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_021.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 0 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_crypto_022.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_022.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_022.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_crypto_022.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Colors, the first layer frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors, the first layer frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors, the first layer frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors, the first layer frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors of CryptoObject, the floor picked frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors of CryptoObject, the floor picked frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors of CryptoObject, the floor picked frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Colors of CryptoObject, the floor picked frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of the yellow ball frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of the yellow ball frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of the yellow ball frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of the yellow ball frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the red and blue balls frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the red and blue balls frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the red and blue balls frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the red and blue balls frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the material paint frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the material paint frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the material paint frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the material paint frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the sky, after a Gaussian Blur of 6 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the sky, after a Gaussian Blur of 6 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the sky, after a Gaussian Blur of 6 frame 0, Draft | largest 1 level(s), 8 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Keyed on the sky, after a Gaussian Blur of 6 frame 4, Draft | largest 1 level(s), 8 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| the sample, Matte of gold and tile, the layer named in lower case frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of gold and tile, the layer named in lower case frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of gold and tile, the layer named in lower case frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Matte of gold and tile, the layer named in lower case frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: the sample render, in `verification/D-522 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the sample with no effect: a red, a blue and a yellow ball over a two-tone floor, under a blue sky | warnings "" | yes |
| colors_objects.png, Colors of CryptoObject: each ball, the floor and the sky one flat colour, their edges mixed | warnings "" | yes |
| colors_materials.png, Colors of CryptoMaterial: the red and blue balls one colour (they share the material paint) | warnings "" | yes |
| colors_floor_picked.png, Colors of CryptoObject with floor picked: the floor white, the rest in colours | warnings "" | yes |
| matte_yellow_ball.png, Matte of yellow ball: that ball white with a soft edge, the rest black | warnings "" | yes |
| keyed_red_blue.png, Keyed on red ball, blue ball: those two balls alone, everything else clear | warnings "" | yes |
| keyed_material_gold.png, Keyed on the material gold: the yellow ball alone | warnings "" | yes |

## Result

228 of 228 checks pass.
