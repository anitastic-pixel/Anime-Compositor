# B-499: frame times with Particle Playground on the card (D-619) — PROVISIONAL, NOT USABLE

Measured on 2026-10-13, built with `cargo test --release --test b493_particle_playground` from
621dcd9d (B-499's code), and run with `--ignored b493_particle_playground_timing`.
**The machine was busy, and so was the graphics card.** Other lanes' tests ran through the
whole measurement: `b532_warp_stabilizer`, `b557_foam`, and two `b155_gpu_chain` runs, which
draw on the same graphics card. The shot with no effect, which draws no particles, took about
twice as long on the card as on the processor (207 to 227 ms against 109 to 116). That points at
the other tests sharing the card, not at the particles. These numbers do not say whether the
card is faster. A quiet re-measure is owed.

A first run, made while this lane's own `b155_gpu_chain` was running too, was worse still (the
shot with no effect from 324 to 960 ms a frame). It is not used.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle
Playground on its first layer. Each listed frame is drawn whole at Full, three times, and the
table gives the median of the three, in milliseconds. Every draw starts with empty caches: a new
frame cache each time, and the card's kept pictures dropped (`gpu.forget()`) before each card
draw. Each shot first draws one frame on the card, not timed, so the card's passes are built.
"card" is the frame the page receives from the card; "processor" is the same frame drawn on the
processor. On the card, the particles are still worked out on the processor and only the
drawing is on the card. The last shot now explodes layer 2. Before B-499 it named a layer "a",
which the reference shot does not have, so it timed nothing.

| Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---|---:|---:|---:|---:|
| no effect | card | 226.7 ms | 205.9 ms | 211.6 ms | 207.4 ms |
| no effect | processor | 113.4 ms | 115.3 ms | 108.7 ms | 116.4 ms |
| Particle Playground as added | card | 208.7 ms | 216.3 ms | 201.0 ms | 205.6 ms |
| Particle Playground as added | processor | 125.2 ms | 136.6 ms | 134.0 ms | 135.8 ms |
| fireworks: 10 shells a second, each bursting into 16 sparks | card | 118.8 ms | 184.1 ms | 123.1 ms | 87.9 ms |
| fireworks: 10 shells a second, each bursting into 16 sparks | processor | 85.2 ms | 67.4 ms | 70.8 ms | 93.1 ms |
| the layer exploded into 16-pixel pieces, falling | card | 153.6 ms | 145.3 ms | 118.8 ms | 152.2 ms |
| the layer exploded into 16-pixel pieces, falling | processor | 77.0 ms | 99.7 ms | 90.7 ms | 87.8 ms |

**Reading it.** The card was slower in every shot, including the one with no effect at all, so
the gap measures the other tests sharing the card, not Particle Playground. Re-run the command
above when no other cargo, rustc or test process is running, and replace this table.
