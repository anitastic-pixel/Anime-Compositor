# B-681: Scatterize on the card in single precision, before and after

Milliseconds a frame at 1920 by 1080, the median of 5 rounds of 8 frames (0 to 210, every 30th),
from `perf_audit_timing` (`app/src/tests/perf_audit.rs`). Each round ran the before and after test
binaries one after the other, so another lane's load falls on both alike. Taken 2026-10-25 04:15 to
04:20, the same rounds as B-680. In brackets: the after/before ratio's range over the 5 rounds.

- Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan); AMD64 Family 26 Model 68
  processor, 24 threads; Windows 11 Education 10.0.26300.
- Build: `cargo test --release` (`CARGO_INCREMENTAL=0`). Before = c4b0696c. After = c4b0696c with
  B-680 and B-681 (disjoint files) in the same binary.
- Configuration: CPU = the processor's frame (`preview_frame_cached`, which an export makes too).
  Card = the viewer on the card (`preview_frame_srgb8`), which takes Scatterize at Draft and Full.
  8 bpc = the viewer with 8 bpc (After Effects) on, which the card refuses (D-763), so the whole
  frame is the processor's.
- Load: about ten other lanes were building and testing. The (none) row, which uses none of the
  changed code, swung between 0.04 and 2.59 of itself from round to round. So only a change the
  after build shows in **every** round is read as real: Card Full.

| Effect | Setting | CPU Draft | CPU Full | Card Draft | Card Full | 8 bpc Draft | 8 bpc Full |
|---|---|---:|---:|---:|---:|---:|---:|
| (none) | as added | 5.7 -> 4.2 [0.04-1.21] | 28.6 -> 23.9 [0.12-1.22] | 15.2 -> 10.3 [0.15-1.85] | 32.7 -> 13.1 [0.24-1.46] | 113.6 -> 52.2 [0.15-1.03] | 123.3 -> 89.1 [0.11-2.59] |
| core.scatterize | as added | 15.5 -> 17.9 [0.48-1.32] | 131.0 -> 115.8 [0.44-1.21] | 35.1 -> 30.4 [0.38-2.39] | **250.8 -> 109.5 [0.27-0.55]** | 71.2 -> 60.9 [0.40-1.51] | 205.2 -> 164.8 [0.46-1.15] |

Where the card's time goes (`perf_audit_stages`, one frame at Full, the perf stages over its 4
card calls): the card's Scatterize drawing stage went from 205.3 and 226.6 ms (two runs before) to
28.6 ms. Building the list and sending it to the card is now most of what is left, and it swung from
252 to 1772 ms between runs under the load, so no number is claimed for it.
