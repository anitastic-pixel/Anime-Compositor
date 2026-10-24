# B-655: A mask's feather blurs at sigma 0.375 x feather (D-775)

Eight tutorials from eight creators fit After Effects' mask edge; seven put its sigma at 0.345 to 0.398 x feather (about 0.375), and ours, at feather / 2, read 1.27 to 1.43 times as wide (median 1.335; Graphician 1.94, an outlier on the same side). A mask's covering is now blurred at sigma 0.375 x feather. The edge is read off a white solid's 100 by 100 rectangle mask: how far out its covering falls to 0.1587, one sigma on a Gaussian's edge. The fixtures' numbers are the reference's. Display Mix's FX-MIX8-009 and 010 (8 bpc and Display) are checked by b642_display_mix.

## The mask's edge: sigma, in pixels, within 3 per cent

| Check | The build's answer | Matches |
| --- | --- | --- |
| Feather 4: sigma 1.5 (it was 2 before D-775) | 1.48 pixels | yes |
| Feather 8: sigma 3 (it was 4 before D-775) | 3.02 pixels | yes |
| Feather 20: sigma 7.5 (it was 10 before D-775) | 7.48 pixels | yes |
| Feather 40: sigma 15 (it was 20 before D-775) | 14.95 pixels | yes |
| Feather 0: a hard edge, as before (nothing covered above it) | 0.00 pixels | yes |

## The fixtures, at the reference's numbers (tolerance 1e-6 and 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MSK-001 frame 0: Today's rectangle written the new way: the left three columns, and every number the same as before D-77. | largest difference 3.1e-8 | yes |
| FX-MSK-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-002 frame 0: The same file written the old way, with one `mask` key: read as one Add mask, the same frame as FX-MSK-001. | largest difference 3.1e-8 | yes |
| FX-MSK-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-003 frame 0: A sloped edge: coverage in whole sixteenths. | largest difference 1.9e-8 | yes |
| FX-MSK-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-004 frame 0: Inverted: the three columns the mask keeps are the ones it now cuts. | largest difference 3.1e-8 | yes |
| FX-MSK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-005 frame 0: At half opacity: the kept columns are half there. | largest difference 1.5e-8 | yes |
| FX-MSK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-006 frame 0: Two masks, the second Add: both halves, so the whole frame. | largest difference 3.1e-8 | yes |
| FX-MSK-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-007 frame 0: Two masks, the second Subtract: the left three columns with its middle column taken out. | largest difference 3.1e-8 | yes |
| FX-MSK-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-008 frame 0: Two masks, the second Intersect: only where both are, column 2. | largest difference 3.1e-8 | yes |
| FX-MSK-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-009 frame 0: Two masks, the second Difference: where one is but not both. | largest difference 3.1e-8 | yes |
| FX-MSK-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-010 frame 0: A second mask in mode None takes no part: the frame of FX-MSK-001. | largest difference 3.1e-8 | yes |
| FX-MSK-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-011 frame 0: A first mask in Subtract takes from the whole layer: a hole in columns 2 and 3. | largest difference 3.1e-8 | yes |
| FX-MSK-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-012 frame 0: Expanded a quarter of a pixel: the rectangle grows on every side, its corners rounded. | largest difference 3.1e-8 | yes |
| FX-MSK-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-013 frame 0: Shrunk a quarter of a pixel: the same rectangle the other way. | largest difference 1.5e-8 | yes |
| FX-MSK-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-014 frame 0: Feathered two pixels: the hard edge at column 3 becomes a soft band. | largest difference 5.7e-8 | yes |
| FX-MSK-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-015 frame 0: A circle of four points with handles, filling the frame's height. | largest difference 2.6e-8 | yes |
| FX-MSK-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-016 frame 0: The same circle, expanded half a pixel. | largest difference 3.1e-8 | yes |
| FX-MSK-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-017 frame 0: A mask switched off takes no part: the whole drawing. | largest difference 3.1e-8 | yes |
| FX-MSK-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-031 frame 0: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame. | largest difference 3.1e-8 | yes |
| FX-MSK-031 frame 1: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame. | largest difference 3.1e-8 | yes |
| FX-MSK-031 frame 2: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame. | largest difference 3.1e-8 | yes |
| FX-MSK-031 frame 3: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame. | largest difference 3.1e-8 | yes |
| FX-MSK-031 frame 4: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame. | largest difference 3.1e-8 | yes |
| FX-MSK-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-032 frame 0: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once. | largest difference 3.1e-8 | yes |
| FX-MSK-032 frame 1: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once. | largest difference 3.1e-8 | yes |
| FX-MSK-032 frame 2: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once. | largest difference 3.1e-8 | yes |
| FX-MSK-032 frame 3: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once. | largest difference 3.1e-8 | yes |
| FX-MSK-032 frame 4: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once. | largest difference 3.1e-8 | yes |
| FX-MSK-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-033 frame 0: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way. | largest difference 3.1e-8 | yes |
| FX-MSK-033 frame 1: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way. | largest difference 3.1e-8 | yes |
| FX-MSK-033 frame 2: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way. | largest difference 3.1e-8 | yes |
| FX-MSK-033 frame 3: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way. | largest difference 3.1e-8 | yes |
| FX-MSK-033 frame 4: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way. | largest difference 3.1e-8 | yes |
| FX-MSK-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-034 frame 0: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does. | largest difference 3.1e-8 | yes |
| FX-MSK-034 frame 1: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does. | largest difference 3.1e-8 | yes |
| FX-MSK-034 frame 2: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does. | largest difference 3.1e-8 | yes |
| FX-MSK-034 frame 3: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does. | largest difference 3.1e-8 | yes |
| FX-MSK-034 frame 4: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does. | largest difference 3.1e-8 | yes |
| FX-MSK-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSK-035 frame 0: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame. | largest difference 3.1e-8 | yes |
| FX-MSK-035 frame 1: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame. | largest difference 3.1e-8 | yes |
| FX-MSK-035 frame 2: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame. | largest difference 3.1e-8 | yes |
| FX-MSK-035 frame 3: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame. | largest difference 3.2e-8 | yes |
| FX-MSK-035 frame 4: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame. | largest difference 3.1e-8 | yes |
| FX-MSK-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGYX-022 frame 0: Alpha Mode masks, the small box feathered 2: a soft cut. | largest difference 8.8e-8 | yes |
| FX-ENERGYX-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_msk_014.json (feather 2, its file unchanged since before D-775) opens with no warning | [] | yes |
| and saves back holding what it held: feather still 2, nothing added | the same | yes |
| and draws a tighter edge (sigma 0.75): column 1's covering 0.7275, not the 0.5662 it drew at sigma 1, with no warning | 0.7275; warnings [] | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| Feather 10, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| Feather 40, expansion 6, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| Feather 25, inverted, opacity 0.6, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| fx_msk_014.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Result

78 of 78 checks pass.
