# B-825 private-app check: three shortcut fixes (D-945)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**6 of 6 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Ctrl+M, then Ctrl+Alt+M | Add to Render queue, Export | Add to Render queue, Export | pass |
| the F9 family: Ctrl+Shift, Shift, none | out, in, both | out, in, both | pass |
| Ctrl+click on layer3 Rotation key at frame 40 (linear) | auto | auto | pass |
| Ctrl+click again: both sides of it straight | bezier, linear, linear | bezier, linear, linear | pass |
| each Ctrl+click one entry to undo | 2 | 2 | pass |
| two Ctrl+Z put the project back | the same | the same | pass |
