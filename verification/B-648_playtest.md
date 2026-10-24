# B-648 playtest: Echo's Composite In Back and In Front (D-768)

## What changed for you

Echo's two Composite operators now stack the copies as After Effects does:

- **Composite In Front**: the copy from the current time is in front, whole, and the older echoes
  lie behind it.
- **Composite In Back**: the copy from the current time is at the back, and each older echo lies
  farther forward, so the oldest one is on top.

Before this change the two were the other way round. Nothing else in Echo changes. Saved projects
open and save exactly as before. A project you saved with In Back now looks the way it used to look
with In Front, and the other way round.

## What to look at

Each picture in `verification/B-648 pictures/` puts three images side by side: After Effects'
frame from a tutorial, this build before the change, and this build after it. Ours uses a stand-in
ship, not the tutor's artwork, so compare which copy is on top, not the drawing. The caption over
each of ours gives two numbers: how much of the present copy is left exactly as it draws on its own
(100% means it is whole on top), and how far it is off on average, in levels of 255.

| Picture | After Effects shows | Before | After | Verdict |
| --- | --- | --- | --- | --- |
| `echo_andy_in_front.png` (Andy Ford, 2:26.5) | present ship whole on top | off by 22.2 | off by 14.5 | matches (*) |
| `echo_andy_in_back.png` (Andy Ford, 2:22) | older copies over the present ship | off by 14.5 | off by 22.2 | matches (*) |
| `echo_angie_in_front.png` (Angie Taylor, 4:10.5) | present pose whole on top | 52.7% whole | 100% whole | matches |
| `echo_angie_in_back.png` (Angie Taylor, 4:03) | older poses over the present pose | 100% whole | 52.7% whole | matches |
| `echo_proart_in_front.png` (Pro_Art, 2:45) | present text whole on top | 44.7% whole | 100% whole | matches |
| `echo_motion_in_front.png` (Motion Inspire, 3:44) | present word whole on top | 44.7% whole | 100% whole | matches |

(*) Andy Ford's Starting Intensity is 0.80, so even the present copy is part see-through and the
echoes behind it show through a little in both builds; the order is what changes.

`verification/B-648_echo_stacking_table.md`: 8 of 8 checks pass.

## Try it

1. Put a moving layer (or a precomposition of one) in a composition and add Echo to it: Echo Time
   -0.1, Number of Echoes 5, Decay 1.
2. Set Echo Operator to **Composite In Front**. The layer at the current time is sharp and whole on
   top; the trail is behind it.
3. Set it to **Composite In Back**. The trail now lies over the current layer, the oldest echo on
   top.

Is the layer at the current time in front with In Front, and at the back with In Back?
