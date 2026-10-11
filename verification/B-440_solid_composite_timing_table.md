# B-440: frame times with Solid Composite (D-560) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b440_solid_composite` from 87ebe60a
(B-440's test; no engine code changed), and run with `--ignored b440_solid_composite_timing`: once
on the card, then once with `B440_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo and rustc builds (six processes) were running
during both runs: the card's Noise alone came out at 47.9 ms against 13.5 on B-433's run the same
day. The numbers are recorded as they came out; a quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers (so nothing is kept), then Solid Composite, every eighth frame of
240 asked for whole at Full. **Again** is the median of the loops after the first (seven on the
card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 47.9 | 134.2 |
| Noise, then Solid Composite, #3366cc, source opacity 50 | 49.2 | 236.9 |
| Noise, then Solid Composite, multiply, #ff8040 at opacity 60 | 42.8 | 210.4 |

First loops (empty caches): card 114.9, 54.4, 50.5; processor 147.0, 136.2, 214.6.

**Reading it.** Every shot draws to the end on both. On the card the effect's cost does not show
above the busy machine's spread (one shot came out faster than the Noise alone); Solid Composite
rides in the fused colour pass B-222 built. The processor's 76 to 103 ms over the Noise is likely
inflated. Re-run the two commands above when no other cargo, rustc or test process is running
and replace this table.
