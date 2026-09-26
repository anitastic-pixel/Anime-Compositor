# B-59: Lens Blur, by hand

Built on 2026-09-26 against D-116, which you accepted the same day as the sixth of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-59_lens_blur_table.md`, 74 of 74, which renders every
FX-LENS case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Lens Blur card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-59a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**. A drawing with small bright dots or sparkles shows the
effect best.

## What to check

1. **Adding it.** Pick **Lens Blur** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Radius** 10 and **Edges** Transparent. The drawing goes out of focus, as in
   the picture's fourth panel.
2. **Discs, not smudges.** Radius 6: each small dot opens into an even round patch with a firm
   rim, as in the third panel, not the soft fading glow Blur gives. Put a Blur of about the same
   size beside it to compare.
3. **Small radius.** Radius 3: compare with the second panel. Radius 0: the drawing is
   untouched.
4. **Edges.** On a layer that fills the frame, Edges Transparent darkens and fades the border;
   Edges Repeat Edge Pixels keeps it solid right to the edge.
5. **Nothing cut off.** With Edges Transparent the blur spreads past the drawing's own box and
   is not clipped at it.
6. **Out of range.** Type 201 in Radius: it is refused with a sentence saying it runs from 0 to
   200, and the card keeps its old number.
7. **Keyed.** Key Radius at 0 and at 10 at a later frame. Scrub between: the drawing slides out
   of focus smoothly.
8. **Draft.** Press **Draft**: the picture is smaller, but just as out of focus.
9. **Big radius.** Radius 100 on a full-size drawing: note roughly how long a frame takes. It is
   expected to be slower than Blur at the same size.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- The disc is always round; there is no blade count or iris shape, and no highlight boost, as
  After Effects' Camera Lens Blur has.
- The whole layer is blurred alike; there is no depth map.
- A radius under 1 changes nothing, but the layer still grows by one empty pixel on each side.
- It runs on the processor only, so large radii on full-size frames are slow.

## What to answer

"works", or which step number did something else and what it did.
