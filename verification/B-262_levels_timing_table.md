# B-262: frame times with Levels' sets for each channel (D-383)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b262_levels_individual --no-run` and run with `--ignored b262_levels_individual_timing`: once on the card, then once with `B262_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds. Only the current build was measured again; the "before" build was not rebuilt, so its busy figures are left to the history in git.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), after build, again: Noise alone card 14.7 and 18.0, processor 44.3 and 41.8; Noise, then Levels as saved before, five settings card 16.6 and 15.2, processor 64.1 and 56.5; Noise, then Levels (Individual Controls), RGB, red, green and blue sets card 17.6 and 15.7, processor 66.3 and 61.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-261's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card, again | Processor, again |
|---|---:|---:|
| Noise alone | 13.4 | 44.3 |
| Noise, then Levels as saved before, five settings | 16.7 | 60.2 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | 17.2 | 64.2 |

**Reading it.** A Levels saved before adds 3.3 ms a frame over the Noise alone on the card (13.4 to
16.7, three layers), about 1.1 ms a 1080p layer, and 15.9 ms on the processor (about 5.3 ms a
layer). The same as the busy run's "before" build (2.7 and 3.2 ms on the card), so the colour pass
carrying four sets costs a Levels saved before nothing measurable. Four sets at once add 3.8 ms on
the card (about 1.3 ms a layer, a little over Target P1's 1 ms) and 19.9 ms on the processor
(about 6.6 ms a layer). The busy run could not separate these from the machine's noise; the quiet
one can.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.6 | 13.4 |
| Noise, then Levels as saved before, five settings | Full | 21.2 | 16.7 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 20.9 | 17.2 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 43.9 | 44.3 |
| Noise, then Levels as saved before, five settings | Full | 59.0 | 60.2 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 65.1 | 64.2 |
