# B-141: Roughen Edges

D-206, accepted on 2026-09-28 with the After Effects picks (B4). Every expected pixel is `Fixtures/roughen_edges/expected_roughen_edges.json`, written by `tools/roughen_edges_reference.py` before this code existed and printed in document 25 as FX-ROUGH-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-ROUGH-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ROUGH-001 frame 0: Border 2, scale 4, the rest as they start: Roughen, complexity 3, evolution 0, speed 0, seed 0: the stripes are eaten into along the drawing's top and left edges and round the empty column and row, up to two pixels deep; more than two pixels in, nothing changes. | largest difference 2.1e-7 | yes |
| FX-ROUGH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-002 frame 0: Border 0: nothing changes. | largest difference 1.9e-7 | yes |
| FX-ROUGH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-003 frame 0: Border 4: deeper bites than FX-ROUGH-001. | largest difference 2.3e-7 | yes |
| FX-ROUGH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-004 frame 0: Roughen Color with the rust edge colour as it starts: FX-ROUGH-001's covering, with a band of rust just inside the new edge. | largest difference 1.9e-7 | yes |
| FX-ROUGH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-005 frame 0: Roughen Color with a blue edge colour, #2060ff. | largest difference 1.9e-7 | yes |
| FX-ROUGH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-006 frame 0: Complexity 6: finer detail over the same broad bites. | largest difference 1.9e-7 | yes |
| FX-ROUGH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-007 frame 0: Complexity 3.9, which counts as 3: FX-ROUGH-001. | largest difference 2.1e-7 | yes |
| FX-ROUGH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-008 frame 0: Seed 7: other bites. | largest difference 2.2e-7 | yes |
| FX-ROUGH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-009 frame 0: Evolution 180: the noise moved half a step in depth, other bites. | largest difference 2.1e-7 | yes |
| FX-ROUGH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-010 frame 0: Speed 90 degrees a frame: frame 0 is FX-ROUGH-001 and frame 2, evolution 180 by then, is FX-ROUGH-009. | largest difference 2.1e-7 | yes |
| FX-ROUGH-010 frame 2: Speed 90 degrees a frame: frame 0 is FX-ROUGH-001 and frame 2, evolution 180 by then, is FX-ROUGH-009. | largest difference 2.1e-7 | yes |
| FX-ROUGH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-011 frame 0: Scale 12: broader bites. | largest difference 1.9e-7 | yes |
| FX-ROUGH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-012 frame 0: Border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 FX-ROUGH-001 and frame 4 FX-ROUGH-003. | largest difference 1.9e-7 | yes |
| FX-ROUGH-012 frame 2: Border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 FX-ROUGH-001 and frame 4 FX-ROUGH-003. | largest difference 2.1e-7 | yes |
| FX-ROUGH-012 frame 4: Border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 FX-ROUGH-001 and frame 4 FX-ROUGH-003. | largest difference 2.3e-7 | yes |
| FX-ROUGH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-013 frame 0: Border keyed from 2 at frame 0 to 500 at frame 4, eased past its end: frame 2 would pass 500, is held at 500, and is border 500. | largest difference 2.1e-7 | yes |
| FX-ROUGH-013 frame 2: Border keyed from 2 at frame 0 to 500 at frame 4, eased past its end: frame 2 would pass 500, is held at 500, and is border 500. | largest difference 1.9e-7 | yes |
| FX-ROUGH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-014 frame 0: FX-ROUGH-001 moved three pixels right: the bites move with the drawing and the three columns left bare stay empty. | largest difference 2.1e-7 | yes |
| FX-ROUGH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-015 frame 0: After a Motion Tile that grows the layer, its tiles mirrored: the noise is the drawing's own, and the drawing's top and left edges, which now meet their own mirror images, are not eaten into; the empty column and row still are. | largest difference 2.1e-7 | yes |
| FX-ROUGH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ROUGH-016 frame 0: Border 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-016 frame 4: Border 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-017 frame 0: Scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-017 frame 4: Scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-018 frame 0: Complexity 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-018 frame 4: Complexity 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-019 frame 0: Complexity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-019 frame 4: Complexity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-020 frame 0: Speed 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-020 frame 4: Speed 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-021 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-021 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-022 frame 0: An edge type "spiky", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-022 frame 4: An edge type "spiky", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-023 frame 0: An edge colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-023 frame 4: An edge colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ROUGH-024 frame 0: Border keyed to 501 at frame 4, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-024 frame 4: Border keyed to 501 at frame 4, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ROUGH-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview halves the border and the scale, and keeps the rest | RoughenEdges { edge_type: "roughen_color", edge_color: "#8a3c14", border: 8.0, size: 10.0, complexity: 4.0, evolution: 90.0, speed: 5.0, seed: 3.0, frame: 0 } | yes |
| a scale of 1.5 at half size is held at 1, the least the setting takes | RoughenEdges { edge_type: "roughen", edge_color: "#8a3c14", border: 8.0, size: 1.0, complexity: 4.0, evolution: 90.0, speed: 5.0, seed: 3.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rough_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rough_005.json, its colour written in capitals, is saved in small letters, as Snowfall's is | "#2060ff" | yes |
| a file with no `edge_type` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an edge colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a border that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| border -1 is refused with a sentence, and nothing changes | Roughen Edges's border runs from 0 to 500, and this is -1. | yes |
| border 501 is refused with a sentence, and nothing changes | Roughen Edges's border runs from 0 to 500, and this is 501. | yes |
| scale 0 is refused with a sentence, and nothing changes | Roughen Edges's size runs from 1 to 1000, and this is 0. | yes |
| scale 1001 is refused with a sentence, and nothing changes | Roughen Edges's size runs from 1 to 1000, and this is 1001. | yes |
| complexity 0 is refused with a sentence, and nothing changes | Roughen Edges's complexity runs from 1 to 10, and this is 0. | yes |
| complexity 11 is refused with a sentence, and nothing changes | Roughen Edges's complexity runs from 1 to 10, and this is 11. | yes |
| evolution 100001 is refused with a sentence, and nothing changes | Roughen Edges's evolution runs from -100000 to 100000, and this is 100001. | yes |
| evolution speed -361 is refused with a sentence, and nothing changes | Roughen Edges's speed runs from -360 to 360, and this is -361. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Roughen Edges's seed runs from 0 to 100000, and this is 100001. | yes |
| edge type "spiky" is refused with a sentence, and nothing changes | Roughen Edges' edge type is "roughen" or "roughen_color", and this is "spiky". | yes |
| edge type "Roughen", written with a capital is refused with a sentence, and nothing changes | Roughen Edges' edge type is "roughen" or "roughen_color", and this is "Roughen". | yes |
| edge colour "#12345" is refused with a sentence, and nothing changes | Roughen Edges's edge colour is written #rrggbb, and this is "#12345". | yes |
| border keyed to 501 is refused with a sentence, and nothing changes | Roughen Edges's border runs from 0 to 500, and this is 501. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| border keyed from 0 to 20 is taken | taken | yes |
| evolution keyed from 0 to 720 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rough_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rough_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rough_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rough_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up drawing on nothing, in `verification/B-141 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| as_it_starts.png, as it starts, border 8, scale 10, complexity 3: no pixel gains covering; every pixel with nothing clear within 9 pixels unchanged; the edges bitten, many of the pixels just inside them now more clear than covered; draws cleanly | [], 0 grown, 3584 of 3584 deep pixels unchanged, 1025 of 1240 edge pixels bitten | yes |
| border_16.png, border 16: deeper bites, more of the drawing clear than as_it_starts.png; no pixel gains covering; every pixel with nothing clear within 17 pixels unchanged; draws cleanly | [], 10946 clear against 9537, 0 grown, 1400 of 1400 deep pixels unchanged | yes |
| scale_40.png, scale 40: wider, gentler bites: different from as_it_starts.png; no pixel gains covering; every pixel with nothing clear within 9 pixels unchanged; draws cleanly | [], 0 grown, 3584 of 3584 deep pixels unchanged | yes |
| complexity_6.png, complexity 6: finer detail along the bites: different from as_it_starts.png; no pixel gains covering; every pixel with nothing clear within 9 pixels unchanged; draws cleanly | [], 0 grown, 3584 of 3584 deep pixels unchanged | yes |
| seed_3.png, seed 3: other bites: different from as_it_starts.png; no pixel gains covering; every pixel with nothing clear within 9 pixels unchanged; draws cleanly | [], 0 grown, 3584 of 3584 deep pixels unchanged | yes |
| roughen_color.png, Roughen Color in rust: the same covering as as_it_starts.png, every pixel within 1; the new edge rust #8a3c14 in many pixels; every pixel with nothing clear within 17 pixels unchanged; draws cleanly | [], 16000 of 16000 the same covering, 1481 rust, 1400 of 1400 deep pixels unchanged | yes |
| roughen_color_blue_16.png, Roughen Color in blue #2060ff at border 16: the same covering as border_16.png, every pixel within 1; a wider band, more pixels of the edge colour than roughen_color.png has of rust; draws cleanly | [], 16000 of 16000 the same covering, 2381 blue against 1481 rust | yes |

## Result

121 of 121 checks pass.
