# B-530: Pixel Motion Blur

D-650, under the owner's /loop request (P0-13, D-600: "Blur + retiming only"). Every expected pixel is `Fixtures/pixel_motion_blur/expected_pixel_motion_blur.json`, written by `tools/pixel_motion_blur_reference.py` and `tools/flow_reference.py` before this code existed and printed in document 25 as FX-PMB-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-PMB-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PMB-001 frame 0: As added: Automatic, so the composition's 180 degrees and 16 samples, Vector Detail 20. The disc is smeared along its run, half a frame's travel; at frame 0 the frame before is the layer's first, and at frame 5 the frame after is past its out point, so each smears one way only. | largest difference 2.8e-7 | yes |
| FX-PMB-001 frame 2: As added: Automatic, so the composition's 180 degrees and 16 samples, Vector Detail 20. The disc is smeared along its run, half a frame's travel; at frame 0 the frame before is the layer's first, and at frame 5 the frame after is past its out point, so each smears one way only. | largest difference 2.8e-7 | yes |
| FX-PMB-001 frame 5: As added: Automatic, so the composition's 180 degrees and 16 samples, Vector Detail 20. The disc is smeared along its run, half a frame's travel; at frame 0 the frame before is the layer's first, and at frame 5 the frame after is past its out point, so each smears one way only. | largest difference 2.8e-7 | yes |
| FX-PMB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-002 frame 2: Manual, 360 degrees in 8 samples: a whole frame's travel. | largest difference 2.2e-7 | yes |
| FX-PMB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-003 frame 2: Manual, 0 degrees: the shutter never opens, the layer exactly as it is. | largest difference 1.4e-7 | yes |
| FX-PMB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-004 frame 2: Manual, 720 degrees: two frames' travel, the farthest it goes. | largest difference 2.8e-7 | yes |
| FX-PMB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-005 frame 2: Vector Detail 100: a vector found for every pixel. | largest difference 2.8e-7 | yes |
| FX-PMB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-006 frame 2: Vector Detail 0: the coarsest vectors, on a picture this small those of Vector Detail 20. | largest difference 2.8e-7 | yes |
| FX-PMB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-007 frame 2: Vector Detail 50: one vector for every two pixels. | largest difference 2.8e-7 | yes |
| FX-PMB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-008 frame 2: A drawing that never moves: the estimate finds no motion and the layer stays as it is. | largest difference 2.8e-7 | yes |
| FX-PMB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-009 frame 2: The textured card sliding 1.5 to the right, as added. | largest difference 2.4e-7 | yes |
| FX-PMB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-010 frame 2: The card, Manual 360 degrees in 16 samples, Vector Detail 100. | largest difference 2.3e-7 | yes |
| FX-PMB-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-011 frame 2: Automatic with the composition's blur set to 90 degrees and 4 samples, its switch off: those are the ones used. | largest difference 1.5e-7 | yes |
| FX-PMB-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-012 frame 2: Manual, 360 degrees in the fewest samples, 2: two copies a half frame apart. | largest difference 1.6e-7 | yes |
| FX-PMB-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-013 frame 2: An Exposure of +1 before Pixel Motion Blur: the frames either side are brightened too, and the brighter picture is smeared. | largest difference 5.5e-7 | yes |
| FX-PMB-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-014 frame 2: An Exposure of -1 after Pixel Motion Blur darkens the smear too. | largest difference 1.4e-7 | yes |
| FX-PMB-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-015 frame 2: The holder moved 3 right and 2 down: FX-PMB-001 moved, since the smear lies on the layer. | largest difference 2.8e-7 | yes |
| FX-PMB-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-016 frame 2: The effect on an adjustment layer above the holder: nothing changes, and the frame says why. | largest difference 1.4e-7 | yes |
| FX-PMB-016: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-PMB-017 frame 0: Manual, the angle keyed from 0 at frame 0 to 360 at frame 4: 180 at frame 2. | largest difference 1.4e-7 | yes |
| FX-PMB-017 frame 2: Manual, the angle keyed from 0 at frame 0 to 360 at frame 4: 180 at frame 2. | largest difference 2.8e-7 | yes |
| FX-PMB-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-018 frame 2: Drawings on twos: at frame 2 the frame after shows the same drawing, so only the motion from the frame before smears; at frame 3 only the motion to the frame after. | largest difference 2.8e-7 | yes |
| FX-PMB-018 frame 3: Drawings on twos: at frame 2 the frame after shows the same drawing, so only the motion from the frame before smears; at frame 3 only the motion to the frame after. | largest difference 2.8e-7 | yes |
| FX-PMB-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-019 frame 2: The layer's in point at frame 2: at frame 2 the frame before is outside the layer, so only the motion to the frame after smears. | largest difference 2.8e-7 | yes |
| FX-PMB-019 frame 3: The layer's in point at frame 2: at frame 2 the frame before is outside the layer, so only the motion to the frame after smears. | largest difference 2.8e-7 | yes |
| FX-PMB-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PMB-020 frame 0: Shutter Angle 721, above 720. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-020 frame 4: Shutter Angle 721, above 720. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMB-021 frame 0: Shutter Samples 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-021 frame 4: Shutter Samples 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMB-022 frame 0: Vector Detail 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-022 frame 4: Vector Detail 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMB-023 frame 0: Shutter Control written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-023 frame 4: Shutter Control written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PMB-024 frame 0: Shutter Samples keyed to 100 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-024 frame 4: Shutter Samples keyed to 100 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-PMB-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| Pixel Motion Blur never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is, since none is a distance | PixelMotionBlur { shutter_control: "manual", shutter_angle: 360.0, shutter_samples: 8.0, vector_detail: 50.0, frames: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmb_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pmb_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the frames either side it reads are never saved | None | yes |
| a file with no `shutter_control` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `vector_detail` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an angle that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shutter Control that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Shutter Angle 721 is refused with a sentence, and nothing changes | Pixel Motion Blur's shutter angle runs from 0 to 720, and this is 721. | yes |
| Shutter Samples 1 is refused with a sentence, and nothing changes | Pixel Motion Blur's shutter samples runs from 2 to 64, and this is 1. | yes |
| Vector Detail -1 is refused with a sentence, and nothing changes | Pixel Motion Blur's vector detail runs from 0 to 100, and this is -1. | yes |
| Shutter Control "maybe" is refused with a sentence, and nothing changes | Pixel Motion Blur's shutter control is "automatic" or "manual", and this is "maybe". | yes |
| Shutter Samples keyed to 100 at frame 4 is refused with a sentence, and nothing changes | Pixel Motion Blur's shutter samples runs from 2 to 64, and this is 100. | yes |
| Manual, 360 degrees in 8 samples, Vector Detail 100 is taken | taken | yes |
| Shutter Angle keyed from 0 at frame 0 to 720 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a ball bouncing on ones over a night-blue solid, in `verification/D-650 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, frame 16 with no effect, draws cleanly | [], 1804 pixels show the ball | yes |
| 2_as_added.png, as added: half a frame's travel, draws cleanly, the smear covering more pixels than the picture before | [], 2911 pixels show the ball | yes |
| 3_one_frame.png, Manual 360 degrees: a whole frame's travel, draws cleanly, the smear covering more pixels than the picture before | [], 3935 pixels show the ball | yes |
| 4_two_frames.png, Manual 720 degrees in 32 samples: two frames' travel, draws cleanly, the smear covering more pixels than the picture before | [], 5987 pixels show the ball | yes |
| 5_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size | [], 120 wide | yes |

## Played frame after frame with the viewer's cache

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, frames 0 to 23 asked for in order through one viewer's cache, as playing asks: each is byte-identical to the same frame drawn alone with no cache, as an export draws it | 24 of 24 byte-identical | yes |
| drawing 17 (frame 16, the frame after 15) rewritten as drawing 1 after it was played: frame 15 through the same cache follows the new drawing, byte-identical to frame 15 drawn alone | byte-identical, changed | yes |
| FX-PMB-018 (on twos), frames 0 to 5 asked for in order through one viewer's cache: each byte-identical to the same frame drawn alone, though frames 2 and 3 show one drawing | 6 of 6 byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pmb_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmb_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmb_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pmb_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

104 of 104 checks pass.
