# B-791 playtest: a Fast quality on Key Cleaner (D-912)

## What changed for you

Key Cleaner has a new choice at the bottom of its controls, **Quality**: Exact or Fast. It is
the same setting Refine Soft Matte and Refine Hard Matte got in B-783.

- **Exact** is the default. It draws exactly what Key Cleaner always drew, to the last digit.
- **Fast** runs the edge-fitting filter on blocks of up to 4 by 4 pixels and then smooths the
  result back to full size, the same way Refine's Fast does. Below an Additional Edge Radius of 8,
  Fast and Exact are identical.
- Fast is your choice, so it is drawn everywhere: in the viewer, in RAM previews and in exports.
  Nothing switches to Fast on its own.
- Screen Key's Edge Refine uses the same fit internally and stays Exact.

**Saved projects:** nothing changes. A project only says "fast" if you chose Fast. Every older
project opens as Exact and saves exactly as it was.

## The numbers

Measured in a quiet slot on 2026-10-27 (whole frame of the reference shot, median of 5 rounds,
`verification/B-791_fast_key_cleaner_timing_table.md`): Key Cleaner as added (radius 10) 253.4 ms
Exact, 195.4 ms Fast; radius 20 268.5 ms, 171.8 ms; radius 40 263.3 ms, 172.5 ms. Fast takes about
two thirds to three quarters of Exact's time on the whole frame.

Exact is unchanged. Key Cleaner's and Screen Key's frames hash the same before and after this
change. The full table is `verification/B-791_fast_key_cleaner_table.md`.

## What to look at

`verification/B-791 pictures/` holds two sets of three pictures. Each set is cut around the spot
where Fast and Exact differ most on frame 100 of the reference shot:

| Set | Exact | Fast | Difference (8 times as bright) |
| --- | --- | --- | --- |
| Key Cleaner as added (radius 10) | `as_added_exact.png` | `as_added_fast.png` | `as_added_difference_x8.png` |
| Key Cleaner at radius 40 | `radius_40_exact.png` | `radius_40_fast.png` | `radius_40_difference_x8.png` |

On average a pixel moves 0.14 levels of 255 at radius 10 and 0.38 levels at radius 40. The
largest single change is 61 levels at radius 10 and 97 at radius 40, in the half-keyed noisy
cloud, where Fast smooths detail finer than a block. At radius 4 nothing differs.

## How to try it

1. Open a project with a keyed layer, or key one with Color Key.
2. Add Key Cleaner and set Additional Edge Radius to 40.
3. Play or scrub, and note how long a frame takes. Set Quality to Fast and do the same.
4. Compare the edge. Hair and soft edges should look the same at a glance. Fine noise inside a
   half-transparent area is smoother.
5. Set Quality back to Exact. The picture is exactly what it was before.
6. Save with Fast and open the file again: Quality is still Fast. Export it: the export is Fast
   too.

**Correct if:** Fast is quicker at large radii, its edge looks like Exact's at a glance, Exact
never changes, and an older project opens as Exact.
