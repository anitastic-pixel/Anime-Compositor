# B-892 playtest: Warp Stabilizer's Crop Less <-> Smooth More (D-1012)

What it is: in the Warp Stabilizer tutorial (5:00) the slider Crop Less <-> Smooth More is
lowered so less of the shot is cut away and enlarged, at the price of a less steady picture.
Now Warp Stabilizer has it, under Additional Scale. It is 50 when added; 100 is the old,
fully steadied picture.

## Steps

1. Open a project with shaky footage and select its layer.
2. Effect > Warp Stabilizer. Set Framing to **Stabilize, Crop**.
3. Play a few frames at Crop Less <-> Smooth More 50 (as added).
4. Type 100 and play again.
5. Type 0 and play again.
6. Set Framing to **Stabilize Only** and move the setting between 0 and 100.
7. Save, close and open the project.

## What you should see

- Step 3: steadier than the footage, with a thin cut-away edge.
- Step 4: as steady as before this change, with a wider cut-away edge.
- Step 5: the footage as filmed: shaking, nothing cut away.
- Step 6: nothing changes; the setting only matters with a crop framing.
- Step 7: the value you left is still there.

## Pictures

In `verification/D-1012 pictures/`, frame 4 of a shaken striped card over night blue, Stabilize,
Crop: `1_smooth_more_100_frame_4.png` (4048 night-blue pixels cut away),
`2_as_added_50_frame_4.png` (2032) and `3_crop_less_0_frame_4.png` (none). Frames 3 and 4 differ
by 1.9, 9.7 and 17.6 levels: lower is less steady.

## Fixtures

- `verification/D-1012_warp_stabilizer_2_table.md`: 43 of 43. FX-WS2-001 to 010 within 2e-5,
  files that save as they were read, refused values, keys, and the pictures above.
- `verification/D-653_warp_stabilizer_table.md`: still 126 of 126.
