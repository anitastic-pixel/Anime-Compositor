# B-164: big blurs worked small in the viewer, by hand

Built on 2026-09-29 as you accepted in D-235 ("decline 234, accept 235"). How it was built is
decision D-221.

When a Gaussian Blur, Glow or Bloom is big, the viewer's graphics card now shrinks the picture
(by 2, 4 or 8), blurs the small picture and enlarges it again. A big blur looks the same either
way, and the small one is far less work. D-235's safety rule decides when: only when the small
blur is still at least 6 pixels wide, so small blurs stay exact. **Exports and renders to file
never do this**; they are exact, as before.

## The checks

- `verification/B-164_downsampled_blurs_table.md`: **24 of 24 pass.** Gaussian Blur, Glow and
  Bloom at 20, 50, 100 and 200, at Full and at Draft, on three layers of the reference shot. No
  pixel is more than 1 level of 255 from the exact frame the processor draws.
- The worst case is in `verification/B-164 pictures/`: `cpu.png` (exact), `gpu.png` (worked
  small) and `difference x64.png`. The difference picture is black where they agree and dark
  grey where they are 1 level apart.
- Every other graphics-card check passes, with its table unchanged.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave the viewer's **Draw on: Auto** and its quality at **Full**.

## What to check

1. **A big Gaussian Blur.** Add a **Gaussian Blur** to layer 1 and set its **Radius σ (px)** to **200**.
   Stop on frame 100. Switch between **Draw on: CPU** and **Draw on: GPU**. The blur should look
   the same both ways.
2. **Speed.** With **Draw on: Auto**, drag along the timeline. It should feel smoother than
   before this build. (Measured with the blur on three layers, the card now takes about 15 ms
   for this frame instead of about 91.)
3. **Glow and Bloom.** Replace the blur with a **Glow** at radius **200**, then a **Bloom** at
   radius **200**. Switch **Draw on: CPU** and **GPU** again. They should look the same.
4. **Small blurs.** Set the Gaussian Blur's **Radius σ (px)** to **5**. Nothing changes from before: small
   blurs are always exact.
5. **Export is exact.** Export frame 100 with the size-200 blur to PNG. It is the processor's
   exact picture, as it always was.

## How long a frame takes

Measured on this machine (RTX 4070 Ti SUPER, Ryzen 9 9900X), release build, nothing else
building. Milliseconds for the card to draw frame 100 with the effect on three layers, the
middle of three runs. The full table is `verification/B-164_timing_table.md`.

| Shot, Full | Before | Now |
|---|---:|---:|
| Gaussian Blur 50 | 24.1 | 13.9 |
| Gaussian Blur 200 | 90.9 | 14.6 |
| Glow 200 | 43.8 | 25.6 |
| Bloom 200 | 59.4 | 31.6 |

At Draft the blurs were already cheap. Two Draft cases (Gaussian Blur 50 and Glow 200) are now
under a millisecond slower, because shrinking and enlarging are two extra steps.

## Not built: a faster Lens Blur (D-222, your decision)

The plan also said to try a faster Lens Blur by a "fast Fourier transform", but only if it stays
within 1 level. It does not quite. Wherever you can see the picture it is within 1 level. But in
the nearly invisible haze at the blur's edge, the hidden colour of those pixels comes out wrong.
You cannot see it: compare `lens 200 as seen over grey exact.png` with `... single-precision
FFT.png` in `verification/B-164 pictures/`. The hidden colour is scrambled, though, as the two
`lens 200 hidden colour` pictures show. D-222 gives you three choices. The recommendation is to
leave Lens Blur exact as it is.

## If something is wrong

Tell me which step, the effect and size, and whether it was Full or Draft.
