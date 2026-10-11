# B-424: Halation, frame times

Written by `tests/b424_halation.rs` (`cargo test --release --test b424_halation -- --ignored`) on 2026-10-11, at commit ba4df69a.

**PROVISIONAL.** Other builds (cargo and rustc from other lanes) were running on this machine during the run, so these times are higher and noisier than a quiet machine gives. Measure again on a quiet machine before quoting them as the effect's cost.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with a changing colour Noise on its first three layers, then Halation on each. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 75.4 | 61.3 |
| Noise, then Halation as added (radius 25) | Full | 96.4 | 87.7 |
| Noise, then Halation, radius 200, amount 200 | Full | 106.1 | 94.1 |
