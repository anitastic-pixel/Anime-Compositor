# B-382: Inner/Outer Key

Built on 2026-10-10 as D-502, under your /loop request: Inner/Outer Key, in **Lines & Mattes**
(beside the other keyers), with After Effects' settings. It cuts a subject out of its background
by two masks you draw on the layer: one just inside the subject's edge, one just outside it. What
lies between them, hair and soft edges, is made see-through as much as it is the colour outside,
and that colour is taken back out of it.

- **Foreground (Inside)** (0): the number of the mask round what stays. Until one is chosen
  nothing is keyed.
- **Background (Outside)** (0): the number of the mask round the edge; everything outside it
  goes. With none, the one mask is used alone (next line).
- **Single Mask Highlight Radius** (10): with no background mask, how many pixels either side of
  the one mask are worked out; inside the band stays, outside it goes.
- **Edge Thin** (0): shrinks what is kept by that many pixels, or grows it when below 0.
- **Edge Feather** (0): softens the edge of what is kept.
- **Edge Threshold** (0): the faintest kept pixels go, the rest grow more solid.
- **Invert Extraction** (Off): keeps the outside and keys the subject instead.
- **Blend With Original** (0): brings that share of the picture back.

After Effects' Additional Foreground, Additional Background and its Cleanup brushes are not built
yet. Adobe publishes no formula and no starting values, so the rule and the settings when added
are our own, written down in document 21. The key is worked on the processor for now; the
graphics card draws whatever follows it.

The check, `verification/D-502_inner_outer_key_table.md` (249 of 249), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-502 pictures/`.

## What to check

1. **The street.** `1_before.png`: a street with three masks that cut nothing (a circle in the
   middle, a larger circle round it, a circle to the upper left).
2. **Two masks.** `2_two_masks.png`: the inner circle kept whole, everything outside the outer
   circle gone (the page shows through), the ring between fading out where it is sky-coloured.
3. **One mask.** `3_one_mask.png`: the middle circle alone, radius 10: the disc kept with a soft
   rim, the rest gone.
4. **Inverted.** `4_inverted.png`: the inner disc gone and the street outside the outer circle
   kept.
5. **Thin and feather.** `5_feathered.png`: like 2, the kept part a little smaller and its edge
   soft.
6. **Blended.** `6_blended.png`: the upper-left circle alone with Blend With Original 50: the
   street outside it half see-through.
7. **As added.** With Foreground 0 the picture is unchanged.
8. **In the app.** Draw a mask just inside a subject's edge and another just outside it (set
   both masks' mode to None so they cut nothing themselves), add Inner/Outer Key (Lines &
   Mattes), set Foreground to the inner mask's number and Background to the outer one's. Then try
   one mask alone with the radius, Edge Feather, Edge Thin both ways, Edge Threshold and Invert
   Extraction. Keyframe the inner mask's path and play it: the key follows.
9. **A missing mask.** Set Foreground to 5 on a layer with two masks: the layer is left as it
   is, with a warning naming the missing mask.
10. **Out of range.** Type 300 in Edge Thin: it is refused with a sentence saying the range.
11. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: radius 10 when added, and how the ring between the masks
fades, which you can compare with After Effects' own on the same shot.

## Speed

Measured on a quiet machine
(`verification/B-382_inner_outer_key_timing_table.md`). Three Inner/Outer Keys on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: the shot after a moving Noise 12.0 / 44.4 ms a frame, two masks 240.3 / 242.1, one mask, radius 10 224.9 / 215.9, two masks, thin 3, feather 12, threshold 20 302.2 / 297.5; about 76.1, 71.0 and 96.7 ms a 1080p layer in the card's frame (the key drawn on the processor, the card drawing what follows it), over the row's Target P3 (8 ms); on the processor about 65.9, 57.2 and 84.4 ms a layer.
