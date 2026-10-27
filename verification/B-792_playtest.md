# B-792 playtest: a Fast quality on Bilateral Blur and Cartoon (D-913)

## What changed for you

Bilateral Blur and Cartoon have a new choice at the bottom of their controls, **Quality**: Exact
or Fast.

- **Exact** is the default and is what the effects always drew, to the last digit.
- **Fast** works the same edge-keeping blur on a coarse grid of cells (the bilateral grid,
  published by the filter's own authors) instead of weighing every pixel's whole circle. Cartoon's
  Quality changes only its smoothing; its edges, line and fill stay exact.
- With a radius under 4.4, or a Threshold so small the grid would be huge, Fast draws exactly what
  Exact draws.
- Fast is your choice, so it is drawn everywhere: in the viewer, in RAM previews and in exports.
  Nothing switches to Fast on its own.
- A layer set to Fast is drawn by the processor, not the graphics card (the card draws Exact only
  and is already quick at it). So on a machine with a card, Fast is mainly for exports.

**Saved projects:** a project only says "fast" if you chose Fast; every older project opens as
Exact and saves exactly as it was.

## The numbers

Speed, measured in a quiet slot on 2026-10-27 (median whole frame of the reference shot, 5 rounds; the raw rounds in B-792_timing_raw.md): Bilateral Blur as added 183.6 ms Exact, 169.7 ms Fast (1.1x); radius 40: 6201.9 ms, 121.0 ms (51x); Cartoon as added 377.2 ms, 208.8 ms (1.8x); Cartoon heavy 6519.9 ms, 195.5 ms (33x).

How far Fast is from Exact at 1920 by 1080, frame 100 of the reference shot, in levels of 255:

| Setting | Average change | Largest change | Pixels within 1 level |
| --- | ---: | ---: | ---: |
| Bilateral Blur as added | 0.22 | 7 | 98.97% |
| Bilateral Blur radius 40 | 0.25 | 18 | 97.25% |
| Cartoon as added | 0.24 | 17 | 95.72% |
| Cartoon heavy (detail radius 40) | 0.33 | 37 | 89.69% |

Exact is unchanged: the four Exact settings give the same fingerprints before and after this
change, on the processor, on the card and in 8-bit output.

## What to look at

`verification/B-792 pictures/` holds four sets of three pictures:

| Set | Exact | Fast | Difference (8 times as bright) |
| --- | --- | --- | --- |
| Bilateral Blur as added | `bilateral_as_added_exact.png` | `bilateral_as_added_fast.png` | `bilateral_as_added_difference_x8.png` |
| Bilateral Blur radius 40 | `bilateral_radius_40_exact.png` | `bilateral_radius_40_fast.png` | `bilateral_radius_40_difference_x8.png` |
| Cartoon as added | `cartoon_as_added_exact.png` | `cartoon_as_added_fast.png` | `cartoon_as_added_difference_x8.png` |
| Cartoon heavy | `cartoon_heavy_exact.png` | `cartoon_heavy_fast.png` | `cartoon_heavy_difference_x8.png` |

Even made 8 times as bright, the difference pictures are nearly black. What is left lies along
edges (the tree trunk, the hill's outline), where a speck smaller than a cell is smoothed a little
differently.

## How to try it

1. Open a project with a picture layer.
2. Add Bilateral Blur. Set Radius to 40.
3. Play or scrub and note how long a frame takes. Set Quality to Fast and do the same.
4. Compare: edges should stay as sharp as Exact's, and flat areas just as smooth. Look for halos
   around edges or stepped bands in gradients; there should be none.
5. Set Quality back to Exact: the picture is exactly what it was before.
6. Do the same with Cartoon (Detail Radius 40).
7. Save with Fast and open the file again: Quality is still Fast. Export it: the export is Fast
   too.
8. Open a project saved before this change: its Bilateral Blur and Cartoon show Quality Exact.

**Correct if:** Fast looks like Exact at a glance with no halos or banding, Exact never changes,
a saved Fast stays Fast, and an older project opens as Exact.

## Not in this step

Fast is not drawn on the graphics card. Cartoon's edge finding is not made faster.
