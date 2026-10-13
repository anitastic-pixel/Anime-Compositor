# D-673: Hair

B-552, after CycoreFX's CC Hair (Simulation): strands grown where the layer is opaque, 3D curves seen straight on, hanging under their weight, coloured from the hair colour and the picture at their roots and lit as hair is, brightest across the light. Nothing moves by itself: a frame is the settings as they are at that frame (document 20). CycoreFX publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/hair_reference.py`. Every expected pixel is `Fixtures/hair/expected_hair.json`, written by the tool before this code existed and printed in document 25 as FX-HAIR-001 to 032. Tolerance 2e-5.

## FX-HAIR-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HAIR-001 frame 3: As added: on a picture 32 pixels high the hair is a short fuzz a pixel long and a twentieth of a pixel thick, 6000 strands, half the picture's colour and half brown. | largest difference 1.7e-7 | yes |
| FX-HAIR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-002 frame 3: A close look: about thirty strands of 6 to 10 pixels, a pixel thick, hanging by half their length. | largest difference 1.5e-7 | yes |
| FX-HAIR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-003 frame 3: Weight 0: the strands stand straight out, short where they face the viewer. | largest difference 1.6e-7 | yes |
| FX-HAIR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-004 frame 3: Weight -100: the strands curl upwards. | largest difference 1.5e-7 | yes |
| FX-HAIR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-005 frame 3: Weight 150 with Constant Mass: the longer strands hang further. | largest difference 1.5e-7 | yes |
| FX-HAIR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-006 frame 3: Color Inheritance 0: every strand brown. | largest difference 1.5e-7 | yes |
| FX-HAIR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-007 frame 3: Color Inheritance 100: every strand the picture's colour at its root. | largest difference 1.7e-7 | yes |
| FX-HAIR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-008 frame 3: Opacity 40: the picture shows through the strands. | largest difference 1.5e-7 | yes |
| FX-HAIR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-009 frame 3: Light Direction 135, from the bottom right. | largest difference 1.5e-7 | yes |
| FX-HAIR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-010 frame 3: Specular 100, Roughness 0.5, Metal 100: a broad shine in the strands' own colour. | largest difference 1.5e-7 | yes |
| FX-HAIR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-011 frame 3: Unlit: Ambient 100, no diffuse or specular light; each strand one flat colour. | largest difference 1.5e-7 | yes |
| FX-HAIR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-012 frame 3: Density 0: no hair, the picture untouched. | largest difference 1.5e-7 | yes |
| FX-HAIR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-013 frame 3: Length 0: no hair, the picture untouched. | largest difference 1.5e-7 | yes |
| FX-HAIR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-014 frame 3: The picture with its right half clear: hair roots only on the left half, the strands hanging a little across. | largest difference 1.5e-7 | yes |
| FX-HAIR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-015 frame 0: Weight keyed from 0 at frame 0 to 100 at frame 12: at frame 0 as FX-HAIR-003, then hanging lower. | largest difference 1.6e-7 | yes |
| FX-HAIR-015 frame 12: Weight keyed from 0 at frame 0 to 100 at frame 12: at frame 0 as FX-HAIR-003, then hanging lower. | largest difference 1.5e-7 | yes |
| FX-HAIR-015 frame 6: Weight keyed from 0 at frame 0 to 100 at frame 12: at frame 0 as FX-HAIR-003, then hanging lower. | largest difference 1.5e-7 | yes |
| FX-HAIR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-016 frame 3: FX-HAIR-002 moved 3 pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-HAIR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-017 frame 3: A blue hair colour, Color Inheritance 0. | largest difference 1.5e-7 | yes |
| FX-HAIR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-018 frame 3: Thickness 100: strands three pixels thick at the root. | largest difference 1.6e-7 | yes |
| FX-HAIR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-019 frame 3: A fur: density 3, length 150, so about 180 strands of 3 to 5 pixels. | largest difference 1.7e-7 | yes |
| FX-HAIR-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-020 frame 3: Roughness 0.01: a thin, tight shine. | largest difference 1.5e-7 | yes |
| FX-HAIR-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HAIR-021 frame 0: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-021 frame 4: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-022 frame 0: Length 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-022 frame 4: Length 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-023 frame 0: Thickness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-023 frame 4: Thickness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-024 frame 0: Weight 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-024 frame 4: Weight 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-025 frame 0: Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-025 frame 4: Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-026 frame 0: Color Inheritance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-026 frame 4: Color Inheritance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-027 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-027 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-028 frame 0: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-028 frame 4: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-029 frame 0: Constant Mass written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-029 frame 4: Constant Mass written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-030 frame 0: A hair colour written "brown". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-030 frame 4: A hair colour written "brown". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-031 frame 0: Light Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-031 frame 4: Light Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HAIR-032 frame 0: Weight keyed to 600 at frame 4, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-032 frame 4: Weight keyed to 600 at frame 4, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HAIR-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hair_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hair_017.json is saved with its 15 settings, 13 numbers, a word and a colour | 15 settings; density 0.5, colour inheritance 0, constant mass "off", hair colour "#2050ff" | yes |
| fx_hair_021.json is refused in a sentence naming length | Hair's length runs from 0 to 1000, and this is -1. | yes |
| fx_hair_022.json is refused in a sentence naming length | Hair's length runs from 0 to 1000, and this is 1001. | yes |
| fx_hair_023.json is refused in a sentence naming thickness | Hair's thickness runs from 0 to 100, and this is 101. | yes |
| fx_hair_024.json is refused in a sentence naming weight | Hair's weight runs from -500 to 500, and this is 501. | yes |
| fx_hair_025.json is refused in a sentence naming density | Hair's density runs from 0 to 1000, and this is -1. | yes |
| fx_hair_026.json is refused in a sentence naming color inheritance | Hair's color inheritance runs from 0 to 100, and this is 101. | yes |
| fx_hair_027.json is refused in a sentence naming opacity | Hair's opacity runs from 0 to 100, and this is 101. | yes |
| fx_hair_028.json is refused in a sentence naming roughness | Hair's roughness runs from 0.001 to 1, and this is 0. | yes |
| fx_hair_029.json is refused in a sentence naming constant mass | Hair's constant mass is off or on, and this is "yes". | yes |
| fx_hair_030.json is refused in a sentence naming hair colour | Hair's hair colour is written #rrggbb, and this is "brown". | yes |
| fx_hair_031.json is refused in a sentence naming light direction | Hair's light direction runs from -3600 to 3600, and this is 3601. | yes |
| fx_hair_032.json is refused in a sentence naming weight | Hair's weight runs from -500 to 500, and this is 600. | yes |
| a file with a Hair whose length is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Hair whose constant mass is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Hair without its hair colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hair_001.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hair_002.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hair_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hair_019.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-673)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hair_001.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_002.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_003.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_004.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_005.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_006.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_007.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_008.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_009.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_010.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_011.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_012.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_013.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_014.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_015.json, frames [0, 12, 6] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_016.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_017.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_018.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_019.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_020.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_021.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_022.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_023.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_024.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_025.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_026.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_027.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_028.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_029.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_030.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_031.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_hair_032.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Hair as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 281668 pixels changed | yes |
| the reference shot, Hair as added on three layers, frame 0, Full | largest difference 1 of 255, 3457 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair as added on three layers, frame 100, Full | largest difference 1 of 255, 1345 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair as added on three layers, frame 239, Full | largest difference 1 of 255, 1670 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair as added on three layers, frame 0, Draft | largest difference 1 of 255, 110 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair as added on three layers, frame 100, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair as added on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1345934 pixels changed | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60 on three layers, frame 0, Full | largest difference 1 of 255, 2063 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60 on three layers, frame 100, Full | largest difference 1 of 255, 1264 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60 on three layers, frame 239, Full | largest difference 1 of 255, 1431 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60 on three layers, frame 0, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60 on three layers, frame 100, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair a long fur: length 200, thickness 6, density 60 on three layers, frame 239, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1269058 pixels changed | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue on three layers, frame 0, Full | largest difference 1 of 255, 1582 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue on three layers, frame 100, Full | largest difference 1 of 255, 1157 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue on three layers, frame 239, Full | largest difference 1 of 255, 981 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue on three layers, frame 100, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hair weight keyed -100 to 200, constant mass, blue on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-673 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed | 41407 pixels changed; warnings [] | yes |
| 2_long_fur.png: drawn without a warning, the street changed | 107592 pixels changed; warnings [] | yes |
| 3_curled_up.png: drawn without a warning, the street changed | 113920 pixels changed; warnings [] | yes |
| 4_blue_hair.png: drawn without a warning, the street changed | 69911 pixels changed; warnings [] | yes |
| 5_shiny.png: drawn without a warning, the street changed | 84247 pixels changed; warnings [] | yes |
| 6_constant_mass.png: drawn without a warning, the street changed | 85517 pixels changed; warnings [] | yes |

## Result

189 of 189 checks pass.
