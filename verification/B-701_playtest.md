# B-701 playtest: the notes beside the frame say when an effect cannot find its mask (D-821)

What changed: a Fill on an adjustment layer that names a mask cannot use it yet, so it fills
nothing (D-786). The frame already said so with EFFECT_PATH_MISSING, but only in the log; the
error details beside the frame stayed empty, so the Fill seemed to do nothing for no reason. Now
the warning is shown beside the frame, as a failed expression's or a missing font's already is.

## What the window says

There is no picture: the fixture's frame is 6 by 2 pixels and the change is the words beside it.
FX-FILLADJ-007 (a Fill naming Fill Mask 1 on an adjustment layer), frame 0, opened in this
build's window by `tools/app_snapshot.py`:

- before: no note beside the frame.
- after: "EFFECT_PATH_MISSING: Adjustment layer adj's Fill names a mask, which a Fill on an
  adjustment layer cannot use yet, so it fills nothing. Set Fill Mask to 0 and All Masks off to
  fill the whole frame beneath, or put the Fill on the layer itself."

## Try it yourself (3 minutes)

1. Open `Fixtures/fill_adjustment/fx_filladj_007.json`.
   - Expected: the error details beside the frame show EFFECT_PATH_MISSING with the sentence above.
2. Open `Fixtures/fill_adjustment/fx_filladj_008.json` (All Masks on).
   - Expected: the same note.
3. Open `Fixtures/fill_adjustment/fx_filladj_001.json` (Fill Mask 0, fills the whole frame).
   - Expected: no note.

Pass if every "Expected" holds. The numbers are in `B-701_frame_notes_table.md` (6 of 6).
