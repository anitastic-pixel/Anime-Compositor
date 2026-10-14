# B-471: frame times with CC Rainfall (D-582)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b471_cc_rainfall --no-run` and run once on the card with `--ignored b471_cc_rainfall_timing`. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (4% busy; 3.7 GB of its memory held by the desktop and open programs); after it the card was 0% busy. The processor was 10% busy before the run and 3% after. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, at 92369655, on a machine that was not quiet, two runs), again: Noise alone 126.2 and 72.5; as added (5000 drops) 55.1 and 46.2; 20000 drops, size 4 48.2 and 62.5; 5000 near drops 38.5 and 38.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 15.9 | 12.3 |
| Noise, then CC Rainfall as added (5000 drops) | Full | 32.0 | 26.2 |
| Noise, then CC Rainfall, 20000 drops, size 4 | Full | 57.7 | 43.5 |
| Noise, then CC Rainfall, 5000 near drops (depth 500), size 6 | Full | 32.6 | 26.8 |

CC Rainfall is new, so there is no "before" build: the "Noise alone" row is the comparison.

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, and on the first layer CC Rainfall after the Noise; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks, Draw on: GPU. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the seven loops after it. Milliseconds a frame.

**Reading it.** On the card, once warm, CC Rainfall as added (5000 drops) costs about 14 ms a frame over the Noise alone (26.2 against 12.3), 20000 drops at size 4 about 31 ms (43.5), and 5000 near drops at size 6 about the same as added (26.8): the cost follows the number of drops, not their size or depth. This settles the busy run's puzzle: there the shots with rain came out faster than the shot without it, which was the other lanes' builds; quiet, rain adds a clear, steady cost. Every shot here but 20000 drops plays in real time (41.7 ms at 24 a second); 20000 drops (43.5) just misses it.
