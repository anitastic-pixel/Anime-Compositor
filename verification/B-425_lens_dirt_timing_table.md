# B-425: Lens Dirt, frame times

Written by `tests/b425_lens_dirt.rs` (`cargo test --release --test b425_lens_dirt -- --ignored`) on 2026-10-11, at commit 017f39ad.

**PROVISIONAL.** Other builds (cargo and rustc from other lanes) were running on this machine during the run, so these times are higher and noisier than a quiet machine gives. Measure again on a quiet machine before quoting them as the effect's cost.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with a changing colour Noise on its first three layers, then Soft Physical Glow or Lens Flare on each, without and with the fourth layer as its dirt. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 134.3 | 20.9 |
| Noise, then Soft Physical Glow (radius 40) | Full | 66.9 | 66.9 |
| Noise, then Soft Physical Glow with the fourth layer as its dirt | Full | 96.2 | 133.8 |
| Noise, then Lens Flare | Full | 102.7 | 76.6 |
| Noise, then Lens Flare with the fourth layer as its dirt | Full | 136.9 | 109.1 |
