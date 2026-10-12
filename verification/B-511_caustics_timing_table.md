# B-511: frame times with Caustics (D-631)

Measured on 2026-10-12, built with `cargo test --release --test b511_caustics` and run with `--ignored`. The machine: AMD Ryzen 9 9900X (24 threads), NVIDIA GeForce RTX 4070 Ti SUPER, Windows 11. **PROVISIONAL:** not a quiet machine; 33 to 36 cargo or rustc processes from other lanes were counted before, between and after the runs. The owner's app was not touched.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor (Caustics has no card path)

**What is timed:** the reference shot (1920 by 1080, 24 a second) with Caustics on its first layer: as added (no water surface), with the layer as its own water surface and Caustics Strength 1, and the same with Bottom Blur 50 (the blur's cost ceiling); frames 0 and 120, each drawn whole on the processor at Full with empty caches, three times; the median of the three, in milliseconds. The build as committed (de4daf9d).

Run 1 (36 other processes counted before, 35 after):

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 684.8 | 188.3 |
| Caustics as added (no water surface) | 261.0 | 1061.9 |
| Caustics, the layer its own water surface, caustics 1 | 1138.3 | 708.4 |
| the same with Bottom Blur 50 (the ceiling) | 2909.8 | 1036.0 |

Run 2, straight after (33 other processes counted after):

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 56.8 | 43.4 |
| Caustics as added (no water surface) | 245.9 | 989.2 |
| Caustics, the layer its own water surface, caustics 1 | 1865.6 | 1068.7 |
| the same with Bottom Blur 50 (the ceiling) | 2848.2 | 1842.4 |

**Reading it.** With more than thirty build processes sharing the processor, the same shot swings by hundreds of milliseconds between runs (no effect, frame 0: 684.8 then 56.8 ms), so only rough sizes can be read. On this busy machine Caustics took about 0.25 to 1.1 s a frame as added, about 0.7 to 1.9 s with the layer as its own water surface and caustics, and about 1.0 to 2.9 s with Bottom Blur 50 as well. Whether frame 120 really costs more than frame 0 as added cannot be told from two runs this noisy. Caustics carries nothing from frame to frame, so a late frame does not run longer for being late. The blur's direct taps are the logged ceiling (D-631); the upgrade, a running box or the FFT path, waits until a timing on a quiet machine asks for it. A quiet re-measure would settle the numbers.
