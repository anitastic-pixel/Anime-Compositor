# B-512: frame times with Card Dance (D-632)

Measured on 2026-10-12, built with `cargo test --release --test b512_card_dance` and run with `--ignored`. The machine: AMD Ryzen 9 9900X (24 threads), NVIDIA GeForce RTX 4070 Ti SUPER, Windows 11. A quiet machine: no other cargo or rustc process was counted before, between or after the runs. The owner's app was not touched.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor (Card Dance has no card path)

**What is timed:** the reference shot (1920 by 1080, 24 a second) with Card Dance on its first layer: as added (8 by 8 cards, each moved by the layer's brightness), 100 by 100 cards each tilted, and 1000 by 1000 cards (the most allowed, the ceiling); frames 0 and 120, each drawn whole on the processor at Full with empty caches, three times; the median of the three, in milliseconds. The build as committed (d38cf0ca).

Run 1:

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 34.4 | 34.9 |
| Card Dance as added (8 by 8 cards) | 51.2 | 48.7 |
| 100 by 100 cards, each tilted | 51.9 | 52.7 |
| 1000 by 1000 cards (the ceiling) | 248.8 | 236.6 |

Run 2, straight after:

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 35.7 | 36.9 |
| Card Dance as added (8 by 8 cards) | 49.1 | 49.0 |
| 100 by 100 cards, each tilted | 51.9 | 52.1 |
| 1000 by 1000 cards (the ceiling) | 233.7 | 237.1 |

**Reading it.** Card Dance as added adds about 13 to 17 ms a frame to the shot's 34 to 37; 100 by 100 tilted cards cost about the same, the 16-pixel tiles keeping each pixel's search short. The ceiling, a million cards, takes about 234 to 249 ms a frame, the cost of working every card's place and binning it. Card Dance carries nothing from frame to frame, so frame 120 costs what frame 0 does. Holding every card's numbers at once is the logged ceiling (about 100 MB at a million cards); working a band at a time waits until it bites.
