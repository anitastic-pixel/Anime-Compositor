# B-101: Iris Wipe, by hand

Built on 2026-09-26 against D-158, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-fifth of the thirty.

The generated halves are `verification/B-101_iris_wipe_table.md`, which renders every
FX-IRIS case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Iris Wipe card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-101a proposal/` shows what to expect.

## Before you start

Open a project with two layers: a full-frame background below and a full-frame picture above.
Select the top layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Iris Wipe** in **Add effect…** on the top layer. It goes to the end of
   the stack, and the card shows **Completion** 0, **Centre** 50, 50, **Feather** 0 and
   **Invert** Off. Nothing changes.
2. **Closing.** Completion 50: the top layer is kept only inside a circle on the middle of the
   frame; everything outside it shows the background.
3. **Nearly shut.** Completion 95: a small circle is left. Completion 100: the layer is gone.
4. **Off-centre.** Centre 25, 75: the circle closes on the lower-left part of the frame. It
   still starts large enough to cover every corner at completion 0.
5. **Feather.** Feather 40: the edge of the circle fades rather than cuts.
6. **Invert.** Invert On: now a hole opens in the middle instead, growing as completion rises,
   and at 100 the layer is gone.
7. **Out of range.** Type 101 in Completion: it is refused with a sentence saying it runs from 0
   to 100, and the card keeps its old number.
8. **Keyed.** Key Completion at 0 and at 100 a second later, with the centre on a character's
   face. Play: the cartoon "that's all" ending, the circle closing on the face.
9. **Draft preview.** Switch to a half-size draft: the circle covers the same part of the
   picture as at full size.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- **Round only.** There is no oval or star shape and no border line round the circle.
- The centre is a fixed point of the layer's own drawing, so moving the layer moves the circle
  with it. To follow a moving character, key the centre.
- It is modelled on After Effects' Iris Wipe and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
