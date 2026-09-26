# B-50: Gaussian Blur on the card, by hand

Built on 2026-09-26 against the B-50 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-50_gpu_gaussian.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with a Gaussian Blur on.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer and pick **Gaussian Blur** from the Effects panel's list of effects to add. Set **Radius σ** to about 10.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture.** Switch **Draw on** between **CPU** and **GPU** a few times. The blur should not change in any way you can see.
2. **Radius σ.** Drag it slowly up to 50 and down to 0. The blur follows on GPU as on CPU. At 0 the blur is gone and nothing else moves.
3. **Edges.** With a large Radius σ, look at the layer's edges. The blur fades out past them the same way on GPU as on CPU, with no edge cut off.
4. **Play at Full.** Set Radius σ back to 10, switch to **Full resolution** and press space. On GPU or Auto it should play smoothly. On CPU it plays slower. The measured medians, with three Gaussian Blurs, are 28 ms a frame on GPU and 59 ms on CPU.
5. **With the others.** Add a Bloom or a Directional Blur to a second layer. Both effects show on GPU as on CPU.
6. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU. The measurements say the two are even at Draft.
7. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the frame number and Radius σ.
- Any edge of a layer cut off, or a strip missing on one side, on GPU only.
- Any moment where the blur lags behind a change you made, or shows an old Radius σ.
- Any message saying the CPU drew the frame, with its exact words.
