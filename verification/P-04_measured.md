Written by `p04_responsiveness` in `app/src/main.rs`. One thread asks for frames of the declared ten-layer fixture; another presses a key over and over and times the answer.

| Quality | Frame p50 (ms) | Frame p95 (ms) | Press p50 (ms) | Press p95 (ms) | Worst press (ms) | Answered inside a frame |
|---|---|---|---|---|---|---|
| Draft | 44.438 | 175.016 | **0.001** | **0.004** | 0.011 | 814 of 815 |
| Full | 67.453 | 213.088 | **0.001** | **0.005** | 0.010 | 877 of 884 |

Copying the project out under the short lock, which is what a frame now does instead of borrowing it: p50 0.0114 ms, p95 0.0116 ms, worst 0.0174 ms over 200.
