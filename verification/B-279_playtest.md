# B-279: Shadow/Highlight

Built on 2026-10-09 as D-400, under your /loop request. **Shadow/Highlight** (in **Color
Correction**) is our version of After Effects' Shadow/Highlight: it lifts the dark parts of a
picture and brings down the bright parts, judging each pixel by how bright the area round it is,
not by its own brightness alone, so a dark doorway in a bright wall is lifted while the wall stays.

The settings, with the values they start at:

- **Shadow Amount** (50): how much the shadows are lifted, 0 to 100.
- **Highlight Amount** (0): how much the highlights are brought down, 0 to 100.
- **Shadow Tonal Width** and **Highlight Tonal Width** (50): how far into the middle tones each
  reaches, 0 to 100; small numbers touch only the darkest or brightest parts.
- **Shadow Radius** and **Highlight Radius** (30 pixels): how big an area round each pixel is
  looked at, 0 to 500.
- **Color Correction** (20): how much colour the changed parts keep, 0 to 100.

With both amounts at 0 the layer is left exactly as it is. Blend With Original is the Mix every
effect has.

Adobe does not publish how its version works. Ours follows the free darktable program's "shadows
and highlights" (the same idea as GIMP's), with one radius and width each for the shadows and the
highlights, as After Effects has. The check, `verification/D-400_shadow_highlight_table.md` (140
of 140), holds every pixel to numbers worked out by a separate program before the code existed,
and holds the graphics card's picture to within 1 level of the processor's (identical on every
test file). The pictures are in `verification/D-400 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the dark shade, doorways and windows lifted; the sky and lit
   walls look the same. No part of the picture gets darker.
2. **More.** `3_shadow_100.png`, Shadow Amount 100: lifted further, flatter.
3. **Highlights.** `4_highlight_80.png`, Shadow 0, Highlight 80 at radius 4: the sky and lit walls
   brought down, the bright detail easier to see; nothing gets brighter.
4. **Both.** `5_both.png`, Shadow 70, Highlight 60 at radius 10: the picture's range drawn in from
   both ends.
5. **Colour.** `6_colour_0.png` against `7_colour_100.png`, Shadow 100 with Color Correction 0
   and 100: the lifted shadows paler at 0, more coloured at 100.
6. **In the app.** Add Shadow/Highlight to a layer with dark and bright parts: the card shows the
   seven settings above, and the dark parts lift at once. Set Shadow Amount to 0: the layer is as
   it was. Raise Highlight Amount: the bright parts come down. Try a small Shadow Radius (1 or 2)
   against a large one (100): small looks crunchy and detailed, large looks smooth.
7. **Starting numbers.** Do the numbers it starts at match After Effects' when Auto Amounts is
   off? They are from the panel as remembered, not from a document; tell me if any differ.
8. **Out of range.** Type 101 in Shadow Amount: it is refused with a sentence saying it runs from
   0 to 100, and the card keeps its old number.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

**Timing (quiet machine).** In `verification/B-279_shadow_highlight_timing_table.md`, the
reference shot with a moving Noise on three layers, played again: 12.6 ms a frame on the card and
42.3 on the processor without the effect; 24.6 and 199.7 with Shadow/Highlight as added; 30.8 and
256.5 with two different radii (12 and 40). About 4 ms a layer on the card as added, about 6 ms
with two radii, which is over the 4 ms aimed at for blur-type effects.

## Not built

- **Auto Amounts** (After Effects starts with it on): it picks the amounts from the picture by a
  rule Adobe does not publish. Here you set the amounts yourself.
- **Temporal Smoothing** and **Scene Detect**, which go with Auto Amounts.
- **Midtone Contrast** and **Black Clip / White Clip** under More Options: no published rule.
- No tutorial with numbers was found to reproduce.
