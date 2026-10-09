# B-260: frame times with Colorama's remaining controls (D-381)

Measured on 2026-10-09. "Before" is a build of 0a97950 (the fixtures commit, D-316's Colorama
only), in its own worktree and target folder; "after" is the code with 92aa96b's speed fix. The
timing test is `b260_colorama_timing` in `tests/b260_colorama.rs` (the before build carries the
same test, its first two shots only), built with `cargo test --release` and run with
`--ignored b260_colorama_timing`; with `B260_CPU` set the processor draws.

**Quiet.** A round counts as quiet when `tasklist` showed no cargo or rustc process for 30
seconds before it and none after it. The card rounds below and the processor's after round were
quiet start and end; the processor's before round started quiet and a build (the other lane's)
had started by its end, so its figures may be a little high. The owner's app was not checked.
Rounds taken while builds ran (card 13.0 to 14.9 ms for Noise alone, processor 44 to 106 ms)
are left out, not averaged in.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the Colorama, every
eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card before | Card after | Processor before | Processor after |
|---|---:|---:|---:|---:|
| Noise alone | 12.1 | 12.2 | 42.2 | 42.5 |
| Noise, then Colorama as a file from before D-381 has it | 14.6 | 15.3 | 56.1 | 54.0 |
| Noise, then Colorama, modify hue, matching by chroma | | 20.8 | | 66.7 |
| Noise, then Colorama, modify alpha, opacities, composite off | | 20.8 | | 57.3 |
| Noise, then Colorama masked by layer4 (on the processor by design) | | 77.1 | | 75.8 |

An old file's Colorama costs what it did: about 0.8 to 1.0 ms a layer on the card (within the
rounds' spread) and about 4 to 5 ms a layer on the processor. Before 92aa96b the processor's
figure for an old file was about 7 ms a layer (an extra sRGB step on every pixel); that build was
never pushed alone. A masked Colorama reads another layer's pixels, so it stays on the processor
(B-222's rule), which is why its card figure is close to the processor's.

## Raw: processor, after

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 41.8 | 42.5 |
| Noise, then Colorama as a file from before D-381 has it | Full | 53.1 | 54.0 |
| Noise, then Colorama, modify hue, matching by chroma | Full | 66.2 | 66.7 |
| Noise, then Colorama, modify alpha, opacities, composite off | Full | 57.4 | 57.3 |
| Noise, then Colorama masked by layer4 (on the processor) | Full | 74.7 | 75.8 |

## Raw: processor, before

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 41.1 | 42.2 |
| Noise, then Colorama as a file from before D-381 has it | Full | 55.1 | 56.1 |
