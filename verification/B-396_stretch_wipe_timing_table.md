# B-396: frame times with Stretch Wipe (D-516) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b396_stretch_wipe` from
a34e2ddf (B-396's code), and run with `--ignored b396_stretch_wipe_timing`: once on the card,
then once with `B396_CPU` set on the processor.
**The machine was not quiet.** Eight other cargo and compiler processes (other lanes' tests and
builds) were running before the card's run, six between the two runs, and two after. The numbers
below are recorded as they came out. They are not a clean measurement of Stretch Wipe: the
card's Noise alone took 20.2 ms a frame here, against 11.8 ms on the quiet machine in B-387's
table, and the processor's 56.2 ms against 57.4 ms in B-390's busy table. A quiet re-measure is
owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Stretch
Wipe, every eighth frame asked for whole at Full. **Again** is the median of the loops after the
first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 20.2 | 56.2 |
| Noise, then Stretch Wipe, scale 300 (as added otherwise) | 27.9 | 183.8 |
| Noise, then Stretch Wipe, scale 100000, direction 90 | 29.6 | 76.7 |
| Noise, then Stretch Wipe, scale 175, direction -135, Center [60, 40] | 77.5 | 143.1 |

First loops (empty caches): card 29.1, 38.0, 25.8, 90.8; processor 144.1, 83.8, 133.2, 137.4.

**Reading it.** Every shot draws to the end on both. The differences between the shots are
smaller than how much the other tests' use of the machine moved them; the stretch is one pass of
the existing `wipe` shader, no blur. Re-run the two commands above when no other cargo, rustc or
test process is running and the card is idle, and replace this table.
