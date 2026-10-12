# B-471: CC Rainfall

Built on 2026-10-12 as D-582 (b), after you said "go with your recommendations on all six":
CC Rainfall, under **Generate** next to Rain, our version of CycoreFX's effect of that name.
It draws rain with depth: drops spread through a space behind the picture, so the far ones are
smaller, slower and fainter than the near ones. Rain (the effect we already had) is not changed.

- **Drops** (5000): how many.
- **Size** (3): how wide a drop at the front is, in pixels.
- **Scene Depth** (5000): how deep the space of drops is; 0 puts every drop at the front.
- **Speed** (4000): how fast a drop at the front falls, in pixels a second.
- **Wind** (0) and **Variation % (Wind)** (0): blow the drops sideways, each its own amount.
- **Spread** (15): how far the drops tilt either way, in degrees.
- **Color** and **Opacity** (25): the drops' colour and strength.
- **Transfer Mode** (Lighten): Lighten never darkens the picture; Composite lays the drops on
  as they are, so dark drops darken it.
- **Composite With Original** (On): Off shows the rain alone.
- **Ground Level** (100): where the drops stop, as a share of the height from the top.
- **Random Seed** (0): a different set of drops.

CycoreFX does not say exactly how it draws, so how deep the space looks, the streak length (what a
drop travels in half a frame) and the settings when added are ours; 5000 drops follows Adobe's
tutorial. Not built yet: Background Reflection, the Refracting look (drops bending the picture
behind them), Offset and Embed Depth.

The check, `verification/D-582_cc_rainfall_table.md` (166 of 166), holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-582 pictures/`, drawn on the street picture at frame 2.

## What to check

1. **Before.** `1_before.png`: the street, no rain.
2. **As added.** `2_as_added.png`: fine, pale, mostly faint streaks all over, the near ones
   longer and brighter. Nothing is darker than before.
3. **Storm.** `3_storm.png`: heavier rain, slanting to the right in the wind.
4. **Near drops.** `4_near_drops.png`: a few big streaks close to the camera, each tilted its
   own way.
5. **Ground level.** `5_ground_level_70.png`: rain over the sky and the upper houses, stopping
   in a line 70% of the way down, so none on the lower houses or the road.
6. **Dark rain.** `6_dark_rain_composite.png`: dark streaks that darken the street (Composite).
7. **Rain alone.** `7_rain_alone.png`: only the streaks, the rest clear (see-through), which a
   picture viewer shows as white or a checkerboard.
8. **In the app.** Add CC Rainfall (Generate) to a layer and play: the rain falls steadily, the
   far drops slower, and it loops without a jump. Change Wind and the rain slants.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how faint the rain is as added (many drops are far away
with Scene Depth 5000), the streak length, and how strongly far drops shrink.

## Speed

Measured on a busy machine, so the numbers are provisional (`verification/B-471_cc_rainfall_timing_table.md`):
on the reference shot the frame times with CC Rainfall were within the ordinary spread between two runs
of the same shot without it, so its own cost could not be measured yet; none of these shots plays in
real time, with or without it. Playing in the app is the real test: if the rain makes playing feel
slower, say so.
