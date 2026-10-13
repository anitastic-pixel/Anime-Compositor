# B-484: frame times with Liquify (D-605)

Measured on 2026-10-12, built with `cargo test --release --test b484_liquify` from 759a4370
(B-484's code), and run with `--ignored b484_liquify_timing`: once on the card, then once with
`B484_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Other
lanes were building and testing beside it, and before the runs the graphics card had 13.1 GB of its
memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Liquify
as shown, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 66.1 | 99.4 |
| Noise, then Liquify Warp, brush 200, stroked across the middle | 158.0 | 248.2 |
| Noise, then Liquify Twirl Clockwise, brush 600, one dab | 73.1 | 158.2 |

First loops (empty caches): card 85.7, 149.3, 89.7; processor 106.9, 218.7, 185.5.

**Reading it.** Liquify is drawn by D-601's backward field, warp mode 151. On the card the Warp
stroke added about 92 ms a frame over three layers, about 30 ms a 1080p layer, and the single Twirl
dab about 7 ms; with the card's memory nearly full of other programs' work, even the Noise alone ran
nearly four times slower than in B-483's run, so this run cannot say how much is the effect and how
much the machine. On the processor the stroke added about 50 ms a layer and the dab about 20 ms.
