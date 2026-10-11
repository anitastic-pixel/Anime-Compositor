# B-431: frame times with CC Ball Action (D-551) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b431_cc_ball_action` from c1c31a65
(B-431's code), and run with `--ignored b431_cc_ball_action_timing`: once on the card, then once
with `B431_CPU` set on the processor.
**The machine was not quiet.** Other lanes' two `b155_gpu_chain` runs and four cargo processes
were running during both runs. The numbers below are recorded as they came out. They are not a
clean measurement of CC Ball Action: the card's Noise alone took 45.0 ms a frame here, against
11.8 ms on the quiet machine in B-387's table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the CC Ball
Action, every eighth frame asked for whole at Full. **Again** is the median of the loops after
the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 45.0 | 102.1 |
| Noise, then CC Ball Action as added | 76.5 | 246.8 |
| Noise, then CC Ball Action, grid spacing 4, rotation 40 about XY | 68.5 | 280.7 |
| Noise, then CC Ball Action, scatter 60, ball size 300 | 76.9 | 311.5 |

First loops (empty caches): card 67.5, 141.0, 142.7, 121.8; processor 100.8, 208.9, 313.5, 310.9.

**Reading it.** Every shot draws to the end on both. On the card each layer adds about 8 to 11 ms,
inside the row's Target P5; part of it is placing and ordering the balls on the processor before
the two passes. The processor lays every ball one after another before lighting the pixels, so
the large balls of the last shot cost most there. How much of each number is the effect and how
much the other runs' use of the machine is not known from this table. Re-run the two commands
above when no other cargo, rustc or test process is running and the card is idle, and replace
this table.
