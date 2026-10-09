# B-238: Lens Blur's blur map

Built on 2026-10-08 under your effects loop request, decided as D-359. **Lens Blur** (in **Blur &
Sharpen**, our Camera Lens Blur) can now take how blurred each part of the picture is from
another layer, as After Effects' Camera Lens Blur does with its Blur Map group. This is also pick
#12 of the plugin list, "Shaped Bokeh with blur map". Where the map's value equals **Blur Focal
Distance** the picture stays sharp; the further a pixel's map value is from it, the wider the iris
opens, up to the full Radius 255 levels away. The iris shape, highlights and edges work as before.

The new settings, with the values they start at:

- **Blur Map Layer** (None): any layer of the composition. None means Lens Blur works exactly as
  it always did.
- **Blur Map Channel** (Luminance): Luminance reads the map's brightness, Alpha its coverage.
- **Blur Map Placement** (Centre): Centre lays the map at its own size in the middle of the layer;
  Stretch Map to Fit stretches it over the whole layer.
- **Blur Focal Distance** (0): the map value kept sharp, 0 (black) to 255 (white).
- **Invert Blur Map** (Off): On turns the map over, so black is far and white is near.

A depth pass from a 3D render works the After Effects way: put the EXR on a layer of its own with
**Pass Extract** showing its depth, and choose that layer as the map.

The check, `verification/D-359_lens_blur_map_table.md` (148 of 148), holds every pixel to numbers
worked out by a separate program before the code existed. It also holds the graphics card's
picture to within 1 level of the processor's, and the old Lens Blur pictures to exactly what they
were. The pictures are in `verification/D-359 pictures/`, twice enlarged.

## What to check

**`town.png`** is a little street: sky, houses, a road with white markings. **`ramp.png`** is the
blur map used below: black at the top (far), white at the bottom (near). **`before.png`** is the
street with no effect.

1. **No map.** `lens_blur_no_map.png` is Lens Blur radius 6 with no map: everything equally soft.
2. **Focused on the road.** `lens_blur_road_sharp.png` uses the ramp as the map with Blur Focal
   Distance 255 (white). The road markings at the bottom should be nearly sharp, the houses softer,
   the top of the sky softest, as a camera focused on the near ground.
3. **Focused on the houses.** `lens_blur_houses_sharp.png` is Blur Focal Distance 140, the ramp's
   grey at the houses. The windows should be sharper than in `lens_blur_no_map.png`, and the road
   markings softer than in `lens_blur_road_sharp.png`.
4. **In the app.** On the reference shot, put Lens Blur on a layer, set Radius to about 10 and
   choose another layer as Blur Map Layer. Drag Blur Focal Distance from 0 to 255 and watch the
   sharp part move to where the map is darker or lighter. Turn Invert Blur Map on and off. Key
   Blur Focal Distance and play it: a focus pull.
5. **Saved and opened again.** Save the project with the map set, close it and open it: the same
   settings and the same picture. A project saved before this change opens with no map.
6. **A missing map.** Delete the map layer: the Lens Blur layer is drawn without the effect and
   the warnings list says the effect's layer is missing (EFFECT_LAYER_MISSING).

With Draw on: GPU the map adds about 5 ms a 1080p layer at radius 10
(`verification/B-238_lens_blur_map_timing_table.md`).

## Not built

- No **Depth** choice in Blur Map Channel; use Pass Extract on the map layer instead.
- No point to click to set the focus (the Bokeh plugin's focus point); set Blur Focal Distance
  by its number.
- Depth from a 3D camera or from 3D layers in the composition, which After Effects can use, is
  off-charter (D-309, D-355).
