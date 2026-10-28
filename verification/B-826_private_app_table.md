# B-826 private-app check: parent and track matte on every chosen layer (D-946)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**4 of 4 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Parent layer4 set on layer2 with layer1 also chosen | layer-4, layer-4 | layer-4, layer-4 | pass |
| one Ctrl+Z | the same | the same | pass |
| Track matte layer4 on both, matte only ticked | layer-4 true, layer-4 true | layer-4 true, layer-4 true | pass |
| one Ctrl+Z | the same | the same | pass |
