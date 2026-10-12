# B-444: frame times with Shadow/Highlight's Auto Amounts, Temporal Smoothing and Scene Detect (D-564) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b444_shadow_highlight_auto` from 9be6174b,
and run with `--ignored b444_shadow_highlight_auto_timing`: once on the card, then once with `B444_CPU`
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
| Noise alone | 12.8 | 55.1 |
| Noise, then Shadow/Highlight as added | 24.1 | 211.3 |
| Noise, then Shadow/Highlight with Auto Amounts on (drawn on the processor) | 198.3 | 253.3 |
| Noise, then Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1 (drawn on the processor) | 473.7 | 567.2 |

First loops (empty caches): card 17.1, 28.3, 187.1, 450.0; processor 51.9, 210.7, 230.6, 580.0.

**Reading it.** Every shot draws to the end on both. Auto Amounts counts the whole layer, so the
effect leaves the card: that shot costs what the processor costs, plus the trip off the card,
about 185 ms more than the Noise alone. Temporal smoothing 0.1 reads the two frames either side of
each one, each drawn again with the effects before Shadow/Highlight, so it costs about 275 ms more
again on the card and 315 on the processor, as D-351's smoothing does (B-231). Re-run the two
commands above when no other cargo, rustc or test process is running and replace this table.
