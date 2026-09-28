# B-110b: shape styles over time, and line joins and caps

D-170, proposed on 2026-09-27. Every expected pixel is `Fixtures/shape_styles/expected_shape_styles.json`, written by `tools/shape_style_reference.py` as B-110a before this code existed and printed in document 25 as FX-SHP-100 to 117; every still case is drawn in `verification/B-110a proposal/style_cases.png` and the moving ones in `verification/B-110a proposal/style_moving_cases.png`. The answer is the largest difference over every sample, against the catalogue's tolerance of 1e-6. FX-SHP-120 to 127 are files the build must refuse whole.

## FX-SHP-100 to 117, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-100 frame 0: A one-pixel line from (1, 1) to (5, 1) with butt caps: it stops square at its end points, so it covers x = 1 to 5 and half of each row. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-101 frame 0: The same with square caps: carried on half the width past each end, x = 0.5 to 5.5, square. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-102 frame 0: The same with round caps written in: exactly D-78's line with nothing written, FX-SHP-008. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-103 frame 0: A chevron, (1, 0) to (4, 1) to (1, 2), one pixel wide, with a mitre join and the default mitre limit of 4 and butt caps: the corner is 3.16 half-widths long, inside the limit, so it comes to a sharp point at x = 5.58. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-104 frame 0: The same with a mitre limit of 3: 3.16 is past it, so the corner is bevelled, exactly FX-SHP-105. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-105 frame 0: The chevron with a bevel join: the corner cut off straight. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-106 frame 0: The chevron with a round join: the corner rounded, reaching x = 4.5. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-107 frame 0: A closed diamond, (1, 1), (3, 0.2), (5, 1), (3, 1.8), mitred: both its side points are sharp, the one at its first point, (1, 1), included, because a closed path is joined where it closes. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-108 frame 0: The diamond bevelled. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-109 frame 0: The line trimmed 25 to 75 with butt caps: the cut ends are butt too, x = 2 to 4 exactly. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-110 frame 0: Trimmed the same with square caps: x = 1.5 to 4.5, square. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-111 frame 0: The mitred diamond trimmed 0 to 50 with an offset of 270 and butt caps: the stretch is its last side and its first, running through its first point, (1, 1), where it is one line with its mitre rather than two cut ends. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-112 frame 0: A line from (1, 1) to (4, 1) and back to (2, 1), mitred with butt caps: a mitre on a line that turns straight back would be endless, so there is none, and nothing is drawn past x = 4. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-113 frame 0: An open path of two points both at (2, 1), with square caps: a square one pixel across, lined up with the frame, x = 1.5 to 2.5. | largest difference 3.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-114 frame 0: The same with butt caps: nothing is drawn. | largest difference 0.0e0; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-115 frame 0: The line with its width keyed from 1 at frame 0 to 2 at frame 4, linear: it thickens a quarter of a pixel each frame. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-115 frame 1: The line with its width keyed from 1 at frame 0 to 2 at frame 4, linear: it thickens a quarter of a pixel each frame. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-115 frame 2: The line with its width keyed from 1 at frame 0 to 2 at frame 4, linear: it thickens a quarter of a pixel each frame. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-115 frame 3: The line with its width keyed from 1 at frame 0 to 2 at frame 4, linear: it thickens a quarter of a pixel each frame. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-115 frame 4: The line with its width keyed from 1 at frame 0 to 2 at frame 4, linear: it thickens a quarter of a pixel each frame. | largest difference 3.6e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-116 frame 0: A filled box over the left three columns, its colour keyed from the fill's blue at frame 0 to the stroke's red at frame 4 and its opacity from 1 to 0.5, both linear. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-116 frame 1: A filled box over the left three columns, its colour keyed from the fill's blue at frame 0 to the stroke's red at frame 4 and its opacity from 1 to 0.5, both linear. | largest difference 1.8e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-116 frame 2: A filled box over the left three columns, its colour keyed from the fill's blue at frame 0 to the stroke's red at frame 4 and its opacity from 1 to 0.5, both linear. | largest difference 2.4e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-116 frame 3: A filled box over the left three columns, its colour keyed from the fill's blue at frame 0 to the stroke's red at frame 4 and its opacity from 1 to 0.5, both linear. | largest difference 1.1e-16; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-116 frame 4: A filled box over the left three columns, its colour keyed from the fill's blue at frame 0 to the stroke's red at frame 4 and its opacity from 1 to 0.5, both linear. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-117 frame 0: The line's stroke colour held on red until frame 2, then blue, and its opacity keyed from 1 at frame 0 to 0 at frame 4, linear. | largest difference 6.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-117 frame 1: The line's stroke colour held on red until frame 2, then blue, and its opacity keyed from 1 at frame 0 to 0 at frame 4, linear. | largest difference 1.2e-8; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-117 frame 2: The line's stroke colour held on red until frame 2, then blue, and its opacity keyed from 1 at frame 0 to 0 at frame 4, linear. | largest difference 3.0e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-117 frame 3: The line's stroke colour held on red until frame 2, then blue, and its opacity keyed from 1 at frame 0 to 0 at frame 4, linear. | largest difference 1.5e-9; tiles of 1 byte-identical; said nothing | yes |
| FX-SHP-117 frame 4: The line's stroke colour held on red until frame 2, then blue, and its opacity keyed from 1 at frame 0 to 0 at frame 4, linear. | largest difference 0.0e0; tiles of 1 byte-identical; said nothing | yes |

## FX-SHP-120 to 127, refused whole (D-170)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHP-120: A join that is not miter, round or bevel. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/stroke/join: expected one of miter, round, bevel. | yes |
| FX-SHP-121: A cap that is not butt, round or square. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/stroke/cap: expected one of butt, round, square. | yes |
| FX-SHP-122: A mitre limit below 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a mitre limit from 1 to 100, not 0.5. | yes |
| FX-SHP-123: A mitre limit above 100. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a mitre limit from 1 to 100, not 101. | yes |
| FX-SHP-124: A key of the stroke width at 0. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a stroke width above 0 and at most 8192, not 0. | yes |
| FX-SHP-125: A key of the fill opacity above 1. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0: expected a fill opacity from 0 to 1, not 1.5. | yes |
| FX-SHP-126: A key of the fill colour of two numbers. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/fill/color/keyframes/1: expected three numbers from 0 to 1 (D-78). | yes |
| FX-SHP-127: An expression on the stroke width. | ProjectSchemaInvalid: At /compositions/0/layers/0/shapes/0/stroke/width_px: expected no expression: a shape's colour, opacity and width take keys only (D-170). | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shp_104.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_115.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_116.json survives a save and a load, to the last bit | equal | yes |
| fx_shp_117.json survives a save and a load, to the last bit | equal | yes |
| a stroke put back to round, round and 4 is saved without the three words | left in the file: [] | yes |

## Result

43 of 43 checks pass.
