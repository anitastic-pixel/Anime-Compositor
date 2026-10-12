# B-461: Curl Noise, Source This Layer and Other Layer

Built on 2026-10-12 as D-583, under your /loop request: Curl Noise (Noise & Grain) can now take its
noise **from a picture** instead of making it. After Effects calls this **Source**: Internal (the
made noise, as before), **This Layer** (the layer's own picture) or **Other Layer** (another
layer's picture). The picture's lightness becomes the noise, so the flow lines run along its
edges, the way the "Starry Night" tutorial swirls a painting.

- **Source**: Internal, This Layer or Other Layer.
- **Source Layer** (Other Layer only): which layer to read.
- **Softness** 0 to 100: blurs the picture first, so the lines follow only its large shapes.
- **Source Mapping** (Other Layer only): Fill stretches the other layer over this one; Center keeps
  its own size in the middle.
- **Mirror at Edge**: off, beyond the picture is black; on, the picture is mirrored back in.

Not built: the Transform's Rotation, Scale and Perspective (as before). Adobe publishes no formula,
so how lightness becomes flow is ours, and our look is not After Effects' pixel for pixel.

**One honest note.** One check, FX-CURL-033, was first written with Curl Noise's starting settings
and our build missed it at one pixel by a hair (0.0001 against the 0.00002 allowed): the build
reads the picture in slightly less exact numbers than the checking program, and 30-pixel flow
lines over sharp edges magnify that. By the coordinator's ruling it now uses small lines, and the
starting settings are instead held card against processor within 1 level on the reference shot.

The check, `verification/D-583_curl_sources_table.md` (138 of 138), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-583 pictures/`.

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **This Layer.** `2_this_layer.png`: grey flow lines that run along the street's own edges
   (buildings, windows, road), not the even swirls of the made noise.
3. **The picture it reads.** `3_this_layer_input_noise.png` (View Input Noise): the street in grey,
   its own lightness.
4. **Softness.** `4_this_layer_softness_40.png`: the lines follow only the street's large shapes.
5. **Other Layer.** `5_rings_curl_generation.png` (View Curl Generation): another layer of rings
   read as the noise, the flow as red and green running round them; `6_rings_flow.png`: grey
   flow lines in circles round the middle.
6. **In the app.** Add Curl Noise to a picture and set Source to This Layer: the lines follow the
   picture. Raise Softness. Set Source to Other Layer and pick a layer under Source Layer; switch
   Source Mapping between Fill and Center; turn Mirror at Edge on with an Offset and watch the
   edge.
7. **Missing layer.** Delete the layer Other Layer reads: the frame draws without the effect, with
   a warning.
8. **Undo, save and open.** Change Source, Undo; save, close and open: the same settings and picture.

## Speed

In `verification/B-461_curl_sources_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Curl Noise on This Layer on each of three layers, played again, 147.0 ms a frame
on the card against 133.5 for the Noise alone, and 79.1 with Softness 50; on the
processor 286.2 against 92.8: a busy machine cannot say whether the card is quicker than the processor for this effect.
