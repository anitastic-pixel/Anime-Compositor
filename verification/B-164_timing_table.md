# B-164: how long the viewer's card takes with a big blur

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads; Windows 11
- Build: release, `cargo test --release --test b164_downsampled_blurs -- --ignored b164_downsampled_blurs_timing`
- Other cargo processes running: none, checked before every run
- Measured on 2026-09-29

The reference shot at frame 100, the effect on its first three layers, drawn by the card as the
viewer asks for it. Before each drawing the card lets go of what it kept (B-153), so the blur is
worked every time. Each run draws the frame 16 times, drops the first and takes the middle one;
"Before" is the build at 95ad517 (the checks-first commit), "B-164" this build, the two built
side by side and run turn about three times. The table gives the middle of the three runs, in
milliseconds for one frame.

| Shot | Quality | Before | B-164 |
|---|---|---:|---:|
| No effect | Draft | 10.3 | 10.2 |
| No effect | Full | 11.9 | 11.8 |
| Gaussian Blur 20 | Draft | 6.6 | 6.3 |
| Gaussian Blur 20 | Full | 16.4 | 12.4 |
| Gaussian Blur 50 | Draft | 6.8 | 7.6 |
| Gaussian Blur 50 | Full | 24.1 | 13.9 |
| Gaussian Blur 100 | Draft | 8.2 | 7.5 |
| Gaussian Blur 100 | Full | 40.6 | 12.8 |
| Gaussian Blur 200 | Draft | 9.2 | 7.7 |
| Gaussian Blur 200 | Full | 90.9 | 14.6 |
| Glow 100 | Draft | 7.8 | 7.4 |
| Glow 100 | Full | 32.2 | 25.9 |
| Glow 200 | Draft | 8.2 | 8.7 |
| Glow 200 | Full | 43.8 | 25.6 |
| Bloom 100 | Draft | 8.8 | 8.3 |
| Bloom 100 | Full | 40.4 | 29.3 |
| Bloom 200 | Draft | 8.8 | 8.6 |
| Bloom 200 | Full | 59.4 | 31.6 |

Two rows are slower: Gaussian Blur 50 and Glow 200 at Draft, by under a millisecond. At Draft
the drawings are a quarter of the size, so the blur was already cheap, and the shrinking and
enlarging add two steps of their own. Every Full row is faster, most at Gaussian Blur 200
(90.9 to 14.6 ms).

## Lens Blur, measured for D-222 (not changed)

The same machine and build, the same shot with a Lens Blur (hexagon iris, roundness 20,
rotation 15, aspect 1.3, highlight gain 2, threshold 80, edges transparent) on the first three
layers; one run, the middle of 11 drawings. Lens Blur is not changed by B-164.

| Radius | Draft | Full |
|---:|---:|---:|
| none | 10.0 | 11.4 |
| 25 | 8.6 | 52.8 |
| 50 | 9.3 | 70.0 |
| 100 | 10.5 | 118.3 |
| 200 | 15.0 | 251.9 |
