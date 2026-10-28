# B-810 playtest: Minimax

What changed: a new effect, **Minimax** (Effect > Lines & Mattes), after After Effects' Minimax.
It grows or shrinks shapes and lines by whole pixels: with **Maximum** each channel takes the
largest value nearby, so bright or solid parts spread; with **Minimum** the smallest, so they
shrink. This is what the tutorial MN518F3vOvE does at 6:25 with "F's Max" to thicken its lines.

1. Open a project with a drawing of lines on a clear background, select its layer and add
   Effect > Lines & Mattes > Minimax.
   - Expect: Operation **Maximum**, Radius **0**, Channel **Color**, Direction **Horizontal &
     Vertical**, Don't Shrink Edges **Off**, and the picture unchanged.
2. Set Channel to **Alpha and Color** and Radius to **3**.
   - Expect: every line is 3 pixels thicker on every side, keeping its colour; corners grow
     square, not round.
3. Set Operation to **Minimum**.
   - Expect: the lines get thinner instead; lines 6 pixels wide or less vanish.
4. Set Operation to **Minimum then Maximum** (Radius 3).
   - Expect: thin lines and specks vanish, thick shapes come back to about their own size.
5. Set Direction to **Just Horizontal**, then **Just Vertical**.
   - Expect: the change happens only sideways, then only up and down.
6. Put the drawing touching the layer's edge, Channel **Alpha**, Operation **Minimum**, and turn
   **Don't Shrink Edges** on and off.
   - Expect: off, the shape is eaten in from the layer's edge too; on, the edge side stays.
7. Switch the viewer to Draft (half size).
   - Expect: the result looks the same size relative to the drawing.
8. Save, close and reopen the project.
   - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b810_minimax` (65 of 65, `B-810_minimax_table.md`): every
pixel of FX-MINIMAX-001 to 020 against `tools/minimax_reference.py`.

Tutorial sheet: `verification/B-810 pictures/minimax_vs_tutorial.png`, MN518F3vOvE at 6:32 and 6:40 (F's Max, value 3, RGB+Alpha, filling the gaps the keyed-out lines leave in the magenta shadow) beside ours rendered by the release app (Maximum, radius 3, Alpha and Color): the gaps close and the edge grows by 3 pixels in square steps, as in the tutorial.
