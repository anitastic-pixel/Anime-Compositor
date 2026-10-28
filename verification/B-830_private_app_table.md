# B-830 private-app check: the Align panel (D-950)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**2 of 2 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Left, to the composition: each chosen layer's left edge | 0, 0 | 0, 0 | pass |
| one Ctrl+Z | the same | the same | pass |
