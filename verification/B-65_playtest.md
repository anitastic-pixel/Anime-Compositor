# B-65: the batch of ten on the card, by hand

Built on 2026-09-26 against the B-65 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-65_gpu_fx.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with these effects on.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture, for each of the ten.** Select the background layer. Add one of the ten from the Effects panel and give it settings that do something. Then switch **Draw on** between **CPU** and **GPU** a few times. The picture should not change in any way you can see. Try this in turn for Curves, Levels, Hue/Saturation, Gradient, Drop Shadow, Lens Blur, Rim Light, Outline, Noise and Chromatic Aberration, removing each before adding the next.
2. **Dragging.** With each effect on **GPU**, drag its main setting slowly: a Curves point, Levels' gamma, the hue dial, Drop Shadow's distance, Lens Blur's radius and roundness, Rim Light's width, Outline's width, Noise's amount, Chromatic Aberration's amount. The picture should follow your hand with no old settings showing.
3. **Lens Blur's iris.** On GPU, set Lens Blur's iris to a hexagon, turn up highlight gain and lower the threshold. The bright spots should turn into hexagons, the same as on CPU.
4. **Noise playing.** Set Noise's **Animate** to on and press space. The grain should change every frame, and look the same on GPU as on CPU when paused on any one frame.
5. **Edges of the grown effects.** Drop Shadow, Outline and Lens Blur reach past the drawing. On GPU, nothing at their edges should be cut off or missing a strip on one side.
6. **Play at Full.** Switch to **Full resolution**, put three of one effect on three layers, and press space. On GPU or Auto it should play more smoothly than on CPU. The measured medians are about 26 ms a frame on GPU against about 39 ms on CPU, and 38 against 83 for Noise.
7. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU. The measurements say the two are even at Draft.
8. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the effect, its settings and the frame number.
- Any edge of a shadow, outline or blur cut off, on GPU only.
- Any moment where the picture lags behind a change you made, or shows old settings.
- Any message saying the CPU drew the frame, with its exact words.
