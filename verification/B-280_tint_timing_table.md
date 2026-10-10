# B-280: frame times with Tint (D-401)

**The card's round on a quiet machine; the processor's round PROVISIONAL.** Measured on
2026-10-09, built with `cargo test --release --test b280_tint` from 28e8e0d1 (B-280's code), and
run with `--ignored b280_tint_timing`: one round on the card, then one with `B280_CPU` set on the
processor. `tasklist` showed no other cargo or rustc before and between the two rounds; when the
processor's round ended, the other lane's cargo and rustc were running, so it may have shared the
machine for part of its run, and its numbers are marked PROVISIONAL.

The older Tint (one colour and an amount) is unchanged and not timed here; Noise alone is the shot
without Tint, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card (quiet) | Processor (PROVISIONAL) |
|---|---:|---:|
| Noise alone | 12.4 | 41.8 |
| Noise, then Tint as added (black to white, amount 100) | 14.2 | 51.3 |
| Noise, then Tint, navy to gold at amount 30 | 13.7 | 51.8 |

First loops (empty caches): card 16.5, 18.2, 20.0; processor 41.6, 51.5, 52.1.

**Reading it.** On the card Tint adds about 1.3 to 1.8 ms over three layers: about 0.4 to 0.6 ms
a 1080p layer, inside Target P1 (1 ms a 1080p layer on the card). On the processor about 3.2 to 3.3 ms a layer (PROVISIONAL).
