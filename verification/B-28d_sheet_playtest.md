# B-28d: the Sheet view, by hand

Built on 2026-09-24 against D-84a, the Sheet view you approved the same day ("approved;
proceed").

The generated half is `verification/B-28d_sheet_table.md`, 57 of 57. It imports FX-XDTS-040 and
compares the sheet the page is given with document 25's table for that cut, frame by frame, all
48 frames. It also checks how A, B and C are written (numbers, hold lines and crosses), that a
trimmed layer is shaded outside its in and out, and that a solid gets no column. This sheet
covers what the table cannot judge: how the tab looks, whether it reads like a paper timesheet,
and whether clicking and playing move the right row.

## Before you start

Import the sample cut as in B-28c's playtest: **Import cut...** and the folder
`Fixtures/xdts/fx_xdts_040`. Keep document 25's FX-XDTS-040 table open beside the window. It
is the sheet written out by hand, and the Sheet tab should look like it.

## What to check

1. **The tab.** Above the timeline, beside **Timeline** and **Graph**, is a third tab,
   **Sheet**. Hovering it says what it shows.
2. **The layout.** Click **Sheet**. The layer bars are replaced by a table: a **frame** column on
   the left (0 to 47), then **A, B, C**, left to right. A is the bottom of the stack, so it is on
   the left, as on paper and in document 25.
3. **The marks.** Compare with document 25:
   - A number is written where a drawing starts. A line runs down the cells while it holds.
   - A reads 1, 2, 3 ... 8 on twos, holds 8 from frame 14 to 23, then steps back down to 1 at
     frame 36 and holds to the end.
   - B has a **×** on frame 16, then empty cells to frame 29, and 1 again on frame 30.
   - C has a **×** on frame 0 and is empty until 1 on frame 20. It has another × on frame 28 and
     1 again on frame 40.
4. **Seconds.** There is a heavier line under frame 23 and under frame 47, the end of each
   second at 24 frames a second.
5. **The current frame.** The row of the frame on screen is highlighted. Use the Left and Right
   arrows: the highlight moves a row at a time, and the picture matches the row. For example, on
   frame 20 the row reads 8 held, empty, 1, and the picture shows red and blue.
6. **Clicking a row.** Click the row for frame 16. The playhead goes there: the picture shows
   only red, and the Timeline tab's playhead is on 16 when you switch back.
7. **Playing.** Press Space on the Sheet tab. The highlight runs down the rows with the picture,
   and the table scrolls to keep it in view if the panel is short.
8. **A trimmed layer.** Go back to **Timeline**, drag the start of A's bar to frame 4 (or trim
   it any way you like), and return to **Sheet**. A's cells before its new start are shaded
   (striped), and its first cell in shows a number, not a hold line.
9. **A layer with no drawings.** Add a solid (**New solid**). The Sheet has no column for it.
   Ctrl+Z it away.
10. **Undo shows at once.** Ctrl+Z the trim from step 8. A's column goes back to what step 3
    described without leaving the tab.
11. **Another composition.** Open the composition you had before the import. The Sheet shows its
    drawing layers instead, or says "No layer here shows drawings" if it has none.
12. **Back to the others.** Click **Timeline** and **Graph**. Each still works as before, and
    Shift+F3 still swaps Timeline and Graph.

## Known limits

- The Sheet only shows timing. You cannot type into it yet; exposures are still changed in the
  inspector's exposure list or by dragging on the timeline. Typing into the sheet would be its
  own decision.
- There are no dialogue or camera columns. The import does not read them (they are in the
  notes), so there is nothing to show.
- It does not print.
- Only layers that show an image sequence get a column. A still, a solid, a shape, a null or a
  nested composition does not.

## What to answer

"works", or which step number did something else and what it did.
