# B-531: Timewarp

D-652, under the owner's /loop request (P0-13, D-600: "Blur + retiming only"). Every expected pixel is `Fixtures/timewarp/expected_timewarp.json`, written by `tools/timewarp_reference.py` and `tools/flow_reference.py` before this code existed and printed in document 25 as FX-TW-001 to 033. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TW-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TW-001 frame 0: As added: Pixel Motion at Speed 50. Frame 0 is the first drawing (the source time, a quarter frame before the in point, held there); frames 1, 2 and 5 are made a quarter, three quarters and a quarter of the way between drawings. | largest difference 1.4e-7 | yes |
| FX-TW-001 frame 1: As added: Pixel Motion at Speed 50. Frame 0 is the first drawing (the source time, a quarter frame before the in point, held there); frames 1, 2 and 5 are made a quarter, three quarters and a quarter of the way between drawings. | largest difference 1.6e-7 | yes |
| FX-TW-001 frame 2: As added: Pixel Motion at Speed 50. Frame 0 is the first drawing (the source time, a quarter frame before the in point, held there); frames 1, 2 and 5 are made a quarter, three quarters and a quarter of the way between drawings. | largest difference 1.7e-7 | yes |
| FX-TW-001 frame 5: As added: Pixel Motion at Speed 50. Frame 0 is the first drawing (the source time, a quarter frame before the in point, held there); frames 1, 2 and 5 are made a quarter, three quarters and a quarter of the way between drawings. | largest difference 1.6e-7 | yes |
| FX-TW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-002 frame 1: Whole Frames at Speed 50: each drawing shown twice, no new pictures. | largest difference 1.4e-7 | yes |
| FX-TW-002 frame 2: Whole Frames at Speed 50: each drawing shown twice, no new pictures. | largest difference 1.4e-7 | yes |
| FX-TW-002 frame 3: Whole Frames at Speed 50: each drawing shown twice, no new pictures. | largest difference 1.4e-7 | yes |
| FX-TW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-003 frame 1: Frame Mix at Speed 50: the two drawings either side faded together. | largest difference 1.1e-7 | yes |
| FX-TW-003 frame 2: Frame Mix at Speed 50: the two drawings either side faded together. | largest difference 1.2e-7 | yes |
| FX-TW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-004 frame 0: Speed 100: the layer exactly as it is. | largest difference 1.4e-7 | yes |
| FX-TW-004 frame 3: Speed 100: the layer exactly as it is. | largest difference 1.4e-7 | yes |
| FX-TW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-005 frame 1: Speed 200: twice as fast, made halfway between drawings 3 and 4 at frame 1; from frame 3 on it holds the last drawing. | largest difference 1.6e-7 | yes |
| FX-TW-005 frame 3: Speed 200: twice as fast, made halfway between drawings 3 and 4 at frame 1; from frame 3 on it holds the last drawing. | largest difference 1.4e-7 | yes |
| FX-TW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-006 frame 2: Speed keyed from 200 at frame 0 down to 0 at frame 4: it slows to a stop at source time 4.5. | largest difference 1.6e-7 | yes |
| FX-TW-006 frame 4: Speed keyed from 200 at frame 0 down to 0 at frame 4: it slows to a stop at source time 4.5. | largest difference 1.5e-7 | yes |
| FX-TW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-007 frame 1: Source Frame keyed from 0 at frame 0 to 5 at frame 4: frame 1 shows source frame 1.25 and frame 2 shows 2.5. | largest difference 1.6e-7 | yes |
| FX-TW-007 frame 2: Source Frame keyed from 0 at frame 0 to 5 at frame 4: frame 1 shows source frame 1.25 and frame 2 shows 2.5. | largest difference 1.6e-7 | yes |
| FX-TW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-008 frame 0: Source Frame 3.5, Frame Mix: halfway between drawings 4 and 5 on every frame. | largest difference 1.1e-7 | yes |
| FX-TW-008 frame 4: Source Frame 3.5, Frame Mix: halfway between drawings 4 and 5 on every frame. | largest difference 1.1e-7 | yes |
| FX-TW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-009 frame 1: Build From One Image: a quarter of the way uses the drawing before only, three quarters the drawing after only. | largest difference 1.6e-7 | yes |
| FX-TW-009 frame 2: Build From One Image: a quarter of the way uses the drawing before only, three quarters the drawing after only. | largest difference 1.7e-7 | yes |
| FX-TW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-010 frame 1: Error Threshold 0: never mixed toward Frame Mix. | largest difference 1.7e-7 | yes |
| FX-TW-010 frame 2: Error Threshold 0: never mixed toward Frame Mix. | largest difference 1.7e-7 | yes |
| FX-TW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-011 frame 1: Error Threshold 10: wherever the two warped pictures disagree, mixed toward Frame Mix. | largest difference 2.1e-7 | yes |
| FX-TW-011 frame 2: Error Threshold 10: wherever the two warped pictures disagree, mixed toward Frame Mix. | largest difference 2.4e-7 | yes |
| FX-TW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-012 frame 1: Filtering Extreme: the warped pictures read by the cubic. | largest difference 2.2e-7 | yes |
| FX-TW-012 frame 2: Filtering Extreme: the warped pictures read by the cubic. | largest difference 2.2e-7 | yes |
| FX-TW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-013 frame 2: Correct Luminance Changes on, on the textured card, whose mean brightness changes as it slides. | largest difference 2.0e-7 | yes |
| FX-TW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-014 frame 2: Tuning changed: Vector Detail 100, Global Smoothness 1, Local Smoothness 0, 8 iterations, Block Size 4, weights 1, 0 and 0. | largest difference 1.6e-7 | yes |
| FX-TW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-015 frame 2: Motion Blur on, Automatic, with the composition's 180 degrees in 8 samples (its switch off): the moments spread a quarter frame of source time. | largest difference 1.3e-7 | yes |
| FX-TW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-016 frame 1: Motion Blur on, Manual, 360 degrees in 4 samples, Whole Frames at Speed 200: drawings 2, 3, 3 and 4 averaged, the source times 1.75, 2.25, 2.75 and 3.25. | largest difference 1.1e-7 | yes |
| FX-TW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-017 frame 2: Motion Blur on, Manual, 360 degrees in 4 samples, Frame Mix on keyed Source Frame. | largest difference 1.1e-7 | yes |
| FX-TW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-018 frame 2: Motion Blur on, Manual, 0 degrees: as with it off. | largest difference 1.7e-7 | yes |
| FX-TW-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-019 frame 1: The textured card, as added. | largest difference 2.1e-7 | yes |
| FX-TW-019 frame 2: The textured card, as added. | largest difference 1.9e-7 | yes |
| FX-TW-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-020 frame 1: An Exposure of +1 before Timewarp: the frames it reads are brightened too. | largest difference 3.1e-7 | yes |
| FX-TW-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-021 frame 1: An Exposure of -1 after Timewarp darkens its picture. | largest difference 8.5e-8 | yes |
| FX-TW-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-022 frame 2: The layer's in point at frame 2: its source time starts there. | largest difference 1.4e-7 | yes |
| FX-TW-022 frame 3: The layer's in point at frame 2: its source time starts there. | largest difference 1.6e-7 | yes |
| FX-TW-022 frame 4: The layer's in point at frame 2: its source time starts there. | largest difference 1.7e-7 | yes |
| FX-TW-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-023 frame 1: The holder moved 3 right and 2 down: FX-TW-001 moved. | largest difference 1.6e-7 | yes |
| FX-TW-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-024 frame 2: The effect on an adjustment layer above the holder: nothing changes, and the frame says why. | largest difference 1.4e-7 | yes |
| FX-TW-024: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-TW-025 frame 1: Drawings on twos at Speed 50: between two frames of one drawing nothing moves, so that drawing shows exactly. | largest difference 1.4e-7 | yes |
| FX-TW-025 frame 2: Drawings on twos at Speed 50: between two frames of one drawing nothing moves, so that drawing shows exactly. | largest difference 1.4e-7 | yes |
| FX-TW-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-026 frame 0: Speed -100: the source time runs back from the in point, held at the first drawing. | largest difference 1.4e-7 | yes |
| FX-TW-026 frame 3: Speed -100: the source time runs back from the in point, held at the first drawing. | largest difference 1.4e-7 | yes |
| FX-TW-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TW-027 frame 0: Speed 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-027 frame 4: Speed 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TW-028 frame 0: Method written "optical". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-028 frame 4: Method written "optical". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TW-029 frame 0: Error Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-029 frame 4: Error Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TW-030 frame 0: Smoothing Iterations 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-030 frame 4: Smoothing Iterations 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TW-031 frame 0: Red Weight 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-031 frame 4: Red Weight 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TW-032 frame 0: Filtering written "sinc". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-032 frame 4: Filtering written "sinc". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TW-033 frame 0: Shutter Samples keyed to 100 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-033 frame 4: Shutter Samples keyed to 100 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-TW-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Timewarp never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is: Block Size is counted at each size the motion is found at, as Vector Detail is | Timewarp { method: "pixel_motion", adjust_time_by: "speed", speed: 50.0, source_frame: 0.0, vector_detail: 20.0, global_smoothness: 0.5, local_smoothness: 0.5, smoothing_iterations: 4.0, build_from_one_image: "off", correct_luminance_changes: "off", filtering: "normal", error_threshold: 1.0, block_size: 6.0, red_weight: 0.3, green_weight: 0.6, blue_weight: 0.1, enable_motion_blur: "off", shutter_control: "automatic", shutter_angle: 180.0, shutter_samples: 5.0, picture: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tw_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tw_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the picture it makes is never saved | None | yes |
| a file with no `method` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `speed` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `blue_weight` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Speed that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Filtering that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Speed 1001 is refused with a sentence, and nothing changes | Timewarp's speed runs from -1000 to 1000, and this is 1001. | yes |
| Error Threshold -1 is refused with a sentence, and nothing changes | Timewarp's error threshold runs from 0 to 10, and this is -1. | yes |
| Method "optical" is refused with a sentence, and nothing changes | Timewarp's method is "whole_frames", "frame_mix" or "pixel_motion", and this is "optical". | yes |
| Shutter Samples keyed to 100 at frame 4 is refused with a sentence, and nothing changes | Timewarp's shutter samples runs from 2 to 64, and this is 100. | yes |
| Frame Mix at Speed 200, Error Threshold 10 is taken | taken | yes |
| Speed keyed from 100 at frame 0 to -100 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a ball bouncing on ones over a night-blue solid, at half speed, in `verification/D-652 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_frame_8_no_effect.png, frame 8 with no effect, draws cleanly | [], 1814 pixels show the ball | yes |
| 3_whole_frames.png, Whole Frames at Speed 50, frame 17: draws cleanly, byte-identical to frame 8 with no effect | [], byte-identical | yes |
| 4_frame_mix.png, Frame Mix at Speed 50, frame 17: draws cleanly, two faint balls covering more pixels than one | [], 2609 pixels show a ball | yes |
| 5_pixel_motion.png, as added (Pixel Motion at Speed 50), frame 17: draws cleanly, one ball between the two, covering fewer pixels than Frame Mix's two | [], 1925 pixels show the ball | yes |
| 6_pixel_motion_draft.png, as added at Draft, draws cleanly, a quarter the size | [], 120 wide | yes |

## Played frame after frame with the viewer's cache

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, frames 0 to 23 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it | 24 of 24 byte-identical | yes |
| drawing 10 (source frame 9, which frame 17 is made from) rewritten as drawing 1 after it was played: frame 17 through the same cache follows the new drawing, byte-identical to frame 17 drawn alone | byte-identical, changed | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tw_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tw_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tw_016.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tw_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

145 of 145 checks pass.
