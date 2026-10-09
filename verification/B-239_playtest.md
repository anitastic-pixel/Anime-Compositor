# B-239: Cross Blur

Built on 2026-10-08 under your effects loop request, decided as D-360. **Cross Blur** (in **Blur &
Sharpen**) is our version of CycoreFX's CC Cross Blur. It blurs the layer across only and,
separately, down only, then lays the two blurs together, so a bright point becomes a soft cross
or plus sign. Used for star glints, shine on lights and horizontal lens streaks.

The settings, with the values they start at:

- **Radius X** (10): how far the blur reaches across, 0 to 500 pixels. 0 leaves that direction
  sharp.
- **Radius Y** (10): how far it reaches down.
- **Transfer Mode** (Blend): how the two blurs are laid together. Blend is half of each. Add,
  Screen, Multiply, Lighten and Darken work like the layer blend modes of the same names.
- **Edges** (Transparent): Transparent lets the blur spread past the layer's edge; Repeat Edge
  Pixels keeps the layer's size and reads the edge pixel past it.

The check, `verification/D-360_cross_blur_table.md` (126 of 126), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-360 pictures/`.

## What to check

**`street.png`** is a little night street: a dark blue sky with five white stars, dark houses
with warm lit windows. **`before.png`** is the street with no effect.

1. **As added.** `cross_as_added.png` is Cross Blur as it starts (Radius X 10, Radius Y 10,
   Blend). Each star and window should become a soft plus sign: arms across and down, nothing
   on the diagonals.
2. **A long flare.** `cross_streak_add.png` is Radius X 40, Radius Y 2, Add. Each star and window
   should throw a long bright streak across, and barely any down. The sky gets a little lighter
   too, because in Add the sky is added to itself.
3. **Darken.** `cross_darken.png` is Radius 8 both ways, Darken. Only where both blurs reach does
   the light stay, so each star shrinks to a faint dot with no arms.
4. **In the app.** Add Cross Blur to a layer with bright points. Drag Radius X and Radius Y apart
   and together; try each Transfer Mode. Key Radius X from 0 to 40 and play it: a flare growing
   across.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU Cross Blur as added costs about 1 ms a 1080p layer, and Radius 100 both ways about
6 ms (`verification/B-239_cross_blur_timing_table.md`).

## Not built

- The passing-light tutorial also puts an Overlay copy of the layer on top and a stroke-only copy
  of the text; both are layer work, not part of this effect.
