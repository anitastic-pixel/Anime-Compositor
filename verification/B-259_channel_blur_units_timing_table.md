# B-259: frame times before and after, Channel Blur's Units

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a (which includes B-259), with
B-223's `b223_gpu_blurs_timing` in `tests/b223_gpu_blurs.rs`, unchanged by B-259, built with
`cargo test --release --test b223_gpu_blurs --no-run` and run once with
`--ignored b223_gpu_blurs_timing`. Before the run `tasklist` showed no cargo, rustc or other test
program, and the card was idle (0% busy; 3.0 GB of its memory held by the desktop); the same after
it. The owner's app was not open. One round, single figures, not medians of rounds. Only the
current build ("after") was measured again; the "before" build (6ae3348) was not rebuilt.

Earlier provisional figures (2026-10-09, another lane's build running throughout; medians of
three rounds), Channel Blur, again: before 22.5, after 21.8 ms; first: before 30.7, after 29.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed** is B-223's: the reference shot, every eighth frame of 240, at Full, with Draw on:
GPU, the blur on the first three layers (the second after a Drop Shadow). **First** is the
median of the first loop's 30 frames, started with empty caches; **Again** the median of the next
seven loops' 210 frames, in ms. The Channel Blur shot (Noise that changes every frame, then
Channel Blur red 12, green 4, blue 0, alpha 6) is saved without units, so it is in Sigma: the rule
an old project keeps. The other five shots do not use Channel Blur.

| Shot | Left to the card | First | Again |
|---|---|---:|---:|
| Noise, then Channel Blur (Sigma) | 2 / 3 / 2 | 26.7 | 23.8 |
| Fast Box Blur 30 x 3 | 1 / 2 / 1 | 13.1 | 11.0 |
| Noise, then Fast Box Blur 30 x 3 | 2 / 3 / 2 | 26.2 | 22.6 |
| Noise, then CC Vector Blur | 2 / 3 / 2 | 47.1 | 44.5 |
| Noise, then Compound Blur | 2 / 3 / 2 | 62.1 | 55.3 |
| Selective Color Blur | 1 / 1 / 1 | 12.4 | 10.7 |

**Reading it:** no change. A Channel Blur in Sigma draws at 23.8 ms a frame again on a quiet
machine, within a couple of milliseconds of both provisional figures (22.5 before, 21.8 after),
still on the card on all three layers, within Target P2. The shots B-259 does not touch also came
out close to their provisional medians. A Channel Blur in Blurriness was not timed: at the same
number its blur is 0.3 times as wide, reaching 6.5 of those sigmas instead of 3, so its kernel is
about two thirds as long as Sigma's.
