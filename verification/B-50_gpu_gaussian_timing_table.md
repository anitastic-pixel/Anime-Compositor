# B-50: frame times with three Gaussian Blurs, CPU and GPU

Written by `tests/b50_gpu_gaussian.rs` (`cargo test --release --test b50_gpu_gaussian -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The shot is the reference shot with the three Gaussian Blurs of the B-50 table. Every frame of it is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the three blurs run inside planning; on the GPU the card runs them. Each path starts with empty caches and plays the shot twice: the first loop fills the caches, the second is what playing it again costs. Medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 17.1 | 17.7 | 17.1 | 16.6 |
| Full | 58.8 | 58.4 | 30.4 | 29.3 |
