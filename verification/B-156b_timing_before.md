# B-156b: frame times before the build

`b152_card_whole_frame_timing` in `tests/b152_card_whole_frame.rs`, run three times on the
checks-first build (e6d5f7d), when the processor averaged a motion-blurred layer's moments and the
card laid the result. No other cargo process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | Run 1 ms | Run 2 ms | Run 3 ms |
|---|---|---:|---:|---:|
| the reference shot | Draft | 7.4 | 7.5 | 7.6 |
| the reference shot | Full | 9.6 | 9.6 | 9.6 |
| the reference shot with motion blur | Draft | 15.6 | 15.4 | 15.1 |
| the reference shot with motion blur | Full | 47.4 | 46.6 | 47.9 |
| the reference shot with frame mix and dissolve | Draft | 47.9 | 42.1 | 45.4 |
| the reference shot with frame mix and dissolve | Full | 58.4 | 62.3 | 61.1 |
| the reference shot with motion blur and Roughen Edges | Draft | 25.8 | 25.9 | 23.0 |
| the reference shot with motion blur and Roughen Edges | Full | 56.8 | 53.2 | 51.2 |
