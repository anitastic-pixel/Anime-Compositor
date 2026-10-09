# B-260: Colorama's remaining controls

Built on 2026-10-09 under your effects loop request, decided as D-381. **Colorama** (in **Color
Correction**) already repainted a layer round a ring of colours (D-316); it now has the rest of
After Effects' Colorama controls. A project saved before draws exactly as it did.

The new settings, with the values they start at:

- **Get Phase From** (Intensity): now also Hue, Lightness, Saturation, Value and Zero.
- **Add Phase From** (as Get Phase From): what is read from the Add Phase layer.
- **Add Mode** (Wrap): Wrap, Clamp, Average or Screen, how the layer's phase is added.
- **Interpolate Palette** (On): Off gives hard bands of the ring's colours, no blends between.
- **Opacity 1 to 5** (100 each): how solid each colour of the ring is, used with Modify Alpha.
- **Modify** (All Channels): what of the ring colour the pixel takes: all of it, only its hue,
  lightness or saturation, only its red, green or blue, or nothing.
- **Modify Alpha** (Off): On, the ring's opacities set how solid the pixel is.
- **Change Empty Pixels** (Off): with Modify Alpha On, the empty parts are coloured too.
- **Matching Mode** (Off), **Matching Colour** (white), **Matching Tolerance** (15),
  **Matching Softness** (0): only the pixels near the colour change, by RGB, hue or chroma.
- **Mask Layer** (none) and **Masking Mode** (Luminance): another layer's brightness or
  covering, either way round, says where the change goes.
- **Composite Over Layer** (On): Off leaves only the changed pixels.
- **Layer Placement** (Stretch to Fit): now places the mask layer too.

The check, `verification/D-381_colorama_table.md` (285 of 285), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-381 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street repainted round the rainbow ring by brightness,
   exactly as Colorama drew it before.
2. **Hue only.** `3_modify_hue.png`, Modify Hue: the street keeps its light and shade and takes
   the ring's hues; the grey road and white markings stay as they were.
3. **Hard bands.** `4_interpolate_off.png`, Interpolate Palette Off: every pixel one of the five
   colours, no blends.
4. **One colour picked.** `5_matching_blue_hue.png`, matching the blue wall's hue, Tolerance 8,
   Softness 10: only the blues change.
5. **In the app, a mask.** Put a white-to-black gradient layer in the composition, hide it, and
   choose it as Mask Layer on a drawing with Colorama: the change is full where the gradient is
   white and fades to none where it is black. Masking Mode Inverted Luminance turns it round.
6. **An old project.** Open a project saved with a Colorama before today: it looks the same.
7. **Saved and opened again.** Change a few of the new settings, save, close and open: the same
   settings and picture.

**Timing.** On a quiet machine (`verification/B-260_colorama_timing_table.md`), the reference shot with a moving Noise on three layers, played again: Noise alone 12.2 ms a frame on the card and 42.5 on the processor; with a Colorama from an old file 15.3 and 54.0 (before this work 14.6 and 56.1, so no slower); with Modify Hue and matching by chroma 20.8 and 66.7; masked by a layer, which keeps it on the processor, 77.1 and 75.8.

## Not built

- Adobe does not publish its method, so the rule is ours (document 21).
- No preset palettes (After Effects' Fire, Golden, Ramp Grayscale and the rest); the ring is up
  to five colours, evenly spaced, rather than a ring of any number of points placed by hand.
- Some names are ours: Modify None, Value and Zero as phases, and the four masking choices.
- The mask layer is placed by the same Layer Placement as the Add Phase layer.
- With an Add Phase layer or a Mask Layer chosen, the processor draws Colorama; otherwise the
  graphics card does.

## Waiting for you

- FX-COLORAMA-034's file says the missing-mask-layer warning comes at each frame only; every
  other effect that reads a layer also warns when the project opens (D-189), and so does this
  one. The fix to the file (one word) is proposed, not made: your call.
