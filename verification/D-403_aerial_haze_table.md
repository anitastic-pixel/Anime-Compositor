# D-403: Aerial Haze

B-282: `core.aerial_haze`, PLUGINS.md's pick #8: each pixel moved toward a haze colour by an amount, evenly or by the brightness of a matte layer (D-189's layer setting, on the card through `map_texture`). Every expected pixel is `Fixtures/aerial_haze/expected_aerial_haze.json`, written by `tools/aerial_haze_reference.py` before this code existed and printed in document 25 as FX-HAZE-001 to 025. Tolerance 2e-5.

## FX-HAZE-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HAZE-001 frame 0: As added: a pale sky blue, #b4c8dc, amount 30, no layer named: every pixel that shows three tenths of the way to the sky blue, evenly; the clear corner stays clear. | largest difference 1.2e-7 | yes |
| FX-HAZE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-002 frame 0: Amount 0: nothing changes. | largest difference 1.5e-7 | yes |
| FX-HAZE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-003 frame 0: Amount 100: every pixel that shows is the sky blue at its own covering. | largest difference 2.7e-8 | yes |
| FX-HAZE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-004 frame 0: White haze, #ffffff, amount 50: every channel halfway to white, the darkest parts lifted most, as raising Levels' output black. | largest difference 1.1e-7 | yes |
| FX-HAZE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-005 frame 0: FX-HAZE-001 with its colour written in capitals, #B4C8DC: the same. | largest difference 1.2e-7 | yes |
| FX-HAZE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-006 frame 0: The grey ramp as the matte, amount 100: column 0 (black) untouched, column 15 (white) the sky blue, the columns between hazed by the ramp's brightness. The ramp layer is moved, scaled and switched off, none of which the matte reads. | largest difference 1.6e-7 | yes |
| FX-HAZE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-007 frame 0: The ramp, amount 60: as FX-HAZE-006 at six tenths. | largest difference 1.6e-7 | yes |
| FX-HAZE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-008 frame 0: A white solid as the matte, amount 30: the same as no matte, FX-HAZE-001. | largest difference 1.2e-7 | yes |
| FX-HAZE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-009 frame 0: A black solid as the matte, amount 100: nothing changes. | largest difference 1.5e-7 | yes |
| FX-HAZE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-010 frame 0: A white matte clear at the top and more covering down each row, amount 100: a clear matte reads as black, so row 0 is untouched and the haze grows down the rows. | largest difference 1.5e-7 | yes |
| FX-HAZE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-011 frame 0: A 4 by 2 checker of white and black, centred, amount 100: outside it the matte is clear, black, so only its four white pixels are hazed. | largest difference 1.5e-7 | yes |
| FX-HAZE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-012 frame 0: The checker tiled, amount 100: hazed and untouched in a checker over the whole layer. | largest difference 1.5e-7 | yes |
| FX-HAZE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-013 frame 0: The checker stretched to 16 by 10, amount 100: soft squares of haze. | largest difference 1.5e-7 | yes |
| FX-HAZE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-014 frame 0: The holder names itself, amount 100: its cream squares, the brightest, are hazed most, its red and blue ones less, and the clear corner stays clear. | largest difference 9.3e-8 | yes |
| FX-HAZE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-015 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, no matte: frame 0 the drawing, frame 2 halfway, frame 4 as FX-HAZE-003. | largest difference 1.5e-7 | yes |
| FX-HAZE-015 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, no matte: frame 0 the drawing, frame 2 halfway, frame 4 as FX-HAZE-003. | largest difference 9.6e-8 | yes |
| FX-HAZE-015 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, no matte: frame 0 the drawing, frame 2 halfway, frame 4 as FX-HAZE-003. | largest difference 2.7e-8 | yes |
| FX-HAZE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-016 frame 0: A layer that is not in the composition, `gone`, amount 100: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-HAZE-016 frame 4: A layer that is not in the composition, `gone`, amount 100: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-HAZE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-HAZE-017 frame 0: A white solid whose in point is frame 4 as the matte, amount 100: before it the matte is empty, which reads as black, so at frame 0 nothing changes; at frame 4 it is FX-HAZE-003. | largest difference 1.5e-7 | yes |
| FX-HAZE-017 frame 4: A white solid whose in point is frame 4 as the matte, amount 100: before it the matte is empty, which reads as black, so at frame 0 nothing changes; at frame 4 it is FX-HAZE-003. | largest difference 2.7e-8 | yes |
| FX-HAZE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-018 frame 0: FX-HAZE-007 on the holder moved 2 right and 1 down: the same picture moved, since the matte lies on the layer. | largest difference 1.6e-7 | yes |
| FX-HAZE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-019 frame 0: The effect on an adjustment layer above the holder, the ramp as its matte, amount 60: the matte lies on the frame, which here is the holder's own rectangle, so this is FX-HAZE-007. | largest difference 1.6e-7 | yes |
| FX-HAZE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAZE-020 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-020 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAZE-021 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-021 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAZE-022 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-022 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAZE-023 frame 0: A haze colour written "#12345", five digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-023 frame 4: A haze colour written "#12345", five digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAZE-024 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-024 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAZE-025 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-025 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAZE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with an Aerial Haze whose matte is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_haze_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_haze_005.json, its colour written in capitals, is saved in small letters, as Gradient Map's are | "#b4c8dc" | yes |
| fx_haze_011.json is saved with its four settings, the layer as written | {"amount":100,"fit":"center","haze_color":"#b4c8dc","layer":"card"} | yes |
| fx_haze_020.json is refused in a sentence naming amount | Aerial Haze's amount runs from 0 to 100, and this is 101. | yes |
| fx_haze_021.json is refused in a sentence naming amount | Aerial Haze's amount runs from 0 to 100, and this is -1. | yes |
| fx_haze_023.json is refused in a sentence naming Haze Color | Aerial Haze's Haze Color is written #rrggbb, and this is "#12345". | yes |
| fx_haze_024.json is refused in a sentence naming fit | Aerial Haze's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| fx_haze_025.json is refused in a sentence naming matte layer | Aerial Haze's matte layer is the name of a layer of this composition, and this is 3. | yes |
| a file with an Aerial Haze whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Aerial Haze without its layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Aerial Haze whose colour is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Aerial Haze's amount runs from 0 to 100, and this is 101. | yes |
| Haze Color #abc is refused with a sentence, and nothing changes | Aerial Haze's Haze Color is written #rrggbb, and this is "#abc". | yes |
| fit fill is refused with a sentence, and nothing changes | Aerial Haze's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Aerial Haze's amount runs from 0 to 100, and this is 150. | yes |
| the ramp as the matte at 60 is taken | taken | yes |
| amount keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_haze_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_haze_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_haze_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_haze_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Aerial Haze as added (even): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073599 pixels changed | yes |
| the reference shot, Aerial Haze as added (even) on three layers, frame 0, Full | largest difference 1 of 255, 1624 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze as added (even) on three layers, frame 100, Full | largest difference 1 of 255, 1120 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze as added (even) on three layers, frame 239, Full | largest difference 1 of 255, 1297 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze as added (even) on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze as added (even) on three layers, frame 100, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze as added (even) on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 160801 pixels changed | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80 on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80 on three layers, frame 0, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80 on three layers, frame 100, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze through layer 4 as the matte at 80 on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 160801 pixels changed | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50 on three layers, frame 0, Full | largest difference 1 of 255, 314 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50 on three layers, frame 100, Full | largest difference 1 of 255, 91 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50 on three layers, frame 239, Full | largest difference 1 of 255, 317 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50 on three layers, frame 0, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50 on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Aerial Haze white haze through layer 4 centred at 50 on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-403 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| amount 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (pale sky blue, 30, even): the whole street paler and bluer, its blacks lifted and its contrast lower; draws cleanly | [], darkest pixel 370 against 160, red spread 127 against 215 | yes |
| 3_amount_70.png, amount 70: a thick fog, nearer the sky blue than 2 is; draws cleanly | [], red spread 51 against 2's 127 | yes |
| 4_matte_top_far.png, amount 80 through a matte white at the top and black at the bottom: the top of the street deep in haze, the bottom row exactly as it was; draws cleanly | [], change in the top row 39360, in the bottom row 0 | yes |
| a matte layer not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

162 of 162 checks pass.
