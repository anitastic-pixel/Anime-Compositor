# B-124b: motion blur, on the processor

D-188 and ADR-019, accepted on 2026-09-28 by the owner's words "I approve of motion blur plan". Every expected time, value, pixel and point is `Fixtures/motion_blur/expected_motion_blur.json`, written by `tools/motion_blur_reference.py` before this code existed and printed in document 25 as FX-MB-001 to 050. Times and values are held to 1e-12, points to 1e-9 and pixels to 1e-6; the answer is the largest difference.

## FX-MB-001 to 005: the moments inside a frame's shutter

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MB-001: As added: shutter 180, phase -90, 16 samples, on frame 10. | 16 moments, largest difference 0.0e0 | yes |
| FX-MB-002: Shutter 360, phase 0, 4 samples, on frame 0: the whole of frame 0. | 4 moments, largest difference 0.0e0 | yes |
| FX-MB-003: Shutter 720, phase -360, 2 samples, on frame 5: a frame either side. | 2 moments, largest difference 0.0e0 | yes |
| FX-MB-004: Shutter 90, phase 90, 3 samples, on frame 0: after the frame, in thirds. | 3 moments, largest difference 0.0e0 | yes |
| FX-MB-005: Shutter 0: off, and the frame itself, whatever the phase and samples. | 1 moments, largest difference 0.0e0 | yes |

## FX-MB-006 to 009: a property read between frames

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MB-006: A linear key from 0 at frame 0 to 100 at frame 4. | largest difference 0.0e0 | yes |
| FX-MB-007: The same with easy ease: the value is 3u^2 - 2u^3 of the way. | largest difference 0.0e0 | yes |
| FX-MB-008: A hold from 10 at frame 2 to 20 at frame 3: it changes at exactly frame 3. | largest difference 0.0e0 | yes |
| FX-MB-009: A position on a curved path, (0, 0) at frame 0 leaving straight down, (100, 100) at frame 2 arriving from the left. | largest difference 0.0e0 | yes |

## FX-MB-010 to 028: pictures (20 by 1 pixels, 24 frames a second)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MB-010 frame 1: A still bar with both switches on and the composition's shutter as added: it is drawn once, the same picture, bit for bit, as with either switch off. | largest difference 0.0e0 | yes |
| FX-MB-011 frame 1: The bar moving 16 pixels a frame, both switches on, shutter 180, phase -90, 4 samples: on frame 1 it is drawn at 3, 5, 7 and 9, and each pixel is the share of the four that cover it. | largest difference 0.0e0 | yes |
| FX-MB-012 frame 1: The same with the composition's switch off: sharp, at 6. | largest difference 0.0e0 | yes |
| FX-MB-013 frame 1: The same with the composition's switch on and the layer's off: sharp. | largest difference 0.0e0 | yes |
| FX-MB-014 frame 1: A file written before motion blur, with neither field: sharp, and saved again it still has neither. | largest difference 0.0e0 | yes |
| FX-MB-015 frame 1: Phase 0: the shutter opens on the frame, so the bar is drawn at 7, 9, 11 and 13, all of it ahead of where it is on the frame. | largest difference 0.0e0 | yes |
| FX-MB-016 frame 1: Shutter 360, phase -180: a whole frame of travel, drawn at 0, 4, 8 and 12, so sixteen pixels are covered a quarter each. | largest difference 0.0e0 | yes |
| FX-MB-017 frame 1: Shutter 0 is motion blur off, whatever the phase: sharp, at 6. | largest difference 0.0e0 | yes |
| FX-MB-018 frame 1: The bar still and the camera panning 16 pixels a frame the other way: the same picture as FX-MB-011, because the camera is read at each moment too. | largest difference 0.0e0 | yes |
| FX-MB-019 frame 1: The bar still on a null that moves 16 pixels a frame: the same picture as FX-MB-011. The bar's switch decides; a null has none. | largest difference 0.0e0 | yes |
| FX-MB-020 frame 0: A held key: the bar sits at 2 and jumps to 12 on frame 1. Frames 0 and 2 are sharp; on frame 1 half the moments are before the jump and half after, so it is seen twice at half strength. | largest difference 0.0e0 | yes |
| FX-MB-020 frame 1: A held key: the bar sits at 2 and jumps to 12 on frame 1. Frames 0 and 2 are sharp; on frame 1 half the moments are before the jump and half after, so it is seen twice at half strength. | largest difference 0.0e0 | yes |
| FX-MB-020 frame 2: A held key: the bar sits at 2 and jumps to 12 on frame 1. Frames 0 and 2 are sharp; on frame 1 half the moments are before the jump and half after, so it is seen twice at half strength. | largest difference 0.0e0 | yes |
| FX-MB-021 frame 1: The opacity keyed 0 to 1 over frames 0 to 2 is read at the whole frame: FX-MB-011's picture at exactly half. | largest difference 0.0e0 | yes |
| FX-MB-022 frame 0: The layer starts on frame 1: on frame 0 it is not there, and on frame 1 all four moments are drawn, including the two before its in point. | largest difference 0.0e0 | yes |
| FX-MB-022 frame 1: The layer starts on frame 1: on frame 0 it is not there, and on frame 1 all four moments are drawn, including the two before its in point. | largest difference 0.0e0 | yes |
| FX-MB-023 frame 1: Drawings hold: the red drawing is exposed on frame 0 and the blue from frame 1. On frame 1 every moment shows the blue, and no red, although two of the moments fall in frame 0. | largest difference 0.0e0 | yes |
| FX-MB-024 frame 1: A still matte over columns 0 to 7 cuts the moving bar after it is blurred. | largest difference 0.0e0 | yes |
| FX-MB-025 frame 1: A still bar with its switch on is drawn once; its matte moves with its own switch on, so the matte is blurred and the bar is cut by the blurred matte. | largest difference 0.0e0 | yes |
| FX-MB-026 frame 1: As added: shutter 180, phase -90, 16 samples. The bar is drawn at 2.25, 2.75 and on to 9.75, between pixels, so each moment is resampled as document 21 says before the sixteen are averaged. | largest difference 0.0e0 | yes |
| FX-MB-027 frame 1: A shape layer whose one shape is the bar: the same picture as FX-MB-011. | largest difference 0.0e0 | yes |
| FX-MB-028 frame 1: A composition layer showing a composition that holds the bar: the same picture as FX-MB-011. The composition inside is drawn once, at the whole frame. | largest difference 0.0e0 | yes |

