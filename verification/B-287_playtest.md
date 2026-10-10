# B-287: Transform

Built on 2026-10-09 as D-408, under your /loop request, after After Effects' Transform effect.
**Transform** (in **Distort**) moves, scales, skews, turns and fades a layer inside its own
frame, after the effects above it in the stack. It is the layer's own transform again, as an
effect, so it can come after other effects (for example, turn the tiles Motion Tile made), and it
has its own motion blur. The layer never grows: what Transform moves off the layer's edges is cut
off, as in After Effects.

The settings, with the values they start at (as added it changes nothing):

- **Anchor Point** (50, 50): the point it scales and turns round, in per cent of the layer's width
  and height.
- **Position** (50, 50): where the anchor point goes.
- **Uniform Scale** (On): with it on, Scale Height scales both ways.
- **Scale Height** and **Scale Width** (100): per cent; Scale Width is read only with Uniform
  Scale off. A negative scale flips.
- **Skew** (0) and **Skew Axis** (0): slants the layer, up to 85 degrees, along the axis.
- **Rotation** (0): degrees, clockwise.
- **Opacity** (100).
- **Use Composition's Shutter Angle** (On) and **Shutter Angle** (0): its motion blur, which needs
  the layer's motion blur switch and the composition's motion blur on, as in After Effects. With
  Use Composition's Shutter Angle on it uses the composition's shutter; off, its own angle.
- **Sampling** (Bilinear): Bicubic keeps edges crisper when the layer is turned or scaled.

The numbers are keyable.

The check, `verification/D-408_transform_table.md` (184 of 184), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-408 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Turned and smaller.** `2_turned_small.png`, Rotation 20 at Scale Height 80: the street
   smaller and tilted clockwise, with clear corners round it; the picture is no bigger.
2. **Skewed.** `3_skew_25_bicubic.png`, Skew 25, Bicubic: the houses lean, the top of the street
   slid right and the bottom left, clear wedges at the sides.
3. **Faded.** `4_opacity_50.png`, Opacity 50: the street see-through.
4. **Motion blur.** `5_sliding_sharp.png` and `6_sliding_blurred.png`: Position keyed to slide the
   street right, frame 2, without and with motion blur (layer switch and composition blur on):
   the second is smeared sideways.
5. **In the app.** Add Transform to a layer: the card shows Anchor Point, Position, Uniform Scale,
   Scale Height, Scale Width, Skew, Skew Axis, Rotation, Opacity, Use Composition's Shutter
   Angle, Shutter Angle and Sampling. Nothing changes until you move one. Drag Rotation: the
   layer turns inside its own edges.
6. **After another effect.** Put Motion Tile above Transform and turn Transform: the tiles turn
   together, filling the corners Transform alone would leave clear.
7. **Its own motion blur.** Key Position or Rotation, switch on the layer's motion blur and the
   composition's: the moving layer smears. Turn Use Composition's Shutter Angle off and set
   Shutter Angle to 360: a longer smear. Shutter Angle 0: no smear.
8. **Out of range.** Type 86 in Skew: it is refused with a sentence saying it runs from -85 to
   85, and the card keeps its old number.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

**Timing.** In `verification/B-287_transform_timing_table.md`, the reference shot with a moving
Noise on three layers, played again (PROVISIONAL: another lane's build was running during part of
it): 11.7 ms a frame on the card without the effect, 15.5 with a turn (about 1.3 ms a layer) and
22.9 with a bicubic skew and scale (about 3.7 ms a layer). On the processor 45.2, 56.2 and 62.3.
With motion blur the layer is drawn on the processor, as every motion-blurred layer is.

## Not built

- After Effects picks how many moments its motion blur takes; ours takes the composition's
  number of samples.
- Adobe does not say exactly how its bicubic samples; ours is the common Catmull-Rom cubic.
