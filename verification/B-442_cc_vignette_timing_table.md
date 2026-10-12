# B-442: frame times with CC Vignette (D-562) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b442_cc_vignette` from a92c6fcb, and
run with `--ignored b442_cc_vignette_timing`: once on the card, then once with `B442_CPU` set on the
processor. **The machine was not quiet.** Other lanes' `b155_gpu_chain` and
`b352_gaussian_blur_legacy` tests and rustc builds were running during both runs. The numbers are
recorded as they came out; a quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers (so nothing is kept), then CC Vignette, every eighth frame of 240
asked for whole at Full. **Again** is the median of the loops after the first (seven on the card,
two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 22.9 | 62.3 |
| Noise, then CC Vignette as added | 27.9 | 208.7 |
| Noise, then CC Vignette, amount -60, angle of view 150, pin highlights 50 | 21.1 | 226.1 |

First loops (empty caches): card 39.8, 34.8, 35.1; processor 55.3, 161.3, 203.2.

**Reading it.** Every shot draws to the end on both. On the card CC Vignette rides in the `tone`
pass as tone mode 110 and its cost does not show above the busy machine's spread (one shot came
out faster than the Noise alone). The processor's 146 to 164 ms over the Noise is far more than a
per-pixel gain should cost and is likely inflated by the other lanes' work. Re-run the two
commands above when no other cargo, rustc or test process is running and replace this table.
