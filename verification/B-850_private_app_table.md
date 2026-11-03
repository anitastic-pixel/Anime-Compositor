# B-850 private-app check: Alt-drag a project item onto a layer replaces its source (D-970)

Written by `checks.py` (lane w12s's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, the four-layer reference shot). Each row drives the window's own code (its key, pointer and mouse handlers) against the app's own engine and reads the project back. Pointer capture is stood in for, because a made-up pointer cannot be captured.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the project row layer2 and the layer row layer1 are on screen | true, true | true, true | pass |
| while Alt is held over layer1 its row is outlined | true | true | pass |
| layer1's source after the Alt-drop | asset-layer2 | asset-layer2 | pass |
| layers in the composition (none added) | 4 | 4 | pass |
| one entry to undo | 1 | 1 | pass |
| one Ctrl+Z puts it back | the same | the same | pass |
| the same drop without Alt adds a layer, as before | 5 | 5 | pass |
| and layer1 keeps its own source | asset-layer1 | asset-layer1 | pass |
| one Ctrl+Z puts it back | the same | the same | pass |
