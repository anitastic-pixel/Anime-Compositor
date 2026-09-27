# B-90: Sharpen, by hand

Built on 2026-09-26 against D-147, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the fourteenth of the thirty.

The generated halves are `verification/B-90_sharpen_table.md`, which renders every FX-SHARPEN
case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Sharpen card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-90a proposal/` shows what to expect.

## Before you start

Open a project with a drawn character layer, ideally one that looks a little soft (scaled up,
or with a slight blur under it). Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Sharpen** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Amount** 100 and **Radius** 1. Edges between colours look crisper: each side of
   an edge is pushed a little further from the other.
2. **Stronger.** Amount 300: the edges get bright and dark halos. Amount 500: the halos burn to
   black, white or pure colour.
3. **Wider.** Radius 4 at amount 100: the halos along each edge widen.
4. **Nothing.** Amount 0, or Radius 0: the drawing untouched.
5. **The outline.** The drawing's outline against empty space gets no halo, and nothing
   appears around the drawing.
6. **Out of range.** Type 501 in Amount: it is refused with a sentence saying it runs from 0 to
   500, and the card keeps its old number.
7. **Keyed.** Key Amount at 0 and at 300 at a later frame. Play: the drawing crisps up.
8. **Draft preview.** Switch to a half-size draft with Radius 4: the halos look about as wide
   on screen as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **There is no threshold**, so film grain and noise are sharpened as readily as edges.
- It sharpens each colour channel on its own, so an edge between two colours can shift their
  hues a little.
- A strong amount flattens the halos to black, white or pure colour.
- A large radius on a large layer is slow.
- The starting amount of 100 may be stronger than you want for a gentle touch-up; say if it
  should start lower.
- It is modelled on After Effects' Unsharp Mask and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And whether 100 is a good
starting amount.
