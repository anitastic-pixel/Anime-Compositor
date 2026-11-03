# B-866 private-app check: Glow Based On Alpha Channel and the turned sawtooths (D-986)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**5 of 5 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Glow Based On offers | Bright parts, Chosen colours, Alpha Channel | Bright parts, Chosen colours, Alpha Channel | pass |
| Alpha Channel chosen: the app keeps it | alpha | alpha | pass |
| the app draws frame 0 with it | 200 | 200 | pass |
| a word that is not a choice is refused, alpha kept | alpha | alpha | pass |
| two Ctrl+Z put the project back | the same | the same | pass |
