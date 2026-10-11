# B-343: Plastic

Built on 2026-10-11 as D-463, under your /loop request and your go-ahead: **Plastic**, in
**Stylize**, after After Effects' CC Plastic. The layer is given a lit, shiny relief, as if
moulded in plastic or metal: the bumps come from a layer (its own shape at first, or the light
and dark of another you pick), and a light of the effect's own shines on them. The layer keeps
its own colours and shape.

- **Bump Layer** (the layer itself): whose light and dark make the bumps. **Bump Layer
  Placement** (Stretch) fits another layer of a different size: stretched, centred or tiled.
- **Property** (Alpha): which part of the bump layer makes the bumps: Red, Green, Blue, Alpha,
  Luminance or Lightness. Bright is high.
- **Softness** (10, 0 to 100 pixels): rounds the bumps off.
- **Height** (50, -100 to 100): how high they stand; below 0 they sink in.
- **Cut Min** (0) and **Cut Max** (100): the bump values below Min lie flat at the bottom and
  those above Max flat at the top; Min above Max turns the bumps inside out.
- **Light Type** (Distant), **Light Intensity** (100), **Light Color** (white), **Light
  Height** (100), **Light Position** (30%, 30%, for a point light) and **Light Direction**
  (-45 degrees, from the top left, for a distant light).
- **Ambient Light Color** (white) and **Ambient** (25): light everywhere. **Diffuse** (75): light
  on the slopes facing it. **Dust** (0): a softer, dusty light. **Specular** (50) and
  **Roughness** (0.05): the shine, sharp when Roughness is small. **Metal** (0): 0 a shine in the
  light's colour, like plastic; 100 in the layer's own colours, like metal.

The controls are After Effects' own; how it draws is our own rule beyond what the manual says,
and the starting values are ours (After Effects' are not published), so it is not matched to
After Effects pixel for pixel. After Effects' light layers cannot be picked as the light.

The check, `verification/D-463_plastic_table.md` (191 of 191), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-463 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the picture like a rounded plastic tile, its edges lit from the
   top left and shaded at the bottom right, with a shine near the top left corner.
2. **Relief.** `3_luminance_relief.png` (Property Luminance, Softness 4, Height 60): the
   buildings, windows and road markings standing out a little, lit from the top left.
3. **Metal or plastic.** `4_metal.png` and `5_plastic_orange_light.png` (the same under an orange
   light, Metal 100 and 0): warmer, metal-tinted colours in 4; in 5 the shine is the light's
   orange.
4. **Flat.** `6_flat.png` (Ambient 100, Diffuse 0, Specular 0): the street as it was.
5. **In the app.** Add Plastic (Stylize) to some text, switch Light Type to Point and drag Light
   Position, raise Height, swap Cut Min and Cut Max, pick another layer as the Bump Layer with
   Property Luminance, and try Metal at 0 and 100.
6. **Out of range.** Type 101 in Softness: it is refused with a sentence giving the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-343_plastic_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Plastic on three layers, played again, 21.5 ms a frame on the card as
added against 15.7 for the Noise alone; on the processor 138.4 against
57.4: a busy machine cannot say whether the card is quicker than the processor for this effect.
