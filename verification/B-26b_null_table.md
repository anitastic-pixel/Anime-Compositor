# B-26b: null layers

D-82, accepted by the owner on 2026-09-23. Every expected pixel is `Fixtures/null/expected_null.json`, written by `tools/null_reference.py` before this code existed and printed in document 25 as FX-NULL-001 to 007. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6. FX-NULL-020 to 029 are files the build must refuse whole.

## FX-NULL-001 to 007, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NULL-001 frame 0: A null on its own, switched on: every pixel transparent. | largest difference 0.0e0; tiles of 1 byte-identical | yes |
| FX-NULL-002 frame 0: A null switched on above the red and grey drawing: the drawing, untouched. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-003 frame 0: The drawing parented to a null that stands two pixels right: the drawing moves two pixels right, and columns 0 and 1 are empty. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-004 frame 0: The same with the null switched off: the same frame. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-005 frame 0: The same with the null alive on frame 1 only: every frame is moved. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-005 frame 1: The same with the null alive on frame 1 only: every frame is moved. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-005 frame 2: The same with the null alive on frame 1 only: every frame is moved. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-006 frame 0: The same with the null at a tenth opacity: the drawing stays opaque, as opacity does not pass to a child. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-NULL-007 frame 0: A null parented to a null, each a pixel right: the drawing moves two, as in FX-NULL-003. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |

## FX-NULL-020 to 029, refused whole (D-82)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NULL-020: A null that names an asset. | ProjectSchemaInvalid: At /compositions/0/layers/1/asset_id: expected no asset_id on an adjustment, composition, solid, shape or null layer, which has no drawing of its own (D-66, D-67, D-74, D-78, D-82). | yes |
| FX-NULL-021: A null with exposures. | ProjectSchemaInvalid: At /compositions/0/layers/1/exposure_spans: expected no exposure_spans on a null layer, which is never drawn (D-82). | yes |
| FX-NULL-022: A null with a source offset. | ProjectSchemaInvalid: At /compositions/0/layers/1/source_offset_frames: expected no source_offset_frames on a null layer, which is never drawn (D-82). | yes |
| FX-NULL-023: A null carrying a solid record. | ProjectSchemaInvalid: At /compositions/0/layers/1/solid: expected no solid record on a layer whose kind is not solid (D-74). | yes |
| FX-NULL-024: A null carrying shapes. | ProjectSchemaInvalid: At /compositions/0/layers/1/shapes: expected no shapes on a layer whose kind is not shape (D-78). | yes |
| FX-NULL-025: A null with a mask. | ProjectSchemaInvalid: At /compositions/0/layers/1/masks: expected no mask, effect or matte on a null layer, and blend mode normal, because it is never drawn (D-82). | yes |
| FX-NULL-026: A null with an effect. | ProjectSchemaInvalid: At /compositions/0/layers/1/effects: expected no mask, effect or matte on a null layer, and blend mode normal, because it is never drawn (D-82). | yes |
| FX-NULL-027: A null with a matte of its own. | ProjectSchemaInvalid: At /compositions/0/layers/1/matte: expected no mask, effect or matte on a null layer, and blend mode normal, because it is never drawn (D-82). | yes |
| FX-NULL-028: A null with a blend mode other than normal. | ProjectSchemaInvalid: At /compositions/0/layers/1/blend_mode: expected no mask, effect or matte on a null layer, and blend mode normal, because it is never drawn (D-82). | yes |
| FX-NULL-029: A drawing whose matte is a null. | ProjectSchemaInvalid: At /compositions/comp-main/layers: expected layer bg not to have the null layer null as its matte (D-82). | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_null_007.json's inner null written back as kind null, with its parent, and none of asset_id, exposures, source offset, solid or shapes | kind "null", parent "outer", drawing keys present [] | yes |
| and it opens again as the same project | equal | yes |

## Commands (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| layer.add_null: a new null needs no asset; its anchor is the middle of its 100 by 100 outline and its position the composition's centre | added true, anchor Vec2(50.0, 50.0), position Vec2(3.0, 1.0) | yes |
| and the frame with a second null added is the frame without it, byte for byte | identical | yes |
| layer.add of a null carrying a blend mode: COMMAND_INVALID_VALUE, and nothing changes | Some(CommandInvalidValue) | yes |
| giving the null a mask: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Some(CommandInvalidValue) | yes |
| giving the null an effect: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Some(CommandInvalidValue) | yes |
| giving the null blend mode multiply: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Some(CommandInvalidValue) | yes |
| giving the null a matte of its own: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Some(CommandInvalidValue) | yes |
| giving the null exposures: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Some(CommandInvalidValue) | yes |
| the drawing given the null as its matte: COMMAND_INVALID_VALUE, and nothing changes | Some(CommandInvalidValue) | yes |
| and what a null does have still works: renaming it | true | yes |
| parent.set: the drawing parented to the null at (52, 50) stays exactly where it was | parented true; frame unchanged | yes |
| and one undo gives back the file as it was | the same project | yes |
| the kind reads back as null | null | yes |

## Result

34 of 34 checks pass.
