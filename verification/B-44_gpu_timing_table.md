# B-44: frame times, CPU and GPU

Written by `tests/b44_gpu_preview.rs` (`cargo test --release --test b44_gpu_preview -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each frame's plan is made first and not timed: reading drawings and running effects are the same work on either path. What is timed is the rest of the frame: on the CPU, `render` and `to_srgb8_straight`; on the GPU, sending any drawing the card does not have yet, drawing, and bringing the eight-bit picture back. **First pass** is the GPU meeting every drawing for the first time; **again** is the same frames with the drawings already on the card. Medians over every frame of the shot, in ms.

| Shot | Quality | CPU | GPU, first pass | GPU, again |
|---|---|---|---|---|
| the reference shot | Draft | 2.48 | 10.77 | 11.45 |
| the reference shot | Full | 15.50 | 14.03 | 13.19 |
| the ten-layer fixture | Draft | 4.08 | 16.60 | 16.76 |
| the ten-layer fixture | Full | 28.74 | 36.21 | 35.83 |
