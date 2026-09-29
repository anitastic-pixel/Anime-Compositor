# B-129: gradient wipe

D-194, accepted by the owner on 2026-09-28 ("take everything"). Every expected pixel is `Fixtures/gradient_wipe/expected_gradient_wipe.json`, written by `tools/gradient_wipe_reference.py` before this code existed and printed in document 25 as FX-GWIPE-001 to 028. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-GWIPE-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GWIPE-001 frame 0: As added: no layer named and completion 0, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-GWIPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-002 frame 0: No layer named, completion 50: with no map nothing changes. | largest difference 1.5e-7 | yes |
| FX-GWIPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-003 frame 0: The grey ramp as the map, completion 50, softness 0: its eight dark columns, 0 to 7, are wiped away and the eight light ones kept whole. The ramp layer is moved, scaled and switched off, none of which the map reads. | largest difference 1.5e-7 | yes |
| FX-GWIPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-004 frame 0: The ramp, completion 50, softness 100: each pixel kept by the ramp's brightness there, column 0 gone, column 15 whole, the rest between. | largest difference 1.9e-7 | yes |
| FX-GWIPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-005 frame 0: The ramp, completion 0, softness 50: nothing changes. | largest difference 1.5e-7 | yes |
| FX-GWIPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-006 frame 0: The ramp, completion 100, softness 50: every pixel clear. | largest difference 0.0e0 | yes |
| FX-GWIPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-007 frame 0: The ramp, completion 100, softness 0: every pixel clear, the white column too. | largest difference 0.0e0 | yes |
| FX-GWIPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-008 frame 0: FX-GWIPE-003 with Invert on: the eight light columns go, the dark ones stay. | largest difference 1.5e-7 | yes |
| FX-GWIPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-009 frame 0: The ramp, completion 30, softness 20: columns 0 to 2 gone, 6 and on whole, 3 to 5 fading in between. | largest difference 2.9e-7 | yes |
| FX-GWIPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-010 frame 0: A white solid, completion 99: nothing is dark enough to go. | largest difference 1.5e-7 | yes |
| FX-GWIPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-011 frame 0: A black solid, completion 1: everything goes at once. | largest difference 0.0e0 | yes |
| FX-GWIPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-012 frame 0: A white map clear at the top and more covering down each row, completion 50: a clear map reads as black, so rows 0 and 1 go and the rest stay. | largest difference 1.5e-7 | yes |
| FX-GWIPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-013 frame 0: A 4 by 2 checker of white and black, centred, completion 50: outside it the map is clear, black, so only its four white pixels are kept. | largest difference 1.5e-7 | yes |
| FX-GWIPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-014 frame 0: The checker tiled: kept and gone in a checker over the whole layer. | largest difference 1.5e-7 | yes |
| FX-GWIPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-015 frame 0: The checker stretched to 16 by 10, completion 50, softness 30: soft squares. | largest difference 1.5e-7 | yes |
| FX-GWIPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-016 frame 0: The holder names itself, completion 50: its red and blue squares, darker than half, go, its cream ones stay, and the clear corner stays clear. | largest difference 9.7e-8 | yes |
| FX-GWIPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-017 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 whole, frame 2 as FX-GWIPE-003, frame 4 clear. | largest difference 1.5e-7 | yes |
| FX-GWIPE-017 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 whole, frame 2 as FX-GWIPE-003, frame 4 clear. | largest difference 1.5e-7 | yes |
| FX-GWIPE-017 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 whole, frame 2 as FX-GWIPE-003, frame 4 clear. | largest difference 0.0e0 | yes |
| FX-GWIPE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-018 frame 0: Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004. | largest difference 1.5e-7 | yes |
| FX-GWIPE-018 frame 2: Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004. | largest difference 2.2e-7 | yes |
| FX-GWIPE-018 frame 4: Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004. | largest difference 1.9e-7 | yes |
| FX-GWIPE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-019 frame 0: A layer that is not in the composition, `gone`, completion 50: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GWIPE-019 frame 4: A layer that is not in the composition, `gone`, completion 50: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-GWIPE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-GWIPE-020 frame 0: A white solid whose in point is frame 4, completion 50: before it the map is empty, which reads as black, so at frame 0 everything is gone; at frame 4 nothing is. | largest difference 0.0e0 | yes |
| FX-GWIPE-020 frame 4: A white solid whose in point is frame 4, completion 50: before it the map is empty, which reads as black, so at frame 0 everything is gone; at frame 4 nothing is. | largest difference 1.5e-7 | yes |
| FX-GWIPE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-021 frame 0: FX-GWIPE-003 on the holder moved 2 right and 1 down: the same picture moved, since the map lies on the layer. | largest difference 1.5e-7 | yes |
| FX-GWIPE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-022 frame 0: The effect on an adjustment layer above the holder, the ramp as its map: the map lies on the frame, which here is the holder's own rectangle, so this is FX-GWIPE-003. | largest difference 1.5e-7 | yes |
| FX-GWIPE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GWIPE-023 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-023 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-024 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-024 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-025 frame 0: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-025 frame 4: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-026 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-026 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-027 frame 0: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-027 frame 4: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GWIPE-028 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-028 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GWIPE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_mixed.json: `a`'s Gradient Wipe names `b`, and `b`'s Displacement Map names `a`: a circle through two kinds of effect, refused the same. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Gradient Wipe naming the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a wipe never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is, since none is a distance | GradientWipe { layer: String("ramp"), fit: "stretch", completion: 40.0, softness: 30.0, invert: "off", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gwipe_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gwipe_028.json, its layer written as the number 3, is saved as the number 3 | 3 | yes |
| the map the effect reads is never saved | None | yes |
| a file with no `fit` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `layer` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `invert` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a completion that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Transition Completion 101 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Gradient Wipe's completion runs from 0 to 100, and this is 101. | yes |
| Transition Softness -1 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Gradient Wipe's softness runs from 0 to 100, and this is -1. | yes |
| Transition Completion keyed to 200 at frame 4 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Gradient Wipe's completion runs from 0 to 100, and this is 200. | yes |
| fit "fill" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Gradient Wipe's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| invert "yes" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Gradient Wipe's invert is "off" or "on", and this is "yes". | yes |
| a layer that is the number 3 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Gradient Wipe's layer is the name of a layer of this composition, and this is 3. | yes |
| a Gradient Wipe on `ramp` naming `holder`, which names `ramp` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| a Displacement Map on `ramp` naming `holder`, whose Gradient Wipe names `ramp` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| naming the holder itself, which is no circle, is taken | taken | yes |
| Invert Gradient on, the checker tiled, softness 100 is taken | taken | yes |
| a Gradient Wipe on `ramp` naming `white` is taken | taken | yes |
| a Gradient Wipe on `white` naming no layer is taken | taken | yes |
| `white`'s Gradient Wipe changed to name `ramp`, which names `white` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: B-127's street over a plum solid, in `verification/B-129 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street with no effect, draws cleanly | [] | yes |
| spot_30.png, the spot at completion 30%, softness 0: a hard hole round the spot, the street whole outside it, draws cleanly | [], 23436 pixels gone, 0 between, 0 on the wrong side of 30% | yes |
| spot_60_soft.png, completion 60%, softness 25: a wider hole with a soft edge, draws cleanly | [], 64165 pixels gone, 40845 fading | yes |
| spot_30_invert.png, completion 30% with Invert Gradient: wiped from the outside in, the street kept within 70% of the way out, draws cleanly | [], 0 between, 0 on the wrong side of 70% | yes |
| clouds_40.png, the clouds at completion 40%, softness 10: soft patches gone, draws cleanly | [], 36341 pixels gone, 18860 fading | yes |
| clouds_75.png, the clouds at 75%: most of the street gone, draws cleanly | [], 102916 pixels gone, more than at 40% | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gwipe_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gwipe_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gwipe_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gwipe_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gwipe_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gwipe_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

132 of 132 checks pass.
