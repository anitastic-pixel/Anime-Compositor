# B-243: Diffusion's second pass

Built on 2026-10-08 under your effects loop request, decided as D-364. This is your pick #2.
**Diffusion** (in **Blur & Sharpen**) now has a second pass. The usual anime diffusion is two
blurred copies of the picture: one in Lighten or Screen, then a second in Soft Light or Overlay
at about 30 to 50 per cent. Before, the second copy meant a second layer. Now it is two more
settings on the same effect.

The new settings, with the values they start at:

- **Second Amount** (0, off): how much of the glow is laid on again, 0 to 100 per cent. It can
  be keyed.
- **Second Blend** (Soft Light): Soft Light, or Overlay for a stronger look.

Both copies use the same blur, set by **Radius**. None of the tutorials gives the second copy a
size of its own. Because the second pass starts at 0, every project made before opens and looks
exactly as it did.

The check, `verification/D-364_diffusion_second_table.md` (71 of 71), holds every pixel to
numbers worked out by a separate program before the code existed. It also holds the graphics
card's picture to within 1 level of the processor's. The pictures are in
`verification/D-364 pictures/`.

## What to check

**`1_before.png`** is a street of coloured buildings with no effect.

1. **Before.** `2_lighten_50.png` is Diffusion at Radius 30, Amount 50, Blend Lighten, with the
   second pass off. This is the glow as it was before this change: soft and lifted, a little
   washed out.
2. **Soft Light added.** `3_lighten_50_soft_light_50.png` is the same with Second Amount 50,
   Soft Light. Compared with picture 2:
   - the colours should be a little richer;
   - the darks should be a little deeper and the lights a little brighter;
   - the soft glow should stay.
3. **Overlay added.** `4_lighten_50_overlay_50.png` is the same with Overlay instead. It should
   do the same as picture 3, but more strongly.
4. **In the app.**
   - Put Diffusion on a cel and set Blend to Lighten.
   - Raise Second Amount to about 40 and switch Second Blend between Soft Light and Overlay.
   - At Second Amount 0, the picture should be what Diffusion always gave.
5. **Saved and opened again.** Save, close and open the project. The settings and the picture
   should be the same.

With Draw on: GPU, the second pass costs about 1 ms a 1080p layer
(`verification/B-243_diffusion_second_timing_table.md`).

## Not built

- In tomoex's tutorial, the second blurred copy is an adjustment layer, so it blurs the picture
  with the first glow already on. Ours blurs the original once and lays that blur on twice. At
  the usual 30 to 50 per cent the difference is small.
- The RETAS lesson's second layer is the sharp original in Normal at 30 per cent, to bring the
  lines back. Diffusion's own Amount below 100 already does this, so nothing was added for it.
