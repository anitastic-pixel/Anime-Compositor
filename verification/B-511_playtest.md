# B-511: Caustics

Built on 2026-10-12 as D-631, the second of the P0-17 group (D-600, your plan: "how AE treats
them/are done"), approved by the coordinator for your /loop request: Caustics, under
**Generate** next to Wave World, our version of After Effects' effect of that name. It shows a
bottom seen through rippling water, the way a pool floor looks from above: the waves bend the
picture, gather the light into bright wavy lines, and the water's surface colour, lit and able
to mirror a sky, lies over it.

- **Bottom** (none: the layer's own picture), **Bottom Scaling** (1), **Bottom Repeat Mode**
  (Tiled), **If Bottom Size Differs** (Stretch to Fit), **Bottom Blur** (0): what lies on the
  floor, how big, how it repeats past its edges, and how soft it is.
- **Water Surface** (none: flat water), **Wave Height** (0.2), **Smoothing** (5): a grey layer,
  best a Wave World, whose bright and dark parts are the waves' crests and troughs.
- **Water Depth** (0.1) and **Refractive Index** (1.2): how far the waves bend the floor; Depth
  0 or Index 1 bends nothing.
- **Surface Color** (blue) and **Surface Opacity** (0.3): the water's own colour and how much
  of it covers the floor.
- **Caustics Strength** (0): the bright and dark patches the waves make on the floor.
- **Sky**, **Sky Scaling**, **Sky Repeat Mode**, **If Sky Size Differs**, **Sky Intensity**
  (0.3), **Sky Convergence** (0.2): a layer mirrored in the water, bent by the waves.
- **Light Type** (Distant Source), **Light Intensity** (1), **Light Color** (white), **Light
  Position**, **Light Height** (1), **Ambient Light** (0.25), **Diffuse Reflection** (0.75),
  **Specular Reflection** (0), **Highlight Sharpness** (5): Card Wipe's light, shining on the
  water's surface.

Adobe gives no formula and few settings when added, so most of those are ours; the Refractive
Index 1.2 is Adobe's. Not built: lighting from the composition's first light (there are no
light layers yet), and the extra blur After Effects gives deeper water.

The check, `verification/D-631_caustics_table.md` (153 of 153), holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-631 pictures/`, drawn at the reference shot's size from it.

## What to check

1. **As added, with water.** `1_as_added_with_water.png`: the picture seen through gentle
   ripples, bent a little, with a light blue tint.
2. **Deeper waves.** `2_deeper_waves.png`: the same picture bent much further.
3. **Caustics.** `3_caustics.png`: bright wavy lines and dark patches across the picture, like
   sunlight on a pool floor.
4. **A tiled floor.** `4_floor_tiled.png`: a floor picture shrunk to a quarter and repeated
   across, seen through the water.
5. **A mirrored sky.** `5_sky_mirrored.png`: a sky picture showing on the water, wobbling with
   the waves.
6. **A point light.** `6_point_light_highlight.png`: a shiny spot where a near light catches the
   wave crests.
7. **Everything.** `7_everything.png`: floor, waves, caustics, sky and light together.
8. **In the app.** Put a Wave World on a layer and switch its eye off. Add Caustics (Generate) to
   another layer and choose the Wave World layer as its Water Surface, raise Caustics Strength,
   and play: the picture should ripple and shimmer like a pool floor.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how strong the waves and the tint look as added, and
that a Bottom layer that has been deleted leaves the effect clear rather than showing the layer.

## Speed

PROVISIONAL, measured while more than thirty other builds ran (`verification/B-511_caustics_timing_table.md`). On the processor at the reference shot's size (1920 by 1080), a frame took about 0.25 to 1.1 seconds with Caustics as added, about 0.7 to 1.9 seconds with water and caustics, and about 1 to 3 seconds with Bottom Blur 50 as well; the same shot with no effect swung from 43 to 685 ms, so these are rough. Caustics has no card path yet, so expect playback below real time at full size; Draft quality or a smaller preview helps. A quiet re-measure would settle the figures.
