# D-383: Levels with a set for each channel, and Levels (Individual Controls)

B-262, the owner's "Both names, one engine": a set of input black, input white, gamma, output black and output white for RGB, red, green, blue and alpha; Levels' Channel menu chooses the set its controls show, Levels (Individual Controls) shows all 25. Each channel's own set first, then the RGB set; the alpha set on the covering with the colour kept. Every expected pixel is `Fixtures/levels_individual/expected_levels_individual.json`, written by `tools/levels_individual_reference.py` before this code existed and printed in document 25 as FX-LVLIC-001 to 020. A Levels saved before D-383, with its five settings, opens as the RGB set and draws as it did: `tests/b55_levels.rs` and its fixtures are unchanged.

## FX-LVLIC-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LVLIC-001 frame 0: Levels (Individual Controls) with every setting as it starts: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LVLIC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-002 frame 0: Red input white 192: red brightened, every red at or above 192 full; green and blue untouched. | largest difference 1.9e-7 | yes |
| FX-LVLIC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-003 frame 0: Green gamma 2: the middle of the green channel brighter; 0 and full green stay; red and blue untouched. | largest difference 1.9e-7 | yes |
| FX-LVLIC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-004 frame 0: Blue output 64 to 192: blue squeezed into that range, so no blue below 64 or above 192; red and green untouched. | largest difference 1.9e-7 | yes |
| FX-LVLIC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-005 frame 0: The order: Red input white 192, then the RGB input black 32 on top. Red is stretched by its own set first and then by the RGB set; green and blue by the RGB set alone. | largest difference 2.2e-7 | yes |
| FX-LVLIC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-006 frame 0: Red output black 255 and white 0: the red channel turned over, green and blue untouched. | largest difference 1.3e-7 | yes |
| FX-LVLIC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-007 frame 0: Alpha output white 128: every covering halved, the colours kept; the empty column stays empty. | largest difference 1.0e-7 | yes |
| FX-LVLIC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-008 frame 0: Alpha output black 64: every covering lifted to at least 64 of 255, so the empty column shows as black at a quarter covering. | largest difference 1.9e-7 | yes |
| FX-LVLIC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-009 frame 0: Alpha input white 128: the half-covered yellow becomes fully covered; the rest, already full, stays. | largest difference 1.9e-7 | yes |
| FX-LVLIC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-010 frame 0: Blue input black and white both 140: a threshold on blue, 0 below 140 and full at or above it. | largest difference 1.9e-7 | yes |
| FX-LVLIC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-011 frame 0: Levels with its Channel menu on Red and Red input white 192: exactly FX-LVLIC-002, the same rule under the other name. | largest difference 1.9e-7 | yes |
| FX-LVLIC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-012 frame 0: Levels with its Channel menu on Alpha, and only the RGB five written, input white 200: the menu changes nothing and the four sets not written start plain, so this draws as D-112's Levels with input white 200. | largest difference 2.4e-7 | yes |
| FX-LVLIC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-013 frame 0: Red gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 untouched, frames 2 and 4 the middle reds brighter and brighter. | largest difference 1.9e-7 | yes |
| FX-LVLIC-013 frame 2: Red gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 untouched, frames 2 and 4 the middle reds brighter and brighter. | largest difference 1.3e-7 | yes |
| FX-LVLIC-013 frame 4: Red gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 untouched, frames 2 and 4 the middle reds brighter and brighter. | largest difference 1.3e-7 | yes |
| FX-LVLIC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-014 frame 0: Every set at once: RGB input black 10 and gamma 1.2, red input white 230 and output black 20, green gamma 0.7 and output white 240, blue input black 30 and gamma 1.5, alpha output white 200. | largest difference 1.5e-7 | yes |
| FX-LVLIC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-015 frame 0: FX-LVLIC-014 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-LVLIC-015 frame 3: FX-LVLIC-014 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-LVLIC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LVLIC-016 frame 0: Red input black 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-016 frame 4: Red input black 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LVLIC-017 frame 0: Green gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-017 frame 4: Green gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LVLIC-018 frame 0: Blue output white -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-018 frame 4: Blue output white -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LVLIC-019 frame 0: Alpha gamma 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-019 frame 4: Alpha gamma 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LVLIC-020 frame 0: Levels with the channel "luma", which is not one of rgb, red, green, blue or alpha. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-020 frame 4: Levels with the channel "luma", which is not one of rgb, red, green, blue or alpha. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LVLIC-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lvlic_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lvlic_012.json, a Levels with its Channel menu and only the RGB five, is saved with its channel and all 25 settings, the ones it left out at their starts | {"alpha_gamma":1,"alpha_input_black":0,"alpha_input_white":255,"alpha_output_black":0,"alpha_output_white":255,"blue_gamma":1,"blue_input_black":0,"blue_input_white":255,"blue_output_black":0,"blue_output_white":255,"channel":"alpha","gamma":1,"green_gamma":1,"green_input_black":0,"green_input_white":255,"green_output_black":0,"green_output_white":255,"input_black":0,"input_white":200,"output_black":0,"output_white":255,"red_gamma":1,"red_input_black":0,"red_input_white":255,"red_output_black":0,"red_output_white":255} | yes |
| fx_lvlic_014.json, Levels (Individual Controls), is saved with its 25 settings and no channel | {"alpha_gamma":1,"alpha_input_black":0,"alpha_input_white":255,"alpha_output_black":0,"alpha_output_white":200,"blue_gamma":1.5,"blue_input_black":30,"blue_input_white":255,"blue_output_black":0,"blue_output_white":255,"gamma":1.2,"green_gamma":0.7,"green_input_black":0,"green_input_white":255,"green_output_black":0,"green_output_white":240,"input_black":10,"input_white":255,"output_black":0,"output_white":255,"red_gamma":1,"red_input_black":0,"red_input_white":230,"red_output_black":20,"red_output_white":255} | yes |
| fx_lvlic_016.json is refused in a sentence naming red input black | Levels (Individual Controls)'s red input black runs from 0 to 255, and this is 256. | yes |
| fx_lvlic_017.json is refused in a sentence naming green gamma | Levels (Individual Controls)'s green gamma runs from 0.1 to 10, and this is 0.05. | yes |
| fx_lvlic_018.json is refused in a sentence naming blue output white | Levels (Individual Controls)'s blue output white runs from 0 to 255, and this is -1. | yes |
| fx_lvlic_019.json is refused in a sentence naming alpha gamma | Levels (Individual Controls)'s alpha gamma runs from 0.1 to 10, and this is 11. | yes |
| fx_lvlic_020.json is refused in a sentence naming "luma" | Levels' channel is "rgb", "red", "green", "blue" or "alpha", and this is "luma". | yes |
| a file with a Levels (Individual Controls) without its red gamma is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Levels whose red input white is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Levels whose channel is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red input black 300 is refused with a sentence, and nothing changes | Levels (Individual Controls)'s red input black runs from 0 to 255, and this is 300. | yes |
| alpha gamma 0 is refused with a sentence, and nothing changes | Levels (Individual Controls)'s alpha gamma runs from 0.1 to 10, and this is 0. | yes |
| green output white keyed to 400 is refused with a sentence, and nothing changes | Levels (Individual Controls)'s green output white runs from 0 to 255, and this is 400. | yes |
| the blue set 20, 230, 1.5, 10, 250 is taken | taken | yes |
| alpha output white keyed from 255 to 0 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| Levels' channel "luma" is refused with a sentence, and nothing changes | Levels' channel is "rgb", "red", "green", "blue" or "alpha", and this is "luma". | yes |
| Levels' Channel menu on Green is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lvlic_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lvlic_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lvlic_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lvlic_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lvlic_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lvlic_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lvlic_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lvlic_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073597 pixels changed | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1 on three layers, frame 0, Full | largest difference 1 of 255, 1548 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1 on three layers, frame 100, Full | largest difference 1 of 255, 1366 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1 on three layers, frame 239, Full | largest difference 1 of 255, 1556 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1 on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1 on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels RGB input 16 to 235 and gamma 1.1 on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073592 pixels changed | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9 on three layers, frame 0, Full | largest difference 1 of 255, 234 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9 on three layers, frame 100, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9 on three layers, frame 239, Full | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9 on three layers, frame 0, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9 on three layers, frame 100, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9 on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red, green and blue each turned over: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073492 pixels changed | yes |
| the reference shot, Levels red, green and blue each turned over on three layers, frame 0, Full | largest difference 1 of 255, 1945 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red, green and blue each turned over on three layers, frame 100, Full | largest difference 1 of 255, 2806 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red, green and blue each turned over on three layers, frame 239, Full | largest difference 1 of 255, 2218 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red, green and blue each turned over on three layers, frame 0, Draft | largest difference 1 of 255, 2658 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red, green and blue each turned over on three layers, frame 100, Draft | largest difference 1 of 255, 2165 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels red, green and blue each turned over on three layers, frame 239, Draft | largest difference 1 of 255, 2700 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels alpha output white 200, on the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Levels alpha output white 200, on the processor on three layers, frame 0, Full | largest difference 1 of 255, 13501 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels alpha output white 200, on the processor on three layers, frame 100, Full | largest difference 1 of 255, 9542 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels alpha output white 200, on the processor on three layers, frame 239, Full | largest difference 1 of 255, 9818 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels alpha output white 200, on the processor on three layers, frame 0, Draft | largest difference 1 of 255, 9330 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels alpha output white 200, on the processor on three layers, frame 100, Draft | largest difference 1 of 255, 9729 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Levels alpha output white 200, on the processor on three layers, frame 239, Draft | largest difference 1 of 255, 9116 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-383 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| Levels (Individual Controls) as it is added changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_levels_as_before.png, a Levels saved before D-383 with its five settings: the same street, byte for byte, as Levels (Individual Controls) with those five as its RGB set; draws cleanly | [], 0 pixels differ | yes |
| 3_warm.png, red input white 220 and blue output white 210: the street warmer, no red darker, green untouched, no blue lighter; draws cleanly | [], warmer everywhere: true | yes |
| 4_channel_blue_gamma.png, Levels with its Channel menu on Blue and blue gamma 1.6: the street bluer in its middles, byte for byte Levels (Individual Controls) with blue gamma 1.6; draws cleanly | [], 0 pixels differ | yes |
| 5_alpha_half.png, alpha output white 128: the street half covering everywhere, its colours kept within 1 level; draws cleanly | [], every covering 128: true, colours kept: true | yes |

## Result

146 of 146 checks pass.
