# B-640 playtest: a new Radio Waves starts at After Effects' times (D-761)

## What changed for you

A Radio Waves you add from now on lives **10 seconds** (Lifespan) and fades out over its last
**5 seconds** (Fade-out Time), as After Effects' panel shows a fresh Radio Waves (Lifespan 10 s in
five tutorials, Fade-out 5 s in six). Ours keeps these two settings in frames, so the numbers you see
depend on the composition: 240 and 120 at 24 fps, 250 and 125 at 25 fps, 300 and 150 at 30 fps.
Before, ours always started at 96 and 48 frames (4 and 2 seconds at 24 fps).

A project you saved before this change looks exactly as it did: its Radio Waves keeps the 96 and 48
(or whatever you set) that the file holds.

No picture of an older project changes, so the proof is the panel's starting values side by side.

## What to look at

`verification/B-640 pictures/1_radio_waves_start_ae_before_after.png`: After Effects' panel
(Himel 356's tutorial, a 30 fps composition) showing Lifespan 10.000 and Fade-out Time 5.000, then
ours before (96 and 48 frames) and after (300 and 150 frames, which is 10 and 5 seconds at 30 fps).

`verification/B-640_radio_waves_start_table.md`: the app check, 12 of 12 pass.

## Try it

1. In a 24 fps composition, add Radio Waves to a layer. Lifespan shows 240 and Fade-out Time 120.
   Play it: each ring now lives 10 seconds instead of 4.
2. Type 96 in Lifespan and 48 in Fade-out Time: both are kept.
3. Open a project you saved before today that has a Radio Waves: it looks the same as before.

## Known differences from After Effects (not fixed here)

- Above 100 frames a second, 10 seconds is more than the setting's top of 1000 frames, so a new
  Radio Waves there starts at 1000 (at 120 fps, 8.3 seconds).
