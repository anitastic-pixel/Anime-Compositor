# B-223: five blurs drawn by the graphics card

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is the
third part of P0-2, the card's effect pipeline, decided as D-342.

Fast Box Blur, Channel Blur, Compound Blur, Selective Color Blur and CC Vector Blur were drawn by
the processor until now, and a layer with one was drawn by the processor from that effect on. Now
the graphics card draws them in the viewer, each by the processor's own rule. Compound Blur and CC
Vector Blur can take another layer as a map; the card is handed that layer as a picture of its
own. Exports never use the graphics card and are untouched.

One limit, on purpose: Selective Color Blur picks the pixels to blur by rounding their colour to 8
bits. Given anything but the processor's exact picture it could pick differently, as HSV Key could
(D-224), so on a layer it only starts a row on the card: an effect before it stays on the
processor.

The check, `verification/B-223_gpu_blurs_table.md`, draws every fixture of the five and the
reference shot with each, on both. The pictures for the worst frame are in
`verification/B-223 pictures/`. The timings are in `verification/B-223_gpu_blurs_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Fast Box Blur.** On one layer add Fast Box Blur. Drag **Blur Radius** up and down, then set
   **Iterations** to 1 and to 5. The blur should grow and shrink smoothly, softer with more
   iterations, with no flicker or blocky steps.
2. **Channel Blur.** Add Channel Blur and give **Red Blurriness** a large value and the others 0.
   Only the red should smear, leaving a red fringe.
3. **Compound Blur.** Add Compound Blur and pick another layer as **Blur Layer**. The layer should
   blur more where the map layer is bright. Move the map layer: the blur should follow.
4. **CC Vector Blur.** Add CC Vector Blur, raise **Amount**, then pick another layer as
   **Vector Map**. The smear should follow the map's shapes.
5. **Selective Color Blur.** Add Selective Color Blur and pick a colour from the layer. Only that
   colour should blur.
6. **Same as an export.** Export a frame (or render it to a still) and put it next to the viewer.
   The two should look the same.
7. **Smooth.** Play the shot. It should play at least as smoothly as before.
8. **Draft.** Switch to **Draft** and repeat steps 1 and 3.

## If something is wrong

Say which step and which effect. The most likely fault would be a map (steps 3 and 4) that looks
shifted against the export, since the card is handed the map layer separately; the check compares
exactly that case.
