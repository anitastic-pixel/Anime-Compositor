# B-163: a rough picture while the hand moves, the chosen one once it stops

With Refine when idle ticked (it is, unless unticked), a frame asked for while the playhead is dragged or a value is scrubbed is made In motion (D-898; Draft before it), and 0.2 s after the hand stops the same frame is made at the quality chosen and replaces it. The scrub rows run the page's own decision lines on a pretend clock (an In motion frame 20 ms, Draft 40 ms, Half 120 ms, Full 300 ms); the window rows ask the real request handler for the frames that scrub sent.

Produced by `cargo test -p anime_compositor_app latest_request_and_refine`, from `app/src/main.rs`.

| Check | Expected | Actual | Result |
|---|---|---|---|
| at Full with Refine when idle, every frame made while the hand moves is In motion (D-898) | true | true | pass |
| and one more is asked for once it stops: frame 30 at Full | /frame/30?q=full | /frame/30?q=full | pass |
| no sooner than 0.2 s after the last move | true | true | pass |
| and no later than 0.2 s after it, or after the frame then being made | true | true | pass |
| the last picture drawn is that Full one | /frame/30?q=full | /frame/30?q=full | pass |
| the picture keeps up: the longest it stands still during the scrub is 20 pretend ms, against 300 without Refine when idle | true | true | pass |
| at draft too (D-898): In motion while the hand moves, then frame 30 at draft once it stops | true /frame/30?q=draft | true /frame/30?q=draft | pass |
| at half too (D-898): In motion while the hand moves, then frame 30 at half once it stops | true /frame/30?q=half | true /frame/30?q=half | pass |
| at Draft without Refine when idle nothing changes: every frame is asked for at Draft | true | true | pass |
| without Refine when idle nothing changes: every frame is asked for at Full | true | true | pass |
| a stand-in is In motion and says so | In motion true | In motion true | pass |
| the picture that replaces it is Full, the same as export | Full false | Full false | pass |
| at 1920 by 1080 | 1920 1080 | 1920 1080 | pass |
| and its pixels are byte for byte frame 30 at Full asked for alone, which is today's Full | true | true | pass |
| and the In motion stand-in of frame 30 is byte for byte frame 30 In motion asked for alone | true | true | pass |
| Refine when idle is on unless it has been unticked | true | true | pass |
| its tick is in the viewer's row beside the Full/Draft button | true | true | pass |
| and works at every quality (D-898) | true | true | pass |
| a stand-in leaves the Full/Draft choice as it was | true | true | pass |
| the Full/Draft button still names what pressing it does | true | true | pass |
| "sharpening…" shows while a stand-in is on screen | true | true | pass |
| the sharp picture is asked for once the hand has been still for SETTLE_MS | true | true | pass |
| layer outlines are asked for at the quality of the picture they are drawn over | true | true | pass |

**23 of 23 checks pass.**

## What this does not cover

How it feels in the window, which is the playtest sheet, `verification/B-162_B-163_playtest.md`. How long this run's window took is printed by the test, not written here, since it differs every run.
