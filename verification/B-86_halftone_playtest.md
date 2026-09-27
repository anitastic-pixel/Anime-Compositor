# B-86: Halftone, by hand

Built on 2026-09-26 against D-143, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the tenth of the thirty.

The generated halves are `verification/B-86_halftone_table.md`, which renders every
FX-HALFTONE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Halftone card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-86a proposal/` shows what to expect.

## Before you start

Open a project with a drawn character layer (skin, shadow, lines, with some large flat areas)
and some empty space around it. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Halftone** in **Add effect…**. It goes to the end of the stack. The
   card shows **Size** 8, **Angle** 45, **Amount** 100, **Ink** black and **Paper** white. The
   drawing turns into black dots on white, like manga screentone: small dots in the light areas,
   large ones in the shadows, solid black in the lines.
2. **Size.** Size 4: finer dots, twice as many across. Size 20: big, obvious dots.
3. **Angle.** Angle 0: the dots line up square with the frame. 45 is the usual printed look.
4. **Colours.** Ink a dark purple, paper the skin colour: the same dots in sepia-like colours.
   Swap ink and paper: white dots on black.
5. **Amount.** Amount 50: the dots are laid half way over the drawing's own colours.
6. **The screen moves with the layer.** Move or animate the layer's position: the dots travel
   with the drawing rather than staying still on the screen.
7. **Soft edges and empty space.** The dots never spill past the drawing; the empty parts stay
   empty.
8. **Out of range.** Type 1 in Size: it is refused with a sentence saying it runs from 2 to 200,
   and the card keeps its old number.
9. **Keyed.** Key Size at 4 and at 16 at a later frame: the dots grow as it plays. Key Angle
   from 0 to 90: the screen turns.
10. **Draft preview.** Switch to a half-size draft: the dots are half as far apart, so the
    picture looks the same, only coarser.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- **The screen is fixed to the drawing's top left corner**, not to the frame or the canvas. Two
  layers with the same halftone do not line up their dots unless their corners line up.
- Each pixel's own lightness sets its dot, rather than the average of the dot's cell, so a cell
  that crosses an edge is partly one dot size and partly another.
- The dots' edges are hard, one pixel either ink or paper, with no smoothing. At small sizes a
  dot is a few pixels and loses its roundness.
- One screen of one ink, round dots only: not the four turned colour screens of a print, and no
  line or square tones.
- It is modelled on After Effects' Color Halftone and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did. And whether the hard dot
edges are what you want, or they should be smoothed.
