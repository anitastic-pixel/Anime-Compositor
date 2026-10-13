# B-486: frame times with Mask Smear (D-607)

Measured on 2026-10-13, built with `cargo test --release --test b486_mask_smear` from 33ea89bc
(B-486's code), and run with `--ignored b486_mask_smear_timing`: once on the card, then once with
`B486_CPU` set on the processor. **These numbers are PROVISIONAL**: other lanes were building and
testing beside it, and before the runs the graphics card had 5.3 GB of its memory in use by other
programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Mask Smear
as shown (the layers' masks: a square, and a large boundary about it, both of mode None), every
eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 15.4 | 50.2 |
| Noise, then Mask Smear, the square carried 200 right | 52.9 | 91.1 |
| Noise, then Mask Smear, turned 30, scaled to 120, within a boundary, Liquid, 60 per cent | 36.7 | 85.1 |

First loops (empty caches): card 19.9, 49.3, 40.9; processor 51.3, 88.4, 82.2.

**Reading it.** Mask Smear is drawn as Reshape is, through D-601's backward field, warp mode 151:
its steps are worked on the processor at each corner of the 256 by 256 grid, every corner blending
the pulls of 64 source points (and 64 boundary points when one is named), before the card draws.
On the card the carried square added about 37 ms a frame over three layers, about 12 ms a 1080p
layer; on the processor about 41 ms a frame, about 14 ms a layer. The boundary run came out faster
than the plain run on both, though it does more work; the plain run's later loops were also slower
than its first. The machine was busy, so these runs cannot say how much is the effect and how much
the machine. All of these are well below B-485's Reshape numbers, which were measured while the
machine was busier still (11.0 GB of the card in use by others).
