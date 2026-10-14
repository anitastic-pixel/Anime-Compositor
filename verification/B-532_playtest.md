# B-532: Warp Stabilizer

Built on 2026-10-13 as D-653, under your /loop request and your D-600 choice for motion from footage ("Blur + retiming only", Warp Stabilizer "if the estimate holds up"). The movement finder held up, so it was built. It is in the **Distort** group and uses After Effects' settings, apart from the gaps below.

It steadies shaky footage. For each pair of neighbouring frames it works out how the whole picture moved, smooths that movement over the frames around (or holds it still), and moves each frame back by the difference. Then it crops and enlarges the picture so the edges the shake uncovers stay hidden.

There is no Analyze button and no waiting: each frame is worked out from the frames around it when it is drawn, so nothing goes out of date when you change the footage.

- **Result** (Smooth Motion): Smooth Motion keeps the camera's intended move and takes out the shake. No Motion holds every frame to the layer's first, as if the camera were on a tripod.
- **Smoothness** (50): how much shake is taken out. Higher smooths over more frames. 0 changes nothing.
- **Method** (Position, Scale, Rotation): what kind of movement to undo. Position only moves. Position, Scale, Rotation also undoes zoom and turn. Perspective also undoes tilt.
- **Preserve Scale** (off): on, it never undoes zoom.
- **Framing** (Stabilize, Crop, Auto-scale): Stabilize Only leaves the uncovered edges see-through. Stabilize, Crop cuts to the largest middle rectangle that stays covered. Auto-scale enlarges that rectangle to fill the frame.
- **Maximum Scale** (150), **Action-Safe Margin** (0), **Additional Scale** (100): limits on the enlargement, a margin allowed to stay cut, and extra zoom on top.
- **Detailed Analysis** (off): on, finds the movement more finely. Slower.

On an adjustment layer it changes nothing, and the frame's warnings say why.

**The rules are our own.** Adobe does not publish how Warp Stabilizer works, so the fit, the smoothing, the crop and the scale are ours (document 21). It is not proven to match After Effects frame for frame.

The check is `verification/D-653_warp_stabilizer_table.md` (124 of 124). It holds every pixel to numbers worked out beforehand by a separate program.

The pictures are in `verification/D-653 pictures/`: a striped scene filmed by a shaking hand, over a night-blue solid.

## What to check

1. **Before.** `1_frame_3_no_effect.png` and `2_frame_4_no_effect.png`: the scene jumps between the two, and night blue shows at the edges.
2. **As added.** `3_as_added_frame_3.png` and `4_as_added_frame_4.png`: the two frames line up much more closely, slightly enlarged, with no night blue at the edges.
3. **Stabilize Only.** `5_stabilize_only_frame_4.png`: steadied but not enlarged, so night blue shows where the shake uncovered the edges.
4. **No Motion.** `6_no_motion_frame_4.png`: held to the first frame's position.
5. **Draft.** `7_as_added_draft.png`: as added at Draft, a quarter the size, the same look.
6. **In the app.** Add Warp Stabilizer (Distort) to shaky footage and play. The shake is gone and a slow pan stays. Set Smoothness to 0: the layer looks exactly as it did.
7. **Drawings on twos.** On a layer held on twos, the held frames add no movement.
8. **Out of range.** Type 1001 in Smoothness. It is refused with a sentence that gives the range.
9. **Saved and opened again.** Save, close and open the project. The settings and the picture are the same.

## Known limits

- Not built: Subspace Warp (After Effects' starting method), Stabilize, Synthesize Edges, Crop Less Smooth More, Rolling Shutter Ripple, Show Track Points and the track-point controls. A project asking for Subspace Warp keeps the effect but leaves it out of the picture, with a warning.
- The crop is worked out frame by frame, so on hard shake the enlargement can breathe in and out slightly.
- Things moving across the picture pull on the fit. Footage mostly filled by one moving object can steady the object instead of the background.

## Speed

Warp Stabilizer is drawn by the processor only. After audit P1, a frame took about 0.42 seconds at Full quality as added, about 0.26 seconds at Half, and about 0.43 seconds with Detailed Analysis on, playing through every eighth frame on a quiet machine (`verification/B-532_warp_stabilizer_timing_table.md`; before P1 these were about 3.4, 1.2 and 6.0 seconds). Each frame works out the motion between 12 pairs of neighbouring frames at the starting Smoothness (more at higher Smoothness); pairs already worked are kept and reused by nearby frames. It is still far from real time. Switch the viewer to Half while you work.
