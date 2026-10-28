# B-831 private-app check: New Comp from Selection (D-951)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**2 of 2 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| compositions, and the new one's size, length and layer | 2, 1920x1080, 240, 1 layer | 2, 1920x1080, 240, 1 layer | pass |
| one Ctrl+Z | the same | the same | pass |
