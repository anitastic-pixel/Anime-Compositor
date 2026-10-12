# B-470: CC Force Motion Blur

D-581 (a), accepted by the owner on 2026-10-12: "go with your recommendations on all six". Every expected pixel is `Fixtures/force_motion_blur/expected_force_motion_blur.json`, written by `tools/force_motion_blur_reference.py` before this code existed and printed in document 25 as FX-FMB-001 to 030. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-FMB-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FMB-001 frame 1: As added (8 samples, the composition's 180 degrees, its blur and the holder's switch on): on frame 1 the bar is drawn at 6 to 13, a pixel apart, an eighth each. The holder itself is still, so its own blur draws it once. | largest difference 0.0e0 | yes |
| FX-FMB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-002 frame 1: Override on at 180 degrees and 4 samples, in a composition whose blur is off and a holder whose switch is off: the bar at 6, 8, 10 and 12, a quarter each. | largest difference 0.0e0 | yes |
| FX-FMB-002 frame 2: Override on at 180 degrees and 4 samples, in a composition whose blur is off and a holder whose switch is off: the bar at 6, 8, 10 and 12, a quarter each. | largest difference 0.0e0 | yes |
| FX-FMB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-003 frame 1: Override off and the composition's blur off: no shutter, so the bar sharp at 6, as without the effect. | largest difference 0.0e0 | yes |
| FX-FMB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-004 frame 1: Override off and the holder's switch off: sharp at 6. | largest difference 0.0e0 | yes |
| FX-FMB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-005 frame 1: Override on at 360 degrees, 4 samples: a whole frame, the bar at 6, 10, 14 and 18, the last cut at the edge. | largest difference 0.0e0 | yes |
| FX-FMB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-006 frame 1: Override on at 0 degrees: one moment, the frame itself. | largest difference 0.0e0 | yes |
| FX-FMB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-007 frame 1: One sample: the frame itself, whatever the angle. | largest difference 0.0e0 | yes |
| FX-FMB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-008 frame 1: 2.9 samples count as 2: the bar at 6 and 10, a half each. | largest difference 0.0e0 | yes |
| FX-FMB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-009 frame 1: Three samples at 180 degrees: moments a sixth of a frame apart, the bar at 6, 8 2/3 and 11 1/3, so its edges fall between pixels. | largest difference 2.6e-8 | yes |
| FX-FMB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-010 frame 1: The bar still at 6 with its opacity keyed 0 to 1 over frames 0 to 2: read at each moment, 0.5, 0.5625, 0.625 and 0.6875, so 0.59375 on average. | largest difference 0.0e0 | yes |
| FX-FMB-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-011 frame 0: A held key at 720 degrees, 4 samples: the bar sits at 2 until frame 1, then 12; on frame 0 the moments are 0, 0.5, 1 and 1.5, so it is seen twice at half. | largest difference 0.0e0 | yes |
| FX-FMB-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-012 frame 0: A drawing red on frame 0 and blue after, at 720 degrees on frame 0: the moments reach past frame 1 but the drawing holds, so all four are red. | largest difference 0.0e0 | yes |
| FX-FMB-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-013 frame 1: Native Motion Blur on, with comp-in's own blur on (180, -90, 4) and the bar's switch on: each of the four moments is itself blurred, the bar at 3, 5, 7 and 9 plus 2k. | largest difference 0.0e0 | yes |
| FX-FMB-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-014 frame 1: The same with Native Motion Blur off: comp-in's blur is switched off inside, so FX-FMB-002. | largest difference 0.0e0 | yes |
| FX-FMB-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-015 frame 1: The bar still on a null running 16 pixels a frame: parents are read at each moment too, so FX-FMB-002. | largest difference 0.0e0 | yes |
| FX-FMB-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-016 frame 1: The bar's in point at frame 2, its run a frame later (at 6 on frame 2), at 720 degrees: on frame 1 the moments reach frame 2.5 but the in point holds, so nothing; on frame 2 the moments 2 to 3.5 put it at 6, 14, 22 and 30. | largest difference 0.0e0 | yes |
| FX-FMB-016 frame 2: The bar's in point at frame 2, its run a frame later (at 6 on frame 2), at 720 degrees: on frame 1 the moments reach frame 2.5 but the in point holds, so nothing; on frame 2 the moments 2 to 3.5 put it at 6, 14, 22 and 30. | largest difference 0.0e0 | yes |
| FX-FMB-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-017 frame 1: Two levels deep: the holder shows comp-mid, which shows comp-in. The moments reach every level, so FX-FMB-002. | largest difference 0.0e0 | yes |
| FX-FMB-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-018 frame 1: The holder itself moving 4 pixels a frame with its own switch on, in a composition blurred at 180, -90, 4, override off and 4 samples: the effect's picture is FX-FMB-002's, then the holder's own blur moves it at -0.75, -0.25, 0.25 and 0.75. | largest difference 0.0e0 | yes |
| FX-FMB-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-019 frame 2: The holder stretched to 200%: on frame 2 it shows comp-in's frame 1, and its moments are half as far apart there, the bar at 6, 7, 8 and 9. | largest difference 0.0e0 | yes |
| FX-FMB-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-020 frame 0: Samples keyed from 1 at frame 0 to 8 at frame 2: 4.5 on frame 1, so four moments, FX-FMB-002's frame; one on frame 0. | largest difference 0.0e0 | yes |
| FX-FMB-020 frame 1: Samples keyed from 1 at frame 0 to 8 at frame 2: 4.5 on frame 1, so four moments, FX-FMB-002's frame; one on frame 0. | largest difference 0.0e0 | yes |
| FX-FMB-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FMB-021 frame 1: The effect on a solid: only a composition layer is blurred, so the solid is drawn as it is, and each frame says so. | largest difference 0.0e0 | yes |
| FX-FMB-021: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_UNSUPPORTED"] | yes |
| FX-FMB-022 frame 1: The effect on an adjustment layer above the holder: the composition below is drawn sharp, and each frame says the effect does nothing there. | largest difference 0.0e0 | yes |
| FX-FMB-022: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_UNSUPPORTED"] | yes |
| FX-FMB-023 frame 1: A holder with a Time Remap (here one that changes nothing): not blurred yet, drawn as it is, and each frame says so. | largest difference 0.0e0 | yes |
| FX-FMB-023: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_UNSUPPORTED"] | yes |
| FX-FMB-024 frame 0: Motion Blur Samples 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-024 frame 1: Motion Blur Samples 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FMB-025 frame 0: Motion Blur Samples 65, above 64. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-025 frame 1: Motion Blur Samples 65, above 64. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FMB-026 frame 0: Shutter Angle 721, above 720. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-026 frame 1: Shutter Angle 721, above 720. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FMB-027 frame 0: Shutter Angle -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-027 frame 1: Shutter Angle -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FMB-028 frame 0: Override Native Shutter written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-028 frame 1: Override Native Shutter written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FMB-029 frame 0: Native Motion Blur written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-029 frame 1: Native Motion Blur written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FMB-030 frame 0: Motion Blur Samples keyed to 100 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-030 frame 1: Motion Blur Samples keyed to 100 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 | yes |
| FX-FMB-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| CC Force Motion Blur never grows the holder's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is, since none is a distance | ForceMotionBlur { motion_blur_samples: 8.0, override_native_shutter: "on", shutter_angle: 360.0, native_motion_blur: "off", picture: None } | yes |

