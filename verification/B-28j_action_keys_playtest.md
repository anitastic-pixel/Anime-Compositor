# B-28j: the Action column and key drawings, by hand

Built on 2026-09-24 against D-84g, which you accepted the same day ("proceed").

The generated halves are `verification/B-28e_sheet_writing_table.md`, which plays FX-SHEET-017
to 024 from document 25, and `verification/B-28g_sheet_print_table.md`, which reads FX-PRINT-011
off the printed page. That page is kept beside it as `B-28g_sheet_print_keys.html`, which opens
in any browser. This sheet covers what the tables cannot: typing, the keys you press, and how
the circles look.

## Before you start

Import the sample cut as before: **Import cut...** and the folder `Fixtures/xdts/fx_xdts_040`.
Choose **Workspace: Timing**, so the Sheet is the tall column on the left.

## What to check

1. **The column.** The Sheet now has an **Action** column, left of **Dialogue**. It is empty
   on every frame.
2. **Writing.** Click the Action cell on frame 20. Type **jumps** and press **Enter**. The cell
   says jumps, and the chosen cell moves down to frame 21.
3. **Replacing.** Click the Action cell on frame 20 again, type **lands** and press **Enter**.
   It now says lands.
4. **Only spaces.** Click the Action cell on frame 21, press **Space** a few times and press
   **Enter**. Nothing is written there. (A space only counts once a letter is typed.)
5. **Moving about.** With an Action cell chosen, the up and down arrows move along the column,
   and the right arrow moves to A's cell on the same frame. **Escape** drops what you had typed
   but not yet entered.
6. **Erasing.** Click the Action cell on frame 20 and press **Delete**. It is empty again.
   Press **Ctrl+Z**: lands comes back. Press **Ctrl+Z** again: jumps comes back.
7. **A key.** Click the A cell on frame 4, where **3** is written, and press **K**. The 3 gets
   a thin oval round it, there and on frame 32, the other place A's 3 is written. The lines
   holding it in between are not circled.
8. **Not a key.** Click an A cell showing a line (the one under a number) and press **K**.
   Nothing is circled, and the window says to press K on the number itself.
9. **Unmarking.** Click the A cell on frame 32 and press **K**. Both ovals go.
   Press **Ctrl+Z**: both come back.
10. **On paper.** Press **Ctrl+P** and print. In the preview, Action has **jumps** on row 21
    and nothing else; A's 3 is circled on rows 5 and 33 and no other number is. Cancel.
    Choose **red** and look again: the ovals are black, like the numbers. Set it back to
    **black**.
11. **Saved.** Save the project (**Ctrl+S**), close the window and open it again. jumps is
    still on frame 20 and A's 3 is still circled.

## Known limits

- An Action note sits on one frame. There is no line down the frames after it, as a dialogue
  line has; a note that lasts is written again where it changes.
- K marks a drawing number of one layer. The same number on another layer is its own drawing
  and is not circled with it.
- The circle is drawn round a written number only; a key held by a line is circled where its
  number is written.

## What to answer

"works", or which step number did something else and what it did.
