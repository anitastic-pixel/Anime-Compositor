# B-680: Caustics' wide blurs through an FFT, before and after

Milliseconds a frame at 1920 by 1080, the median of 5 rounds of 8 frames (0 to 210, every 30th),
from `perf_audit_timing` (`app/src/tests/perf_audit.rs`). Each round ran the before and after test
binaries one after the other, so another lane's load falls on both alike. Taken 2026-10-25 04:15 to
04:20. In brackets: the after/before ratio's range over the 5 rounds.

- Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan); AMD64 Family 26 Model 68
  processor, 24 threads; Windows 11 Education 10.0.26300.
- Build: `cargo test --release` (`CARGO_INCREMENTAL=0`). Before = c4b0696c. After = c4b0696c with
  B-680 and B-681 (disjoint files) in the same binary.
- Configuration: CPU = the processor's frame (`preview_frame_cached`, which an export makes too).
  Card = the viewer on the card (`preview_frame_srgb8`). 8 bpc = the viewer with 8 bpc (After
  Effects) on. Draft and Full are the viewer's resolution. Caustics has no card pass, so all of its
  columns are the processor's work.
- Load: about ten other lanes were building and testing. The (none) row, which uses none of the
  changed code, swung between 0.04 and 2.59 of itself from round to round. So only a change the
  after build shows in **every** round is read as real: the Bottom Blur 40 Full columns.

| Effect | Setting | CPU Draft | CPU Full | Card Draft | Card Full | 8 bpc Draft | 8 bpc Full |
|---|---|---:|---:|---:|---:|---:|---:|
| (none) | as added | 5.7 -> 4.2 [0.04-1.21] | 28.6 -> 23.9 [0.12-1.22] | 15.2 -> 10.3 [0.15-1.85] | 32.7 -> 13.1 [0.24-1.46] | 113.6 -> 52.2 [0.15-1.03] | 123.3 -> 89.1 [0.11-2.59] |
| core.caustics | as added | 39.7 -> 25.5 [0.04-2.83] | 247.8 -> 179.1 [0.26-1.82] | 29.7 -> 23.6 [0.13-1.57] | 227.5 -> 150.8 [0.37-1.34] | 126.5 -> 59.8 [0.30-0.63] | 302.6 -> 265.9 [0.27-1.05] |
| core.caustics | bottom_blur 40 | 32.4 -> 24.9 [0.24-1.86] | **467.2 -> 202.8 [0.37-0.57]** | 41.8 -> 39.2 [0.64-1.73] | **463.8 -> 189.4 [0.37-0.58]** | 66.8 -> 77.5 [0.30-2.01] | 645.4 -> 323.9 [0.33-1.17] |

As added, the blurs stay on the direct taps (Smoothing's radius is 29, under the FFT's 48), so
that row is not expected to change. Draft blurs are a quarter as wide and also stay direct.

The blur alone (`d800_fft_blur_timing`, the lib test, median of 5, 1920 by 1080, 4 channels, an
earlier and quieter moment): sigma 76.8 (Bottom Blur 40), taps 229.9 ms against FFT 62.2 ms. One
channel: 68.3 against 16.9. Sigma 9.6 (as added): taps 37.7 against FFT 61.9, which is why a narrow
blur keeps the taps.
