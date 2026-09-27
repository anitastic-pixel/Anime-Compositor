# B-92: Wave Warp, by hand

Built on 2026-09-26 against D-149, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the sixteenth of the thirty.

The generated halves are `verification/B-92_wave_warp_table.md`, which renders every FX-WAVE
case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Wave Warp card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-92a proposal/` shows what to expect.

## Before you start

Open a project with a drawn layer that has clear straight lines (a flag, a title, a striped
shirt), at least a few hundred pixels across. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Wave Warp** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Shape** Sine, **Height** 10, **Width** 40, **Direction** 90, **Speed** 0,
   **Phase** 0 and **Edges** Transparent. Level lines across the drawing turn into waves, each
   column pushed up or down, a wave every 40 pixels along.
2. **Taller and longer.** Height 30: bigger waves. Width 200: long, slow waves. Height 0: the
   drawing untouched.
3. **Direction.** Direction 0: the wave runs up the drawing and pushes each row left and right,
   so upright lines wave instead. Direction 45: a slanted wave.
4. **Triangle.** Shape **Triangle**: sharp zigzags instead of smooth waves.
5. **Moving.** Speed 20, then play: the wave travels along the drawing, a flag in the wind.
   Phase 180 at speed 0: the wave slides half a wave along.
6. **Edges.** With **Transparent**, the waving edge of the drawing sways out into the space
   around it. With **Repeat**, the layer does not grow and the edge pixels are stretched in to
   fill.
7. **Out of range.** Type 0 in Width: it is refused with a sentence saying it runs from 1 to
   10000, and the card keeps its old number.
8. **Keyed.** Key Height at 0 and at 20 at a later frame. Play: the drawing starts still and
   begins to wave.
9. **Draft preview.** Switch to a half-size draft: the waves look the same size on screen as at
   full size.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every
    setting, both words and every key is still there.

## Known limits, on purpose

- **Two shapes only**, sine and triangle, with none of After Effects' square, sawtooth, circle
  or noise waves.
- **The layer's edges are not pinned**: the whole drawing sways, its border included.
- The push is one straight line across the wave, so the wave never folds over or curls.
- On a small drawing the starting height of 10 is large and pushes much of it about; on a full
  frame it looks right.
- Phase and speed are in degrees of the wave, so 360 is one whole wave; a phase keyed from 0 to
  360 slides the wave once along.
- It is modelled on After Effects' Wave Warp and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
