# B-786: export on the graphics card against the processor (D-907, PROPOSED, off)

Written by `tests/b786_card_export.rs` (`b786_card_export_table`, release, run by name). Each export is the whole job as the export window runs it (`export_sequence`, frames drawn several at once, PNG encoded beside), timed from the call to its report. The processor, the card with one connection (one frame on the card at a time) and the card with three (three frames at once) take turns, 5 rounds, the first to go moving on each round; the median is shown, and each round below it. Re-measured on 2026-10-27 in a quiet slot, nothing else building on the machine (the first run, under other lanes' builds, gave 0.94x and 2.46x).

The pixel columns compare each card frame with the processor's frame of the last round, 8-bit straight sRGB as written; colour where both are fully transparent is not compared (B-754). The viewer's bar is 1 level of 255 (D-165).

| Export | Whole export (median) | Faster than the processor by | Pixels within 1 level of the processor's | Largest difference (levels) | Frames the same bytes |
|---|---|---|---|---|---|
| reference shot, frames 0 to 239, 8-bit PNG, processor | 11776 ms (49.1 ms a frame) | 1.00x | | | |
|  | rounds: 11849, 11706, 11796, 11776, 11732 | | | | |
| reference shot, card, 1 connection | 11031 ms (46.0 ms a frame) | 1.07x | 497664000 of 497664000 (100.0000%) | 1 (frame 0) | 0 of 240 |
|  | rounds: 11031, 11177, 11276, 10815, 10989 | | | | |
| reference shot, card, 3 connections | 11652 ms (48.5 ms a frame) | 1.01x | 497664000 of 497664000 (100.0000%) | 1 (frame 0) | 0 of 240 |
|  | rounds: 11596, 11998, 11612, 11917, 11652 | | | | |

<!-- reference shot report, card: Exported on the graphics card (D-911): the card drew 240 of 240 frames. Its pixels can differ a little from the processor's, so its files are not the processor's bytes. The card drew frames 0 to 239. -->

| 25 layers, frames 0 to 47, 8-bit PNG, processor | 13467 ms (280.6 ms a frame) | 1.00x | | | |
|  | rounds: 13398, 13379, 13646, 13467, 13768 | | | | |
| 25 layers, card, 1 connection | 4012 ms (83.6 ms a frame) | 3.36x | 99532800 of 99532800 (100.0000%) | 1 (frame 0) | 0 of 48 |
|  | rounds: 4055, 4012, 3971, 3990, 4065 | | | | |
| 25 layers, card, 3 connections | 5207 ms (108.5 ms a frame) | 2.59x | 99532800 of 99532800 (100.0000%) | 1 (frame 0) | 0 of 48 |
|  | rounds: 5081, 5156, 5207, 5248, 5299 | | | | |

<!-- 25 layers report, card: Exported on the graphics card (D-911): the card drew 48 of 48 frames. Its pixels can differ a little from the processor's, so its files are not the processor's bytes. The card drew frames 0 to 47. -->


The 25-layer project is the reference shot's four layers and 21 copies, moved, scaled, turned, faded and laid in overlay, lighten, screen, multiply and add, with a Gaussian Blur of 6 on every fifth copy (both ways, which the card draws).

## EXR float

The working floats as written, linear and premultiplied, card against processor.

| Export | Largest difference | Mean difference |
|---|---|---|
| reference shot, frames 0 to 47, EXR float | 0.000430 (frame 16) | 3.90e-5 |
| 25 layers, frames 0 to 23, EXR float | 0.000429 (frame 12) | 2.28e-5 |
