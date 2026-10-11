# B-402: frame times with Cryptomatte (D-522)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b402_cryptomatte` from 0fd4faf1 (B-402's code), and run once with `--ignored b402_cryptomatte_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. Two other lanes' tests (b155_gpu_chain, b107_gpu_fx) were running when the run started and when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 10.5 | 3.3 |
| Cryptomatte, Colors, then Noise | Full | 8.8 | 3.3 |
| Cryptomatte, Matte of two objects, then Noise | Full | 8.3 | 3.7 |
| Cryptomatte, Keyed on two objects, then Noise | Full | 6.0 | 3.9 |

Cryptomatte is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and a CryptoObject layer of two levels, six objects in blocks, each pixel two of them at 0.7 and 0.3), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Cryptomatte, then a Noise that changes every frame, so no finished frame is kept; the decoded matte is kept by the pass cache as D-348's passes are. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** Once the file has been read, a frame took 3.3 to 3.9 milliseconds with Cryptomatte against 3.3 for Noise alone: the colours or the matte are worked out once as the file is read and kept with the file's other passes, so a frame only lays them on. The first loop's figures fall from 10.5 (Noise alone, run first) to 6.0 because each shot after the first starts with the program already warmed up; they are not a sign that Cryptomatte makes frames faster. Two other lanes' tests (b155_gpu_chain and b107_gpu_fx) were running from start to end, so the numbers are PROVISIONAL and should be measured again on a quiet machine before any speed claim.
