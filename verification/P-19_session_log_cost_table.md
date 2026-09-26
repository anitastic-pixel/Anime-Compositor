# P-19: what having the session log on costs

Written by `tests/p19_session_log.rs` under `cargo test --release --test p19_session_log -- --ignored --nocapture`. The reference shot, 240 frames a pass from its first, each drawn the way the window draws one (the viewer's cache, emptied before every pass; the next frame's drawings read ahead). Passes alternate off and on, three rounds each. **ms** is one frame from asking to encoded pixels, P-15's measure and the one a row records; the read-ahead it starts is not waited for, as in the window.

| Quality | Log | Round | ms p50 | ms p95 |
|---|---|---|---|---|
| Draft | off | 1 | 12.11 | 24.64 |
| Draft | on | 1 | 12.34 | 24.11 |
| Draft | off | 2 | 6.37 | 25.17 |
| Draft | on | 2 | 6.25 | 25.69 |
| Draft | off | 3 | 6.49 | 25.73 |
| Draft | on | 3 | 6.64 | 25.62 |
| Full | off | 1 | 21.77 | 35.71 |
| Full | on | 1 | 21.83 | 34.69 |
| Full | off | 2 | 22.17 | 35.54 |
| Full | on | 2 | 22.08 | 35.88 |
| Full | off | 3 | 22.07 | 35.76 |
| Full | on | 3 | 21.93 | 35.50 |
