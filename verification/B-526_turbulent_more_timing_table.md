# B-526: frame times with Turbulent Noise's Overflow, Transform and Sub Settings (D-647)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b526_turbulent_more --no-run` and run with `--ignored b526_turbulent_more_timing`: once on the card, then once with `B526_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.7 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 58.5, processor 111.8; Noise, then Turbulent Noise as added card 60.9, processor 186.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Turbulent Noise on each, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.1 |
| Noise, then Turbulent Noise as added | 19.7 | 78.3 |
| the same with Rotation 30, Offset (40, -25), Perspective Offset, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset (3, 2), Center Subscale, Soft Clamp, Hard Light | 21.8 | 92.3 |

First loops (empty caches): card 15.8, 24.6, 25.6; processor 41.6, 76.5, 91.1.

**Reading it.** With every D-647 setting moved, Turbulent Noise costs about 2 ms a frame more on the card (19.7 to 21.8, three layers) and about 14 ms more on the processor (78.3 to 92.3): the turned point is worked per octave per pixel. With the settings as added the old path is taken; it adds about 7 ms a frame over the Noise alone on the card (12.3 to 19.7) and about 37 on the processor (41.1 to 78.3). The busy run put the card's extra at about 17 ms; quiet, it is about 2, so on the card the new settings cost next to nothing. The processor's extra (about 14 ms) is what the busy run said.
