# B-533: Rolling Shutter Repair

Built on 2026-10-13 as D-654, under your /loop request and your D-600 choice for motion from footage ("Blur + retiming only", Rolling Shutter Repair "then try"). It is in the **Distort** group and uses After Effects' settings. With it, the motion-from-footage item (P0-13) is done within your list.

Phone and DSLR cameras read the picture one row at a time, top to bottom, so in a fast pan upright things lean and in a shake they wobble like jelly. Rolling Shutter Repair finds how the picture moves toward the frames just before and after, and moves each row back to where it would have been at the middle moment of the frame.

There is no Analyze button and no waiting: each frame is worked out from the frames either side when it is drawn.

- **Rolling Shutter Rate** (50): how much of a frame the camera takes to read the picture. DSLRs are often 50 to 70, phones near 100. Raise it until leaning uprights stand straight. 0 changes nothing.
- **Scan Direction** (Top → Bottom): the way the camera reads. Most cameras read top to bottom; try the others if a sideways or upside-down camera was used.
- **Method** (Warp): Warp undoes the whole frame's move, which suits a camera pan. Pixel Motion follows each part of the picture separately, which suits things moving on their own. Slower.
- **Detailed Analysis** (off): with Warp, finds the move more finely. Slower.
- **Pixel Motion Detail** (20): with Pixel Motion, how fine the movement found is. Keep it as low as works.

On an adjustment layer it changes nothing, and the frame's warnings say why.

**The rules are our own.** Adobe does not publish how Rolling Shutter Repair works, so the timing of the rows and both methods are ours (document 21). It is not proven to match After Effects frame for frame.

The check is `verification/D-654_rolling_shutter_repair_table.md` (109 of 109). It holds every pixel to numbers worked out beforehand by a separate program.

The pictures are in `verification/D-654 pictures/`: a striped scene filmed by a phone panning right, over a night-blue solid. Its stripes should stand upright.

## What to check

1. **Before.** `1_frame_3_no_effect.png`: the stripes lean, the bottom further right than the top.
2. **As added.** `2_as_added.png`: the lean is smaller.
3. **The camera's own rate.** `3_rate_80.png`: the stripes stand upright. A thin night-blue edge may show at one side, where rows were moved in from outside the picture.
4. **Pixel Motion.** `4_pixel_motion_80.png`: nearly the same as 3.
5. **Wrong direction.** `5_bottom_to_top_80.png`: the lean gets worse, not better. This is what a wrong Scan Direction looks like.
6. **Draft.** `6_as_added_draft.png`: as added at Draft, a quarter the size, the same look.
7. **In the app.** Add Rolling Shutter Repair (Distort) to phone footage with a fast pan and play. Raise the rate until uprights stand straight. Set it to 0: the layer looks exactly as it did.
8. **Drawings on twos.** On a layer held on twos, it still mends each frame, using the motion on the side that moves.
9. **Out of range.** Type 101 in Rolling Shutter Rate. It is refused with a sentence that gives the range.
10. **Saved and opened again.** Save, close and open the project. The settings and the picture are the same.

## Known limits

- After Effects' Warp bends a mesh, so it can mend parts moving differently. Ours moves the whole frame as one, so use Pixel Motion when things move on their own.
- The motion is taken as steady across a frame, so a pan that speeds up within a single frame is only partly mended.
- The starting Rate (50) and Scan Direction (Top → Bottom) are what tutorials report; no Adobe page read confirms them.

## Speed

Rolling Shutter Repair is drawn by the processor only. On a quiet machine after audit P5, a whole frame took about 0.13 seconds at Full quality with Warp (the setting when added; about 0.74 before P5), about 0.08 seconds at Half, and about 0.26 seconds with Pixel Motion (`verification/B-533_rolling_shutter_repair_timing_table.md`). Expect playback to stutter; switch the viewer to Half while you work.
