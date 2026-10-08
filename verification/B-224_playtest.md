# B-224: Displacement Map and CC Glass drawn by the graphics card

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is the
fourth part of P0-2, the card's effect pipeline, decided as D-343.

Displacement Map and CC Glass were drawn by the processor until now, and a layer with one was
drawn by the processor from that effect on. Now the graphics card draws them in the viewer, each
by the processor's own rule. Both can take another layer as a map; the card is handed that layer
as a picture of its own, as B-223 does for Compound Blur. Exports never use the graphics card and
are untouched.

Found and fixed on the way: since B-223, the card's one-pass colour runs (B-172) could not be
built, so a layer with two or more colour effects in a row was drawn by the processor, saying
GPU_PREVIEW_ON_CPU. One line in B-223's Selective Color Blur pass caused it; the B-155 check is
whole again.

The check, `verification/B-224_gpu_maps_table.md`, draws every fixture of the two and the
reference shot with each, on both. The pictures for the worst frame are in
`verification/B-224 pictures/`. The timings are in `verification/B-224_gpu_maps_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Displacement Map.** On one layer add Displacement Map and pick another layer as
   **Displacement Map Layer**. Drag **Max Horizontal Displacement** and **Max Vertical
   Displacement** up and down. The layer should bend smoothly where the map is bright or dark,
   with no flicker or blocky steps.
2. **Displacement Map, edges.** Turn **Wrap Pixels Around** on: what is pushed off one side should
   come back on the other. Turn it off and **Expand Output** on: the push should carry past the
   layer's edge instead of being cut off.
3. **Displacement Map, map fit.** Switch **Displacement Map Behavior** between Centre Map,
   Stretch Map to Fit and Tile Map. The bend should move with the map.
4. **CC Glass.** Add CC Glass with no bump layer. Raise **Height** and **Softness**: the layer
   should look like raised glass, lit from **Light Direction**. Change **Light Color** and **Light
   Intensity**: the lit side should take that colour.
5. **CC Glass, displacement and a bump layer.** Raise **Displacement**: the picture should bend
   through the glass. Pick another layer as **Bump Map**: the relief should follow its shapes.
6. **A colour run.** On one layer add Levels, then Hue/Saturation, then Tint. The viewer should
   not say it is drawing on the processor (the fault fixed above).
7. **Same as an export.** Export a frame (or render it to a still) and put it next to the viewer.
   The two should look the same.
8. **Smooth.** Play the shot. It should play at least as smoothly as before.
9. **Draft.** Switch to **Draft** and repeat steps 1 and 4.

## If something is wrong

Say which step and which effect. The most likely fault would be a map (steps 1, 3 and 5) that
looks shifted against the export, since the card is handed the map layer separately; the check
compares exactly that case.
