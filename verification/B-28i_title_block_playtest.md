# B-28i: the title block, by hand

Built on 2026-09-24 against D-84f, which you accepted the same day ("proceed").

The generated half is `verification/B-28g_sheet_print_table.md`. It reads FX-PRINT-001 to 010
from document 25. For FX-PRINT-001 and 009 it checks every box of the title block by its label
and what it says. It undoes FX-PRINT-009, and it saves and reopens FX-PRINT-010. The page it
printed for FX-PRINT-009 is kept beside it as `B-28g_sheet_print_title_block.html`, which opens
in any browser. This sheet covers what the table cannot: the boxes in Composition Settings, the
print dialog and the paper.

## Before you start

Import the sample cut as before: **Import cut...** and the folder `Fixtures/xdts/fx_xdts_040`.
Choose **Workspace: Timing**, so the Sheet is the tall column on the left.

## What to check

1. **Empty at first.** Press **Ctrl+P** and click **Print...** in the Print the Sheet window.
   The preview's title block has two rows. On top: **NAME** s01 c012, then empty **EPISODE**,
   **SCENE**, **CUT** and **ANIMATOR** boxes. Below: **TIME (sec + fr)** 2 + 0, **RATE**
   24 fps, **SHEET** Sheet 1 of 1, and a wide empty **MEMO** box. Cancel the print dialog.
2. **The four boxes.** Press **Ctrl+K**. The **Project** panel comes to the front of the left
   column, in front of the Sheet, with Composition Settings open in it. (Before this build it
   stayed hidden behind the Sheet.) Under the length there is a heading, **Sheet**, with four
   empty boxes: **Episode**, **Scene**, **Cut** and **Animator**.
3. **Written.** Type **3**, **1**, **012** and **K. Sato** into them and click **Apply**.
4. **On paper.** Press **Ctrl+P** and print. The top row now reads **s01 c012**, **3**, **1**,
   **012** and **K. Sato**, each under its label. Cancel.
5. **Kept.** Press **Ctrl+K** again. The four boxes still say 3, 1, 012 and K. Sato. Click
   **Leave it**.
6. **One Undo.** Press **Ctrl+Z** once. Press **Ctrl+K**: the four boxes are empty, and the
   name, size, rate and length are as they were. Click **Leave it**. Press **Ctrl+Shift+Z** to
   bring the four back.
7. **Saved.** Save the project (**Ctrl+S**, anywhere you like), close the window and open the
   project again. Press **Ctrl+K**: the four still say 3, 1, 012 and K. Sato. Print: the same
   title block as step 4.
8. **Only in settings.** Click **New composition...**. Its form has no Sheet heading and no
   four boxes. Click **Leave it**.
9. **Red.** Press **Ctrl+P**, choose **red** and print. The title block's lines and small labels
   are red; **s01 c012**, **3**, **1**, **012**, **K. Sato** and the rest of what is written in it
   are black. Set it back to **black**.

## Known limits

- A very long name or animator is cut off at its box's edge.
- MEMO is always empty on paper, ready to write in by hand.
- The Action column is always empty on paper. Writing into it is B-28j, next.

## What to answer

"works", or which step number did something else and what it did.
