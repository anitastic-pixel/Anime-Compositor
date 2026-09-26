# B-44b: frame times, CPU and GPU

Written by `tests/b44_gpu_preview.rs` (`cargo test --release --test b44_gpu_preview -- --ignored`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each frame's plan is made first and not timed: reading drawings and running effects are the same work on either path. What is timed is the rest of the frame: on the CPU, `render` and `to_srgb8_straight`; on the GPU, sending any drawing the card does not have yet, drawing, and bringing the eight-bit picture back. **First pass** is the GPU meeting every drawing for the first time; **again** is the same frames with the drawings already on the card, if it kept them. Medians over every frame of the shot, in ms, and how many drawings each pass sent to the card, which may hold 8.4 GB of them.

| Shot | Quality | CPU | GPU, first pass | GPU, again | Drawings sent, first pass | Drawings sent, again |
|---|---|---|---|---|---|---|
| the reference shot | Draft | 2.78 | 0.91 | 1.14 | 56 | 0 |
| the reference shot | Full | 15.59 | 5.65 | 3.68 | 56 | 0 |
| the ten-layer fixture | Draft | 4.12 | 1.68 | 1.33 | 166 | 0 |
| the ten-layer fixture | Full | 28.96 | 5.87 | 7.76 | 166 | 0 |
