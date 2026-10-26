# B-744 (D-864): export timing, before and after

Measured with `b744_timing` in `tests/b744_export_read_ahead.rs`, one process per build and shot,
the build before (main 7cc14eb3) and this one alternating within each round.

- Machine: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads, Windows 11. The graphics card
  is not used: export draws on the processor.
- Build: release, `CARGO_INCREMENTAL=0`.
- Configuration: PNG sequence, 8 bits, straight alpha, default tile size, frames at once chosen by
  the export (16 at 1920 by 1080, 4 at 4K).
- **Load: not quiet.** Other lanes were building the whole time: 4 to 13 cargo/rustc processes at
  the start of each round (logged per round in the lane's `ab7.log` and `ab_sub.log`).
- Each number is the median of 7 rounds.

"Frame" is one frame drawn the way export draws it (`compose::render_frame`), median over the
shot's listed frames. "Export" is a whole `export::export_sequence` of the first 8 frames (4 for
t3). "Exported files" is one FNV-1a over every file both exports wrote. It was the same before and
after in every round of every shot.

## Run 1: eleven shots

| Shot | Rounds | Frame (render_frame), before -> after ms | Export frames, Block, before -> after ms | Export frames, Render transparent, before -> after ms | Peak working set before / after MB | Exported files |
|---|---:|---:|---:|---:|---:|---|
| ref_L4 | 7/7 | 77.7 -> 51.3 (1.51x) | 8: 710 -> 705 (1.01x) | 8: 713 -> 727 (0.98x) | 1280 / 1282 | identical, 997fbfda962573eb |
| ref_L10 | 7/7 | 327.0 -> 297.8 (1.10x) | 8: 1837 -> 1627 (1.13x) | 8: 1818 -> 2119 (0.86x) | 3056 / 3057 | identical, 30af49ba9dda7624 |
| ref_L25 | 7/7 | 1167.4 -> 819.9 (1.42x) | 8: 4000 -> 3990 (1.00x) | 8: 4370 -> 4194 (1.04x) | 7362 / 7364 | identical, e600f4b85e8884ef |
| ref_L50 | 7/7 | 1976.7 -> 2061.8 (0.96x) | 8: 8620 -> 7300 (1.18x) | 8: 8697 -> 7792 (1.12x) | 14706 / 14708 | identical, cce01cf18b581be0 |
| ref_4k | 7/7 | 141.5 -> 99.2 (1.43x) | 8: 3337 -> 3096 (1.08x) | 8: 3693 -> 3233 (1.14x) | 1048 / 1050 | identical, 068ed07d283ad131 |
| nest_1 | 7/7 | 94.0 -> 61.3 (1.53x) | 8: 783 -> 754 (1.04x) | 8: 759 -> 716 (1.06x) | 1280 / 1283 | identical, 15ba5ae0ad71fcdc |
| nest_3 | 7/7 | 122.9 -> 83.5 (1.47x) | 8: 744 -> 730 (1.02x) | 8: 773 -> 762 (1.01x) | 1281 / 1248 | identical, 7b7b41001400939d |
| t3_radial_8 | 7/7 | 377.9 -> 375.6 (1.01x) | 4: 1352 -> 1368 (0.99x) | 4: 1359 -> 1595 (0.85x) | 945 / 945 | identical, 40877db79d5a5e8a |
| t2_noise_polar_8 | 7/7 | 390.2 -> 415.3 (0.94x) | 8: 2452 -> 2320 (1.06x) | 8: 2274 -> 2220 (1.02x) | 3897 / 3840 | identical, 129c899981147eaf |
| t5_energy_8 | 7/7 | 376.0 -> 321.2 (1.17x) | 8: 1458 -> 1455 (1.00x) | 8: 1453 -> 1506 (0.96x) | 2320 / 2292 | identical, 9e8031d761a2dc96 |
| t1_explosion_8 | 7/7 | 720.4 -> 569.9 (1.26x) | 8: 2327 -> 2005 (1.16x) | 8: 2323 -> 2060 (1.13x) | 6153 / 6408 | identical, 19b5d0208bf01401 |

## Run 2: four shots again, to check the rows that looked slower

| Shot | Rounds | Frame (render_frame), before -> after ms | Export frames, Block, before -> after ms | Export frames, Render transparent, before -> after ms | Peak working set before / after MB | Exported files |
|---|---:|---:|---:|---:|---:|---|
| ref_L4 | 7/7 | 72.8 -> 44.6 (1.63x) | 8: 656 -> 633 (1.04x) | 8: 842 -> 789 (1.07x) | 1281 / 1282 | identical, 997fbfda962573eb |
| ref_L10 | 7/7 | 248.7 -> 295.5 (0.84x) | 8: 1785 -> 1609 (1.11x) | 8: 1805 -> 1541 (1.17x) | 3056 / 3058 | identical, 30af49ba9dda7624 |
| ref_L25 | 7/7 | 788.6 -> 813.5 (0.97x) | 8: 3820 -> 3762 (1.02x) | 8: 3718 -> 3923 (0.95x) | 7361 / 7363 | identical, e600f4b85e8884ef |
| t3_radial_8 | 7/7 | 454.9 -> 380.4 (1.20x) | 4: 1451 -> 1425 (1.02x) | 4: 1515 -> 1498 (1.01x) | 961 / 960 | identical, 40877db79d5a5e8a |

## What this says

- Every exported file is the same, byte for byte, in every round. Peak memory is the same on the
  shots with drawings, within 2 MB. The shots without any (t1, t2, t5) move by a few percent either
  way, from their effects.
- One frame drawn alone is usually faster: about 1.4 to 1.6 times on the 4-layer, 4K and nested
  shots in both runs. On the 10- and 25-layer shots the gain changes from run to run on this loaded
  machine (ref_L25 was 1.42 times in run 1 and 0.97 in run 2).
- A whole export is at most a little faster: between 0.95 and 1.18 times, which is inside this
  machine's noise. Export already draws up to 16 frames side by side, so the 24 threads were already
  busy. Reading each frame's drawings side by side as well mostly reorders work they were already
  doing.
- The proposal's "ref_L25 export frame 2.8 s to about 1 s" was not reached. Its 2.8 s was one frame
  measured with the stage timers on, and the timers cost much of that themselves. Without them the
  same frame took 0.8 to 1.2 s here before this change.
- Nothing measured got slower beyond the noise. Each row that looked slower in one run was even or
  faster in the other run.
