# B-24b: masks in the core

D-77, accepted by the owner on 2026-09-19. Every expected pixel is `Fixtures/masks/expected_masks.json`, written by `tools/mask_reference.py` before this code existed and printed in document 25 as FX-MSK-001 to 019; every case is drawn in `verification/B-24a proposal/mask_cases.png`. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6. FX-MSK-020 to 030 are files the build must refuse whole.

FX-MSK-001 is the one that says nothing moved: it is the rectangle B-06 drew, written D-77's way.

## FX-MSK-001 to 019, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MSK-001 frame 0: Today's rectangle written the new way: the left three columns, and every number the same as before D-77. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-002 frame 0: The same file written the old way, with one `mask` key: read as one Add mask, the same frame as FX-MSK-001. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-003 frame 0: A sloped edge: coverage in whole sixteenths. | largest difference 1.9e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-004 frame 0: Inverted: the three columns the mask keeps are the ones it now cuts. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-005 frame 0: At half opacity: the kept columns are half there. | largest difference 1.5e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-006 frame 0: Two masks, the second Add: both halves, so the whole frame. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-007 frame 0: Two masks, the second Subtract: the left three columns with its middle column taken out. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-008 frame 0: Two masks, the second Intersect: only where both are, column 2. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-009 frame 0: Two masks, the second Difference: where one is but not both. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-010 frame 0: A second mask in mode None takes no part: the frame of FX-MSK-001. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-011 frame 0: A first mask in Subtract takes from the whole layer: a hole in columns 2 and 3. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-012 frame 0: Expanded a quarter of a pixel: the rectangle grows on every side, its corners rounded. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-013 frame 0: Shrunk a quarter of a pixel: the same rectangle the other way. | largest difference 1.5e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-014 frame 0: Feathered two pixels: the hard edge at column 3 becomes a soft band. | largest difference 6.3e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-015 frame 0: A circle of four points with handles, filling the frame's height. | largest difference 2.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-016 frame 0: The same circle, expanded half a pixel. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-017 frame 0: A mask switched off takes no part: the whole drawing. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-MSK-018 frame 0: A mask of two points is kept, diagnosed and takes no part: the whole drawing. | largest difference 3.1e-8; tiles of 1 byte-identical; said MASK_INVALID_OUTLINE | yes |
| FX-MSK-019 frame 0: A mask whose points cross is kept, diagnosed and takes no part. | largest difference 3.1e-8; tiles of 1 byte-identical; said MASK_INVALID_OUTLINE | yes |

## FX-MSK-020 to 030, refused whole (D-77)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MSK-020: Both a `mask` and a `masks` key. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks: expected either the old `mask` key or D-77's `masks` list, not both: there is no way to tell which of the two the person drew. | yes |
| FX-MSK-021: A mode this build does not know. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/mode: expected one of D-77's mask modes: add, subtract, intersect, difference or none. | yes |
| FX-MSK-022: An opacity above 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/opacity: expected an opacity from 0 to 1 (D-77). | yes |
| FX-MSK-023: A feather below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/feather_px: expected a feather of 0 pixels or more (D-77). | yes |
| FX-MSK-024: An expansion past the 8192 pixel limit. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/expansion_px: expected an expansion from -8192 to 8192 pixels (D-77). | yes |
| FX-MSK-025: A point that is not two numbers. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/path/base/points/0/point: expected a pair of numbers: D-77 writes a mask point and each of its two handles as [x, y]. | yes |
| FX-MSK-026: A handle that is not two numbers. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/path/base/points/0/out: expected an array. | yes |
| FX-MSK-027: A mask with no path. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/path: expected this field to be present. | yes |
| FX-MSK-028: A `masks` key that is not a list. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks: expected an array. | yes |
| FX-MSK-029: Masks on an audio layer. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks: expected no masks on an audio layer, which draws nothing (D-71). | yes |
| FX-MSK-030: A keyed path whose key holds a different number of points. | ProjectSchemaInvalid: At /compositions/0/layers/0/masks/0/path/keyframes/0: expected a key holding the same number of points as the path's base: D-77 interpolates a path point by point, and there is no honest way to interpolate between outlines of different lengths. | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_msk_002.json's old `mask` key reads as one Add mask at full opacity, no feather, no expansion, no handles, named Mask 1 | 1 mask, name "Mask 1", mode add, opacity 1, feather 0, expansion 0, handles all zero: true | yes |
| and saving it writes `masks` and no `mask` key at all | masks: true, mask: absent | yes |
| the written mask holds its path as a base of points with both handles | {"in":[0,0],"out":[0,0],"point":[0,0]} | yes |
| and it opens again as the same project | equal | yes |
| fx_msk_016.json's handles survive a save and a load, to the last bit | MaskPoint { point: (3.0, 0.0), in_handle: (-0.5522847498307936, 0.0), out_handle: (0.5522847498307936, 0.0) } | yes |

## A keyed path: kept, drawn at its base, and said out loud (document 28)

| Check | The build's answer | Matches |
| --- | --- | --- |
| a path with one key opens, with PROJECT_FEATURE_UNSUPPORTED | [ProjectFeatureUnsupported] | yes |
| and frame 0 is the path's base, unmoved | largest difference 3.1e-8 | yes |
| and saving writes the keys back exactly as they were | 1 key(s) | yes |

## Commands (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| mask.set with two masks: both land, in order, with their modes | Ok(()); ["add", "subtract"] | yes |
| and one undo gives back the file as it was | the same project | yes |
| and the step is named for what it did | Set 2 masks | yes |
| mask.set with an opacity of 1.5: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Err((CommandInvalidValue, "Mask \"Mask 1\" was given an opacity of 1.5, which is not from 0 to 1.")) | yes |
| mask.set with a feather of -1: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Err((CommandInvalidValue, "Mask \"Mask 1\" was given a feather of -1 pixels, which is below 0.")) | yes |
| mask.set with an expansion of 9000: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Err((CommandInvalidValue, "Mask \"Mask 1\" was given an expansion of 9000 pixels, which is past 8192 either way.")) | yes |
| mask.set with a mask that crosses itself: MASK_INVALID_OUTLINE, refused rather than normalized | Err((MaskInvalidOutline, "The mask crosses itself, which this build does not draw.")) | yes |
| mask.set with an empty list clears them, and the layer is whole again | Ok(()); 0 masks | yes |

## The rule itself (D-77)

| Check | The build's answer | Matches |
| --- | --- | --- |
| a mask of corners hands the sampler its four vertices and nothing else | 4 points | yes |
| one curved segment four pixels long becomes the floor of 16 pieces, not fewer | 16 pieces | yes |
| a segment of 400 pixels becomes 200 pieces of about two pixels each | 200 pieces | yes |
| a first mask in Subtract takes its shape away from the whole layer rather than from nothing | left 0, right 1 | yes |
| a mask in mode none takes no part, so the layer is whole | whole | yes |
| moving every point ten pixels right carries the handles with it, unchanged | the same curve, ten pixels right | yes |
| a point with no handles is a corner | MaskPoint { point: (1.0, 2.0), in_handle: (0.0, 0.0), out_handle: (0.0, 0.0) } | yes |

## Result

53 of 53 checks pass.
