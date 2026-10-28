# B-829 private-app check: an eyedropper after every colour field (D-949)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**4 of 4 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| colour fields on screen with a dropper right after them | 1 of 1 | 1 of 1 | pass |
| the app answers a pick at 960, 540 with a colour | true | true | pass |
| one Ctrl+Z takes the solid off | the same | the same | pass |
| a solid chosen: its colour fields on screen | true | true | pass |
