# B-780: RAM preview made by several workers, and at rest

Written by `cargo test --release -p anime_compositor_app b780_frames`. Frames go through `serve_logged`, the function every frame on screen comes from; "by hand" is a viewer that has made nothing ahead, asked for one frame at a time. Rows not marked with a worker count run `4` workers on the processor, the setting D-900 leaves off (the program runs one); the card keeps B-154's one. Rows after the second run with frames made at rest switched on, D-900's proposal; the program leaves it off.

A frame asked for by hand while the ten layers were made ahead at Full took 144 ms with one worker and 126 ms with 4, and 577 ms on a viewer with nothing made ahead (one run each, not a timing claim).

**30 of 30 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| workers making frames ahead on the processor in the program (D-900: several measured no faster) | 1 | 1 | PASS |
| frames made ahead at rest on the processor in the program (D-900, D-908: off, it slowed the next drag at Full on the processor) | off | off | PASS |
| reference shot, Draft: after play at frame 0 the other 239 frames are made ahead | 239 frames | 239 frames | PASS |
| reference shot, Draft: every frame is sent from memory and is byte for byte the frame made by hand | 240 of 240 | 240 of 240 | PASS |
| reference shot, Full: after play at frame 0 the other 239 frames are made ahead | 239 frames | 239 frames | PASS |
| reference shot, Full: every frame is sent from memory and is byte for byte the frame made by hand | 240 of 240 | 240 of 240 | PASS |
| document 08's ten layers, blurred and graded, Draft: the other 239 frames are made ahead | 239 frames | 239 frames | PASS |
| and every fourth frame is byte for byte the frame made by hand | 60 of 60 | 60 of 60 | PASS |
| the workers' own cels are given back when they finish | 0 bytes | 0 bytes | PASS |
| layer 1 hidden while the loop is being made ahead: every frame is then the edited frame made by hand, from memory | 240 of 240 | 240 of 240 | PASS |
| after undo, frames sent from memory that are not the original frame made by hand | 0 | 0 | PASS |
| a second composition, 48 frames long, is made and opened | another | another | PASS |
| another composition opened while the loop is being made ahead: its frames from memory that are not its own | 0 | 0 | PASS |
| and back on the first, its frames from memory that are not its own | 0 | 0 | PASS |
| 1 worker(s): at rest on frame 100, a quarter of a second later: frames held | 1 | 1 | PASS |
| 1 worker(s): stepping frames 101 to 110 a tenth of a second apart: frames held, only those asked for | 11 | 11 | PASS |
| 1 worker(s): once the hand is still, the rest of the loop is made | 229 frames | 229 frames | PASS |
| 1 worker(s): and every frame is sent from memory, byte for byte the frame made by hand | 240 of 240 | 240 of 240 | PASS |
| 1 worker(s): playing the ten layers at Full, a frame asked for by hand: frames made ahead in the 0.3 s after it | 0 | 0 | PASS |
| 1 worker(s): and once the hand is still the loop is finished | 240 held | 240 held | PASS |
| 4 worker(s): at rest on frame 100, a quarter of a second later: frames held | 1 | 1 | PASS |
| 4 worker(s): stepping frames 101 to 110 a tenth of a second apart: frames held, only those asked for | 11 | 11 | PASS |
| 4 worker(s): once the hand is still, the rest of the loop is made | 229 frames | 229 frames | PASS |
| 4 worker(s): and every frame is sent from memory, byte for byte the frame made by hand | 240 of 240 | 240 of 240 | PASS |
| 4 worker(s): playing the ten layers at Full, a frame asked for by hand: frames made ahead in the 0.3 s after it | 0 | 0 | PASS |
| 4 worker(s): and once the hand is still the loop is finished | 240 held | 240 held | PASS |
| frame 100 at Full, layer 3 set to 80% then 60% opacity, nothing made between: the 60% frame starts from the layers below as kept | 0 made, started from them | 0 made, started from them | PASS |
| the same with the loop made at rest between the two: it still starts from them | 239 made, started from them | 239 made, started from them | PASS |
| with room for 50 Draft frames, the workers stop when it is full | 50 frames, 25920000 bytes | 50 frames, 25920000 bytes | PASS |
| the workers' own cels while they run: a quarter of the memory setting, taken from the viewer's | the setting | the setting | PASS |
