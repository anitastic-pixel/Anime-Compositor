# B-25b: shape layers in the core

D-78, accepted by the owner on 2026-09-20. Every expected pixel is `Fixtures/shapes/expected_shapes.json`, written by `tools/shape_reference.py` as B-25a before this code existed and printed in document 25 as FX-SHP-001 to 017; every still case is drawn in `verification/B-25a proposal/shape_cases.png` and the moving one in `verification/B-25a proposal/shape_moving_case.png`. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6. FX-SHP-020 to 030 are files the build must refuse whole.

The window and the commands are B-25c and are not in this table.

## FX-SHP-001 to 017, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-001 frame 0: A filled rectangle on the left three columns, its edges on pixel boundaries: the same sixteen decisions a mask's edge is made of. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-002 frame 0: The same rectangle ending half way through column 2: that column is exactly half covered, which is the edge quantum ADR-016 states. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-003 frame 0: A sloped edge: coverage in whole sixteenths. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-004 frame 0: An ellipse of four curved segments, filling the frame's height. | largest difference 3.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-005 frame 0: The same rectangle at half fill opacity: half there, and nothing else changed. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-006 frame 0: A stroke and no fill, one pixel wide, on a rectangle taller than the frame: a band half a pixel either side of each upright edge, and the middle of the rectangle empty, because a stroke is on the line and not within it. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-007 frame 0: A stroke on an open path of two points, one pixel wide: the band runs the length of the line and ends in a half circle at each end, because a cap is round by definition. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-008 frame 0: Fill and stroke together on that rectangle: the stroke is laid over its own fill, so column 1 is the fill's colour alone, column 3 the stroke's alone, and the columns they share carry the stroke over the fill. | largest difference 1.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-009 frame 0: Two shapes, the second covering the first: the list is drawn first to last. | largest difference 6.0e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-010 frame 0: A shape layer over the drawing: the drawing shows everywhere the shape does not, and the shape is opaque where it does. | largest difference 3.1e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-011 frame 0: An open path with both a fill and a stroke: the fill closes it with a straight line from its last point to its first, while the stroke does not run along that line. | largest difference 1.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-012 frame 0: A path that crosses itself, filled: the even-odd rule says what is inside, and a shape is drawn rather than diagnosed for it, which is where a shape and a mask part company. | largest difference 3.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-013 frame 0: A mask on the shape layer, keeping its left three columns: a shape layer is masked like any other, after its shapes are drawn - the frame of FX-SHP-001. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-014 frame 0: A shape with neither a fill nor a stroke: nothing is drawn and nothing is said, because a path a person has not decided about yet is not a fault. | largest difference 0.0e0; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-015 frame 0: A shape switched off takes no part: the empty frame. | largest difference 0.0e0; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-016 frame 0: A shape of one point is kept, diagnosed and draws nothing: there is nothing to fill and nothing to stroke between. | largest difference 0.0e0; tiles of 1 byte-identical; said SHAPE_INVALID_OUTLINE | yes |
| FX-SHP-017 frame 0: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the fill slides three columns in four frames, three quarters of a column a frame, by D-77's rule and document 20's. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-017 frame 1: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the fill slides three columns in four frames, three quarters of a column a frame, by D-77's rule and document 20's. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-017 frame 2: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the fill slides three columns in four frames, three quarters of a column a frame, by D-77's rule and document 20's. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-017 frame 3: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the fill slides three columns in four frames, three quarters of a column a frame, by D-77's rule and document 20's. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-017 frame 4: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the fill slides three columns in four frames, three quarters of a column a frame, by D-77's rule and document 20's. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |

## FX-SHP-020 to 030, refused whole (D-78)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-020: A shape layer that names an asset. | ProjectSchemaInvalid: At /compositions/0/layers/0/asset_id: expected no asset_id on an adjustment, composition, solid or shape layer, which has no drawing of its own (D-66, D-67, D-74, D-78). | yes |
| FX-SHP-021: A shape layer with exposures. | ProjectSchemaInvalid: At /compositions/0/layers/0/exposure_spans: expected no exposure_spans on a shape layer, whose drawing is the shapes it carries (D-78). | yes |
| FX-SHP-022: A shape layer with a source offset. | ProjectSchemaInvalid: At /compositions/0/layers/0/source_offset_frames: expected no source_offset_frames on a shape layer, whose drawing is the shapes it carries (D-78). | yes |
| FX-SHP-023: A `shapes` key that is not a list. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes: expected an array. | yes |
| FX-SHP-024: A shape with no path. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/path: expected this field to be present. | yes |
| FX-SHP-025: A `closed` that is not true or false. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/closed: expected true or false. | yes |
| FX-SHP-026: A fill colour above 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill colour of three numbers from 0 to 1, not 1.5. | yes |
| FX-SHP-027: A fill opacity below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill opacity from 0 to 1, not -0.5. | yes |
| FX-SHP-028: A stroke width of 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a stroke width above 0 and at most 8192, not 0. | yes |
| FX-SHP-029: A stroke width past the 8192 pixel limit. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a stroke width above 0 and at most 8192, not 8193. | yes |
| FX-SHP-030: A raster layer carrying a `shapes` key. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes: expected no shapes on a layer whose kind is not shape (D-78). | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shp_004.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_011.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_017.json survives a save and a load, to the last bit | equal | yes |
| a saved shape layer says `shape` and carries a `shapes` list | kind "shape", shapes true | yes |
| and carries no asset, no exposures and no source offset, having no footage behind it | asset_id absent, exposure_spans absent, source_offset_frames absent | yes |
| and its shape holds its path as a base of points with both handles, as a mask's does | {"in":[0,0],"out":[0,0],"point":[1,1.5]} | yes |
| and its `closed` is written out, so an open path stays open | false | yes |
| fx_shp_017.json's keyed path reads back as two keys on the shape itself | 2 keys, at frames [0, 4] | yes |

## The rule itself (D-78)

| Check | The build's answer | Matches |
| --- | --- | --- |
| a path of four corners hands the sampler its four vertices whether it is closed or open | closed 4, open 4 | yes |
| and closing it is what puts the fourth side there: the left edge is on the path of a closed square and two pixels off an open one | closed 0.000, open 2.000 | yes |
| a stroke two pixels wide reaches exactly one pixel past the end of an open path, in a circle | at the end 0.000, one right of it 1.000, one diagonal from it 1.000 | yes |
| a path that crosses itself is drawn as a shape though the same outline is refused as a mask | shape draws: true, mask simple: false | yes |
| a shape of one point draws nothing and is the SHAPE_INVALID_OUTLINE case | renderable false, catalogued true | yes |
| a shape with neither a fill nor a stroke draws nothing and says nothing: not a fault | problem None | yes |
| a fill colour above 1 is a problem the build can say in a sentence | Some("a fill colour of three numbers from 0 to 1, not 1.5") | yes |
| a fill opacity below 0 is a problem the build can say in a sentence | Some("a fill opacity from 0 to 1, not -0.5") | yes |
| a stroke width of 0 is a problem the build can say in a sentence | Some("a stroke width above 0 and at most 8192, not 0") | yes |
| a stroke width past 8192 is a problem the build can say in a sentence | Some("a stroke width above 0 and at most 8192, not 8193") | yes |
| a shape layer's picture is the composition's size, and it starts transparent black | 6 by 2; the far corner [0.0, 0.0, 0.0, 0.0] | yes |

## Result

51 of 51 checks pass.
