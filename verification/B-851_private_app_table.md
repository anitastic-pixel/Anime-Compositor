# B-851 private-app check: the MP4's estimated size in the Render panel (D-971)

Written by `checks.py` (lane w12s's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, the four-layer reference shot). Each row drives the window's own code (its key, pointer and mouse handlers) against the app's own engine and reads the project back. Pointer capture is stood in for, because a made-up pointer cannot be captured.

**4 of 4 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| MP4, quality preview, 240 frames of 1920 x 1080 at 24 a second, no sound | MP4 (H.264), over black, with sound, up to about 5.9 MB | MP4 (H.264), over black, with sound, up to about 5.9 MB | pass |
| MP4, quality standard, 240 frames of 1920 x 1080 at 24 a second, no sound | MP4 (H.264), over black, with sound, up to about 12 MB | MP4 (H.264), over black, with sound, up to about 12 MB | pass |
| MP4, quality high, 240 frames of 1920 x 1080 at 24 a second, no sound | MP4 (H.264), over black, with sound, up to about 30 MB | MP4 (H.264), over black, with sound, up to about 30 MB | pass |
| PNG: no size is said (it depends on the picture) | PNG, 8-bit, lossless | PNG, 8-bit, lossless | pass |
