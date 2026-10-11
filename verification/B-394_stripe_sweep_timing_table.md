# B-394: frame times with Stripe Sweep (D-514) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b394_stripe_sweep` from
ef25bca1 (B-394's code), and run with `--ignored b394_stripe_sweep_timing`: once on the card,
then once with `B394_CPU` set on the processor.
**The machine was not quiet.** Six other cargo processes (other lanes' tests and builds) were
running before and during both runs, and four after. The numbers below are recorded as they
came out. They are not a clean measurement of Stripe Sweep: the card's Noise alone took 76.2 ms
a frame here, against 11.8 ms on the quiet machine in B-387's table, and the processor's 143.5 ms
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
| Noise alone | 76.2 | 143.5 |
| Noise, then Stripe Sweep, completion 50 (as added otherwise) | 101.0 | 152.1 |
| Noise, then Stripe Sweep, completion 50, thickness 4, slant 0, direction 30 | 116.8 | 147.1 |
| Noise, then Stripe Sweep, completion 50, flipped, thickness 200, slant 99 | 141.2 | 146.3 |

First loops (empty caches): card 173.3, 118.7, 162.0, 149.9; processor 151.9, 246.7, 150.9, 121.0.

**Reading it.** Every shot draws to the end on both. The differences between the shots are
smaller than how much the other tests' use of the machine moved them; the sweep is one pass of
the existing `wipe` shader, no blur. Re-run the two commands above when no other cargo, rustc or
test process is running and the card is idle, and replace this table.
