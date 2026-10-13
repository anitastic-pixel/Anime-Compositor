# B-485: frame times with Reshape (D-606)

Measured on 2026-10-13, built with `cargo test --release --test b485_reshape` from f073e287
(B-485's code), and run with `--ignored b485_reshape_timing`: once on the card, then once with
`B485_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Other
lanes were building and testing beside it, and before the runs the graphics card had 11.0 GB of its
memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Reshape
as shown (the layers' masks: a square, the same square moved 200 right, and a large boundary about
both, all of mode None), every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 45.1 | 205.8 |
| Noise, then Reshape, square to the square moved right | 92.2 | 424.3 |
| Noise, then Reshape, the same within a boundary, Liquid, 60 per cent | 153.5 | 382.4 |

First loops (empty caches): card 90.8, 92.6, 89.9; processor 189.7, 565.4, 418.0.

**Reading it.** Reshape is drawn by D-601's backward field, warp mode 151; its steps are worked on
the processor at each corner of the 256 by 256 grid, every corner blending the pulls of 64 source
points (and 64 boundary points when one is named), before the card draws. On the card the square
pulled right added about 47 ms a frame over three layers, about 16 ms a 1080p layer. The boundary
run came out slower on the card than its first loop and faster than the plain run on the processor,
which the effect itself cannot explain (it does more work, not less, either way): the machine was
busy, so this run cannot say how much is the effect and how much the machine. On the processor the
pull added about 60 to 75 ms a layer.
