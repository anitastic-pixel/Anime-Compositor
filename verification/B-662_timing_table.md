# B-662: the motion finder behind Pixel Motion Blur and Timewarp, before and after

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
| core.pixel_motion_blur | as added | CPU/CPU | 36.6 | 36.5 | 188.0 | 166.5 | 31.0 | 33.4 | 190.0 | 159.1 | 46.1 | 50.6 | 223.7 | 192.3 | 5/5 |
| core.pixel_motion_blur | shutter_samples 32, vector_detail 40 | CPU/CPU | 41.3 | 36.5 | 273.4 | 259.1 | 48.2 | 59.8 | 313.4 | 288.9 | 56.8 | 57.9 | 352.2 | 294.6 | 5/5 |
| core.timewarp | as added | CPU/CPU | 40.6 | 38.8 | 189.1 | 159.7 | 40.5 | 40.0 | 225.3 | 170.2 | 63.2 | 46.4 | 254.6 | 197.1 | 5/5 |
| core.timewarp | vector_detail 40, smoothing_iterations 8, shutter_samples 10 | CPU/CPU | 51.7 | 59.5 | 442.3 | 393.1 | 70.3 | 58.0 | 436.5 | 410.9 | 79.0 | 76.4 | 450.3 | 388.0 | 5/4 |
