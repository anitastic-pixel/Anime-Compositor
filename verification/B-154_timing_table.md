# B-154: the 240-frame loop, made and from memory

Written by `b154_loop_timing` in `app/src/main.rs`, release build. Both shots are 1920 by 1080, 240 frames at 24 a second, so a frame is due every 41.7 ms. Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. Milliseconds are the window's whole work for a frame (`serve_logged`), not the web view's transport.

| Shot | Drawn on | Quality | Pass | Median ms | 95th percentile ms | Slowest ms | Frames over 41.7 ms |
|---|---|---|---|---|---|---|---|
| reference shot | CPU | Draft | first pass, each frame made | 2.4 | 10.9 | 21.9 | 0 |
| reference shot | CPU | Draft | second pass, from memory | 0.1 | 0.1 | 0.2 | 0 |
| reference shot | CPU | Draft | made ahead after play: 239 frames in 0.9 s; the loop holds 0.12 GiB | | | | |
| reference shot | CPU | Full | first pass, each frame made | 15.8 | 25.2 | 33.8 | 0 |
| reference shot | CPU | Full | second pass, from memory | 0.9 | 1.1 | 1.5 | 0 |
| reference shot | CPU | Full | made ahead after play: 239 frames in 3.8 s; the loop holds 1.85 GiB | | | | |
| declared ten-layer fixture | CPU | Draft | first pass, each frame made | 7.7 | 29.4 | 43.5 | 2 |
| declared ten-layer fixture | CPU | Draft | second pass, from memory | 0.3 | 0.3 | 0.9 | 0 |
| declared ten-layer fixture | CPU | Draft | made ahead after play: 239 frames in 2.5 s; the loop holds 0.12 GiB | | | | |
| declared ten-layer fixture | CPU | Full | first pass, each frame made | 27.8 | 55.3 | 83.0 | 24 |
| declared ten-layer fixture | CPU | Full | second pass, from memory | 1.1 | 1.4 | 1.8 | 0 |
| declared ten-layer fixture | CPU | Full | made ahead after play: 239 frames in 7.2 s; the loop holds 1.85 GiB | | | | |
| reference shot | card | Draft | first pass, each frame made | 0.8 | 27.1 | 95.2 | 4 |
| reference shot | card | Draft | second pass, from memory | 0.1 | 0.1 | 0.3 | 0 |
| reference shot | card | Draft | made ahead after play: 239 frames in 0.5 s; the loop holds 0.12 GiB | | | | |
| reference shot | card | Full | first pass, each frame made | 3.4 | 12.3 | 25.0 | 0 |
| reference shot | card | Full | second pass, from memory | 0.9 | 1.1 | 1.4 | 0 |
| reference shot | card | Full | made ahead after play: 239 frames in 0.9 s; the loop holds 1.85 GiB | | | | |
| declared ten-layer fixture | card | Draft | first pass, each frame made | 4.9 | 35.5 | 86.5 | 5 |
| declared ten-layer fixture | card | Draft | second pass, from memory | 0.2 | 0.2 | 0.4 | 0 |
| declared ten-layer fixture | card | Draft | made ahead after play: 239 frames in 1.8 s; the loop holds 0.12 GiB | | | | |
| declared ten-layer fixture | card | Full | first pass, each frame made | 6.1 | 40.2 | 67.9 | 11 |
| declared ten-layer fixture | card | Full | second pass, from memory | 1.0 | 1.3 | 1.7 | 0 |
| declared ten-layer fixture | card | Full | made ahead after play: 239 frames in 1.9 s; the loop holds 1.85 GiB | | | | |
