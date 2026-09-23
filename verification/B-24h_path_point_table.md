# B-24h: a point added to a path that moves

D-79 lifts the limit B-24d wrote down: a point may now be added to or taken off a path that has keys, because the point is added to the path and to every key on it at once, which keeps the agreement D-77 rests on -- every key holds as many points as the path itself.

The promise a person is being asked to believe is that **adding a point moves nothing**. That is checked here against the curve rather than against a picture: document 19's cubic is written out again in this file and read at two hundred places along every segment, before and after, with de Casteljau's cut undone by parameter. A cut segment is two segments afterwards, so the first holds the curve from 0 to `t` and the second the rest. A segment with no handles is a straight line and is checked as a line instead, because document 19's cubic walks such a segment unevenly and cutting it changes when the line is walked without changing where it goes.

The drawn pixels are checked too, and they are the one honest exception: `flatten` cuts a curve into pieces about two pixels long, and two short curves are not cut at the same places as one long one. A straight segment is cut exactly and is checked for **no pixel at all**; a curved one is measured against a ceiling of one of a pixel's sixteen samples, written into `tests/b24h_path_points.rs`.

The last table is the other limit B-24d wrote down: every mask edit there has ever been read "Set mask of N points" in the history, because the core takes the whole list of masks as one command. It now reads what changed, which is worked out from the list as it was rather than from anything the window says about itself.

## A curve cut: the curve does not move (D-79)

| Check | The build's answer | As asked |
| --- | --- | --- |
| a blob of curves, cut at 50% of segment 0 | 5 points, 2.14e-14 px apart at the worst | yes |
| a blob of curves, cut at 25% of segment 1 | 5 points, 3.55e-14 px apart at the worst | yes |
| a blob of curves, cut at 10% of segment 2 | 5 points, 2.25e-14 px apart at the worst | yes |
| a blob of curves, cut at 80% of segment 3 -- the segment that closes the path | 5 points, 1.12e-14 px apart at the worst | yes |

## A straight segment cut: a corner, on the line

| Check | The build's answer | As asked |
| --- | --- | --- |
| the new point sits on the line it was put on | (50.0, 19.856), 0.00e0 px off the line | yes |
| and it is a corner: no handles on it, and none grown on its neighbours | (0.0, 0.0) and (0.0, 0.0) on the new point, (0.0, 0.0) and (0.0, 0.0) beside it | yes |

## The pixels drawn

| Check | The build's answer | As asked |
| --- | --- | --- |
| a straight segment cut: not one pixel of the drawn mask differs | 0 pixels differ, worst 0.000000 | yes |
| a curved segment cut, four places: the edge moves by flattening and no more than that | at the worst of the four, 5 of 3072 pixels differ, by 0.062500 of full coverage -- 1 of the sixteen samples in a pixel (ceiling 0.0625) | yes |

## Every key keeps its own shape (D-79)

| Check | The build's answer | As asked |
| --- | --- | --- |
| the base and both keys hold the same number of points, as D-77 asks | base 5, keys [5, 5] | yes |
| the key at frame 0: its own curve is where it was | 2.16e-14 px apart at the worst | yes |
| the key at frame 12: its own curve is where it was | 2.56e-14 px apart at the worst | yes |
| and the shape between the keys does not move either, frames -2 to 15 | 2.93e-14 px apart at the worst of all of them | yes |

## A point taken off (D-79)

| Check | The build's answer | As asked |
| --- | --- | --- |
| the point goes off the base and off every key, and the rest are untouched | base 3 points, keys [3, 3] | yes |

## What the core makes of it

| Check | The build's answer | As asked |
| --- | --- | --- |
| a keyed path with a point added is accepted, where one key alone is refused | accepted, and the history entry reads "Add a point to Mask 1" | yes |
| the D-77 gate still holds: a point added to one key alone is refused | MaskInvalidOutline | yes |
| a point taken off a keyed path is accepted too | accepted, and the history entry reads "Remove a point from Mask 1" | yes |
| a path taken down to two points is refused, by the rule this build already had | Document 19: a polygon mask is a closed ordered list of vertices. Fewer than three enclose no area, so there is nothing for the mask to keep. | yes |
| and undo gives back the project as it was before any of it | the same project | yes |
| saved and opened again, the added point is on the base and on both keys, unremarked | base 5 points, keys [5, 5], and the file is read back with [MediaMissing] | yes |

## What the history entry says (B-24h)

| Check | The build's answer | As asked |
| --- | --- | --- |
| drew a second mask | "Draw Mask 2" | yes |
| pulled one point | "Move a point of Mask 1" | yes |
| pulled the whole path | "Move Mask 1's path" | yes |
| set a feather | "Set Mask 1's feather to 6 px" | yes |
| set an expansion | "Set Mask 1's expansion to -3.5 px" | yes |
| set an opacity | "Set Mask 1's opacity to 40%" | yes |
| changed the mode | "Set Mask 1 to subtract" | yes |
| inverted it | "Invert Mask 1" | yes |
| switched it off | "Switch Mask 1 off" | yes |
| renamed it | "Rename Mask 1 to Face" | yes |
| pressed the path's stopwatch | "Start Face's path moving" | yes |
| put a second key down | "Key Face's path at frame 6" | yes |
| pulled a point at a key | "Move a point of Face at frame 6" | yes |
| dragged that key along its row | "Move Face's path key from frame 6 to 9" | yes |
| eased it with F9 | "Change the ease on Face's path key at frame 0" | yes |
| added a point to the moving path | "Add a point to Face" | yes |
| took it off again | "Remove a point from Face" | yes |
| took the stopwatch off | "Stop Face's path moving" | yes |
| deleted the second mask | "Delete Mask 2" | yes |

## Result

38 of 38 checks pass.
