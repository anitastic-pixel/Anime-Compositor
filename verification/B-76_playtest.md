# B-76: the second batch of ten on the card, by hand

Built on 2026-09-26 against the B-76 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-76_gpu_fx.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with these effects on.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture, for each of the ten.** Select a layer. Add one of the ten from the Effects panel and give it settings that do something. Then switch **Draw on** between **CPU** and **GPU** a few times. The picture should not change in any way you can see. Try this in turn for Distance Gradation, Light Rays, Exposure Flicker, Vignette, Turbulent Displace, Fractal Noise, Gradient Map, Color Balance, Offset and Light Wrap, removing each before adding the next.
2. **Light Wrap on the card.** Until now a frame with a Light Wrap was always drawn by the CPU, and the log said so. Put a Light Wrap on a layer that has something beneath it, with **Draw on** at **GPU**. The log should no longer say the CPU drew the frame, and the edge light should look the same as on CPU. Move the layer across the background: the light should follow what is behind it on GPU too.
3. **Dragging.** With each effect on **GPU**, drag its main setting slowly: Distance Gradation's width, Light Rays' length, Vignette's size and softness, Turbulent Displace's amount, Fractal Noise's size, Gradient Map's midpoint, a Color Balance slider, Offset's shift and Light Wrap's width. The picture should follow your hand with no old settings showing.
4. **Playing the moving ones.** Exposure Flicker, Turbulent Displace and Fractal Noise change over time. Press space on each, on GPU. They should move, and look the same on GPU as on CPU when paused on any one frame.
5. **Offset round the edge.** Drag Offset's shift past the drawing's width: what leaves one side should come back in at the other, on GPU as on CPU.
6. **Edges.** Turbulent Displace with **Edges: Transparent** reaches past the drawing. On GPU nothing at its edges should be cut off or missing a strip on one side.
7. **Play at Full.** Switch to **Full resolution**, put one of the ten on three layers, and press space. On GPU or Auto it should play at least as smoothly as on CPU. The timing table has the measured figures.
8. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU, or better on GPU.
9. **On an adjustment layer.** A frame with an adjustment layer is still drawn by the CPU, as it was before this step, whatever effects it has; the log says so.
10. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the effect, its settings and the frame number.
- Any frame with a Light Wrap that the log still says the CPU drew, with the message's exact words.
- Any edge of a displaced drawing cut off, on GPU only.
- Any moment where the picture lags behind a change you made, or shows old settings.
