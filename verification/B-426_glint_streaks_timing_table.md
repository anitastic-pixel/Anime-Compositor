# B-426: Glint Streaks, frame times

Written by `tests/b426_glint_streaks.rs` (`cargo test --release --test b426_glint_streaks -- --ignored`) on 2026-10-11, at commit f84ba3b8.

**PROVISIONAL.** Other builds (cargo and rustc from other lanes) were running on this machine during the run, so these times are higher and noisier than a quiet machine gives. Measure again on a quiet machine before quoting them as the effect's cost.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with a changing colour Noise on its first three layers, then Cross Glare on each, as added, with alternating ray colours, and with gradient colours and full shimmer. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 23.0 | 18.1 |
| Noise, then Cross Glare as added | Full | 69.1 | 40.1 |
| Noise, then Cross Glare, ray colours alternate | Full | 41.6 | 42.2 |
| Noise, then Cross Glare, gradient, shimmer 100 | Full | 47.5 | 44.6 |
