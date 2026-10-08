# B-225: five generators drawn by the graphics card

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is the
fifth part of P0-2, the card's effect pipeline, decided as D-344.

Beam, 4-Color Gradient, CC Light Sweep, Advanced Lightning and Bevel Edges were drawn by the
processor until now, and a layer with one was drawn by the processor from that effect on. Now the
graphics card draws them in the viewer, each by the processor's own rule. Advanced Lightning's
path, with all its random choices, is still worked out by the processor and handed to the card,
so a bolt looks exactly as it did. Exports never use the graphics card and are untouched.

Radio Waves stays on the processor. On the card it differed only in a few fully see-through
pixels of one fixture, which no one can see; whether to accept that is your choice, D-345 (the
same question as Kaleidoscope's D-240).

The check, `verification/B-225_gpu_generators_table.md`, draws every fixture of the five and the
reference shot with each, on both. The pictures for the worst frame are in
`verification/B-225 pictures/`. The timings are in
`verification/B-225_gpu_generators_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Beam.** On one layer add Beam. Drag **Start Point** and **End Point**, then **Length** and
   **Time**. The beam should move smoothly between the points, with no flicker. Raise
   **Softness**: its edge should blur evenly. Turn **Composite On Original** off: only the beam
   should be left.
2. **4-Color Gradient.** Add 4-Color Gradient. Drag the four points and change **Blend**: the
   colours should flow smoothly with no bands or steps. Try each **Blending Mode** and lower
   **Opacity**: the layer should show through as before.
3. **CC Light Sweep.** Add CC Light Sweep. Drag **Center** and turn **Direction**: the band of
   light should slide across the layer. Try each **Shape** (Linear, Smooth, Sharp) and each
   **Light Reception** (Add, Composite, Cutout). Raise **Edge Intensity** and
   **Edge Thickness**: the layer's edges inside the band should light up.
4. **Advanced Lightning.** Add Advanced Lightning. Play the shot: the bolt should flicker and
   branch exactly as it did before. Try a few **Lightning Type** choices, raise **Glow**, and set
   **Core Edge** to Soft. Turn **Composite on Original** off: only the bolt should be left.
5. **Advanced Lightning with an obstacle.** Raise **Alpha Obstacle** with **At an Obstacle** set
   to Go Round: the bolt should still go round the layer's shapes.
6. **Bevel Edges.** Add Bevel Edges. Change **Edge Thickness** and **Light Angle**: the four
   chiselled edges should catch the light from that side.
7. **Same as an export.** Export a frame (or render it to a still) and put it next to the viewer.
   The two should look the same.
8. **Smooth.** Play the shot. It should play at least as smoothly as before.
9. **Draft.** Switch to **Draft** and repeat steps 1, 3 and 4.

## If something is wrong

Say which step and which effect. The most likely fault would be a bolt (steps 4 and 5) whose
glow looks cut off in a straight line, since the card lights each pixel only from the parts of
the bolt whose reach covers it; the check compares exactly that case.
