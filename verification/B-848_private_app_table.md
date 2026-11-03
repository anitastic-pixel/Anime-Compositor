# B-848 private-app check: a double-click on the shape tool masks the whole layer (D-968)

Written by `checks.py` (lane w12s's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, the four-layer reference shot). Each row drives the window's own code (its key, pointer and mouse handlers) against the app's own engine and reads the project back. Pointer capture is stood in for, because a made-up pointer cannot be captured.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| layer1 drawn at | 1920 x 1080 | 1920 x 1080 | pass |
| masks on layer1 after a double-click with the ellipse in hand | 1 | 1 | pass |
| its outline: the ellipse touching the layer's four sides (x y, in handle, out handle) | 960 0 -530.2 0 530.2 0 | 1920 540 0 -298.2 0 298.2 | 960 1080 530.2 0 -530.2 0 | 0 540 0 298.2 0 -298.2 | 960 0 -530.2 0 530.2 0 | 1920 540 0 -298.2 0 298.2 | 960 1080 530.2 0 -530.2 0 | 0 540 0 298.2 0 -298.2 | pass |
| the tool afterwards | select | select | pass |
| masks on layer4 after a double-click with the selection tool in hand (the click picks the rectangle) | 1 | 1 | pass |
| its outline: the layer's four corners, straight | 0 0 0 0 0 0 | 1920 0 0 0 0 0 | 1920 1080 0 0 0 0 | 0 1080 0 0 0 0 | 0 0 0 0 0 0 | 1920 0 0 0 0 0 | 1920 1080 0 0 0 0 | 0 1080 0 0 0 0 | pass |
| each double-click is one entry to undo | 2 | 2 | pass |
| with no layer selected nothing is masked, and the status line says why | [1,1,0,1] / Select a layer first: the mask covers the whole of it. | [1,1,0,1] / Select a layer first: the mask covers the whole of it. | pass |
| two Ctrl+Z put the project back | the same | the same | pass |
