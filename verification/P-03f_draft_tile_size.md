# P-03(f) the draft tile size

Document 15's P-03, item (f). `compose::DEFAULT_TILE_SIZE` is 128 pixels because `verification/B-05a_scaling_table.md` measured it on a 1920x1080 frame. A draft preview is 480x270 - a quarter in each direction - and the same 128 pixels cut that into four columns of three: **twelve pieces of work for twenty-four hardware threads**, so half the machine has nothing to do however fast each piece is.

This is the same measurement B-05a made, made again at the extent the viewer actually renders at. Produced by `tests/p03f_draft_tile_size.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: the draft plan of twenty frames scattered across each fixture's 240-frame work area, 480x270
- Only `render::render` is timed. The decode, the transfer function and the encode are outside the span on purpose: this item moves none of them, and at a first playthrough they are large enough to hide it entirely
- Each figure is the median of twenty renders, preceded by one untimed render at the same size

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the renderer.

## Measurements

| Workload | Layers | Tile | Tiles | Sweep 1 p50 (ms) | Sweep 2 p50 (ms) |          Against 128px |
|---|---|---|---|---|---|---|
| the reference shot | 4 | 128px | 12 | 1.509 | 1.605 | - |
| the reference shot | 4 | 96px | 15 | 1.359 | 1.390 | -10.0%, -13.4% |
| the reference shot | 4 | 64px | 40 | 1.657 | 1.473 | +9.8%, -8.2% |
| the reference shot | 4 | 48px | 60 | 1.472 | 1.540 | -2.5%, -4.1% |
| the reference shot | 4 | 32px | 135 | 1.444 | 1.433 | -4.3%, -10.8% |
| the reference shot | 4 | 24px | 240 | 1.464 | 1.518 | -3.0%, -5.4% |
| the reference shot | 4 | 16px | 510 | 1.702 | 1.663 | +12.8%, +3.6% |
| the declared ten-layer fixture | 10 | 128px | 12 | 4.862 | 3.774 | - |
| the declared ten-layer fixture | 10 | 96px | 15 | 3.637 | 3.581 | -25.2%, -5.1% |
| the declared ten-layer fixture | 10 | 64px | 40 | 3.691 | 3.599 | -24.1%, -4.6% |
| the declared ten-layer fixture | 10 | 48px | 60 | 3.535 | 3.533 | -27.3%, -6.4% |
| the declared ten-layer fixture | 10 | 32px | 135 | 3.680 | 3.580 | -24.3%, -5.1% |
| the declared ten-layer fixture | 10 | 24px | 240 | 3.455 | 3.607 | -28.9%, -4.4% |
| the declared ten-layer fixture | 10 | 16px | 510 | 3.697 | 3.672 | -24.0%, -2.7% |

## What to check by eye

Two things, and the second one matters more than the first.

**The tile column and the tiles column.** Twelve tiles is the row this item exists to replace, and every smaller size gives the machine more pieces than it has threads. The point where the time stops falling is where per-tile overhead starts costing more than the extra parallelism buys, and that point is what the constant is set to - not the smallest size in the table.

**Every row rendered the same picture.** The test compares each render's encoded bytes against the 128px render of the same frame and fails if one byte differs, so the whole table is forty renders of two pictures. That is ADR-011's requirement and the same guarantee `verification/B-07_effects_table.md` makes at six tile sizes; a tile size is a schedule, never a picture.

For scale, the 24 fps budget document 08 sets is 41.667 ms for the whole frame, of which this table is one stage.


## What this changes

`compose::DRAFT_TILE_SIZE` is the row this table picks, and `preview::preview_frame` uses it when the quality is `Draft`. `DEFAULT_TILE_SIZE` is untouched, which is what document 15 requires: **export still renders in 128px tiles**, measured on the extent it renders at, and ADR-015's separation of preview from export is not disturbed by a preview-only tuning.
