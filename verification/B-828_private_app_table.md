# B-828 private-app check: all sixteen label colours (D-948)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**4 of 4 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| label colours offered, none first | 17, none, red, ..., dark green | 17, none, red, ..., dark green | pass |
| layer1 labelled 16 by the app | 16 | 16 | pass |
| layer1 labelled 9 by the app | 9 | 9 | pass |
| two Ctrl+Z | the same | the same | pass |
