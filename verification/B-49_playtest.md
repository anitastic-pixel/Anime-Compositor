# B-49: Directional Blur on the card, by hand

Built on 2026-09-26 against the B-49 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-49_gpu_directional.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with a Directional Blur on.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer and pick **Directional Blur** from the Effects panel's list of effects to add. Set Length to about 20.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture.** Switch **Draw on** between **CPU** and **GPU** a few times. The blur should not change in any way you can see.
2. **Turn it.** Drag Direction slowly all the way round. The smear turns with it, on GPU as on CPU. Stop at a slant, such as 45, and switch between CPU and GPU again.
3. **Length.** Drag Length up to 100 and down to 0. At 0 the blur is gone and nothing else moves.
4. **Play at Full.** Set Length back to 20, switch to **Full resolution** and press space. On GPU or Auto it should play smoothly. On CPU it plays slower. The measured medians, with three Directional Blurs, are 27 ms a frame on GPU and 61 ms on CPU.
5. **With Bloom too.** Add a Bloom to a second layer. Both effects show on GPU as on CPU.
6. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU. The measurements say the two are even at Draft.
7. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the frame number and the blur's settings.
- Any edge of a layer cut off, or a strip missing on one side, on GPU only.
- Any moment where the blur lags behind a change you made, or shows old settings.
- Any message saying the CPU drew the frame, with its exact words.
