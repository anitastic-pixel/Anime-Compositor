# B-109b: trim paths on shape layers

D-169, proposed on 2026-09-27. Every expected pixel is `Fixtures/shape_trims/expected_shape_trims.json`, written by `tools/shape_trim_reference.py` as B-109a before this code existed and printed in document 25 as FX-SHP-070 to 083; every still case is drawn in `verification/B-109a proposal/trim_cases.png` and the moving one in `verification/B-109a proposal/trim_moving_case.png`. The answer is the largest difference over every sample, against the catalogue's tolerance of 1e-6. FX-SHP-090 to 095 are files the build must refuse whole.

## FX-SHP-070 to 083, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-070 frame 0: A one-pixel stroke along the frame's middle, trimmed 0 to 100 with no offset: the whole line, exactly as with no trim. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-071 frame 0: Trimmed 0 to 50: the left half, columns 0 to 2, ending in a round cap in column 3. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-072 frame 0: Trimmed 25 to 75: the middle, from x = 1.5 to 4.5, a round cap at each end. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-073 frame 0: Start 75 and end 25, the other way round: the same stretch as 25 to 75. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-074 frame 0: Start and end both 40: nothing is drawn. | largest difference 0.0e0; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-075 frame 0: 0 to 50 with an offset of 180 degrees, half a turn: the right half. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-076 frame 0: 0 to 50 with an offset of 270 degrees: the stretch runs off the right end and carries on from the left, so both ends of the line are drawn and the middle is not. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-077 frame 0: An offset of -90 degrees is the same as 270. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-078 frame 0: The closed rectangle taller than the frame, trimmed 0 to 50: its path starts at the top left and runs clockwise, and its first half is its top and its right side, of which only the right-hand band is in the frame. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-079 frame 0: The same with an offset of 180: the second half, and only the left-hand band shows. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-080 frame 0: 0 to 100 with an offset of 90 on a closed path: still the whole outline, the frame of FX-SHP-006. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-081 frame 0: A fill and a trimmed stroke: the fill is whole, and only the stroke is trimmed. | largest difference 1.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-082 frame 0: The ellipse's outline trimmed 0 to 25: a quarter of its length along the curve from its top, the right-hand upper quarter, measured on the flattened outline. | largest difference 3.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-083 frame 0: The line drawn on: the end keyed from 0 at frame 0 to 100 at frame 4, linear, so each frame draws a further quarter. | largest difference 0.0e0; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-083 frame 1: The line drawn on: the end keyed from 0 at frame 0 to 100 at frame 4, linear, so each frame draws a further quarter. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-083 frame 2: The line drawn on: the end keyed from 0 at frame 0 to 100 at frame 4, linear, so each frame draws a further quarter. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-083 frame 3: The line drawn on: the end keyed from 0 at frame 0 to 100 at frame 4, linear, so each frame draws a further quarter. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-083 frame 4: The line drawn on: the end keyed from 0 at frame 0 to 100 at frame 4, linear, so each frame draws a further quarter. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |

## FX-SHP-090 to 095, refused whole (D-169)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-090: A start below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a trim start from 0 to 100, not -10. | yes |
| FX-SHP-091: An end above 100. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a trim end from 0 to 100, not 150. | yes |
| FX-SHP-092: A key of the end above 100. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a trim end from 0 to 100, not 101. | yes |
| FX-SHP-093: An offset of two numbers rather than one. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/trim/offset/base: expected a number. | yes |
| FX-SHP-094: An expression on the start. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected no expression on a trim start. | yes |
| FX-SHP-095: A trim with no end. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/trim/end: expected this field to be present. | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shp_079.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_083.json survives a save and a load, to the last bit | equal | yes |
| a trim taken off is gone from the saved shape | trim absent | yes |

## Result

27 of 27 checks pass.
