# B-833 private-app check: F4, Ctrl+Shift+Y, angles in turns, Wiggle position (D-953)

Written by `pac.py` (the lane's scratch folder) against a private background copy of the lane's release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile, a copy of the reference shot). Each row drives the window's own code against the app's own engine and reads the project back.

**8 of 8 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| F4 three times | nomodes, noswitches, both | nomodes, noswitches, both | pass |
| Rotation 390 shown | 1x+30° | 1x+30° | pass |
| Ctrl+Shift+Y on a drawn layer | no dialog | no dialog | pass |
| Ctrl+Shift+Y on the solid: the dialog, filled | open, Red, #ff0000 | open, Red, #ff0000 | pass |
| OK: the solid changed | Sky, 640, 0,0,1 | Sky, 640, 0,0,1 | pass |
| one Ctrl+Z puts the solid back | the same | the same | pass |
| Wiggle position on layer1 | Slider Control 2, Slider Control 30, expression on | Slider Control 2, Slider Control 30, expression on | pass |
| one Ctrl+Z, and the solid's Ctrl+Z before it: the project as it was | the same | the same | pass |
