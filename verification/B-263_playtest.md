# B-263: Photo Filter

Built on 2026-10-09 under your effects loop request, decided as D-384. **Photo Filter** (in
**Color Correction**) tints the picture as if it were shot through a coloured glass filter.
Its settings are After Effects' own, with the same starting values:

- **Filter**: Warming Filter (85), Warming Filter (81), Cooling Filter (80), Cooling Filter
  (82), Sepia, Underwater or Custom. It starts on Warming Filter (85).
- **Color**: the filter's colour when the Filter is Custom. The presets ignore it.
- **Density**: 0 to 100, how strongly it tints. It starts at 25 and can be keyed.
- **Preserve Luminosity**: on keeps each pixel's brightness while the colour changes. It starts
  on.

The check, `verification/D-384_photo_filter_table.md` (137 of 137), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-384 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_warming_85.png`: the effect as it is added, Warming Filter (85) at 25. The
   street is a little warmer and about as bright as before.
2. **Cooling.** `3_cooling_80.png`: Cooling Filter (80) at 60. The street is bluer.
3. **Sepia.** `4_sepia.png`: Sepia at 80. The street is brown, like an old print.
4. **Underwater.** `5_underwater.png`: Underwater at 70. The street is green-blue and the reds
   are pulled down.
5. **Custom, brightness not kept.** `6_custom_blue_unkept.png`: a custom blue (#3366cc) at 60,
   Preserve Luminosity off. The street is bluer and darker, as through real tinted glass.
6. **Density 0.** Set Density to 0: the picture should not change at all.
7. **In the app.** Key Density from 0 to 100 over a second. The tint should grow steadily.
   Switch Preserve Luminosity on and off: with it off, the picture gets darker.
8. **The colour only counts for Custom.** Pick a preset, then change the Color: nothing should
   change. Pick Custom: now the Color is used.
9. **Saved and opened again.** Change the settings, save, close and open again. You should get
   the same settings and the same picture.

**Timing.** On a quiet machine (`verification/B-263_photo_filter_timing_table.md`), the reference shot with a moving Noise on three layers, played again: 11.7 ms a frame on the card and 41.8 on the processor without the effect, 14.4 and 53.9 with Photo Filter as added, about 0.9 ms a layer on the card.

## Not built

- **Fourteen of Photoshop's presets.** Warming Filter (LBA), Cooling Filter (LBB), Red, Orange,
  Yellow, Green, Cyan, Blue, Violet, Magenta, Deep Red, Deep Blue, Deep Emerald and Deep Yellow
  have no published colour that we found, so they are left out. Pick Custom and choose the
  colour instead.
- **The method is ours.** Adobe does not publish how Photo Filter works. Ours multiplies by the
  filter's colour at the density, then, if Preserve Luminosity is on, brings each pixel back to
  its own brightness.
