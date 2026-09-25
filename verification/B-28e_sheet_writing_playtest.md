# B-28e: writing into the Sheet, by hand

Built on 2026-09-24 against D-84b, writing into the Sheet, which you accepted the same day
("proceed").

The generated half is `verification/B-28e_sheet_writing_table.md`, 23 of 23. It plays every one
of document 25's sixteen FX-SHEET cases, read from document 25 itself. For each case it sets a
layer up as the Before line, writes the cell, and reads the Sheet back against the After line. It
also counts what went into the history and undoes it to check Before comes back. This sheet
covers what the table cannot: choosing a cell, typing, and the keys.

## Before you start

Import the sample cut as in B-28d's playtest: **Import cut...** and the folder
`Fixtures/xdts/fx_xdts_040`. Choose **Workspace: Timing**, so the Sheet is the tall column on the
left.

## What to check

1. **The hint.** The Sheet's title bar now says: click a cell, then type a number and Enter, x
   for a cross, or Delete to erase.
2. **Choosing a cell.** Click A's cell on frame 2 (it shows **2**). It gets a blue outline, and
   the playhead goes to frame 2, as clicking a row did before.
3. **A shaded cell cannot be chosen.** If no layer is trimmed there are no shaded cells. Skip
   this step, or trim A's start to frame 4 on the timeline first. Clicking a striped cell moves
   the playhead but gives no outline. Ctrl+Z the trim afterwards.
4. **Writing a number on a hold (FX-SHEET-001).** Click A's cell on frame 3 (a hold line under
   the 2). Type **5**. The cell shows 5 on a dark background while you type. Press **Enter**.
   - Frame 3 now reads 5, the hold line under it ends at the next number (the 3 on frame 4),
     and the picture on frame 3 changes to drawing 5.
   - The outline has moved down to frame 4.
   - Press **Ctrl+Z**: frame 3 is a hold of 2 again.
5. **Two digits, and taking one back.** On any cell of A, type **1**, **2**, then
   **Backspace**. The cell shows 1. Press **Escape**: the cell shows its old mark again and
   nothing was written.
6. **A drawing the cut does not have (FX-SHEET-015).** On A's frame 0, type **12** and Enter. The
   status line says drawing 12 is not in this sequence and those frames stay empty. The picture
   shows no A there. Ctrl+Z.
7. **A cross (FX-SHEET-003).** Click A's frame 15 (a hold of 8) and press **x**. Frame 15 shows ×
   and A is empty from there down to frame 24, where 7 is written. The picture on frames 15 to 23
   has no A. Ctrl+Z.
8. **Erasing (FX-SHEET-006).** Click A's frame 4 (the 3) and press **Delete**. The 3 is gone,
   and 2 now holds from frame 2 to frame 5, down to the 4 on frame 6. Ctrl+Z.
9. **Erasing a hold is refused (FX-SHEET-009).** Click A's frame 1 (a hold line) and press
   **Delete**. Nothing changes. The status line says nothing is written there to erase, it holds
   the drawing above it. Ctrl+Z has nothing new to undo: the last thing it undoes is whatever you
   did before.
10. **Erasing a cross.** Click B's frame 16 (the ×) and press **Delete**. The 2 on B's frame 15
    now runs on down to frame 29, and the picture shows B on those frames. Ctrl+Z.
11. **The arrows.** With a cell chosen, Up and Down move the outline a row and take the playhead
    with them. Left and Right move it between A, B and C. The outline stops at the top, the
    bottom, the first and last column, and at a striped cell.
12. **Space still plays, Ctrl+Z still undoes.** With a cell chosen, press **Space**: the shot
    plays. Press Space again to stop. Write something, then Ctrl+Z: it undoes.
13. **Nothing else took the keys.** With a cell chosen, press Delete: the Sheet erases the cell
    (or refuses), and **no layer is deleted**. Click a layer on the timeline and press Delete
    there: that still deletes the layer. Ctrl+Z it back.

## Known limits

- One cell at a time. Filling a range, writing on twos, copy and paste, and dragging a mark
  are not in this step.
- A number lasts until the next thing written below it, as on paper. To make a drawing shorter,
  write the next number or a cross where it should end.
- Writing does not reach outside a layer's in and out. Trim the layer on the timeline first.
- The Sheet has no dialogue or camera columns yet, and does not print. Those are the next two
  steps you asked for.

## What to answer

"works", or which step number did something else and what it did.
