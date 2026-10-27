# B-787: RAM preview made at rest on the card only

Written by `cargo test --release -p anime_compositor_app b787_frames`. The program is built with both at-rest switches off (D-908 PROPOSED); the rows below switch the card's on, as proposed: frames are made ahead at rest, once the hand has been still for half a second, only while the graphics card draws the viewer. Frames go through `serve_logged`, the function every frame on screen comes from; "by hand" is a viewer that has made nothing ahead, asked for one frame at a time. Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

**20 of 20 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| frames made ahead at rest while the card draws the viewer, in the program (D-908 PROPOSED) | off | off | PASS |
| and while the processor draws it (D-900: the next drag at Full was slower) | off | off | PASS |
| processor (no card), at rest on frame 100 past the half second: frames made ahead | 0 frames | 0 frames | PASS |
| processor (the card switched off), at rest on frame 100 past the half second: frames made ahead | 0 frames | 0 frames | PASS |
| a frame asked by hand on the processor starts the frames made ahead (as before B-780, it does not) | no | no | PASS |
| a frame asked by hand on the card starts them, to be made once the hand is still | yes | yes | PASS |
| card, Draft: at rest on frame 100, a quarter of a second later: frames held | 1 | 1 | PASS |
| card, Draft: once the hand is still the rest of the loop is made | 239 frames | 239 frames | PASS |
| card, Draft: frames sent from memory, byte for byte the card's frame made by hand | 240 of 240 | 240 of 240 | PASS |
| card: the Draft loop made, then each eighth frame asked at Full: Draft frames sent | 0 | 0 | PASS |
| card switched off after the loop was made: each eighth frame on the processor, card frames sent | 0 | 0 | PASS |
| card, Full: at rest on frame 100, a quarter of a second later: frames held | 1 | 1 | PASS |
| card, Full: once the hand is still the rest of the loop is made | 239 frames | 239 frames | PASS |
| card, Full: frames sent from memory, byte for byte the card's frame made by hand | 30 of 30 | 30 of 30 | PASS |
| card: the hand moves from frame 100 to 20 while the loop is made: frames sent from memory that are not the frame made by hand | 0 | 0 | PASS |
| and every frame is then held | 240 held | 240 held | PASS |
| card: layer 1 hidden while the loop is made at rest: frames from memory, the edited frame made by hand | 240 of 240 | 240 of 240 | PASS |
| card: after undo, frames sent from memory that are not the original frame made by hand | 0 | 0 | PASS |
| card: another composition opened while the loop is made at rest: its frames from memory that are not its own | 0 | 0 | PASS |
| and back on the first, its frames from memory that are not its own | 0 | 0 | PASS |
