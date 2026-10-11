# B-422: frame times with Soft Physical Glow's tint

Written by `tests/b422_glow_tint.rs` (`cargo test --release --test b422_glow_tint -- --ignored`) on 2026-10-11, at commit 34281661.

**PROVISIONAL.** Other builds (cargo and rustc from other lanes) were running on this machine during the run, so these times are higher and noisier than a quiet machine gives. Measure again on a quiet machine before quoting them as the effect's cost.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with animated colour noise on its three layers, then the glow named, if any. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 25.1 | 15.7 |
| Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, no tint | Full | 57.0 | 55.6 |
| Noise, then the same, Tint Core to Edge | Full | 84.7 | 75.6 |
| Noise, then Soft Physical Glow as added, radius 500, Tint Core to Edge | Full | 97.7 | 94.3 |
