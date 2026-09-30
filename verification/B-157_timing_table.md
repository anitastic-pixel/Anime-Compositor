# B-157: the double-precision audit, before and after

`b157_card_effect_timing` in `tests/b157_f64_audit.rs`: the reference shot with the effect on its
first three layers, every eighth frame of 240 asked for as the viewer asks, with the card told to
forget what it holds before each frame, so every frame runs every pass again. The shot with no
effect is run once untimed first, as the card's clocks rise over the first seconds. One loop fills
the processor's caches, then three are timed; the figure is the median of their 90 frames, in ms.
Run three times on each build; the column is the median of the three runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads, 64 GB
- System: Windows 11
- Build: release, both. Before: 46b3370 (B-156c). After: this build. No other cargo process
  was running.

| Effect | Quality | Before ms | After ms | After, less the shot with no effect | After, runs 1 / 2 / 3 | What moved |
|---|---|---:|---:|---:|---|---|
| None | Full | 19.2 | 19.4 | 0.0 | 19.1 / 21.2 / 19.4 | nothing (the floor) |
| None | Draft | 17.3 | 17.6 | 0.0 | 17.2 / 19.3 / 17.6 | nothing (the floor) |
| Cross Glare | Full | 207.4 | 36.7 | 17.3 | 36.7 / 36.4 / 38.9 | its arms' samples summed in single precision |
| Cross Glare | Draft | 17.4 | 14.4 | -3.2 | 14.1 / 14.4 / 15.4 | its arms' samples summed in single precision |
| Bloom | Full | 139.5 | 57.1 | 37.7 | 56.8 / 59.1 / 57.1 | its streaks' running totals in single precision (shared with Directional Blur) |
| Bloom | Draft | 29.3 | 20.3 | 2.7 | 19.7 / 21.1 / 20.3 | its streaks' running totals in single precision (shared with Directional Blur) |
| Directional Blur | Full | 72.4 | 31.6 | 12.2 | 30.7 / 31.6 / 32.7 | its running totals in single precision |
| Directional Blur | Draft | 22.3 | 17.9 | 0.3 | 17.4 / 17.9 / 20.1 | its running totals in single precision |
| Line Blur | Full | 66.8 | 25.9 | 6.5 | 25.8 / 25.9 / 27.0 | its slopes and taps in single precision; a tap near the covering that ends a line is checked again in double |
| Line Blur | Draft | 16.3 | 14.3 | -3.3 | 14.0 / 14.3 / 14.8 | its slopes and taps in single precision; a tap near the covering that ends a line is checked again in double |
| Turbulent Displace | Full | 40.9 | 28.4 | 9.0 | 28.4 / 28.3 / 30.3 | its noise in single precision |
| Turbulent Displace | Draft | 15.0 | 14.4 | -3.2 | 14.1 / 14.4 / 16.4 | its noise in single precision |

What it says:

- **At Full, all five are much quicker:** Cross Glare 207.4 to 36.7 ms a frame, Bloom 139.5 to
  57.1, Directional Blur 72.4 to 31.6, Line Blur 66.8 to 25.9, Turbulent Displace 40.9 to 28.4.
  The shot with no effect does not change beyond run-to-run noise.
- **At Draft the gains are small**, as the card works on a quarter of the pixels.
- At Draft, the shot with no effect measures about 3 ms slower than most shots with an effect,
  in every run, even after its untimed first loop. Why was not looked into; it makes the Draft
  "less the shot with no effect" figures slightly negative, and a Draft figure within about 3 ms
  of it says nothing about an effect.
