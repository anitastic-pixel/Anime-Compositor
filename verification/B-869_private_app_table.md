# B-869 private-app check: A path pasted into Position (D-989)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the mask added is open, three points | false, 3 | false, 3 | pass |
| the new mask's row is on the timeline | true | true | pass |
| pressing its name, then Ctrl+C | Path copied | Path copied | pass |
| pressing layer-1's Position name, then Ctrl+V: a key on each point, where the mask is | 10,10 60,10 60,60 | 10,10 60,10 60,60 | pass |
| the first at the playhead and the last two seconds on | 0, 48 | 0, 48 | pass |
| the middle key roves | false, true, false | false, true, false | pass |
| the app draws a frame along the motion | 200 | 200 | pass |
| one Ctrl+Z takes the keys off | 0 | 0 | pass |
| and one more takes the mask off: the project as it was | the same | the same | pass |
