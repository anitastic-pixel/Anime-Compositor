# B-298: Eyedropper Fill

Built on 2026-10-10 as D-419, under your /loop request: After Effects' Eyedropper Fill, in
**Generate**. It fills the whole layer with one colour picked up from the layer itself, like an
eyedropper: from one spot, or the average of a small circle round it.

- **Sample Point** (the middle): where the colour is taken, as a per cent of the layer's width
  and height.
- **Sample Radius** (0): how far round the point to average, in pixels. 0 takes just the one
  pixel under the point.
- **Average Pixel Colors** (Skip Empty): how see-through pixels in the circle count.
  - *Skip Empty*: they are left out; the colour is the average of the pixels that show.
  - *All*: they count as black, so a circle half empty gives a darker colour.
  - *All Premultiplied*: each pixel counts by how solid it is.
  - *Including Alpha*: the fill itself is as see-through as the circle is on average.
- **Maintain Original Alpha** (Off): On keeps the layer's own shape; the fill only covers what
  was already there. Off fills the whole rectangle, the empty parts too.
- **Blend With Original** (0): how much of the original layer shows through. 100 shows only the
  original.

No formula for this effect could be found (Adobe's page could not be read here), so how each
pixel is worked out is our own rule, written down in document 21. A draft draws everything at
half size and halves the radius, so it averages the same area.

The check, `verification/D-419_eyedropper_fill_table.md` (212 of 212), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-419 pictures/`
(`1_before.png` is the street with nothing on it).

## What to check

1. **As added.** `2_as_added.png`: the whole street one flat blue, the colour of the building at
   its middle.
2. **Radius 60.** `3_radius_60.png`: one flat colour again, a greyer blue: the average of a circle
   round the middle (the building, its windows and its neighbours).
3. **A corner, alpha kept, blended.** `4_upper_left_kept.png`: the sky's colour from the top left
   laid over the street at 60 per cent, the street faintly showing through.
4. **In the app.** Add Eyedropper Fill (Generate) to a layer: it turns one colour. Drag Sample
   Point about: the colour follows what is under it. Raise Sample Radius: the colour becomes an
   average and changes more smoothly as you drag. On a layer with see-through parts, try the four
   Average Pixel Colors with the circle half over an empty area: Skip Empty ignores the empty
   part, All darkens, Including Alpha makes the fill see-through. Turn Maintain Original Alpha on:
   the fill takes the layer's shape. Key Blend With Original from 0 to 100 across a few seconds
   and play: the original fades back in. Switch to Draft: the same picture.
5. **Out of range.** Type 1001 in Sample Point: it is refused with a sentence saying it runs from
   -1000 to 1000.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

Measured only provisionally so far: other lanes' builds and tests were running at the time and
even the Noise alone took over five times what it took a few hours earlier
(`verification/B-298_eyedropper_fill_timing_table.md`), so these numbers say little yet. The
work itself is small: the colour is averaged once a frame, then every pixel is set to it.
