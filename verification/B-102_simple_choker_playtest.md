# B-102: Simple Choker, by hand

Built on 2026-09-26 against D-159, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-sixth of the thirty.

The generated halves are `verification/B-102_simple_choker_table.md`, which renders every
FX-CHOKE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Simple Choker card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-102a proposal/` shows what to expect.

## Before you start

Open a project with a light background below and, above it, a character drawing on a
transparent layer, ideally one with a soft or fringed edge and a few thin lines. Select the
drawing and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Simple Choker** in **Add effect…** on the drawing. It goes to the end of
   the stack, and the card shows **Choke** 0. Nothing changes.
2. **Shrinking.** Choke 2: the drawing's edge pulls in by about two pixels all round; hair-thin
   lines and loose specks vanish, and small holes open wider. The colours inside do not change.
3. **Spreading.** Choke -2: the edge grows out by about two pixels; pinholes and small gaps in
   the lines fill, in the colour of the drawing next to them, not black or grey.
4. **Under one.** Choke 0.5 or -0.5: nothing changes.
5. **Moved.** With choke -3, move the drawing: the spread edge moves with it and is not cut off
   at the drawing's old box.
6. **Out of range.** Type 101 in Choke: it is refused with a sentence saying it runs from -100
   to 100, and the card keeps its old number.
7. **Keyed.** Key Choke at 0 and at -6 a second later. Play: the drawing fattens as it goes, in
   whole-pixel steps rather than smoothly (see below).
8. **Draft preview.** Switch to a half-size draft: the shrink or spread covers about the same
   part of the picture as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the setting
   and its keys are still there.

## Known limits, on purpose

- **Whole-pixel steps.** The reach counts whole pixels, so a keyed choke thickens in steps and
  a large choke's corners round off in pixel stairs.
- **Hard-edged.** A shrink or spread has none of After Effects' Matte Choker softness.
- **The drawing's own edge counts as empty**, so a drawing that fills its whole layer loses its
  outer pixels when shrunk.
- **Slow when large.** A choke of 50 or more on a large drawing takes noticeably longer.
- It is modelled on After Effects' Simple Choker and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
