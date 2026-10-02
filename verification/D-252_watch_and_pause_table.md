# D-252: watch it write, and Pause

Written by `d252_paused_it_waits_then_writes_what_an_unpaused_export_writes` in `app/src/main.rs`. The reference shot, frames 0 to 59, as PNG, with missing drawings written as transparent.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| an export never paused writes sixty frames | 60 | 60 | PASS |
| Pause with nothing running | No export is running. | No export is running. | PASS |
| Pause is said | Pausing after the frame being written. Press Pause again to go on. | Pausing after the frame being written. Press Pause again to go on. | PASS |
| it waits within a few frames of 40: 41 to 45 frames written | 41 to 45 | 41 to 45 | PASS |
| for five seconds the folder does not grow | 41 files | 41 files | PASS |
| the window says it is paused | true | true | PASS |
| the strip holds the five frames last written | [36, 37, 38, 39, 40] | [36, 37, 38, 39, 40] | PASS |
| the picture shown for frame 40 is the written frame 40, shrunk: no byte more than 1 apart | 480 x 270, at most 1 apart | 480 x 270, at most 1 apart | PASS |
| Pause again goes on | Going on. | Going on. | PASS |
| after going on, the folder is the never-paused export's | 60 files, every byte the same | 60 files, every byte the same | PASS |
| the frames marked orange are the ones whose drawing is missing | Frames 14 to 15, 38 to 39. | Frames 14 to 15, 38 to 39. | PASS |
