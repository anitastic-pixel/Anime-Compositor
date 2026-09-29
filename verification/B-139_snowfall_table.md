# B-139: Snowfall

D-204, accepted on 2026-09-28 by the owner's "take everything", which took in the After Effects picks B1 to B12; this is B2. Every expected pixel is `Fixtures/snowfall/expected_snowfall.json`, written by `tools/snowfall_reference.py` before this code existed and printed in document 25 as FX-SNOW-001 to 029. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SNOW-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SNOW-001 frame 0: The settings as they start: colour #ffffff, density 50, spacing 32, size 6, scene depth 50, speed 2, wind 0.5, wiggle 3 every 48 frames, seed 0, opacity 100, on the night solid. The frame is smaller than one near cell, so it sees a few flakes, falling and drifting a little right between frame 0 and frame 4. | largest difference 2.9e-8 | yes |
| FX-SNOW-001 frame 4: The settings as they start: colour #ffffff, density 50, spacing 32, size 6, scene depth 50, speed 2, wind 0.5, wiggle 3 every 48 frames, seed 0, opacity 100, on the night solid. The frame is smaller than one near cell, so it sees a few flakes, falling and drifting a little right between frame 0 and frame 4. | largest difference 2.9e-8 | yes |
| FX-SNOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-002 frame 0: Spacing 6, size 3: flakes of all three planes across the frame, the far ones smaller, falling two pixels a frame, the far ones slower. | largest difference 3.0e-8 | yes |
| FX-SNOW-002 frame 2: Spacing 6, size 3: flakes of all three planes across the frame, the far ones smaller, falling two pixels a frame, the far ones slower. | largest difference 3.0e-8 | yes |
| FX-SNOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-003 frame 0: Spacing 6, size 3, density 0: no cell holds a flake, so the solid is untouched at every frame. | largest difference 2.2e-9 | yes |
| FX-SNOW-003 frame 4: Spacing 6, size 3, density 0: no cell holds a flake, so the solid is untouched at every frame. | largest difference 2.2e-9 | yes |
| FX-SNOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-004 frame 0: Spacing 6, size 3, density 100: every cell holds a flake, FX-SNOW-002's among them in the same places, so every pixel is at least as snowy as there. | largest difference 3.0e-8 | yes |
| FX-SNOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-005 frame 0: Spacing 6, size 3, opacity 50: FX-SNOW-002's flakes at half their strength. | largest difference 2.6e-8 | yes |
| FX-SNOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-006 frame 0: Spacing 6, size 3, opacity 0: the solid, untouched. | largest difference 2.2e-9 | yes |
| FX-SNOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-007 frame 0: Spacing 6, size 0: flakes of no size give no light; the solid, untouched at every frame. | largest difference 2.2e-9 | yes |
| FX-SNOW-007 frame 4: Spacing 6, size 0: flakes of no size give no light; the solid, untouched at every frame. | largest difference 2.2e-9 | yes |
| FX-SNOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-008 frame 0: Spacing 6, size 3, scene depth 0: the three planes all at the near scale, each its own flakes. | largest difference 3.0e-8 | yes |
| FX-SNOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-009 frame 0: Spacing 6, size 3, scene depth 100: the far plane at a third of the near scale, its flakes a pixel wide at most and close together. | largest difference 2.9e-8 | yes |
| FX-SNOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-010 frame 0: Spacing 6, size 3, scene depth 0, no wind and no wiggle, speed 1: every plane falls straight down one pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 four. | largest difference 3.0e-8 | yes |
| FX-SNOW-010 frame 1: Spacing 6, size 3, scene depth 0, no wind and no wiggle, speed 1: every plane falls straight down one pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 four. | largest difference 3.0e-8 | yes |
| FX-SNOW-010 frame 4: Spacing 6, size 3, scene depth 0, no wind and no wiggle, speed 1: every plane falls straight down one pixel a frame, so frame 1 is frame 0 moved one pixel down, and frame 4 four. | largest difference 3.0e-8 | yes |
| FX-SNOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-011 frame 0: Spacing 6, size 3, scene depth 0, no wiggle, speed 0, wind -1: the snow blown left one pixel a frame, so frame 2 is frame 0 moved two pixels left. | largest difference 3.0e-8 | yes |
| FX-SNOW-011 frame 2: Spacing 6, size 3, scene depth 0, no wiggle, speed 0, wind -1: the snow blown left one pixel a frame, so frame 2 is frame 0 moved two pixels left. | largest difference 3.0e-8 | yes |
| FX-SNOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-012 frame 0: Spacing 6, size 3, scene depth 0, speed 0, no wind, wiggle 2 every 4 frames: the flakes sway where they hang, so frame 2 differs and frame 4, one whole sway later, is frame 0. | largest difference 3.0e-8 | yes |
| FX-SNOW-012 frame 2: Spacing 6, size 3, scene depth 0, speed 0, no wind, wiggle 2 every 4 frames: the flakes sway where they hang, so frame 2 differs and frame 4, one whole sway later, is frame 0. | largest difference 3.0e-8 | yes |
| FX-SNOW-012 frame 4: Spacing 6, size 3, scene depth 0, speed 0, no wind, wiggle 2 every 4 frames: the flakes sway where they hang, so frame 2 differs and frame 4, one whole sway later, is frame 0. | largest difference 3.0e-8 | yes |
| FX-SNOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-013 frame 0: Spacing 6, size 3, still: no fall, no wind, no wiggle, so frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-SNOW-013 frame 4: Spacing 6, size 3, still: no fall, no wind, no wiggle, so frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-SNOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-014 frame 0: Spacing 6, size 3, seed 7: flakes of their own. | largest difference 3.0e-8 | yes |
| FX-SNOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-015 frame 0: Spacing 6, size 3, seed 7.9, which counts as 7: FX-SNOW-014. | largest difference 3.0e-8 | yes |
| FX-SNOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-016 frame 0: Spacing 6, size 3, colour a pale blue #a0c8ff written in capitals, #A0C8FF: FX-SNOW-002's flakes at the same strengths, in the blue. | largest difference 2.8e-8 | yes |
| FX-SNOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-017 frame 0: Spacing 6, size 3, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100. | largest difference 2.2e-9 | yes |
| FX-SNOW-017 frame 2: Spacing 6, size 3, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100. | largest difference 2.6e-8 | yes |
| FX-SNOW-017 frame 4: Spacing 6, size 3, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the solid, frame 2 opacity 50 and frame 4 opacity 100. | largest difference 3.0e-8 | yes |
| FX-SNOW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-018 frame 0: Spacing 6, size 3, density keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the solid; frame 2 would pass 100, is held at 100, and is density 100 at frame 2. | largest difference 2.2e-9 | yes |
| FX-SNOW-018 frame 2: Spacing 6, size 3, density keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the solid; frame 2 would pass 100, is held at 100, and is density 100 at frame 2. | largest difference 3.0e-8 | yes |
| FX-SNOW-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-019 frame 0: FX-SNOW-002 moved three pixels right: the planes are the drawing's own, so the snow moves with it, and the three columns left bare stay empty. | largest difference 3.0e-8 | yes |
| FX-SNOW-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-020 frame 0: FX-SNOW-002 on Noise's card: the same flakes, each pixel that shows mixed toward the snow as much as on the solid, the soft edge at its own half covering, and the empty column and row stay empty. | largest difference 1.9e-7 | yes |
| FX-SNOW-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SNOW-021 frame 0: Density 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-021 frame 4: Density 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-022 frame 0: Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-022 frame 4: Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-023 frame 0: Size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-023 frame 4: Size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-024 frame 0: Scene depth -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-024 frame 4: Scene depth -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-025 frame 0: Wind -1001, past -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-025 frame 4: Wind -1001, past -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-026 frame 0: Wiggle 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-026 frame 4: Wiggle 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-027 frame 0: Period 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-027 frame 4: Period 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-028 frame 0: A colour written "#fffff", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-028 frame 4: A colour written "#fffff", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SNOW-029 frame 0: Opacity keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-029 frame 4: Opacity keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 2.2e-9 | yes |
| FX-SNOW-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 100 grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the spacing, size, speed, wind and wiggle, and nothing else | Snowfall { color: "#ffffff", density: 50.0, spacing: 16.0, size: 3.0, depth: 50.0, speed: 1.0, wind: -0.5, wiggle: 1.5, period: 48.0, seed: 5.0, opacity: 100.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_snow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_snow_016.json, its colour written in capitals, is saved in small letters, as Rain's is | "#a0c8ff" | yes |
| a file with no `wind` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a size that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| density 101 is refused with a sentence, and nothing changes | Snowfall's density runs from 0 to 100, and this is 101. | yes |
| spacing 1 is refused with a sentence, and nothing changes | Snowfall's spacing runs from 2 to 1000, and this is 1. | yes |
| size 101 is refused with a sentence, and nothing changes | Snowfall's size runs from 0 to 100, and this is 101. | yes |
| scene depth -1 is refused with a sentence, and nothing changes | Snowfall's depth runs from 0 to 100, and this is -1. | yes |
| speed 1001 is refused with a sentence, and nothing changes | Snowfall's speed runs from 0 to 1000, and this is 1001. | yes |
| wind -1001 is refused with a sentence, and nothing changes | Snowfall's wind runs from -1000 to 1000, and this is -1001. | yes |
| wiggle 101 is refused with a sentence, and nothing changes | Snowfall's wiggle runs from 0 to 100, and this is 101. | yes |
| period 0.5 is refused with a sentence, and nothing changes | Snowfall's period runs from 1 to 1000, and this is 0.5. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Snowfall's seed runs from 0 to 100000, and this is 100001. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Snowfall's opacity runs from 0 to 100, and this is 101. | yes |
| colour "#fffff" is refused with a sentence, and nothing changes | Snowfall's colour is written #rrggbb, and this is "#fffff". | yes |
| colour "white" is refused with a sentence, and nothing changes | Snowfall's colour is written #rrggbb, and this is "white". | yes |
| opacity keyed to 150 is refused with a sentence, and nothing changes | Snowfall's opacity runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| opacity keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_snow_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_snow_010.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_snow_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: snow on a night sky, in `verification/B-139 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| as it starts, frame 0: snow lights some of the sky, and nothing shows outside it | 437 of 9800 sky pixels lit; spilt: false | yes |
| as it starts, frame 24: snow lights some of the sky, and nothing shows outside it | 428 of 9800 sky pixels lit; spilt: false | yes |
| scene depth 0: snow lights some of the sky, and nothing shows outside it | 398 of 9800 sky pixels lit; spilt: false | yes |
| scene depth 100: snow lights some of the sky, and nothing shows outside it | 464 of 9800 sky pixels lit; spilt: false | yes |
| a blizzard: density 100, spacing 12, size 3, wind 4, speed 6: snow lights some of the sky, and nothing shows outside it | 1840 of 9800 sky pixels lit; spilt: false | yes |
| frame 24 is not frame 0: the snow has fallen on | compared | yes |
| the blizzard lights more of the sky than the start | 1840 against 437 | yes |

## Result

125 of 125 checks pass.
