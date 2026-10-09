# B-258: Blobbylize

Built on 2026-10-09 under your effects loop request, decided as D-379. **Blobbylize** (in
**Distort**) is our version of CycoreFX's CC Blobbylize: the layer cut out by a soft, round blob
shape read from one channel of a layer, its own or another, and lit so it looks like drops of
liquid metal, slime or wax. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Blob Layer** (none, the layer itself), **Blob Layer Placement** (Stretch to Fit) and
  **Property** (Alpha): where the blob shape comes from. Property picks red, green, blue,
  alpha, luminance or lightness. Another layer can be stretched, centred or tiled.
- **Softness** (10): pixels, 0 to 100, how round and spread the blobs are.
- **Cut Away** (0): 0 to 100, how much of the thin blob edges is cut off; 100 cuts everything.
- **Light Intensity** (100), **Light Color** (white), **Light Type** (Distant Light),
  **Light Height** (100), **Light Position** (30, 30, for a point light) and **Light Direction**
  (-45, clockwise from up, for a distant light): the effect's own light. After Effects' light
  layers are not used.
- **Ambient** (25), **Diffuse** (75), **Specular** (50), **Roughness** (0.05) and **Metal**
  (100): how the surface takes the light. Specular is the shine, Roughness small for a tight
  sharp shine, Metal how much the shine takes the layer's own colour rather than the light's.

A Blob Layer that is not in the composition warns EFFECT_LAYER_MISSING and the layer itself is
used.

The check, `verification/D-379_blobbylize_table.md` (190 of 190), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-379 pictures/`. They are
saved over white, so see-through parts show pale, and fully cut parts white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street, solid, lit nearly flat; only its outer edge
   rounds off into a soft, slightly see-through rim, brighter at the top left where the light
   is.
2. **Shiny bumps.** `3_luminance_shiny.png`, Property Luminance, Softness 6: the bright windows
   and road markings stand up as shiny bumps; the darker street turns half see-through, so it
   looks pale over white.
3. **Melted away.** `4_luminance_cut_50.png`, Luminance, Softness 4, Cut Away 50, a warm point
   light at 70, 30, height 60: the dark parts are gone, the bright windows and walls are left
   as warm, glossy blobs, with the light's hot spot in the sky at the top right.
4. **In the app.** Put Blobbylize on a text or a shape layer and key Softness from 0 to 30: the
   letters melt into round, glossy blobs. Key Cut Away from 0 to 80: the blobs shrink and break
   apart. Move Light Direction with its dial: the shine moves round the blobs.
5. **Another layer.** Pick a different layer as the Blob Layer: the blob shape comes from it,
   the colour from this layer. Switch the other layer off: the blob shape still comes from it.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Blobbylize costs about 2.0 to 2.2 ms a 1080p layer, and about 33 ms on the processor (`verification/B-256_distort_timing_table.md`; provisional, measured while other tests ran).

## Not built

- The blob, the cut and the lighting are our own reading of CycoreFX's one-sentence
  descriptions; no tutorial with numbers was matched against them.
- Only the effect's own distant or point light; After Effects' light layers are not offered.
- The layer does not grow to hold blobs spreading past its edges.
- CycoreFX's blend with the original is the Mix every effect has.
