# D-282 to D-289: Sketch tools and layers

Built on 2026-10-03 at the owner's request: "go ahead with D-282 to D-284; also unsure it was done, but I did like these", followed by the sketch list. D-282 to D-284 were the three proposals. D-285 to D-289 are the rest of the list that D-281 had not already done.

Already done in D-281, so not rebuilt here:
- a size ring for every tool;
- the colour picker;
- zooming and moving the paper;
- smoother pen lines that use every point the pen reports.

## What changed

- **D-282, stroke eraser (Shift+E):** a tap or a drag takes away each whole stroke the circle touches. The touched strokes turn grey until the hand lets go. One Ctrl+Z puts them all back.
- **D-283, layer opacity, lock and order:** each sketch layer has an Opacity slider, a Lock, and Up and Down.
  - A locked layer refuses every stroke, erase, move, Clear and Delete. The status line says "Unlock it to change what is drawn on it", and Clear and Delete layer are greyed out.
  - The list shows the top layer first.
- **D-284, lasso (L):** draw round strokes to choose them. A stroke is chosen when at least half of it is inside the loop. A dashed box goes round what is chosen, and a bar under the paper says how many are chosen.
  - Drag inside the box to move them.
  - The arrow keys nudge them 1 pixel, or 10 with Shift.
  - Delete takes them away. Esc or Deselect lets them go.
- **D-285, size slider and smoothing:**
  - A slider under the three sizes sets any size from 1 to 120 pixels. [ and ] now make it a quarter bigger or smaller.
  - Smoothing (Off to 10, in the Drawing section on the right) steadies a wobbly line.
- **D-286, see-through pressure:** with "Pen presses: Size and see-through" chosen, a pen stroke is fainter where pressed lightly as well as thinner. "Size" keeps D-271's behaviour.
- **D-287, shapes and fill:**
  - Line, Rectangle and Ellipse draw from where the drag starts to where it ends. U steps through them.
  - Fill, a toggle under them, fills a shape or a drawn outline with the colour.
  - A paint bucket that floods an area bounded by other strokes is **not** built.
- **D-288, onion skin and flipbook:**
  - The list beside Onion skin shows 1, 2, 3 or 5 drawn frames each way, the farther ones fainter.
  - Flipbook flips through the drawings at the composition's speed, each held until the next, without the cut underneath. Esc, the button, or a click on the paper stops it.
- **D-289, Save as picture…:** saves the sketch on this frame as a PNG the size of the composition, see-through where nothing is drawn, every shown layer at its opacity. The window asks where.
  - Frames, previews and exports still never include sketches. This is a separate save only.

The size, Smoothing, Fill, pen setting and onion count are remembered on this machine, not in the project.

## What is saved

- A sketch layer saves `opacity` only when it is below 1, and `locked` only when it is on.
- A stroke saves `filled` and `pressure_opacity` only when they are on.
- A project without them saves byte for byte as before.
- A file with an opacity outside 0 to 1, or a lock or fill that is not true or false, is refused, naming where. Nothing is quietly dropped.

The four new commands are `sketch.remove_strokes`, `sketch.move_strokes`, `sketch.set_look` and `sketch.move_layer`. Each is one undo step.

## Core checks

`tests/d282_sketch_edits.rs` writes `verification/D-282_sketch_edits_table.md`, and **30 of 30 checks pass**. They cover:
- removing and moving strokes, and undoing each;
- refusals for a bad stroke number, a repeated one, none at all, and a move that is not a number;
- the opacity range;
- every refusal on a locked layer;
- reordering;
- the saved fields round trip, byte for byte;
- bad saved values refused at their place.

The D-261 and D-271 sketch checks still pass. That includes D-261's proof that no frame changes with sketches.

## Window evidence

The test copy was built from this change (`$S/tgt`, release) and driven by `cdp3.ps1` on the reference shot (1920 × 1080), with real mouse, pen and key events. The steps are in `$S/nc/d282.js`.

