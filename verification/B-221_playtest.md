# B-221: effects mixed below 100 drawn by the graphics card

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is the
first part of P0-2, the card's effect pipeline, decided as D-340.

Every effect has a **Mix** setting (D-202): below 100 it lays the effect's result back over the
picture it was given. Until now, a layer with any effect mixed below 100 was drawn by the
processor from that effect on, so the viewer slowed down. Now the graphics card does the mixing
too, in the same order and with the same sum the processor uses. Exports never use the graphics
card and are untouched.

The check, `verification/B-221_gpu_mix_table.md`, draws every effect the card can draw at Mix 0,
37 and 100 on both, plus mixed effects in a row of colour effects, mixed blurs and glows that grow
the drawing, and an adjustment layer with mixed effects. The pictures for the worst frame are in
`verification/B-221 pictures/`. The timings are in `verification/B-221_gpu_mix_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Glow at Mix 50.** On one layer, add a Glow strong enough to see, and set its **Mix** to 50.
   The glow should look half as strong, with the layer underneath still visible through it.
2. **Same as an export.** Export that frame (or render it to a still) and put it next to the
   viewer. The two should look the same.
3. **Smooth.** Play the shot. It should play as smoothly as it does with the Glow at Mix 100.
4. **Drag the Mix.** Drag the Glow's Mix from 0 to 100 and back. The glow should fade in and out
   following the slider, with no jump at either end and nothing flashing.
5. **In a row of colour effects.** Add Levels, Hue/Saturation and Curves after the Glow, and set
   Hue/Saturation's Mix to 37. Drag its Hue. The picture should follow.
6. **Draft.** Switch to **Draft** and repeat step 4.

## If something is wrong

Say which step. The most likely fault would be a glow or blur whose mixed result looks shifted
against the layer (steps 1 and 2), because a glow grows the drawing and the two pictures have to
be lined up; the check compares exactly that case.
