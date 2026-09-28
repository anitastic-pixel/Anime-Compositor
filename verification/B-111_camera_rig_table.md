# B-111b: a camera that rides a layer

D-171, proposed on 2026-09-27. Every expected number is `Fixtures/camera_rig/expected_camera_rig.json`, written by `tools/camera_rig_reference.py` before this code existed and printed in document 25 as FX-RIG-001 to 031. Frames are compared sample by sample to within 1e-6, as FX-NULL's are, because a picture is decoded and drawn in 32-bit numbers; points to within 1e-9. The answer given is the largest difference.

## FX-RIG-001 to 005, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIG-001 frame 0: The camera rides a null standing at the middle of the frame, right over the null's own anchor: the frame is exactly the frame with no camera at all. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-002 frame 0: The null two pixels right: the camera goes with it, so the drawing moves two pixels left, and columns 4 and 5 are empty. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-003 frame 0: The null keyed from the middle at frame 0 to two pixels right at frame 2: the drawing slides left one pixel a frame. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-003 frame 1: The null keyed from the middle at frame 0 to two pixels right at frame 2: the drawing slides left one pixel a frame. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-003 frame 2: The null keyed from the middle at frame 0 to two pixels right at frame 2: the drawing slides left one pixel a frame. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-004 frame 0: The null on a second null, each a pixel right: the camera rides the whole chain, and the drawing moves two pixels left, as in FX-RIG-002. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-005 frame 0: The camera one pixel right of the null, in the null's space, with the null at the middle: the drawing moves one pixel left. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-RIG-001 is not merely close: the frame is byte for byte the frame with no camera | identical | yes |

## FX-RIG-010 to 012, where the corners land (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIG-010 frame 0: The null turned a quarter, clockwise, with the camera a pixel right of it in its space: the camera's point swings down to (3, 2), and the picture moves up one pixel without turning, because the view never turns. | camera at (3, 2), depth -8.333333333333334; corners (0, -1), (6, -1), (0, 1), (6, 1); largest difference 0.0e0 | yes |
| FX-RIG-011 frame 0: The null one lens length back, at depth -8.33: the camera rides back with it to twice as far from the drawing, which is drawn at half size about the middle. | camera at (3, 1), depth -16.666666666666668; corners (1.5, 0.5), (4.5, 0.5), (1.5, 1.5), (4.5, 1.5); largest difference 0.0e0 | yes |
| FX-RIG-012 frame 0: The null at 200%, with the camera a pixel right of it in its space: the camera's point is two pixels right, and the picture is not made bigger, because a camera's zoom is its own. | camera at (5, 1), depth -8.333333333333334; corners (-2, 0), (4, 0), (-2, 2), (4, 2); largest difference 0.0e0 | yes |

## FX-RIG-020, opened with a warning (document 28)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIG-020: The camera names a parent that is not in the composition: the file opens, `PARENT_REFERENCE_MISSING` says so, the reference is kept, and the camera is where it would be with no parent, which here is over the middle. | warnings ["ParentReferenceMissing"]; parent kept as Some(Id("gone")) and written back as "gone"; largest difference 3.1e-8 | yes |

## FX-RIG-030 and 031, refused whole (D-171)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIG-030: The camera's parent is a number, not a layer's identifier. | ProjectSchemaInvalid: At /compositions/0/camera/parent: expected a string. | yes |
| FX-RIG-031: The camera's parent is an audio layer, which has no place (D-71). | ProjectSchemaInvalid: At /compositions/comp-main/camera/parent: expected a parent that is not the audio layer sound (D-71). | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rig_004.json's camera written back with its parent | parent "rig" | yes |
| and it opens again as the same project | equal | yes |

## Commands (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| camera.set_parent to none, keeping place at frame 0: the camera stays over the middle, so frame 0 is as it was | done true; position Vec2(3.0, 1.0); frame 0 unchanged | yes |
| and the null's slide no longer moves the shot: frame 2 is frame 0 | the same | yes |
| camera.set_parent back to the null at frame 0: the slide comes back, every frame as it was | done true; frames as before | yes |
| and two undos give back the file as it was | the same project | yes |
| FX-RIG-010's camera taken off the turned null, keeping place: it stands where it stood | done true; from (3, 2) depth -8.333333333333334 to (3, 2) depth -8.333333333333334; largest difference 0.0e0 | yes |
| FX-RIG-011's camera taken off the null one lens length back: its own depth takes the null's | depth before -16.666666666666668 and after -16.666666666666668; written Scalar(-16.666666666666668) | yes |
| camera.set_parent to a parent that is not in the composition: PARENT_REFERENCE_MISSING, and nothing changes | Some(ParentReferenceMissing) | yes |
| camera.set_parent to the audio layer: COMMAND_INVALID_VALUE, and nothing changes | Some(CommandInvalidValue) | yes |

## Result

24 of 24 checks pass.
