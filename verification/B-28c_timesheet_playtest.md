# B-28c: a cut imported from its timesheet, by hand

Built on 2026-09-24 against D-84, which the owner accepted the same day ("cool, proceed").

The generated half is `verification/B-28c_panel_table.md`, 15 of 15. It sends what the window
sends and reads back what the page is given: FX-XDTS-040 arrives as the composition "s01 c012",
160 by 90, 48 frames at 24 a second, on screen, with layers A, B and C bottom to top, each
carrying its column; B is blank from frame 16 to 29; the picture has A and C on frame 20 and A
and B on frame 0; the four notes are the ones document 25 lists; one Undo takes it all away; and
a folder with two timesheets imports nothing and says why. Whether every cut is read right is
B-28b's table, 60 of 60. This sheet covers what neither can judge: the button, the dialog, what
you see playing, and whether the notes and the inspector line read well.

## Before you start

Open any project, or start a new one. Nothing below changes the fixtures. The sample cut is the
folder `Fixtures/xdts/fx_xdts_040`. Inside it are the timesheet `fx_xdts_040.xdts`, a folder of
drawings for each column (`A`, `B`, `C`), and a background, `BG.png`, which no column uses.

Every drawing is a small coloured square on a clear background. **A is red, on the top row. B is
green, on the second row. C is blue, on the third row.** The square steps 12 pixels to the right
for each drawing number, so drawing 1 is at the left and drawing 8 is furthest right. Watching
the squares is watching the timing: a square that stays put is a hold, and a square that
vanishes is a blank cell (the cross on a paper sheet).

Document 25, FX-XDTS-040, has the whole sheet written out a frame to a row, as a paper
timesheet is laid out. Keep it open beside the window.

## What to check

1. **The button.** At the foot of the media bin, beside **Import drawings...**, is **Import
   cut...**. Hovering it says what the folder should hold.
2. **The dialog.** Click it. A folder chooser opens, titled "Import a cut: choose the folder that
   holds its .xdts timesheet". Cancel it: nothing changes and nothing is said.
3. **Importing.** Click it again and choose the folder `fx_xdts_040` itself (not a file inside
   it). The window switches to a new composition, **s01 c012**, 160 by 90. The status line
   reads: "s01 c012 is imported and open: 3 layers from its timesheet, 48 frames at 24 frames a
   second, because a timesheet does not say its rate. 4 notes are below. Ctrl+Z takes it all
   back."
4. **The drawings.** Under Drawings there are three new image sequences, one per column. There
   is none for `BG.png`.
5. **The layers.** The layer list has A, B and C, with **C at the top and A at the bottom**,
   because track 0 is the bottom of the stack. Each bar runs the whole 48 frames.
6. **The notes.** The notes strip has four lines. Two say a dialogue field and a camerawork
   field were not read. One names C's tick mark (an inbetween mark) on frame 41. One names
   `BG.png` as used by no column. **Error details** shows each note's code, starting
   TIMESHEET_.
7. **The inspector.** Choose A. Under Frames there is a line **Timesheet: fx_xdts_040.xdts,
   column A, track 0**. B's says column B, track 1, and C's says column C, track 2. A layer you
   made any other way has no Timesheet line.
8. **The exposures.** With B chosen, its exposure list runs 1, 2, 3, 2, 1, 2, 3, 2, then jumps
   from frame 16 to frame 30. That gap is the blank cell. A's list steps 1 to 8 on twos and then
   comes back down.
9. **Stepping through it.** Go to frame 0, then use the Right arrow and check a few frames
   against document 25's table. For example: on frame 0, red is at the far left and green is at
   the far left. On frame 14, red is furthest right (drawing 8). On frames 16 to 19, green is
   gone and only red shows. On frame 20, blue appears. On frames 28 and 29 only red shows. On
   frame 40, blue comes back at the far left.
10. **Playing it.** Press Space. It plays for two seconds and loops. The red body steps on twos,
    holds, and steps back. The green mouth flickers, vanishes for a while, and comes back. The
    blue effect appears late, goes, and appears again near the end.
11. **Undo.** Press Ctrl+Z once. The composition, its three layers and the three new drawings
    are all gone, and the window goes back to the composition you had before. Ctrl+Shift+Z
    brings the whole cut back.
12. **A folder it refuses.** Click **Import cut...** and choose `Fixtures/xdts/fx_xdts_031`,
    which holds two timesheets. The status line begins "Nothing was imported." and names both
    files. The notes have one line saying the same. Nothing is added.
13. **Saved and reopened.** With the cut imported, **Save As...** into a new folder, then
    **Open** that file. s01 c012 is there with the same layers, the same timing and the same
    Timesheet lines in the inspector.

## Known limits

- The frame rate is always 24 at import, because an XDTS file does not store one. Change it
  afterwards with Composition settings (Ctrl+K) if the cut is at another rate.
- Only the first timetable of a sheet is read. Dialogue and camerawork are named in the notes
  and not read.
- The timing shows as layer bars and exposure lists. There is no timesheet-shaped view yet. That
  is the Sheet view you asked about, to be proposed next as its own decision.
- A cut cannot be re-imported over itself to pick up a changed sheet. Import it again as a new
  composition.
- Undo names the import by its first step, "Import A_%04d.png and 3 more", as every
  several-step edit in this window is named.

## What to answer

"works", or which step number did something else and what it did.
