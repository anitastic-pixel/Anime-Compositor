# B-23b: solid layers

D-74, accepted by the owner on 2026-09-19. Every expected pixel is `Fixtures/solid/expected_solid.json`, written by `tools/solid_reference.py` before this code existed and printed in document 25 as FX-SOL-001 to 008. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6. FX-SOL-020 to 029 are files the build must refuse whole.

## FX-SOL-001 to 008, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SOL-001 frame 0: A solid the size of the frame, on nothing: every pixel its colour, opaque. | largest difference 1.2e-8; tiles of 1 byte-identical | yes |
| FX-SOL-002 frame 0: The same at half opacity over the red and grey drawing. | largest difference 2.9e-8; tiles of 1 byte-identical | yes |
| FX-SOL-003 frame 0: Two pixels square, centred at (3, 1): only columns 2 and 3 are its colour. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-SOL-004 frame 0: A mask keeping its left three columns. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-SOL-005 frame 0: One stop of exposure on the solid doubles its colour, past 1 where it goes. | largest difference 2.4e-8; tiles of 1 byte-identical | yes |
| FX-SOL-006 frame 0: The small solid as a matte-only layer for the drawing: the drawing shows in columns 2 and 3 only. | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-SOL-007 frame 0: Multiplied onto the drawing: each colour times the solid's. | largest difference 2.4e-8; tiles of 1 byte-identical | yes |
| FX-SOL-008 frame 0: Only between its in and out frames (frame 1 only). | largest difference 3.1e-8; tiles of 1 byte-identical | yes |
| FX-SOL-008 frame 1: Only between its in and out frames (frame 1 only). | largest difference 1.2e-8; tiles of 1 byte-identical | yes |
| FX-SOL-008 frame 2: Only between its in and out frames (frame 1 only). | largest difference 3.1e-8; tiles of 1 byte-identical | yes |

## FX-SOL-020 to 029, refused whole (D-74)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SOL-020: A solid that names an asset. | ProjectSchemaInvalid: At /compositions/0/layers/0/asset_id: expected no asset_id on an adjustment, composition, solid or shape layer, which has no drawing of its own (D-66, D-67, D-74, D-78). | yes |
| FX-SOL-021: A solid with exposures. | ProjectSchemaInvalid: At /compositions/0/layers/0/exposure_spans: expected no exposure_spans on a solid layer, which has one drawing of one colour (D-74). | yes |
| FX-SOL-022: A solid with a source offset. | ProjectSchemaInvalid: At /compositions/0/layers/0/source_offset_frames: expected no source_offset_frames on a solid layer, which has one drawing of one colour (D-74). | yes |
| FX-SOL-023: A solid with no solid record. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid: expected this field to be present. | yes |
| FX-SOL-024: A colour of two numbers. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid/color: expected three numbers from 0 to 1 (D-74). | yes |
| FX-SOL-025: A colour above 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid: expected a colour of three numbers from 0 to 1, not 1.5. | yes |
| FX-SOL-026: A colour below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid: expected a colour of three numbers from 0 to 1, not -0.1. | yes |
| FX-SOL-027: A width of 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid: expected a width and height from 1 to 8192, not 0. | yes |
| FX-SOL-028: A height that is not a whole number. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid/height: expected a whole number that is not negative. | yes |
| FX-SOL-029: A raster layer carrying a solid record. | ProjectSchemaInvalid: At /compositions/0/layers/0/solid: expected no solid record on a layer whose kind is not solid (D-74). | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sol_006.json's solid written back: the fixture's keys in the fixture's order, apart from the retired `mask`, and no asset_id, exposures or source offset | id, kind, name, solid, enabled, locked, in_frame, out_frame, transform, matte, blend_mode, effects | yes |
| and its record written back | {"color":[0.2,0.5,0.8],"height":2,"width":2} | yes |
| and it opens again as the same project | equal | yes |

## Commands (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| layer.add_solid: a new solid needs no asset; its anchor is its own centre and its position the composition's | added true, anchor Vec2(3.0, 1.0), position Vec2(3.0, 1.0) | yes |
| layer.add_solid 8193 wide: COMMAND_INVALID_VALUE, in a sentence | Some((CommandInvalidValue, "That solid cannot be made: it needs a width and height from 1 to 8192, not 8193.")) | yes |
| solid.set red, 12 by 8, on the 6 by 2 solid: its record, and its anchor at the same fraction (3, 1) to (6, 4) | Ok(()); Some(Solid { color: [1.0, 0.0, 0.0], width: 12, height: 8 }), anchor Vec2(6.0, 4.0) | yes |
| and one undo gives back the file as it was | the same project | yes |
| solid.set colour 1.5: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Err((CommandInvalidValue, "That solid cannot be made: it needs a colour of three numbers from 0 to 1, not 1.5.")) | yes |
| solid.set width 0: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Err((CommandInvalidValue, "That solid cannot be made: it needs a width and height from 1 to 8192, not 0.")) | yes |
| solid.set height 8193: COMMAND_INVALID_VALUE, in a sentence, and nothing changes | Err((CommandInvalidValue, "That solid cannot be made: it needs a width and height from 1 to 8192, not 8193.")) | yes |
| solid.set on a drawing: COMMAND_INVALID_VALUE | Some(CommandInvalidValue) | yes |
| exposures set on a solid, which has none: COMMAND_INVALID_VALUE rather than dropped on save | Some(CommandInvalidValue) | yes |
| a keyed anchor: every key moves to the same fraction too, (0, 2) to (0, 4) | [Vec2(0.0, 4.0)] | yes |
| the kind reads back as solid | solid | yes |

## Result

34 of 34 checks pass.
