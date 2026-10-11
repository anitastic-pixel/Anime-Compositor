# B-432: frame times with Pass Extract's alpha and UnMult (D-552) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b432_extractor_alpha` from 31dddc4d
(B-432's code), and run with `--ignored b432_extractor_alpha_timing`: once on the card, then once
with `B432_CPU` set on the processor.
**The machine was not quiet.** Another lane's `b155_gpu_chain` run and two cargo processes were
running during both runs. The numbers are recorded as they came out; a quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** One layer, a 1920 by 1080 EXR still laid out as Blender writes it with a soft
`ViewLayer.Combined.A` (written by the test into the system's temporary folder), the effect then
a Noise that changes every frame (so nothing is kept), every eighth frame of 240 asked for whole
at Full. **Again** is the median of the loops after the first (seven on the card, two on the
processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 2.1 | 20.9 |
| Pass Extract, four names, UnMult off, then Noise | 2.1 | 22.4 |
| Pass Extract, four names, UnMult on, then Noise | 2.2 | 22.2 |
| Pass Extract, three names, then Noise | 2.1 | 23.1 |

First loops (empty caches): card 5.7, 5.2, 5.4, 5.5; processor 20.2, 22.2, 21.9, 22.5.

**Reading it.** Every shot draws to the end without a warning on both. On the card the alpha and
UnMult cost nothing measurable, as B-229's three-name Pass Extract did (2.1 against 2.1). On the
processor the extract adds about 1.5 to 2 ms, the same with four names as with three. Re-run the
two commands above when no other cargo, rustc or test process is running and replace this table.
