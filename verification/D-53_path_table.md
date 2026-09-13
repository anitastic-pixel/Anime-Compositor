# D-53: the motion path, against an evaluator that is not this one

Written by `d53_path_segments_match_the_reference_evaluator` in `tests/d53_path.rs`, which runs on every build. **44 of 44 checks pass.**

D-53 asked for the curve to be named and a fixture to pin it to exact values *before any code was written*. The curve is a cubic Bezier through the position keys in composition pixels, one handle either side of each key, stored as offsets from the key - the handles After Effects draws on the canvas. The fraction along it is the fraction of the way through the segment *after* the ease, so the ease says when and the path says where; FX-PATH-003 is the case with both on at once, and its frame 12 lands on the same point as FX-PATH-002's because an ease never moves the path.

Every number in the **expected** column is transcribed from document 25, which got it from `tools/path_reference.py`. That file evaluates the curve by de Casteljau - repeated linear interpolation - in Python, written from document 20. This build expands the polynomial, in Rust. The two were written from the specification and not from each other, which is what makes an agreement between them worth having. The largest disagreement anywhere in this table is **5.684e-14**.

What each case is for:

- **FX-PATH-001** - the handles that make a straight line make a straight line.
- **FX-PATH-001 (absent)** - the same two keys with no `spatial` at all, which document 19 says is that same line.
- **FX-PATH-002** - one curved segment under linear timing; its midpoint is (180, 60) where the straight line would be at (120, 120).
- **FX-PATH-003** - the same curved segment with easy ease on it: the ease says when, the path says where, and frame 12 is still (180, 60).
- **FX-PATH-004** - a handle longer than its own segment, which is legal in space: the layer sets off backwards and overshoots its destination.

| Case | Frame | Expected | This build | Difference | Tolerance | Result |
|---|---|---|---|---|---|---|
| FX-PATH-001 | 0 | (0, 0) | (0, 0) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 | 1 | (9.999999999999998, 4.999999999999999) | (10, 5) | 1.776e-15 | 1e-9 | pass |
| FX-PATH-001 | 6 | (60, 30) | (60, 30) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 | 12 | (120, 60) | (120, 60) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 | 18 | (180, 90) | (180, 90) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 | 23 | (230.00000000000006, 115.00000000000003) | (230.00000000000003, 115.00000000000001) | 2.842e-14 | 1e-9 | pass |
| FX-PATH-001 | 24 | (240, 120) | (240, 120) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 (absent) | 0 | (0, 0) | (0, 0) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 (absent) | 1 | (9.999999999999998, 4.999999999999999) | (10, 5) | 1.776e-15 | 1e-9 | pass |
| FX-PATH-001 (absent) | 6 | (60, 30) | (60, 30) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 (absent) | 12 | (120, 60) | (120, 60) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 (absent) | 18 | (180, 90) | (180, 90) | 0.000e0 | 1e-9 | pass |
| FX-PATH-001 (absent) | 23 | (230.00000000000006, 115.00000000000003) | (230, 115) | 5.684e-14 | 1e-9 | pass |
| FX-PATH-001 (absent) | 24 | (240, 120) | (240, 120) | 0.000e0 | 1e-9 | pass |
| FX-PATH-002 | 0 | (0, 0) | (0, 0) | 0.000e0 | 1e-6 | pass |
| FX-PATH-002 | 6 | (105, 15) | (105, 15) | 0.000e0 | 1e-6 | pass |
| FX-PATH-002 | 12 | (180, 60) | (180, 60) | 0.000e0 | 1e-6 | pass |
| FX-PATH-002 | 18 | (225, 135) | (225, 135) | 0.000e0 | 1e-6 | pass |
| FX-PATH-002 | 24 | (240, 240) | (240, 240) | 0.000e0 | 1e-6 | pass |
| FX-PATH-003 | 0 | (0, 0) | (0, 0) | 0.000e0 | 1e-6 | pass |
| FX-PATH-003 | 6 | (69.140625, 5.859375) | (69.140625, 5.859375) | 0.000e0 | 1e-6 | pass |
| FX-PATH-003 | 12 | (180, 60) | (180, 60) | 0.000e0 | 1e-6 | pass |
| FX-PATH-003 | 18 | (234.140625, 170.859375) | (234.140625, 170.859375) | 0.000e0 | 1e-6 | pass |
| FX-PATH-003 | 24 | (240, 240) | (240, 240) | 0.000e0 | 1e-6 | pass |
| FX-PATH-004 | 0 | (0, 0) | (0, 0) | 0.000e0 | 1e-6 | pass |
| FX-PATH-004 | 5 | (-10.9375, 50.625) | (-10.9375, 50.625) | 0.000e0 | 1e-6 | pass |
| FX-PATH-004 | 10 | (100, 67.5) | (100, 67.5) | 0.000e0 | 1e-6 | pass |
| FX-PATH-004 | 15 | (210.9375, 50.625) | (210.9375, 50.625) | 0.000e0 | 1e-6 | pass |
| FX-PATH-004 | 20 | (200, 0) | (200, 0) | 0.000e0 | 1e-6 | pass |
| FX-PATH-002 held | all | the left key's value, whatever the handles say | the left key's value | 0 | exact | pass |

## How to read a failure here

A row that fails is a disagreement between two evaluators, and either one may be the one that moved. Running `python tools/path_reference.py` prints the Python side's table; document 25 holds what it printed when the fixtures were set. If the Python and the document still agree and this page does not, the build moved.

## What this page does not show

The window. No handle can yet be taken hold of on the canvas; a path reaches a project through its file or through a command, and this page is about the numbers a handle would produce once it can be dragged.

Speed. D-53 chose the fraction of the segment over arc length and recorded the cost: on a curved segment with uneven handles the layer's speed varies a little even under linear timing. No row here measures speed, and FX-PATH-004's spacing is that cost visible in the numbers rather than hidden.
