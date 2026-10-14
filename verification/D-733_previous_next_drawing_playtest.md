# D-733 playtest: previous and next drawing

What it is: OpenToonz's (and Harmony's, TVPaint's) previous / next drawing. The playhead jumps to the frame where the selected layer's drawing changes, skipping the frames where the same drawing is held.

## Steps

1. Open the reference shot (or any project with drawing layers on twos or threes).
2. Click a layer that is on threes (layer 4 in the reference shot) and put the playhead on frame 10.
3. Press **Shift+.** (Shift and the full stop).
   - The playhead goes to frame 12, where the next drawing starts. Press it again: 15, then 18.
4. Press **Shift+,** (Shift and the comma).
   - The playhead goes back one drawing at a time: 15, 12, 9.
5. Click a layer that is on twos (layer 3) and press **Shift+.** a few times.
   - It steps two frames at a time.
6. Click a layer with only one drawing (layer 1) and press **Shift+.**
   - The playhead does not move, and the status line says "No drawing changes after this frame on ...".
7. Open the Sheet: where a layer has a blank (a cross in its column), **Shift+.** stops on the cross, and the next press goes to the drawing after the blank.
8. The comma and full stop on their own still zoom the picture out and in, as before.
9. Animation menu: **Previous drawing** and **Next drawing** are under Next key, showing Shift+, and Shift+. Ctrl+Shift+P, type "drawing": both lines are there.

## Pass if

Every jump lands where the Sheet shows a new drawing number or a blank starting, and nothing in the project changes (Undo has nothing new in it).
