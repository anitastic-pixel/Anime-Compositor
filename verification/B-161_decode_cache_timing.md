# B-161: decoding a drawing against reading its disk copy - PROVISIONAL

**Provisional.** Measured on 2026-09-29 while other builds ran on the same machine in parallel (the G-units were being built at the same time), so the absolute numbers are noisy and a quiet re-measure is due. Produced by `cargo test --release --test b161_decode_cache -- --ignored`.

## Machine, build and configuration

- Machine: the reference machine (Windows 11, 24 logical processors).
- Build: release (`opt-level = 3`).
- Shot: the reference shot, composition frames 0 to 47 (188 drawings asked for, 52 distinct copies, 411.3 MiB on disk).
- Each frame's drawings are read one after another on one thread, as `CelCache::decoded` does; the viewer's read-ahead does them side by side, which divides both columns alike.
- Decode: `CelCache::none`, so no memory cache and no copies. Copy: a cache with no memory budget and the copies on, so every drawing comes from its copy.
- The copies were read while Windows still had them in its file cache, which is the case for playing a shot again in the same sitting. After a restart the first read comes from the SSD itself and is slower.
- 7 runs per frame, each a decode and then a copy read one straight after the other, median of the runs; then the median over the 48 frames. The first pass is timed once, before the runs.

## Result

| Per frame (median over frames) | ms |
|---|---|
| Decode the drawings | 45.04 |
| Read their copies | 32.33 |
| First pass: decode and write the copies | 38.52 |

Reading the copies took 72% of the decode time.

## Every frame

| Frame | Decode ms | Copy ms | First pass ms |
|---|---|---|---|
| 0 | 42.30 | 31.42 | 56.07 |
| 1 | 40.15 | 32.79 | 39.80 |
| 2 | 46.06 | 30.65 | 41.23 |
| 3 | 45.58 | 32.61 | 39.84 |
| 4 | 46.49 | 33.66 | 45.39 |
| 5 | 48.68 | 31.04 | 36.98 |
| 6 | 44.47 | 30.66 | 54.77 |
| 7 | 46.97 | 32.62 | 42.00 |
| 8 | 46.50 | 34.69 | 36.38 |
| 9 | 48.78 | 31.74 | 39.28 |
| 10 | 49.76 | 36.94 | 34.88 |
| 11 | 50.26 | 34.39 | 60.65 |
| 12 | 52.73 | 38.20 | 42.01 |
| 13 | 45.26 | 32.03 | 40.06 |
| 14 | 36.80 | 22.30 | 29.00 |
| 15 | 36.75 | 23.93 | 39.53 |
| 16 | 45.75 | 32.98 | 54.75 |
| 17 | 45.17 | 34.87 | 38.48 |
| 18 | 43.44 | 37.49 | 47.78 |
| 19 | 47.35 | 36.34 | 39.83 |
| 20 | 49.40 | 34.61 | 42.61 |
| 21 | 47.47 | 34.46 | 44.35 |
| 22 | 45.60 | 33.80 | 38.54 |
| 23 | 47.05 | 32.45 | 34.55 |
| 24 | 46.74 | 36.07 | 32.95 |
| 25 | 42.90 | 34.11 | 30.22 |
| 26 | 43.28 | 30.22 | 43.00 |
| 27 | 44.29 | 30.42 | 47.85 |
| 28 | 42.09 | 28.66 | 31.57 |
| 29 | 41.06 | 28.15 | 31.33 |
| 30 | 42.12 | 30.42 | 40.33 |
| 31 | 40.89 | 28.58 | 30.76 |
| 32 | 42.12 | 29.07 | 33.16 |
| 33 | 42.57 | 30.41 | 39.46 |
| 34 | 41.69 | 29.02 | 33.98 |
| 35 | 41.78 | 32.33 | 29.49 |
| 36 | 47.03 | 33.66 | 36.25 |
| 37 | 45.54 | 32.78 | 28.77 |
| 38 | 33.83 | 22.61 | 27.25 |
| 39 | 36.98 | 24.76 | 37.28 |
| 40 | 43.46 | 30.21 | 37.51 |
| 41 | 42.73 | 40.37 | 30.85 |
| 42 | 46.18 | 34.50 | 40.85 |
| 43 | 44.40 | 30.86 | 31.97 |
| 44 | 45.04 | 29.63 | 31.53 |
| 45 | 40.41 | 29.40 | 38.52 |
| 46 | 40.84 | 29.08 | 28.36 |
| 47 | 53.94 | 33.68 | 27.54 |
