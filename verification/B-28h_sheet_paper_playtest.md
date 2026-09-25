# B-28h: the paper sheet laid out as a studio's, by hand

Built on 2026-09-24 against D-84e, which you accepted the same day ("proceed"). It also covers
B-28g's steps, which print the same things.

The generated half is `verification/B-28g_sheet_print_table.md`. It reads FX-PRINT-001 to 008
from document 25 and checks each page the window writes: its header, its halves, its columns,
its second numbers and its end line. It also checks that every printed cell is the same as the
Sheet's. Five of those pages are kept beside it and open in any browser:
`B-28g_sheet_print_040.html`, `B-28g_sheet_print_300_frames.html`,
`B-28g_sheet_print_30_fps.html`, `B-28g_sheet_print_3_seconds.html` and
`B-28g_sheet_print_red.html`. `B-28g_sheet_print_reference_shot.html` is the project you printed
at the first playtest. This sheet covers what the table cannot: the new window, the print
dialog and the paper.

## Before you start

Import the sample cut as before: **Import cut...** and the folder `Fixtures/xdts/fx_xdts_040`.
Choose **Workspace: Timing**, so the Sheet is the tall column on the left.

## What to check

1. **The window.** Click **Print...** on the Sheet's title bar. A small window, **Print the
   Sheet**, opens over the app. It asks two things: **A page holds** (6 seconds, in two columns
   of 3) and **Printed in** (black). It has two buttons, **Print...** and **Leave it**.
2. **Leave it.** Click **Leave it**. The window closes and nothing prints. Open it again and
   press **Escape**. It closes the same way.
3. **The dialog.** Open it again and click its **Print...**. The system's print dialog opens and
   shows a preview of the Sheet, not of the whole window.
4. **To PDF.** Choose **Microsoft Print to PDF** as the printer and print. Save the file anywhere
   and open it.
5. **One page.** The PDF has one page, with no date or address in its corners. Across its top is
   a ruled title block: **CUT** s01 c012, **TIME (sec + fr)** 2 + 0, **RATE** 24 fps,
   **SHEET** Sheet 1 of 1, and an empty **MEMO** box.
6. **Counted from 1.** The left half runs from row **1** to row **72** and the right half from
   **73** to **144**. The Sheet on screen still starts at 0; on paper its frame 0 is row 1.
7. **The columns.** Each half's headings are **sec, frame, Action, Dialogue, A, B, C**, then
   **three empty columns**, then **Camera**. Action is empty, ready to write in by hand.
8. **The marks.** The left half matches the Sheet on screen, one row down: **MIKA Over here!**
   on row 1, **FOLLOW** on row 21, and the numbers, hold lines and crosses as on screen.
9. **The seconds.** In the **sec** column, **1**, **2** and **3** stand beside rows 24, 48 and
   72, and **4**, **5** and **6** beside rows 96, 120 and 144, each on the thick line that ends
   its second. A small **▶** sits in the same column at each half second: rows 12, 36, 60 and
   so on. The heavy double line is under row **48**, the end of the cut.
10. **Three seconds a page.** Press **Ctrl+P**. The window opens again and remembers black and 6
    seconds. Choose **3 seconds, in one column** and print. The preview has one page with one
    wide column of rows 1 to 72, the Action and Dialogue columns wider than before.
11. **Remembered.** Press Ctrl+P again. **A page holds** already says 3 seconds. Leave it.
12. **Red.** Press Ctrl+P, choose **6 seconds** and **red**, and print. The rules, the headings,
    the frame and second numbers and the title block's small labels are red. The numbers,
    lines, crosses and words in the cells, and **s01 c012**, **2 + 0**, **24 fps** and
    **Sheet 1 of 1**, are black. Set it back to **black** before you go on.
13. **A longer cut.** Open the composition settings (Ctrl+K), set the length to **300** and click
    OK. Print at 6 seconds: **3** pages, **Sheet 1 of 3** to **Sheet 3 of 3**, **12 + 12**, the
    heavy line under row 300 on page 3, and seconds numbered to **18** on page 3. Print again at
    3 seconds: **5** pages, **Sheet 5 of 5** ending at row 360. Ctrl+Z.
14. **Nothing to print.** Make a new composition (**New composition...**) and add only a solid
    (**New solid**, or Ctrl+Y). **Print...** is gone from the Sheet's title bar. Press Ctrl+P.
    No window opens, and the line at the bottom of the window reads **There is nothing to print:
    no layer here shows drawings.**

## Known limits

- The page size and orientation are the print dialog's own. A page always holds 6 or 3 seconds.
- The title block still has no episode, scene, cut or animator boxes. That is B-28i, next.
- The Action column is always empty on paper. Writing into it is B-28j, after B-28i.
- More than six drawing layers print more than six cel columns, and a page gets crowded.
- Long words in Action, Dialogue or Camera are cut off at the column's edge.

## What to answer

"works", or which step number did something else and what it did.
