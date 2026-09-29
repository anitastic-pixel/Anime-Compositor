# B-127: compound blur

D-191, accepted by the owner on 2026-09-28 ("take everything"). Every expected pixel is `Fixtures/compound_blur/expected_compound_blur.json`, written by `tools/compound_blur_reference.py` before this code existed and printed in document 25 as FX-CBLUR-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CBLUR-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CBLUR-001 frame 0: As added: no layer named, so nothing is blurred. | largest difference 1.5e-7 | yes |
| FX-CBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-002 frame 0: The ramp as the map, Maximum Blur 6: sharp at the left, blurred more and more to the right. The ramp layer is moved, scaled and switched off, none of which the map reads. | largest difference 2.0e-7 | yes |
| FX-CBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-003 frame 0: A white solid as the map, Maximum Blur 6: every pixel blurred the most, document 21's Gaussian at sigma 2. | largest difference 1.9e-7 | yes |
| FX-CBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-004 frame 0: A black solid as the map: nothing is blurred. | largest difference 1.5e-7 | yes |
| FX-CBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-005 frame 0: The black solid inverted: every pixel blurred the most, as FX-CBLUR-003. | largest difference 1.9e-7 | yes |
| FX-CBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-006 frame 0: The ramp inverted: blurred at the left, sharp at the right. | largest difference 2.3e-7 | yes |
| FX-CBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-007 frame 0: A grey solid, linear 0.25, luma 0.537: between the two largest levels, everywhere the same. | largest difference 1.8e-7 | yes |
| FX-CBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-008 frame 0: FX-CBLUR-002 with the edges repeated: the outer pixels are held rather than fading into clear. | largest difference 2.0e-7 | yes |
| FX-CBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-009 frame 0: The ramp with Maximum Blur 0: nothing is blurred. | largest difference 1.5e-7 | yes |
| FX-CBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-010 frame 0: A 4 by 2 checker as the map, centred: only the eight pixels it covers, columns 6 to 9 of rows 4 and 5, are read; the white ones blurred. | largest difference 1.6e-7 | yes |
| FX-CBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-011 frame 0: The checker tiled: its pattern repeated over the whole layer. | largest difference 1.9e-7 | yes |
| FX-CBLUR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-012 frame 0: The checker stretched to 16 by 10, softened between its squares. | largest difference 1.9e-7 | yes |
| FX-CBLUR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-013 frame 0: The holder names itself: its own brightness is the map; the cream squares blurred most, the clear corner not at all. | largest difference 2.1e-7 | yes |
| FX-CBLUR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-014 frame 0: Maximum Blur keyed from 0 at frame 0 to 12 at frame 4 with the ramp: frame 0 is untouched and frame 2 is FX-CBLUR-002. | largest difference 1.5e-7 | yes |
| FX-CBLUR-014 frame 2: Maximum Blur keyed from 0 at frame 0 to 12 at frame 4 with the ramp: frame 0 is untouched and frame 2 is FX-CBLUR-002. | largest difference 2.0e-7 | yes |
| FX-CBLUR-014 frame 4: Maximum Blur keyed from 0 at frame 0 to 12 at frame 4 with the ramp: frame 0 is untouched and frame 2 is FX-CBLUR-002. | largest difference 2.3e-7 | yes |
| FX-CBLUR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-015 frame 0: A layer that is not in the composition, `gone`: nothing is blurred, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-CBLUR-015 frame 4: A layer that is not in the composition, `gone`: nothing is blurred, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-CBLUR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CBLUR-016 frame 0: A white solid whose in point is frame 4: an empty map before it, so nothing is blurred at frame 0 and all of it at frame 4. | largest difference 1.5e-7 | yes |
| FX-CBLUR-016 frame 4: A white solid whose in point is frame 4: an empty map before it, so nothing is blurred at frame 0 and all of it at frame 4. | largest difference 1.9e-7 | yes |
| FX-CBLUR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-017 frame 0: FX-CBLUR-016 inverted: all of it at frame 0, as FX-CBLUR-003, and nothing at frame 4. | largest difference 1.9e-7 | yes |
| FX-CBLUR-017 frame 4: FX-CBLUR-016 inverted: all of it at frame 0, as FX-CBLUR-003, and nothing at frame 4. | largest difference 1.5e-7 | yes |
| FX-CBLUR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-018 frame 0: FX-CBLUR-002 on the holder moved 2 right and 1 down: the same picture moved, since the map lies on the layer. | largest difference 2.0e-7 | yes |
| FX-CBLUR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-019 frame 0: The effect on an adjustment layer above the holder, the ramp as its map: the map lies on the frame, which here is the holder's own rectangle, so this is FX-CBLUR-002. | largest difference 2.0e-7 | yes |
| FX-CBLUR-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CBLUR-020 frame 0: Maximum Blur 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-020 frame 4: Maximum Blur 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CBLUR-021 frame 0: Maximum Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-021 frame 4: Maximum Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CBLUR-022 frame 0: Maximum Blur keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-022 frame 4: Maximum Blur keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CBLUR-023 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-023 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CBLUR-024 frame 0: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-024 frame 4: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CBLUR-025 frame 0: Edges written "wrap". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-025 frame 4: Edges written "wrap". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CBLUR-026 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-026 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CBLUR-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| chain_adjustment.json frame 0: `a`, a drawing, names `b`, an adjustment layer, whose Compound Blur, Maximum Blur 0, names `a`: not a circle, since an adjustment layer's map is its white rectangle and its effects are not run for it. It opens, and `a` is blurred the most everywhere, as FX-CBLUR-003. | largest difference 1.9e-7 | yes |
| cycle_off.json: `a` names `b` and `b` names `a` by an effect switched off: still refused, since switching it on would close the circle. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_three.json: `a` names `b`, `b` names `c`, `c` names `a`: refused. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| cycle_two.json: `a` names `b` and `b` names `a`: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Maximum Blur 500, the most, never grows the drawing's bounds | 0 | yes |
| a half-size draft preview halves the Maximum Blur, and nothing else | CompoundBlur { layer: String("ramp"), fit: "stretch", max_blur: 10.0, invert: "off", edges: "transparent", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cblur_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cblur_026.json, its layer written as the number 3, is saved as the number 3 | 3 | yes |
| the map the effect reads is never saved | None | yes |
| a file with no `fit` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `layer` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Maximum Blur that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Maximum Blur 501 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Compound Blur's max blur runs from 0 to 500, and this is 501. | yes |
| Maximum Blur -1 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Compound Blur's max blur runs from 0 to 500, and this is -1. | yes |
| fit "fill" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Compound Blur's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| invert "yes" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Compound Blur's invert is "off" or "on", and this is "yes". | yes |
| edges "wrap" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Compound Blur's edges are "transparent" or "repeat", and this is "wrap". | yes |
| a layer that is the number 3 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Compound Blur's layer is the name of a layer of this composition, and this is 3. | yes |
| a Compound Blur on `ramp` naming `holder`, which names `ramp` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| naming the holder itself, which is no circle, is taken | taken | yes |
| a Compound Blur on `ramp` naming `white` is taken | taken | yes |
| a Compound Blur on `white` naming no layer is taken | taken | yes |
| `white`'s Compound Blur changed to name `ramp`, which names `white` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| a second Compound Blur on `white`, switched off, naming `ramp` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |
| adding a layer `gone`, which the holder names, with a Compound Blur naming the holder is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |

## Pictures: a street at a quarter of 1920 by 1080, in `verification/B-127 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street with no effect, draws cleanly | [] | yes |
| depth_of_field.png, the depth map, Maximum Blur 10, edges repeated: the house fronts sharp, the sky and the road blurred, draws cleanly | [], rows 170 to 210 kept true, sky changed true, corner alpha 255 | yes |
| depth_inverted.png, the same, inverted: the house fronts blurred, the sky above row 100 sharp, draws cleanly | [], rows 0 to 100 kept true, house fronts changed true | yes |
| edges_transparent.png, the depth map with the edges left transparent: the frame's rim fades into clear, draws cleanly | [], rows 170 to 210 kept true, corner alpha 80 | yes |
| own_brightness.png, the street naming itself: bright sky and lit windows blurred most, the dark road least, draws cleanly | [], houses changed true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cblur_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cblur_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cblur_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cblur_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cblur_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

125 of 125 checks pass.
