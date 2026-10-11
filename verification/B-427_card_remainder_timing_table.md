# B-427: the card's remainders, frame times

Written by `tests/b427_card_remainder.rs` (`cargo test --release --test b427_card_remainder -- --ignored`) on 2026-10-11, at commit 674637f8.

**PROVISIONAL.** Other builds (cargo and rustc from other lanes) were running on this machine during the run, so these times are higher and noisier than a quiet machine gives. Measure again on a quiet machine before quoting them as the effect's cost.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with each noise on its first three layers, a negative depth with no cycle and with a cycle of 3, the case the fix changed. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Fractal Noise, evolution -3000 moving, no cycle | Full | 66.0 | 56.7 |
| Fractal Noise, evolution -3000 moving, cycle 3 | Full | 43.2 | 63.0 |
| Noise Alpha, phase -100, no cycle | Full | 165.8 | 32.6 |
| Noise Alpha, phase -100, cycle 3 | Full | 86.5 | 36.4 |
