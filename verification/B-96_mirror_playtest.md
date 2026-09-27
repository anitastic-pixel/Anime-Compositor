# B-96: Mirror, by hand

Built on 2026-09-26 against D-153, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twentieth of the thirty.

The generated halves are `verification/B-96_mirror_table.md`, which renders every FX-MIRROR
case against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Mirror card sends every setting the command reads. This sheet covers what the
tables cannot: how it looks and feels in the window. The picture in
`verification/B-96a proposal/` shows what to expect.

## Before you start

Open a project with a full-frame background layer with plenty of detail, ideally a face or a
character. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Mirror** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Centre** 50, 50 and **Angle** 0. The right half of the picture is untouched and
   the left half is its reflection, as if a mirror stood straight up in the middle.
2. **Turning.** Angle 90: the bottom half is kept and the top shows its reflection. Angle 180:
   the left half is kept. Angle 270: the top half is kept.
3. **Leaning.** Angle 30: the mirror leans. The kept side stays sharp; the reflected side is a
   little softer.
4. **Moving the line.** Centre 25, 50: the line moves to a quarter of the way across, and the
   kept side is now three quarters of the picture.
5. **Edge.** Centre 100, 50: the line is the right edge and everything is on the reflected
   side, whose reflection is past the edge: the layer goes empty. That is on purpose.
6. **Out of range.** Type 3601 in Angle: it is refused with a sentence saying it runs from
   -3600 to 3600, and the card keeps its old number.
7. **Keyed.** Key Angle at 0 and at 360 at a later frame. Play: the mirror line spins round the
   middle, a kaleidoscope-like transition.
8. **Draft preview.** Switch to a half-size draft: the line sits in the same place as at full
   size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **Exact only at whole quarter turns.** At 0, 90, 180 and 270 the reflection copies pixels
  exactly; at any other angle the reflected side is slightly softened.
- One line only, not the several of a kaleidoscope.
- It reflects within the layer only, so a line near an edge reflects emptiness onto part of the
  other side.
- It is modelled on After Effects' Mirror and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
