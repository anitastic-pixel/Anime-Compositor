# B-161: decoding a drawing against reading its disk copy

Produced by `cargo test --release --test b161_decode_cache -- --ignored`, with no other build running on the machine; a busy machine gives noisy numbers.

## Machine, build and configuration

- Machine: AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER on driver 610.88 (not used here), Windows 11; the drawings and their copies on the same SSD.
- Measured on 2026-09-30 at c9bf93e, three runs taken with no other cargo process or build running; this file is the middle run (the second), the three are below. It replaces the provisional figures of 2026-09-29 (decode 45.04, copy 32.33, first pass 38.52 ms), measured while other builds ran.
- Build: release (`opt-level = 3`).
- Shot: the reference shot, composition frames 0 to 47 (188 drawings asked for, 52 distinct copies, 411.3 MiB on disk).
- Each frame's drawings are read one after another on one thread, as `CelCache::decoded` does; the viewer's read-ahead does them side by side, which divides both columns alike.
- Decode: `CelCache::none`, so no memory cache and no copies. Copy: a cache with no memory budget and the copies on, so every drawing comes from its copy.
- The copies were read while Windows still had them in its file cache, which is the case for playing a shot again in the same sitting. After a restart the first read comes from the SSD itself and is slower.
- 7 runs per frame, each a decode and then a copy read one straight after the other, median of the runs; then the median over the 48 frames. The first pass is timed once, before the runs.

## Result

| Per frame (median over frames) | ms |
|---|---|
| Decode the drawings | 35.40 |
| Read their copies | 24.30 |
| First pass: decode and write the copies | 28.37 |

Reading the copies took 69% of the decode time.

## The three runs

| Run | Decode ms | Copy ms | Copy against decode | First pass ms |
|---|---|---|---|---|
| 1 | 35.38 | 24.56 | 69% | 28.47 |
| 2 (this file) | 35.40 | 24.30 | 69% | 28.37 |
| 3 | 35.28 | 23.91 | 68% | 29.84 |
| Median | 35.38 | 24.30 | 69% | 28.47 |

## Every frame

| Frame | Decode ms | Copy ms | First pass ms |
|---|---|---|---|
| 0 | 34.32 | 23.27 | 81.34 |
| 1 | 33.56 | 23.88 | 29.01 |
| 2 | 34.21 | 23.35 | 33.74 |
| 3 | 33.46 | 23.70 | 37.55 |
| 4 | 37.53 | 26.56 | 36.00 |
| 5 | 35.38 | 23.87 | 55.58 |
| 6 | 35.40 | 24.27 | 34.38 |
| 7 | 36.53 | 24.60 | 26.85 |
| 8 | 35.85 | 24.76 | 73.96 |
| 9 | 35.85 | 24.52 | 31.46 |
| 10 | 37.97 | 26.35 | 32.46 |
| 11 | 35.38 | 24.87 | 31.05 |
| 12 | 34.47 | 24.55 | 34.42 |
| 13 | 35.31 | 24.36 | 28.06 |
| 14 | 29.19 | 18.76 | 25.14 |
| 15 | 29.17 | 17.98 | 25.32 |
| 16 | 36.23 | 26.30 | 30.77 |
| 17 | 35.45 | 25.26 | 76.97 |
| 18 | 35.00 | 23.73 | 36.11 |
| 19 | 35.47 | 23.64 | 27.42 |
| 20 | 34.77 | 24.36 | 32.47 |
| 21 | 34.85 | 24.15 | 34.78 |
| 22 | 36.64 | 24.91 | 30.03 |
| 23 | 35.33 | 24.28 | 33.70 |
| 24 | 35.80 | 23.93 | 31.18 |
| 25 | 35.95 | 24.38 | 25.06 |
| 26 | 34.75 | 23.77 | 23.55 |
| 27 | 35.28 | 24.18 | 28.37 |
| 28 | 35.45 | 24.17 | 25.12 |
| 29 | 35.23 | 24.52 | 24.06 |
| 30 | 34.97 | 24.15 | 29.98 |
| 31 | 36.33 | 25.31 | 25.95 |
| 32 | 35.72 | 24.13 | 24.34 |
| 33 | 35.92 | 24.66 | 27.81 |
| 34 | 35.90 | 24.77 | 25.20 |
| 35 | 35.32 | 25.19 | 23.61 |
| 36 | 36.21 | 25.23 | 27.56 |
| 37 | 34.70 | 24.28 | 23.78 |
| 38 | 29.08 | 18.35 | 17.74 |
| 39 | 29.19 | 18.04 | 21.91 |
| 40 | 35.52 | 25.59 | 24.46 |
| 41 | 36.27 | 24.30 | 23.55 |
| 42 | 35.55 | 24.09 | 29.85 |
| 43 | 35.85 | 24.57 | 23.31 |
| 44 | 35.46 | 24.41 | 23.41 |
| 45 | 35.39 | 24.07 | 30.22 |
| 46 | 35.85 | 25.05 | 24.41 |
| 47 | 35.10 | 23.84 | 24.29 |
