# B-297: Fill

Built on 2026-10-10 as D-418, under your /loop request: After Effects' Fill, in **Generate**.
It paints the layer, or just the inside of its masks, one colour. See-through parts of the layer
stay see-through.

- **Fill Mask** (0): which mask to fill, by number. 0 means none: the whole layer is filled.
- **All Masks** (Off): On fills every mask on the layer, whichever number Fill Mask says.
- **Color** (red).
- **Invert** (Off): On fills everything except the mask.
- **Horizontal Feather**, **Vertical Feather** (0, 0): soften the fill's edge left and right,
  and up and down.
- **Opacity** (100): how strongly the colour covers.

The masks it reads are any masks on the layer that are switched on, even ones set to None that
cut nothing, just as Stroke reads them. If Fill Mask names a mask that is not there or is
switched off, nothing is filled and the app says so.

Adobe's page names the controls and gives no formula, so how each pixel is worked out is our own
rule, written down in document 21. A draft draws everything at half size, the same fill a little
rougher.

The check, `verification/D-418_fill_table.md` (274 of 274), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-418 pictures/`
(`1_before.png` is the street with nothing on it; the street layer has two masks set to None, a
big circle in the middle and a smaller one top left, which on their own change nothing).

## What to check

1. **As added.** `2_as_added.png`: the whole street solid red.
2. **One mask.** `3_mask_orange.png`: a hard-edged orange disc in the middle, the rest the street.
3. **Feathered.** `4_mask_feathered.png`: the same disc with a soft, blurred edge.
4. **Inverted.** `5_inverted_dark.png`: everything outside the disc darkened, the disc itself
   left as the street.
5. **All Masks.** `6_all_masks_blue.png`: both discs blue and slightly see-through, their edges
   soft left and right but sharp top and bottom.
6. **In the app.** Add Fill (Generate) to a layer: it turns red. Draw a mask on it, set its mode
   to None, and set Fill Mask to 1: only the inside of the mask is red. Raise the feathers: the
   edge softens. Turn Invert on: the outside fills instead. Draw a second mask and turn All
   Masks on: both fill. Key Opacity across a few seconds and play: the colour fades in. Switch
   to Draft: the same picture.
7. **Missing mask.** Set Fill Mask to 3 on a layer with one mask: the layer is left as it is and
   the app warns that there is no such mask to fill.
8. **Out of range.** Type 1001 in Fill Mask: it is refused with a sentence saying it runs from 0
   to 1000.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## The two-point mask (decided)

Two of the checks fill by a "mask" of only two points, which encloses nothing. The pixels come
out exactly as expected, and the app, as it does for every such mask anywhere, also warns that
the mask cannot be drawn. You decided on 2026-10-10 that those two checks expect that warning,
so they now pass like every other check. No picture changed.

## Speed

Measured only provisionally so far: other lanes' tests were running at the time
(`verification/B-297_fill_timing_table.md`). On the test shot (three 1920 by 1080 layers),
filling the whole layer cost too little to see on the graphics card. Filling by masks costs
more, roughly 7 to 20 ms a layer a frame, because the masks' shapes are worked out on the
processor before the card feathers and paints them.
