# D-281: Sketch paper zoom and move, a ring for every tool, any colour, smoother fast lines

Built on 2026-10-03 at the owner's request ("perfect! proceed with your recommendation"). These are the sketch improvements that change nothing saved or exported. The ones that need a change to what is saved are proposed as D-282 to D-284.

## What changed

- **Zoom:** the mouse wheel over the paper zooms from 25% to 800%, about the point under the hand, so that point stays where it is. The heading shows the zoom beside the turn, for example "0° · 133%".
- **Move:** dragging with the middle button, or with the left button while Space is held over the paper, moves the paper.
  - Space over the paper is the hand, not Play. Anywhere else Space still plays.
  - The pointer becomes a hand while Space is held.
- **Reset** puts the paper back straight, at 100%, in the middle.
- **A ring for every tool:** the brush and pencil now show a ring as wide as they draw, as the eraser did since D-271. A pen's pressure changes it while drawing.
- **Any colour:** a round picker after the five colours opens Windows' colour chooser. Picking a colour draws with it, and switches from the eraser to the brush, as the swatches do.
- **Smoother fast lines:** a fast stroke now keeps every point the pen or mouse reported between two screen refreshes, not just one per refresh. The 4-pixel spacing is unchanged.

Zoom and move are views only. A stroke drawn while zoomed is saved in the composition's own pixels, as before.

## Window evidence

The test copy was built from this change (`$S/tgt`, release) and driven by `cdp3.ps1` on the reference shot, with real mouse, wheel and key events.

| Step | Seen |
|---|---|
| Sketch workspace (Alt+4), brush, hand over the paper | The ring showed, 9.84 px wide: size 10 at the paper's scale on screen, as worked out. |
| Pencil | The ring was 4.92 px wide: half the brush, as the pencil draws. |
| Any colour set to #ff8800 | The sketch colour became #ff8800, the tool stayed the brush, and no swatch was lit. |
| Three wheel turns towards the screen | The zoom went to 133.1% and the paper from 1890 to 2516 px wide. The heading read "0° · 133%". The picture point under the hand was 1082, 489 before and after. |
| Middle-button drag of 80, 40 | The paper moved by 80, 40. Nothing was drawn. |
| Space held, left drag of -100, -30 | The paper moved by -100, -30. No stroke was added, and the frame stayed at 0, so Space did not play. |
| A 13-point stroke drawn, zoomed and moved | The first point was saved at 907, 555, exactly where the paper on screen puts it. The stroke was #ff8800, size 10, 13 points. |
| Reset | The zoom went back to 1, the move to 0, 0, and the heading to "0°". |
| Page errors | None. |

**Not shown by this test:** smoother fast lines. The test sends one mouse event per move, so there are no in-between points for it to keep. On a real pen or mouse, Windows reports extra points during fast moves and the stroke now keeps them. The playtest below is how to judge it.

The app's own checks: 88 passed, 0 failed, 5 ignored.

## Pictures

In `verification/D-281 pictures/`:
- `f1_zoomed.png`: the paper at 133% with the ring;
- `f2_drawn_zoomed.png`: an orange stroke drawn zoomed and moved;
- `f3_reset.png`: after Reset.

## Proposed, not built

These need a change to what a sketch saves, so each is a proposal (D-282 to D-284):
- a stroke eraser that removes whole strokes;
- sketch layer opacity, lock and reordering;
- a lasso to select and move strokes.

## Playtest

1. Press Alt+4 for Sketch. Hover the paper with the brush: a ring shows the brush's width. Switch to the pencil: the ring halves.
2. Turn the wheel over a detail: the paper zooms in with that detail staying under the pointer.
3. Hold the middle button and drag, or hold Space and drag: the paper moves. Release Space: Space no longer moves it.
4. Draw a quick scribble, fast. It should look smooth, not made of straight pieces.
5. Click the round picker after the colours, pick any colour and draw.
6. Click Reset: the paper is back straight, whole and in the middle.
