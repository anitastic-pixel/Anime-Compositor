# B-47: Bloom on the card, by hand

Built on 2026-09-26 against the B-47 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-47_gpu_bloom.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with a Bloom on.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer and pick **Bloom** from the Effects panel's list of effects to add. Leave its settings at the defaults: Threshold 80, Radius 20, Intensity 1, no streaks.
- Click **Draw on GPU**.

## What to check

1. **The same picture.** Click **Draw on CPU** and **Draw on GPU** a few times. The glow should not change in any way you can see.
2. **Streaks.** Set Streaks to **Star** and Streak Length to about 60. Switch between CPU and GPU again. The picture should not change.
3. **Play at Full.** Switch to **Full resolution** and press space.
   - On GPU it should play close to smoothly.
   - On CPU it plays far slower. The measured medians, with three Blooms, are 27 ms a frame on GPU and 163 ms on CPU.
4. **Drag the settings.** On GPU, drag Threshold, Radius and Streak Length slowly up and down, one at a time. The bloom follows. When you let go, the picture is the one you would get on CPU at those settings.
5. **Turn the angle.** Drag Streak Angle. The streaks turn on GPU as they do on CPU.
6. **Nothing lit.** Set Intensity to 0. The bloom disappears on both, and nothing else in the picture moves.
7. **Draft.** Set Intensity back to 1 and switch to **Draft**. Playing should feel about the same on CPU and GPU. The measurements say the two are even at Draft.
8. **Export is untouched.** An export made with the switch on GPU is the same file as one made on CPU.

## What to report

- Anything that looks different between CPU and GPU, with the frame number and the bloom's settings.
- Any edge of a layer cut off, or a strip missing on its right or bottom, on GPU only.
- Any moment where the bloom lags behind a change you made, or shows old settings.
- Any message saying the CPU drew the frame, with its exact words.
