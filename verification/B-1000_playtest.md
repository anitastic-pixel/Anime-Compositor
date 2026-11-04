# B-1000 playtest: frames made ahead stay after an edit that draws nothing (D-1119)

What changed: the preview memory used to throw away every frame it had made ahead after any
edit at all, even one that changes no pixel. Now an edit to something no frame is drawn from
keeps them: a layer's colour label, its lock, its shy switch, its markers, and the
composition's markers. An edit that does change the picture still drops them, and so does a
rename (an expression can find a layer by its name). Keeping frames after a rename is the
follow-up D-1138.

## Before and after

Counts, not times, so a busy machine does not change them. AMD Ryzen 9 9900X, RTX 4070 Ti
SUPER, Windows 11 Education 10.0.26300; debug test build, `CARGO_TARGET_DIR=target/lanes/p27b`,
`CARGO_INCREMENTAL=0`; one run of `p27_speed::d1119_frames_made_ahead_survive_an_edit_that_draws_nothing`
on the reference shot, Draft, 240 frames made ahead. Before: B-999 (64256bfa). After: this commit.

| The edit, after the 240 frames were made | Frames still sent from memory, before | After | Of those, byte for byte the frame made from scratch |
|---|---|---|---|
| Colour label | 0 | **240** | 240 of 240 |
| Lock | 0 | **240** | 240 of 240 |
| Shy | 0 | **240** | 240 of 240 |
| A layer marker | 0 | **240** | 240 of 240 |
| A composition marker | 0 | **240** | 240 of 240 |
| Rename a layer | 0 | 0 | (none kept, as planned) |
| Hide a layer (changes the picture) | 0 | 0 | (none kept, as it must) |

## Try it

1. Open the reference shot and let the green ready line fill the whole time ruler.
2. Change the first layer's colour label, lock it, mark it shy, and add a marker.
   - Expect: the green line stays full each time, and playback stays smooth at once.
3. Hide the first layer (eye switch).
   - Expect: the green line empties and fills again, and the layer is gone from the picture.
4. Rename a layer.
   - Expect: the green line empties and fills again, as before this change.

Pass if every "Expect" holds. If a frame ever shows a layer you hid, or the old look after an
edit, say which edit.

Checked by: the test above, the app suite (including B-154's "never stale" and B-780's byte
checks), and the core's own tests.
