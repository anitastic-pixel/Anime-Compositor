# B-74: Offset, by hand

Built on 2026-09-26 against D-131, which you accepted the same day as the ninth of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-74_offset_table.md`, 82 of 82, which renders every
FX-OFFSET case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Offset card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The picture in `verification/B-74a proposal/` shows what to expect.

## Before you start

Open a project with a drawn background that fills its layer edge to edge, ideally one that
tiles, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Offset** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Shift** 0, 0 as two boxes, x and y. The background does not change.
2. **Sliding.** Shift 100, 0: the background slides 100 pixels right, and what leaves the right
   edge comes back in at the left. Shift 0, -50: it slides up, and the top comes back at the
   bottom.
3. **All the way round.** Set x to the layer's width in pixels: the background is back where it
   started.
4. **A looping slide.** Key Shift at 0, 0 on the first frame and at the layer's width, 0 on the
   last. Play with looping on: the background slides round for ever with no jump.
5. **Parts of a pixel.** Shift 0.5, 0: the picture softens very slightly, as each pixel mixes with
   its neighbour.
6. **Stays inside the layer.** Move the layer so the frame shows its edge. The slid picture stays
   inside the layer; nothing is drawn beyond it.
7. **After a shadow.** Add a Drop Shadow before the Offset: the shadow slides round with the
   drawing.
8. **Out of range.** Type 100001 in x: it is refused with a sentence saying it runs from -100000
   to 100000, and the card keeps its old number.
9. **Draft.** Press **Draft**: the picture is smaller, and it is slid by the same share of the
   layer as at full resolution.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the shift and
    its keys are still there.

## Known limits, on purpose

- It always wraps. There is no setting to let the picture slide off and leave the edge empty.
- The wrap goes round the layer as it reaches the effect, so after an effect that grows the
  layer, such as a Drop Shadow or a Glow, one whole width is a little more than the drawing's.
- There is no point box on the card: the shift is pixels, not a place on the drawing.
- It is modelled on After Effects' Offset and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
