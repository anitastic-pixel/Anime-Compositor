# B-424: Halation

Built on 2026-10-11 as D-544, your pick #13 (Halation). It is a new effect, **Halation**, in the
Light & Glow group, built on Soft Physical Glow's engine as `docs/effects/PLUGINS.md` settles
("separate, small, on the shared soft-glow engine"). The rule is our own.

Film gives halation when bright light passes through it, bounces off the back and comes back
as a warm red-orange halo. So the halo lies just outside the brightest parts, hugging their
edges, and the bright parts themselves are left alone. That is what sets it apart from a glow
tinted red.

- **Threshold** (60) and **Threshold Smooth** (50): how bright the light must be to make
  halation, and how soft that cut is.
- **Radius** (25 pixels): how far it spreads.
- **Amount** (100 per cent, up to 400): how strong it is.
- **Colour** (red-orange, #ff4020).

All but the colour can be keyed.

Not built: a choice of film stock, grain or gate weave (other picks), and a mask for where it
applies.

The check, `verification/D-544_halation_table.md` (313 of 313), holds every pixel to numbers
worked out by a separate program before the code existed, runs Soft Physical Glow's earlier
checks (45, plus D-542's 22 and D-543's 20) again unchanged, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-544 pictures/`.

## What to check

1. **As added.** `halation_added.png`: the town with Halation as added: a thin red-orange rim
   round the bright windows, the road's white dashes and the bright sky's edge.
2. **Wide.** `halation_wide.png`: radius 60, amount 300: the rim wider and stronger. The windows
   and dashes themselves keep their colour; the halo is round them.
3. **White.** `halation_white.png`: the same in white, for comparison: the shape of the halo
   without its colour.
4. **Low threshold.** `halation_low.png`: threshold 20, radius 60, amount 200: more of the town
   is bright enough, so more edges get a halo.
5. **In the app.** Add Halation (Light & Glow) to a layer with bright parts on a dark ground.
   Raise Amount and Radius and watch the halo; set Threshold lower and watch more edges pick it
   up. Change the colour. Key Amount from 0 to 200 and play it: the halo fades in.
6. **Out of range.** Type 101 in Threshold: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the colour as added (red-orange), the strength as added,
and that the halo is added as light (like a glow on Screen), so it never darkens anything.

## Speed

Measured, but PROVISIONAL: other builds were running on this machine at the time, so the numbers are higher and noisier than a quiet machine gives. On the graphics card, the reference shot with colour noise and then Halation on its three layers (playing it again) took 87.7 ms a frame with Halation as added (radius 25) and 94.1 at radius 200, amount 200, against 61.3 for the noise alone. The full table is `verification/B-424_halation_timing_table.md`. A quiet re-measure is owed. What to check by eye: whether scrubbing a shot with Halation still feels smooth enough to you.
