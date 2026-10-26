# B-777: drawings kept at 8 bits - the timing

Written from `tests/b777_compact_cels.rs`'s `b777_compact_cels_timing` (ignored; `--release --ignored`), for D-897.

**What it does.** Document 08's ten-layer shot (`tests/common` `build_fixture`, 166 distinct 1920x1080 drawings) drawn on the processor at Draft through `preview_frame_cached`, with the viewer's cache (`CelCache::viewer_sized`) at four memory settings. Three loops of frames 0 to 239, then 240 frames in a scattered order (frame i x 97 mod 240). Timings are of loop 3 and of the scattered order. A decode is a drawing read from its file again (the cache's miss count).

**Machine and build.** AMD Ryzen 9 9900X (24 threads), 64 GB, Windows 11; release build; base main 9be18462 ("before") against this build ("after"), the same test source compiled against each. Five rounds, before and after alternating (after first in even rounds). Other lanes were building and testing on the same machine, so single numbers wander; the table gives the median of the five rounds and the range.

**The settings.** The viewer's memory setting, of which the cels get 9/16 (D-105, P-11). 16 GiB is about this machine's Automatic (15.4 GiB); 8 GiB is a 32 GB machine's Automatic; 1 GiB is the smallest. In the app the cels share the setting with finished frames (B-154), so they have less than this as playback is remembered.

| Setting | Build | Drawings held | MiB held | Decodes in loop 3 | Loop 3 median ms | Loop 3 p95 ms | Scattered median ms | Scattered p95 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1 GiB | before | 18 | 570 | 1290 | 39.9 (36.8-41.5) | 63.2 (58.9-68.0) | 49.6 (46.4-54.7) | 68.9 (60.5-80.0) |
| 1 GiB | after | 72 | 570 | 1287 | 43.2 (41.1-46.1) | 62.0 (61.4-67.7) | 55.8 (52.4-60.1) | 71.8 (65.3-84.2) |
| 4 GiB | before | 72 | 2278 | 1287 | 41.6 (38.8-55.2) | 67.0 (60.8-257.9) | 49.7 (47.4-52.7) | 69.4 (65.9-75.0) |
| 4 GiB | after | 166 | 2294 | 0 | 19.2 (17.5-20.8) | 30.4 (23.0-35.0) | 23.0 (20.4-26.3) | 33.4 (26.3-36.6) |
| 8 GiB | before | 145 | 4588 | 237 | 10.0 (9.7-10.8) | 31.8 (30.1-32.4) | 39.8 (37.8-44.5) | 62.3 (59.7-75.8) |
| 8 GiB | after | 166 | 4604 | 0 | 18.1 (17.8-19.6) | 29.1 (26.0-31.6) | 19.3 (18.0-19.5) | 30.4 (23.6-32.3) |
| 16 GiB | before | 166 | 5252 | 0 | 8.8 (8.8-9.7) | 13.8 (10.3-16.5) | 9.1 (8.7-9.9) | 14.6 (9.8-20.0) |
| 16 GiB | after | 166 | 6565 | 0 | 9.2 (8.7-12.6) | 14.7 (10.6-19.8) | 9.2 (9.1-14.2) | 17.2 (10.8-27.9) |

40 lines in all, five rounds of each build at each setting. The raw lines are in `S/wave11/L6/rounds.jsonl` (the lane's scratch folder).

