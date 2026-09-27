# B-89: Find Edges, by hand

Built on 2026-09-26 against D-146, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the thirteenth of the thirty.

The generated halves are `verification/B-89_find_edges_table.md`, which renders every
FX-FINDEDGES case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Find Edges card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-89a proposal/` shows what to expect.

## Before you start

Open a project with a drawn, coloured character layer with clear colour areas, and some empty
space around it. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Find Edges** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Invert** Off and **Amount** 100. The drawing turns white with every boundary
   between two colours, and its outline, drawn as a dark line: a pencil line test of the cel.
2. **Inverted.** Set Invert to **On**: light lines on black, a neon or chalk look.
3. **Part way.** Amount 50: the drawing's own colours show through, half way to the sketch.
   Amount 0: the drawing untouched.
4. **Empty space stays empty.** Around the drawing nothing appears, even with Invert on: the
   black is only where the drawing is.
5. **Out of range.** Type 101 in Amount: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
6. **Keyed.** Key Amount at 0 and at 100 at a later frame. Play: the colour drains away into the
   line drawing.
7. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the choice,
   the amount and every key is still there.

## Known limits, on purpose

- **A line one pixel wide comes out as two thin lines**, one either side of it, with a pale
  middle: the rule compares each pixel's neighbours, not the pixel itself.
- In a half-size draft preview the lines look thicker than at full size.
- Edges are found from lightness alone, so a red and a green of equal brightness meet with no
  line.
- The sketch is always grey, never coloured, and every strong edge is equally black.
- It is modelled on After Effects' Find Edges and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And whether the doubled lines
around thin outlines bother you.
