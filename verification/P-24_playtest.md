# P-24: the application itself, by hand

Measured on 2026-09-30 against the owner's request ("let's test for stability/performance for the app itself, like startup and dragging the window around"). P-24 measures and reports; nothing in the app was changed. Its two problems were fixed by B-174 (D-247), and steps 2, 6 and 8 describe the app after that fix. Never performed.

The measured half is `verification/P-24_app_stability_table.md`, written by `tools/app_stability.ps1`: start-up times, dragging and resizing with a real mouse, 50 open-and-close cycles, a 20-minute soak, and the problems found, each with a proposed fix. This sheet covers what a script cannot: whether it feels right in your hands.

## Before you start

Use the release build, with nothing heavy running. Close any copy of the app that is already open.

## What to check

1. **Start-up.** Start the app. The window appears at once, the interface draws within about a second, and everything has stopped changing within about two seconds. Close it and start it again: the second start should be no slower.
2. **The viewer at the opening size.** Before touching anything, look at the viewer. The very first time after B-174 the window opens maximized, and the picture shows at once. (Before B-174 the window opened too small and the viewer was squeezed to 30 by 17 points with no picture: problem 1 in the P-24 table, fixed by B-174, D-247.)
3. **Dragging, idle.** Make the window large enough to see the picture. Drag it around the screen by the title bar for a few seconds, in circles. The window follows the mouse without stutter, never turns white or grey, and Windows never says "Not responding". Let go: it stays where you dropped it, and the picture is the same as before.
4. **Dragging while playing.** Press space to play. Drag the window around again for a few seconds. The picture keeps playing while you drag, and when you let go the frame number is where the clock says it should be (it has not paused and fallen behind). Do the same with a heavy project of your own if you have one.
5. **Resizing.** Stop playback. Drag the window's bottom-right corner out, then in, then back to about where it was. The interface follows the corner, the viewer picture resizes with it, and nothing is left blank. Then do the same while playing: playback keeps going.
6. **Maximize, minimize, restore.** Click maximize, then restore; then minimize, and click the app on the taskbar to bring it back. Each time the picture comes back within a moment, at the frame it was on. Watch the zoom figure under the viewer (for example "28%, fit"): at the same window size it reads the same after restoring as before, and the picture sits at the same size. (Before B-174 it could differ by a point or two: problem 2 in the P-24 table, fixed by B-174.)
7. **Closing.** Close the window with the X while playing. It closes within a moment, and nothing named anime_compositor_app or msedgewebview2 is left in Task Manager a few seconds later.
8. **The window remembered (B-174).** Restore the window, drag it somewhere and give it a size of your choosing, and close it. Start the app again: it opens at that place and size. Maximize it, close it, start again: it opens maximized, and Restore takes it back to the place and size you chose. If you have a second display, leave the window on it, close the app, unplug the display and start again: the window opens maximized on the main display, never off-screen. The place is kept in `window.txt` in `%APPDATA%\dev.anitastic.anime-compositor\`; deleting that file makes the next start a first launch again.

## What would be a failure

The window opening with no picture in the viewer, or off-screen; the window turning white or "Not responding" at any point; the picture freezing while dragging and not catching up after; the app taking clearly longer to start than the table says; a crash; or anything left running after it closes.
