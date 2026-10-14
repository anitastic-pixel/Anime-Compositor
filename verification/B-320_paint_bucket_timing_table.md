# B-320: frame times with Paint Bucket (D-440)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b320_paint_bucket --no-run` and run with `--ignored b320_paint_bucket_timing`: once on the card, then once with `B320_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.1 GB to 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 9% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 39.3, processor 50.4; Noise, then Paint Bucket as added card 96.6, processor 175.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Paint Bucket, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.5 | 42.7 |
| Noise, then Paint Bucket as added | 37.7 | 152.4 |
| Noise, then Paint Bucket, tolerance 30, Feather 20 | 34.6 | 185.8 |
| Noise, then Paint Bucket, Alpha Channel, Choke 40 | 33.7 | 156.6 |

First loops (empty caches): card 16.7, 92.0, 101.8, 93.7; processor 41.9, 154.8, 182.0, 151.3.

**Reading it.** On the card three Paint Buckets add about 21 to 25 ms a frame, about 7 to 8 ms a
1080p layer (33.7 to 37.7 against 12.5); on the processor about 37 to 48 ms a layer (152.4 to
185.8 against 42.7). The matching, flood fill and distances are made on the processor on either
path, then sent up, so the card is about five times quicker only because it does the rest. The
busy run's 20 ms a layer on the card, and its slow first loop of about 300 ms, were the machine:
quiet, the first loop is about 92 to 102 ms a frame.
