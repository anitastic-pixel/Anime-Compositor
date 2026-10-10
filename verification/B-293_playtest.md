# B-293: Circle

Built on 2026-10-10 as D-414, under your /loop request: After Effects' Circle, in **Generate**.
It draws a disk or a ring, in place of the layer or laid over it.

- **Center** (50, 50 per cent): where the circle sits.
- **Radius** (75 pixels): its size.
- **Edge** (None): None draws a solid disk. Edge Radius draws a ring between Radius and **Edge
  Radius**. Thickness draws a ring **Thickness** pixels wide, inside the radius. Thickness *
  Radius makes that width a share of the radius (Thickness 25 is a quarter of it), so the ring
  keeps its look as the radius changes; Thickness & Feather * Radius does the same for the
  feathers.
- **Feather Outer Edge**, **Feather Inner Edge** (0): soften the outside and the inside of the
  ring.
- **Invert Circle** (Off): On draws everything except the circle.
- **Color** (white), **Opacity** (100).
- **Blending Mode** (None): None puts the circle in the layer's place, the rest see-through.
  Normal, Add, Multiply, Screen, Overlay and Soft Light lay it on the layer; Stencil Alpha shows the
  layer only through the circle.

After Effects also offers Hard Light, Color Dodge, Color Burn, Darken, Lighten, Difference,
Exclusion, Hue, Saturation, Color, Luminosity and Silhouette Alpha here; the program has none of
those blends yet, so they are not offered. Adobe's manual does not say how the products with the
radius are measured; reading Thickness as a share (per cent) of the radius is our choice.

The check, `verification/D-414_circle_table.md` (268 of 268), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-414 pictures/` (`town.png` is the layer). In these
pictures a see-through part shows as white or as your viewer's checker.

## What to check

1. **As added.** `2_as_added.png`: a white disk in the middle, the rest see-through.
2. **Normal, feathered.** `3_normal_feathered.png`: a soft orange disk over the street at 60 per
   cent, its edge fading out.
3. **Ring.** `4_ring_multiply.png`: a dark violet ring darkening the street, the middle untouched.
4. **Vignette.** `5_vignette.png`: inverted, black, feathered wide: the corners darkened, the middle
   clear.
5. **Stencil Alpha.** `6_stencil.png`: the street seen only through a soft disk.
6. **In the app.** Add Circle (Generate) to a layer: a white disk in its place. Drag Center: it
   moves. Set Edge to Thickness and Thickness to 20: a ring. Turn Invert Circle on with Blending
   Mode Normal: the layer shows only outside. Key Radius from 20 to 300 and play: the circle grows.
   Switch to Draft: the same circle, a little softer.
7. **Out of range.** Type -1 in Radius: it is refused with a sentence saying it runs from 0 to
   10000.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-293_circle_timing_table.md` (quiet): the reference shot with a moving Noise and a Circle on three layers, played
again, 16.6 ms a frame on the card as added against 11.9 for the Noise alone, and 18.7
with a Thickness ring and Overlay; on the processor 53.2 and 63.9 against 44.5.
