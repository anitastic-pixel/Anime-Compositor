# B-395: frame times with Push Iris (D-515) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b395_push_iris` from
dec6b539 (B-395's code), and run with `--ignored b395_push_iris_timing`: once on the card,
then once with `B395_CPU` set on the processor.
**The machine was not quiet.** Eight other cargo processes (other lanes' tests and builds) were
running before and during both runs, with a compiler between them, and six after. The numbers below are recorded as they
came out. They are not a clean measurement of Push Iris: the card's Noise alone took 129.1 ms
a frame here, against 11.8 ms on the quiet machine in B-387's table, and the processor's 140.8 ms
against 57.4 ms in B-390's busy table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Stripe
Sweep, every eighth frame asked for whole at Full. **Again** is the median of the loops after the
first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 129.1 | 140.8 |
| Noise, then Push Iris, completion 50 (as added otherwise) | 102.7 | 218.4 |
| Noise, then Push Iris, completion 50, reversed | 78.7 | 233.6 |
| Noise, then Push Iris, completion 30, Center [20, 70] | 120.1 | 179.5 |

First loops (empty caches): card 160.9, 67.9, 122.0, 172.6; processor 159.4, 206.0, 227.6, 176.4.

**Reading it.** Every shot draws to the end on both. The differences between the shots are
smaller than how much the other tests' use of the machine moved them; the iris is one pass of
the existing `wipe` shader, no blur. Re-run the two commands above when no other cargo, rustc or
test process is running and the card is idle, and replace this table.
