# B-660: the matte kit's box sums, before and after

Milliseconds a frame at 1920 by 1080, the median of 5 rounds of 8 frames (0 to 210, every 30th),
from `perf_audit_timing` (`app/src/tests/perf_audit.rs`), the before and after test binaries run
in turn each round so another lane's load falls on both alike (`S/wave7/w7d/ab.sh`), 2026-10-24
05:19 to 05:43.

- Machine: NVIDIA GeForce RTX 4070 Ti SUPER; AMD64 Family 26 Model 68 processor, 24 threads;
  Windows 11 Education 10.0.26300.
- Build: `cargo test --release` (`CARGO_INCREMENTAL=0`), before = 440cecf0, after = this commit
  with the other W7 speed changes (B-660 to B-662, disjoint files) in the same binary.
- Configuration: CPU = the processor's frame (`preview_frame_cached`, which an export makes too);
  Card = the viewer on the card (`preview_frame_srgb8`); 8 bpc = the viewer with 8 bpc (After
  Effects) on, which the card refuses (D-763), so the whole frame is the processor's; Draft and
  Full the viewer's resolution.
- Load: other lanes were building; `rustc` was running at the start of 4 of the 10 runs. Rows
  that do not use the changed code ((none)) move by up to 6% between the two sides, so a change
  under about 6% is not read as a change.

| Effect | Setting | Taken | CPU Draft before | CPU Draft after | CPU Full before | CPU Full after | Card Draft before | Card Draft after | Card Full before | Card Full after | 8 bpc Draft before | 8 bpc Draft after | 8 bpc Full before | 8 bpc Full after | rounds |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| (none) | as added | CPU/CPU | 2.7 | 3.0 | 18.0 | 18.1 | 6.0 | 6.2 | 8.3 | 8.4 | 29.7 | 30.7 | 48.4 | 51.2 | 5/5 |
| core.key_cleaner | as added | CPU/CPU | 19.8 | 23.3 | 242.5 | 232.9 | 36.3 | 31.2 | 282.8 | 240.8 | 58.0 | 46.1 | 360.3 | 284.0 | 5/4 |
| core.key_cleaner | additional_edge_radius 40 | CPU/CPU | 32.7 | 26.2 | 328.2 | 256.1 | 46.2 | 31.4 | 271.0 | 253.1 | 51.0 | 48.3 | 320.3 | 287.5 | 5/4 |
| core.matte_choker | as added | card/card | 12.7 | 10.2 | 93.4 | 71.8 | 38.0 | 13.1 | 20.3 | 22.4 | 33.1 | 31.6 | 110.5 | 100.3 | 5/4 |
| core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card | 13.8 | 14.0 | 303.1 | 232.9 | 38.2 | 40.2 | 55.1 | 57.2 | 37.0 | 38.0 | 288.3 | 274.7 | 5/4 |
| core.refine_hard_matte | as added | CPU/CPU | 6.1 | 8.9 | 243.1 | 236.8 | 22.9 | 20.1 | 276.9 | 262.7 | 37.2 | 35.2 | 264.5 | 264.6 | 5/4 |
| core.refine_hard_matte | feather 40, decontamination_radius 40 | CPU/CPU | 19.7 | 24.2 | 225.0 | 243.4 | 36.7 | 45.2 | 246.1 | 274.0 | 40.5 | 55.5 | 242.7 | 297.7 | 5/4 |
| core.refine_soft_matte | as added | CPU/CPU | 26.1 | 40.0 | 342.9 | 350.9 | 41.8 | 42.9 | 351.9 | 348.1 | 53.4 | 52.5 | 343.8 | 340.4 | 5/4 |
| core.refine_soft_matte | edge_radius 40, feather 40, decontamination_radius 40 | CPU/CPU | 43.1 | 43.3 | 463.6 | 459.2 | 56.3 | 59.1 | 587.1 | 489.4 | 83.3 | 69.2 | 642.7 | 519.2 | 5/4 |
