# B-868 private-app check: Flip the layer horizontally or vertically (D-988)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the Layer menu lists both lines | true, true | true, true | pass |
| Flip horizontally, two layers selected: layer-1 Scale | -100, 100 | -100, 100 | pass |
| and layer-2 Scale | -100, 100 | -100, 100 | pass |
| the app draws frame 0 with it | 200 | 200 | pass |
| one Ctrl+Z puts both back | the same | the same | pass |
| Flip vertically: layer-1 Scale | 100, -100 | 100, -100 | pass |
| Flip vertically again: as it started | 100, 100 | 100, 100 | pass |
| two Ctrl+Z put the project back | the same | the same | pass |
| nothing selected: the status line says so | Select an unlocked layer first. | Select an unlocked layer first. | pass |
