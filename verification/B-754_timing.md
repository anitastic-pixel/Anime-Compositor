# B-754 (D-874) timing: Float and 32 bpc (After Effects) frames on the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads.
- Build: release. W7's `perf_audit_timing` (`app/src/tests/perf_audit.rs`), whose last two columns are timed in the depth `W7_DEPTH` names (float or ae) instead of 8 bpc; built from 7cc14eb3 with the same test change (before) and from B-754 (after).
- Load: a shared machine, four other lanes building and testing at the same time. The CPU and Display card columns run the same code in both builds; their spread (for example Bilateral Blur radius 40's Display card Full, 18.9 to 87.6 ms over the five after rounds) is the noise.
- Shot: the reference shot, 1920 by 1080, the effect on layer-2, frames 0 to 210 by 30, median ms of the whole frame as the viewer asks for it.
- Rounds: 5 alternating before/after rounds in each depth, 2026-10-25 17:15 to 17:47. Each cell is the median of the five rounds' medians, before -> after. "Card takes it" is for the Display composition; "Depth" columns are the composition set to the depth named.
- Draft: a frame with little on it is slower in Draft (3.3 -> 11.2 ms with no effect in Float): the card's fixed cost per frame, which Display already pays (its Card Draft column), against the processor's quarter-size frame. Every Full row and every heavy row is faster.

### Float: 5 rounds, medians of the per-round medians, ms, before -> after
| Effect | Setting | Card takes it before/after | CPU Draft | CPU Full | Card Draft | Card Full | Depth Draft | Depth Full |
|---|---|---|---:|---:|---:|---:|---:|---:|
| (none) | as added | CPU/CPU / CPU/CPU | 3.5 -> 3.9 | 21.4 -> 23.6 | 10.4 -> 12.5 | 11.5 -> 11.7 | 3.3 -> 11.2 | 20.9 -> 14.0 |
| core.gaussian_blur | as added | CPU/CPU / CPU/CPU | 4.5 -> 4.2 | 29.3 -> 29.0 | 8.5 -> 6.5 | 11.0 -> 11.9 | 4.2 -> 8.2 | 22.4 -> 15.4 |
| core.gaussian_blur | sigma_px 40 | card/card / card/card | 6.4 -> 6.9 | 41.0 -> 44.7 | 9.5 -> 8.2 | 18.1 -> 14.0 | 6.4 -> 8.4 | 42.3 -> 18.2 |
| core.directional_blur | as added | card/card / card/card | 5.8 -> 5.3 | 40.2 -> 40.7 | 10.7 -> 10.4 | 19.2 -> 18.1 | 6.2 -> 11.5 | 40.7 -> 18.5 |
| core.directional_blur | length 40 | card/card / card/card | 6.2 -> 7.6 | 38.5 -> 43.9 | 17.0 -> 13.0 | 16.3 -> 19.3 | 6.1 -> 10.2 | 36.6 -> 18.0 |
| core.find_edges | as added | card/card / card/card | 4.7 -> 6.9 | 31.5 -> 53.8 | 10.4 -> 10.1 | 13.3 -> 13.0 | 6.3 -> 8.3 | 32.9 -> 17.0 |
| core.bilateral_blur | as added | card/card / card/card | 8.0 -> 8.5 | 52.0 -> 61.2 | 15.3 -> 11.8 | 16.7 -> 17.6 | 8.9 -> 10.7 | 51.8 -> 16.4 |
| core.bilateral_blur | radius 40 | card/card / card/card | 7.9 -> 8.7 | 315.9 -> 332.8 | 26.0 -> 20.7 | 17.7 -> 42.8 | 9.6 -> 11.5 | 313.5 -> 18.9 |
| core.cartoon | as added | card/card / card/card | 13.0 -> 12.6 | 87.8 -> 85.3 | 21.3 -> 10.3 | 18.8 -> 16.6 | 12.2 -> 13.6 | 82.4 -> 17.1 |
| core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | card/card / card/card | 13.6 -> 18.4 | 358.7 -> 378.4 | 30.3 -> 20.6 | 21.6 -> 21.3 | 14.4 -> 9.9 | 357.7 -> 23.2 |
| core.matte_choker | as added | card/card / card/card | 14.6 -> 14.4 | 82.4 -> 83.2 | 23.6 -> 15.4 | 21.8 -> 24.8 | 15.0 -> 13.5 | 84.1 -> 26.3 |
| core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card / card/card | 21.2 -> 25.0 | 318.3 -> 345.1 | 48.8 -> 36.9 | 62.9 -> 59.1 | 23.1 -> 11.5 | 278.7 -> 62.3 |

### 32 bpc (After Effects): 5 rounds, medians of the per-round medians, ms, before -> after
| Effect | Setting | Card takes it before/after | CPU Draft | CPU Full | Card Draft | Card Full | Depth Draft | Depth Full |
|---|---|---|---:|---:|---:|---:|---:|---:|
| (none) | as added | CPU/CPU / CPU/CPU | 2.9 -> 3.0 | 19.6 -> 24.6 | 9.1 -> 8.6 | 11.4 -> 9.9 | 3.4 -> 11.8 | 19.7 -> 15.8 |
| core.gaussian_blur | as added | CPU/CPU / CPU/CPU | 4.0 -> 4.0 | 25.8 -> 29.6 | 6.8 -> 6.7 | 15.4 -> 10.2 | 4.3 -> 8.5 | 25.6 -> 21.7 |
| core.gaussian_blur | sigma_px 40 | card/card / card/card | 5.5 -> 5.5 | 43.6 -> 44.0 | 13.7 -> 10.5 | 16.5 -> 14.2 | 6.8 -> 10.4 | 47.1 -> 39.7 |
| core.directional_blur | as added | card/card / card/card | 5.3 -> 6.3 | 44.4 -> 41.2 | 15.4 -> 11.2 | 19.6 -> 14.8 | 6.0 -> 10.7 | 48.9 -> 34.3 |
| core.directional_blur | length 40 | card/card / card/card | 5.6 -> 7.9 | 41.3 -> 41.6 | 14.9 -> 11.4 | 15.4 -> 19.2 | 5.7 -> 12.1 | 41.3 -> 36.7 |
| core.find_edges | as added | card/card / card/card | 4.7 -> 6.8 | 32.1 -> 34.9 | 9.2 -> 9.6 | 12.8 -> 12.8 | 6.4 -> 8.9 | 33.2 -> 14.9 |
| core.bilateral_blur | as added | card/card / card/card | 6.7 -> 7.2 | 51.2 -> 52.0 | 13.5 -> 8.9 | 16.2 -> 12.4 | 6.0 -> 9.4 | 47.0 -> 15.9 |
| core.bilateral_blur | radius 40 | card/card / card/card | 8.1 -> 9.6 | 412.2 -> 354.3 | 31.6 -> 19.2 | 19.9 -> 18.0 | 8.2 -> 8.6 | 334.4 -> 18.0 |
| core.cartoon | as added | card/card / card/card | 13.1 -> 10.1 | 77.0 -> 81.4 | 18.1 -> 11.8 | 17.6 -> 17.6 | 9.5 -> 10.1 | 76.2 -> 17.1 |
| core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | card/card / card/card | 13.9 -> 10.8 | 353.6 -> 371.7 | 18.8 -> 18.1 | 18.5 -> 18.1 | 11.0 -> 8.9 | 407.9 -> 19.7 |
| core.matte_choker | as added | card/card / card/card | 13.2 -> 12.5 | 84.3 -> 74.0 | 31.9 -> 9.2 | 25.5 -> 23.3 | 14.4 -> 9.9 | 85.7 -> 26.6 |
| core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card / card/card | 20.8 -> 16.5 | 290.1 -> 254.5 | 21.0 -> 28.4 | 57.9 -> 55.0 | 18.9 -> 12.0 | 311.5 -> 60.5 |
