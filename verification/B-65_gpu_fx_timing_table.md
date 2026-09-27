# B-65: frame times with each of the ten, CPU and GPU

Written by `tests/b65_gpu_fx.rs` (`cargo test --release --test b65_gpu_fx -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with one of the ten on three layers, as in the B-65 table. Every fourth frame, 60 in all, is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the effects run inside planning; on the GPU the card runs the last effect of each layer. Each path starts with empty caches and plays the frames twice: the first loop fills the caches, the second is what playing it again costs. Medians in ms.

| Effect | Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---|---:|---:|---:|---:|
| Curves | Draft | 24.8 | 25.9 | 26.5 | 25.5 |
| Curves | Full | 41.9 | 41.5 | 27.4 | 25.7 |
| Levels | Draft | 26.5 | 27.0 | 27.7 | 25.3 |
| Levels | Full | 38.4 | 38.7 | 29.8 | 26.2 |
| Hue/Saturation | Draft | 26.2 | 26.2 | 29.3 | 27.5 |
| Hue/Saturation | Full | 38.8 | 39.0 | 27.7 | 27.1 |
| Gradient | Draft | 26.2 | 26.8 | 26.1 | 25.5 |
| Gradient | Full | 38.6 | 38.9 | 28.3 | 26.0 |
| Drop Shadow | Draft | 26.2 | 26.2 | 29.1 | 27.0 |
| Drop Shadow | Full | 38.8 | 38.9 | 24.7 | 25.0 |
| Lens Blur | Draft | 26.2 | 26.4 | 28.5 | 28.2 |
| Lens Blur | Full | 39.5 | 39.0 | 25.5 | 26.2 |
| Rim Light | Draft | 26.8 | 26.3 | 26.7 | 26.3 |
| Rim Light | Full | 38.4 | 39.0 | 25.1 | 24.7 |
| Outline | Draft | 26.1 | 26.3 | 25.6 | 25.7 |
| Outline | Full | 38.3 | 38.2 | 26.8 | 25.1 |
| Noise | Draft | 29.6 | 26.2 | 30.6 | 30.3 |
| Noise | Full | 83.3 | 82.7 | 37.8 | 37.6 |
| Chromatic Aberration | Draft | 26.0 | 26.1 | 25.8 | 25.4 |
| Chromatic Aberration | Full | 38.1 | 38.8 | 28.0 | 26.5 |
