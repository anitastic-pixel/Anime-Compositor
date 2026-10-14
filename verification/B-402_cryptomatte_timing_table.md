# B-402: frame times with Cryptomatte (D-522)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b402_cryptomatte --no-run` and run once with `--ignored b402_cryptomatte_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 3.3; Cryptomatte, Colors, then Noise 3.3 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.5 | 2.0 |
| Cryptomatte, Colors, then Noise | Full | 5.6 | 2.1 |
| Cryptomatte, Matte of two objects, then Noise | Full | 5.5 | 2.1 |
| Cryptomatte, Keyed on two objects, then Noise | Full | 4.5 | 2.0 |

Cryptomatte is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and a CryptoObject layer of two levels, six objects in blocks, each pixel two of them at 0.7 and 0.3), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Cryptomatte, then a Noise that changes every frame, so no finished frame is kept; the decoded matte is kept by the pass cache as D-348's passes are. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** Once the file has been read, a frame took 2.0 to 2.1 milliseconds with Cryptomatte against 2.0 for Noise alone: the colours or the matte are worked out once as the file is read and kept with the file's other passes, so a frame only lays them on, and the cost is too small to measure here. The first loop's figures (4.5 to 5.6) include reading the file and warming up. That is the busy run's reading; its figures were higher (3.3 to 3.9) because two other lanes' tests were running.
