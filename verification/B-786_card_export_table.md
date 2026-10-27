# B-786: export on the graphics card against the processor (D-907, PROPOSED, off)

Written by `tests/b786_card_export.rs` (`b786_card_export_table`, release, run by name). Each export is the whole job as the export window runs it (`export_sequence`, frames drawn several at once, PNG encoded beside), timed from the call to its report. The processor, the card with one connection (one frame on the card at a time) and the card with three (three frames at once) take turns, 5 rounds, the first to go moving on each round; the median is shown, and each round below it. The machine is shared with other lanes of wave 11, so the times are the machine's load as well as the work.

The pixel columns compare each card frame with the processor's frame of the last round, 8-bit straight sRGB as written; colour where both are fully transparent is not compared (B-754). The viewer's bar is 1 level of 255 (D-165).

| Export | Whole export (median) | Faster than the processor by | Pixels within 1 level of the processor's | Largest difference (levels) | Frames the same bytes |
|---|---|---|---|---|---|
| reference shot, frames 0 to 239, 8-bit PNG, processor | 12596 ms (52.5 ms a frame) | 1.00x | | | |
|  | rounds: 12835, 12524, 12626, 12596, 12191 | | | | |
| reference shot, card, 1 connection | 13378 ms (55.7 ms a frame) | 0.94x | 497664000 of 497664000 (100.0000%) | 1 (frame 0) | 0 of 240 |
|  | rounds: 13378, 12863, 11537, 13956, 15482 | | | | |
| reference shot, card, 3 connections | 15491 ms (64.5 ms a frame) | 0.81x | 497664000 of 497664000 (100.0000%) | 1 (frame 0) | 0 of 240 |
|  | rounds: 15434, 15494, 12412, 16250, 15491 | | | | |

<!-- reference shot report, card: Exported on the graphics card (D-907, a trial): the card drew 240 of 240 frames. Its pixels can differ a little from the processor's, so its files are not the processor's bytes. The card drew frames 0 to 239. -->

| 25 layers, frames 0 to 47, 8-bit PNG, processor | 15540 ms (323.8 ms a frame) | 1.00x | | | |
|  | rounds: 15540, 19800, 18401, 15056, 15169 | | | | |
| 25 layers, card, 1 connection | 6306 ms (131.4 ms a frame) | 2.46x | 99532800 of 99532800 (100.0000%) | 1 (frame 0) | 0 of 48 |
|  | rounds: 4412, 7041, 6810, 5741, 6306 | | | | |
| 25 layers, card, 3 connections | 8934 ms (186.1 ms a frame) | 1.74x | 99532800 of 99532800 (100.0000%) | 1 (frame 0) | 0 of 48 |
|  | rounds: 6631, 10370, 15105, 8934, 8597 | | | | |

<!-- 25 layers report, card: Exported on the graphics card (D-907, a trial): the card drew 48 of 48 frames. Its pixels can differ a little from the processor's, so its files are not the processor's bytes. The card drew frames 0 to 47. -->


The 25-layer project is the reference shot's four layers and 21 copies, moved, scaled, turned, faded and laid in overlay, lighten, screen, multiply and add, with a Gaussian Blur of 6 on every fifth copy (both ways, which the card draws).

## EXR float

The working floats as written, linear and premultiplied, card against processor.

| Export | Largest difference | Mean difference |
|---|---|---|
| reference shot, frames 0 to 47, EXR float | 0.000430 (frame 16) | 3.90e-5 |
| 25 layers, frames 0 to 23, EXR float | 0.000429 (frame 12) | 2.28e-5 |
