# P-18: a first playthrough in real time, reading ahead off and on

Written by `tests/p18_read_ahead.rs` under `cargo test --release --test p18_read_ahead -- --ignored --nocapture`. Draft quality, the viewer's own cache (`CelCache::viewer`), emptied before every pass, and one pass through the work area in real time at 24 fps. **Gap** stands in for what the page does between receiving one frame and asking for the next; it is an assumption, not a measurement. **Wait** is from asking for a frame to having its pixels, so it includes any time spent waiting for the read-ahead to finish.

## the reference shot (4 layers)

| Gap | Reading ahead | Round | Frames shown | Frames dropped | Wait p50 ms | Wait p95 ms |
|---|---|---|---|---|---|---|
| 0 ms | off | 1 | 1888 | 0 | 2.4 | 14.7 |
| 0 ms | on | 1 | 1949 | 0 | 2.4 | 12.7 |
| 0 ms | off | 2 | 1984 | 0 | 2.2 | 14.0 |
| 0 ms | on | 2 | 1973 | 0 | 2.4 | 12.7 |
| 8 ms | off | 1 | 604 | 0 | 2.6 | 18.1 |
| 8 ms | on | 1 | 676 | 0 | 3.0 | 14.5 |
| 8 ms | off | 2 | 587 | 0 | 2.8 | 18.5 |
| 8 ms | on | 2 | 660 | 0 | 3.4 | 15.4 |

Every one of the 1949 frames shown by the first pass with reading ahead on is byte-identical to the same frame rendered with no cache. A frame is due every 41.7 ms; a dropped frame is one the clock passed over because the one before it was not ready in time (D-32).

## the declared ten-layer fixture (10 layers)

| Gap | Reading ahead | Round | Frames shown | Frames dropped | Wait p50 ms | Wait p95 ms |
|---|---|---|---|---|---|---|
| 0 ms | off | 1 | 377 | 0 | 25.7 | 51.2 |
| 0 ms | on | 1 | 400 | 0 | 22.5 | 38.9 |
| 0 ms | off | 2 | 404 | 0 | 24.7 | 49.9 |
| 0 ms | on | 2 | 423 | 0 | 20.5 | 38.1 |
| 8 ms | off | 1 | 231 | 18 | 36.0 | 50.6 |
| 8 ms | on | 1 | 333 | 0 | 16.2 | 30.8 |
| 8 ms | off | 2 | 230 | 15 | 36.1 | 50.1 |
| 8 ms | on | 2 | 339 | 0 | 16.0 | 30.4 |

Every one of the 400 frames shown by the first pass with reading ahead on is byte-identical to the same frame rendered with no cache. A frame is due every 41.7 ms; a dropped frame is one the clock passed over because the one before it was not ready in time (D-32).

