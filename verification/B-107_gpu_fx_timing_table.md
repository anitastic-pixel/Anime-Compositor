# B-107: frame times with each of the third batch's twenty-nine, CPU and GPU

Written by `tests/b107_gpu_fx.rs` (`cargo test --release --test b107_gpu_fx -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with one of the twenty-nine on three layers, as in the B-107 table. Every fourth frame, 60 in all, is asked for as the viewer asks, whole: planning, the effects, drawing, and the eight-bit picture. On the CPU the effects run inside planning; on the GPU the card runs the last effect of each layer. Each path starts with empty caches and plays the frames twice: the first loop fills the caches, the second is what playing it again costs. Medians in ms.

| Effect | Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---|---:|---:|---:|---:|
| Invert | Draft | 25.6 | 26.1 | 26.2 | 26.3 |
| Invert | Full | 38.7 | 39.0 | 26.5 | 26.4 |
| Brightness & Contrast | Draft | 27.1 | 27.0 | 26.2 | 25.8 |
| Brightness & Contrast | Full | 39.0 | 38.3 | 27.0 | 25.8 |
| Black & White | Draft | 26.5 | 26.7 | 26.7 | 26.1 |
| Black & White | Full | 37.1 | 37.4 | 26.8 | 25.9 |
| Posterize | Draft | 26.2 | 26.4 | 26.5 | 26.1 |
| Posterize | Full | 37.9 | 38.4 | 27.4 | 25.5 |
| Threshold | Draft | 26.3 | 26.5 | 26.1 | 25.5 |
| Threshold | Full | 37.7 | 37.8 | 25.8 | 25.0 |
| Channel Mixer | Draft | 26.3 | 26.2 | 27.2 | 26.9 |
| Channel Mixer | Full | 38.6 | 37.5 | 28.7 | 27.4 |
| Vibrance | Draft | 27.2 | 27.6 | 26.8 | 27.0 |
| Vibrance | Full | 39.6 | 40.3 | 28.6 | 27.9 |
| Leave Color | Draft | 27.1 | 27.4 | 26.1 | 25.6 |
| Leave Color | Full | 39.5 | 38.5 | 29.9 | 25.2 |
| Solarize | Draft | 26.5 | 26.4 | 25.8 | 25.3 |
| Solarize | Full | 36.6 | 36.6 | 25.2 | 26.5 |
| Halftone | Draft | 25.6 | 26.1 | 26.2 | 25.8 |
| Halftone | Full | 38.3 | 37.2 | 25.3 | 25.0 |
| Mosaic | Draft | 25.5 | 25.8 | 25.3 | 25.2 |
| Mosaic | Full | 37.4 | 37.9 | 24.7 | 24.3 |
| Emboss | Draft | 25.6 | 25.9 | 25.3 | 25.2 |
| Emboss | Full | 37.3 | 37.1 | 25.3 | 24.3 |
| Find Edges | Draft | 25.7 | 26.1 | 25.7 | 25.0 |
| Find Edges | Full | 38.4 | 37.4 | 24.9 | 25.4 |
| Sharpen | Draft | 25.5 | 25.7 | 27.7 | 25.2 |
| Sharpen | Full | 37.5 | 37.5 | 24.8 | 24.4 |
| Diffusion | Draft | 25.8 | 25.6 | 25.5 | 25.0 |
| Diffusion | Full | 37.2 | 36.7 | 24.6 | 24.3 |
| Wave Warp | Draft | 29.6 | 25.6 | 27.0 | 26.3 |
| Wave Warp | Full | 93.2 | 94.6 | 39.7 | 35.4 |
| Ripple | Draft | 31.7 | 27.2 | 27.0 | 26.6 |
| Ripple | Full | 107.5 | 105.1 | 38.0 | 34.3 |
| Twirl | Draft | 25.6 | 25.6 | 25.0 | 25.0 |
| Twirl | Full | 38.8 | 41.4 | 27.8 | 27.7 |
| Bulge | Draft | 29.1 | 28.2 | 27.3 | 26.2 |
| Bulge | Full | 39.6 | 37.8 | 29.5 | 28.4 |
| Mirror | Draft | 26.1 | 26.0 | 26.1 | 25.5 |
| Mirror | Full | 38.5 | 37.9 | 28.8 | 24.8 |
| Linear Wipe | Draft | 25.8 | 27.0 | 27.2 | 26.2 |
| Linear Wipe | Full | 37.6 | 37.9 | 30.9 | 28.7 |
| Radial Wipe | Draft | 27.1 | 27.4 | 27.0 | 27.1 |
| Radial Wipe | Full | 39.1 | 42.0 | 26.6 | 25.6 |
| Venetian Blinds | Draft | 27.1 | 26.9 | 26.2 | 25.5 |
| Venetian Blinds | Full | 36.8 | 36.8 | 25.1 | 24.7 |
| Iris Wipe | Draft | 25.8 | 26.1 | 26.1 | 25.5 |
| Iris Wipe | Full | 37.1 | 36.8 | 26.6 | 24.6 |
| Simple Choker | Draft | 26.0 | 26.1 | 26.0 | 25.6 |
| Simple Choker | Full | 37.2 | 37.0 | 25.9 | 24.8 |
| Speed Lines | Draft | 28.9 | 25.8 | 27.4 | 28.3 |
| Speed Lines | Full | 79.7 | 80.8 | 37.6 | 34.8 |
| Cross Glare | Draft | 26.0 | 26.4 | 25.5 | 25.0 |
| Cross Glare | Full | 120.3 | 120.6 | 24.8 | 24.6 |
| Camera Shake | Draft | 30.6 | 26.0 | 26.8 | 26.9 |
| Camera Shake | Full | 87.7 | 90.0 | 36.4 | 34.3 |
| Rain | Draft | 29.4 | 26.3 | 28.5 | 27.0 |
| Rain | Full | 78.4 | 79.8 | 34.9 | 33.8 |
