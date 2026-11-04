# B-930: dashed strokes on shape layers

D-1050, accepted on 2026-10-27. Every expected pixel is `Fixtures/shape_dashes/expected_shape_dashes.json`, written by `tools/shape_dash_reference.py` before this code existed and printed in document 25 as FX-SHP-130 to 140; every still case is drawn in `verification/D-1050 pictures/dash_cases.png` and the moving one in `verification/D-1050 pictures/dash_moving_case.png`. The answer is the largest difference over every sample, against the catalogue's tolerance of 1e-6. FX-SHP-141 to 146 are files the build must refuse whole.

## FX-SHP-130 to 140, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-130 frame 0: The one-pixel line along the frame's middle, dash 1 and gap 2, round caps: dashes from x = 0 to 1 and 3 to 4, each with a round cap half a pixel past its ends. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-131 frame 0: The same with butt caps: columns 0 and 3 exactly, nothing else. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-132 frame 0: Butt caps, offset 1: the pattern moves one pixel back along the line, so columns 2 and 5. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-133 frame 0: Butt caps, offset -1: the same as offset 2, columns 1 and 4. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-134 frame 0: A gap of 0: the whole line, exactly as with no dashes. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-135 frame 0: Dash 0, gap 2, round caps: round dots at x = 0, 2, 4 and 6. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-136 frame 0: Dash 0.25 and gap 0.25, under a pixel together: the whole line. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-137 frame 0: The closed rectangle taller than the frame, dash 2 and gap 1, butt caps: its right side is the dash from y = -1 to 1 and from 2 to 3, so the right band is drawn in the top row and not the bottom; its left side is the same, its top row the last dash, which runs on through the start into the first. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-138 frame 0: Trimmed 50 to 100, then dash 1 and gap 1, butt caps: the dashes are measured from the trimmed line's start, so columns 3 and 5. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-139 frame 0: A fill and a dashed stroke: the fill is whole; only the stroke is dashed. | largest difference 1.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-140 frame 0: Butt caps, dash 1 and gap 2, the offset keyed from 0 at frame 0 to 3 at frame 3, linear: the dashes march back along the line a pixel a frame, and frames 3 and 4 are frame 0 again. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-140 frame 1: Butt caps, dash 1 and gap 2, the offset keyed from 0 at frame 0 to 3 at frame 3, linear: the dashes march back along the line a pixel a frame, and frames 3 and 4 are frame 0 again. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-140 frame 2: Butt caps, dash 1 and gap 2, the offset keyed from 0 at frame 0 to 3 at frame 3, linear: the dashes march back along the line a pixel a frame, and frames 3 and 4 are frame 0 again. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-140 frame 3: Butt caps, dash 1 and gap 2, the offset keyed from 0 at frame 0 to 3 at frame 3, linear: the dashes march back along the line a pixel a frame, and frames 3 and 4 are frame 0 again. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-140 frame 4: Butt caps, dash 1 and gap 2, the offset keyed from 0 at frame 0 to 3 at frame 3, linear: the dashes march back along the line a pixel a frame, and frames 3 and 4 are frame 0 again. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |

## FX-SHP-141 to 146, refused whole (D-1050)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-141: A dash below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a dash dash from 0 to 10000, not -1. | yes |
| FX-SHP-142: A gap below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a dash gap from 0 to 10000, not -0.5. | yes |
| FX-SHP-143: A key of the gap above 10000. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a dash gap from 0 to 10000, not 20000. | yes |
| FX-SHP-144: An offset of two numbers rather than one. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/dashes/offset/base: expected a number. | yes |
| FX-SHP-145: An expression on the dash. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected no expression on a dash dash. | yes |
| FX-SHP-146: Dashes with no gap. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/dashes/gap: expected this field to be present. | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shp_138.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_140.json survives a save and a load, to the last bit | equal | yes |
| dashes taken off is gone from the saved shape | dashes absent | yes |

## Result

24 of 24 checks pass.
