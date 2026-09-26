# B-05a tiled render, scaling by thread count

ADR-011: "identical output to single-threaded evaluation, plus measured scaling on the reference machine". The identical-output half is `B-05a_tiling_proof.md`; this is the timing half. Produced by `tests/b05a_transform.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: one 1920x1080 frame, four reference-shot layers, each transformed and composited, bilinear sampling throughout
- Each figure is one render, preceded by an untimed warm render on the same pool

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the renderer.

## Measurements

| Tile | Threads | Milliseconds | Speed-up over 1 thread |
|---|---|---|---|
| 32px | 1 | 40.3 | 1.00x |
| 32px | 2 | 24.9 | 1.62x |
| 32px | 4 | 11.6 | 3.46x |
| 32px | 8 | 8.2 | 4.92x |
| 32px | 12 | 7.5 | 5.36x |
| 32px | 24 | 7.4 | 5.45x |
| 64px | 1 | 38.1 | 1.00x |
| 64px | 2 | 22.9 | 1.66x |
| 64px | 4 | 10.8 | 3.53x |
| 64px | 8 | 8.2 | 4.62x |
| 64px | 12 | 6.1 | 6.23x |
| 64px | 24 | 5.6 | 6.84x |
| 128px | 1 | 39.4 | 1.00x |
| 128px | 2 | 26.6 | 1.48x |
| 128px | 4 | 13.0 | 3.02x |
| 128px | 8 | 7.3 | 5.39x |
| 128px | 12 | 7.4 | 5.29x |
| 128px | 24 | 5.8 | 6.77x |
| 256px | 1 | 42.0 | 1.00x |
| 256px | 2 | 23.3 | 1.80x |
| 256px | 4 | 11.9 | 3.53x |
| 256px | 8 | 8.5 | 4.96x |
| 256px | 12 | 8.7 | 4.85x |
| 256px | 24 | 7.6 | 5.53x |

## How to read this

Document 21: "Tile size is a tunable measured on the reference machine, not a constant chosen in advance." That is what the tile column is for. Too small and the render spends its time in scheduling; too large and threads sit idle at the end of the frame while the last few tiles finish. The best row is a measurement, not a number chosen ahead of time, and no default tile size is hard-coded anywhere in `src/`.

Speed-up is measured against the same tile size on one thread, so it describes the scaling of the parallel decomposition rather than comparing tile sizes with each other. Perfect scaling is not expected: frame assembly is serial, the machine has 12 physical cores behind its 24 hardware threads, and a workload that reads four full-resolution layers per frame is partly bound by memory bandwidth.

Every render in this table was compared against the first one and was byte-identical, so nothing here trades correctness for speed.
