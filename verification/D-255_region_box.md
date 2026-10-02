# D-255: the region box

A new button on the viewer's bar, **Region of interest**, after the field guide's. It is also in View › Region of interest and in the command finder.

- **Click it, then drag a box on the picture.** On a still frame only that box is drawn. Outside it is grey.
- **Click it again** to draw the whole picture.
- **A click without a drag, or Escape during the drag,** draws no box.
- **The box belongs to this window**, like the grid. It isn't in the project or in an export.

## Picture

`D-255 pictures/region_box.png`: the reference shot at frame 60, drawn on the CPU, with a box from 30% to 75% across and 20% to 70% down. Inside the dashed line is the picture. Outside is grey, with nothing drawn under it.

`D-255 pictures/scale_2.0.png`: the window at 200% scale. The viewer's bar wraps onto a second row, and the new button is the third on that row.

## The check (`d255_check.js`), written first

**On the build before the change:** `BUTTON MISSING`. The check stopped there.

**On the new build**, drawn on the CPU:

| Step | What the page said |
|---|---|
| The button | "Region of interest: drag a box on the picture and only it is drawn; click again for all of it" |
| Clicked | button lit, waiting for a drag |
| Dragged from 20%, 25% to 60%, 70% | box 0.2, 0.25, 0.6, 0.7; no layer selected by the drag |
| Box 10%–40% across, 10%–50% down | asked for that part only; 146 × 110 pixels came back; **0 bytes differ** from the same pixels of the whole frame; 0 differ on the screen; the corner outside is empty; grey outline in the same place |
| Box 45%–90% across, 30%–95% down | 218 × 178 pixels; **0 bytes differ**; 0 on the screen; the corner outside is empty |
| Box at the left edge, a quarter wide, full height | 121 × 270 pixels; **0 bytes differ**; 0 on the screen |
| Clicked again | no box; button off; the whole frame asked for; grey gone |
| A box, then the view turned 15° | the box cleared; the button greyed out while turned |
| Errors on the page | none |

The window's own layout and settings were put back afterwards.

## Limits, stated

- **On the graphics card, the box makes nothing faster.** Your machine draws the preview on the card, which always sends the whole picture. The box then greys out what is outside it, and that's all. Only drawing on the CPU (the viewer's Auto / GPU / CPU menu) skips the outside.
- **Play draws the whole picture**, greyed outside the box, as before. Making play faster would be its own change to the frame memory.
- **The box is off while the view is turned or mirrored.** Turning the view clears it.
- **The box is drawn with the mouse.** There is no keyboard way to draw one. The button, the View menu entry and the command finder all clear it.

## Checks

- All 81 of the app's tests pass. The list of wired controls now includes `regionbtn` (`verification/B-12c_keyboard_table.md`).
- No exported picture can change. No engine code was touched.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click the Region of interest button (dashed box, right of the field guide), then drag across the middle of the picture | Only the box is drawn; grey outside | |
| 2 | Click the picture with the button lit but don't drag | No box, and nothing selected | |
| 3 | Click the button again | The whole picture | |
| 4 | Make a box, then turn the view with ↻ | The box goes, and the button is greyed while turned | |

Anything marked ✗, tell me the row number.
