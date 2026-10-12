# D-672: Drizzle

B-551, after CycoreFX's CC Drizzle (Simulation): raindrops on a pond from the in point, each a ring of ripples that spreads and dies away, bending the layer and catching the light by Blobbylize's rule. Nothing is carried from frame to frame: every frame counts the drops from the in point (document 20). CycoreFX publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/drizzle_reference.py`. Every expected pixel is `Fixtures/drizzle/expected_drizzle.json`, written by the tool before this code existed and printed in document 25 as FX-DRIZZLE-001 to 036. Tolerance 2e-5.

## FX-DRIZZLE-001 to 036 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DRIZZLE-001 frame 0: As added: ten drops a second, each ripple spreading for a second to a radius of an eighth of the picture's height (4 pixels here), two rings, lit from the top left: at frame 0 no drop yet, the still water lit evenly; at frames 13 and 25 small rings over the picture. | largest difference 1.2e-7 | yes |
| FX-DRIZZLE-001 frame 13: As added: ten drops a second, each ripple spreading for a second to a radius of an eighth of the picture's height (4 pixels here), two rings, lit from the top left: at frame 0 no drop yet, the still water lit evenly; at frames 13 and 25 small rings over the picture. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-001 frame 25: As added: ten drops a second, each ripple spreading for a second to a radius of an eighth of the picture's height (4 pixels here), two rings, lit from the top left: at frame 0 no drop yet, the still water lit evenly; at frames 13 and 25 small rings over the picture. | largest difference 1.4e-7 | yes |
| FX-DRIZZLE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-002 frame 14: Four drops a second spreading to 24 pixels, Displacement 200, unlit: the picture bent by wide rings and untouched between them; frame 8 has one young ripple, frame 20 three. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-002 frame 20: Four drops a second spreading to 24 pixels, Displacement 200, unlit: the picture bent by wide rings and untouched between them; frame 8 has one young ripple, frame 20 three. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-002 frame 8: Four drops a second spreading to 24 pixels, Displacement 200, unlit: the picture bent by wide rings and untouched between them; frame 8 has one young ripple, frame 20 three. | largest difference 1.7e-7 | yes |
| FX-DRIZZLE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-003 frame 14: FX-DRIZZLE-002 lit as added: the rings stand out, lighter on the sides facing the top-left light. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-004 frame 14: Rippling 0: one ring a ripple. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-005 frame 14: Rippling 1080: four rings a ripple, closer together. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-006 frame 14: Displacement 0: the picture is not bent, only lit by the rings. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-007 frame 14: Displacement -200: bent the other way. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-008 frame 14: Ripple Height 0: still water, so unlit the picture is untouched. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-009 frame 14: Ripple Height -100: the rings dip where they rose, twice as deep. | largest difference 1.6e-7 | yes |
| FX-DRIZZLE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-010 frame 14: Longevity 0.5: each ripple spreads in half a second and is gone, so fewer live at once. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-010 frame 20: Longevity 0.5: each ripple spreads in half a second and is gone, so fewer live at once. | largest difference 1.6e-7 | yes |
| FX-DRIZZLE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-011 frame 14: Longevity 0: no ripple lives; unlit, the picture untouched. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-012 frame 14: Drip Rate 0: no drops; unlit, the picture untouched. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-013 frame 11: Drip Rate keyed from 0 at frame 0 to 24 at frame 12: by frame 11 five drops have been born, the rate summed from the in point. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-013 frame 6: Drip Rate keyed from 0 at frame 0 to 24 at frame 12: by frame 11 five drops have been born, the rate summed from the in point. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-014 frame 14: Longevity keyed from 1 at frame 0 to 0.25 at frame 12: it is read at the in point only, so the same as FX-DRIZZLE-002. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-014 frame 20: Longevity keyed from 1 at frame 0 to 0.25 at frame 12: it is read at the in point only, so the same as FX-DRIZZLE-002. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-015 frame 14: FX-DRIZZLE-002 with the layer starting at frame 6: frame 14 is 002's frame 8 and frame 26 its frame 20, because the drops start at the in point. | largest difference 1.7e-7 | yes |
| FX-DRIZZLE-015 frame 26: FX-DRIZZLE-002 with the layer starting at frame 6: frame 14 is 002's frame 8 and frame 26 its frame 20, because the drops start at the in point. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-016 frame 14: FX-DRIZZLE-003 moved 3 pixels right: the same, moved. | largest difference 1.3e-7 | yes |
| FX-DRIZZLE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-017 frame 14: FX-DRIZZLE-002 at 30 frames a second: frame 14 is seven fifteenths of a second in, not seven twelfths. | largest difference 1.8e-7 | yes |
| FX-DRIZZLE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-018 frame 14: A point light at the middle, 10 pixels up: the water brightest under it and darker towards the corners. | largest difference 1.2e-7 | yes |
| FX-DRIZZLE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-019 frame 14: An orange light, Specular 100, Roughness 0.5, Metal 0, Diffuse 0: broad highlights in the light's own orange on the rings. | largest difference 5.3e-8 | yes |
| FX-DRIZZLE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-020 frame 14: The picture with its right half clear: the clear half stays clear except where a ring bends the picture across the edge. | largest difference 1.2e-7 | yes |
| FX-DRIZZLE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-021 frame 14: Light Direction 135, from the bottom right: the other sides of the rings light. | largest difference 2.0e-7 | yes |
| FX-DRIZZLE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-022 frame 14: Spreading 0: no ripple has a size; lit as added, the still water. | largest difference 1.2e-7 | yes |
| FX-DRIZZLE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-023 frame 14: Spreading keyed from 300 at frame 0 to 100 at frame 12: every live ripple takes the spreading as it is now, so at frame 14 all are a third the size of FX-DRIZZLE-002's. | largest difference 1.7e-7 | yes |
| FX-DRIZZLE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-024 frame 25: Thirty drops a second, Rippling 720, lit as added: a downpour. | largest difference 2.2e-7 | yes |
| FX-DRIZZLE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DRIZZLE-025 frame 0: Drip Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-025 frame 4: Drip Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-026 frame 0: Drip Rate 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-026 frame 4: Drip Rate 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-027 frame 0: Longevity 101, above 100 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-027 frame 4: Longevity 101, above 100 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-028 frame 0: Rippling 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-028 frame 4: Rippling 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-029 frame 0: Displacement 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-029 frame 4: Displacement 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-030 frame 0: Ripple Height -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-030 frame 4: Ripple Height -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-031 frame 0: Spreading 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-031 frame 4: Spreading 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-032 frame 0: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-032 frame 4: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-033 frame 0: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-033 frame 4: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-034 frame 0: A light colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-034 frame 4: A light colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-035 frame 0: Light position across 1001, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-035 frame 4: Light position across 1001, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DRIZZLE-036 frame 0: Drip Rate keyed to 2000 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-036 frame 4: Drip Rate keyed to 2000 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DRIZZLE-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_drizzle_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_drizzle_019.json is saved with its 18 settings, 16 numbers, a word and a colour, and no clock or history | 18 settings; specular 100, roughness 0.5, light type "distant", light colour "#ff8000" | yes |
| fx_drizzle_025.json is refused in a sentence naming drip rate | Drizzle's drip rate runs from 0 to 1000, and this is -1. | yes |
| fx_drizzle_026.json is refused in a sentence naming drip rate | Drizzle's drip rate runs from 0 to 1000, and this is 1001. | yes |
| fx_drizzle_027.json is refused in a sentence naming longevity | Drizzle's longevity runs from 0 to 100, and this is 101. | yes |
| fx_drizzle_028.json is refused in a sentence naming rippling | Drizzle's rippling runs from 0 to 3600, and this is 3601. | yes |
| fx_drizzle_029.json is refused in a sentence naming displacement | Drizzle's displacement runs from -1000 to 1000, and this is 1001. | yes |
| fx_drizzle_030.json is refused in a sentence naming ripple height | Drizzle's ripple height runs from -1000 to 1000, and this is -1001. | yes |
| fx_drizzle_031.json is refused in a sentence naming spreading | Drizzle's spreading runs from 0 to 1000, and this is 1001. | yes |
| fx_drizzle_032.json is refused in a sentence naming roughness | Drizzle's roughness runs from 0.001 to 1, and this is 0. | yes |
| fx_drizzle_033.json is refused in a sentence naming light type | Drizzle's light type is distant or point, and this is "spot". | yes |
| fx_drizzle_034.json is refused in a sentence naming light colour | Drizzle's light colour is written #rrggbb, and this is "white". | yes |
| fx_drizzle_035.json is refused in a sentence naming light position x | Drizzle's light position x runs from -1000 to 1000, and this is 1001. | yes |
| fx_drizzle_036.json is refused in a sentence naming drip rate | Drizzle's drip rate runs from 0 to 1000, and this is 2000. | yes |
| a file with a Drizzle whose drip rate is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Drizzle whose light type is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Drizzle without its light colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_drizzle_002.json frame 14 in tiles of 1 and of 64 | byte-identical | yes |
| fx_drizzle_018.json frame 14 in tiles of 1 and of 64 | byte-identical | yes |
| fx_drizzle_024.json frame 25 in tiles of 1 and of 64 | byte-identical | yes |
| fx_drizzle_015.json frame 26 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-672)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_drizzle_001.json, frames [0, 13, 25] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_002.json, frames [14, 20, 8] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_003.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_004.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_005.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_006.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_007.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_008.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_009.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_010.json, frames [14, 20] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_011.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_012.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_013.json, frames [11, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_014.json, frames [14, 20] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_015.json, frames [14, 26] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_016.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_017.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_018.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_019.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_020.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_021.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_022.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_023.json, frames [14] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_024.json, frames [25] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_025.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_026.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_027.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_028.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_029.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_030.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_031.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_032.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_033.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_034.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_035.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_drizzle_036.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Drizzle as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073432 pixels changed | yes |
| the reference shot, Drizzle as added on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle as added on three layers, frame 100, Full | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle as added on three layers, frame 239, Full | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle as added on three layers, frame 0, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle as added on three layers, frame 100, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle as added on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072371 pixels changed | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle on three layers, frame 0, Full | largest difference 1 of 255, 1123 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle on three layers, frame 100, Full | largest difference 1 of 255, 1113 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle on three layers, frame 239, Full | largest difference 1 of 255, 1028 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle on three layers, frame 0, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle on three layers, frame 100, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle thirty drops a second, Rippling 720, Spreading 150, a point light at the middle on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine on three layers, frame 0, Full | largest difference 1 of 255, 2446 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine on three layers, frame 100, Full | largest difference 1 of 255, 2197 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine on three layers, frame 239, Full | largest difference 1 of 255, 2634 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine on three layers, frame 0, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine on three layers, frame 100, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Drizzle drip rate keyed 0 to 40, an orange shine on three layers, frame 239, Draft | largest difference 1 of 255, 102 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-672 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 129525 pixels changed; warnings [] | yes |
| 2_downpour.png, frame 48: drawn without a warning, the street changed | 129154 pixels changed; warnings [] | yes |
| 3_point_light.png, frame 24: drawn without a warning, the street changed | 129448 pixels changed; warnings [] | yes |
| 4_orange_shine.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 5_big_slow_ripples.png, frame 48: drawn without a warning, the street changed | 129590 pixels changed; warnings [] | yes |
| 6_bent_unlit.png, frame 24: drawn without a warning, the street changed | 6388 pixels changed; warnings [] | yes |

## Result

211 of 211 checks pass.
