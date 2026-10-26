# B-740 playtest: the viewer keeps each layer's finished mask coverage (D-860)

## What changed for you

Until now, the viewer worked out a layer's masks again on every frame, even when the masks
held still. Now it keeps each layer's finished mask and reuses it until something that changes
the mask changes. That covers its points, feather, expansion, mode, inverted, opacity, the
layer's size, the composition's size, Draft or Full, and the frame if the mask is keyed. Every
picture is the same as before, to the bit. Export is not changed.

On the reference shot with 25 layers, the masks' share of a frame drops from 65 ms to 16 ms.
The whole frame goes from 256 to 217 ms at Full and from 104 to 51 ms at Draft. Tutorial 2's
noise polar shot goes from 300 to 184 ms at Draft. These are medians of 5 alternating rounds on
the RTX 4070 Ti SUPER machine, release build, with other lanes running. The full numbers are in
D-860 in document 14. No fixture changed and no saved project changes.

## What to look at

- `verification/B-740_mask_cache_table.md`: every check passes. It covers 14 different edits,
  each drawn by the warm cache and compared with a fresh, uncached picture. It also covers all
  24 fixture folders with masks, and a keyed mask played back and forth.
- `verification/B-740 pictures/edits_sheet.jpg`: four close-ups of the masked red shape from the
  warm cache: before any edit, after a point moved, after feather 20 to 60, and after the second
  mask was set to Intersect. Each should show its edit; none should look like the one before it.
- `verification/B-740 pictures/frame4_whole_frame.jpg`: the whole frame the close-ups come from.

## Try it

1. Open a project with a masked layer whose drawing changes from frame to frame. Play it, or
   scrub, at Draft. It should look exactly as before and move a little more smoothly.
2. Drag a mask point, change Feather, switch the mask's mode, or tick Inverted, then let go.
   The viewer should show the change at once, never the old mask.
3. Press Ctrl+Z. The old mask should come back at once.
4. Switch between Draft and Full. Each should look as it always did.
