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
| 32px | 1 | 70.9 | 1.00x |
| 32px | 2 | 45.1 | 1.57x |
| 32px | 4 | 26.5 | 2.68x |
| 32px | 8 | 20.0 | 3.54x |
| 32px | 12 | 17.6 | 4.03x |
| 32px | 24 | 17.0 | 4.18x |
| 64px | 1 | 74.3 | 1.00x |
| 64px | 2 | 44.1 | 1.68x |
| 64px | 4 | 29.2 | 2.54x |
| 64px | 8 | 21.5 | 3.46x |
| 64px | 12 | 20.4 | 3.64x |
| 64px | 24 | 18.3 | 4.07x |
| 128px | 1 | 69.2 | 1.00x |
| 128px | 2 | 47.9 | 1.45x |
| 128px | 4 | 26.7 | 2.59x |
| 128px | 8 | 18.3 | 3.78x |
| 128px | 12 | 17.1 | 4.05x |
| 128px | 24 | 16.8 | 4.12x |
| 256px | 1 | 71.8 | 1.00x |
| 256px | 2 | 44.3 | 1.62x |
| 256px | 4 | 26.6 | 2.70x |
| 256px | 8 | 20.2 | 3.55x |
| 256px | 12 | 17.8 | 4.02x |
| 256px | 24 | 16.2 | 4.44x |

## How to read this

Document 21: "Tile size is a tunable measured on the reference machine, not a constant chosen in advance." That is what the tile column is for. Too small and the render spends its time in scheduling; too large and threads sit idle at the end of the frame while the last few tiles finish. The best row is a measurement, not a number chosen ahead of time, and no default tile size is hard-coded anywhere in `src/`.

Speed-up is measured against the same tile size on one thread, so it describes the scaling of the parallel decomposition rather than comparing tile sizes with each other. Perfect scaling is not expected: frame assembly is serial, the machine has 12 physical cores behind its 24 hardware threads, and a workload that reads four full-resolution layers per frame is partly bound by memory bandwidth.

Every render in this table was compared against the first one and was byte-identical, so nothing here trades correctness for speed.
