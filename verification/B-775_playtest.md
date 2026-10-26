# B-775 playtest: Lens Blur by Fourier transform in the viewer (D-895)

What changed: in the viewer, a large Lens Blur is now drawn a faster way. This applies when the
radius is 100 or more at Full, the edges are transparent, and there is no blur map. The result
is within 1 level of the exact blur. Smaller radii, Draft frames, "repeat" edges, blur maps and
exports are all unchanged.

## What to look at

1. Open `verification/B-775 pictures/`.
   - `cpu.png` is the exact blur and `card.png` is the new way, both at radius 200, at Draft
     size, from the reference shot. **Expect:** you cannot tell them apart.
   - `difference.png` is black where the two agree and grey where a colour is 1 level apart.
     **Expect:** mostly black, with scattered faint dots and no white (white would mean 2 or more).
2. Open `verification/B-775_fft_lens_blur_table.md`.
   - **Expect:** 83 of 83 checks pass, and every reference-shot row says "largest 1, 0 over 1".
3. Open `verification/B-775_timing_table.md` (milliseconds per frame; the last two columns are
   the old and new way).
   - **Expect:** at radius 200 at Full, the new way takes about half the time (285 to 138).
     At 100 it is a little faster (126 to 119). Below 100 the viewer keeps the old way, which
     the table shows is as fast or faster there.
4. In the app (once this is merged), put a Lens Blur of radius 200 on a full-size layer and
   scrub at Full.
   - **Expect:** it looks the same as before and responds faster. Set the radius to 99, then
     100: **expect** no visible jump.

## If something looks wrong

Say which picture, row or frame. To undo it, set `LENS_FFT_FROM` in `src/gpu.rs` to
`f64::INFINITY`. That is one line, and every Lens Blur then uses the exact way again.
