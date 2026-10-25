# B-709 playtest: Drop Shadow's softness up to 2000 (D-829)

What changed: Drop Shadow's Softness now goes up to 2000, as in After Effects. Before, it stopped
at 500, and a project with a bigger number left that shadow out with a warning. Softness 0 to 500
looks exactly as before.

## What the picture shows

`B-709 pictures/neon_softness_three_way.png`, Smertimba's "soft neon glow with drop shadows"
(https://www.youtube.com/watch?v=gsihkt5sO2w): the word NEON with seven pink Drop Shadows,
softness 10, 50, 100, 200, 400, 1000 and 2000 (the last at 50%), 8 bpc.

| Panel | Red going up from the centre at 200 / 320 / 450 px | How the glow fades (320 and 450 over 200) |
| --- | --- | --- |
| After Effects at 2:16 | 63 / 34 / 15 | 0.54, 0.24 |
| before: the old app (1000 and 2000 refused) | 19 / 1 / 0 | 0.03, 0.00 |
| after: this build | 83 / 44 / 22 | 0.52, 0.26 |

Verdict: **close**. The glow now spreads across the frame and fades the way After Effects' does
(within 0.02). Before, the glow stopped near the letters. Ours is brighter overall because the
tutorial's font (Montserrat Thin) is not installed and Segoe UI Light, about 2.2 times bolder,
stands in.

Speed (this machine, a 960 by 200 title in a 1920 by 1080 frame): softness 2000 takes 106 ms on
the CPU and 139 ms on the graphics card; softness 500 took 31 / 28 ms. See
`B-709_drop_shadow_softness_2000_table.md` (21 of 21 checks pass).

## Try it yourself (3 minutes)

1. Make a text layer, type a word, and add Effect > Perspective > Drop Shadow.
2. Set Distance to 0 and type 2000 into Softness. Expected: it takes 2000; the shadow becomes a
   wide, faint haze over much of the frame.
3. Drag Softness down to 400. Expected: the haze pulls in close to the word.
4. Type 2001. Expected: it is held at 2000.
5. Open any project from before this build. Expected: it looks the same.
