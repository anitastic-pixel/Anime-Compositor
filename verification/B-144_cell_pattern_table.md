# B-144: Cell Pattern

D-209, accepted on 2026-09-28 with the After Effects picks (B7). Every expected pixel is `Fixtures/cell_pattern/expected_cell_pattern.json`, written by `tools/cell_pattern_reference.py` before this code existed and printed in document 25 as FX-CELL-001 to 033. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CELL-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CELL-001 frame 0: Bubbles, four pixels a cell, the other settings as they start: invert off, contrast 100, disperse 1, evolution 0, seed 0, black to white, opacity 100, normal. The card is covered by grey balls, white at each cell's point and dark where cells meet, the drawing gone under them, every pixel at its own covering and the empty pixels empty. | largest difference 3.0e-8 | yes |
| FX-CELL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-002 frame 0: Crystals: faceted cones, darker than the bubbles away from the points. | largest difference 3.0e-8 | yes |
| FX-CELL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-003 frame 0: Plates: flat white plates, dark only close to the seams. | largest difference 3.0e-8 | yes |
| FX-CELL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-004 frame 0: Static plates: each cell one flat grey of its own. | largest difference 3.0e-8 | yes |
| FX-CELL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-005 frame 0: Invert on: FX-CELL-001 turned over, dark balls on light seams. | largest difference 3.8e-8 | yes |
| FX-CELL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-006 frame 0: Contrast 300: the greys pushed out towards black and white. | largest difference 3.0e-8 | yes |
| FX-CELL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-007 frame 0: Contrast 0: every pixel that shows the middle grey, halfway from black to white. | largest difference 3.0e-8 | yes |
| FX-CELL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-008 frame 0: Disperse 0: every point in its cell's middle, so the balls sit in a square grid, the pattern repeating every four pixels. | largest difference 3.0e-8 | yes |
| FX-CELL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-009 frame 0: Disperse 0 and evolution 90: a point in its cell's middle has no circle to go round, so the frame is FX-CELL-008's. | largest difference 3.0e-8 | yes |
| FX-CELL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-010 frame 0: Evolution 90: the points turned a quarter of the way round their circles, the balls moved. | largest difference 3.0e-8 | yes |
| FX-CELL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-011 frame 0: Seed 7: other points, other balls. | largest difference 3.2e-8 | yes |
| FX-CELL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-012 frame 0: Seed 7.9: the whole part counted, so the frame is FX-CELL-011's. | largest difference 3.2e-8 | yes |
| FX-CELL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-013 frame 0: Size 1000: one cell far larger than the card, so the card sees one soft part of a ball. | largest difference 3.0e-8 | yes |
| FX-CELL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-014 frame 0: Colours #203070 to #FFD060, the second in capitals: navy seams and gold balls. | largest difference 3.1e-8 | yes |
| FX-CELL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-015 frame 0: Opacity 50: the balls laid on at half strength over the card. | largest difference 1.2e-7 | yes |
| FX-CELL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-016 frame 0: Opacity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-CELL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-017 frame 0: Blend multiply: the card darkened by the pattern, the line still dark. | largest difference 1.9e-7 | yes |
| FX-CELL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-018 frame 0: Blend add at opacity 30: the card lightened by the pattern. | largest difference 2.4e-7 | yes |
| FX-CELL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-019 frame 0: Evolution keyed from 0 at frame 0 to 360 at frame 4, linear: the balls move round and are back where they began at frame 4; frame 0 is FX-CELL-001. | largest difference 3.0e-8 | yes |
| FX-CELL-019 frame 2: Evolution keyed from 0 at frame 0 to 360 at frame 4, linear: the balls move round and are back where they began at frame 4; frame 0 is FX-CELL-001. | largest difference 3.0e-8 | yes |
| FX-CELL-019 frame 4: Evolution keyed from 0 at frame 0 to 360 at frame 4, linear: the balls move round and are back where they began at frame 4; frame 0 is FX-CELL-001. | largest difference 3.0e-8 | yes |
| FX-CELL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-020 frame 0: Size keyed from 4 at frame 0 to 1000 at frame 4, eased past its end: frame 2 would pass 1000, is held at 1000, and is size 1000. | largest difference 3.0e-8 | yes |
| FX-CELL-020 frame 2: Size keyed from 4 at frame 0 to 1000 at frame 4, eased past its end: frame 2 would pass 1000, is held at 1000, and is size 1000. | largest difference 3.0e-8 | yes |
| FX-CELL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-021 frame 0: FX-CELL-001 moved three pixels right: the pattern moves with the drawing. | largest difference 2.9e-8 | yes |
| FX-CELL-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-022 frame 0: After a Motion Tile that grows the layer: the pattern is fixed to the drawing's own space, so the frame is FX-CELL-001's. | largest difference 3.0e-8 | yes |
| FX-CELL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-023 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-023 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-024 frame 0: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-024 frame 4: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-025 frame 0: Disperse 1.6, above 1.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-025 frame 4: Disperse 1.6, above 1.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-026 frame 0: Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-026 frame 4: Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-027 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-027 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-028 frame 0: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-028 frame 4: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-029 frame 0: Pattern "tubular", which is not "bubbles", "crystals", "plates" or "static_plates". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-029 frame 4: Pattern "tubular", which is not "bubbles", "crystals", "plates" or "static_plates". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-030 frame 0: Invert "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-030 frame 4: Invert "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-031 frame 0: Light colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-031 frame 4: Light colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-032 frame 0: Blend "darken", which is not "normal", "multiply", "screen" or "add". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-032 frame 4: Blend "darken", which is not "normal", "multiply", "screen" or "add". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-033 frame 0: Disperse keyed to 1.6 at frame 4, above 1.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-033 frame 4: Disperse keyed to 1.6 at frame 4, above 1.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview halves the size, 16 to 8, and nothing else | CellPattern { pattern: "bubbles", invert: "off", contrast: 100.0, disperse: 1.0, size: 8.0, evolution: 0.0, seed: 0.0, dark_color: "#000000", light_color: "#ffffff", opacity: 100.0, blend: "normal" } | yes |
| a draft never takes the size under 1: 1.5 is held at 1, as Fractal Noise's is | CellPattern { pattern: "bubbles", invert: "off", contrast: 100.0, disperse: 1.0, size: 1.0, evolution: 0.0, seed: 0.0, dark_color: "#000000", light_color: "#ffffff", opacity: 100.0, blend: "normal" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cell_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_014.json, its light colour written in capitals, is saved in small letters, as Snowfall's is | "#203070" "#ffd060" | yes |
| a file with no `blend` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a size that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an invert that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| contrast -1 is refused with a sentence, and nothing changes | Cell Pattern's contrast runs from 0 to 1000, and this is -1. | yes |
| contrast 1001 is refused with a sentence, and nothing changes | Cell Pattern's contrast runs from 0 to 1000, and this is 1001. | yes |
| disperse -0.1 is refused with a sentence, and nothing changes | Cell Pattern's disperse runs from 0 to 1.5, and this is -0.1. | yes |
| disperse 1.6 is refused with a sentence, and nothing changes | Cell Pattern's disperse runs from 0 to 1.5, and this is 1.6. | yes |
| size 0 is refused with a sentence, and nothing changes | Cell Pattern's size runs from 1 to 1000, and this is 0. | yes |
| size 1001 is refused with a sentence, and nothing changes | Cell Pattern's size runs from 1 to 1000, and this is 1001. | yes |
| evolution 100001 is refused with a sentence, and nothing changes | Cell Pattern's evolution runs from -100000 to 100000, and this is 100001. | yes |
| seed -1 is refused with a sentence, and nothing changes | Cell Pattern's seed runs from 0 to 100000, and this is -1. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Cell Pattern's seed runs from 0 to 100000, and this is 100001. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Cell Pattern's opacity runs from 0 to 100, and this is 101. | yes |
| pattern "tubular" is refused with a sentence, and nothing changes | Cell Pattern's pattern is "bubbles", "crystals", "plates" or "static_plates", and this is "tubular". | yes |
| pattern "Bubbles", written with a capital is refused with a sentence, and nothing changes | Cell Pattern's pattern is "bubbles", "crystals", "plates" or "static_plates", and this is "Bubbles". | yes |
| invert "yes" is refused with a sentence, and nothing changes | Cell Pattern's invert is "on" or "off", and this is "yes". | yes |
| dark colour "#12345" is refused with a sentence, and nothing changes | Cell Pattern's dark colour is written #rrggbb, and this is "#12345". | yes |
| light colour "white" is refused with a sentence, and nothing changes | Cell Pattern's light colour is written #rrggbb, and this is "white". | yes |
| blend "darken" is refused with a sentence, and nothing changes | Cell Pattern's blend is "normal", "multiply", "screen" or "add", and this is "darken". | yes |
| disperse keyed to 1.6 is refused with a sentence, and nothing changes | Cell Pattern's disperse runs from 0 to 1.5, and this is 1.6. | yes |
| size keyed to 0 is refused with a sentence, and nothing changes | Cell Pattern's size runs from 1 to 1000, and this is 0. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| evolution keyed from 0 to 3600 is taken | taken | yes |
| seed keyed from 0 to 100 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cell_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up cel, in `verification/B-144 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| bubbles.png, bubbles sixteen pixels a cell: grey balls over the whole card, white at their middles and near black where they meet, the drawing under them gone; every covering kept, nothing around the card; draws cleanly | [], greys from 0 to 255 | yes |
| crystals.png, crystals: faceted cones, the same cells darker on the whole than the bubbles; draws cleanly | [], average grey 71 against the bubbles' 157 | yes |
| plates.png, plates: flat white plates, dark only in the seams, over four in ten pixels pure white, against under one in ten in the bubbles; draws cleanly | [], 5213 of 11097 pixels white, the bubbles 18 | yes |
| static_plates.png, static plates: each cell one flat grey, so nine in ten pixels match the one to their right; draws cleanly | [], 10161 of 11016 matching | yes |
| invert.png, invert on: the bubbles turned over, dark balls on light seams, each pixel and the bubbles' same pixel adding to white within 2; draws cleanly | [], furthest 0 | yes |
| contrast_300.png, contrast 300: the greys pushed out to black and white, far more pixels at one or the other; draws cleanly | [], 6775 pixels at black or white against 19 | yes |
| disperse_0.png, disperse 0: every point in its cell's middle, the balls in a square grid repeating every 16 pixels across; draws cleanly | [], repeats true | yes |
| evolution_90.png, evolution 90: the points a quarter of the way round their circles, the balls moved, most pixels changed from the bubbles; draws cleanly | [], 11020 of 11097 changed | yes |
| navy_gold_multiply.png, navy to gold laid on by multiply: the card warm in the balls, red over blue, and cool in the seams, blue over red, the drawing's lines still dark; draws cleanly | [], [190, 163, 79, 255] in a ball, [22, 36, 93, 255] in a seam | yes |

## Result

157 of 157 checks pass.
