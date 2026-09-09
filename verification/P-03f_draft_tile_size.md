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
| the reference shot | 4 | 128px | 12 | 1.925 | 1.777 | - |
| the reference shot | 4 | 96px | 15 | 1.718 | 1.680 | -10.7%, -5.5% |
| the reference shot | 4 | 64px | 40 | 1.709 | 1.737 | -11.2%, -2.3% |
| the reference shot | 4 | 48px | 60 | 1.602 | 1.668 | -16.8%, -6.1% |
| the reference shot | 4 | 32px | 135 | 1.592 | 1.909 | -17.3%, +7.4% |
| the reference shot | 4 | 24px | 240 | 1.616 | 1.845 | -16.0%, +3.8% |
| the reference shot | 4 | 16px | 510 | 1.776 | 1.850 | -7.7%, +4.1% |
| the declared ten-layer fixture | 10 | 128px | 12 | 4.745 | 4.875 | - |
| the declared ten-layer fixture | 10 | 96px | 15 | 4.207 | 4.068 | -11.3%, -16.6% |
| the declared ten-layer fixture | 10 | 64px | 40 | 4.265 | 4.019 | -10.1%, -17.6% |
| the declared ten-layer fixture | 10 | 48px | 60 | 4.085 | 3.685 | -13.9%, -24.4% |
| the declared ten-layer fixture | 10 | 32px | 135 | 4.166 | 3.925 | -12.2%, -19.5% |
| the declared ten-layer fixture | 10 | 24px | 240 | 4.117 | 3.766 | -13.2%, -22.8% |
| the declared ten-layer fixture | 10 | 16px | 510 | 3.850 | 4.000 | -18.9%, -18.0% |

## What to check by eye

Two things, and the second one matters more than the first.

**The tile column and the tiles column.** Twelve tiles is the row this item exists to replace, and every smaller size gives the machine more pieces than it has threads. The point where the time stops falling is where per-tile overhead starts costing more than the extra parallelism buys, and that point is what the constant is set to - not the smallest size in the table.

**Every row rendered the same picture.** The test compares each render's encoded bytes against the 128px render of the same frame and fails if one byte differs, so the whole table is forty renders of two pictures. That is ADR-011's requirement and the same guarantee `verification/B-07_effects_table.md` makes at six tile sizes; a tile size is a schedule, never a picture.

For scale, the 24 fps budget document 08 sets is 41.667 ms for the whole frame, of which this table is one stage.


## What this changes

`compose::DRAFT_TILE_SIZE` is the row this table picks, and `preview::preview_frame` uses it when the quality is `Draft`. `DEFAULT_TILE_SIZE` is untouched, which is what document 15 requires: **export still renders in 128px tiles**, measured on the extent it renders at, and ADR-015's separation of preview from export is not disturbed by a preview-only tuning.
