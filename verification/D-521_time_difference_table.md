# B-401: Time Difference

D-521, under the owner's /loop request of 2026-10-11. Every expected pixel is `Fixtures/time_difference/expected_time_difference.json`, written by `tools/time_difference_reference.py` before this code existed and printed in document 25 as FX-TDIFF-001 to 031. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TDIFF-001 to 031 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TDIFF-001 frame 2: As added: the layer against itself now, so nothing differs and every pixel that shows is black, the clear ones clear. | largest difference 3.0e-8 | yes |
| FX-TDIFF-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-002 frame 0: Against itself one frame back: the ball's old and new places show, the rest is black. At frame 0 the frame before is outside the layer, a clear target, so the layer shows as it is. | largest difference 1.4e-7 | yes |
| FX-TDIFF-002 frame 2: Against itself one frame back: the ball's old and new places show, the rest is black. At frame 0 the frame before is outside the layer, a clear target, so the layer shows as it is. | largest difference 9.7e-8 | yes |
| FX-TDIFF-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-003 frame 2: Against itself one frame ahead. At frame 4 the next frame holds the same drawing, so all black; at frame 7 the next frame is past the out point, so the layer as it is. | largest difference 9.7e-8 | yes |
| FX-TDIFF-003 frame 4: Against itself one frame ahead. At frame 4 the next frame holds the same drawing, so all black; at frame 7 the next frame is past the out point, so the layer as it is. | largest difference 3.0e-8 | yes |
| FX-TDIFF-003 frame 7: Against itself one frame ahead. At frame 4 the next frame holds the same drawing, so all black; at frame 7 the next frame is past the out point, so the layer as it is. | largest difference 1.4e-7 | yes |
| FX-TDIFF-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-004 frame 2: Absolute Difference off, one frame back: no difference is mid grey, lighter where the layer is lighter now, darker where it was lighter. | largest difference 7.6e-8 | yes |
| FX-TDIFF-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-005 frame 2: Contrast 20, one frame back: every difference three times as strong, held at white. | largest difference 4.2e-7 | yes |
| FX-TDIFF-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-006 frame 2: Alpha Channel Target, one frame back: the covering is the target's, so the ball's old place on the stripe shows and its new place does not. | largest difference 9.7e-8 | yes |
| FX-TDIFF-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-007 frame 2: Alpha Channel Blend: the mean of the two coverings. | largest difference 5.0e-8 | yes |
| FX-TDIFF-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-008 frame 2: Alpha Channel Max: the larger of the two coverings. | largest difference 9.7e-8 | yes |
| FX-TDIFF-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-009 frame 2: Alpha Channel Full On: every pixel covered, the clear corner black. | largest difference 9.7e-8 | yes |
| FX-TDIFF-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-010 frame 2: Alpha Channel Lightness Of Result: covered as far as the difference is light, so the still background goes clear. | largest difference 9.3e-8 | yes |
| FX-TDIFF-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-011 frame 2: Alpha Channel Max Of Result: covered by the strongest channel of the difference. | largest difference 1.5e-7 | yes |
| FX-TDIFF-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-012 frame 2: Alpha Channel Alpha Difference: covered by how much the covering changed, only on the stripe where the ball came and went. | largest difference 9.7e-8 | yes |
| FX-TDIFF-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-013 frame 2: Alpha Channel Alpha Difference Only: as FX-TDIFF-012, with every colour white. | largest difference 3.0e-8 | yes |
| FX-TDIFF-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-014 frame 2: The plate as the target, now: the background matches and goes black, only the ball and its trail show. | largest difference 9.7e-8 | yes |
| FX-TDIFF-014 frame 4: The plate as the target, now: the background matches and goes black, only the ball and its trail show. | largest difference 9.7e-8 | yes |
| FX-TDIFF-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-015 frame 2: The plate from frame 4 as the target: at frame 2 it is clear, so the layer shows as it is; at frame 4 it is FX-TDIFF-014. | largest difference 1.4e-7 | yes |
| FX-TDIFF-015 frame 4: The plate from frame 4 as the target: at frame 2 it is clear, so the layer shows as it is; at frame 4 it is FX-TDIFF-014. | largest difference 9.7e-8 | yes |
| FX-TDIFF-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-016 frame 0: A layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.4e-7 | yes |
| FX-TDIFF-016 frame 4: A layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.4e-7 | yes |
| FX-TDIFF-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-TDIFF-017 frame 2: A second layer of the same drawings as the target, one frame back: FX-TDIFF-002. | largest difference 9.7e-8 | yes |
| FX-TDIFF-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-018 frame 0: Time Offset keyed from 0 at frame 0 to -4 frames at frame 4: at frame 0 black; at frame 2 two frames back; at frame 4 four frames back. | largest difference 3.0e-8 | yes |
| FX-TDIFF-018 frame 2: Time Offset keyed from 0 at frame 0 to -4 frames at frame 4: at frame 0 black; at frame 2 two frames back; at frame 4 four frames back. | largest difference 9.7e-8 | yes |
| FX-TDIFF-018 frame 4: Time Offset keyed from 0 at frame 0 to -4 frames at frame 4: at frame 0 black; at frame 2 two frames back; at frame 4 four frames back. | largest difference 9.7e-8 | yes |
| FX-TDIFF-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-019 frame 2: FX-TDIFF-002 on the holder moved 2 right and 1 down: the same picture moved, since the target lies on the layer. | largest difference 9.7e-8 | yes |
| FX-TDIFF-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-020 frame 2: An Exposure of +1 before Time Difference, against itself now: the target is the drawing without its effects, so the brightening shows as a difference. | largest difference 3.7e-8 | yes |
| FX-TDIFF-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-021 frame 2: FX-TDIFF-004 with an Exposure of -1 after Time Difference: the grey picture darkened. | largest difference 3.8e-8 | yes |
| FX-TDIFF-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-022 frame 2: On an adjustment layer above the holder with no target named: an adjustment layer has no picture of its own, so nothing changes. | largest difference 1.4e-7 | yes |
| FX-TDIFF-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-023 frame 2: On an adjustment layer with the plate as the target: the frame beneath, here the holder's own rectangle, against the plate, FX-TDIFF-014. | largest difference 9.7e-8 | yes |
| FX-TDIFF-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TDIFF-024 frame 0: Contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-024 frame 4: Contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-025 frame 0: Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-025 frame 4: Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-026 frame 0: Time Offset 61 seconds, above 60. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-026 frame 4: Time Offset 61 seconds, above 60. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-027 frame 0: Time Offset -61 seconds, below -60. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-027 frame 4: Time Offset -61 seconds, below -60. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-028 frame 0: An Alpha Channel written "lightness". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-028 frame 4: An Alpha Channel written "lightness". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-029 frame 0: An Absolute Difference written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-029 frame 4: An Absolute Difference written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-030 frame 0: A target written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-030 frame 4: A target written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TDIFF-031 frame 0: Time Offset keyed to 100 seconds at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-031 frame 4: Time Offset keyed to 100 seconds at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TDIFF-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the ball, each with a Time Difference whose target is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Time Difference never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is, since none is a distance | TimeDifference { layer: String("plate"), time_offset: -0.5, contrast: 20.0, absolute_difference: "off", alpha_channel: "blend", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tdiff_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tdiff_030.json, its target written as the number 3, is saved as the number 3 | 3 | yes |
| the target frame the effect reads is never saved | None | yes |
| a file with no `layer` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `alpha_channel` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Contrast that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Absolute Difference that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Contrast 101 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Time Difference's contrast runs from 0 to 100, and this is 101. | yes |
| Time Offset -61 seconds is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Time Difference's time offset runs from -60 to 60, and this is -61. | yes |
| Absolute Difference "maybe" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Time Difference's absolute difference is "off" or "on", and this is "maybe". | yes |
| Alpha Channel "lightness" is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Time Difference's alpha channel is one of "original", "target", "blend", "max", "full_on", "lightness_of_result", "max_of_result", "alpha_difference", "alpha_difference_only", and this is "lightness". | yes |
| a target that is the number 3 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Time Difference's target is the name of a layer of this composition, and this is 3. | yes |
| Time Offset keyed to 100 seconds at frame 4 is refused with a sentence, and nothing changes | EFFECT_PARAMETER_INVALID Time Difference's time offset runs from -60 to 60, and this is 100. | yes |
| a Time Difference on `plate` naming `holder`, whose Time Difference names `plate` is refused with a sentence, and nothing changes | EFFECT_LAYER_CYCLE That layer setting would make layers read each other in a circle. | yes |
| naming the holder itself, which is no circle, is taken | taken | yes |
| no target named, one frame back is taken | taken | yes |
| Time Offset keyed from 0 at frame 0 to -1 second at frame 4 is taken | taken | yes |
| a Time Difference on `plate` naming `late` is taken | taken | yes |
| each of the nine Alpha Channel words, with the widest Time Offset and Contrast, is taken | original, target, blend, max, full_on, lightness_of_result, max_of_result, alpha_difference, alpha_difference_only | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a ball bouncing on ones over a night-blue solid, in `verification/D-521 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, frame 16 with no effect, draws cleanly, the ball in columns 282 to 330 | [], columns 282 to 329 | yes |
| 2_moved.png, one frame back, Alpha Channel Full On: black, the ball's old and new places lit, draws cleanly in columns 266 to 330 | [], columns 266 to 329 | yes |
| 3_grey.png, the same with Absolute Difference off: grey, the new place lighter, the old darker, draws cleanly in columns 266 to 330 | [], columns 266 to 329 | yes |
| 4_strong.png, the same as 2_moved with Contrast 30: every difference four times as strong, draws cleanly in columns 266 to 330 | [], columns 266 to 329 | yes |
| 5_moved_draft.png, 2_moved at Draft, draws cleanly, a quarter the size: columns 66 to 82 | [], 120 wide, columns 66 to 82 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tdiff_002.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tdiff_015.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tdiff_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tdiff_023.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

140 of 140 checks pass.
