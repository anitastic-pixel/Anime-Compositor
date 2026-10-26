# B-770: frames answered off the window's thread (D-890)

Written by `b770_frames_off_the_window_thread_stay_ordered_and_current` in `app/src/tests/async_frames.rs`.

Each row asks a frame on the frame thread (`off_the_window_thread`, the one the window now uses) with `have=` the picture the page holds, after the command named ran on the test's own thread, draws the answer as the page does, and compares it with the whole frame asked plainly, byte for byte. Reference shot, CPU.

**12 of 12 pictures are the current whole frame, byte for byte.**

| Viewer state | Quality | Frame | Sent as | Result |
|---|---|---|---|---|
| first ask | Full | 0 | whole | PASS |
| after an edit (layer-2 moved) | Full | 0 | changed 1330,389,301,302 | PASS |
| during a drag (layer-3 to 710,200) | Full | 0 | changed 0,180,1920,800 | PASS |
| during a drag (layer-3 to 730,200) | Full | 0 | changed 710,380,180,240 | PASS |
| during a drag (layer-3 to 760,200) | Full | 0 | changed 730,380,190,240 | PASS |
| after the drag ends | Full | 0 | changed 0,0,0,0 | PASS |
| another frame | Full | 60 | changed 760,380,1120,700 | PASS |
| switched to Draft | Draft | 60 | whole | PASS |
| switched back to Full | Full | 60 | whole | PASS |
| after an undo | Full | 60 | changed 800,180,1080,900 | PASS |
| in another composition | Full | 60 | changed 789,689,503,391 | PASS |
| read queued before the alpha view was switched on, answered after it (newer, not staler) | Full | 60 | changed 0,0,1920,1080 | PASS |

64 jobs queued one after another ran in the order queued: PASS.
