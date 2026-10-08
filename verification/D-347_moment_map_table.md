# B-227: Moment Map

D-347, after After Effects' Time Displacement (EFFECTS.md P0-4). Every expected pixel is `Fixtures/time_displacement/expected_time_displacement.json`, written by `tools/time_displacement_reference.py` before this code existed and printed in document 25 as FX-TDISP-001 to 028. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TDISP-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TDISP-001 frame 6: As added: Max Displacement 1 second, Time Resolution 60, the layer's own brightness as the map, Stretch: each pixel from a moment chosen by its own brightness; moments past the layer's twelve frames are clear. | largest difference 1.5e-7 | yes |
| FX-TDISP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-002 frame 0: The ramp as the map, black at the left to white at the right, Max 0.5: column x is 4 x - 30 sixtieths of a second from now, so column 0 shows frame 1 and column 15 frame 11 at frame 6; at frame 0 the left half asks for frames before the layer, clear. | largest difference 1.5e-7 | yes |
| FX-TDISP-002 frame 6: The ramp as the map, black at the left to white at the right, Max 0.5: column x is 4 x - 30 sixtieths of a second from now, so column 0 shows frame 1 and column 15 frame 11 at frame 6; at frame 0 the left half asks for frames before the layer, clear. | largest difference 1.5e-7 | yes |
| FX-TDISP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-003 frame 6: The same at Max -0.5: the other way, the left shows the frames after. | largest difference 1.5e-7 | yes |
| FX-TDISP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-004 frame 6: The ramp, Max 0.5, Time Resolution 2: moments in half seconds, so only frames 1, 6 and 11 are seen. | largest difference 1.5e-7 | yes |
| FX-TDISP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-005 frame 6: The ramp, Max 0.4, Time Resolution 3: a third of a second is 3.33 frames, held to the frame holding it: frame 9 after and frame 2 before. | largest difference 1.5e-7 | yes |
| FX-TDISP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-006 frame 6: The ramp, Max 0.5, Time Resolution 100, past the frame rate: every pixel is still one of the drawings, nothing in between; the same frame as FX-TDISP-002. | largest difference 1.5e-7 | yes |
| FX-TDISP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-007 frame 6: The layer from frame 3 to frame 8 only: the moments outside it are clear. | largest difference 1.3e-7 | yes |
| FX-TDISP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-008 frame 6: A small map, 4 by 2 steps of grey, stretched over the layer. | largest difference 1.5e-7 | yes |
| FX-TDISP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-009 frame 6: The same map centred: outside it there is no map, so no shift. | largest difference 1.5e-7 | yes |
| FX-TDISP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-010 frame 6: The same map tiled. | largest difference 1.5e-7 | yes |
| FX-TDISP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-011 frame 6: An orange solid as the map: its brightness, 0.664, is 20 sixtieths of a second, so the whole layer is 3 frames ahead: frame 6 shows frame 9. | largest difference 1.5e-7 | yes |
| FX-TDISP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-012 frame 6: A white map at half covering, laid over mid grey: half the way to white, 30 sixtieths, so frame 6 shows frame 11. | largest difference 1.5e-7 | yes |
| FX-TDISP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-013 frame 6: The ramp with an Exposure of -1 on it: a map's effects count (D-189), so it is darker and every column reaches further back. | largest difference 1.5e-7 | yes |
| FX-TDISP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-014 frame 6: A mask keeping columns 0 to 7: every moment is masked; the left half is FX-TDISP-002's. | largest difference 1.3e-7 | yes |
| FX-TDISP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-015 frame 6: An Exposure of +1 before it is not seen: the moments are the drawings and the layer's own map is its drawing. FX-TDISP-001. | largest difference 1.5e-7 | yes |
| FX-TDISP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-016 frame 6: An Exposure of -1 after it darkens the result. | largest difference 7.6e-8 | yes |
| FX-TDISP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-017 frame 6: The holder moved 2 right and 1 down: FX-TDISP-002 moved, since the map lies on the layer. | largest difference 1.3e-7 | yes |
| FX-TDISP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-018 frame 6: The effect on an adjustment layer above the holder: nothing changes. | largest difference 1.5e-7 | yes |
| FX-TDISP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-019 frame 0: Max keyed from 0 at frame 0 to 1 at frame 12, the ramp: at frame 0 nothing moves; at frame 6 it is 0.5, FX-TDISP-002. | largest difference 1.5e-7 | yes |
| FX-TDISP-019 frame 6: Max keyed from 0 at frame 0 to 1 at frame 12, the ramp: at frame 0 nothing moves; at frame 6 it is 0.5, FX-TDISP-002. | largest difference 1.5e-7 | yes |
| FX-TDISP-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-020 frame 6: Max 0: every pixel from now, the layer as it is. | largest difference 1.5e-7 | yes |
| FX-TDISP-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDISP-021 frame 6: A layer that is not in the composition, `gone`: the layer itself is the map, FX-TDISP-001, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-TDISP-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-TDISP-022 frame 0: Max 10.5 seconds, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-022 frame 4: Max 10.5 seconds, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDISP-023 frame 0: Max -10.5 seconds, below -10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-023 frame 4: Max -10.5 seconds, below -10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDISP-024 frame 0: Time Resolution 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-024 frame 4: Time Resolution 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDISP-025 frame 0: Time Resolution 1000, above 999. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-025 frame 4: Time Resolution 1000, above 999. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDISP-026 frame 0: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-026 frame 4: A fit written "fill". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDISP-027 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-027 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDISP-028 frame 0: Max keyed to 12 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-028 frame 4: Max keyed to 12 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-TDISP-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tdisp_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdisp_014.json, its mask in the first form, is saved as one mask of the same four corners in the current form, and the saved file draws frame 6 the same | 1 mask(s), frame 6 byte-identical | yes |
| the moments it lays are never saved | None None | yes |
| fx_tdisp_026.json is refused in a sentence | Moment Map's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| fx_tdisp_027.json is refused in a sentence | Moment Map's map is the name of a layer of this composition, and this is 3. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| max 10.5 seconds is refused with a sentence, and nothing changes | Moment Map's max time runs from -10 to 10, and this is 10.5. | yes |
| fit "fill" is refused with a sentence, and nothing changes | Moment Map's fit is "center", "stretch" or "tile", and this is "fill". | yes |
| max -2 seconds, is taken | taken | yes |
| fit tile, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tdisp_001.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tdisp_002.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tdisp_010.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tdisp_014.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a sliding street, in `verification/D-347 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street at frame 12, draws cleanly | [] | yes |
| as_added.png, as added: Max 1 second, Time Resolution 60, its own brightness: the bright sky from later, the dark road from a second back, before the street began, so the road is clear | [], 54680 pixels changed | yes |
| squeeze.png, the ramp, Max 0.5: the left edge from half a second back, the right edge from half a second on, so the sliding street comes out squeezed, more buildings across the frame | [], 32160 pixels changed | yes |
| still.png, Max 0: the street as it is, the same as before.png | [], 0 pixels changed | yes |

## Result

109 of 109 checks pass.
