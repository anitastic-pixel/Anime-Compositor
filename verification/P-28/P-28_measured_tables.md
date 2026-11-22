## 1. Keys and time (written by `tools/p28_media.py report`)

Each frame asked for out of order in a fresh window, against the same frame in a second fresh window that drew 0..119 in order.

| frame | out of order = in order (processor) | largest difference | card out of order vs processor: largest | pixels off by more than 2 |
|---|---|---|---|---|
| 0 | yes, byte for byte | 0 | 2 | 0 |
| 24 | yes, byte for byte | 0 | 1 | 0 |
| 47 | yes, byte for byte | 0 | 1 | 0 |
| 48 | yes, byte for byte | 0 | 0 | 0 |
| 49 | yes, byte for byte | 0 | 0 | 0 |
| 50 | yes, byte for byte | 0 | 0 | 0 |
| 54 | yes, byte for byte | 0 | 0 | 0 |
| 55 | yes, byte for byte | 0 | 1 | 0 |
| 90 | yes, byte for byte | 0 | 1 | 0 |
| 119 | yes, byte for byte | 0 | 1 | 0 |

Hold frames 48 and 49 identical: processor **yes**, card **yes**. All ten frames the same in both orders: **yes**.

Render problems reported by the window: none.

## 2. Compositing (written by `tools/p28_media.py report`)

Soft edge of the subject at frame 47: 131200 edge pixels (alpha between 2% and 98%), the title's box left out.

| expected as | mean difference (8-bit) | darkest miss (8-bit, negative = darker than expected) | largest miss |
|---|---|---|---|
| mixed in linear light (the project's working space) | +0.00 | -1.0 | 1.0 |
| mixed in sRGB values (what a fringe or gamma slip looks like) | +11.05 | -0.7 | 35.3 |

Inside the subject (fully opaque) the picture equals the subject's own pixels to within 0 (layer order: subject over the plate). The subject-only render has alpha with see-through pixels.

4x crop of the edge at x 740..820, y 500..580: `P-28_edge_4x.png` (the app, then the two expectations).

Screen glow at frame 90: 827020 pixels where the glow copy shows.

| Screen worked out | mean miss (8-bit) | largest miss |
|---|---|---|
| in linear light | 0.20 | 6 |
| in sRGB values | 7.18 | 27 |

The red/blue split (Chromatic Aberration on an adjustment layer, frames 48-54): pixels whose blue is 0.

| frame | pixels with blue 0 | band from the left / right / top / bottom edge (row 540, column 960) |
|---|---|---|
| 47 | 0 | 0 px / 0 px / 0 px / 0 px |
| 48 | 40600 | 10 px / 10 px / 5 px / 5 px |
| 49 | 40600 | 10 px / 10 px / 5 px / 5 px |
| 50 | 40600 | 10 px / 10 px / 5 px / 5 px |
| 54 | 40600 | 10 px / 10 px / 5 px / 5 px |
| 55 | 0 | 0 px / 0 px / 0 px / 0 px |

## 3. Speed (written by `tools/p28_media.py report`)

Machine: AMD Ryzen 9 9900X 12-Core Processor | 12 cores 24 threads | 61.5 GB | Microsoft Windows 11 Education 10.0.26300. Build: `I:\AI Coding\Claude\TotallyNotAfterEffects\target\lanes\p28\release\anime_compositor_app.exe` (release, written 2026-11-21T22:56:27), checkout 89fec3d2. Window placed off the screens (ANIME_COMPOSITOR_BACKGROUND=1), own web view profile, 1920x1080, 24 fps, 120 frames.

| what | where | frames | median ms | fastest | slowest | source |
|---|---|---|---|---|---|---|
| Full, each frame drawn fresh, in order | processor | 120 | 74.6 | 31.3 | 114.9 | window's own time (x-ms) |
| Full, the ten frames out of order | processor | 10 | 87.6 | 31.2 | 113.4 | x-ms |
| Draft (480x270), ten frames | processor | 10 | 39.4 | 0.5 | 52.5 | x-ms |
| Full, ten frames out of order | card | 30 log rows | 2.5 | - | 95.2 | session log (its slowest 5%) |
| Full, ten frames out of order, whole round trip incl. reading pixels back | card | 10 | 636 | 602 | 706 | measured by the script |
| Draft, ten frames | card | 30 log rows | 0.8 | - | 29.2 | session log (its slowest 5%) |
| Draft, ten frames, whole round trip incl. reading pixels back | card | 10 | 108 | 80 | 142 | measured by the script |

| playback at Full | memory filled (all 120) | frames held | memory held | three loops: frames asked for (log rows) | frames passed over (dropped) | median ms |
|---|---|---|---|---|---|---|
| processor | 11.55 s | 120 | 996 MB | 274 in 15.04 s (360 frames played) | 87 | 1.7 |
| card | 6.73 s | 120 | 996 MB | 2156 in 15.01 s (360 frames played) | 0 | 1.0 |

## 4. Sound (written by `tools/p28_media.py report`)

The click starts at sample 96000 of sfx_impact.wav = 2.000 s = frame 48 (2000 samples a frame).

| where | first sample at or above 0.1 | means |
|---|---|---|
| window's playback mix, from 0, 120 frames | 96000 | 96000 from the start = frame 48.000 |
| window's playback mix, from 47, 1 frames | None | silent |
| window's playback mix, from 48, 1 frames | 0 | 96000 from the start = frame 48.000 |
| window's playback mix, from 24, 24 frames | None | silent |
| exported WAV | - | no WAV in the PNG export folder |

## 5. Viewer against export (written by `tools/p28_media.py report`)

| export | how it went | files |
|---|---|---|
| format=png&wav=on | dialog filler: box: Search Box id=SearchEditBox; chosen; export started: False | 0 |
| format=mp4&quality=high | dialog filler: box: Search Box id=SearchEditBox; chosen; export started: False | 0 |

0 PNG frames, 0 MP4 frames decoded.

| frame | PNG = viewer | PNG largest difference | MP4 largest difference | MP4 mean difference |
|---|---|---|---|---|
| 0 | - | - | - | - |
| 48 | - | - | - | - |
| 50 | - | - | - | - |
| 90 | - | - | - | - |
| 119 | - | - | - | - |

**Not measured:** nothing was exported, so there is no side-by-side picture.

Contact sheet of the ten frames: `P-28_contact_sheet.png`.

