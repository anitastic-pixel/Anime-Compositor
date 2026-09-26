# B-51: frame times with three Glows, CPU and GPU

Written by `tests/b51_gpu_glow.rs` (`cargo test --release --test b51_gpu_glow -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The shot is the reference shot with the three Glows of the B-51 table. Every frame of it is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the three Glows run inside planning; on the GPU the card runs them. Each path starts with empty caches and plays the shot twice: the first loop fills the caches, the second is what playing it again costs. Medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.0 | 16.0 | 17.1 | 17.2 |
| Full | 53.2 | 52.4 | 16.2 | 15.9 |
