# B-24d: a mask's path set moving

D-77 said it before anything was built: a path is interpolated point by point, the point and both of its handles, between keys that hold the same number of points, by document 20's curves. FX-MSK-031 to 035 are five cases of that, five frames each, drawn in `verification/B-24d keys/mask_key_cases.png` and printed in document 25. Every expected pixel is `Fixtures/masks/expected_masks.json`, written by `tools/mask_reference.py` before this code existed; the build's frame is compared sample by sample and the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6.

Until today a file with keys on a path opened, was kept and was drawn at its base with `PROJECT_FEATURE_UNSUPPORTED` said out loud. That fallback is gone: the diagnostic is retired and the path moves.

## The five moving cases, frame by frame (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| *FX-MSK-031: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame.* | | |
| FX-MSK-031 frame 0 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-031 frame 1 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-031 frame 2 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-031 frame 3 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-031 frame 4 | largest difference 3.1e-8; said nothing | yes |
| *FX-MSK-032: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once.* | | |
| FX-MSK-032 frame 0 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-032 frame 1 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-032 frame 2 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-032 frame 3 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-032 frame 4 | largest difference 3.1e-8; said nothing | yes |
| *FX-MSK-033: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way.* | | |
| FX-MSK-033 frame 0 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-033 frame 1 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-033 frame 2 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-033 frame 3 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-033 frame 4 | largest difference 3.1e-8; said nothing | yes |
| *FX-MSK-034: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does.* | | |
| FX-MSK-034 frame 0 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-034 frame 1 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-034 frame 2 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-034 frame 3 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-034 frame 4 | largest difference 3.1e-8; said nothing | yes |
| *FX-MSK-035: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame.* | | |
| FX-MSK-035 frame 0 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-035 frame 1 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-035 frame 2 | largest difference 3.1e-8; said nothing | yes |
| FX-MSK-035 frame 3 | largest difference 3.2e-8; said nothing | yes |
| FX-MSK-035 frame 4 | largest difference 3.1e-8; said nothing | yes |

## The shape at a frame, read out loud (document 20)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MSK-031's left edge over frames 0 to 4, linear between keys at 0 and 4 | [0.0, 0.75, 1.5, 2.25, 3.0] | yes |
| FX-MSK-032's first key holds, so the shape waits and then arrives whole | [0.0, 0.0, 0.0, 0.0, 3.0] | yes |
| FX-MSK-033's easy ease is late at a quarter, level at the half and early at three quarters | [0.0, 0.3874857931419583, 1.5, 2.612514206858042, 3.0] | yes |
| FX-MSK-034 keys at 1 and 3 only: before the first key and after the last, the shape holds | [0.0, 0.0, 1.5, 3.0, 3.0] | yes |
| FX-MSK-035 keys the handles, not the points, so the right edge bows out a step at a time | [0.0, 0.5, 1.0, 1.5, 2.0] | yes |
| a mask asked for its shape at a frame gives back a mask that stands still there | 0 key(s) | yes |
| and a path with no keys gives back its base at every frame | 0 key(s) | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_msk_033.json's keys survive a save and a load, to the last bit | 2 key(s), frames [0, 4] | yes |
| a written key is a frame, a whole outline and its interpolation, document 19's way | frame 0, 4 point(s), interp "ease", ease present | yes |
| and no file with keys on a path is diagnosed any more | [] | yes |

## Refusals (D-77)

| Check | The build's answer | Matches |
| --- | --- | --- |
| a key holding a different number of points is refused, with MASK_INVALID_OUTLINE | MaskInvalidOutline: D-77 interpolates a path point by point, so every key on it holds the same points as the path itself, one key to a frame. The masks are unchanged. | yes |
| two keys at one frame are refused the same way | MaskInvalidOutline: D-77 interpolates a path point by point, so every key on it holds the same points as the path itself, one key to a frame. The masks are unchanged. | yes |
| keys given out of order are accepted and kept in frame order | Ok("Set mask of 4 points"); frames [0, 4] | yes |
| and one undo gives back the mask as it was | the same project | yes |

## Result

39 of 39 checks pass.
