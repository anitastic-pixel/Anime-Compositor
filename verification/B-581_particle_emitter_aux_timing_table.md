# B-581: frame times with Particle Emitter's second system (D-711) — NOT USABLE

Measured on 2026-10-13, built with `cargo test --release --test b581_particle_emitter_aux` from
869c761f (B-581's code), and run with `--ignored b581_particle_emitter_aux_timing`. **The machine
was not quiet:** another lane's `b155_gpu_chain` test, which draws on the same graphics card, and
two cargo processes ran through the measurement. The numbers below show it: the shot with no
effect is slower than the shots with effects, and the card slower than the processor. They are
recorded as measured and must not be read as costs. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080) with Particle Emitter on its first layer,
frames 0, 60, 120 and 239 each drawn with empty caches, three times, the median, by the card and
by the processor.

| Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---|---:|---:|---:|---:|
| no effect | card | 174.4 ms | 157.1 ms | 158.1 ms | 161.9 ms |
| no effect | processor | 85.2 ms | 98.5 ms | 92.7 ms | 84.9 ms |
| Particle Emitter as added (second system off) | card | 179.0 ms | 174.1 ms | 141.6 ms | 132.1 ms |
| Particle Emitter as added (second system off) | processor | 107.7 ms | 47.3 ms | 59.0 ms | 43.4 ms |
| as added, each trailing 30 a second living 1 s | card | 75.1 ms | 134.6 ms | 166.3 ms | 162.1 ms |
| as added, each trailing 30 a second living 1 s | processor | 105.0 ms | 120.6 ms | 119.3 ms | 125.2 ms |
| 200 a second bouncing on a floor, 10 sparks a bounce | card | 158.6 ms | 152.8 ms | 170.5 ms | 170.3 ms |
| 200 a second bouncing on a floor, 10 sparks a bounce | processor | 103.1 ms | 128.9 ms | 125.5 ms | 124.5 ms |
