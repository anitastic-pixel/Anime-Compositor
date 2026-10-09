# B-262: frame times with Levels' sets for each channel (D-383)

**PROVISIONAL: the machine was not quiet.** Measured on 2026-10-09 on the code commit b5cc01f
("after"), against 104a03f, the commit before it ("before"), each built with
`cargo test --release` and run with `--ignored b262_levels_individual_timing`, copies of the two
test binaries run one after the other: before on the card, before on the processor (`B262_CPU`
set), after on the card, after on the processor, then the four again. `tasklist` showed 2 cargo
or rustc processes (another lane building) at the start of five of the eight runs, so the
figures move by several milliseconds from round to round (Noise alone, the same shot in both
builds, reads 41.8 to 58.2 ms on the processor). The owner's app was open.

The "before" build has no Levels (Individual Controls): its third row draws Noise alone with a
kind it does not know, so it is shown only for completeness and is not a comparison.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-261's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card before, r1 / r2 | Card after, r1 / r2 | Processor before, r1 / r2 | Processor after, r1 / r2 |
|---|---:|---:|---:|---:|
| Noise alone | 12.2 / 14.3 | 14.7 / 18.0 | 43.8 / 58.2 | 44.3 / 41.8 |
| Noise, then Levels as saved before, five settings | 14.9 / 17.5 | 16.6 / 15.2 | 61.9 / 76.2 | 64.1 / 56.5 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | 12.6 / 14.6 | 17.6 / 15.7 | 47.5 / 58.9 | 66.3 / 61.1 |

A Levels saved before, on the card: 2.7 and 3.2 ms over Noise alone before, 1.9 and -2.8 after,
inside the machine's noise; the colour pass now carries four sets but skips the three left plain.
On the processor the old Levels costs 18.1 and 18.0 ms over Noise alone before, 19.8 and 14.7
after, the same within the noise. Four sets at once: 2.9 and -2.3 ms over Noise alone on the card,
inside the noise and under 1 ms a layer, within Target P1; 22.0 and 19.3 ms on the processor. To be measured again
on a quiet machine.

## Raw

### Round 1, before, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.0 | 12.2 |
| Noise, then Levels as saved before, five settings | Full | 19.5 | 14.9 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 15.9 | 12.6 |

### Round 2, before, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.6 | 14.3 |
| Noise, then Levels as saved before, five settings | Full | 22.0 | 17.5 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 16.4 | 14.6 |

### Round 1, before, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 42.6 | 43.8 |
| Noise, then Levels as saved before, five settings | Full | 61.5 | 61.9 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 46.8 | 47.5 |

### Round 2, before, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 56.9 | 58.2 |
| Noise, then Levels as saved before, five settings | Full | 74.3 | 76.2 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 57.8 | 58.9 |

### Round 1, after, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 20.9 | 14.7 |
| Noise, then Levels as saved before, five settings | Full | 21.7 | 16.6 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 20.5 | 17.6 |

### Round 2, after, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 24.0 | 18.0 |
| Noise, then Levels as saved before, five settings | Full | 25.9 | 15.2 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 18.9 | 15.7 |

### Round 1, after, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 42.5 | 44.3 |
| Noise, then Levels as saved before, five settings | Full | 67.1 | 64.1 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 67.7 | 66.3 |

### Round 2, after, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.9 | 41.8 |
| Noise, then Levels as saved before, five settings | Full | 56.3 | 56.5 |
| Noise, then Levels (Individual Controls), RGB, red, green and blue sets | Full | 60.7 | 61.1 |
