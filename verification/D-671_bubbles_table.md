# D-671: Bubbles

B-550, after CycoreFX's CC Bubbles (Simulation): the layer turned into bubbles that rise or fall from the in point and wobble, each showing the picture through itself, turned over (Inverse) or upright (World), shaded one of five ways. Nothing is carried from frame to frame: every frame works each bubble out from the in point (document 20). CycoreFX publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/bubbles_reference.py`. Every expected pixel is `Fixtures/bubbles/expected_bubbles.json`, written by the tool before this code existed and printed in document 25 as FX-BUBBLE-001 to 034. Tolerance 2e-5.

## FX-BUBBLE-001 to 034 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BUBBLE-001 frame 0: As added: 100 bubbles of size 1 (on a picture 32 high, between half a pixel and a pixel and a half in radius), speed 0.5, wobble 5 at once a second, Inverse, Fade Inwards: a scatter of tiny faint dots on clear, the picture itself gone. | largest difference 8.6e-8 | yes |
| FX-BUBBLE-001 frame 24: As added: 100 bubbles of size 1 (on a picture 32 high, between half a pixel and a pixel and a half in radius), speed 0.5, wobble 5 at once a second, Inverse, Fade Inwards: a scatter of tiny faint dots on clear, the picture itself gone. | largest difference 8.9e-8 | yes |
| FX-BUBBLE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-002 frame 0: Six bubbles of size 8, 3 to 10 pixels in radius, each the picture under it turned over and spread, clear in the middle, rising: frame 12 is half a second later. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-002 frame 12: Six bubbles of size 8, 3 to 10 pixels in radius, each the picture under it turned over and spread, clear in the middle, rising: frame 12 is half a second later. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-003 frame 0: World reflection: each bubble the picture round it, upright. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-004 frame 0: Shading None: fully filled bubbles. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-005 frame 0: Lighten: white towards the rim. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-006 frame 0: Darken: black towards the rim. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-007 frame 0: Fade Outwards: clear at the rim, solid in the middle. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-008 frame 0: Speed -2: falling, half the picture a second at pace 1. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-008 frame 6: Speed -2: falling, half the picture a second at pace 1. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-009 frame 0: Speed 0 and wobble 0: still, so frame 12 is frame 0. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-009 frame 12: Speed 0 and wobble 0: still, so frame 12 is frame 0. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-010 frame 0: Wobble 20 at twice a second: each swings twice its radius either side. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-010 frame 3: Wobble 20 at twice a second: each swings twice its radius either side. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-011 frame 0: Speed 0, wobble frequency 0: the wobble holds where it started, so frame 6 is frame 0. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-011 frame 6: Speed 0, wobble frequency 0: the wobble holds where it started, so frame 6 is frame 0. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-012 frame 3: Size 0: no bubble, clear. | largest difference 0.0e0 | yes |
| FX-BUBBLE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-013 frame 3: Amount 0: no bubble, clear. | largest difference 0.0e0 | yes |
| FX-BUBBLE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-014 frame 3: Seed 7: bubbles of their own. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-015 frame 3: Seed 7.9 counts as 7: FX-BUBBLE-014. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-016 frame 0: The picture with its right half clear: a bubble over it shows clear. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-017 frame 4: Speed keyed from 0 at frame 0 to 4 at frame 8: by frame 8 the bubbles have risen as far as at a steady 2 for a third of a second. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-017 frame 8: Speed keyed from 0 at frame 0 to 4 at frame 8: by frame 8 the bubbles have risen as far as at a steady 2 for a third of a second. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-018 frame 4: Wobble frequency keyed from 0 to 3 over 8 frames, wobble 20, speed 0. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-018 frame 8: Wobble frequency keyed from 0 to 3 over 8 frames, wobble 20, speed 0. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-019 frame 18: FX-BUBBLE-002 with the layer starting at frame 6: frame 6 is 002's frame 0 and frame 18 its frame 12, because the bubbles start at the in point. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-019 frame 6: FX-BUBBLE-002 with the layer starting at frame 6: frame 6 is 002's frame 0 and frame 18 its frame 12, because the bubbles start at the in point. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-020 frame 12: FX-BUBBLE-002 moved 3 pixels right: the same bubbles, moved, cut at the layer's edge. | largest difference 1.3e-7 | yes |
| FX-BUBBLE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-021 frame 12: FX-BUBBLE-002 at 30 frames a second: frame 12 is two fifths of a second in, not a half. | largest difference 1.4e-7 | yes |
| FX-BUBBLE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-022 frame 0: Forty bubbles with everything: size 5, speed 1.5, wobble 12 at 1.5, World, Lighten, seed 3. | largest difference 1.6e-7 | yes |
| FX-BUBBLE-022 frame 10: Forty bubbles with everything: size 5, speed 1.5, wobble 12 at 1.5, World, Lighten, seed 3. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-023 frame 0: Amount keyed from 0 at frame 0 to 12 at frame 4: six at frame 2, FX-BUBBLE-002's six. | largest difference 0.0e0 | yes |
| FX-BUBBLE-023 frame 2: Amount keyed from 0 at frame 0 to 12 at frame 4: six at frame 2, FX-BUBBLE-002's six. | largest difference 1.2e-7 | yes |
| FX-BUBBLE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-024 frame 0: Size 40: bubbles bigger than the picture, each a fisheye of it. | largest difference 1.8e-7 | yes |
| FX-BUBBLE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BUBBLE-025 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-025 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-026 frame 0: Amount 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-026 frame 4: Amount 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-027 frame 0: Speed 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-027 frame 4: Speed 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-028 frame 0: Wobble amplitude -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-028 frame 4: Wobble amplitude -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-029 frame 0: Wobble frequency 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-029 frame 4: Wobble frequency 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-030 frame 0: Size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-030 frame 4: Size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-031 frame 0: Random seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-031 frame 4: Random seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-032 frame 0: Reflection "mirror". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-032 frame 4: Reflection "mirror". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-033 frame 0: Shading "Lighten": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-033 frame 4: Shading "Lighten": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BUBBLE-034 frame 0: Speed keyed to 200 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-034 frame 4: Speed keyed to 200 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BUBBLE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bubble_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bubble_022.json is saved with its 8 settings, 6 numbers and 2 words, and no clock or history | 8 settings; amount 40, reflection "world", shading "lighten" | yes |
| fx_bubble_025.json is refused in a sentence naming amount | Bubbles's amount runs from 0 to 10000, and this is -1. | yes |
| fx_bubble_026.json is refused in a sentence naming amount | Bubbles's amount runs from 0 to 10000, and this is 10001. | yes |
| fx_bubble_027.json is refused in a sentence naming speed | Bubbles's speed runs from -100 to 100, and this is 101. | yes |
| fx_bubble_028.json is refused in a sentence naming wobble amplitude | Bubbles's wobble amplitude runs from 0 to 1000, and this is -1. | yes |
| fx_bubble_029.json is refused in a sentence naming wobble frequency | Bubbles's wobble frequency runs from 0 to 100, and this is 101. | yes |
| fx_bubble_030.json is refused in a sentence naming size | Bubbles's size runs from 0 to 100, and this is 101. | yes |
| fx_bubble_031.json is refused in a sentence naming random seed | Bubbles's random seed runs from 0 to 10000, and this is -1. | yes |
| fx_bubble_032.json is refused in a sentence naming reflection | Bubbles' reflection is one of "inverse", "world", and this is "mirror". | yes |
| fx_bubble_033.json is refused in a sentence naming shading | Bubbles' shading is one of "none", "lighten", "darken", "fade_inwards", "fade_outwards", and this is "Lighten". | yes |
| a file with a Bubbles whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bubbles whose shading is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bubbles without its reflection is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bubble_002.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bubble_022.json frame 10 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bubble_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bubble_017.json frame 8 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-671)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bubble_001.json, frames [0, 24] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_002.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_008.json, frames [0, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_009.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_010.json, frames [0, 3] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_011.json, frames [0, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_012.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_013.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_014.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_015.json, frames [3] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_016.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_017.json, frames [4, 8] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_018.json, frames [4, 8] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_019.json, frames [18, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_020.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_021.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_022.json, frames [0, 10] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_023.json, frames [0, 2] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_024.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_025.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_026.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_027.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_028.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_029.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_030.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_031.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_032.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_033.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_bubble_034.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Bubbles as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Bubbles as added on three layers, frame 0, Full | largest difference 1 of 255, 141 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles as added on three layers, frame 100, Full | largest difference 1 of 255, 192 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles as added on three layers, frame 239, Full | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles as added on three layers, frame 0, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles as added on three layers, frame 100, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles as added on three layers, frame 239, Draft | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073258 pixels changed | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten on three layers, frame 0, Full | largest difference 1 of 255, 327 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten on three layers, frame 100, Full | largest difference 1 of 255, 359 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten on three layers, frame 239, Full | largest difference 1 of 255, 260 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten on three layers, frame 0, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten on three layers, frame 100, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles forty big bubbles, World, Lighten on three layers, frame 239, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1996744 pixels changed | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles on three layers, frame 0, Full | largest difference 1 of 255, 1040 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles on three layers, frame 100, Full | largest difference 1 of 255, 610 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles on three layers, frame 239, Full | largest difference 1 of 255, 864 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles on three layers, frame 0, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles on three layers, frame 100, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bubbles speed keyed 0 to 3, None shading, 300 bubbles on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-671 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 2_soap_bubbles.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 3_world_lighten.png, frame 24: drawn without a warning, the street changed | 129481 pixels changed; warnings [] | yes |
| 4_darken_falling.png, frame 24: drawn without a warning, the street changed | 129358 pixels changed; warnings [] | yes |
| 5_fade_outwards.png, frame 24: drawn without a warning, the street changed | 129524 pixels changed; warnings [] | yes |
| 6_one_fisheye.png, frame 0: drawn without a warning, the street changed | 128904 pixels changed; warnings [] | yes |

## Result

201 of 201 checks pass.
