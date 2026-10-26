# B-767 timing: dragging a layer's bar, before and after (D-887)

Machine: NVIDIA GeForce RTX 4070 Ti SUPER, 24 logical processors, Windows 11 Education 10.0.26300.
Build: release, `b3c01d7e` (before) and `b3c01d7e` plus B-767 (after), each run as a private copy of
the window with its own WebView2 profile. Other lanes' builds and windows ran at the same time, so a
single number can swing by 10 to 20 ms; before and after were run back to back on the same shot.

How it was measured: the pointer is driven at 60 moves a second for 3 seconds (a bar is pulled 400 px).
- **bar shown** - from a move of the hand to the first screen frame where the bar is drawn at the
  frame under the hand (median / 95th percentile, ms). This is the delay the owner described.
- **viewer** - from a move of the hand to the viewer showing the frame for it (median / 95th, ms).
- **key shown** - the same as bar shown, for a key diamond (this drag was already drawn in the page).
- **scrub** - from a move of the playhead to the viewer showing that frame (median / 95th, ms).
Two runs of each; both are given.

Shots: `ref_L4` (the reference shot, 4 layers) and `ref_L25` (25 layers), each with a feathered mask
and rotation keys added.

## 25 layers

| config | bar shown before | bar shown after | viewer before | viewer after |
|---|---|---|---|---|
| card, Draft | 216 / 438; 680 / 1122 | 14.9 / 15.3; 15.1 / 16.1 | 148 / 168; 146 / 195 | 31.4 / 32.0; 31.6 / 31.9 |
| card, Full | 317 / 841; 165 / 409 | 9.2 / 10.0; 6.2 / 7.1 | 145 / 171; 135 / 168 | 25.6 / 27.5; 22.7 / 23.1 |
| processor, Draft | 465 / 1192; 696 / 1284 | 14.9 / 15.3; 9.6 / 9.9 | 181 / 186; 153 / 188 | 48.0 / 49.3; 42.7 / 43.8 |
| processor, Full | 409 / 1110; (stalled: 2 frames in 3 s) | 14.9 / 15.2; 9.3 / 9.6 | 153 / 186; 503 / 503 | 47.9 / 48.4; 42.4 / 59.0 |

| config | key shown before | key shown after | scrub before | scrub after |
|---|---|---|---|---|
| card, Draft | 14.1 / 14.4; 13.9 / 14.1 | 3.9 / 4.1; 4.5 / 4.7 | 44.0 / 48.9; 36.0 / 36.9 | 48.6 / 48.8; 24.2 / 24.6 |
| card, Full | 6.0 / 6.2; 16.0 / 16.9 | 14.4 / 15.2; 1.5 / 1.8 | 44.9 / 45.9; 24.6 / 41.6 | 36.3 / 36.7; 22.7 / 23.2 |
| processor, Draft | 16.8 / 17.1; 15.0 / 15.2 | 8.8 / 17.3; 4.7 / 5.0 | 45.4 / 61.7; 41.5 / 89.9 | 34.1 / 50.5; 41.7 / 43.4 |
| processor, Full | 14.8 / 15.0; 8.0 / 8.4 | 16.9 / 17.1; 12.7 / 13.0 | 62.5 / 95.4; 46.0 / 62.6 | 43.6 / 45.2; 38.9 / 40.5 |

## 4 layers

| config | bar shown before | bar shown after | viewer before | viewer after |
|---|---|---|---|---|
| card, Draft | 33.5 / 40.2; 31.7 / 36.4 | 16.3 / 16.7; 14.3 / 14.7 | 47.8 / 64.7; 47.1 / 47.8 | 32.6 / 33.1; 30.7 / 31.1 |
| card, Full | 21.9 / 23.9; 23.1 / 26.9 | 11.8 / 12.4; 5.3 / 6.6 | 38.0 / 38.6; 39.4 / 39.8 | 28.2 / 29.0; 21.6 / 22.8 |
| processor, Draft | 23.3 / 28.9; 33.5 / 37.1 | 13.8 / 14.3; 5.7 / 6.2 | 36.8 / 57.5; 47.4 / 64.3 | 30.3 / 30.6; 22.2 / 22.5 |
| processor, Full | 20.9 / 42.3; 22.5 / 51.9 | 2.9 / 4.9; 2.3 / 16.1 | 37.9 / 68.0; 51.1 / 86.8 | 19.3 / 32.2; 18.7 / 32.8 |

| config | key shown before | key shown after | scrub before | scrub after |
|---|---|---|---|---|
| card, Draft | 8.1 / 8.7; 14.1 / 14.8 | 6.4 / 6.7; 16.4 / 16.7 | 35.8 / 110.1; 30.9 / 36.6 | 33.3 / 79.3; 22.8 / 40.2 |
| card, Full | 4.7 / 5.1; 14.5 / 15.4 | 9.6 / 9.9; 14.9 / 15.4 | 25.2 / 40.7; 24.1 / 39.8 | 29.9 / 39.0; 31.1 / 39.0 |
| processor, Draft | 14.8 / 15.1; 12.1 / 12.5 | 7.6 / 8.8; 6.5 / 6.9 | 36.0 / 59.9; 36.6 / 60.0 | 39.0 / 64.2; 41.4 / 60.2 |
| processor, Full | 3.6 / 4.1; 6.1 / 10.3 | 9.1 / 9.3; 8.0 / 8.4 | 36.0 / 66.2; 37.7 / 62.5 | 31.7 / 51.6; 37.9 / 53.9 |

## Reading it

- The bar now follows the hand within one or two screen frames (under 17 ms) on every shot and
  setting. Before, on 25 layers, it fell behind by a fifth of a second to over a second, and the
  viewer by about 150 ms; now the viewer is 23 to 59 ms behind.
- The key and the playhead were not changed by this build; their numbers move within the noise of
  a busy machine (one screen frame is 16.7 ms).
- Checked in the private window: a bar pulled 47 frames is one undo entry, undo puts it back,
  Escape during the pull puts it back with no entry, and after each the bar is drawn at the frame
  the project holds (check `lag/check.py`: 5 of 5 pass).
