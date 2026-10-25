# B-685: the motion finder in single precision (D-804)

Measured by the scratch test `w8b_flow_cost` (kept outside the repository, lane w8b's scratch
folder): each case is drawn twice in one run, once with the finder in double precision (B-684)
and once in single precision (B-685), and the two pictures and the two found motion fields are
compared. Bar (wave 8): at 8 bpc at most 1 level on at least 99.9 % of pixels and at most 2 anywhere;
in float at most 1e-4 on solid areas; no motion moved more than 0.05 px.

## Pixel cost

| set | pixels | within 1 level | largest (levels) | float, solid | float, any | motion moved most (px) | vectors over 0.05 px | finder calls | verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Fixtures/pixel_motion_blur, every frame, Full and Draft | 156672 | 100.00000% | 1 | 2.38e-7 | 2.98e-7 | 2.31e-6 | 0 of 215424 | 396 | pass |
| Fixtures/timewarp, every frame, Full and Draft | 215424 | 100.00000% | 0 | 2.38e-7 | 1.43e-6 | 8.03e-6 | 0 of 187136 | 344 | pass |
| Fixtures/warp_stabilizer, every frame, Full and Draft | 195840 | 100.00000% | 1 | 2.38e-7 | 5.66e-7 | 4.27e-6 | 0 of 45696 | 84 | pass |
| Fixtures/rolling_shutter_repair, every frame, Full and Draft | 169728 | 100.00000% | 0 | 5.36e-7 | 1.43e-6 | 2.25e-5 | 0 of 91392 | 168 | pass |
| reference shot, Pixel Motion Blur as added, frames 0 to 210 every 30th, Full | 16588800 | 100.00000% | 1 | 2.38e-7 | 2.38e-7 | 6.55e-5 | 0 of 33177600 | 16 | pass |
| reference shot, Pixel Motion Blur heavy, frames 0 to 210 every 30th, Full | 16588800 | 100.00000% | 1 | 2.98e-7 | 2.98e-7 | 6.05e-5 | 0 of 33177600 | 16 | pass |
| reference shot, Timewarp speed 50, frames 0 to 210 every 30th, Full | 16588800 | 100.00000% | 0 | 6.85e-7 | 6.85e-7 | 6.32e-5 | 0 of 29030400 | 14 | pass |
| reference shot, Timewarp heavy, frames 0 to 210 every 30th, Full | 16588800 | 100.00000% | 1 | 4.17e-7 | 4.17e-7 | 8.95e-5 | 0 of 33177600 | 16 | pass |
| reference shot, Warp Stabilizer (fx_ws_001's settings), frames 0 to 210 every 30th, Full | 16588800 | 100.00000% | 1 | 9.54e-7 | 9.54e-7 | 6.60e-5 | 0 of 49766400 | 24 | pass |
| reference shot, Rolling Shutter Repair by Warp (fx_rsr_001's settings), same frames | 16588800 | 100.00000% | 0 | 1.94e-7 | 1.94e-7 | 6.05e-5 | 0 of 8294400 | 4 | pass |
| reference shot, Rolling Shutter Repair by Pixel Motion (fx_rsr_001's settings), same frames | 16588800 | 100.00000% | 1 | 2.38e-7 | 2.38e-7 | 6.55e-5 | 0 of 31104000 | 15 | pass |

11 of 11 pass. Every fixture expected value still holds: b530 (Pixel Motion Blur), b531 (Timewarp),
b532 (Warp Stabilizer), b533 (Rolling Shutter Repair), b264 (the finder against its reference) and
the finder's own library test.

## Speed

Machine: RTX 4070 Ti SUPER, 24-thread AMD64 Family 26 Model 68 processor, release build, shared
with other lanes running. The reference shot, 1920 by 1080, frames 0 to 210 every 30th, Full, on the
processor; 7 alternating rounds (before, after, before, ...), median of each.

| case | rounds | finder before (ms) | finder after (ms) | finder speed-up | frame before (ms) | frame after (ms) | frame speed-up |
|---|---:|---:|---:|---:|---:|---:|---:|
| Pixel Motion Blur as added | 7 | 88.8 | 76.8 | 1.16x | 184.2 | 170.7 | 1.08x |
| Timewarp speed 50 | 7 | 81.6 | 65.9 | 1.24x | 182.6 | 166.2 | 1.10x |

W7's viewer timing (`perf_audit_timing`, 5 alternating rounds, medians in ms; columns CPU Draft,
CPU Full, card Draft, card Full, 8 bpc Draft, 8 bpc Full). On this busy machine the rows swing by
more than the change; they are recorded, not claimed. The `(none)` row is no slower.

| row | before | after |
|---|---|---|
| (none) | 4.8 / 25.6 / 12.1 / 18.2 / 61.9 / 79.5 | 3.7 / 22.2 / 17.3 / 16.3 / 56.0 / 75.0 |
| Pixel Motion Blur as added | 78.1 / 287.3 / 74.7 / 320.8 / 72.0 / 343.9 | 82.2 / 242.9 / 59.2 / 253.7 / 92.2 / 317.1 |
| Pixel Motion Blur heavy | 77.5 / 398.8 / 73.0 / 480.1 / 179.1 / 492.9 | 91.0 / 307.8 / 66.1 / 495.4 / 123.7 / 464.4 |
| Timewarp as added | 57.7 / 218.4 / 53.9 / 270.2 / 67.3 / 308.9 | 74.8 / 340.7 / 66.2 / 287.1 / 102.2 / 447.2 |
| Timewarp heavy | 93.4 / 783.3 / 56.8 / 434.9 / 96.7 / 564.9 | 68.5 / 538.7 / 51.9 / 340.2 / 156.4 / 537.0 |

## W7 hash check

`perf_audit_unchanged` against B-684: the processor's floats moved, by design, only in the seven
rows of the four effects (Pixel Motion Blur x2, Timewarp x2, Warp Stabilizer, Rolling Shutter
Repair x2). The `(none)` row and CC Wide Time (which does not use the finder) are unchanged.
`verification/W7_pixel_hashes.txt` is left as taken at 440cecf0.
