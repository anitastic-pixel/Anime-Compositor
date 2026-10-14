# B-324: frame times with Vegas (D-444)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b324_vegas --no-run` and run with `--ignored b324_vegas_timing`: once on the card, then once with `B324_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.1 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 13.0, processor 45.9; Noise, then Vegas as added (mask 1, 32 segments, Width 2) card 24.9, processor 48.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Vegas on the layer's masks, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 42.6 |
| Noise, then Vegas as added (mask 1, 32 segments, Width 2) | 22.5 | 45.5 |
| Noise, then Vegas, All Masks, 64 segments, Width 24 | 54.6 | 50.6 |
| Noise, then Vegas, All Masks, 8 segments, Length 0.5, Even, Width 60 | 51.7 | 55.7 |

First loops (empty caches): card 15.9, 25.8, 55.9, 53.8; processor 42.3, 44.1, 54.6, 54.4.

**Reading it.** On the card Vegas as added costs about 3.3 ms a 1080p layer (12.6 to 22.5, three
layers), and with all masks about 13 to 14 ms a layer (51.7 and 54.6); the dashes' straight bits
are worked out on the processor, then one pass a layer, each pixel looking only at the bits near
its band of rows. On the processor it costs about 1 to 4.4 ms a layer (45.5 to 55.7 against
42.6). So at every setting measured the processor is quicker than the card, by three to six
times with all masks; the busy run said only that the card was not always quicker.
