# B-99: Radial Wipe, by hand

Built on 2026-09-26 against D-156, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-third of the thirty.

The generated halves are `verification/B-99_radial_wipe_table.md`, which renders every
FX-RWIPE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Radial Wipe card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-99a proposal/` shows what to expect.

## Before you start

Open a project with two layers: a full-frame background below and a full-frame picture above.
Select the top layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Radial Wipe** in **Add effect…** on the top layer. It goes to the end
   of the stack, and the card shows **Completion** 0, **Start angle** 0, **Centre** 50, 50,
   **Wipe** Clockwise and **Feather** 0. Nothing changes.
2. **Sweeping.** Completion 25: the top right quarter is gone, like a clock hand swept from
   twelve to three, and the background shows there.
3. **Half and most.** Completion 50: the right half is gone. Completion 75: only the top left
   quarter is left.
4. **Directions.** Completion 25 with Wipe Counterclockwise: the top left quarter goes instead.
   Wipe Both, completion 50: the top half goes, swept both ways from twelve.
5. **Start.** Start angle 90, completion 25: the sweep starts at three o'clock.
6. **Soft.** Completion 50, Feather 60: the sweeping edge fades over a wedge instead of cutting.
   The start line, at twelve, stays hard.
7. **Out of range.** Type 361 in Feather: it is refused with a sentence saying it runs from 0 to
   360, and the card keeps its old number.
8. **Keyed.** Key Completion at 0 and at 100 a second later. Play: a clock-hand wipe reveals the
   picture beneath, the classic "time passes" transition.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **The start line is always hard**, even with a feather, clockwise or counterclockwise; only the
  sweeping edge is softened. With Both there is no start line to see.
- The centre and the wipe belong to the layer's own box, not the frame.
- It is modelled on After Effects' Radial Wipe and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
