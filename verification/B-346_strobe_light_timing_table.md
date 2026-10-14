# B-346: frame times with Strobe Light (D-466)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b346_strobe_light --no-run` and run with `--ignored b346_strobe_light_timing`: once on the card, then once with `B346_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 1% busy; 4.3 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 61.6, processor 98.7; Noise, then Strobe Light always on (Copy, Strobe Period 0.01) card 64.4, processor 136.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Strobe Light flashing all the time (Strobe Period 0.01) on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.8 |
| Noise, then Strobe Light always on (Copy, Strobe Period 0.01) | 14.4 | 44.1 |
| the same, Xor with Blend With Original 30 | 14.8 | 45.1 |

First loops (empty caches): card 15.7, 18.4, 17.8; processor 41.2, 43.7, 44.2.

**Reading it.** On the card, Strobe Light costs under 1 ms a 1080p layer in both settings (14.4 and
14.8 against 12.3, three layers): one warp pass while a flash is on, each pixel read once, and
nothing at all between flashes. On the processor it costs about the same, 1 ms a layer or a little
less (44.1 and 45.1 against 41.8). The busy run could not say this.
