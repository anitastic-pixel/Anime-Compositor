# B-399: frame times with CC Spotlight's Gel renders (D-519) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b399_cc_spotlight_gel` from
63b55fd4 (B-399's code), and run with `--ignored b399_cc_spotlight_gel_timing`: once on the
card, then once with `B399_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo runs (two cargo processes) were running before
the card's run, between the two runs and after them. The numbers below are recorded as they came
out. They are not a clean measurement of the Gel renders: the card's Noise alone took 51.0 ms a
frame here, against 11.8 ms on the quiet machine in B-387's table, and the processor's 81.5 ms
against 45.9 ms there. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then a CC Spotlight
with layer 4 as its gel, every eighth frame asked for whole at Full. **Again** is the median of
the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 51.0 | 81.5 |
| Noise, then CC Spotlight, gel add, layer 4 as the gel, opening 90 | 167.0 | 140.9 |
| Noise, then CC Spotlight, gel shadow, layer 4 as the gel | 141.7 | 141.3 |
| Noise, then CC Spotlight, gel only, layer 4 as the gel, low and wide | 99.7 | 150.6 |

First loops (empty caches): card 65.1, 206.2, 179.7, 99.5; processor 75.4, 142.4, 136.5, 136.9.

**Reading it.** Every shot draws to the end on both. Each gel shot draws layer 4 a second time
each frame, as D-189's map, before the one `spotlight` pass; B-387's quiet table put the light
alone at about 1 ms a layer on the card, so most of what is added here is that map and the other
runs' use of the machine, which this table cannot separate. Re-run the two commands above when no
other cargo, rustc or test process is running and the card is idle, and replace this table.
