# B-847 private-app check: U and UU with no layer selected (D-967)

Written by `checks.py` (lane w12s's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, the four-layer reference shot). Each row drives the window's own code (its key, pointer and mouse handlers) against the app's own engine and reads the project back. Pointer capture is stood in for, because a made-up pointer cannot be captured.

**6 of 6 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| U with no layer selected opens every layer's animated properties (only layer3 has keys, on Rotation) | layer-1: layer-2: layer-3:rotation layer-4: | layer-1: layer-2: layer-3:rotation layer-4: | pass |
| U again, after a pause, closes them | layer-1: layer-2: layer-3: layer-4: | layer-1: layer-2: layer-3: layer-4: | pass |
| UU with no layer selected opens what has changed on every layer (in this shot only layer3's Rotation differs from a new layer) | layer-1: layer-2: layer-3:rotation layer-4: | layer-1: layer-2: layer-3:rotation layer-4: | pass |
| P with no layer selected still opens nothing, as in After Effects | layer-1: layer-2: layer-3: layer-4: | layer-1: layer-2: layer-3: layer-4: | pass |
| U with layer1 selected looks at layer1 only: layer3's keyed Rotation stays closed, as before | layer-1: layer-2: layer-3: layer-4: | layer-1: layer-2: layer-3: layer-4: | pass |
| U with layer3 selected opens its Rotation, as before | layer-1: layer-2: layer-3:rotation layer-4: | layer-1: layer-2: layer-3:rotation layer-4: | pass |
