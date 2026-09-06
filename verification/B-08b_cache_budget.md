# B-08b: what the cache is worth, measured

D-37 unparked the cache on a measurement and asked for one back: the same playback, in the same build, on the same machine, with and without it. This is that. Produced by `tests/b08b_cache.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: `verification/B-08a_project.json`, the four-layer reference shot, previewed at draft resolution
- Tile size: `compose::DEFAULT_TILE_SIZE`

- Frames per run: 48, consecutively from frame 0, which is two seconds of the shot at 24 fps

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the cache.

## The same playthrough at four budgets

| Budget | Bytes | Total ms | Median ms per frame | Slowest ms | Frames per second at the median | Decodes | Answered from memory | Evictions | Held at the end |
|---|---|---|---|---|---|---|---|---|---|
| none (the path export takes) | 0 | 4793.4 | 99.93 | 107.47 | 10.0 | 188 | 0 | 0 | 0 |
| one cel | 33177600 | 4836.4 | 100.81 | 114.96 | 9.9 | 188 | 0 | 187 | 33177600 |
| 128 MB (the viewer's default) | 134217728 | 2021.8 | 42.54 | 102.12 | 23.5 | 87 | 101 | 83 | 132710400 |
| 512 MB | 536870912 | 1966.5 | 41.45 | 101.37 | 24.1 | 87 | 101 | 71 | 530841600 |

## How to read this

The first row is the build as it was before D-37: every frame decodes every cel it needs, four decodes a frame, and it is still the path an export takes. Each row below it is the same forty-eight frames with more memory allowed, and the only thing that changes between rows is how many of those decodes happen at all. `tests/b08b_cache.rs` separately checks that the pixels do not change between them, which is the claim that makes this table about speed rather than about a trade.

The 'one cel' row is a real result and not a formality. A cache too small to hold the frame it is working on evicts what it is about to need next, so it answers nothing from memory, evicts once per decode, and finishes no faster than having no cache at all. A cache is not a thing you can add a little of.

The two rows below it are within a fraction of a millisecond of each other on four times the memory, which is why the default is the smaller of them. Playback is sequential: a cel is asked for again within a few frames or not for a long time, so what has to fit is the reuse distance, not the shot. That is also why both rows still evict — neither holds the whole shot, and neither needs to.

What is still not measured here is the transport into the webview, which has its own cost and its own place to be fixed. `verification/B-08_window_shell.md` is where playback is counted end to end, in dropped frames, by photographing a running window, and it is the artifact this change has to move next.
