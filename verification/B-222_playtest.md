# B-222: ten colour effects drawn by the graphics card

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is the
second part of P0-2, the card's effect pipeline, decided as D-341.

Exposure, Tint, Shift Channels, Solid Composite, Change to Color, Color Key, Select Color, Line
Recolor, Colorama and Extract each change a pixel using only that pixel. Until now the processor
drew them, and a layer with one was drawn by the processor from that effect on. Now the graphics
card draws them in the viewer, and draws a row of them with other colour effects in one go
(B-172). Exports never use the graphics card and are untouched.

Two limits, both on purpose:

- Color Key, Select Color and Line Recolor decide pixel by pixel whether a colour is "the" colour,
  rounding it to 8 bits first. Given anything but the processor's exact picture they could decide
  differently, as HSV Key could (D-224), so on a layer they only start a row on the card: an
  effect before them stays on the processor.
- A Colorama that adds another layer's brightness (its **Layer** setting) stays on the processor,
  since it reads pixels that are not its own.

The check, `verification/B-222_gpu_colour_table.md`, draws every fixture of the ten and the
reference shot with each, alone and in a row with neighbours, on both. The pictures for the
worst frame are in `verification/B-222 pictures/`. The timings are in
`verification/B-222_gpu_colour_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Exposure.** On one layer add Exposure. Drag **Exposure** up and down, then set **Offset**
   and **Gamma** away from 0 and 1. The layer should brighten and darken smoothly, with no
   flicker or blocky steps.
2. **Color Key.** Add a Color Key and pick a colour from the layer. That colour should vanish,
   and the edge should soften as you raise **Softness**.
3. **A row.** After the Color Key add Exposure and Tint, then drag Tint's **Amount**. The picture
   should follow the slider at once.
4. **Same as an export.** Export that frame (or render it to a still) and put it next to the
   viewer. The two should look the same.
5. **Smooth.** Play the shot. It should play at least as smoothly as before.
6. **The rest.** Try Shift Channels, Solid Composite, Change to Color, Select Color, Line Recolor,
   Colorama and Extract one at a time. Each should look the same as an export of the frame.
7. **Draft.** Switch to **Draft** and repeat step 3.

## If something is wrong

Say which step and which effect. The most likely fault would be a keyed edge (steps 2 and 6) that
looks different from the export, since those effects choose pixels by an exact colour; the check
compares exactly that case.
