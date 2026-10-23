# B-641 playtest: Cell Pattern gains Offset, Overflow, Cycle Evolution and Tiling (D-762)

## What changed for you

Cell Pattern's panel has the After Effects settings it was missing:

- **Offset** (x, y in pixels) slides the whole pattern; the layer underneath stays put.
- **Overflow**: Clip (as before), Soft Clamp (high contrast eases into white and black instead of
  cutting off), Wrap Back (values past white fold back, drawing rings inside the cells).
- **Cycle Evolution** with **Cycle (in Revolutions)**: with it on, the pattern comes back exactly
  after that many turns of Evolution, so a key from 0 to 360 x Cycle loops without a jump.
- **Enable Tiling** with **Cells Horizontal** and **Cells Vertical**: the pattern repeats after that
  many cells across and down.
- **Contrast** now goes to 10000 (it stopped at 1000), as After Effects' does.

A project saved before this change looks and saves exactly as it did: the test draws all of the
older Cell Pattern fixtures bit for bit as before and saves them back unchanged. The Bubbles look is
not changed.

## What to look at

In `verification/B-641 pictures/`:

- `1_offset_before_after.png`: Offset 0 against Offset 40, 0; the cells move 40 pixels right.
- `2_overflow_contrast_300.png`: Clip, Soft Clamp and Wrap Back at Contrast 300.
- `3_cycle_loop.png`: Cycle 2, Evolution 0 to 720 over frames 0 to 4; frame 4 is frame 0 again.
- `4_tiling.png`: Tiling off against 4 cells across and 2 down; the pattern repeats.
- `5_panel_ae_vs_ours.png`: After Effects' panel (mayankwise tutorial) next to a written list of
  our panel's settings. It is a list, not a screenshot of our window.

`verification/B-641_cell_pattern_more_table.md`: 234 of 234 checks pass (the 26 new fixtures on the
processor and the card, every older fixture unchanged, refusals, undo, tiles).

## Try it

1. Add Cell Pattern to a layer. Set Offset to 100, 0: the cells slide right.
2. Set Contrast to 400 and switch Overflow between Clip, Soft Clamp and Wrap Back.
3. Turn Cycle Evolution on, Cycle 2. Key Evolution 0 at the start and 720 at the end: play it as
   a loop; there is no jump at the seam.
4. Turn Enable Tiling on, Cells Horizontal 4, Cells Vertical 2: the pattern repeats.
5. Open a project saved before today with a Cell Pattern: it looks the same.

## Known differences from After Effects (not fixed here)

- After Effects shows Offset as a point (it starts at the layer's middle, e.g. 960, 540); ours is a
  slide from where the pattern sits, starting at 0, 0.
- Our Evolution already repeats every turn without Cycle Evolution (a choice of D-209); with it on,
  the cells turn at their own speeds and the loop is Cycle turns long. How After Effects moves the
  cells inside its cycle could not be seen in the tutorials.
- The tiling starts at 10 cells each way; After Effects' starting numbers were not seen.
- After Effects has 12 patterns; ours still has 4, and its Bubbles look differs from ours (kept on
  purpose, since Cell Pattern was built as an inspired effect: improve only).
- Evolution is shown in degrees, not After Effects' turns plus degrees.
