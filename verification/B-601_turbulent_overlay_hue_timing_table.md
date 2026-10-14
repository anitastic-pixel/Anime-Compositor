# B-601: Turbulent Noise Overlay and Hue, timing (D-731)

**Measured again on a quiet machine** on 2026-10-14 at 3f506d38, built with `cargo test --release --test b601_turbulent_overlay_hue --no-run` and run with `--ignored b601_turbulent_overlay_hue_timing` on its own: once on the card, then once with `B601_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. The check found the machine quiet at once: before both runs the card was 0% busy and the processor 1%. Both runs finished between the once-a-minute load readings. After the runs the card was 0% busy and the processor 3% and 2%. One run each. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The owner was using the computer lightly (web browsing) during these runs, as on a normal day.

Earlier figures (2026-10-13, PROVISIONAL, another lane's test and cargo ran throughout), Again, card / processor: Noise alone 43.3 / 115.0; as added 55.0 / 220.9; Overlay 53.4 / 111.4; Hue 48.0 / 157.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, then Turbulent Noise as added, in Overlay and in Hue; every eighth frame of 240 drawn whole at Full, as the viewer asks. Median milliseconds a frame: the first pass, then the later passes. Produced by `tests/b601_turbulent_overlay_hue.rs`, `b601_turbulent_overlay_hue_timing` (`--release --ignored`, `B601_CPU` set for the processor).

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 15.5 | 12.1 |
| Noise, then Turbulent Noise as added | Full | 24.5 | 19.5 |
| the same in Overlay | Full | 23.1 | 19.6 |
| the same in Hue | Full | 23.4 | 19.7 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 41.5 | 41.8 |
| Noise, then Turbulent Noise as added | Full | 77.5 | 77.6 |
| the same in Overlay | Full | 77.9 | 77.7 |
| the same in Hue | Full | 77.6 | 78.4 |

## Reading it

- On the card, Turbulent Noise as added costs about 7 ms a frame over the Noise alone (12.1 to 19.5, three layers). Overlay and Hue cost the same (19.6 and 19.7).
- On the processor it costs about 36 ms a frame (41.8 to 77.6), and Overlay and Hue are within 1 ms of it.
- The card is about 4 times quicker. The earlier figures, which went up and down between modes, were from the busy machine.
