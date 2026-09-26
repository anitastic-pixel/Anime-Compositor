# B-46: frame times with three Radial Blurs, CPU and GPU

Written by `tests/b46_gpu_radial.rs` (`cargo test --release --test b46_gpu_radial -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The shot is the reference shot with the three Radial Blurs of the B-46 table. Every frame of it is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the three blurs run inside planning; on the GPU the card runs them. Each path starts with empty caches and plays the shot twice: the first loop fills the caches, the second is what playing it again costs. Medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.5 | 17.0 | 17.5 | 17.2 |
| Full | 160.1 | 152.1 | 25.7 | 26.5 |
