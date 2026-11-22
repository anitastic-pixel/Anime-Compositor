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

## After: the session log counts frames shown (b1a24bd3 plus the change; transport unchanged)

Machine: AMD Ryzen 9 9900X 12-Core Processor | 12 cores 24 threads | 61.5 GB | Microsoft Windows 11 Education 10.0.26300. Build: `anime_compositor_app.exe` release, written 2026-11-21T23:32:31, checkout b1a24bd3, measured 2026-11-21T23:34:01.

| shot | memory filled | frames held | three loops | frames asked for (page) | frames passed over (session log) | frames shown (session log) | page's rounds: median ms | slowest 5% ms |
|---|---|---|---|---|---|---|---|---|
| P-28 shot | 11.1 s | 120 (996 MB) | 15.03 s (360 frames played) | 274 | **88** | 273 | 50.2 | 66.9 |
| reference shot | 5.44 s | 240 (1992 MB) | 30.02 s (720 frames played) | 621 | **99** | 621 | 48.6 | 50.0 |
| P-28 shot, on the card | 6.36 s | 120 (996 MB) | 15.0 s (360 frames played) | 900 | **0** | 360 | not timed (the card paints the window itself) | |

Where one round goes, median and slowest 5% in ms (a round is the time between two frames asked for):

| shot | ask | window | bytes | put | rest | wait | round | bytes a frame | page timer: median / slowest 5% / longest gap ms |
|---|---|---|---|---|---|---|---|---|---|
| P-28 shot | 37.3 / 45.1 | 1.5 / 1.9 | 5.9 / 6.6 | 0.4 / 0.5 | 1.8 / 2.1 | 5.9 / 13.9 | 50.2 / 66.9 | 8294400 | 16.7 / 16.8 / 17.0 |
| reference shot | 35.4 / 39.1 | 1.4 / 1.7 | 5.8 / 6.2 | 0.3 / 0.4 | 1.7 / 1.9 | 3.7 / 4.9 | 48.6 / 50.0 | 8294400 | 6.9 / 7.1 / 13.9 |

