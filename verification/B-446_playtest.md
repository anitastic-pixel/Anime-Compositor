# B-446: Turbulent Noise's full type lists

Built on 2026-10-12 as D-567, under your /loop request. Turbulent Noise and Fractal Noise (one
engine underneath) had only two fractal types, Basic and Turbulent Basic, and two noise types,
Smooth and Block. They now have After Effects' whole lists: seventeen fractal types (Basic,
Turbulent Basic, Turbulent Smooth, Turbulent Sharp, Dynamic, Dynamic Progressive, Dynamic Twist,
Max, Smeary, Swirly, Rocky, Cloudy, Terrain, Subscale, Small Bumps, Strings, Threads) and five
noise types (Smooth, Block, Linear, Soft Linear, Spline). Adobe does not publish how each is
worked out, so each one is this program's own version, made to look the way its name says.
Projects saved before today look exactly as they did.

The check, `verification/D-567_turbulent_noise_types_table.md` (330 of 330), holds every
pixel to numbers a separate program worked out before the code was written, reruns every older
Turbulent Noise and Fractal Noise check unchanged, and holds the graphics card's picture to the
processor's (46 of 46 within 1 level). The pictures are in `verification/D-567 pictures/`,
each the noise at size 40 over the town street.

## What to check

1. **The starting point.** `t01_basic.png`: soft grey clouds, as before.
2. **Each fractal type is its own look.** `t02_turbulent.png` to `t17_threads.png`, one per type,
   in the menu's order. Look for: Turbulent Smooth, dark with rounded light creases; Turbulent
   Sharp, thin dark creases; the three Dynamic types, clouds pulled and swirled like smoke (Twist
   the most curled); Max, lighter clouds; Smeary, streaked sideways; Swirly, black and white bands;
   Rocky, flat patches in a few greys, like camouflage; Cloudy, soft light puffs on dark; Terrain,
   light ridges on dark; Subscale, fine detail only in the light parts; Small Bumps, a fine grain;
   Strings, thin light lines on black; Threads, thin threads at every size.
3. **Each noise type.** `n02_block.png`: square steps, as before. `n03_linear.png`,
   `n04_soft_linear.png` and `n05_spline.png`: clouds like Basic's, Linear a little harder,
   Spline the softest.
4. **Fractal Noise has them too.** `c1_cycle_2_at_90.png`: Fractal Noise with Dynamic Progressive
   on Spline and Cycle Evolution 2: in the app, key Evolution from 0 to 720 and play; it loops.
5. **In the app.** Add Turbulent Noise, open Fractal Type: seventeen choices, Noise Type: five.
   Pick a few and scrub; each draws without a warning. The same menus are on Fractal Noise.

Not settled: Adobe publishes no formulas, so how each type looks is this program's own, made after
its name. Turbulent Noise still lacks Overflow, Transform, Sub Settings and the longer blend list.

## Speed

`verification/B-446_turbulent_noise_types_timing_table.md`: PROVISIONAL, the machine busy with other lanes: on the card, a new type at complexity 6 30.9 ms a frame against 28.9 for basic on smooth (Noise alone 24.1), the heaviest setting (spline, complexity 20) 69.2; the processor 126.3, 138.1 and 339.8.
