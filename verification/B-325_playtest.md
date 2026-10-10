# B-325: Turbulent Noise

Built on 2026-10-10 as D-445, under your /loop request: **Turbulent Noise**, in **Generate** (After
Effects keeps it under Noise & Grain). After Effects' Turbulent Noise is its newer, faster Fractal
Noise with fewer controls. Here it is a second name over our Fractal Noise, the way Tritone is over
Gradient Map: the same clouds, grey, still unless you key Evolution.

- **Fractal Type**: **Basic** (soft clouds) or **Turbulent** (clouds creased into dark veins).
- **Noise Type**: **Smooth** or **Block** (square steps).
- **Invert**: turns the clouds over, light where they were dark.
- **Contrast** (100) and **Brightness** (0).
- **Size** (100 pixels a cloud), with **Scale Width** and **Scale Height** (100 per cent of it)
  to stretch them.
- **Offset Turbulence**: slides the clouds, in pixels.
- **Complexity** (6): how much fine detail is layered on.
- **Evolution**: key it to make the clouds change; a full turn (360) gives a new cloud.
- **Random Seed**: a different set of clouds.
- **Opacity** (100) and **Blending Mode**: **Normal**, **Multiply**, **Screen** or **Add** over
  the layer.

What is not there, compared with After Effects: only two Fractal Types and two Noise Types (After
Effects has many), no Overflow, no Rotation, Uniform Scaling or Perspective Offset, no Sub Settings,
and only the four Blending Modes above. The clouds are our own noise, not Adobe's, so they will not
match After Effects pixel for pixel.

The check, `verification/D-445_turbulent_noise_table.md` (265 of 265), holds every pixel to numbers
worked out by a separate program before the code existed, holds the graphics card's picture to
within 1 level of the processor's, and checks that Turbulent Noise draws exactly what Fractal Noise
draws with the same settings, no speed, black to white. The pictures are in
`verification/D-445 pictures/`, over the street (`town.png`; `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street covered by large soft grey clouds.
2. **Smaller clouds.** `3_size_30.png`: size 30, smaller clouds.
3. **Turbulent.** `4_turbulent.png`: the same size, the clouds creased into dark veins.
4. **Block.** `5_block.png`: square steps instead of soft clouds.
5. **Inverted.** `6_inverted.png`: picture 3's clouds turned over, light where
   they were dark.
6. **Multiply.** `7_multiply_over_the_street.png`: the street darkened by the clouds, nothing
   lighter.
7. **In the app.** Add Turbulent Noise (Generate) to a picture, key Evolution from 0 to 360 and
   play: the clouds change smoothly. Try Turbulent, Block, Invert, Complexity and the Blending
   Modes.
8. **The same as Fractal Noise.** Add Fractal Noise with Speed 0 and the same Size, Complexity and
   Seed: it draws the same clouds.
9. **Out of range.** Type 21 in Complexity: it is refused with a sentence giving the range.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-325_turbulent_noise_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a
moving Noise and a Turbulent Noise on three layers, played again, 30.8 ms a frame on the card
as added against 16.6 for the Noise alone; on the processor 80.5 against 42.6:
a busy machine cannot say whether the card is quicker than the processor for this effect.
