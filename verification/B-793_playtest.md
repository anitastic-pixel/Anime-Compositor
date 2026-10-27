# B-793 playtest: a Fast quality on Glow, Bloom, Deband and Halation (D-914)

## What changed for you

Glow, Bloom, Deband and Halation each have a new choice in their controls, **Quality**: Exact or
Fast.

- **Exact** is the default and is what the effects always drew, to the last digit.
- **Fast** does the wide blur inside the effect on a smaller picture, cells of 2, 4 or 8 pixels,
  and reads it back smoothly (as Nuke's Blur does at a lower quality). Deband instead looks at
  every 2nd, 4th or 8th pixel, so its hard edges stay hard. Halation's blurs are one step coarser.
- Fast only does anything when the blur is wide: Glow and Bloom from a radius of about 24, Deband
  from a Blur of 24, and Halation from a Radius of about 12. Below that, Fast is Exact.
- Fast is your choice, so it is drawn everywhere: in the viewer, in RAM previews and in exports.
  Nothing switches to Fast on its own.
- The graphics card does not draw Glow, Bloom or Deband at Fast. With the card on, the viewer
  draws them at Fast on the processor, which can be slower than Exact on the card. Fast is for
  exports and for working without the card. Halation at Fast is on the card as well.

**Saved projects:** nothing changes. A project only says "fast" if you chose Fast; every older
project opens as Exact and saves exactly as it was.

## The numbers

Measured in a quiet slot on 2026-10-27 (whole frame of the reference shot, median of 5 rounds,
`verification/B-793_fast_blur_timing_table.md`), Exact against Fast:

- Glow, radius 30: 72.3 ms, 73.7 ms (no gain). Radius 60: 80.5 ms, 69.6 ms (14% less).
- Bloom, radius 30, star: 204.6 ms, 199.7 ms (no real gain). Radius 100: 233.7 ms, 187.6 ms (20% less).
- Deband, blur 30: 135.6 ms, 139.0 ms (no gain). Blur 60: 185.1 ms, 152.7 ms (18% less).
- Halation as added (radius 25): 147.4 ms, 123.8 ms (16% less). Radius 100: 160.3 ms, 130.3 ms (19% less).

So Fast saves about a sixth of the frame on the big blurs and nothing on the small ones; at small
sizes leave it on Exact.

## What to look at

`verification/B-793 pictures/` holds four sets of three pictures, each cut around the spot where
Fast and Exact differ most on frame 100 of the reference shot:

| Set | Exact | Fast | Difference (8 times as bright) |
| --- | --- | --- | --- |
| Glow at radius 60 | `glow_radius_60_exact.png` | `glow_radius_60_fast.png` | `glow_radius_60_difference_x8.png` |
| Bloom at radius 100, star streaks | `bloom_radius_100_exact.png` | `bloom_radius_100_fast.png` | `bloom_radius_100_difference_x8.png` |
| Deband at blur 60 | `deband_blur_60_exact.png` | `deband_blur_60_fast.png` | `deband_blur_60_difference_x8.png` |
| Halation at radius 100 | `halation_radius_100_exact.png` | `halation_radius_100_fast.png` | `halation_radius_100_difference_x8.png` |

On average a pixel moves 0.01 levels of 255 for Glow and Bloom, 0.03 for Halation and 0.08 for
Deband. The largest single change is 1 level for Glow and Bloom, 4 for Halation and 23 for
Deband, which is along the soft edge of the cloud, where looking at every 4th pixel sees the edge
a little differently. Exact and Fast should look the same at a glance in every set.

## How to try it

1. Open a project with something bright in it.
2. Add Glow and set its Radius to 60.
3. Play or scrub with the graphics card off: note how long a frame takes. Set Quality to Fast and
   do the same.
4. Compare the glow: it should look the same at a glance.
5. Set Quality back to Exact: the picture is exactly what it was before.
6. Do the same with Bloom (Radius 100), Deband (Blur 60) and Halation (Radius 100).
7. Save with Fast, open the file again: Quality is still Fast. Export it: the export is Fast too.

**Correct if:** Fast is quicker at large radii, it looks like Exact at a glance, Exact never
changes, and an older project opens as Exact.

## Not in this step

Glow in one direction only (Glow Dimensions Horizontal or Vertical) stays Exact at Fast. Glow,
Bloom and Deband at Fast are not drawn on the graphics card.
