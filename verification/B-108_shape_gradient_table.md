# B-108b: gradient fills and strokes on shape layers

D-168, proposed on 2026-09-27. Every expected pixel is `Fixtures/shape_gradients/expected_shape_gradients.json`, written by `tools/shape_gradient_reference.py` as B-108a before this code existed and printed in document 25 as FX-SHP-040 to 051; every still case is drawn in `verification/B-108a proposal/gradient_cases.png` and the moving one in `verification/B-108a proposal/gradient_moving_case.png`. The answer is the largest difference over every sample, against the catalogue's tolerance of 1e-6. FX-SHP-060 to 069 are files the build must refuse whole.

## FX-SHP-040 to 051, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-040 frame 0: A linear gradient, red at the left edge to blue at the right, filling the frame: every column a step further from red to blue, mixed as encoded colour. | largest difference 2.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-041 frame 0: The same stops between x = 1 and x = 5: column 0 is red and column 5 blue exactly, because the ends pad outwards. | largest difference 1.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-042 frame 0: A radial gradient from the frame's middle (3, 1) out to (6, 1): the two middle columns nearly red and the colour turning blue with the distance, the same on each side. | largest difference 1.9e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-043 frame 0: Four stops, red and red to 0.5 then blue and blue: two stops at one offset are a hard step, so the left three columns are red and the right three blue. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-044 frame 0: Three stops, red, yellow at 0.5, blue: the middle of the frame passes through yellow. | largest difference 2.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-045 frame 0: A stop's opacity: blue fully there at the left, fading to nothing at the right. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-046 frame 0: The fill's own opacity at 50% still halves the gradient: FX-SHP-040 at half strength. | largest difference 1.3e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-047 frame 0: Start and end at the same point: the whole fill is the last stop's colour, the flat blue of a fill with no gradient. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-048 frame 0: FX-SHP-040's stops written blue first: stops are taken in the order of their offsets, so the frame is FX-SHP-040's. | largest difference 2.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-049 frame 0: A diagonal: from the top-left corner (0, 0) to the bottom-right (6, 2), so the bottom row is further along than the top. | largest difference 2.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-050 frame 0: A gradient stroke, one pixel wide, on a rectangle taller than the frame: the stroke's two upright bands take the gradient's colour where they stand, red on the left band and nearer blue on the right, with no fill between. | largest difference 1.3e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-051 frame 0: The start keyed from the left edge to the right and the end from the right to the left, over four frames, linear: the gradient turns round, and at frame 2, where the two meet, the fill is flat blue. | largest difference 2.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-051 frame 1: The start keyed from the left edge to the right and the end from the right to the left, over four frames, linear: the gradient turns round, and at frame 2, where the two meet, the fill is flat blue. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-051 frame 2: The start keyed from the left edge to the right and the end from the right to the left, over four frames, linear: the gradient turns round, and at frame 2, where the two meet, the fill is flat blue. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-051 frame 3: The start keyed from the left edge to the right and the end from the right to the left, over four frames, linear: the gradient turns round, and at frame 2, where the two meet, the fill is flat blue. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-051 frame 4: The start keyed from the left edge to the right and the end from the right to the left, over four frames, linear: the gradient turns round, and at frame 2, where the two meet, the fill is flat blue. | largest difference 2.6e-8; tiles of 1 byte-identical; said nothing | yes |

## FX-SHP-060 to 069, refused whole (D-168)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-060: A gradient of one stop. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill gradient of 2 to 64 stops, not 1. | yes |
| FX-SHP-061: A gradient of 65 stops, past the limit of 64. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill gradient of 2 to 64 stops, not 65. | yes |
| FX-SHP-062: A stop offset above 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill gradient stop offset from 0 to 1, not 1.5. | yes |
| FX-SHP-063: A stop colour above 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill gradient stop colour of three numbers from 0 to 1, not 1.5. | yes |
| FX-SHP-064: A stop opacity below 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill gradient stop opacity from 0 to 1, not -0.5. | yes |
| FX-SHP-065: A gradient type that is neither linear nor radial. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/fill/gradient/type: expected one of linear, radial. | yes |
| FX-SHP-066: A start point of one number rather than two. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/fill/gradient/start/base: expected an array. | yes |
| FX-SHP-067: An expression on the end point. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected no expression on a fill gradient end. | yes |
| FX-SHP-068: Motion-path handles on a key of the start point. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/fill/gradient/start/keyframes/0/spatial: expected no spatial: the motion path belongs to position keyframes and to no other property. | yes |
| FX-SHP-069: A gradient with no stops key. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/fill/gradient/stops: expected this field to be present. | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shp_044.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_050.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_051.json survives a save and a load, to the last bit | equal | yes |
| stops are saved in the order they were written, blue first, not re-sorted | [{"color":[0.2,0.5,0.8],"offset":1,"opacity":1},{"color":[0.8,0.2,0.1],"offset":0,"opacity":1}] | yes |
| and the fill keeps its own flat colour beside the gradient, for when it is taken off | [0.8,0.2,0.1] | yes |

## Result

31 of 31 checks pass.
