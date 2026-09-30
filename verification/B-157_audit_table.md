# B-157: what each effect costs the card, before and after

`b157_card_effect_timing` in `tests/b157_f64_audit.rs`, one run on each build, every effect the
card draws (the list `tests/b156_gpu_adjust.rs` checks), each on the reference shot's first three
layers. "Less none" is the effect's frame time less the shot's with no effect: what the effect's
passes cost at 1920 by 1080 (Full) or half that (Draft), with its drawings sent again every frame.
One run each, so a figure under about 2 ms is within run-to-run noise; the five effects this build
changes are timed three times in `verification/B-157_timing_table.md`. "(processor)" marks an
effect whose frames the processor drew.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads, 64 GB
- System: Windows 11
- Build: release, both. Before: 46b3370 (B-156c). After: this build. No other cargo process
  was running.

| Effect | Full, less none, before | Full, less none, after | Draft, less none, before | Draft, less none, after | Changed by B-157 |
|---|---:|---:|---:|---:|---|
| Cross Glare | 184.6 | 19.1 | -0.3 | -0.6 | yes: its arms' samples summed in single precision |
| Median | 143.8 | 146.2 | 2.2 | 1.2 |  |
| Bloom | 119.4 | 38.4 | 14.4 | 2.7 | yes: its streaks' running totals in single precision (shared with Directional Blur) |
| Directional Blur | 54.1 | 11.5 | 7.3 | 0.3 | yes: its running totals in single precision |
| Line Blur | 48.5 | 7.4 | -1.0 | -3.5 | yes: its slopes and taps in single precision; a tap near the covering that ends a line is checked again in double |
| Lens Blur | 29.9 | 31.1 | -1.4 | -1.1 |  |
| Simple Choker | 23.4 | 23.7 | -0.7 | 1.9 |  |
| Turbulent Displace | 22.5 | 10.6 | -2.2 | -2.4 | yes: its noise in single precision |
| Distance Gradation | 21.2 | 23.6 | 0.1 | 2.3 |  |
| Light Rays | 21.0 | 23.4 | -1.2 | 0.9 |  |
| Smart Blur | 18.4 | 17.5 | -0.6 | -3.0 |  |
| Paraffin | 13.6 | 13.5 | -2.9 | -2.9 |  |
| Glow | 12.2 | 13.2 | -2.6 | -0.3 |  |
| Kira-kira | 12.0 | 11.9 | -2.2 | -2.3 |  |
| Find Edges | 11.5 | 10.5 | -2.0 | -2.7 |  |
| Color Lookup | 11.2 | 8.1 | -1.9 | -3.2 |  |
| Fractal Noise | 11.1 | 11.7 | -1.2 | -2.4 |  |
| Roughen Edges | 11.0 | 9.8 | -2.1 | -2.8 |  |
| Emboss | 10.9 | 7.6 | -2.3 | -3.2 |  |
| Polar Coordinates | 10.8 | 10.7 | -3.0 | -3.3 |  |
| Radial Shadow | 10.5 | 10.0 | -1.8 | -1.9 |  |
| Snowfall | 10.2 | 10.1 | -2.4 | -2.8 |  |
| Curves | 9.4 | 11.4 | -1.3 | -1.0 |  |
| Halftone | 9.4 | 7.7 | -1.2 | -3.0 |  |
| Radial Wipe | 9.4 | 7.5 | -1.4 | -3.1 |  |
| Color Balance | 9.3 | 8.1 | -1.9 | -2.6 |  |
| Offset | 9.1 | 6.8 | -2.6 | -2.9 |  |
| Sharpen | 9.0 | 11.1 | -1.9 | 0.6 |  |
| Cell Pattern | 9.0 | 8.8 | -2.6 | -2.7 |  |
| Gradient Map | 8.9 | 7.5 | -1.3 | -2.8 |  |
| Rain | 8.9 | 8.5 | -1.2 | -3.3 |  |
| Optics Compensation | 8.6 | 8.8 | -3.0 | -2.0 |  |
| Invert | 8.1 | 7.9 | -2.2 | -2.8 |  |
| Levels | 8.0 | 9.8 | -3.1 | -0.5 |  |
| Ripple | 7.9 | 9.0 | -3.2 | -2.3 |  |
| Hue/Saturation | 7.8 | 9.1 | -3.2 | -2.2 |  |
| Mosaic | 7.8 | 5.7 | -1.4 | -3.1 |  |
| Drop Shadow | 7.7 | 8.3 | -2.1 | -1.1 |  |
| Noise | 7.6 | 7.9 | -3.2 | -2.7 |  |
| Motion Tile | 7.5 | 5.4 | -2.8 | -3.5 |  |
| Chromatic Aberration | 7.4 | 7.9 | -3.3 | -2.6 |  |
| Brightness & Contrast | 7.4 | 8.2 | -3.0 | -0.8 |  |
| Posterize | 7.4 | 9.7 | -3.0 | -2.1 |  |
| Iris Wipe | 7.4 | 5.5 | -2.4 | -3.6 |  |
| Corner Pin | 7.4 | 9.2 | -3.3 | -1.2 |  |
| Leave Color | 7.3 | 7.5 | -3.1 | -3.4 |  |
| Threshold | 7.2 | 10.4 | -3.2 | -2.2 |  |
| Channel Mixer | 7.2 | 8.4 | -3.2 | -2.5 |  |
| Vibrance | 7.2 | 7.2 | -3.0 | -3.3 |  |
| HSV Key | 7.2 | 7.2 | -3.2 | -3.3 |  |
| Outline | 7.1 | 7.8 | -2.0 | -1.3 |  |
| Black & White | 7.1 | 9.4 | -3.2 | -1.0 |  |
| Solarize | 7.1 | 6.8 | -1.7 | -3.3 |  |
| Venetian Blinds | 7.1 | 5.3 | -1.8 | -3.3 |  |
| Radial Blur | 6.9 | 6.7 | -2.6 | -2.9 |  |
| Gradient | 6.8 | 7.8 | -3.2 | -2.7 |  |
| Rim Light | 6.5 | 7.1 | -1.9 | -1.4 |  |
| Wave Warp | 6.5 | 8.7 | -3.2 | -1.8 |  |
| Linear Wipe | 6.5 | 5.0 | -1.7 | -3.4 |  |
| Speed Lines | 6.2 | 8.1 | -3.2 | -1.2 |  |
| Camera Shake | 6.2 | 6.9 | -1.4 | -2.5 |  |
| Vignette | 6.1 | 7.0 | -3.3 | -2.9 |  |
| Bulge | 5.7 | 5.6 | -3.3 | -3.5 |  |
| Mirror | 5.7 | 5.6 | -3.1 | -3.5 |  |
| Twirl | 5.6 | 5.5 | -3.3 | -3.5 |  |
| Diffusion | 5.5 | 7.9 | -2.1 | 0.5 |  |
| Bevel Alpha | 5.1 | 5.0 | -2.1 | -2.2 |  |
| Exposure Flicker | 4.4 | 5.9 | -3.3 | -2.2 |  |
| Gaussian Blur | 1.9 | 0.9 | -2.7 | -3.8 |  |

The shot with no effect: before 19.4 ms at Full and 17.4 at Draft, after 19.6 and 17.5.

What it says:

- **Changed by B-157:** the five effects whose passes spent most of their time on double
  precision, which this card runs at 1/64 the speed of single. Their pictures stay within 1 level
  of the processor's (D-228).
- **Median** is the dearest left, but it is already single precision; its cost is sorting each
  pixel's neighbours, which this audit does not change.
- **Kept in double precision, not tried in this build:** Lens Blur's and Simple Choker's running
  totals share one buffer, and the Choker divides by a covering that can be very small, where
  rounding could move a colour by more than a level. Roughen Edges' noise feeds a cut-off, where
  Line Blur showed single precision can tip a pixel across. Distance Gradation's distance passes
  were not looked at. Light Rays' own passes are single precision already.
- **The rest** cost the card about 12 ms a frame or less at Full, three layers at once, and are
  left as they are.
- At Draft the shot with no effect measures slower than most shots with an effect (see
  `verification/B-157_timing_table.md`), so the Draft columns are mostly slightly negative.
