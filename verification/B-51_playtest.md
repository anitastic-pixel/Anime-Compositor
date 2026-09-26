# B-51: Glow on the card, by hand

Built on 2026-09-26 against the B-51 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-51_gpu_glow.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with a Glow on.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer and pick **Glow** from the Effects panel's list of effects to add. Set **Glow Based On** to **Bright parts**, **Glow Threshold** to about 60 and **Glow Radius** to about 30.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture.** Switch **Draw on** between **CPU** and **GPU** a few times. The glow should not change in any way you can see.
2. **Glow Threshold.** Drag it slowly from 100 down to 0. More of the picture glows as it falls, the same on GPU as on CPU. At 100 almost nothing glows.
3. **Glow Radius.** Drag it up to 200 and down to 0. The glow spreads and tightens the same on both. At 0 it is a sharp brightening with no haze.
4. **Glow Intensity.** Drag it from 0 to 5. At 0 the glow is gone and nothing else moves.
5. **Chosen colours and tint.** On the red disc's layer, add a Glow with **Glow Based On** set to **Chosen colours**. Pick the disc's red as the colour, set **Glow Colours** to **One colour** (orange, say) and **Glow Operation** to **Screen**. The disc should glow orange, the same on GPU as on CPU.
6. **Play at Full.** Switch to **Full resolution** and press space. On GPU or Auto it should play smoothly. On CPU it plays slower. The measured medians, with three Glows, are 16 ms a frame on GPU and 53 ms on CPU.
7. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU. The measurements say the two are even at Draft.
8. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the frame number and the Glow's settings.
- Any edge of the glow cut off, or a strip missing on one side, on GPU only.
- Any moment where the glow lags behind a change you made, or shows old settings.
- Any message saying the CPU drew the frame, with its exact words.
