# B-533: Rolling Shutter Repair

D-654, under the owner's /loop request (P0-13, D-600: "Blur + retiming only"). Every expected pixel is `Fixtures/rolling_shutter_repair/expected_rolling_shutter_repair.json`, written by `tools/rolling_shutter_repair_reference.py`, `tools/warp_stabilizer_reference.py` and `tools/flow_reference.py` before this code existed and printed in document 25 as FX-RSR-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RSR-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RSR-001 frame 0: As added: Warp at 50 per cent, scanned top to bottom, on the card filmed scanning in 80 per cent of a frame. Frame 0 is the layer's first, its motion before taken as the motion after turned round; frame 5 its last. | largest difference 2.2e-7 | yes |
| FX-RSR-001 frame 2: As added: Warp at 50 per cent, scanned top to bottom, on the card filmed scanning in 80 per cent of a frame. Frame 0 is the layer's first, its motion before taken as the motion after turned round; frame 5 its last. | largest difference 3.4e-7 | yes |
| FX-RSR-001 frame 5: As added: Warp at 50 per cent, scanned top to bottom, on the card filmed scanning in 80 per cent of a frame. Frame 0 is the layer's first, its motion before taken as the motion after turned round; frame 5 its last. | largest difference 2.7e-7 | yes |
| FX-RSR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-002 frame 2: Rate 80, the camera's own: the slant taken out. | largest difference 7.2e-7 | yes |
| FX-RSR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-003 frame 2: Rate 0: the layer exactly as it is. | largest difference 1.6e-7 | yes |
| FX-RSR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-004 frame 2: Rate 100. | largest difference 6.1e-7 | yes |
| FX-RSR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-005 frame 2: Bottom to Top on the card scanned top to bottom: the slant doubled, not taken out. | largest difference 6.6e-7 | yes |
| FX-RSR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-006 frame 2: Left to Right on the card panning down and scanned left to right. | largest difference 2.5e-7 | yes |
| FX-RSR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-007 frame 2: Right to Left on the same card. | largest difference 3.3e-7 | yes |
| FX-RSR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-008 frame 2: Pixel Motion at 80 per cent. | largest difference 1.6e-6 | yes |
| FX-RSR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-009 frame 2: Pixel Motion, Pixel Motion Detail 100. | largest difference 1.2e-6 | yes |
| FX-RSR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-010 frame 2: Pixel Motion Detail keyed from 20 at frame 0 to 100 at frame 4: 60 at frame 2. | largest difference 1.5e-6 | yes |
| FX-RSR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-011 frame 2: Warp with Detailed Analysis on. | largest difference 2.6e-7 | yes |
| FX-RSR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-012 frame 2: Rate keyed from 0 at frame 0 to 100 at frame 4: 50 at frame 2, as added. | largest difference 3.4e-7 | yes |
| FX-RSR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-013 frame 2: Pixel Motion Blur's disc running over nothing, Pixel Motion at 100: only its own pixels move. | largest difference 1.8e-7 | yes |
| FX-RSR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-014 frame 2: An Exposure of +1 before Rolling Shutter Repair: the frames it reads are brightened too. | largest difference 1.2e-6 | yes |
| FX-RSR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-015 frame 2: An Exposure of -1 after Rolling Shutter Repair darkens its picture. | largest difference 3.4e-7 | yes |
| FX-RSR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-016 frame 2: The holder moved 3 right and 2 down: FX-RSR-001 moved. | largest difference 2.1e-7 | yes |
| FX-RSR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-017 frame 2: The layer's in point at frame 2: frame 2 is its first, its motion before taken as the motion after turned round. | largest difference 2.2e-7 | yes |
| FX-RSR-017 frame 3: The layer's in point at frame 2: frame 2 is its first, its motion before taken as the motion after turned round. | largest difference 2.2e-7 | yes |
| FX-RSR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-018 frame 2: The effect on an adjustment layer above the holder: nothing changes, and the frame says why. | largest difference 1.6e-7 | yes |
| FX-RSR-018: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-RSR-019 frame 2: The card on twos: each frame's held side takes the other side's motion turned round. | largest difference 4.8e-7 | yes |
| FX-RSR-019 frame 3: The card on twos: each frame's held side takes the other side's motion turned round. | largest difference 4.3e-7 | yes |
| FX-RSR-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSR-020 frame 0: Rolling Shutter Rate 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-020 frame 4: Rolling Shutter Rate 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSR-021 frame 0: Rolling Shutter Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-021 frame 4: Rolling Shutter Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSR-022 frame 0: Scan Direction written "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-022 frame 4: Scan Direction written "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSR-023 frame 0: Method written "optical". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-023 frame 4: Method written "optical". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSR-024 frame 0: Pixel Motion Detail 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-024 frame 4: Pixel Motion Detail 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSR-025 frame 0: Rolling Shutter Rate keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-025 frame 4: Rolling Shutter Rate keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSR-026 frame 0: Detailed Analysis written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-026 frame 4: Detailed Analysis written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-RSR-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Rolling Shutter Repair never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is: the motion is found at the size drawn, the rate is a share of a frame and the detail a share of the estimate's | RollingShutterRepair { rolling_shutter_rate: 50.0, scan_direction: "top_to_bottom", method: "warp", detailed_analysis: "off", pixel_motion_detail: 20.0, picture: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rsr_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rsr_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the picture it makes is never saved | None | yes |
| a file with no `rolling_shutter_rate` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `method` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `pixel_motion_detail` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Rolling Shutter Rate that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scan Direction that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Rolling Shutter Rate 101 is refused with a sentence, and nothing changes | Rolling Shutter Repair's rolling shutter rate runs from 0 to 100, and this is 101. | yes |
| Scan Direction "diagonal" is refused with a sentence, and nothing changes | Rolling Shutter Repair's scan direction is "top_to_bottom", "bottom_to_top", "left_to_right" or "right_to_left", and this is "diagonal". | yes |
| Method "optical" is refused with a sentence, and nothing changes | Rolling Shutter Repair's method is "warp" or "pixel_motion", and this is "optical". | yes |
| Pixel Motion Detail keyed to 150 at frame 4 is refused with a sentence, and nothing changes | Rolling Shutter Repair's pixel motion detail runs from 0 to 100, and this is 150. | yes |
| Pixel Motion, Left to Right, rate 80 is taken | taken | yes |
| Rolling Shutter Rate keyed from 0 at frame 0 to 100 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a striped scene filmed by a panning phone camera, over a night-blue solid, in `verification/D-654 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_frame_3_no_effect.png, frame 3 with no effect: draws cleanly, the stripes leaning by about the pan times the scan between the bands (6 x 0.8 / 2 = 2.4 pixels) | [], 2.40 pixels | yes |
| 2_as_added.png, as added, 50 per cent: the lean lessened; draws cleanly | [], 0.90 pixels | yes |
| 3_rate_80.png, rate 80, the camera's own: the stripes upright, the lean under a third; draws cleanly | [], 0.00 pixels | yes |
| 4_pixel_motion_80.png, Pixel Motion at 80: the lean under a half; draws cleanly | [], -0.00 pixels | yes |
| 5_bottom_to_top_80.png, the wrong Scan Direction: draws cleanly, the lean made worse, over one and a half times | [], 4.73 pixels | yes |
| 6_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size | [], 120 wide | yes |

## Played frame after frame with the viewer's cache

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, frames 0 to 7 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it | 8 of 8 byte-identical | yes |
| drawing 5 (frame 4, the frame after frame 3) rewritten as drawing 1 after it was played: frame 3 through the same cache follows the new drawing, byte-identical to frame 3 drawn alone | byte-identical, changed | yes |

## Warp's moves kept for a frame shown again (audit P5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, frame 3 drawn a second time uses both moves the first drawing worked, and is byte-identical to the first, drawn with nothing kept | 2 moves reused the second time, 0 the first; byte-identical | yes |
| Detailed Analysis switched on after frame 3 was drawn as added: no kept move is used, and frame 3 is byte-identical to frame 3 with Detailed Analysis drawn with nothing kept | 0 moves reused; byte-identical; differs from as added | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rsr_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rsr_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rsr_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rsr_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

111 of 111 checks pass.
