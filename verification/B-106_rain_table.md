# B-106: rain

D-163, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the thirtieth of the third batch. Every expected pixel is `Fixtures/rain/expected_rain.json`, written by `tools/rain_reference.py` before this code existed and printed in document 25 as FX-RAIN-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RAIN-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAIN-001 frame 0: The settings as they start: colour #c8d8ff, density 30, spacing 24, length 20, width 1, direction 170, speed 30, seed 0, opacity 60, on the night solid. The frame is smaller than one cell of the field, so it sees few drops: none at frame 0, one long pale streak drifting a little right, at frame 3, and none again at frame 4, the rain having fallen 30 pixels a frame past it. | largest difference 2.2e-9 | yes |
| FX-RAIN-001 frame 3: The settings as they start: colour #c8d8ff, density 30, spacing 24, length 20, width 1, direction 170, speed 30, seed 0, opacity 60, on the night solid. The frame is smaller than one cell of the field, so it sees few drops: none at frame 0, one long pale streak drifting a little right, at frame 3, and none again at frame 4, the rain having fallen 30 pixels a frame past it. | largest difference 1.3e-8 | yes |
| FX-RAIN-001 frame 4: The settings as they start: colour #c8d8ff, density 30, spacing 24, length 20, width 1, direction 170, speed 30, seed 0, opacity 60, on the night solid. The frame is smaller than one cell of the field, so it sees few drops: none at frame 0, one long pale streak drifting a little right, at frame 3, and none again at frame 4, the rain having fallen 30 pixels a frame past it. | largest difference 2.2e-9 | yes |
| FX-RAIN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-002 frame 0: Spacing 4, length 5: many short drops across the frame. | largest difference 1.9e-8 | yes |
| FX-RAIN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-003 frame 0: Spacing 4, length 5, density 0: no cell holds a drop, so the solid is untouched at every frame. | largest difference 2.2e-9 | yes |
| FX-RAIN-003 frame 4: Spacing 4, length 5, density 0: no cell holds a drop, so the solid is untouched at every frame. | largest difference 2.2e-9 | yes |
| FX-RAIN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-004 frame 0: Spacing 4, length 5, density 100: every cell holds a drop, FX-RAIN-002's among them in the same places, so every pixel is at least as rainy as there. | largest difference 2.1e-8 | yes |
| FX-RAIN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-005 frame 0: Spacing 4, length 5, opacity 100: FX-RAIN-002's drops at five thirds of their strength. | largest difference 2.9e-8 | yes |
| FX-RAIN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-006 frame 0: Spacing 4, length 5, opacity 0: the solid, untouched. | largest difference 2.2e-9 | yes |
| FX-RAIN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-007 frame 0: Spacing 4, length 5, width 3: FX-RAIN-002's drops thicker, every pixel at least as rainy. | largest difference 2.6e-8 | yes |
| FX-RAIN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-008 frame 0: Spacing 4, length 5, width 0: hairline drops, never more than half their strength, every pixel at most as rainy as FX-RAIN-002. | largest difference 1.6e-8 | yes |
| FX-RAIN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-009 frame 0: Spacing 4, length 0: each drop a dot, every pixel at most as rainy as FX-RAIN-002. | largest difference 1.4e-8 | yes |
| FX-RAIN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-010 frame 0: Spacing 4, length 5, direction 180, speed 1: falling straight down one pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 four; the streaks run down the columns. | largest difference 1.8e-8 | yes |
| FX-RAIN-010 frame 1: Spacing 4, length 5, direction 180, speed 1: falling straight down one pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 four; the streaks run down the columns. | largest difference 1.8e-8 | yes |
| FX-RAIN-010 frame 4: Spacing 4, length 5, direction 180, speed 1: falling straight down one pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 four; the streaks run down the columns. | largest difference 1.8e-8 | yes |
| FX-RAIN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-011 frame 0: FX-RAIN-010 with direction 540, a whole turn past 180: the same. | largest difference 1.8e-8 | yes |
| FX-RAIN-011 frame 1: FX-RAIN-010 with direction 540, a whole turn past 180: the same. | largest difference 1.8e-8 | yes |
| FX-RAIN-011 frame 4: FX-RAIN-010 with direction 540, a whole turn past 180: the same. | largest difference 1.8e-8 | yes |
| FX-RAIN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-012 frame 0: Spacing 4, length 5, direction 90, speed 0: falling to the right, the streaks run along the rows; with speed 0 frame 4 is frame 0. | largest difference 1.3e-8 | yes |
| FX-RAIN-012 frame 4: Spacing 4, length 5, direction 90, speed 0: falling to the right, the streaks run along the rows; with speed 0 frame 4 is frame 0. | largest difference 1.3e-8 | yes |
| FX-RAIN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-013 frame 0: Spacing 4, length 5, seed 7: drops of their own. | largest difference 1.6e-8 | yes |
| FX-RAIN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-014 frame 0: Spacing 4, length 5, seed 7.9, which counts as 7: FX-RAIN-013. | largest difference 1.6e-8 | yes |
| FX-RAIN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-015 frame 0: FX-RAIN-002 with its colour written in capitals, #C8D8FF: the same. | largest difference 1.9e-8 | yes |
| FX-RAIN-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-016 frame 0: Spacing 4, length 5, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100. | largest difference 2.2e-9 | yes |
| FX-RAIN-016 frame 2: Spacing 4, length 5, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100. | largest difference 1.5e-8 | yes |
| FX-RAIN-016 frame 4: Spacing 4, length 5, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100. | largest difference 2.8e-8 | yes |
| FX-RAIN-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-017 frame 0: Spacing 4, length 5, density keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the solid; frame 2 would pass 100, is held at 100, and is density 100 at frame 2. | largest difference 2.2e-9 | yes |
| FX-RAIN-017 frame 2: Spacing 4, length 5, density keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the solid; frame 2 would pass 100, is held at 100, and is density 100 at frame 2. | largest difference 2.3e-8 | yes |
| FX-RAIN-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-018 frame 0: FX-RAIN-002 moved three pixels right: the field is the drawing's own, so the rain moves with it, and the three columns left bare stay empty. | largest difference 1.9e-8 | yes |
| FX-RAIN-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-019 frame 0: FX-RAIN-002 on Noise's card: the same drops, each pixel that shows mixed toward the rain as much as on the solid, the soft edge at its own half covering, and the empty column and row stay empty. | largest difference 2.0e-7 | yes |
| FX-RAIN-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAIN-020 frame 0: Density 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-020 frame 4: Density 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-021 frame 0: Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-021 frame 4: Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-022 frame 0: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-022 frame 4: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-023 frame 0: Width 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-023 frame 4: Width 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-024 frame 0: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-024 frame 4: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-025 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-025 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-026 frame 0: A colour written "#c8d8f", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-026 frame 4: A colour written "#c8d8f", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAIN-027 frame 0: Speed keyed to 1001 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-027 frame 4: Speed keyed to 1001 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-RAIN-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 20 grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the spacing, length, width and speed, and nothing else | Rain { color: "#c8d8ff", density: 30.0, spacing: 12.0, length: 10.0, width: 1.0, direction: 170.0, speed: 15.0, seed: 5.0, opacity: 60.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rain_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rain_015.json, its colour written in capitals, is saved in small letters, as Colour          Key's is | "#c8d8ff" | yes |
| a file with no `speed` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a density that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| density 101 is refused with a sentence, and nothing changes | Rain's density runs from 0 to 100, and this is 101. | yes |
| spacing 1 is refused with a sentence, and nothing changes | Rain's spacing runs from 2 to 1000, and this is 1. | yes |
| length -1 is refused with a sentence, and nothing changes | Rain's length runs from 0 to 1000, and this is -1. | yes |
| width 21 is refused with a sentence, and nothing changes | Rain's width runs from 0 to 20, and this is 21. | yes |
| direction 3601 is refused with a sentence, and nothing changes | Rain's direction runs from -3600 to 3600, and this is 3601. | yes |
| speed 1001 is refused with a sentence, and nothing changes | Rain's speed runs from 0 to 1000, and this is 1001. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Rain's seed runs from 0 to 100000, and this is 100001. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Rain's opacity runs from 0 to 100, and this is 101. | yes |
| colour "#c8d8f" is refused with a sentence, and nothing changes | Rain's colour is written #rrggbb, and this is "#c8d8f". | yes |
| colour "blue" is refused with a sentence, and nothing changes | Rain's colour is written #rrggbb, and this is "blue". | yes |
| speed keyed to 1001 is refused with a sentence, and nothing changes | Rain's speed runs from 0 to 1000, and this is 1001. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| opacity keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rain_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rain_010.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rain_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

108 of 108 checks pass.
