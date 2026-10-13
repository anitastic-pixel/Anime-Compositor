# B-531: Timewarp

Built on 2026-10-12 as D-652, under your /loop request and your D-600 choice for motion from footage ("Blur + retiming only"). It is in the **Time** group and uses After Effects' settings.

It plays a layer slower, faster, backwards or at a chosen frame. When a frame falls between two drawings it can repeat the nearer one (Whole Frames), fade the two together (Frame Mix), or move both toward each other along their own motion and blend them (Pixel Motion, the starting choice). Pixel Motion uses the same movement finder as Pixel Motion Blur.

- **Method** (Pixel Motion): Whole Frames, Frame Mix or Pixel Motion, as above.
- **Adjust Time By** (Speed): Speed plays the layer at a per cent of its rate. Source Frame picks the frame to show, counted from the layer's start, and can be keyed.
- **Speed** (50): 50 is half speed, 200 double, negative runs backwards. -1000 to 1000.
- **Tuning**: how the movement is found (Vector Detail, the two smoothness settings, iterations, Block Size, the three colour weights), whether to build from one drawing only, Correct Luminance Changes, Filtering (Normal or the sharper Extreme), and Error Threshold (where the two moved pictures disagree, fade toward Frame Mix instead).
- **Motion Blur** (off): a shutter blur across the retimed motion, with the composition's settings (Automatic) or its own angle and samples (Manual).

It reads the layer with the effects above it in the stack. On an adjustment layer it changes nothing, and the frame's warnings say why.

**The rules are our own.** Adobe does not publish how Timewarp works, so the source time, the in-between and the starting values are ours (document 21). It is not proven to match After Effects frame for frame.

The check is `verification/D-652_timewarp_table.md` (145 of 145). It holds every pixel to numbers worked out beforehand by a separate program.

The pictures are in `verification/D-652 pictures/`: an orange ball bouncing on ones over a night-blue solid. At half speed, frame 17 is a quarter of the way from frame 8's drawing to frame 9's.

## What to check

1. **Before.** `1_frame_8_no_effect.png` and `2_frame_9_no_effect.png`: the two drawings frame 17 is made from.
2. **Whole Frames.** `3_whole_frames.png`: exactly frame 8's drawing.
3. **Frame Mix.** `4_frame_mix.png`: two faint balls, frame 8's stronger.
4. **Pixel Motion.** `5_pixel_motion.png`: one solid ball, a quarter of the way from frame 8's place toward frame 9's.
5. **Draft.** `6_pixel_motion_draft.png`: the same at Draft, a quarter the size.
6. **In the app.** Add Timewarp (Time) to a moving layer and play. It moves at half speed, smoothly, with no doubled images. Set Speed to 100: the layer looks exactly as it did. Set Method to Whole Frames: each drawing shows twice.
7. **Source Frame.** Set Adjust Time By to Source Frame and key it from 0 to 10 over 40 frames: the first 10 frames of the layer play slowly across the 40.
8. **Backwards.** Set Speed to -100 from the layer's start: it holds the first frame, since there is nothing before it.
9. **Out of range.** Type 1001 in Speed. It is refused with a sentence that gives the range.
10. **Saved and opened again.** Save, close and open the project. The settings and the picture are the same.

## Known limits

- Fast motion, or things appearing and disappearing, can tear or smear in Pixel Motion. Error Threshold fades those places toward Frame Mix. Raising it hides tears; 0 turns it off.
- Not built: Matte Layer, Matte Channel, Warp Layer, Show and Source Crops.
- The layer can't run past its own ends: past its last frame, Timewarp holds that frame.

## Speed

Timewarp is drawn by the processor only. With Pixel Motion (the setting when added), a frame whose motion has to be worked out takes about 2 seconds at Full quality and about a quarter of a second at Half, on a busy machine (`verification/B-531_timewarp_timing_table.md`, PROVISIONAL). Whole Frames and Frame Mix are much quicker, about a tenth of a second. Expect playback to stutter with Pixel Motion; switch the viewer to Half while you work.
