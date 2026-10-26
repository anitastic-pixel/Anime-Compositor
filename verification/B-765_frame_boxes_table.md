# B-765: the frame brings its own boxes (D-885)

Written by `b765_the_frame_brings_its_own_boxes` in `app/src/tests/frame_boxes.rs`.

A still frame asked with `b=1` carries the `/boxes` answer after its pixels, from the same snapshot, so the page asks no second question per frame and an outline can never be another frame's. Each row asks the frame plainly, then with `b=1`, then `/boxes` on its own, on the reference shot's viewer, on the CPU.

**25 of 25 checks pass.**

| Viewer state | Quality asked | Frame | Quality said | Pixels with the boxes vs the plain frame | Boxes vs `/boxes` | Result |
|---|---|---|---|---|---|---|
| as opened | Draft | 0 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| as opened | Draft | 120 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| as opened | Full | 0 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| as opened | Full | 120 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| after an edit (layer-2 moved) | Draft | 0 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| after an edit (layer-2 moved) | Draft | 120 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| after an edit (layer-2 moved) | Full | 0 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| after an edit (layer-2 moved) | Full | 120 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| during a drag (layer-3, not let go) | Draft | 0 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| during a drag (layer-3, not let go) | Draft | 120 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| during a drag (layer-3, not let go) | Full | 0 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| during a drag (layer-3, not let go) | Full | 120 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| after the drag ends | Draft | 0 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| after the drag ends | Draft | 120 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| after the drag ends | Full | 0 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| after the drag ends | Full | 120 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| after an undo | Draft | 0 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| after an undo | Draft | 120 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| after an undo | Full | 0 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| after an undo | Full | 120 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| in another composition (a copy, one layer moved) | Draft | 0 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| in another composition (a copy, one layer moved) | Draft | 120 | Draft | 518400 bytes, same | 4 layers, same | PASS |
| in another composition (a copy, one layer moved) | Full | 0 | Full | 8294400 bytes, same | 4 layers, same | PASS |
| in another composition (a copy, one layer moved) | Full | 120 | Full | 8294400 bytes, same | 4 layers, same | PASS |

A frame asked without `b=1` carries no boxes: PASS.