| Step | Seen |
|---|---|
| Three brush strokes, at heights 300, 500 and 700 | 3 strokes on Layer 1 at frame 0. |
| Shift+E, then a tap on the middle stroke | The tool became the stroke eraser, and the heading read "taking whole strokes off Layer 1". The middle stroke went, leaving heights 300 and 700, and Undo read "Erase a sketch stroke". |
| Ctrl+Z | All three strokes were back: 300, 500, 700. |
| Lasso round the top stroke | Stroke 0 was chosen, the bar read "1 stroke chosen: drag to move, arrows nudge", and the box was 295,295 to 705,305. |
| Drag inside the box by 60, 40 | The stroke's first point went from 300,300 to 360,340, it stayed chosen, and Undo read "Move a sketch stroke". |
| Right arrow | 361,340. |
| Delete | 2 strokes left, nothing chosen. |
| Ctrl+Z | 3 strokes, with the moved one back at 361,340. |
| Rectangle with Fill, dragged from 900,250 to 1200,450 | Saved filled, with its corners exactly 900,250 and 1200,450. The heading read "drawing on Layer 1 with the rectangle, filled". |
| U, then an unfilled ellipse from 1300,250 to 1600,450 | U chose the ellipse. It was saved unfilled, spanning exactly 1300,250 to 1600,450. |
| U again, then a line from 900,600 to 1500,800 | U chose the line. It was saved as two points, 900,600 and 1500,800. |
| Size slider to 40, then ] once and [ twice | 40, then 50, then 32. |
| The ring at 32 | 31.5 px wide on screen, as worked out from the paper's scale. |
| The same zigzag (14 px each way) at Smoothing Off and at 8 | The saved points wander 14.0 px and 3.0 px from their middle line. |
| A pen stroke, light to firm, with "Size and see-through" | Saved with pressure_opacity, and 41 pressures from 0.05 to 1. |
| Opacity slider to 40 | The layer saves opacity 0.4, the slider reads 40%, and the layer's row adds "40%". |
| Lock, then a drag on the paper | The layer was locked and the button read "Locked", with Clear greyed out. The strokes stayed at 9, and the status read "Layer 1 is locked. Unlock it to change what is drawn on it." |
| Unlock, + New layer, Down | Layer 2 started on top with Up greyed out. Down put it under Layer 1, with Down then greyed out, and Undo read "Reorder the sketch layers". |
| Ellipses on frames 1, 2 and 3, Onion skin at 2 each way, on frame 3 | Frames 2 and 1 showed, frame 1 fainter. |
| Flipbook from frame 0 (24 fps) | It showed frames 1, 2, 3 … 8 in turn, and the heading read "frame 8 · flipbook". |
| Esc | The flipbook stopped and its button went off. |
| The picture Save as picture sends | 1920 × 1080 and a real PNG (its first bytes are the PNG signature), with a see-through corner. Bytes that are not a PNG were refused: "The sketch did not arrive as a PNG picture, so nothing was saved." |
| Page errors | None. |

**Not shown by this test:** the Save dialog itself and the file it writes. Windows' Save dialog cannot be driven by the test, so playtest step 9 is how to judge it.

The app's own checks: 88 passed, 0 failed, 5 ignored.

## Pictures

In `verification/D-282 pictures/`:
- `a1_three_strokes.png`: the three strokes;
- `a2_lasso_chosen.png`: the top stroke chosen, with the bar under the paper;
- `a3_lasso_moved.png`: it moved down and right;
- `a4_shapes.png`: a filled rectangle, an ellipse and a line;
- `a5_smoothing.png`: the zigzag at Off (upper) and at 8 (lower);
- `a6_pen_see_through.png`: the pen stroke fading in from light to firm;
- `a7_opacity_40.png`: the layer at 40%;
- `a8_locked.png`: the locked layer;
- `a9_onion_two_each_way.png`: frame 3, with frames 2 and 1 fading behind;
- `a10_flipbook.png`: the flipbook running, without the cut.

## Playtest

1. Press Alt+4 for Sketch. Draw three lines. Press Shift+E and tap one: the whole line goes. Press Ctrl+Z: it comes back.
2. Press L and draw a loop round a line: a dashed box goes round it. Drag inside the box, then press the arrow keys, then press Delete. Press Ctrl+Z.
3. Click the Rectangle (or press U), turn on Fill, and drag: a filled rectangle. Press U for the ellipse and the line.
4. Drag the size slider under the three dots, and watch the ring change.
5. Set Smoothing to 8 and draw a shaky line: it comes out steadier.
6. With a pen, choose "Size and see-through" and press lightly, then firmly: the light part is fainter.
7. Move the Opacity slider, then click Lock and try to draw: nothing is drawn, and the status line says why. Click + New layer, then Up and Down.
8. Draw on a few frames in a row. Turn on Onion skin and pick "2 each way". Click Flipbook, then press Esc.
9. Click "Save as picture…", choose a place, and open the file: your sketch on a see-through background, without the picture underneath.
