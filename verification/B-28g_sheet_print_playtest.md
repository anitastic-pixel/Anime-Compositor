# B-28g: printing the Sheet, by hand

Built on 2026-09-24 against D-84d, printing the Sheet, which you accepted the same day
("proceed").

The generated half is `verification/B-28g_sheet_print_table.md`. It reads FX-PRINT-001 to 004
from document 25 and checks each page the window writes: its header, its halves, its columns
and its end line. It also checks that every printed cell is the same as the Sheet's. Three of
those pages are kept beside it and open in any browser: `B-28g_sheet_print_040.html`,
`B-28g_sheet_print_300_frames.html` and `B-28g_sheet_print_30_fps.html`. A fourth,
`B-28g_sheet_print_reference_shot.html`, is the project you printed at the first playtest, kept
to look at. This sheet covers what the table cannot: the print dialog and the paper.

## Before you start

Import the sample cut as before: **Import cut...** and the folder `Fixtures/xdts/fx_xdts_040`.
Choose **Workspace: Timing**, so the Sheet is the tall column on the left.

## What to check

1. **The button.** The Sheet's title bar shows **Print...** on its right.
2. **The dialog.** Click **Print...**. The system's print dialog opens and shows a preview of the
   Sheet, not of the whole window.
3. **To PDF.** Choose **Microsoft Print to PDF** as the printer and print. Save the file anywhere
   and open it.
4. **One page.** The PDF has one page, with no date or address in its corners. Across its top is
   a ruled title block of boxes, each with a small label: **CUT** s01 c012, **TIME (sec + fr)**
   2 + 0, **RATE** 24 fps, **SHEET** Sheet 1 of 1, and an empty **MEMO** box to write in.
5. **Two halves.** The left half runs from frame 0 to frame 71 and the right half from 72 to 143.
   Each half's headings are **frame, Dialogue, A, B, C, Camera**.
6. **The marks.** The left half matches the Sheet on screen, row for row: the numbers, the hold
   lines, the crosses, **MIKA Over here!** at frame 0 and **FOLLOW** at frame 20.
7. **The lines.** Every row is the same height. A thick black line marks the end of each second,
   under 23, 47, 71 and so on, and those frame numbers are bold. A thin black line and a small
   **▶** mark each half second, under 11, 35, 59 and so on. A heavy double line runs under frame
   47, the end of the cut; the frame numbers after it are pale and their rows are empty.
8. **Ctrl+P.** Close the dialog, click the viewer and press **Ctrl+P**. The same dialog opens on
   the Sheet.
9. **A longer cut.** Open the composition settings (Ctrl+K), set the length to **300** and click
   OK. Print again. The preview has **3** pages, headed **Sheet 1 of 3** to **Sheet 3 of 3** and
   **12 + 12**. On page 3 the heavy line is under frame 299. Ctrl+Z.
10. **Nothing to print.** Make a new composition (**New composition...**) and add only a solid
    (**New solid**, or Ctrl+Y).
    **Print...** is gone from the Sheet's title bar. Press Ctrl+P. No dialog opens, and the line
    at the bottom of the window reads **There is nothing to print: no layer here shows
    drawings.**

## Known limits

- The page size and orientation are the dialog's own settings. The window does not choose
  them, and six seconds always go on one page.
- There is no title block for the production, scene or animator.
- Only the composition on screen prints.
- The Sheet cannot yet be written back out as an XDTS file.

## What to answer

"works", or which step number did something else and what it did.
