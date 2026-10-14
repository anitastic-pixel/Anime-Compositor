# B-274: frame times with Arbitrary Map (D-395)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b274_arbitrary_map --no-run` and run with `--ignored b274_arbitrary_map_timing`: once on the card, then once with `B274_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone card 12.4 and 13.4, processor 43.4 and 42.9; Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 card 15.6 and 16.6, processor 58.0 and 58.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card, again | Processor, again |
|---|---:|---:|
| Noise alone | 13.4 | 44.7 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | 21.3 | 59.1 |

**Reading it.** Arbitrary Map adds 7.9 ms a frame over the Noise alone on the card (13.4 to 21.3,
three layers), about 2.6 ms a 1080p layer, over Target P1's 1 ms; on the processor 14.4 ms (44.7
to 59.1), about 4.8 ms a layer. This changes the busy run's reading of about 1.1 ms a layer within
Target P1: the busy runs gave 15.6 and 16.6 ms for the frame, this quiet one 21.3. One quiet run,
so a second would show whether the card figure moves this much from run to run.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.7 | 13.4 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | Full | 25.3 | 21.3 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 44.4 | 44.7 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | Full | 58.8 | 59.1 |
