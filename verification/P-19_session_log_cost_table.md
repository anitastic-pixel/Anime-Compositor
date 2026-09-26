# P-19: what having the session log on costs

Written by `tests/p19_session_log.rs` under `cargo test --release --test p19_session_log -- --ignored --nocapture`. The reference shot, 240 frames a pass from its first, each drawn the way the window draws one (the viewer's cache, emptied before every pass; the next frame's drawings read ahead). Passes alternate off and on, three rounds each. **ms** is one frame from asking to encoded pixels, P-15's measure and the one a row records; the read-ahead it starts is not waited for, as in the window.

| Quality | Log | Round | ms p50 | ms p95 |
|---|---|---|---|---|
| Draft | off | 1 | 13.65 | 40.33 |
| Draft | on | 1 | 7.18 | 26.47 |
| Draft | off | 2 | 6.59 | 25.65 |
| Draft | on | 2 | 6.16 | 25.45 |
| Draft | off | 3 | 6.47 | 26.22 |
| Draft | on | 3 | 5.97 | 26.79 |
| Full | off | 1 | 22.43 | 36.00 |
| Full | on | 1 | 22.67 | 35.31 |
| Full | off | 2 | 23.19 | 38.10 |
| Full | on | 2 | 23.57 | 38.33 |
| Full | off | 3 | 23.80 | 38.32 |
| Full | on | 3 | 23.56 | 37.64 |
