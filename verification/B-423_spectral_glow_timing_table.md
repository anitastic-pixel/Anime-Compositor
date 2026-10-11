# B-423: frame times with Soft Physical Glow's Spectral

Written by `tests/b423_spectral_glow.rs` (`cargo test --release --test b423_spectral_glow -- --ignored`) on 2026-10-11, at commit bfb3f07f.

**PROVISIONAL.** Other builds (cargo and rustc from other lanes) were running on this machine during the run, so these times are higher and noisier than a quiet machine gives. Measure again on a quiet machine before quoting them as the effect's cost.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with animated colour noise on its three layers, then the glow named, if any. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 111.2 | 51.2 |
| Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, Spectral off | Full | 120.8 | 141.4 |
| Noise, then the same, Spectral on (80, 100, 120) | Full | 173.2 | 147.0 |
| Noise, then Soft Physical Glow as added, radius 500, Spectral on (80, 100, 120) | Full | 359.3 | 802.0 |
