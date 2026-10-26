# B-777: drawings kept at 8 bits - the timing

Written from `tests/b777_compact_cels.rs`'s `b777_compact_cels_timing` (ignored; `--release --ignored`), for D-897.

**What it does.** Document 08's ten-layer shot (`tests/common` `build_fixture`, 166 distinct 1920x1080 drawings) drawn on the processor at Draft through `preview_frame_cached`, with the viewer's cache (`CelCache::viewer_sized`) at four memory settings. Three loops of frames 0 to 239, then 240 frames in a scattered order (frame i x 97 mod 240). Timings are of loop 3 and of the scattered order. A decode is a drawing read from its file again (the cache's miss count).

**Machine and build.** AMD Ryzen 9 9900X (24 threads), 64 GB, Windows 11; release build; base main 9be18462 ("before") against this build ("after"), the same test source compiled against each. Five rounds, before and after alternating (after first in even rounds). Other lanes were building and testing on the same machine, so single numbers wander; the table gives the median of the five rounds and the range.

**The settings.** The viewer's memory setting, of which the cels get 9/16 (D-105, P-11). 16 GiB is about this machine's Automatic (15.4 GiB); 8 GiB is a 32 GB machine's Automatic; 1 GiB is the smallest. In the app the cels share the setting with finished frames (B-154), so they have less than this as playback is remembered.

| Setting | Build | Drawings held | MiB held | Decodes in loop 3 | Loop 3 median ms | Loop 3 p95 ms | Scattered median ms | Scattered p95 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1 GiB | before | 18 | 570 | 1290 | 42.5 (38.1-62.3) | 60.5 (57.7-105.3) | 49.3 (48.0-71.5) | 65.2 (63.1-176.7) |
| 1 GiB | after | 18 | 570 | 1290 | 39.9 (36.4-44.4) | 62.5 (54.6-70.7) | 49.5 (47.4-52.8) | 69.9 (62.5-74.3) |
| 4 GiB | before | 72 | 2278 | 1287 | 41.5 (40.2-68.1) | 60.6 (58.4-106.4) | 50.8 (47.4-58.0) | 77.8 (64.7-89.9) |
| 4 GiB | after | 166 | 2286 | 0 | 20.6 (19.0-24.1) | 37.5 (34.9-44.3) | 29.1 (27.8-33.5) | 44.0 (41.8-64.8) |
| 8 GiB | before | 145 | 4588 | 237 | 10.9 (9.7-12.5) | 34.3 (28.2-38.3) | 39.1 (38.6-44.0) | 59.3 (57.3-88.3) |
| 8 GiB | after | 166 | 4588 | 0 | 10.8 (8.9-20.7) | 23.3 (20.9-45.2) | 13.2 (12.7-21.3) | 25.0 (21.5-91.4) |
| 16 GiB | before | 166 | 5252 | 0 | 9.0 (8.5-19.3) | 12.6 (9.8-26.2) | 9.0 (8.9-12.0) | 12.7 (10.2-26.2) |
| 16 GiB | after | 166 | 5252 | 0 | 8.9 (8.4-11.5) | 10.6 (9.5-107.2) | 9.2 (8.5-12.3) | 12.8 (10.5-26.8) |

40 lines in all, five rounds of each build at each setting. The raw lines are in `S/wave11/L6/rounds.jsonl` (the lane's scratch folder).

