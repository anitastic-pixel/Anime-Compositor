# B-347: Texturize

Built on 2026-10-11 as D-467, under your /loop request and your go-ahead: **Texturize**, in
**Stylize**, after After Effects' Texturize. Another layer's picture is pressed into the layer as
a lit relief, so the layer looks printed on paper, canvas, brick or stone. The texture's own
colours are not laid on; only its lights and darks, as bumps.

- **Texture Layer** (none): the layer whose picture is the texture. Switch that layer's eye off
  so only the relief shows. With none picked nothing happens.
- **Light Direction** (135 degrees): where the light comes from, clockwise from up.
- **Texture Contrast** (1, 0 to 2): how deep the relief is. 0 leaves the layer alone.
- **Texture Placement** (Tile Texture): or Center Texture (one copy in the middle) or Stretch
  Texture To Fit.

The control names are After Effects' own (Adobe's page could not be read, so they come from
Maxon's help and a tutorial); how the relief is drawn is our own rule (Emboss's, read from the
texture), so it is not matched to After Effects pixel for pixel.

The check, `verification/D-467_texturize_table.md` (140 of 140), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-467 pictures/`: the
street (`town.png`, `1_before.png` without the effect) and a small brick picture (`brick.png`) as
the texture.

## What to check

1. **Nothing picked.** `2_as_added.png`: as added, with no Texture Layer: the street unchanged.
2. **Bricks.** `3_bricks.png`: the brick picture tiled as the texture: a brick wall pressed into
   the street, the street's colours still there.
3. **Light from the other side.** `4_light_from_bottom_right.png`: Light Direction -45: the
   edges that 3 lit are now in shade, and the other way round.
4. **Deeper.** `5_contrast_2.png`: Texture Contrast 2: the same bricks, pressed in harder.
5. **One brick.** `6_one_brick_centred.png`: Center Texture: one brick in the middle, the rest of
   the street untouched.
6. **In the app.** Put a picture of a texture under a layer, switch its eye off, add Texturize
   (Stylize) to the layer above and pick it as the Texture Layer; turn the Light Direction dial
   and raise Texture Contrast.
7. **Out of range.** Type 3 in Texture Contrast: it is refused with a sentence giving the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-347_texturize_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Texturize with another layer tiled as the texture on each of three layers, played
again, 122.7 ms a frame on the card against 32.1 for the Noise alone; on the
processor 360.6 against 124.0: a busy machine cannot say whether the card is quicker than the processor for this effect.
