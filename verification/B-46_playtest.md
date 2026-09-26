# B-46: Radial Blur on the card, by hand

Built on 2026-09-26 against the B-46 entry in document 15. Not yet tried.

`verification/B-46_gpu_radial.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with a Radial Blur on.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer and pick **Radial Blur** from the Effects panel's list of effects to add. Set Type to **Zoom** and Amount to about 30.
- Click **Draw on GPU**.

## What to check

1. **The same picture.** Click **Draw on CPU** and **Draw on GPU** a few times. The blur should not change in any way you can see.
2. **Spin as well.** Set Type to **Spin**, then Amount to 100. Switch between CPU and GPU again. The picture should not change.
3. **Play at Full.** Switch to **Full resolution** and press space.
   - On GPU it should play close to smoothly.
   - On CPU it plays far slower. The measured medians are 26 ms a frame on GPU and 152 ms on CPU.
4. **Drag the amount.** On GPU, drag the Amount slowly up and down. The blur follows. When you let go, the picture is the one you would get on CPU at that amount.
5. **Move the centre.** Drag the Centre's numbers. The blur follows on GPU as it does on CPU.
6. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU. The measurements say the two are even at Draft.
7. **Export is untouched.** An export made with the switch on GPU is the same file as one made on CPU.

## What to report

- Anything that looks different between CPU and GPU, with the frame number and the blur's settings.
- Any moment where the blur lags behind a change you made, or shows old settings.
- Any message saying the CPU drew the frame, with its exact words.
