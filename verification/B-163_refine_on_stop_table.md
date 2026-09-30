# B-163: Draft while the hand moves, the Full picture once it stops

At Full, with Refine when idle ticked (it is, unless unticked), a frame asked for while the playhead is dragged or a value is scrubbed is made at Draft, and 0.2 s after the hand stops the same frame is made at Full and replaces it. The scrub rows run the page's own decision lines on a pretend clock (a Draft frame 40 ms, a Full one 300 ms); the window rows ask the real request handler for the frames that scrub sent.

Produced by `cargo test -p anime_compositor_app latest_request_and_refine`, from `app/src/main.rs`.

| Check | Expected | Actual | Result |
|---|---|---|---|
| at Full with Refine when idle, every frame made while the hand moves is Draft | true | true | pass |
| and one more is asked for once it stops: frame 30 at Full | /frame/30?q=full | /frame/30?q=full | pass |
| no sooner than 0.2 s after the last move | true | true | pass |
| and no later than 0.2 s after it, or after the frame then being made | true | true | pass |
| the last picture drawn is that Full one | /frame/30?q=full | /frame/30?q=full | pass |
| the picture keeps up: the longest it stands still during the scrub is 40 pretend ms, against 300 without Refine when idle | true | true | pass |
| at Draft nothing changes: every frame is asked for at Draft | true | true | pass |
| without Refine when idle nothing changes: every frame is asked for at Full | true | true | pass |
| a stand-in is Draft and says so | Draft true | Draft true | pass |
| the picture that replaces it is Full, the same as export | Full false | Full false | pass |
| at 1920 by 1080 | 1920 1080 | 1920 1080 | pass |
| and its pixels are byte for byte frame 30 at Full asked for alone, which is today's Full | true | true | pass |
| and the Draft stand-in of frame 30 is byte for byte frame 30 at Draft asked for alone | true | true | pass |
| Refine when idle is on unless it has been unticked | true | true | pass |
| its tick is in the viewer's row beside the Full/Draft button | true | true | pass |
| and does nothing at Draft | true | true | pass |
| a stand-in leaves the Full/Draft choice as it was | true | true | pass |
| the Full/Draft button still names what pressing it does | true | true | pass |
| "sharpening…" shows while a stand-in is on screen | true | true | pass |
| the sharp picture is asked for once the hand has been still for SETTLE_MS | true | true | pass |
| layer outlines are asked for at the quality of the picture they are drawn over | true | true | pass |

**21 of 21 checks pass.**

## What this does not cover

How it feels in the window, which is the playtest sheet, `verification/B-162_B-163_playtest.md`. For scale, this run's window took 17.1 ms for the Draft stand-in of frame 30 and 27.8 ms for its Full picture, on the CPU, in a test build sharing the machine with other builds: a note, not a measurement.
