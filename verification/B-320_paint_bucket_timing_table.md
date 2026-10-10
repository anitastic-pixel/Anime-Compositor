# B-320: frame times with Paint Bucket (D-440)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b320_paint_bucket` from 3e94b50d (B-320's code), and run with `--ignored
b320_paint_bucket_timing`: once on the card, then once with `B320_CPU` set on the processor.
The machine was not quiet: two other cargo processes and a b301_light_burst test were running.
The processor's rows jump about (Feather 20 slower than Choke 40 again, and Choke's first loop
2118 ms), so the busy machine shows in them most. To be measured again on a quiet machine.

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
| Noise alone | 39.3 | 50.4 |
| Noise, then Paint Bucket as added | 96.6 | 175.0 |
| Noise, then Paint Bucket, tolerance 30, Feather 20 | 111.4 | 958.3 |
| Noise, then Paint Bucket, Alpha Channel, Choke 40 | 102.5 | 567.8 |

First loops (empty caches): card 44.5, 309.1, 318.5, 297.7; processor 55.2, 185.5, 240.1, 2118.3.

**Reading it.** On the card three Paint Buckets add about 57 to 72 ms a frame, about 20 ms a
layer: the matching, flood fill and distances, made on the processor on either path, then sent up.
The card's first loop is slow (about 300 ms) while its passes are first built.
