# B-532: Warp Stabilizer

D-653, under the owner's /loop request (P0-13, D-600: "Blur + retiming only"). Every expected pixel is `Fixtures/warp_stabilizer/expected_warp_stabilizer.json`, written by `tools/warp_stabilizer_reference.py` and `tools/flow_reference.py` before this code existed and printed in document 25 as FX-WS-001 to 030. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-WS-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WS-001 frame 0: As added: Smooth Motion at Smoothness 50, Position, Scale, Rotation, Stabilize, Crop, Auto-scale, on the card shaken about a steady pan. Frames 0 and 5 are the layer's ends, where the window closes to nothing, so they are as drawn; frames 2 and 3 are steadied and scaled up past their moving edges. | largest difference 1.6e-7 | yes |
| FX-WS-001 frame 2: As added: Smooth Motion at Smoothness 50, Position, Scale, Rotation, Stabilize, Crop, Auto-scale, on the card shaken about a steady pan. Frames 0 and 5 are the layer's ends, where the window closes to nothing, so they are as drawn; frames 2 and 3 are steadied and scaled up past their moving edges. | largest difference 2.0e-7 | yes |
| FX-WS-001 frame 3: As added: Smooth Motion at Smoothness 50, Position, Scale, Rotation, Stabilize, Crop, Auto-scale, on the card shaken about a steady pan. Frames 0 and 5 are the layer's ends, where the window closes to nothing, so they are as drawn; frames 2 and 3 are steadied and scaled up past their moving edges. | largest difference 1.9e-7 | yes |
| FX-WS-001 frame 5: As added: Smooth Motion at Smoothness 50, Position, Scale, Rotation, Stabilize, Crop, Auto-scale, on the card shaken about a steady pan. Frames 0 and 5 are the layer's ends, where the window closes to nothing, so they are as drawn; frames 2 and 3 are steadied and scaled up past their moving edges. | largest difference 1.5e-7 | yes |
| FX-WS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-002 frame 2: Stabilize Only: steadied, the edges the shake uncovers left clear. | largest difference 2.1e-7 | yes |
| FX-WS-002 frame 3: Stabilize Only: steadied, the edges the shake uncovers left clear. | largest difference 1.9e-7 | yes |
| FX-WS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-003 frame 2: Stabilize, Crop: steadied, cut to the largest middle rectangle the shake never uncovers, not scaled. | largest difference 2.1e-7 | yes |
| FX-WS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-004 frame 2: Method Position, Stabilize Only: moves only. | largest difference 1.4e-7 | yes |
| FX-WS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-005 frame 2: The card also turned a little each frame, Position, Scale, Rotation, Stabilize Only. | largest difference 2.1e-7 | yes |
| FX-WS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-006 frame 2: The turned card, Perspective, Stabilize Only. | largest difference 2.1e-7 | yes |
| FX-WS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-007 frame 2: The turned card, Preserve Scale on, Stabilize Only. | largest difference 2.0e-7 | yes |
| FX-WS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-008 frame 1: No Motion, Stabilize Only: every frame held to the layer's first, the pan taken out too. | largest difference 2.0e-7 | yes |
| FX-WS-008 frame 3: No Motion, Stabilize Only: every frame held to the layer's first, the pan taken out too. | largest difference 7.5e-7 | yes |
| FX-WS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-009 frame 2: Smoothness 0: the window is one frame, the layer exactly as it is. | largest difference 1.6e-7 | yes |
| FX-WS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-010 frame 2: Smoothness 8: one frame either side, where 50 reaches as far as the layer allows, two at frame 2. | largest difference 1.7e-7 | yes |
| FX-WS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-011 frame 2: Maximum Scale 101: the scale stops short, and the cut edges show clear. | largest difference 2.5e-7 | yes |
| FX-WS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-012 frame 2: Action-Safe Margin 10: scaled less, since a tenth of each side may stay cut. | largest difference 2.1e-7 | yes |
| FX-WS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-013 frame 2: Additional Scale 120 on Stabilize Only. | largest difference 2.1e-7 | yes |
| FX-WS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-014 frame 2: Additional Scale keyed from 100 at frame 0 to 140 at frame 4: 120 at frame 2. | largest difference 2.1e-7 | yes |
| FX-WS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-015 frame 2: Detailed Analysis on: the motion found with a vector for every pixel. | largest difference 1.7e-7 | yes |
| FX-WS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-016 frame 2: Pixel Motion Blur's disc running steadily over nothing, Stabilize Only: only its own pixels count, and a steady run is left nearly as it is. | largest difference 1.7e-7 | yes |
| FX-WS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-017 frame 2: An Exposure of +1 before Warp Stabilizer: the frames it reads are brightened too. | largest difference 4.6e-7 | yes |
| FX-WS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-018 frame 2: An Exposure of -1 after Warp Stabilizer darkens its picture. | largest difference 1.2e-7 | yes |
| FX-WS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-019 frame 2: The holder moved 3 right and 2 down: FX-WS-002 moved. | largest difference 2.1e-7 | yes |
| FX-WS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-020 frame 2: The layer's in point at frame 2: frame 2 is its first, as drawn; frame 3 has one frame either side. | largest difference 1.6e-7 | yes |
| FX-WS-020 frame 3: The layer's in point at frame 2: frame 2 is its first, as drawn; frame 3 has one frame either side. | largest difference 1.7e-7 | yes |
| FX-WS-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-021 frame 2: The effect on an adjustment layer above the holder: nothing changes, and the frame says why. | largest difference 1.6e-7 | yes |
| FX-WS-021: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-WS-022 frame 2: The shaken card on twos: the frames that repeat a drawing add no motion. | largest difference 1.9e-7 | yes |
| FX-WS-022 frame 3: The shaken card on twos: the frames that repeat a drawing add no motion. | largest difference 1.8e-7 | yes |
| FX-WS-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS-023 frame 0: Smoothness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-023 frame 4: Smoothness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-024 frame 0: Method written "subspace_warp", which this program leaves out. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-024 frame 4: Method written "subspace_warp", which this program leaves out. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-025 frame 0: Framing written "stabilize_synthesize_edges", which this program leaves out. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-025 frame 4: Framing written "stabilize_synthesize_edges", which this program leaves out. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-026 frame 0: Maximum Scale 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-026 frame 4: Maximum Scale 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-027 frame 0: Action-Safe Margin 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-027 frame 4: Action-Safe Margin 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-028 frame 0: Additional Scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-028 frame 4: Additional Scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-029 frame 0: Result written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-029 frame 4: Result written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS-030 frame 0: Maximum Scale keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS-030 frame 4: Maximum Scale keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Warp Stabilizer never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is: the move is found at the size drawn, and every setting is a share of the picture or a count of frames | WarpStabilizer { result: "smooth_motion", smoothness: 50.0, method: "position_scale_rotation", preserve_scale: "off", framing: "stabilize_crop_auto_scale", maximum_scale: 150.0, action_safe_margin: 0.0, additional_scale: 100.0, detailed_analysis: "off", picture: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ws_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the picture it makes is never saved | None | yes |
| a file with no `result` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `smoothness` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `detailed_analysis` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Smoothness that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Framing that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Smoothness 1001 is refused with a sentence, and nothing changes | Warp Stabilizer's smoothness runs from 0 to 1000, and this is 1001. | yes |
| Method "subspace_warp" is refused with a sentence, and nothing changes | Warp Stabilizer's method is "position", "position_scale_rotation" or "perspective", and this is "subspace_warp". | yes |
| Framing "stabilize_synthesize_edges" is refused with a sentence, and nothing changes | Warp Stabilizer's framing is "stabilize_only", "stabilize_crop" or "stabilize_crop_auto_scale", and this is "stabilize_synthesize_edges". | yes |
| Maximum Scale keyed to 2000 at frame 4 is refused with a sentence, and nothing changes | Warp Stabilizer's maximum scale runs from 100 to 1000, and this is 2000. | yes |
| No Motion, Perspective, Stabilize Only is taken | taken | yes |
| Additional Scale keyed from 100 at frame 0 to 140 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a striped scene filmed by a shaking hand, over a night-blue solid, in `verification/D-653 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_frame_3_no_effect.png, frame 3 with no effect, draws cleanly | [] | yes |
| frames 3 and 4 with no effect differ by the shake | 17.6 levels on average in the middle | yes |
| 3_as_added_frame_3.png and 4_as_added_frame_4.png, as added: draw cleanly, enlarged so no night blue shows at the edges | [] [], 0 and 0 night-blue pixels | yes |
| 5_stabilize_only_frame_4.png, Stabilize Only: draws cleanly, the edges the shake uncovers show the night blue | [], 1470 night-blue pixels | yes |
| 6_no_motion_frame_4.png, No Motion, Stabilize Only: draws cleanly, and frames 3 and 4 differ in the middle by under a third of what the shaken frames do | [], 0.3 levels against 17.6 | yes |
| 7_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size | [], 120 wide | yes |

## Played frame after frame with the viewer's cache

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, frames 0 to 7 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it | 8 of 8 byte-identical | yes |
| drawing 6 (frame 5, inside frame 3's window) rewritten as drawing 1 after it was played: frame 3 through the same cache follows the new drawing, byte-identical to frame 3 drawn alone | byte-identical, changed | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ws_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ws_003.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ws_006.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ws_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

124 of 124 checks pass.