## What the pictures claim beyond their numbers

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MB-010: the still bar with its switch on is the picture with it off, bit for bit | the same bits | yes |
| FX-MB-014: the file from before motion blur, saved again, holds what it held and neither field | the same, neither field | yes |
| fx_mb_011.json opened and saved holds what it held, both switches included | the same | yes |
| fx_mb_016.json opened and saved holds what it held, both switches included | the same | yes |
| fx_mb_025.json opened and saved holds what it held, both switches included | the same | yes |
| FX-MB-011's frame leaves the bar's moments for the card to add up (B-156b; before it, the processor averaged them, B-152) | true | yes |
| FX-MB-025's frame, whose matte alone is blurred, is marked too | true | yes |
| FX-MB-010's still bar, drawn once, is not marked: the card may draw it | false | yes |
| Draft uses the same four moments: FX-MB-011 at Draft is the average of the bar drawn still at each | largest difference 0.0e0 | yes |

## FX-MB-050: where a corner lands at each moment (1920 by 1080)

| Check | The build's answer | Matches |
| --- | --- | --- |
| frame 0, moment -0.1875: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 0.0e0; 0.0e0 | yes |
| frame 0, moment -0.0625: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 0.0e0; 0.0e0 | yes |
| frame 0, moment 0.0625: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 5.7e-14; 2.3e-13 | yes |
| frame 0, moment 0.1875: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 0.0e0; 0.0e0 | yes |
| frame 12, moment 11.8125: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 0.0e0; 2.3e-13 | yes |
| frame 12, moment 11.9375: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 8.5e-14; 3.4e-13 | yes |
| frame 12, moment 12.0625: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 2.8e-14; 2.3e-13 | yes |
| frame 12, moment 12.1875: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen | true; 0.0e0; 1.1e-13 | yes |

## FX-MB-030 to 041: files that are refused

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MB-030: A shutter angle above 720. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-031: A shutter angle below 0. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-032: A shutter phase above 360. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-033: One sample. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-034: Sixty-five samples. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-035: Samples that are not a whole number. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-036: A composition's record with no samples. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-037: A composition's record with a field this build does not know. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-038: A composition's switch that is not true or false. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-039: A layer's switch that is not true or false. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-040: The switch on a null, which draws nothing. | PROJECT_SCHEMA_INVALID | yes |
| FX-MB-041: The switch on an adjustment layer. | PROJECT_SCHEMA_INVALID | yes |

## The two commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| the composition's shutter with an angle above 720 is refused | true | yes |
| the composition's shutter with an angle below 0 is refused | true | yes |
| the composition's shutter with a phase below -360 is refused | true | yes |
| the composition's shutter with one sample is refused | true | yes |
| the composition's shutter with sixty-five samples is refused | true | yes |
| the composition's shutter with an angle that is not a number is refused | true | yes |
| shutter 180, phase -90, 16 samples is taken, and undo puts back the file's 4 | taken true, 16 samples, undone true | yes |
| the switch on a null is refused | true | yes |
| switching FX-MB-011's bar off draws FX-MB-013's sharp picture | largest difference 0.0e0 | yes |

## Pictures for the playtest, frame 5 of a 960 by 540 shot, in `verification/B-124 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| sharp.png, the composition's switch off: both sharp: draws cleanly; a point near the card's corner and one in the bar's sweep change only where blurred; the ground is untouched | []; card false, bar false, ground true | yes |
| as_added.png, switched on as added, shutter 180, phase -90, 16 samples: draws cleanly; a point near the card's corner and one in the bar's sweep change only where blurred; the ground is untouched | []; card true, bar true, ground true | yes |
| shutter_360.png, shutter 360, phase -180, 32 samples: a whole frame of travel: draws cleanly; a point near the card's corner and one in the bar's sweep change only where blurred; the ground is untouched | []; card true, bar true, ground true | yes |
| four_samples.png, shutter 180 with only 4 samples: four copies can be seen: draws cleanly; a point near the card's corner and one in the bar's sweep change only where blurred; the ground is untouched | []; card true, bar true, ground true | yes |
| bar_switch_off.png, as added with the bar's own switch off: the card blurs, the bar is sharp: draws cleanly; a point near the card's corner and one in the bar's sweep change only where blurred; the ground is untouched | []; card true, bar false, ground true | yes |
| draft.png, as added at Draft, a quarter of the size each way, draws cleanly at 240 by 135 | [] | yes |

## Result

75 of 75 checks pass.
