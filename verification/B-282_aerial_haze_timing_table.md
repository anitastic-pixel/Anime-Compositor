# B-282: frame times with Aerial Haze (D-403)

**Both rounds on a quiet machine.** Measured on 2026-10-09, built with
`cargo test --release --test b282_aerial_haze` from e88e2646 (B-282's code), and run with
`--ignored b282_aerial_haze_timing`: one round on the card, then one with `B282_CPU` set on the
processor. `tasklist` showed no other cargo, rustc or test program before, between and after
the two rounds.

Noise alone is the shot without Aerial Haze, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card (quiet) | Processor (quiet) |
|---|---:|---:|
| Noise alone | 12.0 | 43.2 |
| Noise, then Aerial Haze as added (pale sky blue, 30, even) | 13.9 | 45.8 |
| Noise, then Aerial Haze at 80 through layer 4 as the matte | 44.3 | 66.8 |

First loops (empty caches): card 15.3, 17.3, 42.1; processor 43.3, 44.2, 66.2.

**Reading it.** Evenly, on the card Aerial Haze adds about 1.9 ms over three layers: about
0.6 ms a 1080p layer, inside Target P1 (1 ms a 1080p layer on the card); on the processor about
0.9 ms a layer. Through a matte layer it adds about 32 ms over three layers on the card (about
10.8 ms a layer) and about 24 ms on the processor (about 7.9 ms a layer). That cost is the matte
layer itself, drawn on the processor and sent to the card before the haze pass reads it (D-189's
path for a layer setting); the haze pass is the same pass as the even case. Gradient Wipe, which
reads its map the same way, shows the same (43.7 ms in B-226's timing). Not inside P1 with a
matte layer.
