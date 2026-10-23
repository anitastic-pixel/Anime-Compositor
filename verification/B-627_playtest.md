# B-627 playtest: a layer's eight more blend modes (D-748)

What changed: a layer can now be put in Darken, Lighten, Hard Light, Difference, Hue, Saturation,
Color or Luminosity, as in After Effects. Normal, Multiply, Screen and Add mix exactly as before.

## Steps

1. Open the program and make a composition with two layers: a picture or video at the bottom and
   a second, different picture above it.
2. On the top layer's row in the timeline, open the blend mode menu (it says "Normal").
3. Check the menu lists, in this order: Normal, Darken, Multiply, Add, Lighten, Screen, Overlay,
   Soft light, Hard light, Difference, Hue, Saturation, Color, Luminosity, Stencil alpha,
   Stencil luma.
4. Pick each of the eight new ones in turn and look at the viewer.
5. Save the project, close it, open it again: the top layer still shows the mode you picked.
6. Export a frame in Hard light or Difference and open it: it looks the same as the viewer.

## What to see

- Darken: only the darker parts of either picture show. Lighten: only the lighter parts.
- Hard light: the top picture's light parts brighten, its dark parts darken; a 50% grey top
  changes nothing.
- Difference: where the two pictures are the same it goes black; a white top turns the bottom
  into its negative.
- Hue / Color: the bottom picture's lightness with the top's colour; Saturation: the bottom's
  colours made as vivid as the top's; Luminosity: the bottom's colours at the top's lightness.
- With a graphics card, the status may say the frame was drawn on the processor: these modes
  are drawn there (as Overlay already was), slower but the same picture.

## Pictures

`verification/B-627 pictures/`: each sheet is After Effects (left), the program before this
change (middle, the mode missing so drawn Normal) and after (right), from the tutorials:

- `threeway_gds_1_hard_light.jpg`: Gareth David Studio, paper in Hard Light at 80% over a castle
  video. Before: median 21 / 90th 45 levels from After Effects; after: 2 / 8 (matches).
- `threeway_km_3_lighten.jpg`: Kenneth Moore, a wall in Lighten over the sea. Before 88 / 194;
  after 4 / 12 (close).
- `threeway_km_8_difference.jpg`: Kenneth Moore, the wall in Difference. Before 69 / 161; after
  6 / 18 (close).

The two "close" ones are the size of the error in reading the picture from the video: the
textbook formula itself is 4 / 12 and 6 / 18 from After Effects on those stills.
