# B-849 private-app check: Ctrl+T picks the text tool; Space held pans (D-969)

Written by `checks.py` (lane w12s's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, the four-layer reference shot). Each row drives the window's own code (its key, pointer and mouse handlers) against the app's own engine and reads the project back. Pointer capture is stood in for, because a made-up pointer cannot be captured.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Ctrl+T with layer1 (no mask) selected | text | text | pass |
| Ctrl+T with layer2 and its mask chosen: the free transform box, as before | box, select | box, select | pass |
| Space pressed: playing at once, as before | true | true | pass |
| Space held (two repeats): still playing, not started and stopped again | true | true | pass |
| a drag of 100 left and 60 up with Space held moves the view (scroll right, down) | 100, 60 | 100, 60 | pass |
| the drag put the playing back | false | false | pass |
| nothing in the project changed (no layer was moved or chosen) | the same, 0 | the same, 0 | pass |
| Space let go: the hand is put away | false | false | pass |
| a drag without Space does not move the view | 0 | 0 | pass |
| a tap of Space plays | true | true | pass |
| a second tap stops | false | false | pass |
