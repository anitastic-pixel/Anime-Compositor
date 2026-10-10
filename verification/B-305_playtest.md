# B-305: Lens Flare

Built on 2026-10-10 as D-426, under your /loop request: **Lens Flare**, in **Generate**,
PLUGINS.md pick #9. It draws the flare a bright light makes shining into a camera lens over the
layer: a hot glow with rays at the light, a coloured halo round it, and a row of coloured
"ghost" circles along a line from the light through the middle of the picture.

- **Flare Center** (30, 30 per cent): where the light is. It can sit off the picture; then only
  the ghosts and the edge of the glow show.
- **Flare Brightness** (100): how strong the flare is, 0 to 300. 0 shows no flare.
- **Lens Type**: **50-300mm Zoom** (a blue halo, many ghosts), **35mm Prime** (smaller, whiter,
  eight rays) or **105mm Prime** (a big glow, twelve long rays, a faint warm halo).
- **Blend With Original** (0): 100 shows the layer without the flare.

The controls are After Effects' own; the look of each lens is built from the parts Optical Flares
uses, and is ours, not After Effects' (theirs is not matched pixel for pixel). Like After Effects',
the flare is drawn on the layer it is on, so put it on a full-frame solid (black, set to Add or
Screen) to light a whole shot. All logged.

The check, `verification/D-426_lens_flare_table.md` (146 of 146), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-426 pictures/`, each the
street at frame 0 (`town.png` is the layer).

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png`: the zoom flare up and left: a hot glow with six rays, a faint
   blue halo, and coloured ghosts running down to the lower right.
3. **35mm.** `3_prime_35mm.png`: a smaller, whiter glow, eight thin rays, three ghosts.
4. **105mm.** `4_prime_105mm.png`: the light up and right, a big white glow, twelve long rays, a
   faint warm halo, ghosts down to the left.
5. **Bright.** `5_bright.png`: brightness 250 near the top middle: the glow washes the sky.
6. **Blended.** `6_blended.png`: blend 60: the same flare as 2, fainter.
7. **In the app.** Add Lens Flare (Generate) to a picture and drag Flare Center across it: the
   ghosts swing round the middle of the picture, always on the line through the light.
   Switch Lens Type: the flare changes at once.
8. **Out of range.** Type 301 in Flare Brightness: it is refused with a sentence saying it runs
   from 0 to 300.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-305_lens_flare_timing_table.md` (quiet): the reference shot with a
moving Noise and a Lens Flare on three layers, played again, 17.7 ms a frame on the card as
added against 13.3 for the Noise alone; on the processor 86.8 against 41.1:
the card is quicker than the processor for this effect.
