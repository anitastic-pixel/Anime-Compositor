# B-52: the before and after pictures

Written by `tests/b52_gpu_edges.rs` (`b52_pictures`). Frame 0 of the reference shot, whose background fills the frame, with one blur on the background: its edges left transparent, as before D-109, and then repeated. Drawn by the CPU, as an export is, and shown over a grey checkerboard, which shows through wherever the picture is see-through. The pictures are in `verification/B-52 pictures/`. **The rule: with the edges transparent some pixels are see-through, and with them repeated none is.**

| Blur | Transparent | Pixels see-through | Repeat Edge Pixels | Pixels see-through | Result |
|---|---|---:|---|---:|---|
| a Gaussian Blur of sigma 10 | `gaussian_transparent.png` | 147228 | `gaussian_repeat.png` | 0 | PASS |
| a Directional Blur at 45 degrees, 60 long | `directional_transparent.png` | 120384 | `directional_repeat.png` | 0 | PASS |
| a Radial Blur, zoom 20 about the middle | `radial_transparent.png` | 320748 | `radial_repeat.png` | 0 | PASS |
