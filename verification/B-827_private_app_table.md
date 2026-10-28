# B-827 private-app check: sums in number fields (D-947)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**6 of 6 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| 960 typed as 960-400 | 560 (560) | 560 (560) | pass |
| 10 typed as *2 | 20 (20) | 20 (20) | pass |
| 100 typed as 2*(3+4) | 14 (14) | 14 (14) | pass |
| 100 typed as /4 | 25 (25) | 25 (25) | pass |
| layer1 Position x typed as +50 in the app | 50 | 50 | pass |
| one Ctrl+Z | the same | the same | pass |
