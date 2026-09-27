# B-88: Emboss, by hand

Built on 2026-09-26 against D-145, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twelfth of the thirty.

The generated halves are `verification/B-88_emboss_table.md`, which renders every FX-EMBOSS
case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Emboss card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The picture in `verification/B-88a proposal/` shows what to expect.

## Before you start

Open a project with a drawn character layer with clear dark outlines, and some empty space
around it. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Emboss** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Direction** 135, **Relief** 1, **Contrast** 100 and **Mode** Grey. The drawing
   turns a flat middle grey with its outlines standing up as thin ridges, light on one side and
   dark on the other, like a coin.
2. **Wider and stronger.** Relief 4: the ridges widen. Contrast 300: they go nearly black and
   white. Relief 0 or Contrast 0: flat grey with no ridges at all.
3. **The light turns.** Drag Direction through 0, 90, 180 and 270: the lit and shaded sides of
   every ridge swap round as the light moves.
4. **Over the colours.** Set Mode to **Over the colours**: the drawing keeps its own colours and
   the edges are lit and shaded over them. Contrast 0 here leaves the drawing unchanged.
5. **Empty space stays empty.** Around the drawing nothing appears; soft line edges stay soft.
6. **Out of range.** Type 101 in Relief: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
7. **Keyed.** Key Direction at 0 and at 360 at a later frame. Play: the light sweeps once round
   the drawing.
8. **Draft preview.** Switch to a half-size draft with Relief 4: the ridges look about as wide
   on screen as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting,
   the mode and every key is still there.

## Known limits, on purpose

- **It looks at brightness only**, so two colours of the same brightness side by side make no
  ridge.
- A relief that is not a whole number, or any slanting direction, reads between pixels, so the
  ridges are softer there.
- Where the drawing touches its layer's edge there is no ridge along that edge.
- A large contrast holds most edges at pure black or white.
- The mode is saved as the words "grey" and "color"; "gray" is refused. The mixed spelling is a
  wording choice you may want changed.
- It is modelled on After Effects' Emboss and Color Emboss and is not claimed to match them.

## What to answer

"works", or which step number did something else and what it did.
