# P-15: what a mask costs to rasterise

Produced by `tests/p15_mask_cost.rs`, which is `#[ignore]`d in normal runs. **This measures `mask::coverage` and nothing around it.**

The owner's window measured 17750.1 ms inside a single Draft frame while a mask point was being dragged, against the 2.502 ms `verification/P-01_frame_trace.md` measures for a warm Draft frame with no mask on it. This table is where that time was.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: cargo release profile, `opt-level = 3`
- Debug assertions in this build: false
- One mask, mode Add, opacity 1. Outline of 8 bezier corners.
- Threads available to rayon: 24

## The field, sample by sample and by scanline

No feather in this table: the field is the part P-15 changed. "Sample by sample" is the rule the build followed until 2026-09-22, one run; "now" is the median of five.

| Layer | Mask | Outline edges | Sample by sample, ms | Now, ms | Times faster |
|---|---|---|---|---|---|
| 1920x1080 | plain | 1392 | 30567.8 | 6.33 | 4828x |
| 1920x1080 | expanded 4 px | 1392 | 269706.1 | 32.68 | 8254x |
| 1280x720 | plain | 928 | 8640.7 | 2.96 | 2918x |
| 1280x720 | expanded 4 px | 928 | 82252.4 | 11.53 | 7135x |
| 480x270 | plain | 352 | 482.3 | 0.54 | 893x |
| 480x270 | expanded 4 px | 352 | 4101.9 | 1.99 | 2058x |

The smallest gain in the table is 893 times.

## One thread and all of them

The same call at 1920x1080, run inside a rayon pool of one thread and inside the default pool, median of five each. This separates what the new rule saved from what the threads saved, and it is the only honest way to say how much the threads are worth on a machine that has twenty-four of them and a layer that has two million pixels.

| Mask | One thread, ms | All threads, ms | Times faster |
|---|---|---|---|
| plain | 34.34 | 6.14 | 5.6x |
| expanded 4 px | 238.12 | 23.56 | 10.1x |
| feathered 20 px | 145.92 | 16.56 | 8.8x |

## How to read this

**The layer size is the layer's, not the preview's.** A mask is rasterised in layer space, before the transform and before the composite, so a Draft preview of a 1080p cel still rasterises the mask at 1920x1080. That is why switching to Draft did nothing for the owner's drag, and it is a property of where document 21 puts the mask rather than a fault.

**A drag pays this once per update per masked layer.** The figure to compare against is the 41.667 ms a 24 fps clock allows, and for a drag, the tens of milliseconds a hand notices.

**The same picture, either column.** `verification/B-06_mask_table.md` is where that is checked, not here: two of its rows compare the plain field against the sample-by-sample field pixel for pixel, and two more compare the expanded field against an independently written one. This file only says how long each took.
