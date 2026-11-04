# B-998 playtest: an arrow-key nudge of several layers redraws the page once (D-1117, first part)

What changed: pressing an arrow key with several layers selected moves them all in one history
step, as before. Before, the page read the whole project back and redrew every panel after each
layer's move and once more at the end: four full redraws for three layers. Now it sends the moves
and reads and redraws once, after the press is committed.

This is the smallest part of D-1117. Redrawing only the panels an edit touches, and building only
the timeline rows on screen, are split off as follow-up D-1136; they are what would make a single
edit at 200 layers fast.

## Before and after

Very busy machine (other lanes' builds and tests running; a rename took 3.8 to 4.9 s here against
1.6 s an hour earlier): these timings are not final. AMD Ryzen 9 9900X, RTX 4070 Ti SUPER,
Windows 11 Education 10.0.26300; release build, `CARGO_TARGET_DIR=target/lanes/p27b`,
`CARGO_INCREMENTAL=0`, `-j 4`; `tools/p21_page_cost.py` (new row `nudge_3`), background copy with
its own web view profile, 7 runs each, medians. Before: the build at d625eaf0 (B-997). After:
this commit's build. Run back to back.

| One arrow press, three layers selected (`nudge_3`) | 4 layers | 200 layers |
|---|---|---|
| Before | 85.4 ms | 23,668 ms |
| After | **22.0 ms** | **5,086 ms** |

A single edit (`edit_total`) and one full redraw (`draw_panels`) are unchanged by this part;
their numbers moved only with how busy the machine was.

## Try it

1. Open a project with a few layers. Select three of them in the timeline.
2. Press the right arrow key five times, then Shift+left arrow once.
   - Expect: all three move together on screen, 1 pixel a press and 10 with Shift, and the
     Effect Controls and timeline show their new positions.
3. Press Ctrl+Z once.
   - Expect: only the last press (the Shift one) is undone, for all three layers at once, as
     before.
4. With a big project (100 or more layers), do step 2 again.
   - Expect: each press finishes noticeably sooner than before.

Pass if every "Expect" holds. If any layer is left behind or a press needs two undos, say which
step.

Checked by the app suite (every existing page-text pin unchanged); the nudge's own history
behaviour (one step per press) is the window's `property.drag_end`, which this change does not
touch.
