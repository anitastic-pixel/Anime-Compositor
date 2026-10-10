# B-275: Selective Color

Built on 2026-10-09 as D-396, under your /loop request. **Selective Color** (in **Color
Correction**) is our version of After Effects' Selective Color, which is Photoshop's: it turns the
cyan, magenta, yellow and black up or down in one family of colours at a time, so you can, for
example, make only the reds deeper or only the shadows bluer and leave everything else alone.

The settings, with the values they start at:

- **Method** (Relative): Relative scales each change by how much room the colour has left, as
  Photoshop starts; Absolute changes it by the full amount.
- **Reds, Yellows, Greens, Cyans, Blues, Magentas, Whites, Neutrals, Blacks** (each 0, 0, 0, 0):
  four per cents for each family, cyan, magenta, yellow and black, -100 to 100, each keyable.
  Adding cyan takes red away, magenta takes green away, yellow takes blue away, and black darkens
  all three; a minus amount does the opposite.

How much a pixel belongs to a family is worked out from its colour, smoothly, so there are no
hard edges between families. With every amount at 0 the layer is left exactly as it is. After
Effects' **Colors** menu only chooses which family's sliders are shown, so here all nine are
shown at once.

The rule is Clément Bœsch's reverse-engineering of Photoshop's, published with the free FFmpeg
program. The check, `verification/D-396_selective_color_table.md` (159 of 159), holds every pixel
to numbers worked out by a separate program before the code existed, reproduces Bœsch's own
measurements of Photoshop on an orange (180, 100, 50) to within 1 level, and holds the graphics
card's picture to within 1 level of the processor's (it came out identical on almost every test
file). The pictures are in `verification/D-396 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Reds only.** `2_reds_cyan.png`, Absolute, Reds cyan +100: the red walls turn brown; the
   blue, green and sandy walls, the sky and the road are exactly as before.
2. **Blues only.** `3_blues_bluer.png`, Absolute, Blues yellow -100: the sky and the blue walls
   bluer; nothing else changes.
3. **Shadows.** `4_blacks_lifted.png`, Relative, Blacks black -50: the dark road and windows
   lifted; anything light is untouched.
4. **Lights.** `5_whites_darker.png`, Absolute, Whites black +100: only the lightest pixels darken:
   the white road markings go black, the pale windows and the pale sky above the roofs grey; the
   walls, the dark road and the deep blue sky are untouched.
5. **All nine.** `6_all_nine.png`, every family moved a little: a gentle regrade of the whole
   street.
6. **In the app.** Add Selective Color to a layer with something red in it: the card shows
   Method Relative and nine rows of four fields, all 0, and the drawing does not change. Set
   Method to Absolute and Reds to 100, 0, 0, 0: the red goes dark. Put Reds back to 0, 0, 0, 0:
   the layer is as it was. Try Relative with the same numbers: the change is smaller where red is
   already strong.
7. **Out of range.** Type 101 in any of Reds' four fields: it is refused with a sentence saying it
   runs from -100 to 100, and the card keeps its old numbers.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

**Timing (provisional, the machine was busy).** In `verification/B-275_selective_color_timing_table.md`, the reference shot with a moving Noise on three layers, played again: 12.8 ms a frame on the card and 44.8 on the processor without the effect, 16.2 and 61.1 with Selective Color on the reds, 16.7 and 59.3 with all nine families, about 1.0 to 1.3 ms a layer on the card.

## Not built

- After Effects' Colors menu (which family's sliders are shown) is not a setting; all nine rows
  are shown.
- FFmpeg works on whole levels, where a colour whose darkest channel is exactly 128 is not a
  white at all; we work in fractions, where it is a white by the smallest amount (1 part in 255).
  No picture can tell the two apart by more than a fraction of a level.
- No tutorial with numbers was found to reproduce; Bœsch's Photoshop measurements are reproduced
  instead (table rows "Photoshop, measured by Boesch").
