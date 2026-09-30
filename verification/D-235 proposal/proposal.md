# D-235 (PROPOSED): big blurs worked at a smaller size, preview only

Written on 2026-09-29 by the agent. Nothing is built. This is for you to decide. My
recommendation is **yes, with a safety rule**. The details are below.

## What it is

A large blur is costly because each pixel has to average a wide area of other pixels. Gaussian
Blur at a sigma of 100 takes 601 pixels across and then 601 down for every pixel. Glow and
Bloom have a blur inside them too.

The idea is to shrink the picture first, blur it, and then enlarge it again:

1. Shrink by a factor of 2, 4 or 8, by averaging each 2 by 2, 4 by 4 or 8 by 8 block of pixels
   into one.
2. Blur the small picture by a correspondingly smaller amount.
3. Enlarge it back, blending smoothly between its pixels.

A wide blur has no fine detail left, so the small picture loses almost nothing. It is the
standard trick in games and in most compositing programs.

This is for the viewer only. The picture you export would still be worked exactly.

## What you would see

For a big, soft blur you would see nothing different. When the blur is small compared with the
shrink factor, fine things go wrong. Edges of the glow step or shimmer. With Bloom, whose halo
includes a tight blur 1/8 as wide as its radius, the halo loses its bright core.

## The safety rule

After shrinking, the blur that is left must still be at least **6 pixels** (its sigma). Where it
would not be, the program uses a smaller factor, down to no shrinking at all, which is exact
today's result. This check is made separately for each of Bloom's four blurs.

I tried smaller rules first. At 2 or 4 pixels a few cases were still 2 to 3 levels off. At 6,
every case here was within 1 level.

## The pictures

Every picture shows three panels side by side:
- left: the exact result;
- middle: the shortcut's result;
- right: the difference, made 16 times brighter so you can see it. Black there means the two
  panels are the same.

The frame is the reference shot's frame, `verification/B-05a_reference_frame.png`. Glow and
Bloom are at their own default thresholds (60 and 80) and intensity 1. For Gaussian Blur the
number is its sigma setting. For Glow and Bloom it is their radius setting.

Where the plain shortcut fails:
- `gaussian_20_factor8_plain.png`: a small blur shrunk 8 times. It is 3 levels off at edges.
- `glow_20_factor8_plain.png`: 18 levels off. The glow's edges step.
- `bloom_20_factor8_plain.png`: 60 levels off. The whole halo has gone wrong.

Where the plain shortcut already works:
- the `_50_factor2_plain.png`, `_100_factor4_plain.png` and `_200_factor8_plain.png` pictures.

With the safety rule (each is within 1 level):
- `gaussian_200_factor8_guarded.png`
- `glow_200_factor8_guarded.png`
- `bloom_200_factor8_guarded.png`

## The numbers

A level is one step of 255 in the 8-bit picture you see. The program's standard, which you
accepted for the card, is **at most 1 level** off. The tables show:
- the worst difference anywhere in the frame, in levels;
- the share of the frame's 2,073,600 pixels that are off by more than 1 level;
- the PSNR, the usual score for how close two pictures are, where higher is closer and above
  about 50 dB is very hard to see;
- the speed-up (see the next section).

**The plain shortcut, always at the factor asked for:**

| Effect and size | Factor 2 | Factor 4 | Factor 8 |
| --- | --- | --- | --- |
| Gaussian Blur 20 | 1, 0%, 69.0 dB, 6.8x | 1, 0%, 63.9 dB, 27x | **3**, 0.10%, 58.0 dB, 44x |
| Gaussian Blur 50 | 1, 0%, 71.5 dB, 7.5x | 1, 0%, 63.7 dB, 41x | 1, 0%, 61.6 dB, 97x |
| Gaussian Blur 100 | 1, 0%, 74.5 dB, 7.7x | 1, 0%, 69.9 dB, 50x | 1, 0%, 63.2 dB, 162x |
| Gaussian Blur 200 | 1, 0%, 76.8 dB, 7.9x | 1, 0%, 72.1 dB, 56x | 1, 0%, 68.7 dB, 247x |
| Glow 20 | **3**, 0.006%, 67.5 dB, 5.3x | **7**, 0.14%, 60.8 dB, 13x | **18**, 12.7%, 50.4 dB, 16x |
| Glow 50 | 1, 0%, 70.5 dB, 6.6x | **2**, 0.006%, 63.7 dB, 24x | **4**, 0.10%, 59.4 dB, 37x |
| Glow 100 | 1, 0%, 72.1 dB, 7.2x | 1, 0%, 67.3 dB, 35x | **2**, 0.03%, 61.3 dB, 69x |
| Glow 200 | 1, 0%, 74.7 dB, 7.6x | 1, 0%, 70.0 dB, 46x | 1, 0%, 66.0 dB, 122x |
| Bloom 20 | **8**, 6.6%, 52.8 dB, 4.0x | **30**, 17.3%, 42.0 dB, 7.1x | **60**, 28.2%, 35.0 dB, 7.9x |
| Bloom 50 | **3**, 0.002%, 63.0 dB, 5.5x | **9**, 7.3%, 52.6 dB, 15x | **22**, 19.5%, 43.5 dB, 19x |
| Bloom 100 | 1, 0%, 67.4 dB, 6.5x | **2**, 0.009%, 60.8 dB, 23x | **10**, 9.2%, 51.3 dB, 35x |
| Bloom 200 | 1, 0%, 70.5 dB, 7.2x | 1, 0%, 65.2 dB, 34x | **3**, 0.06%, 59.3 dB, 65x |

