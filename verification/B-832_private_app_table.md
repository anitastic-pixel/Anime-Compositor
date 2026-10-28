# B-832 private-app check: the viewer's resolution list (D-952)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**4 of 4 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the list | Full, Half, Third, Quarter, Auto | Full, Half, Third, Quarter, Auto | pass |
| Third chosen: quality, lit | third, third | third, third | pass |
| the app draws frame 0 at Third | 640x360 | 640x360 | pass |
| Full chosen again | full, full | full, full | pass |
