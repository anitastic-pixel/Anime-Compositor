# B-397: frame times with Twister (D-517) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b397_twister` from 7fc9c9fe
(B-397's code), and run with `--ignored b397_twister_timing`: once on the card, then once with
`B397_CPU` set on the processor.
**The machine was not quiet.** Another lane's cargo run (two cargo processes) was running before
the card's run, between the two runs and after them. The numbers below are recorded as they came
out. They are not a clean measurement of Twister: the card's Noise alone took 64.5 ms a frame
here, against 11.8 ms on the quiet machine in B-387's table, and the processor's 113.3 ms against
57.4 ms in B-390's busy table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Twister,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 64.5 | 113.3 |
| Noise, then Twister, completion 50, layer 4 on the back (as added otherwise) | 168.6 | 204.4 |
| Noise, then Twister, completion 50, axis 30, shading off, nothing on the back | 57.6 | 139.1 |
| Noise, then Twister, completion 75, Center [30, 60], layer 4 on the back | 200.2 | 184.9 |

First loops (empty caches): card 152.6, 157.0, 72.2, 193.1; processor 121.6, 246.5, 146.4, 184.9.

**Reading it.** Every shot draws to the end on both. The two shots with layer 4 on the back are
the slow ones on the card as on the processor: they draw layer 4 a second time each frame, as
D-189's map, before the one `twist` pass; with nothing on the back the card's figure is the
Noise's own. How much of that is the effect and how much the other run's use of the card is not
known from this table. Re-run the two commands above when no other cargo, rustc or test process
is running and the card is idle, and replace this table.
