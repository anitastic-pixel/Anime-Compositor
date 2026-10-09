# B-259: frame times before and after, Channel Blur's Units

**PROVISIONAL: the machine was never quiet.** Measured on 2026-10-09. Another lane's
`cargo test` (two cargo processes, a debug build of other tests in a separate folder) was
running before and after every run, and was still running after waiting 30 minutes before the
first run and 5 minutes before each of the others. The sixth run (after) was slowed about three
times across every shot, including the shots B-259 does not touch, by that other work; the
medians below set it aside as the odd one out of three.

The timing test is B-223's `b223_gpu_blurs_timing` in `tests/b223_gpu_blurs.rs`, unchanged by
B-259, built with `cargo test --release --test b223_gpu_blurs` on two builds, each in its own
folder, the two test programs copied aside and run turn about for three rounds, the order turned
each round (before, after; after, before; before, after):

- **before**: 6ae3348, the fixtures commit, the build before B-259.
- **after**: B-259 (023700d).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed** is B-223's: the reference shot, every eighth frame of 240, at Full, with Draw on:
GPU, the blur on the first three layers (the second after a Drop Shadow). **First** is the
median of the first loop's 30 frames, started with empty caches; **Again** the median of the next
seven loops' 210 frames, in ms. The Channel Blur shot (Noise that changes every frame, then
Channel Blur red 12, green 4, blue 0, alpha 6) is saved without units, so it is in Sigma both
before and after: the rule an old project keeps. The other five shots do not use Channel Blur and
show how much the machine moved.

Each figure is the median of the three rounds.

| Shot | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---:|---:|---:|---:|
| Noise, then Channel Blur (Sigma) | 2 / 3 / 2 | 2 / 3 / 2 | 30.7 | 29.4 | 22.5 | 21.8 |
| Fast Box Blur 30 x 3 | 1 / 2 / 1 | 1 / 2 / 1 | 11.9 | 12.4 | 10.5 | 10.5 |
| Noise, then Fast Box Blur 30 x 3 | 2 / 3 / 2 | 2 / 3 / 2 | 26.6 | 32.5 | 22.1 | 22.7 |
| Noise, then CC Vector Blur | 2 / 3 / 2 | 2 / 3 / 2 | 47.1 | 47.7 | 44.3 | 44.2 |
| Noise, then Compound Blur | 2 / 3 / 2 | 2 / 3 / 2 | 53.0 | 52.0 | 52.8 | 52.7 |
| Selective Color Blur | 1 / 1 / 1 | 1 / 1 / 1 | 11.6 | 14.9 | 9.9 | 10.5 |

Each round, Channel Blur, Again: before 24.1, 21.8, 22.5; after 21.8, 21.7, 45.6 (the slowed sixth
run). First: before 26.2, 38.4, 30.7; after 29.4, 24.9, 54.6.

**Reading it:** no change. A Channel Blur in Sigma draws as fast as before (22.5 and 21.8 ms
again, within the spread of the rounds), still on the card on all three layers, within Target P2
as before. A Channel Blur in Blurriness was not timed: at the same number its blur is 0.3 times as
wide, reaching 6.5 of those sigmas instead of 3, so its kernel is about two thirds as long as
Sigma's.
