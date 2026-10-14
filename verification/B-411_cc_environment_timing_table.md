# B-411: frame times with CC Environment (D-531)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b411_cc_environment --no-run` and run with `--ignored b411_cc_environment_timing`: once on the card, then once with `B411_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2% busy; 3.6 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again, on the card: Noise alone 14.8; Noise, then CC Environment, spherical, filtered 56.9; Noise, then CC Environment, probe 56.9; Noise, then CC Environment, vertical cross 50.8. On the processor: Noise alone 330.3; Noise, then CC Environment, spherical, filtered 725.7; Noise, then CC Environment, probe 184.0; Noise, then CC Environment, vertical cross 102.1 ms.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.1 | 12.4 |
| Noise, then CC Environment, spherical, filtered | Full | 48.9 | 48.2 |
| Noise, then CC Environment, probe | Full | 41.0 | 40.1 |
| Noise, then CC Environment, vertical cross | Full | 41.0 | 39.7 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.9 | 41.2 |
| Noise, then CC Environment, spherical, filtered | Full | 76.3 | 77.2 |
| Noise, then CC Environment, probe | Full | 60.1 | 60.7 |
| Noise, then CC Environment, vertical cross | Full | 59.6 | 60.3 |

CC Environment is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then CC Environment on them with the background layer `layer-4` as the environment, field of view 90; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** On the card, once warm, the three mappings cost about 27 to 36 ms a frame over Noise alone (12.4, three layers), about 9 to 12 ms a 1080p layer, the filtered spherical map (four rays a pixel) the most (48.2); the probe and vertical cross cost about 40. On the processor they cost about 19 to 36 ms a frame (60.3 to 77.2 against 41.2), so here the processor is a little quicker than the card for the probe and the cross, and the same for the filtered spherical map. The busy run's card figures (36 to 42 ms) were a little high; its processor figures could not be read and now can.
