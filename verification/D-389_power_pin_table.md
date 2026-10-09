# D-389: Power Pin

B-268, after CycoreFX's CC Power Pin: the four corners pinned as Corner Pin pins them, Perspective easing from a flat stretch (0) to full perspective (100), the pinned shape grown or shrunk past its sides by the four expansions, and Unstretch drawing the other way round, the pinned shape stretched back out to fill the layer. The formulas are this program's own. Every expected pixel is `Fixtures/power_pin/expected_power_pin.json`, written by `tools/power_pin_reference.py` before this code existed and printed in document 25 as FX-POWERPIN-001 to 025. Tolerance 2e-5.

## FX-POWERPIN-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-POWERPIN-001 frame 0: The settings as they start: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-002 frame 0: Shrunk to the middle, the pins at (25, 25), (75, 25), (25, 75) and (75, 75): the drawing at half its size in the middle, as Corner Pin draws it. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-003 frame 0: A keystone, Top Left at (25, 0) and Top Right at (75, 0), Perspective 100: the drawing leans back as Corner Pin leans it, its far upper half drawn smaller, so the blue band lands in rows 2 and 3. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-004 frame 0: The same keystone at Perspective 0: squeezed evenly instead, every row as tall as before, so the band stays in rows 4 and 5. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-005 frame 0: The same keystone at Perspective 50: half way between. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-006 frame 0: Unstretch on, the pins shrunk to the middle: the other way round, the middle of the drawing stretched out to fill it, twice the size. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-007 frame 0: Unstretch on with the keystone: the keystone's shape stretched out to fill the drawing, its top widened. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-008 frame 0: The pins shrunk to the middle and every expansion 50: the pinned shape grown back out by half of it on each side, which is the whole drawing: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-009 frame 0: Expansion Left -25 with the pins where they start: the left side pulled in a quarter of the way, the drawing squeezed into columns 4 to 15. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-010 frame 0: Expansion Right 50 and Bottom 50 with the pins where they start: the drawing grown half as much again to the right and down from its top left corner; the layer grows to hold it. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-011 frame 0: Top Right at (100, 100) and Bottom Right at (100, 0), crossed like a bow tie: no drawable shape, so the frame is clear. | largest difference 0.0e0 | yes |
| FX-POWERPIN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-012 frame 0: The left side squeezed small and far away, Top Left at (0, 45) and Bottom Left at (0, 55), so the right side is the near one, and Expansion Right 100 at Perspective 100: the near side grown by the drawing's whole width would run past the vanishing line, so the frame is clear. | largest difference 0.0e0 | yes |
| FX-POWERPIN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-013 frame 0: The same at Perspective 0: squeezed evenly there is no vanishing line, and the grown shape is drawn. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-014 frame 0: The keystone with Perspective keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 FX-POWERPIN-004, frame 2 FX-POWERPIN-005 and frame 4 FX-POWERPIN-003. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-014 frame 2: The keystone with Perspective keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 FX-POWERPIN-004, frame 2 FX-POWERPIN-005 and frame 4 FX-POWERPIN-003. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-014 frame 4: The keystone with Perspective keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 FX-POWERPIN-004, frame 2 FX-POWERPIN-005 and frame 4 FX-POWERPIN-003. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-015 frame 0: Top Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4 at Perspective 0: frame 0 the drawing, then its right side ever shorter, squeezed evenly. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-015 frame 2: Top Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4 at Perspective 0: frame 0 the drawing, then its right side ever shorter, squeezed evenly. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-015 frame 4: Top Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4 at Perspective 0: frame 0 the drawing, then its right side ever shorter, squeezed evenly. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-016 frame 0: Top Left pulled out to (-25, 0), the layer moved three pixels right, Perspective 100: the layer grows four pixels each side as Corner Pin's FX-PIN-006 does, and the frame is Corner Pin's. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-017 frame 0: Expansion Top keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-017 frame 2: Expansion Top keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-017 frame 4: Expansion Top keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-018 frame 0: Unstretch on with the keystone at Perspective 0, the layer moved two pixels right: the evenly squeezed keystone stretched back out; nothing grows. | largest difference 2.5e-7 | yes |
| FX-POWERPIN-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POWERPIN-019 frame 0: Perspective 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-019 frame 4: Perspective 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POWERPIN-020 frame 0: Perspective -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-020 frame 4: Perspective -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POWERPIN-021 frame 0: Expansion Top 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-021 frame 4: Expansion Top 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POWERPIN-022 frame 0: Expansion Left -41, below -40. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-022 frame 4: Expansion Left -41, below -40. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POWERPIN-023 frame 0: Unstretch "yes", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-023 frame 4: Unstretch "yes", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POWERPIN-024 frame 0: Top Left at (-401, 0), its x below -400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-024 frame 4: Top Left at (-401, 0), its x below -400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POWERPIN-025 frame 0: Bottom Right keyed to (100, 600) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-025 frame 4: Bottom Right keyed to (100, 600) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POWERPIN-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_powerpin_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_powerpin_007.json is saved with its four pins, perspective, unstretch and expansions | {"bottom_left":[0,100],"bottom_right":[100,100],"expansion_bottom":0,"expansion_left":0,"expansion_right":0,"expansion_top":0,"perspective":100,"top_left":[25,0],"top_right":[75,0],"unstretch":"on"} | yes |
| fx_powerpin_019.json is refused in a sentence | Power Pin's perspective runs from 0 to 100, and this is 101. | yes |
| fx_powerpin_020.json is refused in a sentence | Power Pin's perspective runs from 0 to 100, and this is -1. | yes |
| fx_powerpin_021.json is refused in a sentence | Power Pin's expansion top runs from -40 to 100, and this is 101. | yes |
| fx_powerpin_022.json is refused in a sentence | Power Pin's expansion left runs from -40 to 100, and this is -41. | yes |
| fx_powerpin_023.json is refused in a sentence | Power Pin's unstretch is "on" or "off", and this is "yes". | yes |
| fx_powerpin_024.json is refused in a sentence | Power Pin's top left runs from -400 to 500, and this is -401. | yes |
| a file with a Power Pin with no `perspective` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Power Pin whose top right is three numbers is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| perspective 100.5 is refused with a sentence, and nothing changes | Power Pin's perspective runs from 0 to 100, and this is 100.5. | yes |
| expansion bottom -40.5 is refused with a sentence, and nothing changes | Power Pin's expansion bottom runs from -40 to 100, and this is -40.5. | yes |
| bottom left at 0, 500.5 is refused with a sentence, and nothing changes | Power Pin's bottom left runs from -400 to 500, and this is 500.5. | yes |
| unstretch "On", written with a capital is refused with a sentence, and nothing changes | Power Pin's unstretch is "on" or "off", and this is "On". | yes |
| expansion right keyed to 150 is refused with a sentence, and nothing changes | Power Pin's expansion right runs from -40 to 100, and this is 150. | yes |
| pins at 10, 5 / 90, 0 / 0, 100 / 100, 90, perspective 50, unstretched, expansions 10, -5, 20, 0 is taken | taken | yes |
| perspective keyed from 0 to 100 is taken | taken | yes |
| top right keyed from 100, 0 to 100, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_powerpin_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_powerpin_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_powerpin_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_powerpin_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_powerpin_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_powerpin_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_powerpin_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_powerpin_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_powerpin_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_powerpin_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_powerpin_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2042031 pixels changed | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0 on three layers, frame 0, Full | largest difference 1 of 255, 4227 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0 on three layers, frame 100, Full | largest difference 1 of 255, 751 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0 on three layers, frame 239, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0 on three layers, frame 0, Draft | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0 on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin a keystone, top pins at 25, 0 and 75, 0 on three layers, frame 239, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2015365 pixels changed | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top on three layers, frame 0, Full | largest difference 1 of 255, 5115 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top on three layers, frame 100, Full | largest difference 1 of 255, 720 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top on three layers, frame 239, Full | largest difference 1 of 255, 1171 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top on three layers, frame 0, Draft | largest difference 1 of 255, 154 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top on three layers, frame 100, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin the same keystone at perspective 30, expanded 20 at the top on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2016947 pixels changed | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120 on three layers, frame 0, Full | largest difference 1 of 255, 2574 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120 on three layers, frame 100, Full | largest difference 1 of 255, 1101 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120 on three layers, frame 239, Full | largest difference 1 of 255, 1975 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120 on three layers, frame 0, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120 on three layers, frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin pins pulled past the frame, -20, -10 and 110, 120 on three layers, frame 239, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin unstretched from a tilted square: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1980516 pixels changed | yes |
| the reference shot, Power Pin unstretched from a tilted square on three layers, frame 0, Full | largest difference 1 of 255, 1873 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin unstretched from a tilted square on three layers, frame 100, Full | largest difference 1 of 255, 1057 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin unstretched from a tilted square on three layers, frame 239, Full | largest difference 1 of 255, 741 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin unstretched from a tilted square on three layers, frame 0, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin unstretched from a tilted square on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Power Pin unstretched from a tilted square on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-389 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts, each pin at its own corner: nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_keystone.png, a keystone: the street leaning back, its top narrower, the top corners clear; draws cleanly | [], 100191 pixels changed | yes |
| 4_keystone_flat.png, the same keystone at perspective 0: squeezed evenly instead; draws cleanly | [], 51836 pixels changed | yes |
| 5_expanded.png, pinned small in the middle, expanded 50 left and right: a wide band across the middle; draws cleanly | [], 119916 pixels changed | yes |
| 6_unstretched.png, unstretch with the keystone: the street stretched the other way, its top widened; draws cleanly | [], 110761 pixels changed | yes |

## Result

171 of 171 checks pass.
