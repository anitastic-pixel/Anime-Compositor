# B-1004: playback on the processor from memory (D-1141)

Written by `tools/b1004_playback.py report` from `B-1004_playback_measured.json`; every number measured, none estimated.
Each run: a fresh release build copy on its own web view profile, the window placed off the screens
(ANIME_COMPOSITOR_BACKGROUND=1), 1920x1080, Full, the processor (the card switched off), played until every
frame of the work area was in memory, then three loops timed. **On screen was not measured**: every launch
in this work is kept off the person's screens, so a window on screen could not be started.

## Before any change (316bb88c)

Machine: AMD Ryzen 9 9900X 12-Core Processor | 12 cores 24 threads | 61.5 GB | Microsoft Windows 11 Education 10.0.26300. Build: `anime_compositor_app.exe` release, written 2026-11-21T23:21:46, checkout 316bb88c, measured 2026-11-21T23:26:42.

| shot | memory filled | frames held | three loops | frames asked for (page) | frames passed over (session log) | frames shown (session log) | page's rounds: median ms | slowest 5% ms |
|---|---|---|---|---|---|---|---|---|
| P-28 shot | 11.04 s | 120 (996 MB) | 15.05 s (360 frames played) | 261 | **100** | not counted | 51.9 | 67.7 |
| reference shot | 5.23 s | 240 (1992 MB) | 30.0 s (720 frames played) | 598 | **122** | not counted | 50.0 | 51.8 |

Where one round goes, median and slowest 5% in ms (a round is the time between two frames asked for):

| shot | ask | window | bytes | put | rest | wait | round | bytes a frame | page timer: median / slowest 5% / longest gap ms |
|---|---|---|---|---|---|---|---|---|---|
| P-28 shot | 40.9 / 45.7 | 1.6 / 2.1 | 6.1 / 7.0 | 0.4 / 0.5 | 1.8 / 2.2 | 6.6 / 13.9 | 51.9 / 67.7 | 8294400 | 16.7 / 16.8 / 17.0 |
| reference shot | 34.9 / 37.4 | 1.4 / 1.7 | 5.7 / 6.2 | 0.3 / 0.4 | 1.6 / 1.8 | 5.9 / 7.9 | 50.0 / 51.8 | 8294400 | 16.7 / 16.8 / 17.0 |

The same transport timed alone, played and stopped, every frame from memory (median of six; ms from asking to the
last byte in the page, less the window's own time):

| shot | Full 1920x1080 | Half | Third | Quarter (Draft) | no pixels (D-886, the frame the page holds) | 1 / 2 / 3 / 4 whole frames asked at once: ms for all |
|---|---|---|---|---|---|---|
| P-28 shot | 40.9 (8.29 MB) | 11.0 (2.07 MB) | 4.7 (0.92 MB) | 3.2 (0.52 MB) | 1.2 (0.00 MB) | 42.3 / 72.8 / 100.7 / 137.8 |
| reference shot | 39.6 (8.29 MB) | 10.8 (2.07 MB) | 4.8 (0.92 MB) | 2.9 (0.52 MB) | 1.0 (0.00 MB) | 41.9 / 63.7 / 87.0 / 110.7 |

ask: the request's start to the answer's headers, less the window's own time; window: the window's own time (x-ms);
bytes: the headers to the last byte in the page; put: putImageData onto the canvas; rest: the rest of the page's work
on the answer; wait: from that to the next frame asked for (the page's timer). At 24 frames a second a round must
stay under 41.7 ms for nothing to be passed over.
