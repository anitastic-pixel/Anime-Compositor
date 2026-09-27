# B-94: Twirl, by hand

Built on 2026-09-26 against D-151, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the eighteenth of the thirty.

The generated halves are `verification/B-94_twirl_table.md`, which renders every FX-TWIRL case
against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Twirl card sends every setting the command reads. This sheet covers what the
tables cannot: how it looks and feels in the window. The picture in
`verification/B-94a proposal/` shows what to expect.

## Before you start

Open a project with a full-frame background layer with plenty of detail. Select the layer and
press **Full resolution**.

## What to check

1. **Adding it.** Pick **Twirl** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Angle** 90, **Radius** 50 and **Centre** 50, 50. A small patch in the middle of
   the picture, 50 pixels round, is twisted clockwise, most at its centre, fading to nothing at
   its edge.
2. **Bigger.** Radius 400: the twist covers most of the picture. Angle -90: it twists the other
   way.
3. **Strong.** Angle 720: a whirlpool, the picture wound round twice at the middle.
4. **Nothing.** Angle 0, or Radius 0: the picture untouched.
5. **Centre.** Centre 25, 25: the twist moves up and to the left.
6. **Out of range.** Type 3601 in Angle: it is refused with a sentence saying it runs from
   -3600 to 3600, and the card keeps its old number.
7. **Keyed.** Key Angle at 0 and at 720 at a later frame, with Radius 400. Play: the picture
   winds up into a whirlpool, the classic transition into a flashback.
8. **Draft preview.** Switch to a half-size draft: the twist covers the same part of the picture
   as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **A large angle breaks up at the middle** into loose pixels rather than a smooth smear, where
  the picture is wound tightest.
- The twist is a circle, not the drawing's shape.
- The layer does not grow, so a twist cannot swirl a drawing out past its own edge.
- It is modelled on After Effects' Twirl and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
