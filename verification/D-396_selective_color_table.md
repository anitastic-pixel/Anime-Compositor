# D-396: Selective Color

B-275, after After Effects' Selective Color (Photoshop's adjustment): the cyan, magenta, yellow and black turned up or down in one family of colours at a time (reds, yellows, greens, cyans, blues, magentas, whites, neutrals, blacks), relative or absolute, by Clement Boesch's reverse-engineering of Photoshop's (FFmpeg's selectivecolor), in fractions. Every expected pixel is `Fixtures/selective_color/expected_selective_color.json`, written by `tools/selective_color_reference.py` before this code existed and printed in document 25 as FX-SELC-001 to 023. Tolerance 2e-5.

## FX-SELC-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SELC-001 frame 0: The settings as they start: relative, every amount 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-SELC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-002 frame 0: Reds, cyan +100, absolute: the reds lose their red by as much as they are red, pure red goes black, orange olive, skin greyer; colours with no red family (greys, green, cyan, blue) are untouched. | largest difference 1.3e-7 | yes |
| FX-SELC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-003 frame 0: The same, relative: a channel's change is scaled by how much room it has below full, so pure red and orange, whose red is full, are untouched and the darker rows change most. | largest difference 1.8e-7 | yes |
| FX-SELC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-004 frame 0: Yellows, yellow -100, absolute: the yellows' blue raised by as much as they are yellow: pure yellow becomes white, orange pink, the warm tones greyer. | largest difference 1.9e-7 | yes |
| FX-SELC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-005 frame 0: Greens, magenta +100, absolute: the greens' green taken down, pure green to black; yellow and cyan, where green ties for largest, untouched. | largest difference 1.9e-7 | yes |
| FX-SELC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-006 frame 0: Cyans, cyan -100, relative: the cyans' red raised, pure cyan becomes white. | largest difference 1.9e-7 | yes |
| FX-SELC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-007 frame 0: Blues, yellow +100, absolute: the blues' blue taken down, pure blue to black; cyan and magenta, where blue ties for largest, untouched. | largest difference 1.9e-7 | yes |
| FX-SELC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-008 frame 0: Magentas, magenta -50 and black +50, absolute: the black darkens red and blue by half; on green the black and the magenta cut partly cancel, so pure magenta's green rises a quarter. | largest difference 1.9e-7 | yes |
| FX-SELC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-009 frame 0: Whites, black +100, absolute: the light pixels darkened by how light they are, white to black, the light greys and the skin and warm highlight darker; nothing at or below half grey changes. | largest difference 1.4e-7 | yes |
| FX-SELC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-010 frame 0: The same, relative: a full channel can't be changed, so pure white stays white (Adobe's note); the light greys still darken. | largest difference 2.4e-7 | yes |
| FX-SELC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-011 frame 0: Neutrals, cyan -30, yellow +40, relative: the midtones warmed, more red and less blue, most at half grey; pure colours, black and white untouched. | largest difference 1.7e-7 | yes |
| FX-SELC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-012 frame 0: Blacks, black -50, relative: the dark pixels lifted by how dark they are, black to half grey's level; nothing at or above half grey changes. | largest difference 1.9e-7 | yes |
| FX-SELC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-013 frame 0: Reds, cyan 40, magenta -60, yellow 10, black 20, absolute, Bœsch's Photoshop settings: the black darkening every channel as the colour changes shift them. | largest difference 1.2e-7 | yes |
| FX-SELC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-014 frame 0: Reds, every amount +100, absolute: each change held at the channel's own floor, the reds go to black by as much as they are red. | largest difference 1.2e-7 | yes |
| FX-SELC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-015 frame 0: All nine families at once, relative: each pixel moved by every family it belongs to, added together. | largest difference 1.6e-7 | yes |
| FX-SELC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-016 frame 0: All nine, absolute: the same amounts, unscaled, so larger changes. | largest difference 1.8e-7 | yes |
| FX-SELC-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-017 frame 0: Reds keyed from 0, 0, 0, 0 at frame 0 to cyan +100 at frame 4, linear, absolute: frame 0 untouched, frame 2 cyan +50, frame 4 as FX-SELC-002. | largest difference 1.9e-7 | yes |
| FX-SELC-017 frame 2: Reds keyed from 0, 0, 0, 0 at frame 0 to cyan +100 at frame 4, linear, absolute: frame 0 untouched, frame 2 cyan +50, frame 4 as FX-SELC-002. | largest difference 1.3e-7 | yes |
| FX-SELC-017 frame 4: Reds keyed from 0, 0, 0, 0 at frame 0 to cyan +100 at frame 4, linear, absolute: frame 0 untouched, frame 2 cyan +50, frame 4 as FX-SELC-002. | largest difference 1.3e-7 | yes |
| FX-SELC-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-018 frame 0: FX-SELC-015 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-SELC-018 frame 3: FX-SELC-015 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-SELC-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELC-019 frame 0: Reds' cyan 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-019 frame 4: Reds' cyan 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELC-020 frame 0: Blacks' black -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-020 frame 4: Blacks' black -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELC-021 frame 0: Greens three numbers, not four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-021 frame 4: Greens three numbers, not four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELC-022 frame 0: Method "Relative": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-022 frame 4: Method "Relative": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELC-023 frame 0: Method "percentage", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-023 frame 4: Method "percentage", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELC-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selc_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_selc_013.json is saved with its ten settings, the method and nine families of four | {"blacks":[0,0,0,0],"blues":[0,0,0,0],"cyans":[0,0,0,0],"greens":[0,0,0,0],"magentas":[0,0,0,0],"method":"absolute","neutrals":[0,0,0,0],"reds":[40,-60,10,20],"whites":[0,0,0,0],"yellows":[0,0,0,0]} | yes |
| fx_selc_019.json is refused in a sentence naming reds | Selective Color's reds runs from -100 to 100, and this is 101. | yes |
| fx_selc_020.json is refused in a sentence naming blacks | Selective Color's blacks runs from -100 to 100, and this is -101. | yes |
| fx_selc_021.json is refused in a sentence naming greens | Selective Color's greens are four numbers, the cyan, magenta, yellow and black, and this has 3. | yes |
| fx_selc_022.json is refused in a sentence naming method | Selective Color's method is "relative" or "absolute", and this is "Relative". | yes |
| fx_selc_023.json is refused in a sentence naming method | Selective Color's method is "relative" or "absolute", and this is "percentage". | yes |
| a file with a Selective Color whose reds are words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Selective Color without its reds is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Selective Color without its method is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| reds cyan 101 is refused with a sentence, and nothing changes | Selective Color's reds runs from -100 to 100, and this is 101. | yes |
| method "percentage" is refused with a sentence, and nothing changes | Selective Color's method is "relative" or "absolute", and this is "percentage". | yes |
| greens of three numbers is refused with a sentence, and nothing changes | Selective Color's greens are four numbers, the cyan, magenta, yellow and black, and this has 3. | yes |
| reds keyed to cyan 150 is refused with a sentence, and nothing changes | Selective Color's reds runs from -100 to 100, and this is 150. | yes |
| absolute, blues yellow -100 is taken | taken | yes |
| reds keyed from 0 to cyan 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selc_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selc_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selc_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selc_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selc_017.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selc_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selc_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_selc_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Selective Color all nine families, relative: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073551 pixels changed | yes |
| the reference shot, Selective Color all nine families, relative on three layers, frame 0, Full | largest difference 1 of 255, 860 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, relative on three layers, frame 100, Full | largest difference 1 of 255, 1009 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, relative on three layers, frame 239, Full | largest difference 1 of 255, 866 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, relative on three layers, frame 0, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, relative on three layers, frame 100, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, relative on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, absolute: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Selective Color all nine families, absolute on three layers, frame 0, Full | largest difference 1 of 255, 7578 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, absolute on three layers, frame 100, Full | largest difference 1 of 255, 10854 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, absolute on three layers, frame 239, Full | largest difference 1 of 255, 5542 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, absolute on three layers, frame 0, Draft | largest difference 1 of 255, 235 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, absolute on three layers, frame 100, Draft | largest difference 1 of 255, 327 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color all nine families, absolute on three layers, frame 239, Draft | largest difference 1 of 255, 149 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color reds cyan +100, absolute: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 117645 pixels changed | yes |
| the reference shot, Selective Color reds cyan +100, absolute on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color reds cyan +100, absolute on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color reds cyan +100, absolute on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color reds cyan +100, absolute on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color reds cyan +100, absolute on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Selective Color reds cyan +100, absolute on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-396 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| as added (relative, every amount 0) changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_reds_cyan.png, reds cyan +100, absolute: the red walls and skin lose red; pixels whose red is not the largest are untouched, no red rises; draws cleanly | [], others untouched: true, no red rises: true, 16020 pixels lost red | yes |
| 3_blues_bluer.png, blues yellow -100, absolute: the sky bluer; pixels whose blue is not the largest are untouched, no blue falls; draws cleanly | [], others untouched: true, no blue falls: true, 107394 pixels bluer | yes |
| 4_blacks_lifted.png, blacks black -50, relative: the shadows lifted; pixels whose largest channel is 128 or more untouched, nothing darker; draws cleanly | [], bright untouched: true, nothing darker: true, 29184 pixels lifted | yes |
| 5_whites_darker.png, whites black +100, absolute: the lightest pixels darkened; pixels whose smallest channel is 127 or less untouched, nothing brighter; draws cleanly | [], darker pixels untouched: true, nothing brighter: true, 63576 pixels darkened | yes |
| 6_all_nine.png, all nine families at once, relative: the street regraded; draws cleanly | [], 128544 pixels changed | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [60, 0, 0, 0], absolute, gives red 132 within 1 level; draws cleanly | [], (132, 100, 50) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [0, -60, 0, 0], absolute, gives green 148 within 1 level; draws cleanly | [], (180, 148, 50) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [0, 0, -70, 0], absolute, gives blue 106 within 1 level; draws cleanly | [], (180, 100, 106) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [100, 100, 100, 0], absolute, gives red 124, green 69, blue 34 within 1 level; draws cleanly | [], (124, 69, 34) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [-100, -100, -100, 0], absolute, gives red 204, green 149, blue 114 within 1 level; draws cleanly | [], (204, 149, 114) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [40, -60, 10, -40], absolute, gives red 193 within 1 level; draws cleanly | [], (193, 149, 77) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [40, -60, 10, 20], absolute, gives red 126 within 1 level; draws cleanly | [], (126, 142, 34) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [40, -60, 10, -80], absolute, gives blue 112 within 1 level; draws cleanly | [], (204, 149, 112) | yes |
| Photoshop, measured by Boesch: (180, 100, 50) with reds [40, -60, 10, -10], absolute, gives blue 51 within 1 level; draws cleanly | [], (159, 149, 51) | yes |

## Result

159 of 159 checks pass.