The same transport timed alone, played and stopped, every frame from memory (median of six; ms from asking to the
last byte in the page, less the window's own time):

| shot | Full 1920x1080 | Half | Third | Quarter (Draft) | no pixels (D-886, the frame the page holds) | 1 / 2 / 3 / 4 whole frames asked at once: ms for all |
|---|---|---|---|---|---|---|
| P-28 shot | 39.6 (8.29 MB) | 11.0 (2.07 MB) | 4.6 (0.92 MB) | 2.9 (0.52 MB) | 1.2 (0.00 MB) | 42.3 / 65.3 / 86.4 / 110.9 |
| reference shot | 40.1 (8.29 MB) | 11.0 (2.07 MB) | 4.6 (0.92 MB) | 3.2 (0.52 MB) | 1.2 (0.00 MB) | 153.5 / 128.2 / 111.2 / 131.3 |

## D-1142 before: the frame request only (2a7220b0)

Machine: AMD Ryzen 9 9900X 12-Core Processor | 12 cores 24 threads | 61.5 GB | Microsoft Windows 11 Education 10.0.26300. Build: `anime_compositor_app.exe` release, written 2026-11-22T07:11:20, checkout 2a7220b0, measured 2026-11-22T07:12:46.

| shot | memory filled | frames held | three loops | frames asked for (page) | frames passed over (session log) | frames shown (session log) | page's rounds: median ms | slowest 5% ms |
|---|---|---|---|---|---|---|---|---|
| P-28 shot | 19.81 s | 120 (996 MB) | 15.04 s (360 frames played) | 211 | **151** | 211 | 70.7 | 83.6 |
| reference shot | 5.52 s | 240 (1991 MB) | 30.06 s (720 frames played) | 519 | **203** | 519 | 55.5 | 73.3 |
| P-28 shot, on the card | 7.78 s | 120 (996 MB) | 15.0 s (360 frames played) | 2154 | **0** | 361 | not timed (the card paints the window itself) | |

Where one round goes, median and slowest 5% in ms (a round is the time between two frames asked for):

| shot | ask | window | bytes | put | rest | wait | round | bytes a frame | page timer: median / slowest 5% / longest gap ms |
|---|---|---|---|---|---|---|---|---|---|
| P-28 shot | 55.5 / 66.4 | 2.1 / 3.3 | 9.0 / 12.2 | 0.5 / 1.3 | 2.8 / 4.5 | 0.1 / 1.9 | 70.7 / 83.6 | 8294400 | 6.9 / 7.1 / 13.9 |
| reference shot | 41.4 / 57.5 | 1.7 / 2.6 | 7.2 / 11.4 | 0.4 / 0.9 | 2.2 / 3.6 | 0.1 / 3.7 | 55.5 / 73.3 | 8294400 | 6.9 / 7.1 / 14.1 |

The same transport timed alone, played and stopped, every frame from memory (median of six; ms from asking to the
last byte in the page, less the window's own time):

| shot | Full 1920x1080 | Half | Third | Quarter (Draft) | no pixels (D-886, the frame the page holds) | 1 / 2 / 3 / 4 whole frames asked at once: ms for all |
|---|---|---|---|---|---|---|
| P-28 shot | 64.8 (8.29 MB) | 17.6 (2.07 MB) | 7.6 (0.92 MB) | 5.4 (0.52 MB) | 2.3 (0.00 MB) | 64.7 / 102.8 / 137.9 / 170.4 |
| reference shot | 63.7 (8.29 MB) | 19.1 (2.07 MB) | 7.5 (0.92 MB) | 5.6 (0.52 MB) | 1.7 (0.00 MB) | 64.9 / 85.8 / 124.0 / 180.4 |

The same frames asked both ways, stopped, from memory: through the shared buffer and as the frame request's body,
compared byte for byte in the page (SHA-256, first 8 bytes in hex):

| shot | frame | bytes through the shared buffer | bytes in the body | byte-identical | SHA-256 shared buffer | SHA-256 body |
|---|---|---|---|---|---|---|
| P-28 shot | | | | **this build has no shared buffer road** | | |
| reference shot | | | | **this build has no shared buffer road** | | |

## D-1142 prototype: shared buffer

Machine: AMD Ryzen 9 9900X 12-Core Processor | 12 cores 24 threads | 61.5 GB | Microsoft Windows 11 Education 10.0.26300. Build: `anime_compositor_app.exe` release, written 2026-11-22T07:15:33, checkout 2a7220b0, measured 2026-11-22T07:15:39.

| shot | memory filled | frames held | three loops | frames asked for (page) | frames passed over (session log) | frames shown (session log) | page's rounds: median ms | slowest 5% ms | road (session log) | frames through the shared buffer |
|---|---|---|---|---|---|---|---|---|---|---|
| P-28 shot | 14.5 s | 120 (996 MB) | 15.01 s (360 frames played) | 2040 | **0** | 361 | 7.0 | 10.6 | the shared buffer (D-1142) | 2039 of 2040 |
| reference shot | 5.78 s | 240 (1992 MB) | 30.01 s (720 frames played) | 3580 | **0** | 721 | 7.1 | 13.4 | the shared buffer (D-1142) | 3580 of 3580 |
| P-28 shot, on the card | 8.49 s | 120 (996 MB) | 15.01 s (360 frames played) | 2107 | **0** | 361 | not timed (the card paints the window itself) | | no frame played yet | 0 of 2107 |

Where one round goes, median and slowest 5% in ms (a round is the time between two frames asked for):

| shot | ask | window | bytes | put | rest | wait | round | bytes a frame | page timer: median / slowest 5% / longest gap ms |
|---|---|---|---|---|---|---|---|---|---|
| P-28 shot | 2.3 / 3.7 | 1.4 / 2.1 | 0.0 / 0.0 | 0.4 / 0.6 | 1.6 / 2.5 | 1.3 / 2.0 | 7.0 / 10.6 | 8294400 | 6.9 / 7.1 / 7.5 |
| reference shot | 2.5 / 4.6 | 1.6 / 3.6 | 0.0 / 0.0 | 0.5 / 0.8 | 1.7 / 3.3 | 1.1 / 2.9 | 7.1 / 13.4 | 8294400 | 6.9 / 7.1 / 14.1 |

The same transport timed alone, played and stopped, every frame from memory (median of six; ms from asking to the
last byte in the page, less the window's own time):

| shot | Full 1920x1080 | Half | Third | Quarter (Draft) | no pixels (D-886, the frame the page holds) | 1 / 2 / 3 / 4 whole frames asked at once: ms for all |
|---|---|---|---|---|---|---|
| P-28 shot | 46.8 (8.29 MB) | 16.2 (2.07 MB) | 5.6 (0.92 MB) | 3.4 (0.52 MB) | 1.3 (0.00 MB) | 55.4 / 76.0 / 116.1 / 143.8 |
| reference shot | 69.1 (8.29 MB) | 20.4 (2.07 MB) | 7.9 (0.92 MB) | 6.7 (0.52 MB) | 2.3 (0.00 MB) | 58.2 / 91.4 / 147.8 / 144.6 |

The same frames asked both ways, stopped, from memory: through the shared buffer and as the frame request's body,
compared byte for byte in the page (SHA-256, first 8 bytes in hex):

| shot | frame | bytes through the shared buffer | bytes in the body | byte-identical | SHA-256 shared buffer | SHA-256 body |
|---|---|---|---|---|---|---|
| P-28 shot | 0 | 8294400 | 8294400 | **yes** | 79f925839dfdb79a | 79f925839dfdb79a |
| P-28 shot | 40 | 8294400 | 8294400 | **yes** | 704b4712a1a1c6bf | 704b4712a1a1c6bf |
| P-28 shot | 119 | 8294400 | 8294400 | **yes** | 8ea4a19b89f6f6e9 | 8ea4a19b89f6f6e9 |
| reference shot | 0 | 8294400 | 8294400 | **yes** | 987795d37e39a902 | 987795d37e39a902 |
| reference shot | 80 | 8294400 | 8294400 | **yes** | a2fd5b7f028dffe5 | a2fd5b7f028dffe5 |
| reference shot | 239 | 8294400 | 8294400 | **yes** | 25961111ac1c95b6 | 25961111ac1c95b6 |

ask: the request's start to the answer's headers, less the window's own time; window: the window's own time (x-ms);
bytes: the headers to the last byte in the page; put: putImageData onto the canvas; rest: the rest of the page's work
on the answer; wait: from that to the next frame asked for (the page's timer). At 24 frames a second a round must
stay under 41.7 ms for nothing to be passed over.
