# B-281: Tritone

Built on 2026-10-09 as D-402, under your /loop request. **Tritone** (in **Color Correction**)
colours a layer with three colours you choose: one for the dark parts, one for the middle and
one for the light parts.

The settings, with the values they start at:

- **Highlights** (white, #ffffff): the colour the lightest parts become.
- **Midtones** (a sepia brown, #8c7355, our own choice): the colour the middle parts become.
- **Shadows** (black, #000000): the colour the darkest parts become.
- **Blend With Original** (0): how much of the layer as it was shows through, 0 to 100; 100
  leaves the layer exactly as it is.

As added it turns the layer into an old-photo brown. The three colours are typed as #rrggbb,
like Gradient Map's, and are not keyable; Blend With Original is keyable.

Tritone is a second name over Gradient Map, as Levels (Individual Controls) is over Levels: the
same engine with its midpoint in the middle. Adobe does not publish its exact rule; ours is
Gradient Map's. Gradient Map itself is unchanged.

The check, `verification/D-402_tritone_table.md` (179 of 179), holds every pixel to numbers worked
out by a separate program before the code existed, reruns Gradient Map's own checks unchanged,
shows Tritone and Gradient Map with the same colours draw the same picture byte for byte, and
holds the graphics card's picture to within 1 level of the processor's (identical on every test
file). The pictures are in `verification/D-402 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street in browns, like an old photograph.
2. **Three colours.** `3_night_to_sunset.png`, Shadows navy (#1a2a6c), Midtones red (#c0392b),
   Highlights pale yellow (#fdf3a7): the shadows navy, the middle red, the bright parts pale
   yellow.
3. **Blend.** `4_blend_70.png`, the same three at Blend With Original 70: the street with a light
   wash of the three colours, nearer the street than 2.
4. **Same as Gradient Map.** `5_gradient_map_same.png` is Gradient Map with the same three
   colours, midpoint 50: it should look exactly like `3_night_to_sunset.png`.
5. **In the app.** Add Tritone to a layer: the card shows Highlights, Midtones, Shadows and Blend
   With Original. The layer turns brown at once. Pick three colours: the layer takes them. Set
   Blend With Original to 100: the layer is as it was.
6. **Out of range.** Type 101 in Blend With Original: it is refused with a sentence saying it
   runs from 0 to 100, and the card keeps its old number.
7. **Saved and opened again.** Save, close and open the project: the same colours, blend and
   picture.

## Speed

**Timing.** In `verification/B-281_tritone_timing_table.md`, the reference shot with a moving
Noise on three layers, played again: 12.7 ms a frame on the card without the effect, 14.2 with
Tritone as added and 14.2 night to sunset at blend 70; about half a millisecond a layer on the
card, inside the 1 ms aimed at. On the processor 42.6 without and about 51 to 52 with it. Both
rounds on a quiet machine.

## Not built

- After Effects' exact brightness rule is not published; ours is Gradient Map's.
- The three colours are not keyable (as Gradient Map's are not).
- No tutorial with numbers was found to reproduce.
