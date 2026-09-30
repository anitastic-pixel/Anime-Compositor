# B-154: the 240-frame loop on the build before RAM preview

The same loop as `verification/B-154_timing_table.md`, asked for frame by frame through `serve_logged`, on the build before B-154 (d7cb950, the checks-first commit) and on B-154, three runs of each, turn about. Release build; Ryzen 9 9900X, 64 GB, Windows 11; card NVIDIA GeForce RTX 4070 Ti SUPER, driver 610.88, Vulkan; no other cargo process running. Each number is the middle of the three runs. Milliseconds; a frame is due every 41.7 ms.

| Shot | Drawn on | Quality | Pass | Before: median | Before: 95th percentile | Before: frames over 41.7 ms | B-154: median | B-154: 95th percentile | B-154: frames over 41.7 ms |
|---|---|---|---|---|---|---|---|---|---|
| reference shot | CPU | Draft | first pass | 2.4 | 11.4 | 0 | 2.4 | 10.9 | 0 |
| reference shot | CPU | Draft | second pass | 2.3 | 2.6 | 0 | 0.1 | 0.1 | 0 |
| reference shot | CPU | Full | first pass | 15.2 | 23.3 | 0 | 16.1 | 24.0 | 0 |
| reference shot | CPU | Full | second pass | 15.0 | 16.2 | 0 | 0.9 | 1.1 | 0 |
| declared ten-layer fixture | CPU | Draft | first pass | 8.1 | 29.2 | 2 | 7.9 | 29.4 | 2 |
| declared ten-layer fixture | CPU | Draft | second pass | 7.8 | 9.3 | 0 | 0.2 | 0.3 | 0 |
| declared ten-layer fixture | CPU | Full | first pass | 27.7 | 61.3 | 25 | 27.8 | 58.7 | 24 |
| declared ten-layer fixture | CPU | Full | second pass | 27.6 | 29.6 | 0 | 1.1 | 1.4 | 0 |
| reference shot | card | Draft | first pass | 0.8 | 27.1 | 4 | 0.8 | 27.1 | 4 |
| reference shot | card | Draft | second pass | 0.6 | 0.8 | 0 | 0.1 | 0.1 | 0 |
| reference shot | card | Full | first pass | 2.4 | 11.6 | 0 | 3.4 | 12.6 | 0 |
| reference shot | card | Full | second pass | 2.3 | 2.9 | 0 | 0.9 | 1.1 | 0 |
| declared ten-layer fixture | card | Draft | first pass | 5.0 | 35.3 | 5 | 4.9 | 35.5 | 6 |
| declared ten-layer fixture | card | Draft | second pass | 4.8 | 5.3 | 0 | 0.2 | 0.3 | 0 |
| declared ten-layer fixture | card | Full | first pass | 4.6 | 38.7 | 11 | 5.9 | 40.1 | 11 |
| declared ten-layer fixture | card | Full | second pass | 3.6 | 4.4 | 0 | 1.0 | 1.3 | 0 |

## Each run

Before, run 1:

| Shot | Drawn on | Quality | Pass | Median | 95th | Slowest | Over |
|---|---|---|---|---|---|---|---|
| reference shot | CPU | Draft | first pass | 2.4 | 11.1 | 21.4 | 0 |
| reference shot | CPU | Draft | second pass | 2.3 | 2.6 | 2.7 | 0 |
| reference shot | CPU | Full | first pass | 15.0 | 23.3 | 32.4 | 0 |
| reference shot | CPU | Full | second pass | 15.0 | 16.2 | 17.1 | 0 |
| declared ten-layer fixture | CPU | Draft | first pass | 7.9 | 28.6 | 44.1 | 2 |
| declared ten-layer fixture | CPU | Draft | second pass | 7.6 | 8.7 | 9.2 | 0 |
| declared ten-layer fixture | CPU | Full | first pass | 27.5 | 55.8 | 90.7 | 25 |
| declared ten-layer fixture | CPU | Full | second pass | 27.1 | 29.6 | 31.1 | 0 |
| reference shot | card | Draft | first pass | 0.8 | 27.4 | 91.1 | 3 |
| reference shot | card | Draft | second pass | 0.6 | 0.8 | 1.3 | 0 |
| reference shot | card | Full | first pass | 2.3 | 11.6 | 24.2 | 0 |
| reference shot | card | Full | second pass | 2.3 | 2.9 | 3.1 | 0 |
| declared ten-layer fixture | card | Draft | first pass | 4.6 | 34.7 | 65.5 | 4 |
| declared ten-layer fixture | card | Draft | second pass | 4.7 | 5.3 | 5.8 | 0 |
| declared ten-layer fixture | card | Full | first pass | 4.6 | 39.0 | 62.5 | 11 |
| declared ten-layer fixture | card | Full | second pass | 3.6 | 4.4 | 4.8 | 0 |

