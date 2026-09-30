# B-162: only the newest frame is made, and a stale one is not drawn

When the playhead is dragged fast, the viewer should not spend its time finishing frames the hand has already left. The page sends one request at a time and keeps only the newest one waiting (P-04 built that); this unit adds that a sharp picture of a place the hand has left is not drawn when it arrives late, and checks the whole bargain. The scrub rows run the page's own decision lines on a pretend clock (a Draft frame 40 ms, a Full one 300 ms); the window rows ask the real request handler.

Produced by `cargo test -p anime_compositor_app latest_request_and_refine`, from `app/src/main.rs`.

| Check | Expected | Actual | Result |
|---|---|---|---|
| a fast scrub at Full with Refine when idle: 31 frames are asked for and fewer are made | true | true | pass |
| at Full with Refine when idle: every frame made was the newest one asked for when it began | 0 older frames made | 0 older frames made | pass |
| at Full with Refine when idle: the last picture drawn is frame 30 | /frame/30 | /frame/30 | pass |
| a fast scrub at Full without it: 31 frames are asked for and fewer are made | true | true | pass |
| at Full without it: every frame made was the newest one asked for when it began | 0 older frames made | 0 older frames made | pass |
| at Full without it: the last picture drawn is frame 30 | /frame/30 | /frame/30 | pass |
| a fast scrub at Draft: 31 frames are asked for and fewer are made | true | true | pass |
| at Draft: every frame made was the newest one asked for when it began | 0 older frames made | 0 older frames made | pass |
| at Draft: the last picture drawn is frame 30 | /frame/30 | /frame/30 | pass |
| without Refine when idle every answer is drawn, which is how the viewer has always been | /frame/0?q=full /frame/18?q=full /frame/30?q=full | /frame/0?q=full /frame/18?q=full /frame/30?q=full | pass |
| the hand moves again while the sharp picture of frame 10 is being made: it is not drawn | /frame/10?q=full | /frame/10?q=full | pass |
| and the picture ends on frame 20 at Full | /frame/20?q=full | /frame/20?q=full | pass |
| the window, asked for the frames the scrub sent, ends on frame 30 | 30 | 30 | pass |
| and its pixels are byte for byte the ones frame 30 at Full makes when asked for alone | true | true | pass |
| the page asks for a frame at the quality `askedAs` gives | true | true | pass |
| and draws an answer only when `drawnWhenBack` says so | true | true | pass |
| one request out and the newest waiting (P-04, unchanged) | true | true | pass |

**17 of 17 checks pass.**

## What this does not cover

Stopping a frame half way. The window answers the viewer's requests one after another on its own thread, so word that a newer frame is wanted cannot reach a render until it has finished, and the renderer has no point at which to stop. The frame in flight is always finished; at most one more waits behind it. Nor does it cover how it feels in the window: that is the playtest sheet, `verification/B-162_B-163_playtest.md`.
