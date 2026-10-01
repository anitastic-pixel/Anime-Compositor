# P-24: the application itself, by hand

Measured on 2026-09-30 against the owner's request ("let's test for stability/performance for the app itself, like startup and dragging the window around"). P-24 measures and reports; nothing in the app was changed. Never performed.

The measured half is `verification/P-24_app_stability_table.md`, written by `tools/app_stability.ps1`: start-up times, dragging and resizing with a real mouse, 50 open-and-close cycles, a 20-minute soak, and the problems found, each with a proposed fix. This sheet covers what a script cannot: whether it feels right in your hands.

## Before you start

Use the release build, with nothing heavy running. Close any copy of the app that is already open.

## What to check

1. **Start-up.** Start the app. The window appears at once, the interface draws within about a second, and everything has stopped changing within about two seconds. Close it and start it again: the second start should be no slower.
2. **The viewer at the opening size.** Before touching anything, look at the viewer. At the size the window opens, the composition panel shows only its controls and no picture (the viewer is squeezed to 30 by 17 points). This is problem 1 in the table. Make the window bigger (or maximize it) and the picture appears.
3. **Dragging, idle.** Make the window large enough to see the picture. Drag it around the screen by the title bar for a few seconds, in circles. The window follows the mouse without stutter, never turns white or grey, and Windows never says "Not responding". Let go: it stays where you dropped it, and the picture is the same as before.
4. **Dragging while playing.** Press space to play. Drag the window around again for a few seconds. The picture keeps playing while you drag, and when you let go the frame number is where the clock says it should be (it has not paused and fallen behind). Do the same with a heavy project of your own if you have one.
5. **Resizing.** Stop playback. Drag the window's bottom-right corner out, then in, then back to about where it was. The interface follows the corner, the viewer picture resizes with it, and nothing is left blank. Then do the same while playing: playback keeps going.
6. **Maximize, minimize, restore.** Click maximize, then restore; then minimize, and click the app on the taskbar to bring it back. Each time the picture comes back within a moment, at the frame it was on. Watch the zoom figure under the viewer (for example "28%, fit"): it may differ by a point or two after restoring, at the same window size, and the picture may sit a little smaller or larger than before. That is problem 2 in the table, cosmetic.
7. **Closing.** Close the window with the X while playing. It closes within a moment, and nothing named anime_compositor_app or msedgewebview2 is left in Task Manager a few seconds later.

## What would be a failure

The window turning white or "Not responding" at any point; the picture freezing while dragging and not catching up after; the app taking clearly longer to start than the table says; a crash; or anything left running after it closes.
