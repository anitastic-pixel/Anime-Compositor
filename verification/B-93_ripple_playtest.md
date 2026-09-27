# B-93: Ripple, by hand

Built on 2026-09-26 against D-150, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the seventeenth of the thirty.

The generated halves are `verification/B-93_ripple_table.md`, which renders every FX-RIPPLE
case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Ripple card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-93a proposal/` shows what to expect.

## Before you start

Open a project with a full-frame background layer with plenty of detail (water, a landscape, a
room). Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Ripple** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Centre** 50, 50, **Amplitude** 5, **Wavelength** 30, **Speed** 20, **Phase** 0
   and **Fade** 0. Rings bend the picture around its middle, like a pebble dropped in a pond.
2. **Moving.** Play: the rings spread outward. Speed -20: they move inward. Speed 0: they stand
   still.
3. **Bigger and closer.** Amplitude 15: a stronger bend. Wavelength 10: tight rings. Amplitude 0:
   the picture untouched.
4. **Fade.** Fade 200: the rings die away 200 pixels from the centre, the rest of the picture
   untouched.
5. **Centre.** Centre 0, 0: the rings spread from the top left corner.
6. **Out of range.** Type 0 in Wavelength: it is refused with a sentence saying it runs from 1
   to 10000, and the card keeps its old number.
7. **Keyed.** Key Amplitude at 10 and at 0 at a later frame. Play: the ripple calms to still
   water.
8. **Draft preview.** Switch to a half-size draft: the rings look the same size on screen as at
   full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **One centre only**, with no second source to interfere with it.
- The rings are perfect circles, with no squash for a surface seen at an angle.
- A large amplitude with a short wavelength breaks the picture up rather than blurring it.
- No light or shading on the crests, only the bending.
- The layer does not grow, so a ripple pushing the picture past its edge is cut there.
- Speed and phase are in degrees of the ring, so 360 is one whole ring; speed 360 looks still.
- It is modelled on After Effects' Wave World and Ripple and is not claimed to match them.

## What to answer

"works", or which step number did something else and what it did.
