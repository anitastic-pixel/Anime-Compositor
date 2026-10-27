# B-783 playtest: a Fast quality on Refine Soft Matte and Refine Hard Matte (D-903)

## What changed for you

Refine Soft Matte and Refine Hard Matte have a new choice at the bottom of their controls,
**Quality**: Exact or Fast.

- **Exact** is the default and is what the effects always drew, to the last digit.
- **Fast** works the edge-fitting filter on blocks of up to 4 by 4 pixels and reads it back
  smoothly (the Fast Guided Filter, published by the filter's own authors). With a large radius the
  effect takes about two thirds of the time. With a radius under 8 nothing changes, so Fast only
  matters when Edge Radius, Feather or Decontamination Radius is 8 or more.
- Fast is your choice, so it is drawn everywhere: in the viewer, in RAM previews and in exports.
  Nothing switches to Fast on its own.

**Saved projects:** nothing changes. A project only says "fast" if you chose Fast; every older
project opens as Exact and saves exactly as it was.

## The numbers

Reference shot, 1920 by 1080, one layer keyed (Color Key), a moving Noise, then the effect, on
the processor at Full (what an export pays, and what the viewer pays, since the card does not draw
these effects). AMD processor, 24 threads, release build, 5 alternating rounds, other builds
running on the machine at the same time.

| Setting | Exact | Fast | Fast takes |
| --- | ---: | ---: | ---: |
| Refine Soft Matte as added (edge radius 10) | 312.2 ms | 245.8 ms | 79% |
| Refine Soft Matte, edge radius, feather and decontamination radius 40 | 431.9 ms | 266.9 ms | 62% |
| Refine Hard Matte, feather and decontamination radius 40 | 236.5 ms | 158.5 ms | 67% |
| Refine Hard Matte, feather 16 | 236.1 ms | 152.6 ms | 65% |

The whole table, round by round: `verification/B-783_fast_refine_timing_table.md`.

## What to look at

`verification/B-783 pictures/` holds three sets of three pictures, each cut around the spot where
Fast and Exact differ most on frame 100 of the reference shot:

| Set | Exact | Fast | Difference (8 times as bright) |
| --- | --- | --- | --- |
| Refine Soft Matte as added | `soft_as_added_exact.png` | `soft_as_added_fast.png` | `soft_as_added_difference_x8.png` |
| Refine Soft Matte at radius 40 | `soft_radius_40_exact.png` | `soft_radius_40_fast.png` | `soft_radius_40_difference_x8.png` |
| Refine Hard Matte at feather 40 | `hard_feather_40_exact.png` | `hard_feather_40_fast.png` | `hard_feather_40_difference_x8.png` |

The differences are in the half-keyed cloud, where the moving Noise makes every pixel different
from its neighbour: Fast smooths detail finer than a block, which is the price of the speed. On
average a pixel moves 0.16 to 0.39 levels of 255. The largest single change is 62 levels in Soft
and 225 in Hard: Hard Matte's Contrast sharpens the matte, so a speck that sits right at the edge
can flip between kept and dropped.

## How to try it

1. Open a project with a keyed layer (or key one with Color Key).
2. Add Refine Soft Matte. Set Edge Radius and Feather to 40.
3. Play or scrub: note how long a frame takes. Set Quality to Fast and do the same.
4. Compare the edge: hair and soft edges should look the same at a glance; fine noise inside a
   half-transparent area is smoother.
5. Set Quality back to Exact: the picture is exactly what it was before.
6. Save with Fast, open the file again: Quality is still Fast. Export it: the export is Fast too.

**Correct if:** Fast is visibly quicker at large radii, its edge looks like Exact's at a glance,
Exact never changes, and an older project opens as Exact.

## Not in this step

Key Cleaner uses the same filter but has no Quality yet (the next step). The fill colour and the
decontamination are still worked at full size, which is why Refine Hard Matte with
decontamination saves less.
