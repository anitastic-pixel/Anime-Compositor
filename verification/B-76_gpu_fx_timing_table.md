# B-76: frame times with each of the second ten, CPU and GPU

Written by `tests/b76_gpu_fx.rs` (`cargo test --release --test b76_gpu_fx -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with one of the ten on three layers, as in the B-76 table. Every fourth frame, 60 in all, is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the effects run inside planning; on the GPU the card runs the last effect of each layer. Each path starts with empty caches and plays the frames twice: the first loop fills the caches, the second is what playing it again costs. Medians in ms.

| Effect | Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---|---:|---:|---:|---:|
| Distance Gradation | Draft | 25.8 | 26.3 | 25.8 | 25.6 |
| Distance Gradation | Full | 39.0 | 38.3 | 25.9 | 25.1 |
| Light Rays | Draft | 26.5 | 26.0 | 25.9 | 26.2 |
| Light Rays | Full | 39.1 | 39.8 | 26.4 | 26.2 |
| Exposure Flicker | Draft | 31.1 | 28.2 | 28.5 | 28.7 |
| Exposure Flicker | Full | 84.8 | 80.8 | 35.6 | 34.2 |
| Vignette | Draft | 27.6 | 26.8 | 27.1 | 26.8 |
| Vignette | Full | 41.8 | 40.7 | 27.3 | 26.0 |
| Turbulent Displace | Draft | 39.8 | 27.1 | 29.0 | 29.1 |
| Turbulent Displace | Full | 231.2 | 225.9 | 54.8 | 53.7 |
| Fractal Noise | Draft | 31.9 | 26.2 | 29.1 | 28.3 |
| Fractal Noise | Full | 125.7 | 122.7 | 40.0 | 38.7 |
| Gradient Map | Draft | 26.6 | 26.7 | 26.8 | 26.1 |
| Gradient Map | Full | 40.4 | 41.3 | 26.5 | 25.0 |
| Color Balance | Draft | 26.5 | 26.1 | 26.6 | 26.5 |
| Color Balance | Full | 39.8 | 39.6 | 25.8 | 25.6 |
| Offset | Draft | 27.7 | 27.0 | 26.4 | 26.7 |
| Offset | Full | 40.1 | 41.3 | 28.5 | 26.5 |
| Light Wrap | Draft | 39.7 | 40.3 | 27.7 | 26.6 |
| Light Wrap | Full | 149.1 | 150.1 | 55.2 | 53.6 |
