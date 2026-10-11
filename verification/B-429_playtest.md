# B-429: Reduce Interlace Flicker

Built on 2026-10-11 as D-549. After Effects' old **Reduce Interlace Flicker** (Stylize, now
Obsolete) is back under its own name, as a second name over our **Gaussian Blur**: a slight
blur up and down only, so a thin horizontal line no longer sits on one field of interlaced
video and twitters.

- **Softness** (1 as added): the size of the up-and-down blur in pixels, 0 to 500. 0 changes
  nothing; 0.5 to 1 is the usual amount. Can be keyed.

It is exactly Gaussian Blur with Blur Dimensions vertical, Repeat Edge Pixels on, and its size
the softness. It is drawn on the processor, as Gaussian Blur's one-way blur always is.

The check, `verification/D-549_reduce_interlace_flicker_table.md` (95 of 95 checks), holds every pixel to
numbers worked out by a separate program before the code existed, runs Gaussian Blur's earlier
checks again unchanged, and shows Reduce Interlace Flicker drawing exactly what that Gaussian
Blur draws. The pictures are in `verification/D-549 pictures/`.

## What to check

1. **Before.** `1_before.png`: thin lines, one pixel each: on the left white and black rows
   across, on the right white and black columns up and down.
2. **As added.** `2_as_added.png`: the rows on the left melt into an even grey, as the blur
   reaches up and down; the columns on the right stay sharp, as the blur never reaches sideways.
3. **Light.** `3_softness_0_3.png`: at 0.3 the rows on the left are only softened, the white
   and black rows still easy to tell apart.
4. **Street.** `4_street.png`: the town at the default 1: the window rows, the roof edges and
   the road's dashes slightly soft up and down, the buildings' side edges still sharp; compare
   it with `town.png`.
5. **In the app.** Add Reduce Interlace Flicker (Stylize) over thin horizontal text or lines:
   they soften up and down only. Raise Softness to see it plainly.
6. **Out of range.** Type 501 in Softness: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same setting and picture.

Our own choices you may want to judge: Softness counts pixels of Gaussian size (After Effects
does not say what it counts), and the top and bottom rows are repeated so the edges do not
fade.

## Speed

On the reference shot with three noisy layers it takes a frame to about 109 milliseconds as added whether the graphics card or the processor draws, as the blur is the processor's either way (the Noise alone takes 30.2 on the card and 63.3 on the processor); these are PROVISIONAL, measured while other builds were running (`verification/B-429_reduce_interlace_flicker_timing_table.md`).
