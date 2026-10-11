# D-552: Pass Extract's alpha channel and UnMult

B-432, after fnord's EXtractoR (3D Channel, ProEXR manual 2.6.6): Pass Extract's channel names may be four, the fourth the alpha, and UnMult divides the three values by that alpha before the black and white points; the result is laid over the alpha. One or three names still give alpha 1, and UnMult is left out of the file while off, so every project saved before draws and saves as it did. Every expected pixel is `Fixtures/extractor_alpha/expected_extractor_alpha.json`, written by `tools/extractor_alpha_reference.py` before this code existed and printed in document 25 as FX-EXA-001 to 021. Tolerance 2e-5.

## FX-EXA-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EXA-001 frame 0: Red, green, blue and alpha named, Reflect.R, .G, .B, .A, UnMult left out (off), the points as added: the stored colour taken as straight and laid over its alpha, so the soft columns are darkened twice; clear where the alpha is 0 or below. | largest difference 1.5e-8 | yes |
| FX-EXA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-002 frame 0: The same, UnMult on: the render back as it was written, its colour divided by its alpha and multiplied again; the column whose alpha is past 1 is held at 1. | largest difference 5.6e-17 | yes |
| FX-EXA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-003 frame 0: UnMult on, Black 0.2, White 0.8: the straight colour stretched, then laid over the alpha. | largest difference 2.0e-8 | yes |
| FX-EXA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-004 frame 0: UnMult on, Invert on: the straight colour turned over, the alpha kept. | largest difference 3.0e-8 | yes |
| FX-EXA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-005 frame 0: UnMult on, Black 0.25, White 0.5, Clamp off: values past 1 kept, times the alpha. | largest difference 2.2e-16 | yes |
| FX-EXA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-006 frame 0: UnMult off, Black 0.25, White 0.5, Clamp off. | largest difference 6.0e-8 | yes |
| FX-EXA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-007 frame 0: UnMult on, Black and White both 0.5: a cut on the straight colour. | largest difference 0.0e0 | yes |
| FX-EXA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-008 frame 0: The alpha from another channel, Mask.Y (0 at the top to 1 at the bottom), UnMult on: the colour fades in down the picture, the top row clear. | largest difference 5.6e-17 | yes |
| FX-EXA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-009 frame 0: One channel three times and an alpha, Reflect.G x3, Reflect.A, UnMult on: a grey over the alpha. | largest difference 5.6e-17 | yes |
| FX-EXA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-010 frame 0: Three names, UnMult on: no alpha, so the picture is opaque and UnMult changes nothing (D-349's picture, as FX-EXA-011). | largest difference 0.0e0 | yes |
| FX-EXA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-011 frame 0: Three names, UnMult off: D-349's opaque picture. | largest difference 0.0e0 | yes |
| FX-EXA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-012 frame 0: One name, UnMult on: opaque, as UnMult off. | largest difference 0.0e0 | yes |
| FX-EXA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-013 frame 0: The data window only columns 2 to 6 of rows 1 to 4, UnMult on: outside it the alpha is 0, clear. | largest difference 0.0e0 | yes |
| FX-EXA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-014 frame 0: Not-a-number and infinities in the alpha, UnMult on: not-a-number (0) and minus infinity clear, plus infinity held at 1; the warning every frame. | largest difference 5.6e-17 | yes |
| FX-EXA-014: what opening it warns of, and what frame 4 warns of | [] and ["MEDIA_EXR_ADJUSTED"] | yes |
| FX-EXA-015 frame 0: The layer moved 2 right and 1 down: FX-EXA-002 moved. | largest difference 0.0e0 | yes |
| FX-EXA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-016 frame 0: White Point keyed from 1 at frame 0 to 0.5 at frame 4, UnMult on: frame 0 is FX-EXA-002. | largest difference 5.6e-17 | yes |
| FX-EXA-016 frame 4: White Point keyed from 1 at frame 0 to 0.5 at frame 4, UnMult on: frame 0 is FX-EXA-002. | largest difference 0.0e0 | yes |
| FX-EXA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-017 frame 0: An Exposure of -1 after it: the colour halved, the alpha kept, the picture being premultiplied. | largest difference 2.8e-17 | yes |
| FX-EXA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXA-018 frame 0: Four names, the alpha one the file lacks (Reflect.Alpha): nothing changes, the warning every frame. | largest difference 0.0e0 | yes |
| FX-EXA-018: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-EXA-019 frame 0: Five names: nothing changes, the warning every frame. | largest difference 0.0e0 | yes |
| FX-EXA-019: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_CHANNEL_MISSING"] | yes |
| FX-EXA-020 frame 0: UnMult written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-EXA-020 frame 4: UnMult written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-EXA-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXA-021 frame 0: UnMult written "On" (the words are lower case). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-EXA-021 frame 4: UnMult written "On" (the words are lower case). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-EXA-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_exa_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_exa_001.json: a project with no UnMult writes none | {"black_point":0,"channel":"Reflect.R, Reflect.G, Reflect.B, Reflect.A","clamp":"on","invert":"off","pass":"named","white_point":1} | yes |
| fx_exa_006.json: UnMult written off stays off | {"black_point":0.25,"channel":"Reflect.R, Reflect.G, Reflect.B, Reflect.A","clamp":"off","invert":"off","pass":"named","unmult":"off","white_point":0.5} | yes |
| fx_exa_002.json: UnMult on is saved on | {"black_point":0,"channel":"Reflect.R, Reflect.G, Reflect.B, Reflect.A","clamp":"on","invert":"off","pass":"named","unmult":"on","white_point":1} | yes |
| fx_exa_020.json is refused in a sentence naming it | Pass Extract's unmult is "off" or "on", and this is "yes". | yes |
| fx_exa_021.json is refused in a sentence naming it | Pass Extract's unmult is "off" or "on", and this is "On". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| UnMult "yes" is refused with a sentence, and nothing changes | Pass Extract's unmult is "off" or "on", and this is "yes". | yes |
| UnMult on, is taken | taken | yes |
| the alpha from Mask.Y, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_exa_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_exa_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_exa_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_exa_001.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_001.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_001.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_001.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_002.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_002.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_002.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_002.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_003.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_003.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_003.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_003.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_004.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_004.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_004.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_004.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_005.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_005.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_005.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_005.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_006.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_006.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_006.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_006.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_007.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_007.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_007.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_007.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_008.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_008.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_008.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_008.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_009.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_009.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_009.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_009.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_010.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_010.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_010.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_010.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_011.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_011.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_012.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_012.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_012.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_013.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_013.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_013.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_013.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_014.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_exa_014.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_exa_014.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_exa_014.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "MEDIA_EXR_ADJUSTED" | yes |
| fx_exa_015.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_015.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_015.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_015.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_016.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_016.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_016.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_016.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| fx_exa_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_exa_017.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_exa_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_exa_017.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 2 of 2 effect(s) on the card; warnings "" | yes |
| fx_exa_018.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_018.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_018.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_018.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_019.json frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| fx_exa_019.json frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_CHANNEL_MISSING" | yes |
| the sample, Combined with its alpha, UnMult off frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult off frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult off frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult off frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult on frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult on frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult on frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined with its alpha, UnMult on frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the normals over Combined's alpha, black -1, white 1 frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the normals over Combined's alpha, black -1, white 1 frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the normals over Combined's alpha, black -1, white 1 frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the normals over Combined's alpha, black -1, white 1 frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the mist over Combined's alpha, UnMult on frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the mist over Combined's alpha, UnMult on frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the mist over Combined's alpha, UnMult on frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, the mist over Combined's alpha, UnMult on frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined, black 0.2, white 0.8, UnMult on, three names frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined, black 0.2, white 0.8, UnMult on, three names frame 4, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined, black 0.2, white 0.8, UnMult on, three names frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the sample, Combined, black 0.2, white 0.8, UnMult on, three names frame 4, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 0 level(s) | yes |

## Pictures: the sample render in Blender's layout, in `verification/D-552 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the Blender-style file with no effect: three balls on a floor under a clear sky | warnings "" | yes |
| combined_unmult_off.png, Pass Extract, Combined's four channels, UnMult off: the same picture, its sky clear | warnings "" | yes |
| combined_unmult_off.png matches before.png within 1 level | largest 0 level(s), 0 pixel(s) differ | yes |
| combined_unmult_on.png, the same with UnMult on: the same picture again (this render's alpha is 0 or 1, so dividing changes nothing) | warnings "" | yes |
| combined_unmult_on.png matches before.png within 1 level | largest 0 level(s), 0 pixel(s) differ | yes |
| normals_over_alpha.png, Pass Extract, the normals with Combined's alpha, Black -1, White 1: the balls and floor in normal colours, the sky clear rather than grey | warnings "" | yes |
| normals_over_alpha.png is clear wherever before.png is | 22649 of 22649 sky pixels clear | yes |
| mist_over_alpha.png, Pass Extract, the mist with Combined's alpha: near dark, far light, the sky clear rather than white | warnings "" | yes |
| mist_over_alpha.png is clear wherever before.png is | 22649 of 22649 sky pixels clear | yes |

## Result

184 of 184 checks pass.
