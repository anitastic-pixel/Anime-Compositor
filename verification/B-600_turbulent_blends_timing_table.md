# B-600: Turbulent Noise blending modes, timing (D-730)

**Measured again on a quiet machine** on 2026-10-14 at 3f506d38, built with `cargo test --release --test b600_turbulent_blends --no-run` and run with `--ignored b600_turbulent_blends_timing` on its own: once on the card, then once with `B600_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. The check found the machine quiet at once: before the card run the card was 3% busy and the processor 4%, before the processor run the card 3% and the processor 1%. Both runs finished between the once-a-minute load readings. After the runs the card was 1% and 0% busy and the processor 2% and 3%. One run each. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The owner was using the computer lightly (web browsing) during these runs, as on a normal day.

Earlier figures (2026-10-13, PROVISIONAL, other lanes' tests and cargo ran throughout), Again, card / processor: Noise alone 45.2 / 107.3; as added 50.1 / 221.3; Luminosity 57.0 / 103.2; None at opacity 60 69.0 / 204.0; Stencil Luma 53.8 / 198.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, then Turbulent Noise in each mode; every eighth frame of 240 drawn whole at Full, as the viewer asks. Median milliseconds a frame: the first pass, then the later passes. Produced by `tests/b600_turbulent_blends.rs`, `b600_turbulent_blends_timing` (`--release --ignored`, `B600_CPU` set for the processor).

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.0 | 12.3 |
| Noise, then Turbulent Noise as added | Full | 23.6 | 19.9 |
| the same in Luminosity | Full | 23.7 | 20.1 |
| the same in None at opacity 60 | Full | 34.8 | 30.6 |
| the same in Stencil Luma | Full | 23.3 | 19.9 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 42.3 | 42.1 |
| Noise, then Turbulent Noise as added | Full | 78.6 | 78.3 |
| the same in Luminosity | Full | 79.8 | 79.8 |
| the same in None at opacity 60 | Full | 139.5 | 139.2 |
| the same in Stencil Luma | Full | 80.7 | 80.9 |

## Reading it

- On the card, Turbulent Noise as added costs about 8 ms a frame over the Noise alone (12.3 to 19.9, three layers). Luminosity and Stencil Luma cost the same (20.1 and 19.9).
- None at opacity 60 costs about 11 ms more than the other modes on the card (30.6) and about 60 ms more on the processor (139.2).
- On the processor Turbulent Noise as added costs about 36 ms a frame (42.1 to 78.3); Luminosity and Stencil Luma are within 3 ms of it.
- The card is about 4 times quicker. The earlier figures, which went up and down between modes, were from the busy machine.
