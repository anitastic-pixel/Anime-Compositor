# D-412: Map Chromatic Displacement

B-291: PLUGINS.md's pick #19, Prism Displacement and Red Giant's Chromatic Displacement merged into `core.displacement_map`: Red, Green and Blue Amount (`red_amount`, `green_amount`, `blue_amount`, -1000 to 1000 per cent of D-193's displacement) and Spectrum (`spectrum`, 3 to 32 samples, taken whole), all keyable. The samples run from red through green to blue, each read at its own share of the displacement; each colour is the samples' mean weighted toward it, the covering the largest. 3 samples part three colours; more give Red Giant's smooth rainbow. 100, 100, 100 and 3, what a file without them means, is D-193's rule. Every expected pixel is `Fixtures/map_chromatic/expected_map_chromatic.json`, written by `tools/map_chromatic_reference.py` before this code existed and printed in document 25 as FX-MAPCHROMA-001 to 019. Tolerance 2e-5.

## FX-MAPCHROMA-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MAPCHROMA-001 frame 0: The ramp at most 2, all three amounts 100 written and 3 samples: FX-DMAP-002 exactly. | largest difference 3.1e-7 | yes |
| FX-MAPCHROMA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-002 frame 0: A white solid at most 2, red 0, green 100, blue 200, 3 samples: red stays put, green is read 2 right and 2 below, blue 4 right and 4 below; the covering is the largest of the three. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-003 frame 0: FX-MAPCHROMA-002 with 9 samples: each channel blended over the colours beside it, so between red's and blue's places the fringes run smoothly. | largest difference 1.7e-7 | yes |
| FX-MAPCHROMA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-004 frame 0: The ramp, red 50, green 100, blue 150, 3 samples: where the ramp pushes hardest, at its ends, the colours part most. | largest difference 5.4e-7 | yes |
| FX-MAPCHROMA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-005 frame 0: FX-MAPCHROMA-004 with 16 samples, an even count: the rainbow. | largest difference 3.3e-7 | yes |
| FX-MAPCHROMA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-006 frame 0: The painted map, blue across at most 2.5 and luminance down at most 1.5 (FX-DMAP-008), red -100, green 0, blue 100, 5 samples: red moves against the push, green stays, blue with it. | largest difference 2.9e-7 | yes |
| FX-MAPCHROMA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-007 frame 0: The white solid, blue keyed from 100 at frame 0 to 300 at frame 4, the others absent: frame 0 is FX-DMAP-003, then blue parts from the rest. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-007 frame 2: The white solid, blue keyed from 100 at frame 0 to 300 at frame 4, the others absent: frame 0 is FX-DMAP-003, then blue parts from the rest. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-007 frame 4: The white solid, blue keyed from 100 at frame 0 to 300 at frame 4, the others absent: frame 0 is FX-DMAP-003, then blue parts from the rest. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-008 frame 0: FX-MAPCHROMA-002 with Wrap on: what each channel reads from beyond the right or the bottom comes from the other side. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-009 frame 0: FX-MAPCHROMA-004 on the holder moved 2 right and 1 down: the same picture, moved. | largest difference 5.4e-7 | yes |
| FX-MAPCHROMA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-010 frame 0: FX-MAPCHROMA-005 on an adjustment layer above the holder: the frame is the holder's rectangle, so FX-MAPCHROMA-005. | largest difference 3.3e-7 | yes |
| FX-MAPCHROMA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-011 frame 0: Spectrum 4.5: its whole part, 4 samples. | largest difference 4.1e-7 | yes |
| FX-MAPCHROMA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-012 frame 0: All three amounts 50 with 9 samples: no colour parts, the ramp's displacement halved (FX-DMAP-002 at most 1). | largest difference 2.3e-7 | yes |
| FX-MAPCHROMA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-013 frame 0: Spectrum keyed from 3 at frame 0 to 11 at frame 4 on FX-MAPCHROMA-002: frame 0 is FX-MAPCHROMA-002, frame 3 9 samples, FX-MAPCHROMA-003. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-013 frame 3: Spectrum keyed from 3 at frame 0 to 11 at frame 4 on FX-MAPCHROMA-002: frame 0 is FX-MAPCHROMA-002, frame 3 9 samples, FX-MAPCHROMA-003. | largest difference 1.7e-7 | yes |
| FX-MAPCHROMA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-014 frame 0: No layer named: nothing moves, whatever the amounts. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAPCHROMA-015 frame 0: Red amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-015 frame 4: Red amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAPCHROMA-016 frame 0: Blue amount -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-016 frame 4: Blue amount -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAPCHROMA-017 frame 0: Spectrum 2, below 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-017 frame 4: Spectrum 2, below 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAPCHROMA-018 frame 0: Spectrum 33, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-018 frame 4: Spectrum 33, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAPCHROMA-019 frame 0: Green amount keyed from 100 at frame 0 to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-019 frame 4: Green amount keyed from 100 at frame 0 to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-MAPCHROMA-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: every colour moving alike, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| displacement_map/fx_dmap_001.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_002.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_003.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_004.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_005.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_006.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_007.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_008.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_009.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_010.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_011.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_012.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_013.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_014.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_015.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_016.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_017.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_018.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_019.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_020.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_021.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_022.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_023.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_024.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_025.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_026.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_027.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_028.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_029.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_030.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |
| displacement_map/fx_dmap_031.json is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3 | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mapchroma_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mapchroma_003.json is saved with Red 0, Green 100, Blue 200 and Spectrum 9 | {"blue_amount":200,"fit":"stretch","green_amount":100,"horizontal":"red","layer":"white","max_horizontal":2,"max_vertical":2,"red_amount":0,"spectrum":9,"vertical":"green","wrap":"off"} | yes |
| fx_mapchroma_015.json (Red Amount 1001) is refused in a sentence naming red amount | Displacement Map's red amount runs from -1000 to 1000, and this is 1001. | yes |
| fx_mapchroma_016.json (Blue Amount -1001) is refused in a sentence naming blue amount | Displacement Map's blue amount runs from -1000 to 1000, and this is -1001. | yes |
| fx_mapchroma_017.json (Spectrum 2) is refused in a sentence naming spectrum | Displacement Map's spectrum runs from 3 to 32, and this is 2. | yes |
| fx_mapchroma_018.json (Spectrum 33) is refused in a sentence naming spectrum | Displacement Map's spectrum runs from 3 to 32, and this is 33. | yes |
| a file with a Displacement Map whose Spectrum is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the maxima and leaves the amounts, shares and not distances, and the spectrum | DisplacementMap { layer: String("white"), fit: "stretch", horizontal: "red", max_horizontal: 1.0, vertical: "green", max_vertical: 1.0, wrap: "off", expand: "off", red_amount: 0.0, green_amount: 100.0, blue_amount: 200.0, spectrum: 3.0, map: None } | yes |
| with Expand Output and a maximum of 2.5, the drawing grows by 3 with every colour at 100, by 5 with Blue at 200, by 8 with Red at -300 | 3, 5, 8 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Red Amount 1001 is refused with a sentence, and nothing changes | Displacement Map's red amount runs from -1000 to 1000, and this is 1001. | yes |
| Spectrum 2 is refused with a sentence, and nothing changes | Displacement Map's spectrum runs from 3 to 32, and this is 2. | yes |
| Green Amount keyed to 2000 is refused with a sentence, and nothing changes | Displacement Map's green amount runs from -1000 to 1000, and this is 2000. | yes |
| water: Red 50, Green 100, Blue 150, Spectrum 16 is taken | taken | yes |
| Blue Amount keyed from 100 to 300 is taken | taken | yes |
| Spectrum keyed from 3 to 32 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mapchroma_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mapchroma_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mapchroma_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mapchroma_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mapchroma_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mapchroma_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mapchroma_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mapchroma_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mapchroma_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3: the processor's frame 102 differs from the same displacement with every colour at 100, so the comparisons below test the colours parting | 137662 pixels changed | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 0, Full | largest difference 1 of 255, 1973 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 1, Full | largest difference 1 of 255, 1974 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 100, Full | largest difference 1 of 255, 761 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 102, Full | largest difference 1 of 255, 838 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 239, Full | largest difference 1 of 255, 739 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 1, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 102, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3 on three layers, frame 239, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16: the processor's frame 102 differs from the same displacement with every colour at 100, so the comparisons below test the colours parting | 1769263 pixels changed | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 0, Full | largest difference 1 of 255, 1335 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 1, Full | largest difference 1 of 255, 1332 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 100, Full | largest difference 1 of 255, 966 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 102, Full | largest difference 1 of 255, 954 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 239, Full | largest difference 1 of 255, 759 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 1, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 102, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16 on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7: the processor's frame 102 differs from the same displacement with every colour at 100, so the comparisons below test the colours parting | 148465 pixels changed | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 0, Full | largest difference 1 of 255, 1240 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 1, Full | largest difference 1 of 255, 1241 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 100, Full | largest difference 1 of 255, 1026 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 102, Full | largest difference 1 of 255, 1029 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 239, Full | largest difference 1 of 255, 741 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 0, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 1, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 102, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Displacement Map hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7 on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: B-128's street and maps, in `verification/D-412 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_waves_alike.png, waves, 8 pixels, every colour at 100: D-193's displacement as before; draws cleanly | []; 0 pixels differ from the picture before | yes |
| 2_waves_split.png, the same with Red 40, Green 100, Blue 160, Spectrum 3: three colours parted into hard fringes; draws cleanly, and differs from the picture before | []; 82882 pixels differ from the picture before | yes |
| 3_waves_spectrum.png, the same with Spectrum 16: the fringes a smooth rainbow, as through water; draws cleanly, and differs from the picture before | []; 54561 pixels differ from the picture before | yes |
| 4_ball_alike.png, the glass ball, 30 pixels, every colour at 100; draws cleanly | []; 97074 pixels differ from the picture before | yes |
| 5_ball_prism.png, the glass ball with Red 70, Green 100, Blue 130, Spectrum 12: its rim parts into a rainbow, as glass does; draws cleanly, and differs from the picture before | []; 25442 pixels differ from the picture before | yes |

## Result

175 of 175 checks pass.
