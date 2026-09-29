# B-125: a layer of this composition as an effect's setting, by hand

Built on 2026-09-28 against D-189, which your "take everything" accepted with the After Effects
picks. Compound Blur, Displacement Map and Gradient Wipe (A2, A3, A4) each read another layer's
picture. This unit is the part they share: given the layer the effect is on and the layer its
setting names, the picture the effect reads, called the map.

## There is nothing to try in the app yet

No effect has a layer setting until Compound Blur (A2), which brings the drop-down on its card.
Nothing you can open, click or play is any different today, and nothing is meant to be. The
steps below are for A2's sheet; this one is only for looking at pictures and a table.

## What to look at

`verification/B-125b_layer_map_table.md` is the check. Each of document 25's 37 cases, FX-LMAP-001
to 042, asks the program for a map and compares every pixel with the numbers
`tools/effect_layer_reference.py` worked out on its own before the program had this code.
**37 of 37 checks pass.**

`verification/B-125 pictures/maps.png` is every map the program made, in the table's order, six to
a row, each pixel drawn 16 times bigger, on a grey checkerboard where the map is clear. Beside the
table, it should look like this:

1. **Row 1 and the start of row 2, the blue-to-white card.** The same card, in the middle of the
   holder's 8 by 6, whether the card layer is moved, turned, faded, switched off or matte-only
   (FX-LMAP-001 to 003): where a layer sits makes no difference to its map. The fourth has its
   right side cut off by a mask (004); the fifth is brighter, from its Exposure (005); the sixth,
   its Exposure switched off, is the plain card again (006). The first of row 2 is softer, its
   blur cut back to the card's own size (007).
2. **The small one in row 2 (008).** An effect naming its own layer reads the drawing without its
   effects, so it is the card at its own size.
3. **The red-to-yellow cards (009, 012).** The card's second drawing, exposed at that frame by the
   layer's own timing. Between them, a layer not yet in at frame 0 gives a clear map (010), and
   the same layer a frame later shows its first drawing, the blue card (011).
4. **Row 3.** A solid's colour, a shape layer's shape, and a composition layer's inner frame
   plain and brightened (013 to 016); then two clear maps: a frame after the inner composition's
   two frames are over (017), and a composition that is missing (018).
5. **Row 4.** A null gives a clear map (019). The white half is an adjustment layer: white
   through its mask (020). Then a drawing whose file is missing, clear (021); the big
   gradient cut to its middle (022); the card tiled (023); and the big gradient tiled, which
   comes out as its middle again (024).
6. **Row 5.** Stretch: the card and the big picture squeezed or stretched to fill, never tinted
   at the edges (025, 026); the card stretched to its own size, exactly itself (027); a small
   holder, centred and tiled (028, 029); the masked card stretched (030).
7. **Row 6, the Draft maps.** A quarter of the size each way where the holder's own effects run
   at a quarter (031, 033, 034), and full size on a solid, whose effects run at full size (032).

Three names give no map and no picture: a deleted layer's and a layer of another composition,
which say `EFFECT_LAYER_MISSING` (040, 041), and no name at all, which says nothing (042).

## Known limits, on purpose

- No card version. The effects that read a map will run on the processor until a card unit of
  their own.
- The drop-down, what happens to the setting when a layer is duplicated or put in a preset, and
  `EFFECT_LAYER_CYCLE` for effects whose settings lead round in a circle, come with A2.

## What to report

"looks right", or which numbered picture looks wrong and how. There is nothing to time: no frame
you can play draws differently.
