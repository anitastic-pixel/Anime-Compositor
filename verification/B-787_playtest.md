# B-787 playtest: frames made ahead while you rest, on the graphics card only (D-908, PROPOSED, switched off)

What you asked for: when the graphics card draws the viewer and you stop on a frame for half a second, the app quietly makes the frames around it, so Play starts smoothly at once; the processor keeps waiting for Play.

What the build does: it is **built, but switched off**. Switched on, one kind of drag on the card got slower: dragging a mask point and holding still halfway (about 12 ms to 36 ms behind the hand at Full). Two fixes brought every other drag back to its old speed, but not that one, so by your rule it stays off until it is solved or you choose otherwise (D-908). The app's own checks turn it on and prove the frames it makes are exactly the frames the viewer draws.

Also fixed and on: a test of the memory (B-780) that sometimes failed on a busy machine. One cause was a real, rare race where a frame could still be made ahead just after you moved; the other was that the half second of rest was counted from when a slow frame was asked rather than when it was finished.

You need: the app and the 25-layer reference shot or any shot of your own a few seconds long.

## 1. Nothing is made while you rest (as before)
1. Set the drawing switch in the viewer's row to **GPU**, then to **CPU**, and each time click a frame in the middle of the timeline and rest five seconds.
2. Click the next frame on the ruler.
   - **Expected:** the green line along the ruler covers only the frames you have looked at, exactly as before this change. The loop is made when you press Play.

## 2. Dragging is as before
1. On the card and on the processor, drag a layer, scale it, drag a mask point (stop halfway for a moment), and drag a number in Effect Controls.
   - **Expected:** the picture follows the hand as it did before this change.

## 3. Playing after an edit
1. Press Play, then hide a layer (its eye), and play again; undo and play again; switch quality Draft and back; switch GPU and CPU; open another composition and come back.
   - **Expected:** every frame shows the current state; never an old picture.

Proof: `verification/B-787_card_at_rest_table.md` (the app's own checks, all passing, with the card's switch turned on for its rows) and `verification/B-780_ram_workers_table.md` (30 of 30). Drag and first-play timings are taken by main in a quiet slot and go in `verification/B-787_card_at_rest_timing_table.md`.
