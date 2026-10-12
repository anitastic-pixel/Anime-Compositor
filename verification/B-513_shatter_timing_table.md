# B-513: frame times with Shatter (D-633)

Measured on 2026-10-12, built with `cargo test --release --test b513_shatter` and run with `--ignored`. The machine: AMD Ryzen 9 9900X (24 threads), NVIDIA GeForce RTX 4070 Ti SUPER, Windows 11. A quiet machine: no other cargo or rustc process was counted before, between or after the runs. The owner's app was not touched.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor (Shatter has no card path)

**What is timed:** the reference shot (1920 by 1080, 24 a second) with Shatter on its first layer: as added (bricks 10 across, one force in the middle), glass 50 across with a force big enough to free every piece, a small force swept across the layer over frames 0 to 120 (so the release is walked over every step), and 1000 tiles across (the most allowed, the ceiling); frames 0 and 120, each drawn whole on the processor at Full with empty caches, three times; the median of the three, in milliseconds. The build as committed (a17a9450).

Run 1:

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 35.7 | 34.3 |
| Shatter as added (bricks 10 across) | 94.0 | 88.0 |
| glass 50 across, every piece freed | 136.3 | 103.9 |
| force swept across over frames 0 to 120 (the release walked every step) | 96.5 | 96.7 |
| 1000 across (the ceiling) | 4658.0 | 2781.3 |

Run 2, straight after:

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 40.0 | 36.9 |
| Shatter as added (bricks 10 across) | 96.3 | 93.0 |
| glass 50 across, every piece freed | 141.0 | 108.1 |
| force swept across over frames 0 to 120 (the release walked every step) | 97.4 | 97.5 |
| 1000 across (the ceiling) | 5042.1 | 2677.3 |

**Reading it.** Shatter as added adds about 55 to 60 ms a frame to the shot's 34 to 40. Freeing every glass piece costs about 100 ms at frame 0 and less by frame 120, when many pieces have flown out of the picture. Walking the release over 480 steps for the swept force costs no more than as added here. The ceiling, 1000 tiles across (about half a million pieces), takes 2.7 to 5.0 s a frame: numbering the pieces and drawing that many slabs. The two logged ceilings (pieces as pixel sets; the release walked from the in point each frame) wait until they bite.
