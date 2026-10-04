# B-128: displacement map

D-193, accepted by the owner on 2026-09-28 ("take everything"). Every expected pixel is `Fixtures/displacement_map/expected_displacement_map.json`, written by `tools/displacement_map_reference.py` before this code existed and printed in document 25 as FX-DMAP-001 to 031. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DMAP-001 to 031 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DMAP-001 frame 0: As added: no layer named, so nothing moves. | largest difference 1.5e-7 | yes |
| FX-DMAP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-002 frame 0: The grey ramp as the map, red across and green down, both at most 2: column 0, black, reads 2 left and 2 up, column 15, white, 2 right and 2 down, and between them in step. The ramp layer is moved, scaled and switched off, none of which the map reads. | largest difference 3.1e-7 | yes |
| FX-DMAP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-003 frame 0: A white solid: every pixel reads the one 2 right and 2 below, so the picture moves 2 left and 2 up and clear comes in at the right and the bottom. | largest difference 1.5e-7 | yes |
| FX-DMAP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-004 frame 0: A black solid: the picture moves 2 right and 2 down. | largest difference 1.5e-7 | yes |
| FX-DMAP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-005 frame 0: The white solid with both maxima -2: as FX-DMAP-004. | largest difference 1.5e-7 | yes |
| FX-DMAP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-006 frame 0: The black solid, across Full and down Off, both at most 2: the picture moves 2 left, whatever the map, and not up or down. | largest difference 1.5e-7 | yes |
| FX-DMAP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-007 frame 0: The ramp, both directions Off: nothing moves. | largest difference 1.5e-7 | yes |
| FX-DMAP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-008 frame 0: A painted map of greens and blues, blue across at most 2.5 and luminance down at most 1.5. | largest difference 3.6e-7 | yes |
| FX-DMAP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-009 frame 0: The painted map, hue across and saturation down, both at most 2. | largest difference 4.9e-7 | yes |
| FX-DMAP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-010 frame 0: The painted map, lightness across at most -2 and red down at most 1.5. | largest difference 2.6e-7 | yes |
| FX-DMAP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-011 frame 0: A white map clear at the top and more covering down each row, red and green, at most 2: row 0 does not move, since a clear map moves nothing, and each row lower moves more, up and to the left. | largest difference 1.6e-7 | yes |
| FX-DMAP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-012 frame 0: The same map, alpha both ways: row 0, clear, reads 2 left and 2 up, so it is clear itself; the bottom row, nearly covered, reads nearly 2 right and 2 down. | largest difference 2.1e-7 | yes |
| FX-DMAP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-013 frame 0: FX-DMAP-003 with Wrap on: the picture moves 2 left and 2 up, and what leaves at the left and the top comes back at the right and the bottom. | largest difference 1.5e-7 | yes |
| FX-DMAP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-014 frame 0: A 4 by 2 checker of white and black as the map, centred, at most 2: only the eight pixels it covers, columns 6 to 9 of rows 4 and 5, move. | largest difference 1.5e-7 | yes |
| FX-DMAP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-015 frame 0: The checker tiled: its pattern repeated over the whole layer. | largest difference 1.5e-7 | yes |
| FX-DMAP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-016 frame 0: The checker stretched to 16 by 10, softened between its squares. | largest difference 2.1e-7 | yes |
| FX-DMAP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-017 frame 0: The holder names itself, at most 2: its own colours move it; the clear corner, a clear map, does not move. | largest difference 3.3e-7 | yes |
| FX-DMAP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-018 frame 0: Across keyed from 0 at frame 0 to 4 at frame 4, down 0, with the ramp: frame 0 does not move, frame 2 moves across as FX-DMAP-002 does. | largest difference 1.5e-7 | yes |
| FX-DMAP-018 frame 2: Across keyed from 0 at frame 0 to 4 at frame 4, down 0, with the ramp: frame 0 does not move, frame 2 moves across as FX-DMAP-002 does. | largest difference 2.5e-7 | yes |
| FX-DMAP-018 frame 4: Across keyed from 0 at frame 0 to 4 at frame 4, down 0, with the ramp: frame 0 does not move, frame 2 moves across as FX-DMAP-002 does. | largest difference 5.2e-7 | yes |
| FX-DMAP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-019 frame 0: A layer that is not in the composition, `gone`: nothing moves, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-DMAP-019 frame 4: A layer that is not in the composition, `gone`: nothing moves, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-DMAP-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-DMAP-020 frame 0: A white solid whose in point is frame 4: an empty map before it, so nothing moves at frame 0, and at frame 4 the picture moves as in FX-DMAP-003. | largest difference 1.5e-7 | yes |
| FX-DMAP-020 frame 4: A white solid whose in point is frame 4: an empty map before it, so nothing moves at frame 0, and at frame 4 the picture moves as in FX-DMAP-003. | largest difference 1.5e-7 | yes |
| FX-DMAP-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-021 frame 0: FX-DMAP-002 on the holder moved 2 right and 1 down: the same picture moved, since the map lies on the layer. | largest difference 3.1e-7 | yes |
| FX-DMAP-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-022 frame 0: The effect on an adjustment layer above the holder, the ramp as its map: the map lies on the frame, which here is the holder's own rectangle, so this is FX-DMAP-002. | largest difference 3.1e-7 | yes |
| FX-DMAP-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-023 frame 0: FX-DMAP-002 with Wrap on: the same in the middle, and at the edges what is read from beyond one side comes from the other. | largest difference 3.1e-7 | yes |
| FX-DMAP-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DMAP-024 frame 0: Maximum across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-024 frame 4: Maximum across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-025 frame 0: Maximum down -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-025 frame 4: Maximum down -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-026 frame 0: Maximum across keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-026 frame 4: Maximum across keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-027 frame 0: Across written "half", After Effects' Half, which is left out. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-027 frame 4: Across written "half", After Effects' Half, which is left out. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-028 frame 0: Down written "purple". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-028 frame 4: Down written "purple". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-029 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-029 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-030 frame 0: Wrap written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-030 frame 4: Wrap written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DMAP-031 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-031 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-DMAP-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_mixed.json: `a`'s Displacement Map names `b`, and `b`'s Compound Blur names `a`: a circle through two kinds of effect, refused the same. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Displacement Map naming the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| maxima of 1000 and -1000, the most, never grow the drawing's bounds | 0 | yes |
| a half-size draft preview halves both maxima, and nothing else | DisplacementMap { layer: String("ramp"), fit: "stretch", horizontal: "red", max_horizontal: 10.0, vertical: "green", max_vertical: -4.0, wrap: "off", expand: "off", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dmap_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dmap_031.json, its layer written as the number 3, is saved as the number 3 | 3 | yes |
| the map the effect reads is never saved | None | yes |
| a file with no `fit` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `layer` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `wrap` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a maximum across that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Max Horizontal Displacement 1001 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map's max horizontal runs from -1000 to 1000, and this is 1001. | yes |
| Max Vertical Displacement -1001 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map's max vertical runs from -1000 to 1000, and this is -1001. | yes |
| across "half" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map reads red, green, blue, alpha, luminance, hue, lightness, saturation, full or off, and this is "half". | yes |
| down "purple" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map reads red, green, blue, alpha, luminance, hue, lightness, saturation, full or off, and this is "purple". | yes |
| fit "fill" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| wrap "yes" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map's wrap is "off" or "on", and this is "yes". | yes |
| a layer that is the number 3 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Displacement Map's layer is the name of a layer of this composition, and this is 3. | yes |
| a Displacement Map on `ramp` naming `holder`, which names `ramp` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| a Compound Blur on `ramp` naming `holder`, whose Displacement Map names `ramp` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| naming the holder itself, which is no circle, is taken | taken | yes |
| luminance across instead of red is taken | taken | yes |
| a Displacement Map on `ramp` naming `white` is taken | taken | yes |
| a Displacement Map on `white` naming no layer is taken | taken | yes |
| each of the ten channel words, across and down, with Tile Map and Wrap on, is taken | red, green, blue, alpha, luminance, hue, lightness, saturation, full, off | yes |
| `white`'s Displacement Map changed to name `ramp`, which names `white` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: B-127's street, in `verification/B-128 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street with no effect, draws cleanly | [] | yes |
| waves.png, the waves, red across and green down at most 3: the street ripples, and clear comes in at the edges, draws cleanly | [], 60556 pixels changed, 745 rim pixels less than opaque | yes |
| waves_wrapped.png, the waves at most 10 with Wrap Pixels Around: stronger, and the edges filled from the far side, draws cleanly | [], 86129 pixels changed, 0 pixels less than opaque | yes |
| glass_ball.png, the glass ball, at most 40% of its size: the street magnified inside it and untouched outside, draws cleanly | [], 0 pixels outside the ball changed, 22003 inside it | yes |
| heat_haze.png, the street naming itself, luminance across at most 4: bright parts pushed sideways more than dark, draws cleanly | [], 6936 pixels changed | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dmap_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmap_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmap_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmap_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmap_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dmap_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

142 of 142 checks pass.
