# B-443: frame times with Shadow/Highlight's Midtone Contrast and clips (D-563) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b443_shadow_highlight_clip` from 5aafd541,
and run with `--ignored b443_shadow_highlight_clip_timing`: once on the card, then once with `B443_CPU`
set on the processor. **The machine was not quiet.** Other lanes' rustc builds and tests were running
during both runs. The numbers are recorded as they came out; a quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers (so nothing is kept), then Shadow/Highlight, every eighth frame of
240 asked for whole at Full. **Again** is the median of the loops after the first (seven on the
card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 19.4 | 42.1 |
| Noise, then Shadow/Highlight as added | 43.2 | 175.5 |
| Noise, then Shadow/Highlight as added with midtone contrast 60 | 29.0 | 179.9 |
| Noise, then Shadow/Highlight as added with both clips 1 (drawn on the processor) | 281.1 | 256.8 |

First loops (empty caches): card 24.1, 39.2, 34.3, 401.9; processor 40.9, 173.3, 178.9, 261.2.

**Reading it.** Every shot draws to the end on both. On the card Midtone Contrast is a few more
sums in D-400's `shhi` pass: the shot with it came out faster than the one without, so its cost
is lost in the busy machine's spread (B-279's quiet figure for as added was 24.6). A clip above 0
counts the whole layer, so the effect leaves the card: that shot costs what the processor costs,
plus the trip off the card, about 260 ms more than the Noise alone. Re-run the two commands above
when no other cargo, rustc or test process is running and replace this table.
