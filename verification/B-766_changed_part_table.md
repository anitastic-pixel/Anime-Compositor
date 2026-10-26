# B-766: only the part that changed is sent (D-886)

Written by `b766_the_changed_part_makes_the_whole_frame` in `app/src/tests/changed_part.rs`.

Each row asks a frame as the page does, with `have=` the picture it holds, draws the answer onto the picture it holds as the page does, then compares that picture with the whole frame asked plainly, byte for byte. On the reference shot's viewer, on the CPU.

**21 of 21 pictures are the whole frame, byte for byte; 17 came as only the part that changed.**

| Viewer state | Quality | Frame | Sent as | Bytes sent | Result |
|---|---|---|---|---|---|
| first ask, nothing held | Full | 0 | whole | 8294400 of 8294400 | PASS |
| same frame again | Full | 0 | changed 0,0,0,0 | 0 of 8294400 | PASS |
| after an edit (layer-2 moved) | Full | 0 | changed 1330,389,301,302 | 363608 of 8294400 | PASS |
| during a drag (layer-3 to 710,200) | Full | 0 | changed 0,180,1920,800 | 6144000 of 8294400 | PASS |
| during a drag (layer-3 to 730,200) | Full | 0 | changed 710,380,180,240 | 172800 of 8294400 | PASS |
| during a drag (layer-3 to 760,200) | Full | 0 | changed 730,380,190,240 | 182400 of 8294400 | PASS |
| after the drag ends | Full | 0 | changed 0,0,0,0 | 0 of 8294400 | PASS |
| another frame | Full | 1 | changed 0,0,0,0 | 0 of 8294400 | PASS |
| another frame | Full | 2 | changed 760,380,320,240 | 307200 of 8294400 | PASS |
| another frame | Full | 60 | changed 920,380,960,700 | 2688000 of 8294400 | PASS |
| another frame | Full | 120 | changed 642,220,1238,860 | 4258720 of 8294400 | PASS |
| switched to Draft | Draft | 120 | whole | 518400 of 518400 | PASS |
| Draft, next frame | Draft | 121 | changed 0,0,0,0 | 0 of 518400 | PASS |
| switched back to Full | Full | 121 | whole | 8294400 of 8294400 | PASS |
| after an undo | Full | 121 | changed 0,180,1920,800 | 6144000 of 8294400 | PASS |
| alpha view on | Full | 121 | changed 0,0,1920,1080 | 8294400 of 8294400 | PASS |
| alpha view, next frame | Full | 122 | changed 0,0,0,0 | 0 of 8294400 | PASS |
| alpha view off | Full | 122 | changed 0,0,1920,1080 | 8294400 of 8294400 | PASS |
| after an answer the page threw away | Full | 6 | whole | 8294400 of 8294400 | PASS |
| in another composition (a copy, one layer moved) | Full | 6 | changed 1322,989,277,91 | 100828 of 8294400 | PASS |
| that composition, next frame | Full | 7 | changed 1183,980,416,100 | 166400 of 8294400 | PASS |

A frame asked without `have=` is sent whole, with no `x-sent` or `x-changed`: PASS.
