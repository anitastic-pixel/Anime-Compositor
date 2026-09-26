# B-48: frame times with D-40's 1 GiB and with Automatic, on the CPU and the card

Written by `tests/b48_memory.rs` (`cargo test --release --test b48_memory -- --ignored`).

- Machine memory: 66.1 GB; Automatic gives the viewer 16.5 GB
- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory; it may hold 8.4 GB of drawings
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with B-47's three Blooms, every frame asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. On the CPU the three blooms run inside planning; on the card the card runs them, and the Gaussian Blur before one of them stays on the CPU. Both keep what they read and what the CPU's effects make in the viewer's cache, whose size is the Memory column. Each row starts with empty caches and plays the shot twice: the first loop fills them, the second is what playing it again costs. Medians over all 240 frames, in ms, and what the viewer's cache held at the end.

| Quality | Memory | Drawn on | First loop | Again | Held at the end |
|---|---|---|---:|---:|---:|
| Draft | 1 GiB (before) | CPU | 17.2 | 17.4 | 0.7 GB |
| Draft | 1 GiB (before) | GPU | 17.3 | 17.4 | 0.7 GB |
| Draft | Automatic | CPU | 4.8 | 4.7 | 1.9 GB |
| Draft | Automatic | GPU | 4.2 | 4.2 | 1.9 GB |
| Full | 1 GiB (before) | CPU | 172.0 | 171.0 | 1.1 GB |
| Full | 1 GiB (before) | GPU | 27.9 | 28.2 | 1.0 GB |
| Full | Automatic | CPU | 16.9 | 16.7 | 3.3 GB |
| Full | Automatic | GPU | 3.6 | 3.5 | 2.7 GB |
