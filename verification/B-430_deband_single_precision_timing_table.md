# B-430: Deband's blur in single precision on the card (D-550) — PROVISIONAL

Measured on 2026-10-11 with B-336's own timing test, `cargo test --release --test b336_deband
-- --ignored b336_deband_timing`, on the card only (the processor's Deband is not changed),
four runs in turn: the double-precision blur of B-336 (as committed, d14c5ab8), the
single-precision blur of B-430, then each again.
**The machine was not quiet.** Other lanes' cargo runs (two to seven cargo and compiler
processes) were running before and after every run. The first double-precision run is plainly
disturbed (the shot alone took 47.6 ms a frame played again, against 14.4 the first time), so
the comparison below leans on the second pair. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Deband on
each of its first three layers, every eighth frame asked for whole at Full. **Again** is the
median of the seven loops after the first, milliseconds a frame.

| Shot | Double (1st) | Single (1st) | Double (2nd) | Single (2nd) |
|---|---:|---:|---:|---:|
| The shot alone | 47.6 | 15.1 | 14.7 | 12.1 |
| Deband as added | 69.5 | 35.9 | 55.1 | 29.1 |
| Deband blur 30, diffusion 4, noise 4 | 56.1 | 38.0 | 76.5 | 31.6 |

First loops (empty caches): double 14.4, 87.0, 91.2 and 22.8, 62.0, 71.9; single 16.9, 64.3,
42.7 and 12.8, 33.7, 35.6.

**Reading it.** Over the shot alone, in the second pair: as added, 40.4 ms a frame in double
precision (about 13.5 a layer, as B-336's quiet table's 13.7) against 17.0 in single (about 5.7 a
layer); blur 30, 61.8 (about 20.6 a layer, B-336's 21.2) against 19.5 (about 6.5 a layer). The
first single run agrees (6.9 and 7.6 a layer). So the blur in single precision is about two and
a half to three times quicker on this card, which does double-precision sums at 1/64 the rate of
single (D-228); the effect is still over the row's Target P2 (4 ms or less a layer). The rest is
mode 5, which still works the edge weight in double precision where it moves a diffusion offset.
Re-run the four commands above when no other cargo, rustc or test process is running and the
card is idle, and replace this table.
