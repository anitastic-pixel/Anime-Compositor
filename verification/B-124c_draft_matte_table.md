# B-124c: a matted layer at Draft

Written by `tests/b124c_draft_matte.rs`. B-124's playtest sheet reported that a layer with a matte disappears at Draft: the Draft step shrank each layer's placement to the smaller frame, and not its matte's. It now shrinks both. Each shot is 960 by 540: a dark ground, and an orange layer the size of the frame alpha-matted by a white 200 by 200 square, which must be exactly an orange 200 by 200 square placed where the matte is. Full is what an export writes and was never touched. Tolerance 1e-6.

## The matted layer is the square it is matted by

| Check | The build's answer | Matches |
| --- | --- | --- |
| the square straight, its corner at (300, 148), at Full: 960 by 540, and the same as the orange square | 960 by 540, largest difference 0.0e0 | yes |
| the square straight, its corner at (300, 148), at Draft: 240 by 135, and the same as the orange square | 240 by 135, largest difference 0.0e0 | yes |
| the square turned 30 degrees in the middle, at Full: 960 by 540, and the same as the orange square | 960 by 540, largest difference 6.0e-8 | yes |
| the square turned 30 degrees in the middle, at Draft: 240 by 135, and the same as the orange square | 240 by 135, largest difference 6.0e-8 | yes |
| the square at 150% turned 45 degrees, near a corner, at Full: 960 by 540, and the same as the orange square | 960 by 540, largest difference 6.0e-8 | yes |
| the square at 150% turned 45 degrees, near a corner, at Draft: 240 by 135, and the same as the orange square | 240 by 135, largest difference 6.0e-8 | yes |
| the square turned 30 degrees, inside a composition layer, at Full: 960 by 540, and the same as the orange square | 960 by 540, largest difference 6.0e-8 | yes |
| the square turned 30 degrees, inside a composition layer, at Draft: 240 by 135, and the same as the orange square | 240 by 135, largest difference 6.0e-8 | yes |

## Draft is Full a quarter of the size each way

| Check | The build's answer | Matches |
| --- | --- | --- |
| the straight square: 40,000 orange pixels at Full, and 40,000 / 16 = 2,500 at Draft | 40000 at Full, 2500 at Draft | yes |
| the same Draft frame drawn as it was before the fix has no orange pixel: the fault | 0 | yes |

## The picture

| Check | The build's answer | Matches |
| --- | --- | --- |
| `verification/B-124c pictures/before_after.png`: the turned square at Full; at Draft before the fix, empty; at Draft now, the same square; both Draft frames drawn 4 times bigger | written | yes |

## Result

11 of 11 checks pass.
