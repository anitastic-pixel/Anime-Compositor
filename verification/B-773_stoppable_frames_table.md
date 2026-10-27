# B-773: stoppable frames (D-893)

Written by `b773_a_stopped_frame_is_never_shown_or_kept` in `app/src/tests/stoppable_frames.rs`.

Each row stops a viewer frame part way, as a `/stop` from the page arriving then would: first at the first point it looks, then asked again and stopped at the second, and so on (between the top composition's layers, before each tile, after the draw), until it comes back whole. Every stopped answer must be `PREVIEW_FRAME_STOPPED`'s: status 204, `x-stopped: 1`, no picture, no `x-sent`, and the page's picture left as it was. The answer that comes back whole must be made afresh (`x-cached: 0`, so nothing of the stopped frame was kept as a frame) and the page's picture must be byte for byte the frame a second viewer, given the same changes and never stopped, makes. On the reference shot's viewer, on the CPU.

**11 of 11 stopped frames were never shown or kept, and the frame after each is byte for byte the uninterrupted one.**

| Viewer state | Quality | Frame | The stopped answer | The next answer | Pixels against the never-stopped viewer | Result |
|---|---|---|---|---|---|---|
| first frame | Full | 0 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| after an edit (layer-2 moved) | Full | 0 | stopped at each of 9 points, no picture, nothing sent | made afresh | same | PASS |
| during a drag (layer-3 to 710,200) | Full | 0 | stopped at each of 9 points, no picture, nothing sent | made afresh | same | PASS |
| during a drag (layer-3 to 760,200) | Full | 0 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| before the undo (layer-2 to 600,300) | Full | 0 | stopped at each of 9 points, no picture, nothing sent | made afresh | same | PASS |
| after an undo (layer-2 back to 640,360) | Full | 0 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| a frame change | Full | 60 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| a quality change (Draft) | Draft | 60 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| Draft, a frame change | Draft | 61 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| a quality change (back to Full) | Full | 61 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |
| a switch of composition | Full | 61 | stopped at each of 8 points, no picture, nothing sent | made afresh | same | PASS |

A frame that is not the viewer's (an export or a stored check asks this way) is never stopped, even right after a stop: PASS.

## When the page stops a frame

The page's own `stopsInFlight`, run under Node. A drag's own frames are never stopped, or the viewer would draw nothing until the hand let go.

| Frame in flight, and what is waiting | Expected | The page's rule | Result |
|---|---|---|---|
| the sharp Full picture, and the drag has started again (B-162 would throw it away) | stop | stop | PASS |
| an In motion frame of a drag (D-898), and the drag goes on | let it finish | let it finish | PASS |
| the Draft picture chosen, and the drag has started again (D-898: as Full above) | stop | stop | PASS |
| a Full frame with Refine off, and the drag goes on | let it finish | let it finish | PASS |
| any frame, and the hand is still (an undo, a click, a typed value) | stop | stop | PASS |
| the same, but the last answer was a stopped one | let it finish | let it finish | PASS |
| a frame, and nothing is waiting | let it finish | let it finish | PASS |
| playback | let it finish | let it finish | PASS |

The page sends `/stop` and handles `x-stopped` (asking again when nothing newer waits): PASS.
