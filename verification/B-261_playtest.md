# B-261: Gamma/Pedestal/Gain

Built on 2026-10-09 under your effects loop request, decided as D-382. **Gamma/Pedestal/Gain**
(in **Color Correction**) is a new effect after After Effects' effect of that name: it lifts the
shadows, then shapes red, green and blue each on its own.

The settings, with the values they start at (at these the layer is left exactly as it is):

- **Black Stretch** (1, up to 4): lifts the dark values of every channel; black and full
  colour stay where they are.
- **Red, Green and Blue Gamma** (1, 0.1 to 10): above 1 lightens that channel's middle, below 1
  darkens it, as Levels' gamma does.
- **Red, Green and Blue Pedestal** (0, -1 to 1): the lowest value that channel can reach.
- **Red, Green and Blue Gain** (1, 0 to 4): the highest value that channel can reach. A
  pedestal above the gain turns the channel over.

All ten can be keyed. The check, `verification/D-382_gamma_pedestal_gain_table.md` (141 of 141),
holds every pixel to numbers worked out by a separate program before the code existed, and the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-382 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** Add Gamma/Pedestal/Gain to a drawing: nothing changes.
2. **Shadows lifted.** `2_black_stretch_3.png`, Black Stretch 3: the dark parts of the street
   open up, nothing gets darker, the whites stay white.
3. **Warmer.** `3_warm.png`, Red Gamma and Red Gain 1.2, Blue Gain 0.8, Blue Pedestal 0.05: the
   street warmer, its blacks a touch blue.
4. **Negative.** `4_negative.png`, every Pedestal 1 and every Gain 0: the street's negative.
5. **In the app.** Key Red Gain from 1 to 0 over a second: the red drains out of the layer.
6. **Saved and opened again.** Change a few settings, save, close and open: the same settings
   and picture.

**Timing.** On a quiet machine (`verification/B-261_gamma_pedestal_gain_timing_table.md`), the reference shot with a moving Noise on three layers, played again: 12.8 ms a frame on the card and 41.6 on the processor without the effect, 14.9 and 53.6 with every channel its own curve, about 0.7 ms a layer on the card.

## Not built

- Adobe does not publish its method, so the rule is ours (document 21): the stretch curve, and
  gamma read as Levels' (above 1 lightens), are our choices.
- The result is held between black and white, as After Effects' 8-bit effect is.
