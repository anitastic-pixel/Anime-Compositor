# B-322: frame times with Scribble (D-442)

Measured on 2026-10-10, built with `cargo test --release --test
b322_scribble` from 55123e3d (B-322's code), and run with `--ignored
b322_scribble_timing`: once on the card, then once with `B322_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Scribble over a six-sided mask (mode None) on each, every eighth frame asked for whole
at Full. **Again** is the median of the loops after the first (seven on the card, two on the
processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.0 | 41.3 |
| Noise, then Scribble as added (spacing 5, about 200 lines) | 94.9 | 131.0 |
| Noise, then Scribble stroke 12, spacing 30, curviness 60, Jumpy | 28.5 | 58.0 |
| Noise, then Scribble Centered Edge 80, spacing 2, stroke 2 | 80.0 | 59.2 |

First loops (empty caches): card 15.7, 97.4, 32.1, 79.9; processor 40.7, 130.2, 57.8, 58.5.

**Reading it.** Over Target P2 (4 ms or less added): three Scribbles add about 16 to 83 ms a
frame on the card. The lines are found on the processor, and Path Stroke's `stroke` pass lists
the runs in bands of 16 rows only, so with hundreds of lines crossing every band each pixel
tests every line; with Centered Edge at spacing 2 the card is slower than the processor. Not
changed here, since the pass is shared with Path Stroke: a follow-up (lists per 16 by 16 tile)
would be its own unit.
