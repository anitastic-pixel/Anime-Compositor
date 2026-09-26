# B-49: frame times with three Directional Blurs, CPU and GPU

Written by `tests/b49_gpu_directional.rs` (`cargo test --release --test b49_gpu_directional -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The shot is the reference shot with the three Directional Blurs of the B-49 table. Every frame of it is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the three blurs run inside planning; on the GPU the card runs them. Each path starts with empty caches and plays the shot twice: the first loop fills the caches, the second is what playing it again costs. Medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.6 | 17.1 | 16.3 | 16.3 |
| Full | 59.7 | 61.3 | 27.3 | 27.4 |
