# B-661: Caustics' blur, before and after

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
| core.caustics | as added | CPU/CPU | 8.4 | 8.5 | 108.8 | 97.4 | 23.0 | 22.5 | 126.8 | 128.3 | 35.1 | 31.9 | 169.9 | 144.7 | 5/4 |
| core.caustics | bottom_blur 40 | CPU/CPU | 12.1 | 11.9 | 371.5 | 322.6 | 26.6 | 28.4 | 394.8 | 332.1 | 33.3 | 34.7 | 415.1 | 373.8 | 5/4 |
