# B-666 playtest: Fill on an adjustment layer (D-786)

What changed: a Fill put on an adjustment layer, left at Fill Mask 0 and All Masks off, now fills
everything beneath it with its colour. Before, it did nothing and said nothing.

## What the pictures show

`B-666 pictures/smoke_rig_8bpc_before_after.png` and `smoke_rig_32bpc_before_after.png`: Ben
Marriott's "2D Smoke & Explosions" cloud rig rebuilt in our app, frame 7.

- AE 9:45: After Effects' strokes are pure white.
- before: our strokes are mottled grey (135 to 178 of 255, none at 255): the Fill was skipped.
- after: our strokes are pure white (255), 1.8% of the frame, at 8 and 32 bpc alike.
- after, no matte: the same rig without the stand-in matte, white wherever the noise layer reaches.

Still different, not part of this fix: After Effects' strokes are broken into pieces by the noise
layer's Preserve Underlying Transparency (T switch), which this program does not have.

## Try it yourself (5 minutes)

1. New composition. Add a solid of any colour, then an adjustment layer above it.
2. On the adjustment layer add Fill (Generate). Leave Fill Mask at 0.
   - Expected: the whole solid turns red.
3. Set the Fill's Opacity to 50.
   - Expected: half way between the solid's colour and red.
4. Set Fill Mask to 1.
   - Expected: the solid shows its own colour again. The frame now carries the warning
     EFFECT_PATH_MISSING (checked in the table, rows FX-FILLADJ-007 and 008).
5. Move the Fill onto the solid itself with Fill Mask 0.
   - Expected: red, exactly as before this change.

Pass if every "Expected" holds. The numbers are in `B-666_fill_adjustment_table.md` (26 of 26).
