# B-156: frame times before the build

`b156_gpu_adjust_timing` in `tests/b156_gpu_adjust.rs`, run three times on the checks-first
build (6a24bc5), when the card sent every frame with an adjustment layer to the processor. No
other cargo process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | Drawn on | Run 1 ms | Run 2 ms | Run 3 ms |
|---|---|---|---:|---:|---:|
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame | Draft | CPU | 18.2 | 18.1 | 17.8 |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame | Full | CPU | 67.4 | 61.3 | 61.4 |
| the same, over part of it | Draft | CPU | 20.3 | 18.3 | 20.6 |
| the same, over part of it | Full | CPU | 62.1 | 60.9 | 64.4 |
| a moving Noise, then the same, over the whole frame | Draft | CPU | 19.1 | 19.2 | 19.0 |
| a moving Noise, then the same, over the whole frame | Full | CPU | 75.0 | 75.9 | 75.5 |
