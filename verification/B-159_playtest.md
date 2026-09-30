# B-159: the layers below an edit kept

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 10 of the GPU plan (G10), which you added to the queue "to your discretion": D-241.

When you change one layer (drag its opacity, move it, and so on) and the processor draws the
viewer, the window now keeps the picture of the layers below that layer, and draws only that layer
and the ones above it on top of it again. **The pixels are byte for byte what they were**:
`verification/B-159_below_table.md` edits every layer of the reference shot three ways, in five
versions of the shot, and compares all 544 pictures with ones drawn from scratch; all are
identical. Exports never use it.

On the declared ten-layer shot at Full, editing the top layer went from 24.5 to 7.6 ms a frame
(`verification/B-159_timing_table.md`). Editing a layer near the bottom saves little, and editing
the bottom layer saves nothing. When the graphics card draws the viewer, nothing changes.

## What to check

Open the reference shot and set **Draw on: CPU**, then **Full resolution**. Stay on one frame.

1. **Drag an opacity.** Pick the top layer and drag its Opacity slowly from 100 down to 0 and back.
   The layer should fade smoothly, and the layers under it should stay exactly as they were: no
   flicker, no brief flash of a different picture, no layer going missing.
2. **Move a layer.** Drag the same layer's Position about. It should follow the hand, a little
   more readily than before, and the picture beneath should not change.
3. **Change layers.** Do the same to a layer lower down, then to the top one again. Each should
   look right straight away.
4. **Undo.** Press Ctrl+Z a few times. Each step back should show the picture as it was at that
   step, fully drawn.
5. **Step a frame.** Press the right arrow once, then the left. Both frames should look as they did
   before any of this.
6. **The adjustment layer.** If your shot has an adjustment layer, drag the opacity of a layer below
   it and one above it. Both should look right. (Here the saving is smaller: the adjustment layer is
   drawn again each time.)

## If something is wrong

Say which step and which layer. The most likely fault would be a layer below the one you edited
showing an old state of itself (for instance after step 3 or 4); the byte checks cover those
cases, so it would point at something the checks do not reach.