Bold marks a result past the 1-level standard. Notice that "only above radius 50, factor 2"
would not be safe on its own, because Bloom at 50 with factor 2 is 3 levels off.

**With the safety rule.** The largest factor allowed is 8, and the program may use a smaller
one. In the table, "used" is the factor each blur actually got; Bloom has four blurs, so it shows
four factors.

| Effect and size | Worst | Off by >1 | PSNR | Used | Speed-up |
| --- | --- | --- | --- | --- | --- |
| Gaussian Blur 20 | 1 | 0% | 69.0 dB | 2 | 6.8x |
| Gaussian Blur 50 | 1 | 0% | 61.6 dB | 8 | 97x |
| Gaussian Blur 100 | 1 | 0% | 63.2 dB | 8 | 162x |
| Gaussian Blur 200 | 1 | 0% | 68.7 dB | 8 | 247x |
| Glow 20 | 0 (exact) | 0% | exact | none | 1x |
| Glow 50 | 1 | 0% | 70.5 dB | 2 | 6.6x |
| Glow 100 | 1 | 0% | 67.3 dB | 4 | 35x |
| Glow 200 | 1 | 0% | 66.0 dB | 8 | 122x |
| Bloom 20 | 0 (exact) | 0% | exact | none | 1x |
| Bloom 50 | 1 | 0% | 76.0 dB | 2/1/1/1 | 1.8x |
| Bloom 100 | 1 | 0% | 72.2 dB | 4/2/1/1 | 3.8x |
| Bloom 200 | 1 | 0% | 69.9 dB | 8/4/2/1 | 10x |

## How much faster (an estimate, not a timing)

I did not time this in the program. The speed-ups are counts of multiply-adds per pixel:
- **Today, exact:** each pixel takes 2 × (2 × ceil(3 sigma) + 1) multiply-adds for a blur that
  goes across and then down.
- **The shortcut:** one read per pixel to shrink, the small blur on 1/4, 1/16 or 1/64 as many
  pixels, and four taps per pixel to enlarge.

The real gain will be smaller than these counts suggest. That is especially true on the card,
where reading memory and not doing arithmetic often sets the pace. It should still be large for
big blurs: tens of times for Gaussian Blur and Glow at 100 and over.

## My recommendation

**Accept it with the safety rule (sigma of at least 6 left after shrinking, factor at most 8),
for the viewer only.**
- Exports stay exact.
- Nothing changes for small blurs, because the rule keeps them exact.
- Every case measured here stays within the 1 level the card is already held to.

The next steps before switching it on:
- Build it with the effects' own fixtures and this frame as its checks.
- Keep the switch off until those pass on the card.

This test used one frame. A picture with harder bright edges could find a case the frame did
not.

## Your choices

1. **Accept with the safety rule, on in the viewer, exports exact** (recommended).
2. **Accept with the safety rule, but behind a viewer switch that is off by default.** You turn
   it on when you want the speed.
3. **Use it only while playing or scrubbing.** When you stop, the exact frame replaces it, which
   is the "refine on stop" item G9 already queued. Then even a small difference would only be
   seen in motion.
4. **Decline.** Big blurs stay exact and as slow as today.

## Where it comes from

- `tools/d235_downsample_blur_experiment.py` makes every number and picture here. Run it with
  `python tools/d235_downsample_blur_experiment.py`.
- Its exact blur is checked against `tools/adjust_reference.py`'s blur before anything is
  printed.
- Glow and Bloom follow `tools/glow_reference.py` and `tools/bloom_reference.py`, with Bloom's
  streaks off.
- `experiment_output.txt` is the printout from the run that made these files.
