# B-62: Noise, by hand

Built on 2026-09-26 against D-119, which you accepted the same day as the ninth of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-62_noise_table.md`, 83 of 83, which renders every
FX-NOISE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Noise card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The picture in `verification/B-62a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Noise** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Amount** 10, **Mode** Mono, **Seed** 0 and **Animate** On. A fine grey grain
   appears over the drawing, as in the picture's second panel. Nothing appears outside the
   drawing.
2. **Stronger.** Amount 30: a coarser-looking grain, as in the third panel.
3. **Colour.** Mode Colour: the grain is speckled with colour, as in the fourth panel.
4. **Moving grain.** Play or step frame by frame: the grain changes every frame. Animate Off:
   it is the same on every frame.
5. **The same every time.** Step away from a frame and back: its grain is exactly the same.
   Close and open the project: still the same.
6. **Seed.** Seed 7: a different grain. Seed 7.5 looks the same as 7.
7. **Moving the layer.** With Animate Off, move the layer: the grain moves with the drawing,
   rather than staying fixed to the screen.
8. **Amount 0.** The drawing is as it was.
9. **Out of range.** Type 101 in Amount: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
10. **Keyed.** Key Amount at 0 and at 40 at a later frame. Scrub between: the grain fades in.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- In **Draft** the grain is one preview pixel wide and its pattern is not the full-resolution
  pattern; only the full-resolution frame is pinned by the fixtures.
- Pure white can only darken and pure black can only lighten, so the grain looks weaker on them.
- The grain is one pixel wide; there is no grain size or softness, as After Effects' Add Grain
  has.

## What to answer

"works", or which step number did something else and what it did.
