# B-226: the last four effects drawn by the graphics card

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is the
sixth and last part of P0-2, the card's effect pipeline, decided as D-346.

Block Dissolve, Gradient Wipe, Line Smoothing and Line Width were drawn by the processor until
now, and a layer with one was drawn by the processor from that effect on. Now the graphics card
draws them in the viewer, each by the processor's own rule. Block Dissolve's random blocks are
still chosen by the processor and handed to the card, so they fall exactly as before. Gradient
Wipe's gradient layer is handed to the card as a picture, as Displacement Map's is. Line
Smoothing and Line Width decide which pixels to touch by exact comparisons, so the card starts
its work at them, with the processor's drawing. Exports never use the graphics card and are
untouched.

With this, P0-2 is done. What still draws on the processor does so by a decision or by design:
Kaleidoscope (your D-240), Echo and Posterize Time (they need several frames), Radio Waves (D-345,
still waiting for your choice), and Unsharp Mask's threshold (D-317).

The check, `verification/B-226_gpu_last_four_table.md`, draws every fixture of the four and the
reference shot with each, on both: 940 of 940 within 1 level of 255. The pictures for the worst
frame are in `verification/B-226 pictures/`. The timings are in
`verification/B-226_gpu_last_four_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Block Dissolve.** On one layer add Block Dissolve. Drag **Transition Completion** from 0 to
   100: blocks should vanish one by one in the same places every time, and the layer should be
   gone at 100. Change **Block Width** and **Block Height**, then raise **Feather**: the blocks'
   edges should soften evenly, with no hard lines along the block borders.
2. **Gradient Wipe.** Add Gradient Wipe and pick another layer as **Gradient Layer**. Drag
   **Transition Completion**: the layer should disappear from the darkest parts of that layer to
   the brightest. Raise **Transition Softness**, turn **Invert Gradient** on, and try each
   **Gradient Placement**.
3. **Line Smoothing.** Add Line Smoothing. Zoom in on a drawn line and raise **Softness**: the
   stair-steps along the line should smooth out, and nothing else should change. Raise
   **Threshold**: fewer steps should be smoothed.
4. **Line Width.** Add Line Width. Set **Width** to 3, then to -2: the lines should grow thicker,
   then thinner, evenly all round. Set **Width Of** to Chosen colours, pick a line colour and
   raise **Tolerance**: only lines of that colour should change.
5. **Same as an export.** Export a frame (or render it to a still) and put it next to the viewer.
   The two should look the same.
6. **Smooth.** Play the shot. It should play at least as smoothly as before.
7. **Draft.** Switch to **Draft** and repeat steps 1, 3 and 4.

## If something is wrong

Say which step and which effect. The most likely fault would be in Line Smoothing (step 3): a
smoothed step that looks different from the export, since the card follows each line's steps in
the processor's order; the check compares that on every fixture.
