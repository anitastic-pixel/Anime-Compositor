# D-505: Unmult

B-385, after After Effects' Unmult (Keying, new in After Effects 26.0): a black or white background turned into transparency. Adobe names the controls but publishes no formula and no defaults we found, so the rule and the settings when added are ours, stated in `tools/unmult_reference.py`: the covering becomes how far the pixel is from the background (over black its brightest channel, over white how far its darkest channel falls below white), less the level, over the softness; Remove Color Matting restores the colour from before the background. Worked in linear light. Every expected pixel is `Fixtures/unmult/expected_unmult.json`, written by the tool before this code existed and printed in document 25 as FX-UNMULT-001 to 019. Tolerance 2e-5.

## FX-UNMULT-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-UNMULT-001 frame 0: The settings as they start: background black, black level 0, softness 0, remove color matting on, clip HDR results off. Every pixel's covering becomes its brightest channel in linear light, so black is gone, white stays whole and a mid grey keeps about a fifth; the colour over black is exactly as it was. | largest difference 1.2e-7 | yes |
| FX-UNMULT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-002 frame 0: Black level 10, softness 0: every pixel whose brightest channel is 10% or less is gone too, the rest as FX-UNMULT-001. | largest difference 1.2e-7 | yes |
| FX-UNMULT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-003 frame 0: Black level 10, softness 20: those pixels fade in over the next 20% instead of starting at once. | largest difference 1.7e-7 | yes |
| FX-UNMULT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-004 frame 0: Remove color matting off: the same coverings as FX-UNMULT-001, but each pixel keeps its own colour, so the part-covered ones look darker over black than before. | largest difference 1.4e-7 | yes |
| FX-UNMULT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-005 frame 0: Background white, white level 100: white is gone, black stays whole, a colour's covering is how far its darkest channel is below white; the colour over white is exactly as it was. | largest difference 1.3e-7 | yes |
| FX-UNMULT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-006 frame 0: Background white, white level 80, softness 20: the pixels within 20% of white in their darkest channel go, the next 20% fade in. | largest difference 1.3e-7 | yes |
| FX-UNMULT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-007 frame 0: Background white, remove color matting off: FX-UNMULT-005's coverings with each pixel's own colour. | largest difference 1.4e-7 | yes |
| FX-UNMULT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-008 frame 0: Softness 50 at black level 0: every covering is the brightest channel times the brightest channel over half, so the darker pixels thin out faster. | largest difference 1.4e-7 | yes |
| FX-UNMULT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-009 frame 0: Black level 100, softness 10: no pixel is brighter than it, so the whole layer is gone. | largest difference 0.0e0 | yes |
| FX-UNMULT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-010 frame 0: An Exposure of 2 stops first, so the brighter pixels run past 1, then the settings as they start: a pixel brighter than 1 stays whole with its colour past 1 kept. | largest difference 5.0e-7 | yes |
| FX-UNMULT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-011 frame 0: The same with clip HDR results on: the colours past 1 are held at 1, so white comes back to white, the brightest colours lose their brightness. | largest difference 2.9e-7 | yes |
| FX-UNMULT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-012 frame 0: Black level keyed from 0 at frame 0 to 40 at frame 4, linear, softness 10. | largest difference 1.2e-7 | yes |
| FX-UNMULT-012 frame 2: Black level keyed from 0 at frame 0 to 40 at frame 4, linear, softness 10. | largest difference 2.6e-7 | yes |
| FX-UNMULT-012 frame 4: Black level keyed from 0 at frame 0 to 40 at frame 4, linear, softness 10. | largest difference 3.6e-7 | yes |
| FX-UNMULT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-013 frame 2: Softness keyed from 0 at frame 0 to 40 at frame 4, linear, background white, white level 90. | largest difference 1.3e-7 | yes |
| FX-UNMULT-013 frame 4: Softness keyed from 0 at frame 0 to 40 at frame 4, linear, background white, white level 90. | largest difference 2.6e-7 | yes |
| FX-UNMULT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-014 frame 0: FX-UNMULT-003 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-UNMULT-014 frame 3: FX-UNMULT-003 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-UNMULT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UNMULT-015 frame 0: Black level 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-015 frame 4: Black level 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UNMULT-016 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-016 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UNMULT-017 frame 0: Background color "grey": not black or white. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-017 frame 4: Background color "grey": not black or white. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UNMULT-018 frame 0: Remove color matting "yes": not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-018 frame 4: Remove color matting "yes": not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UNMULT-019 frame 0: Clip HDR results "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-019 frame 4: Clip HDR results "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-UNMULT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_unmult_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_unmult_003.json is saved with its six settings, background black, black level 10, softness 20, remove color matting on | {"background_color":"black","black_level":10,"clip_hdr_results":"off","remove_color_matting":"on","softness":20,"white_level":100} | yes |
| fx_unmult_015.json is refused in a sentence naming black level | Unmult's black level runs from 0 to 100, and this is 101. | yes |
| fx_unmult_016.json is refused in a sentence naming softness | Unmult's softness runs from 0 to 100, and this is -1. | yes |
| fx_unmult_017.json is refused in a sentence naming background color | Unmult's background color is "black" or "white", and this is "grey". | yes |
| fx_unmult_018.json is refused in a sentence naming remove color matting | Unmult's remove color matting is "on" or "off", and this is "yes". | yes |
| fx_unmult_019.json is refused in a sentence naming clip HDR results | Unmult's clip HDR results is "off" or "on", and this is "On". | yes |
| a file with an Unmult whose softness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Unmult without its background colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Unmult without its white level is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| black level 101 is refused with a sentence, and nothing changes | Unmult's black level runs from 0 to 100, and this is 101. | yes |
| background color "grey" is refused with a sentence, and nothing changes | Unmult's background color is "black" or "white", and this is "grey". | yes |
| softness keyed to 300 is refused with a sentence, and nothing changes | Unmult's softness runs from 0 to 100, and this is 300. | yes |
| background white, white level 90 is taken | taken | yes |
| black level keyed from 0 to 40 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_unmult_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_unmult_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_unmult_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_unmult_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_unmult_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_unmult_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_unmult_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_unmult_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Unmult as added (black): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1661377 pixels changed | yes |
| the reference shot, Unmult as added (black) on three layers, frame 0, Full | largest difference 1 of 255, 290 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult as added (black) on three layers, frame 100, Full | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult as added (black) on three layers, frame 239, Full | largest difference 1 of 255, 281 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult as added (black) on three layers, frame 0, Draft | largest difference 1 of 255, 84 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult as added (black) on three layers, frame 100, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult as added (black) on three layers, frame 239, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1730430 pixels changed | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off on three layers, frame 0, Full | largest difference 1 of 255, 419 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off on three layers, frame 100, Full | largest difference 1 of 255, 297 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off on three layers, frame 239, Full | largest difference 1 of 255, 358 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off on three layers, frame 0, Draft | largest difference 1 of 255, 1054 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off on three layers, frame 100, Draft | largest difference 1 of 255, 954 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult white, white level 80, softness 20, remove color matting off on three layers, frame 239, Draft | largest difference 1 of 255, 1078 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1661377 pixels changed | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on on three layers, frame 0, Full | largest difference 1 of 255, 114 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on on three layers, frame 100, Full | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on on three layers, frame 239, Full | largest difference 1 of 255, 141 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on on three layers, frame 0, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on on three layers, frame 100, Draft | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Unmult black level 20, softness 10, clip HDR results on on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-505 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_flare_before.png, a flare shot on black (white core, orange glow) and a mid-grey square; 1b lays it over a sky as it is, so the black hides the sky; draws cleanly | [] | yes |
| 2_flare_unmult.png, as added: the black see-through, the core whole and white, the glow pixel covering 9 of 255 at straight (255, 175, 90), the grey square covering 55 at straight white; 2b over the sky shows the glow with no black box; draws cleanly | [], black [0, 0, 0, 0], core [255, 255, 255, 255], glow [255, 175, 90, 9], grey [255, 255, 255, 55] | yes |
| 3_black_level_10_softness_20.png: the faint outer glow gone (the glow pixel covering 0), the core whole, the grey square covering 32 by hand; draws cleanly | [], core [255, 255, 255, 255], glow [0, 0, 0, 0], grey [255, 255, 255, 32] | yes |
| 4_remove_color_matting_off.png: the same coverings, but each pixel keeps its own colour, so the glow and the grey stay dark (the grey straight 128 at covering 55, the glow straight (53, 33, 12) at 9); over the sky the glow looks dirtier; draws cleanly | [], glow [53, 33, 12, 9], grey [128, 128, 128, 55] | yes |
| 5_smoke_before.png, dark smoke on white; 5b with background white: the white see-through, the smoke's middle covering 245 at straight black, the grey square covering 200 at straight black, by hand; draws cleanly | [] [], white [0, 0, 0, 0], smoke [0, 0, 0, 245], grey [0, 0, 0, 200] | yes |

## Result

132 of 132 checks pass.
