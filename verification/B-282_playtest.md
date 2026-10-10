# B-282: Aerial Haze

Built on 2026-10-09 as D-403, under your /loop request: the eighth of your plugin picks in
`docs/effects/PLUGINS.md`. **Aerial Haze** (in **Color Correction**) makes a layer look far away:
it moves every colour toward the colour of the air, so the blacks lift, the contrast drops and
the colours fade toward blue. It does it evenly, or by the brightness of another layer (a matte)
so that only the far parts get hazy.

The settings, with the values they start at:

- **Haze Color** (a pale sky blue, #b4c8dc, our own choice): the colour of the air.
- **Amount** (30): how far toward the haze colour, 0 to 100; 0 leaves the layer as it is, 100
  makes every pixel the haze colour.
- **Matte Layer** (none): a layer of the composition whose brightness says how far away each
  part is: white is far (full haze), black is near (no haze). With none, the haze is even. The
  matte layer can be switched off; it is still read.
- **Matte Placement** (Stretch Matte to Fit): how the matte lies on the layer: stretched to fit,
  centred, or tiled.

Haze Color is typed as #rrggbb and is not keyable; Amount is keyable.

The rule is ours, after taka2's note on aerial perspective (raise the black level rather than
brighten, lower the saturation and shift toward the colour of the air). The note gives no numbers
and says nothing about whites, so whites move toward the haze too: a pale blue haze makes pure
white a little bluer and darker.

The check, `verification/D-403_aerial_haze_table.md` (162 of 162), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-403 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the whole street paler and bluer, its darkest parts lifted to
   a blue-grey, as if seen through a light mist.
2. **Thick haze.** `3_amount_70.png`, Amount 70: a thick fog, much nearer the sky blue than 1.
3. **Through a matte.** `matte.png` is white at the top and black at the bottom.
   `4_matte_top_far.png`, Amount 80 through it: the top of the street deep in haze, fading down
   to the bottom row, which is exactly as it was.
4. **In the app.** Add Aerial Haze to a background layer: the card shows Haze Color, Amount,
   Matte Layer and Matte Placement. The layer turns misty at once. Set Amount to 0: the layer
   is as it was. Pick a darker blue: the haze takes it.
5. **A matte layer.** Make a gradient layer (white at the top, black at the bottom), switch it
   off, and choose it as the Matte Layer: only the top of the background is hazy. Try Centre
   Matte and Tile Matte.
6. **A matte that is gone.** If the matte layer is deleted, or a project names a layer that is
   not there, the background is drawn as it was, with a warning that the layer it reads is not
   in the composition (checked in the table with a project naming `gone`).
7. **Out of range.** Type 101 in Amount: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
8. **Saved and opened again.** Save, close and open the project: the same colour, amount, matte
   and picture.

## Speed

**Timing.** In `verification/B-282_aerial_haze_timing_table.md`, the reference shot with a moving
Noise on three layers, played again, both rounds on a quiet machine: 12.0 ms a frame on the card
without the effect and 13.9 with Aerial Haze as added, about 0.6 ms a layer, inside the 1 ms
aimed at. Through a matte layer it is 44.3 ms: the matte layer is drawn on the processor first,
as Gradient Wipe's map is (43.7 ms there), so a matte costs about 10 ms a layer. On the processor
43.2 without, 45.8 even, 66.8 with a matte.

## Not built

- No depth pass: a matte layer stands in for distance.
- No separate black-level and saturation controls: one colour and one amount do both.
- Haze Color is not keyable.
- taka2's note gives no numbers, so there is nothing to match exactly.
