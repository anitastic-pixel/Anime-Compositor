# D-501: Difference Matte

B-381: `core.difference_matte`, after After Effects' Difference Matte (Keying): each pixel compared with a clean plate, another layer of the composition named by D-189's layer setting (on the card through `map_texture`), and keyed out where they match; the rule and the settings when added are ours. Every expected pixel is `Fixtures/difference_matte/expected_difference_matte.json`, written by `tools/difference_matte_reference.py` before this code existed and printed in document 25 as FX-DMATTE-001 to 026. Tolerance 2e-5.

## FX-DMATTE-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DMATTE-001 frame 0: As added: no layer named, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-DMATTE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-002 frame 0: The plate as the difference layer, tolerance 15: every pixel the shot shares with the plate keyed out, the green subject kept whole, the clear corner clear. | largest difference 7.1e-8 | yes |
| FX-DMATTE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-003 frame 0: FX-DMATTE-002 as the matte: the subject white, the rest black, the clear corner clear. | largest difference 0.0e0 | yes |
| FX-DMATTE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-004 frame 0: A plate 10 levels darker, tolerance 3: every difference is 3.92 per cent, above 3, so nothing is keyed and nothing changes. | largest difference 1.5e-7 | yes |
| FX-DMATTE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-005 frame 0: The darker plate, tolerance 5: 3.92 is within 5, so the background is keyed and only the subject stays. | largest difference 7.1e-8 | yes |
| FX-DMATTE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-006 frame 0: The darker plate, tolerance 2, softness 4: the background kept at (3.92 - 2) / 4, about 0.48, the subject whole. | largest difference 1.0e-6 | yes |
| FX-DMATTE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-007 frame 0: A grainy plate, 8 levels lighter and darker by turns, tolerance 2, no blur: every difference is 3.14 per cent, so nothing is keyed. | largest difference 1.5e-7 | yes |
| FX-DMATTE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-008 frame 0: The grainy plate, tolerance 2, softness 4, blurred at 1 before the difference: the blur smooths the grain away, so most of the background is keyed, while the subject and its edge stay. | largest difference 1.6e-6 | yes |
| FX-DMATTE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-009 frame 0: The plate, tolerance 5, softness 10, blurred at 1: the subject kept whole and a soft halo of the background around it, the rest keyed. | largest difference 5.4e-7 | yes |
| FX-DMATTE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-010 frame 0: The plate's middle 8 by 6 centred: it lies exactly where it was cut, so the background inside it is keyed; outside it the plate is clear, which differs from everything that shows, so that is kept. | largest difference 1.5e-7 | yes |
| FX-DMATTE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-011 frame 0: The middle 8 by 6 stretched to 16 by 10: it no longer lies on the shot, so far less is keyed. | largest difference 1.5e-7 | yes |
| FX-DMATTE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-012 frame 0: The holder names itself, tolerance 0: every pixel matches itself, so everything is keyed. | largest difference 0.0e0 | yes |
| FX-DMATTE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-013 frame 0: A layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-DMATTE-013 frame 4: A layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-DMATTE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-DMATTE-014 frame 0: The plate with its in point at frame 4: before it the plate is empty, clear, which differs from everything that shows, so at frame 0 nothing changes; at frame 4 it is FX-DMATTE-002. | largest difference 1.5e-7 | yes |
| FX-DMATTE-014 frame 4: The plate with its in point at frame 4: before it the plate is empty, clear, which differs from everything that shows, so at frame 0 nothing changes; at frame 4 it is FX-DMATTE-002. | largest difference 7.1e-8 | yes |
| FX-DMATTE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-015 frame 0: The darker plate, tolerance keyed from 0 at frame 0 to 10 at frame 4: frame 0 unchanged, frames 2 and 4 the background keyed. | largest difference 1.5e-7 | yes |
| FX-DMATTE-015 frame 2: The darker plate, tolerance keyed from 0 at frame 0 to 10 at frame 4: frame 0 unchanged, frames 2 and 4 the background keyed. | largest difference 7.1e-8 | yes |
| FX-DMATTE-015 frame 4: The darker plate, tolerance keyed from 0 at frame 0 to 10 at frame 4: frame 0 unchanged, frames 2 and 4 the background keyed. | largest difference 7.1e-8 | yes |
| FX-DMATTE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-016 frame 0: FX-DMATTE-006 on the holder moved 2 right and 1 down: the same picture moved, since the plate lies on the layer. | largest difference 1.0e-6 | yes |
| FX-DMATTE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-017 frame 0: The effect on an adjustment layer above the holder, the plate as its difference layer: the plate lies on the frame, which here is the holder's own rectangle, so this is FX-DMATTE-002. | largest difference 7.1e-8 | yes |
| FX-DMATTE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMATTE-018 frame 0: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-018 frame 4: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-019 frame 0: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-019 frame 4: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-020 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-020 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-021 frame 0: Blur 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-021 frame 4: Blur 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-022 frame 0: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-022 frame 4: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-023 frame 0: A view written "alpha". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-023 frame 4: A view written "alpha". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-024 frame 0: A fit written "tile", which After Effects' sizes do not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-024 frame 4: A fit written "tile", which After Effects' sizes do not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-025 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-025 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMATTE-026 frame 0: Tolerance keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-026 frame 4: Tolerance keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMATTE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the shot, each with a Difference Matte whose difference layer is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dmatte_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmatte_010.json is saved with its six settings, the layer as written | {"blur_before_difference":0,"fit":"center","layer":"patch","matching_softness":0,"matching_tolerance":15,"view":"final"} | yes |
| fx_dmatte_018.json is refused in a sentence naming matching tolerance | Difference Matte's matching tolerance runs from 0 to 100, and this is 101. | yes |
| fx_dmatte_019.json is refused in a sentence naming matching tolerance | Difference Matte's matching tolerance runs from 0 to 100, and this is -1. | yes |
| fx_dmatte_020.json is refused in a sentence naming matching softness | Difference Matte's matching softness runs from 0 to 100, and this is 101. | yes |
| fx_dmatte_021.json is refused in a sentence naming blur before difference | Difference Matte's blur before difference runs from 0 to 100, and this is 101. | yes |
| fx_dmatte_022.json is refused in a sentence naming blur before difference | Difference Matte's blur before difference runs from 0 to 100, and this is -1. | yes |
| fx_dmatte_023.json is refused in a sentence naming view | Difference Matte's view is "final" or "matte", and this is "alpha". | yes |
| fx_dmatte_024.json is refused in a sentence naming fit | Difference Matte's fit is "center" or "stretch", and this is "tile". | yes |
| fx_dmatte_025.json is refused in a sentence naming difference layer | Difference Matte's difference layer is the name of a layer of this composition, and this is 3. | yes |
| a file with a Difference Matte whose tolerance is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Difference Matte without its layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Difference Matte whose view is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tolerance 101 is refused with a sentence, and nothing changes | Difference Matte's matching tolerance runs from 0 to 100, and this is 101. | yes |
| view alpha is refused with a sentence, and nothing changes | Difference Matte's view is "final" or "matte", and this is "alpha". | yes |
| fit tile is refused with a sentence, and nothing changes | Difference Matte's fit is "center" or "stretch", and this is "tile". | yes |
| tolerance keyed to 150 is refused with a sentence, and nothing changes | Difference Matte's matching tolerance runs from 0 to 100, and this is 150. | yes |
| the plate as the difference layer at tolerance 5 is taken | taken | yes |
| tolerance keyed from 0 to 10 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dmatte_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_014.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmatte_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dmatte_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_dmatte_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Difference Matte against layer 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 10464 pixels changed | yes |
| the reference shot, Difference Matte against layer 4 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4 stretched, as the matte, tolerance 8, softness 12 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 9799 pixels changed | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Difference Matte against layer 4, tolerance 5, softness 10, blurred at 2 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-501 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_keyed.png, against the plate as added (tolerance 15): only the disc is left, every pixel of the street keyed out; draws cleanly | [], 14296 of the disc's 14296 pixels whole, 0 pixels of the street showing | yes |
| 4_matte.png, as the matte only: the disc white, the street black; draws cleanly | [], 14296 of 14296 disc pixels white, 115304 of 115304 street pixels black | yes |
| 6 and 7: against a plate 6 levels lighter (about 2.4 per cent), tolerance 1 keys almost none of the street and tolerance 5 keys all of it, the disc whole in both; draws cleanly | [] [], street pixels showing 115304 at 1 and 0 at 5, disc pixels whole 14296 and 14296 of 14296 | yes |
| a difference layer not in the composition: the shot exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

173 of 173 checks pass.
