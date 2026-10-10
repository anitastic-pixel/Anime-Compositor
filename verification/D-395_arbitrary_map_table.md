# D-395: Arbitrary Map

B-274, after After Effects' PS Arbitrary Map: a Photoshop arbitrary map (.amp) read as Adobe's file format specification gives it, the colour tables applied by Color Lookup's 1D rule (each channel's table, then the master's), Phase cycling every table, and the alpha table, with Apply Phase Map To Alpha on, as Curves' alpha curve (D-302). Every expected pixel and reason is `Fixtures/arbitrary_map/expected_arbitrary_map.json`, written by `tools/arbitrary_map_reference.py` before this code existed and printed in document 25 as FX-AMAP-001 to 022. Tolerance 2e-5.

## FX-AMAP-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AMAP-001 frame 0: straight_256.amp, one straight table: the drawing, unchanged but for rounding. | largest difference 1.9e-7 | yes |
| FX-AMAP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-002 frame 0: master_256.amp, one table, the master, a lightening curve (entry x is 255 (x / 255)^0.6): every colour channel through it, black kept, the grey #808080 to #a9a9a9, the blue #3a6fd8 to #699be7. | largest difference 9.3e-8 | yes |
| FX-AMAP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-003 frame 0: rgb_768.amp, three tables, red, green and blue, the master left straight: red inverted, green flattened to between 64 and 191, blue along a quarter sine; white to #00bfff, black to #ff4000. | largest difference 8.2e-8 | yes |
| FX-AMAP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-004 frame 0: master_rgb_1024.amp, four tables, the master (an S-curve) then red (a power of 1.5), green (inverted) and blue (lifted to 30 and narrowed): each channel through its own table, then the master. | largest difference 1.4e-7 | yes |
| FX-AMAP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-005 frame 0: alpha_1280.amp, five tables, FX-AMAP-004's four and an alpha table (the square root), Apply Phase Map To Alpha off: FX-AMAP-004's picture; alpha untouched. | largest difference 1.4e-7 | yes |
| FX-AMAP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-006 frame 0: FX-AMAP-005 with Apply Phase Map To Alpha on: the colours as FX-AMAP-005, and the covering through the fifth table, the half-covered column to 181 of 255 and the quarter one to 128, the straight colour kept. | largest difference 1.4e-7 | yes |
| FX-AMAP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-007 frame 0: two_512.amp, two tables: the master (inverted) and red (squared); green and blue straight, so every channel is inverted and red squared first. | largest difference 3.2e-8 | yes |
| FX-AMAP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-008 frame 0: FX-AMAP-002 with phase 64: the master cycled 64 levels right, so entry x is what entry x - 64 was, wrapping: black (entry 0) takes entry 192's value, and the grey takes entry 64's. | largest difference 2.0e-7 | yes |
| FX-AMAP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-009 frame 0: FX-AMAP-003 with phase -100.25: each of the three tables cycled left, between entries mixed; the master, which the file does not hold, stays straight. | largest difference 1.4e-7 | yes |
| FX-AMAP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-010 frame 0: FX-AMAP-002 with the phase keyed from 0 at frame 0 to 96 at frame 4, linear: frame 0 FX-AMAP-002, frame 2 cycled 48, frame 4 cycled 96. | largest difference 9.3e-8 | yes |
| FX-AMAP-010 frame 2: FX-AMAP-002 with the phase keyed from 0 at frame 0 to 96 at frame 4, linear: frame 0 FX-AMAP-002, frame 2 cycled 48, frame 4 cycled 96. | largest difference 1.5e-7 | yes |
| FX-AMAP-010 frame 4: FX-AMAP-002 with the phase keyed from 0 at frame 0 to 96 at frame 4, linear: frame 0 FX-AMAP-002, frame 2 cycled 48, frame 4 cycled 96. | largest difference 1.6e-7 | yes |
| FX-AMAP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-011 frame 0: FX-AMAP-002 with Apply Phase Map To Alpha on: master_256.amp has no alpha table, so alpha is left as it is: FX-AMAP-002's picture. | largest difference 9.3e-8 | yes |
| FX-AMAP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-012 frame 0: FX-AMAP-006 with phase 100: the alpha table cycled with the others, so a pixel that did not show (entry 0 takes entry 156's value, 199) now shows, black, at 199 of 255. | largest difference 1.2e-7 | yes |
| FX-AMAP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-013 frame 0: No map file chosen, the effect as it is added: the drawing, untouched, with no warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-014 frame 0: FX-AMAP-002 moved three pixels right: the same, moved. | largest difference 9.3e-8 | yes |
| FX-AMAP-014 frame 3: FX-AMAP-002 moved three pixels right: the same, moved. | largest difference 9.3e-8 | yes |
| FX-AMAP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AMAP-015 frame 0: The map asset's file maps/gone.amp is not there: the drawing, untouched, with MEDIA_MISSING when the file is opened and at every frame; the asset and the setting are kept, to relink. | largest difference 1.9e-7 | yes |
| FX-AMAP-015: what opening it warns of, and what frame 4 warns of | ["MEDIA_MISSING"] and ["MEDIA_MISSING"] | yes |
| FX-AMAP-016 frame 0: The setting names asset-nothing, which the project does not have: the drawing, untouched, with EFFECT_PARAMETER_INVALID; the setting is kept as written. | largest difference 1.9e-7 | yes |
| FX-AMAP-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AMAP-017 frame 0: The setting names asset-bands, the drawing, which is not a lookup file: the drawing, untouched, with EFFECT_PARAMETER_INVALID. | largest difference 1.9e-7 | yes |
| FX-AMAP-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AMAP-018 frame 0: Phase 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-018 frame 4: Phase 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AMAP-019 frame 0: Phase -256, below -255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-019 frame 4: Phase -256, below -255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AMAP-020 frame 0: Phase keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-020 frame 4: Phase keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AMAP-021 frame 0: Apply Phase Map To Alpha "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-021 frame 4: Apply Phase Map To Alpha "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AMAP-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-AMAP-022: a map file the reading rule refuses

| Check | The build's answer | Matches |
| --- | --- | --- |
| opening it says nothing: the file is read at the frame, not on opening | [] | yes |
| frame 0: the drawing, untouched | largest difference 1.9e-7 | yes |
| frame 0 says MEDIA_DECODE_FAILED, once | ["MEDIA_DECODE_FAILED"] | yes |
| frame 4: the drawing, untouched | largest difference 1.9e-7 | yes |
| frame 4 says MEDIA_DECODE_FAILED, once | ["MEDIA_DECODE_FAILED"] | yes |
| it names the file and why, and asks for a .amp file | ["The lookup file \"odd_300\" cannot be read, so the layer \"art\" is drawn without its look. Export the look again as a .amp file, or relink the asset to another one."] | yes |
| the asset and the setting are kept as written | "maps/refused/odd_300.amp" and "asset-map" | yes |

## The reading rule: each refused file, and its reason word for word

| Check | The build's answer | Matches |
| --- | --- | --- |
| refused/cube.amp is refused: the file is 135 bytes long, not a whole number of 256-byte tables | the file is 135 bytes long, not a whole number of 256-byte tables | yes |
| refused/empty.amp is refused: the file is empty | the file is empty | yes |
| refused/odd_300.amp is refused: the file is 300 bytes long, not a whole number of 256-byte tables | the file is 300 bytes long, not a whole number of 256-byte tables | yes |
| refused/six_1536.amp is refused: the file holds 6 tables, and this program reads at most five: master, red, green, blue and alpha | the file holds 6 tables, and this program reads at most five: master, red, green, blue and alpha | yes |
| a map file changed on disk is read afresh, not kept from before | Ok("read"), then Err("the file is 300 bytes long, not a whole number of 256-byte tables") | yes |
| a .cube file is not read as a map, nor a map as a .cube file |  | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing, and a half-size draft leaves it as it is | 0, ArbitraryMap { map: "asset-map", phase: 64.0, apply_to_alpha: "on", table: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_amap_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_amap_012.json is saved with its map, phase and alpha setting | {"apply_to_alpha":"on","map":"asset-map","phase":100} | yes |
| fx_amap_018.json is refused in a sentence | Arbitrary Map's phase runs from -255 to 255, and this is 256. | yes |
| fx_amap_019.json is refused in a sentence | Arbitrary Map's phase runs from -255 to 255, and this is -256. | yes |
| fx_amap_021.json is refused in a sentence | Arbitrary Map's apply phase map to alpha is "off" or "on", and this is "yes". | yes |
| a file with an Arbitrary Map with no `apply_to_alpha` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Arbitrary Map with no `map` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| naming asset-nothing, which the project does not have, is refused with a sentence, and nothing changes | Arbitrary Map cannot use asset-nothing: it is not a lookup file of this project. | yes |
| naming asset-bands, a drawing and not a map file, is refused with a sentence, and nothing changes | Arbitrary Map cannot use asset-bands: it is not a lookup file of this project. | yes |
| phase 255.5 is refused with a sentence, and nothing changes | Arbitrary Map's phase runs from -255 to 255, and this is 255.5. | yes |
| apply phase map to alpha "yes" is refused with a sentence, and nothing changes | Arbitrary Map's apply phase map to alpha is "off" or "on", and this is "yes". | yes |
| phase keyed to -300 is refused with a sentence, and nothing changes | Arbitrary Map's phase runs from -255 to 255, and this is -300. | yes |
| no file chosen, is taken | taken | yes |
| phase -255, alpha on is taken | taken | yes |
| phase keyed from 0 to 255 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Choosing a file on the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AMAP-013 with master_256.amp chosen is FX-AMAP-002's frame | largest difference 9.3e-8 | yes |
| choosing rgb_768.amp, a file the project does not have yet, brings it in and uses it: FX-AMAP-003's frame | taken, largest difference 8.2e-8 | yes |
| one undo takes back both the new file and the setting | the frame and the asset list as they were | yes |

## Collect Files

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AMAP-002 collected: the map file is copied, listed as kind lut and used by the layer whose effect names it | [("media/asset-bands/bands.png", "copied"), ("media/asset-map/master_256.amp", "copied")], Some((String("lut"), Array [Object {"composition": String("comp-main"), "layer": String("art")}])) | yes |
| the collected project draws the same frame from its copy | the same frame | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_amap_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_amap_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_amap_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_amap_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_amap_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_amap_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_amap_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_amap_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, master_rgb_1024.amp as it starts: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, master_rgb_1024.amp as it starts on three layers, frame 0, Full | largest difference 1 of 255, 2530 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, master_rgb_1024.amp as it starts on three layers, frame 100, Full | largest difference 1 of 255, 1148 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, master_rgb_1024.amp as it starts on three layers, frame 239, Full | largest difference 1 of 255, 1748 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, master_rgb_1024.amp as it starts on three layers, frame 0, Draft | largest difference 1 of 255, 155 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, master_rgb_1024.amp as it starts on three layers, frame 100, Draft | largest difference 1 of 255, 111 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, master_rgb_1024.amp as it starts on three layers, frame 239, Draft | largest difference 1 of 255, 133 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, rgb_768.amp, phase -100.25: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, rgb_768.amp, phase -100.25 on three layers, frame 0, Full | largest difference 1 of 255, 31653 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, rgb_768.amp, phase -100.25 on three layers, frame 100, Full | largest difference 1 of 255, 28147 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, rgb_768.amp, phase -100.25 on three layers, frame 239, Full | largest difference 1 of 255, 31235 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, rgb_768.amp, phase -100.25 on three layers, frame 0, Draft | largest difference 1 of 255, 1584 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, rgb_768.amp, phase -100.25 on three layers, frame 100, Draft | largest difference 1 of 255, 1183 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, rgb_768.amp, phase -100.25 on three layers, frame 239, Draft | largest difference 1 of 255, 1556 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off on three layers, frame 0, Full | largest difference 1 of 255, 5055 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off on three layers, frame 100, Full | largest difference 1 of 255, 4363 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off on three layers, frame 239, Full | largest difference 1 of 255, 4265 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off on three layers, frame 0, Draft | largest difference 1 of 255, 1143 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off on three layers, frame 100, Draft | largest difference 1 of 255, 775 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, alpha_1280.amp, phase 64, alpha off on three layers, frame 239, Draft | largest difference 1 of 255, 1100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-395 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_brighter.png, master_256.amp, the master raised (t to the 0.6): the street brighter, the colours kept; draws cleanly and changes the picture | [], 129600 pixels changed | yes |
| 3_rgb.png, rgb_768.amp, red inverted, green halved, blue on a sine: strange colours; draws cleanly and changes the picture | [], 129600 pixels changed | yes |
| 4_four_tables.png, master_rgb_1024.amp, an S-curve master over three channel tables; draws cleanly and changes the picture | [], 129600 pixels changed | yes |
| 5_phase_64.png, the same, phase 64: every table cycled a quarter, the colours shifted again; draws cleanly and changes the picture | [], 129600 pixels changed | yes |
| 6_phase_minus_128.png, the same, phase -128: half way round; draws cleanly and changes the picture | [], 129600 pixels changed | yes |
| 7_alpha_on.png, alpha_1280.amp with Apply Phase Map To Alpha on: the square-root alpha table leaves the street's covering whole; draws cleanly and changes the picture | [], 129600 pixels changed | yes |
| 4_four_tables.png and 5_phase_64.png differ, so Phase changes the picture |  | yes |

## Result

161 of 161 checks pass.
