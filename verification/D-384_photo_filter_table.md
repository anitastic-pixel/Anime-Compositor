# D-384: Photo Filter

B-263, after After Effects' Photo Filter: the picture as if shot through a coloured glass filter, picked from six presets or given as a colour, its Density saying how strongly it tints, Preserve Luminosity keeping each pixel's brightness. Every expected pixel is `Fixtures/photo_filter/expected_photo_filter.json`, written by `tools/photo_filter_reference.py` before this code existed and printed in document 25 as FX-PFILT-001 to 020. Tolerance 2e-5.

## FX-PFILT-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PFILT-001 frame 0: The settings as they start: Warming Filter (85), density 25, preserve luminosity on: every colour a little warmer, its brightness kept; black stays black, white turns a pale warm white. | largest difference 1.7e-7 | yes |
| FX-PFILT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-002 frame 0: The same with preserve luminosity off: warmer and a little darker, since the orange glass holds back some of the green and blue. | largest difference 1.8e-7 | yes |
| FX-PFILT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-003 frame 0: Density 100, preserve luminosity on: the full filter, the brightness kept; pure blue, which the orange glass stops entirely, goes black. | largest difference 2.7e-7 | yes |
| FX-PFILT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-004 frame 0: Density 100, preserve luminosity off: each channel multiplied by the filter's colour, white becomes the filter's orange itself. | largest difference 1.5e-7 | yes |
| FX-PFILT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-005 frame 0: Density 0: no filter, the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-PFILT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-006 frame 0: Warming Filter (81), density 50: a yellower warming. | largest difference 1.9e-7 | yes |
| FX-PFILT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-007 frame 0: Cooling Filter (80), density 50: everything bluer, the brightness kept. | largest difference 2.5e-7 | yes |
| FX-PFILT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-008 frame 0: Cooling Filter (82), density 50: a paler, cyan cooling. | largest difference 2.6e-7 | yes |
| FX-PFILT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-009 frame 0: Sepia, density 60: a brown tint, the brightness kept. | largest difference 1.9e-7 | yes |
| FX-PFILT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-010 frame 0: Underwater, density 60: a green-blue tint, the reds pulled down. | largest difference 2.2e-7 | yes |
| FX-PFILT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-011 frame 0: Custom magenta #ff00ff, density 50, preserve luminosity off: the green channel halved, red and blue kept. | largest difference 1.9e-7 | yes |
| FX-PFILT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-012 frame 0: Warming Filter (85) with the colour set to magenta: the colour is used only for a custom filter, so the same as FX-PFILT-001. | largest difference 1.7e-7 | yes |
| FX-PFILT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-013 frame 0: Density keyed from 0 at frame 0 to 100 at frame 4, linear, preserve luminosity off: frame 0 untouched, frame 2 halfway, frame 4 the full filter as FX-PFILT-004. | largest difference 1.9e-7 | yes |
| FX-PFILT-013 frame 2: Density keyed from 0 at frame 0 to 100 at frame 4, linear, preserve luminosity off: frame 0 untouched, frame 2 halfway, frame 4 the full filter as FX-PFILT-004. | largest difference 1.5e-7 | yes |
| FX-PFILT-013 frame 4: Density keyed from 0 at frame 0 to 100 at frame 4, linear, preserve luminosity off: frame 0 untouched, frame 2 halfway, frame 4 the full filter as FX-PFILT-004. | largest difference 1.5e-7 | yes |
| FX-PFILT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-014 frame 0: FX-PFILT-011 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-PFILT-014 frame 3: FX-PFILT-011 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-PFILT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PFILT-015 frame 0: Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-015 frame 4: Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PFILT-016 frame 0: Density 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-016 frame 4: Density 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PFILT-017 frame 0: Filter "warming", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-017 frame 4: Filter "warming", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PFILT-018 frame 0: Filter "Warming_85": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-018 frame 4: Filter "Warming_85": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PFILT-019 frame 0: Color "#fff", which is not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-019 frame 4: Color "#fff", which is not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PFILT-020 frame 0: Preserve luminosity "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-020 frame 4: Preserve luminosity "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PFILT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pfilt_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pfilt_011.json is saved with its four settings | {"color":"#ff00ff","density":50,"filter":"custom","preserve_luminosity":"off"} | yes |
| fx_pfilt_015.json is refused in a sentence naming density | Photo Filter's density runs from 0 to 100, and this is -1. | yes |
| fx_pfilt_016.json is refused in a sentence naming density | Photo Filter's density runs from 0 to 100, and this is 101. | yes |
| fx_pfilt_017.json is refused in a sentence naming filter | Photo Filter's filter is one of "warming_85", "warming_81", "cooling_80", "cooling_82", "sepia", "underwater" or "custom", and this is "warming". | yes |
| fx_pfilt_018.json is refused in a sentence naming filter | Photo Filter's filter is one of "warming_85", "warming_81", "cooling_80", "cooling_82", "sepia", "underwater" or "custom", and this is "Warming_85". | yes |
| fx_pfilt_019.json is refused in a sentence naming colour | Photo Filter's colour is written #rrggbb, and this is "#fff". | yes |
| fx_pfilt_020.json is refused in a sentence naming preserve luminosity | Photo Filter's preserve luminosity is "off" or "on", and this is "yes". | yes |
| a file with a Photo Filter whose density is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Photo Filter without its filter is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| density 101 is refused with a sentence, and nothing changes | Photo Filter's density runs from 0 to 100, and this is 101. | yes |
| filter "warm" is refused with a sentence, and nothing changes | Photo Filter's filter is one of "warming_85", "warming_81", "cooling_80", "cooling_82", "sepia", "underwater" or "custom", and this is "warm". | yes |
| density keyed to 150 is refused with a sentence, and nothing changes | Photo Filter's density runs from 0 to 100, and this is 150. | yes |
| Cooling Filter (80) at density 80 is taken | taken | yes |
| density keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pfilt_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pfilt_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pfilt_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pfilt_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pfilt_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pfilt_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_pfilt_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073528 pixels changed | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25 on three layers, frame 0, Full | largest difference 1 of 255, 554 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25 on three layers, frame 100, Full | largest difference 1 of 255, 603 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25 on three layers, frame 239, Full | largest difference 1 of 255, 597 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25 on three layers, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25 on three layers, frame 100, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter as added, Warming Filter (85) at 25 on three layers, frame 239, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept on three layers, frame 0, Full | largest difference 1 of 255, 3615 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept on three layers, frame 100, Full | largest difference 1 of 255, 182 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept on three layers, frame 239, Full | largest difference 1 of 255, 1037 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept on three layers, frame 0, Draft | largest difference 1 of 255, 93 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept on three layers, frame 100, Draft | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter Cooling Filter (80) at 70, luminosity kept on three layers, frame 239, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off on three layers, frame 0, Full | largest difference 1 of 255, 282970 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off on three layers, frame 100, Full | largest difference 1 of 255, 267264 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off on three layers, frame 239, Full | largest difference 1 of 255, 279447 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off on three layers, frame 0, Draft | largest difference 1 of 255, 8043 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off on three layers, frame 100, Draft | largest difference 1 of 255, 7431 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Photo Filter custom #ff00ff at 50, luminosity off on three layers, frame 239, Draft | largest difference 1 of 255, 7867 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-384 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| density 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_warming_85.png, as added, Warming Filter (85) at 25: the street warmer; draws cleanly | [], red less blue -34.0 before, 4.2 after | yes |
| 3_cooling_80.png, Cooling Filter (80) at 60: the street cooler; draws cleanly | [], red less blue -34.0 before, -124.2 after | yes |
| 4_sepia.png, Sepia at 80: the street brown; draws cleanly | [], red less blue -34.0 before, 59.7 after | yes |
| 5_underwater.png, Underwater at 70: the street green-blue, the reds pulled down; draws cleanly | [], red less blue -34.0 before, -122.2 after | yes |
| 6_custom_blue_unkept.png, a custom blue #3366cc at 60, luminosity off: the street bluer and darker, no channel brighter; draws cleanly | [], no channel brighter: true | yes |

## Result

137 of 137 checks pass.
