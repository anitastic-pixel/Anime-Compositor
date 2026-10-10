# B-303: Glue Gun

Built on 2026-10-10 as D-424, under your /loop request: **Glue Gun**, in **Generate**, our name
for CycoreFX's CC Glue Gun. Animate its brush and it squeezes out a glossy, blobby stroke along
the brush's path, like toothpaste or a glue gun. The paint can mirror the layer, and it is lit the
way Blobbylize is.

- **Brush Position** (50, 50 per cent): where the brush is. Key it to draw.
- **Stroke Width** (20 pixels): how thick the stroke is.
- **Density** (5 blobs a frame): how many blobs the brush lays each frame. A slow brush lays them
  close together; a still brush piles them up.
- **Time Span** (1 second): how long the stroke lasts behind the brush. 0 keeps it for ever.
- **Reflection** (50): how much the paint mirrors the layer from across the stroke.
- **Strength** (50): how much the blobs run into one another. 0 leaves separate round beads.
- **Paint Style** (Plain): Wobbly makes each blob swing about by **Wobble Width**, **Wobble
  Height** (10 pixels) and **Wobble Speed** (1 turn a second).
- **Light** and **surface**: the same as Blobbylize's (intensity, colour, distant or point light,
  height, position, direction, ambient, diffuse, specular, roughness, metal).

CycoreFX's manual says what each setting does in words but gives no formula, ranges or starting
values, so those are ours, and the look is not matched pixel for pixel. After Effects' own lights
are not offered (only the effect's light). Only the brush is read from the frames before: Stroke
Width and every other setting apply to the whole stroke as they are now. All logged.

The check, `verification/D-424_glue_gun_table.md` (198 of 198), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-424 pictures/`, each the
street at frame 4 with the brush keyed over frames 0 to 4 (`town.png` is the layer).

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png`: the brush never moved, so one round glossy blob sits in the
   middle, lit from the top left.
3. **A stroke.** `3_sweep.png`: the brush moved from bottom left to top right, the stroke kept for
   ever: a shiny tube of paint across the street, the houses showing in it.
4. **Wobbly, lit from above.** `4_wobbly_point.png`: the same across the middle, wobbly, with a
   point light above: a lumpy stroke whose blobs swing out of line.
5. **Mirror, orange shine.** `5_mirror_orange.png`: a thick stroke, Reflection 100, an orange
   highlight: the street seen mirrored inside a glassy tube.
6. **Beads.** `6_blobs.png`, Density 1, Strength 0, width 30: five separate glossy beads, one for
   each frame.
7. **In the app.** Add Glue Gun (Generate) to a picture, key Brush Position at two or three places
   a second apart, and play: the stroke is drawn as the brush moves, and with Time Span 1 its tail
   dries away a second behind. Set Time Span 0: the whole stroke stays.
8. **Out of range.** Type 501 in Stroke Width: it is refused with a sentence saying it runs from 0
   to 500.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-303_glue_gun_timing_table.md` (quiet): the reference shot with a moving
Noise and a Glue Gun on three layers, played again, 17.3 ms a frame on the card as added
against 12.2 for the Noise alone; on the processor 65.2 against 44.3. With the
brush keyed across at width 60, Density 10, Time Span 2: 37.1 on the card against the
processor's 88.1: the card is quicker than the processor for this effect.