## An effect setting keyed inside the holder is read at each moment

| Check | The build's answer | Matches |
| --- | --- | --- |
| the inner bar still, its Exposure keyed from 0 stops at frame 0 to 2 at frame 2; Override on, 180 degrees, 4 samples: frame 1 is the average of the bar with Stops fixed at 1, 1.125, 1.25 and 1.375 and no CC Force Motion Blur (within 1e-5) | largest difference 0e0 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fmb_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fmb_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the averaged picture it lays is never saved | None | yes |
| a file with no `native_motion_blur` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `motion_blur_samples` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a sample count that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Override Native Shutter that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Motion Blur Samples 0 is refused with a sentence, and nothing changes | CC Force Motion Blur's motion blur samples runs from 1 to 64, and this is 0. | yes |
| Motion Blur Samples 65 is refused with a sentence, and nothing changes | CC Force Motion Blur's motion blur samples runs from 1 to 64, and this is 65. | yes |
| Shutter Angle 721 is refused with a sentence, and nothing changes | CC Force Motion Blur's shutter angle runs from 0 to 720, and this is 721. | yes |
| Override Native Shutter "maybe" is refused with a sentence, and nothing changes | CC Force Motion Blur's override native shutter is "off" or "on", and this is "maybe". | yes |
| Motion Blur Samples keyed to 100 at frame 4 is refused with a sentence, and nothing changes | CC Force Motion Blur's motion blur samples runs from 1 to 64, and this is 100. | yes |
| 16 samples, Override on at 360 degrees, Native Motion Blur off is taken | taken | yes |
| Motion Blur Samples keyed from 1 at frame 0 to 8 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a white square crossing a night-blue solid inside a composition layer, in `verification/D-581 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, frame 6 with no effect: the square in columns 232 to 279, draws cleanly in columns 232 to 279 | [], columns 232 to 279 | yes |
| 2_as_added.png, as added, the composition's motion blur off: nothing to blur, the same columns, draws cleanly in columns 232 to 279 | [], columns 232 to 279 | yes |
| 3_override_180.png, Override on, 180 degrees, 8 samples: a smear 14 pixels ahead, draws cleanly in columns 232 to 293 | [], columns 232 to 293 | yes |
| 4_override_360.png, Override on, 360 degrees, 16 samples: a smear 30 pixels ahead, draws cleanly in columns 232 to 309 | [], columns 232 to 309 | yes |

## Played frame after frame with the viewer's cache (D-528)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the 360-degree picture, frames 0 to 23 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it | 24 of 24 byte-identical; 99532800 bytes kept for effects | yes |
| FX-FMB-013, native blur inside, frames 0 to 4 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it | 5 of 5 byte-identical; 3200 bytes kept for effects | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fmb_002.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fmb_013.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fmb_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fmb_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

126 of 126 checks pass.
