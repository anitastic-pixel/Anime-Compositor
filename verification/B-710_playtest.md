# B-710 playtest: effects along an adjustment layer's own masks (D-830)

What changed: an effect that draws along a mask (Stroke, Vegas, Scribble, Energy Stroke, Audio
Spectrum and Audio Waveform on a Path, Inner/Outer Key, Reshape, Mask Smear, Liquify's Freeze
Area, Particle Playground's Wall, and Fill naming a mask) now works on an adjustment layer. It uses
the adjustment layer's own masks and draws on everything beneath, as in After Effects. Before, it
drew nothing and the frame notes said EFFECT_PATH_MISSING. Every other project looks as before.

## What the picture shows

`B-710 pictures/adjustment_masks_before_after.png`: the smoke rebuild from B-708 with an
adjustment layer on top. The layer has two masks set to mode None (paths only): an ellipse, with
an orange Stroke along it, and a rectangle, with a yellow Scribble inside it.

Left, before (the B-709 build), frame 7: no outline and no hatching. Orange covers 0.00% of the
frame and yellow 0.00%. The frame notes said EFFECT_PATH_MISSING. Right, after: the orange ellipse
outline is drawn over the smoke, and the yellow diagonal hatching fills the rectangle. Orange covers
1.68% of the frame and yellow 2.62%. Correct if the outline and the hatching appear where the two
masks are and the smoke beneath is unchanged.

No After Effects still is shown. The two After Effects sources (AE Juice's stroke-mask guide and an
Adobe Community answer) are written steps, not pictures, so this sheet is before and after only.

The checks: `B-710_adjustment_masks_table.md`, 52 of 52 pass. Each of the 15 effect settings on
an adjustment layer draws exactly what the same effect draws on the layer beneath, with the same
masks. Scaling or moving the adjustment layer moves its masks too. The graphics card matches the
CPU (0 levels apart).

## Try it yourself (3 minutes)

1. Open any project and add Layer > New > Adjustment Layer on top.
2. With the adjustment layer selected, draw an ellipse mask and set its mode to None.
3. Add Effect > Generate > Stroke. Mask 1 is chosen. Expected: an outline along the ellipse, over
   everything beneath. Before this build nothing showed and the frame notes said
   EFFECT_PATH_MISSING.
4. Move or scale the adjustment layer. Expected: the outline follows the mask.
5. Set the mask's mode back to Add. Expected: the inside half of the line shows and the outside half
   is cut away, because an Add mask also limits where the adjustment layer applies.
6. Set Stroke's Path to Mask 2. Expected: nothing is drawn, and the frame notes say
   EFFECT_PATH_MISSING (there is no Mask 2).
