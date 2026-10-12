# B-530: Pixel Motion Blur

Built on 2026-10-12 as D-650, under your /loop request and your D-600 choice for motion from footage ("Blur + retiming only"). It is in the **Time** group and uses After Effects' settings.

It works out how each part of the layer moved between the frame before and the frame after, then smears the layer along that movement, the way a camera's open shutter would. That gives motion blur to footage, and to drawings that move because they are redrawn. Layer motion blur can't do either, since it only blurs a layer's own movement.

- **Shutter Control** (Automatic): Automatic uses the composition's motion blur angle and samples, even when the composition's switch is off. Manual uses the two settings below.
- **Shutter Angle** (180): how long the shutter stays open. 180 smears half a frame's travel, 360 a whole frame's, and 720 (the most) two frames'. 0 changes nothing.
- **Shutter Samples** (16): how many looks make up the smear. Fewer is faster but can show steps.
- **Vector Detail** (20): how finely the movement is worked out. 100 finds a movement for every pixel, which is slow; 20 finds one every 4 pixels.

It reads the layer with the effects above it in the stack. Before the layer starts or after it ends, there is no frame either side, so the smear goes one way only. On an adjustment layer it changes nothing, and the frame's warnings say why.

**The movement finder is our own.** Adobe does not publish how After Effects finds movement, so ours is new (ADR-022, document 21). It is not proven to match After Effects frame for frame. The starting values (180, 16, 20) and the smear centred on the frame are also our choices.

The check is `verification/D-650_pixel_motion_blur_table.md` (104 of 104). It holds every pixel to numbers worked out beforehand by a separate program.

The pictures are in `verification/D-650 pictures/`: an orange ball bouncing on ones over a night-blue solid, frame 16.

## What to check

1. **Before.** `1_before.png`: one sharp ball.
2. **As added.** `2_as_added.png`: the ball drawn out into a short soft streak along the line it is bouncing on, centred where the sharp ball was.
3. **One frame.** `3_one_frame.png`: Manual, 360 degrees. The streak is about twice as long and fainter.
4. **Two frames.** `4_two_frames.png`: Manual, 720 degrees in 32 samples. Longer and fainter again.
5. **Draft.** `5_as_added_draft.png`: as added at Draft, a quarter the size, the same look.
6. **In the app.** Add Pixel Motion Blur (Time) to a moving layer and play. Moving parts smear along their own paths and still parts stay sharp. Set Shutter Control to Manual and Shutter Angle to 0: the layer looks exactly as it did.
7. **Drawings on twos.** On a layer animated on twos, each drawing smears only toward the drawing it changes from or to.
8. **Out of range.** Type 721 in Shutter Angle. It is refused with a sentence that gives the range.
9. **Saved and opened again.** Save, close and open the project. The settings and the picture are the same.

## Known limits

These are the same failures Adobe lists for its own Pixel Motion Blur:
- A fast shape over a sharp still background smears mostly inward. Its edge goes soft rather than streaking out far.
- Things appearing or disappearing between frames can smear oddly.

The smear is made from the layer's own picture, so motion from beyond the layer's edges is not seen.

## Speed

Measured while other lanes were building, so PROVISIONAL (`verification/B-530_pixel_motion_blur_timing_table.md`).

It runs on the processor only. On the 1920 by 1080 reference shot, three layers, it looked cheap, but only because that shot holds each drawing for many frames, and a repeated frame reuses its smear.

Where the drawings change, working out the movement costs roughly 200 to 400 ms per 1080p layer at the starting settings, and seconds at Vector Detail 100. That is far above the 100 ms target, so playing it in the viewer will be slow the first time through.

A card version is proposed as D-651, for you to accept or refuse.
