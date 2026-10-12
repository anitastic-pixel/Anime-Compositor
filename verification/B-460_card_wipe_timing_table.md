# B-460: frame times with Card Wipe (D-580)

Measured on 2026-10-11, built with `cargo test --release --test b460_card_wipe` from 3f366efc (B-460's code), and run with `--ignored b460_card_wipe_timing`, once on the card and once with `B460_CPU` set. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11; quiet but for 2 cargo or rustc processes counted before the runs (while the release build compiled), 0 between them and 0 after. The owner's app was not touched.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.6 | 12.5 |
| Noise, then Card Wipe, completion 50, 8 by 8 | Full | 70.1 | 71.4 |
| Noise, then Card Wipe, camera turned, point light | Full | 70.8 | 74.8 |
| Noise, then Card Wipe, 100 by 100 cards | Full | 84.4 | 83.0 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.2 | 40.8 |
| Noise, then Card Wipe, completion 50, 8 by 8 | Full | 114.9 | 118.9 |
| Noise, then Card Wipe, camera turned, point light | Full | 124.6 | 128.2 |
| Noise, then Card Wipe, 100 by 100 cards | Full | 138.8 | 134.9 |

Card Wipe is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then Card Wipe on them (each with the shot's background as its back); every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** Noise alone took 12.5 ms a frame once warm on the card, no slower than B-411's 14.8 ms, so the machine was taken as quiet. On the card, once warm, Card Wipe on three layers costs about 59 ms a frame over Noise alone at completion 50 with 8 by 8 cards, about 62 ms with the camera turned and a point light, and about 70 ms with 100 by 100 cards, about 20 to 23 ms a layer; with it the frame is slower than real time (41.7 ms at 24 a second). On the processor it costs about 78 to 94 ms a frame over Noise alone. The card pass traces each pixel's ray through the cards it can reach in double precision, which this card runs slowly; a faster card pass would be a separate piece of work.
