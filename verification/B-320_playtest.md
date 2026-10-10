# B-320: Paint Bucket

Built on 2026-10-10 as D-440, under your /loop request: After Effects' Paint Bucket, in
**Generate**. It works like a paint program's bucket: it fills the patch of similar colour round a
point with a colour, leaving the rest of the layer alone.

- **Fill Point** (the middle): where the bucket is poured, as a per cent of the layer's width and
  height.
- **Fill Selector** (Color & Alpha): what counts as "similar".
  - *Color & Alpha*: the same colour and see-through-ness.
  - *Straight Color*: the same colour, however see-through; it fills only where the layer shows.
  - *Transparency*: the see-through part (pour it in an empty place).
  - *Opacity*: the solid part (pour it on something solid).
  - *Alpha Channel*: every place as see-through as the point, anywhere in the layer, joined or not.
- **Tolerance** (10): how different a colour may be and still be filled. 0 fills only the exact
  colour; 100 fills everything.
- **View Threshold** (Off): On shows what matches in white, the rest black, to help set Tolerance.
- **Stroke** (Antialias): the fill's edge.
  - *Antialias*: a slightly softened edge.
  - *Feather*: a soft edge, as soft as **Feather Softness** (10).
  - *Spread*: the fill grown outward by **Spread Radius** (3) pixels, over the outline.
  - *Choke*: the fill shrunk by Spread Radius.
  - *Stroke*: only a band **Stroke Width** (3) pixels wide just inside the edge of the area.
- **Invert Fill** (Off): fills everything except the area.
- **Color** (red), **Opacity** (100) and **Blending Mode** (Normal): how the colour goes on.
  *Fill Only* shows the fill alone, the rest of the layer gone.

No formula for this effect could be found, so how each pixel is worked out is our own rule,
written down in document 21. After Effects offers more blending modes than these six (Darken,
Lighten, Difference and others); a project asking for one is refused with a sentence rather than
drawn some other way. A draft draws everything at half size and halves the three distances.

The check, `verification/D-440_paint_bucket_table.md` (282 of 282), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-440 pictures/`
(`1_before.png` is the street with nothing on it).

## What to check

1. **As added.** `2_as_added.png`: the building in the middle of the street turned bright red; its
   windows, the buildings beside it, the sky and the road unchanged.
2. **The road, feathered, blue, multiply.** `3_road_feather_blue.png`: the road tinted dark blue,
   the white dashes left white but slightly glowing at their edges, the sky and buildings
   unchanged.
3. **View Threshold.** `4_sky_threshold.png`: only black and white: the sky (poured at the top,
   tolerance 20) white, everything else black.
4. **Stroke.** `5_sky_stroke.png`: a thin white line running round the edge of the sky: along the
   rooftops and round the picture's border.
5. **In the app.** Add Paint Bucket (Generate) to a layer: the patch under the middle turns red.
   Drag Fill Point about: the fill jumps to whatever patch is under it. Raise Tolerance: the fill
   creeps into neighbouring colours. Turn View Threshold on while adjusting Tolerance, then off.
   Try each Stroke: Feather softens, Spread grows, Choke shrinks, Stroke leaves an outline. Turn
   Invert Fill on: everything but the patch is filled. Try Fill Only. Key Fill Point across a few
   seconds and play: the fill moves from patch to patch. Switch to Draft: the same picture.
6. **Out of range.** Type 101 in Tolerance: it is refused with a sentence saying it runs from 0 to
   100.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

Measured only provisionally: other lanes' builds and tests were running at the time
(`verification/B-320_paint_bucket_timing_table.md`). On the card, three Paint Buckets on a 1920 by 1080 shot added
about 60 to 70 ms a frame over the Noise alone. Most of that is finding the area, which is done on
the processor even when the card draws: a flood fill goes pixel to pixel and does not split up
well.