Before, run 2:

| Shot | Drawn on | Quality | Pass | Median | 95th | Slowest | Over |
|---|---|---|---|---|---|---|---|
| reference shot | CPU | Draft | first pass | 2.7 | 11.6 | 24.5 | 0 |
| reference shot | CPU | Draft | second pass | 2.5 | 2.8 | 3.2 | 0 |
| reference shot | CPU | Full | first pass | 17.4 | 24.4 | 33.3 | 0 |
| reference shot | CPU | Full | second pass | 16.0 | 17.3 | 19.6 | 0 |
| declared ten-layer fixture | CPU | Draft | first pass | 8.1 | 30.8 | 45.7 | 2 |
| declared ten-layer fixture | CPU | Draft | second pass | 8.0 | 9.5 | 10.1 | 0 |
| declared ten-layer fixture | CPU | Full | first pass | 27.7 | 62.7 | 97.0 | 27 |
| declared ten-layer fixture | CPU | Full | second pass | 27.6 | 29.4 | 33.1 | 0 |
| reference shot | card | Draft | first pass | 0.7 | 27.0 | 89.1 | 4 |
| reference shot | card | Draft | second pass | 0.6 | 0.8 | 1.0 | 0 |
| reference shot | card | Full | first pass | 2.4 | 11.3 | 24.5 | 0 |
| reference shot | card | Full | second pass | 2.2 | 2.5 | 2.8 | 0 |
| declared ten-layer fixture | card | Draft | first pass | 5.0 | 35.3 | 74.4 | 5 |
| declared ten-layer fixture | card | Draft | second pass | 4.8 | 5.3 | 6.2 | 0 |
| declared ten-layer fixture | card | Full | first pass | 4.6 | 38.4 | 64.6 | 11 |
| declared ten-layer fixture | card | Full | second pass | 3.5 | 4.4 | 5.1 | 0 |

Before, run 3:

| Shot | Drawn on | Quality | Pass | Median | 95th | Slowest | Over |
|---|---|---|---|---|---|---|---|
| reference shot | CPU | Draft | first pass | 2.3 | 11.4 | 22.1 | 0 |
| reference shot | CPU | Draft | second pass | 2.3 | 2.5 | 3.1 | 0 |
| reference shot | CPU | Full | first pass | 15.2 | 22.6 | 33.5 | 0 |
| reference shot | CPU | Full | second pass | 14.9 | 16.0 | 17.0 | 0 |
| declared ten-layer fixture | CPU | Draft | first pass | 8.1 | 29.2 | 44.9 | 1 |
| declared ten-layer fixture | CPU | Draft | second pass | 7.8 | 9.3 | 9.8 | 0 |
| declared ten-layer fixture | CPU | Full | first pass | 27.9 | 61.3 | 90.9 | 24 |
| declared ten-layer fixture | CPU | Full | second pass | 30.7 | 33.2 | 35.1 | 0 |
| reference shot | card | Draft | first pass | 1.2 | 27.1 | 90.7 | 5 |
| reference shot | card | Draft | second pass | 0.9 | 1.3 | 1.7 | 0 |
| reference shot | card | Full | first pass | 2.9 | 12.8 | 27.4 | 0 |
| reference shot | card | Full | second pass | 3.0 | 3.4 | 3.6 | 0 |
| declared ten-layer fixture | card | Draft | first pass | 5.8 | 37.9 | 68.8 | 5 |
| declared ten-layer fixture | card | Draft | second pass | 6.3 | 7.0 | 7.8 | 0 |
| declared ten-layer fixture | card | Full | first pass | 5.2 | 38.7 | 73.9 | 11 |
| declared ten-layer fixture | card | Full | second pass | 4.7 | 5.3 | 5.7 | 0 |

