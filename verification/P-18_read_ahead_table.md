# P-18: a first playthrough in real time, reading ahead off and on

Written by `tests/p18_read_ahead.rs` under `cargo test --release --test p18_read_ahead -- --ignored --nocapture`. Draft quality, the viewer's own cache (`CelCache::viewer`), emptied before every pass, and one pass through the work area in real time at 24 fps. **Gap** stands in for what the page does between receiving one frame and asking for the next; it is an assumption, not a measurement. **Wait** is from asking for a frame to having its pixels, so it includes any time spent waiting for the read-ahead to finish.

## the reference shot (4 layers)

| Gap | Reading ahead | Round | Frames shown | Frames dropped | Wait p50 ms | Wait p95 ms |
|---|---|---|---|---|---|---|
| 0 ms | off | 1 | 1789 | 0 | 2.4 | 15.6 |
| 0 ms | on | 1 | 1727 | 0 | 2.7 | 13.7 |
| 0 ms | off | 2 | 1456 | 0 | 3.0 | 17.9 |
| 0 ms | on | 2 | 1657 | 0 | 2.9 | 14.6 |
| 8 ms | off | 1 | 534 | 0 | 3.5 | 20.8 |
| 8 ms | on | 1 | 648 | 0 | 3.1 | 15.8 |
| 8 ms | off | 2 | 579 | 0 | 2.8 | 19.1 |
| 8 ms | on | 2 | 654 | 0 | 3.1 | 15.5 |

Every one of the 1727 frames shown by the first pass with reading ahead on is byte-identical to the same frame rendered with no cache. A frame is due every 41.7 ms; a dropped frame is one the clock passed over because the one before it was not ready in time (D-32).

## the declared ten-layer fixture (10 layers)

| Gap | Reading ahead | Round | Frames shown | Frames dropped | Wait p50 ms | Wait p95 ms |
|---|---|---|---|---|---|---|
| 0 ms | off | 1 | 342 | 0 | 27.5 | 52.8 |
| 0 ms | on | 1 | 376 | 0 | 23.9 | 39.4 |
| 0 ms | off | 2 | 299 | 1 | 37.0 | 54.7 |
| 0 ms | on | 2 | 377 | 0 | 23.8 | 39.3 |
| 8 ms | off | 1 | 211 | 31 | 39.0 | 54.5 |
| 8 ms | on | 1 | 308 | 0 | 20.1 | 35.1 |
| 8 ms | off | 2 | 217 | 25 | 37.9 | 52.7 |
| 8 ms | on | 2 | 305 | 0 | 20.0 | 39.9 |

Every one of the 376 frames shown by the first pass with reading ahead on is byte-identical to the same frame rendered with no cache. A frame is due every 41.7 ms; a dropped frame is one the clock passed over because the one before it was not ready in time (D-32).

