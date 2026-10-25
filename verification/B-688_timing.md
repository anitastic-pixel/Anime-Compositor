# B-688 (D-808): 8 bpc (After Effects) timings before and after

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- Build: release
- Shot: the reference shot, 1920 by 1080, the effect on layer-2 (a new drawing every frame), frames 0 to 210 by 30, median ms of the whole frame as the viewer asks for it (sRGB bytes)
- Configuration: W7's perf_audit_timing (app/src/tests/perf_audit.rs) for the effects named; before = c4b0696c, after = B-688, both release builds
- Rounds: 5, alternating before and after, 2026-10-25 about 04:50 to 05:05; each cell is the median of the 5 rounds
- Load: a shared and busy machine (other lanes were building and testing at the same time)
- Columns: Viewer = the composition in float (Display), 8 bpc = the same composition set to 8 bpc (After Effects); ms per frame

| Effect | Setting | Rounds | Viewer Draft before | Viewer Draft after | Viewer Full before | Viewer Full after | 8 bpc Draft before | 8 bpc Draft after | 8 bpc Full before | 8 bpc Full after |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| (none) | as added | 5/5 | 8.8 | 6.7 | 9.9 | 9.4 | 38.8 | 11.6 | 60.8 | 13.6 |
| core.gaussian_blur | as added | 5/5 | 9.9 | 5.4 | 10.7 | 9.5 | 36.4 | 9.3 | 69.8 | 14.4 |
| core.gaussian_blur | sigma_px 40 | 5/5 | 9.5 | 7.2 | 23.3 | 13.1 | 40.2 | 8.9 | 84.4 | 17.7 |
| core.directional_blur | as added | 5/5 | 11.0 | 8.3 | 17.1 | 12.7 | 40.6 | 9.7 | 89.8 | 17.5 |
| core.directional_blur | length 40 | 5/5 | 8.7 | 6.9 | 27.3 | 12.0 | 39.7 | 11.5 | 124.9 | 20.2 |
| core.find_edges | as added | 5/5 | 16.6 | 6.8 | 17.4 | 12.0 | 47.6 | 9.2 | 82.7 | 15.6 |
| core.bilateral_blur | as added | 5/5 | 14.3 | 8.6 | 13.8 | 11.9 | 42.5 | 9.1 | 104.8 | 15.6 |
| core.bilateral_blur | radius 40 | 5/5 | 13.7 | 15.3 | 28.6 | 19.8 | 41.0 | 9.4 | 401.5 | 18.9 |
| core.cartoon | as added | 5/5 | 26.2 | 7.3 | 14.0 | 13.3 | 39.7 | 9.2 | 114.2 | 17.5 |
| core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | 5/5 | 29.9 | 18.6 | 18.8 | 17.3 | 42.5 | 12.2 | 390.0 | 20.5 |
| core.matte_choker | as added | 5/5 | 30.6 | 8.1 | 24.1 | 21.9 | 43.2 | 11.5 | 115.0 | 29.5 |
| core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | 5/5 | 31.4 | 24.2 | 56.6 | 56.6 | 44.4 | 11.5 | 306.6 | 60.3 |
