# B-433: frame times with Shift Channels (D-553) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b433_shift_channels` from 33d701bd
(B-433's test; no engine code changed), and run with `--ignored b433_shift_channels_timing`: once
on the card, then once with `B433_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo and rustc builds (four to seven processes) were
running during both runs. The numbers are recorded as they came out; a quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers (so nothing is kept), then Shift Channels, every eighth frame of
240 asked for whole at Full. **Again** is the median of the loops after the first (seven on the
card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.5 | 46.9 |
| Noise, then Shift Channels, alpha from luminance | 21.3 | 74.5 |
| Noise, then Shift Channels, red from hue, green from saturation, blue from lightness, alpha from luminance | 30.5 | 62.4 |

First loops (empty caches): card 17.5, 25.5, 32.7; processor 43.3, 80.1, 63.0.

**Reading it.** Every shot draws to the end on both. The processor's numbers do not order
sensibly (the heavier setting came out faster), which is the busy machine showing; the card's
cost, about 8 to 17 ms over the Noise, is also likely inflated. Re-run the two commands above
when no other cargo, rustc or test process is running and replace this table.
