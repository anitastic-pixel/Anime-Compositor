# B-98: Linear Wipe, by hand

Built on 2026-09-26 against D-155, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-second of the thirty.

The generated halves are `verification/B-98_linear_wipe_table.md`, which renders every
FX-LWIPE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Linear Wipe card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-98a proposal/` shows what to expect.

One case, FX-LWIPE-032, a file with completion written as the word "50", is shown in the table
as in dispute rather than passed or failed: the fixture expects the file to open, and this
build refuses it, as it refuses every effect with a number written as a word. D-164, proposed
in document 14, asks you which is right.

## Before you start

Open a project with two layers: a full-frame background below and a full-frame picture above.
Select the top layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Linear Wipe** in **Add effect…** on the top layer. It goes to the end
   of the stack, and the card shows **Completion** 0, **Angle** 90 and **Feather** 0. Nothing
   changes.
2. **Wiping.** Completion 50: the left half of the top layer is gone, and the background shows
   there, with a hard straight edge down the middle.
3. **All the way.** Completion 100: the top layer is gone entirely.
4. **Direction.** Completion 30 and Angle 270: the wipe comes from the right instead. Angle 0:
   from the bottom. Angle 45: from the bottom-left corner, on a diagonal.
5. **Soft.** Feather 40: the edge fades over about 40 pixels instead of cutting.
6. **Out of range.** Type 101 in Completion: it is refused with a sentence saying it runs from
   0 to 100, and the card keeps its old number.
7. **Keyed.** Key Completion at 0 and at 100 a second later, with Feather 30. Play: the top
   picture is wiped away left to right, revealing the one beneath, the classic scene change.
8. **Draft preview.** Switch to a half-size draft: the edge is as soft, for its size, as at full
   size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **The wipe crosses the layer's own box**, not the frame, so on a small layer it finishes when
  the edge has crossed that layer, not the screen.
- The feather is centred on the edge and fits inside the sweep, so completion 0 and 100 are
  always exact.
- It is modelled on After Effects' Linear Wipe and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did; and for D-164, whether a
file with a number written as a word should open or be refused.
