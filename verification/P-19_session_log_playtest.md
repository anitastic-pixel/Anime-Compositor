# P-19: the session log, by hand

Built on 2026-09-25 against the P-19 entry in document 15, which you accepted the same day ("proceed"). Never performed.

The generated halves are `verification/P-19_session_log_table.md` (10 of 10) and `verification/P-19_session_log_window_table.md` (20 of 20). This sheet covers what they cannot: the panel as you see it, the Save dialog, and closing and reopening the window. `verification/P-19_session_log.md` explains what was built.

## Before you start

Use the release build. It opens on the reference shot.

## What to check

1. **Off at the start.** Click **Session log…** in the menu bar. A panel opens at the top right. The box "Record where each frame's time goes" is not ticked, and the panel says **Off. Nothing is being recorded.** The table of slowest frames is empty.
2. **The command search.** Close the panel with **Close**. Press Ctrl+Shift+P, type "session" and choose **Session log**. The same panel opens.
3. **On, then play.** Tick the box. The panel says **Recording. No frame drawn yet.** Press space to play the reference shot for a few seconds, then stop. Within a second the panel shows how many frames were logged, a median, a slowest-5% time, the frames the clock dropped, and the stage with the most time in all. The table lists up to 20 frames, slowest first. Each one says Draft, playing, where its time went, and "reading ahead".
4. **The panel does not get in the way.** While the panel is open, play and scrub. The viewer keeps working, and the count of frames logged keeps rising.
5. **Draft to Full.** Switch the preview to **Full resolution** and play again. The new rows say Full, and their times are higher than the Draft ones.
6. **Stepped frames.** Stop, then step with the arrow keys. The stepped frames are logged too, and say "stepped", not "playing".
7. **Save a copy.** Click **Save a copy…**. The Save dialog opens, suggesting "session log.md". Save it somewhere you can find it. With the next frame, the status line at the bottom says where it was saved. Open the file in any text editor, or a Markdown viewer. It starts with the date, the operating system, the number of processor cores and the build ("release"), then a summary line, then one table row per frame logged.
8. **Clear.** Click **Clear**. The panel's count goes to 0 and the table empties. The box stays ticked.
9. **Off again.** Untick the box and play. The count stays at 0.
10. **Gone on close.** Tick the box, play a few seconds so there are rows, then close the whole app. Open it again and open **Session log…**. The box is not ticked and the log is empty.

## What to report

Anything that reads wrong, looks wrong or is in the way, and the number of the step. A time that seems too high or too low is worth reporting too, with the saved copy attached.
