# B-44: the GPU preview, by hand

Built on 2026-09-25 against the B-44 entry in document 15. Never performed.

The generated halves are `verification/B-44_gpu_preview_table.md` (105 of 105) and `verification/B-44_gpu_window_table.md` (9 of 9). This sheet covers what they cannot: the picture as you see it while scrubbing and playing, and the button's words. `verification/B-44_gpu_preview.md` explains what was built.

## Before you start

Use the release build. It opens on the reference shot.

## What to check

1. **Starts on CPU.** The label under the viewer ends with **drawn on CPU**. The button beside Draft / Full resolution reads **Draw on GPU**.
2. **Switch it on.** Click **Draw on GPU**. The status line names your graphics card and says exports are still drawn on the CPU. The label now ends with **drawn on GPU**, and the button reads **Draw on CPU**. Hold the mouse over the button: its tooltip names the card.
3. **The same picture.** Step through a few frames with the arrow keys, clicking the button between CPU and GPU on each. The picture should not change at all that you can see.
4. **Scrub and play at Draft.** Drag the playhead across the timeline, then press space to play. The picture keeps up and looks right the whole way.
5. **Scrub and play at Full.** Switch to **Full resolution** and do the same. Expect it to be about as smooth as on the CPU, not smoother; `B-44_gpu_preview.md` says why.
6. **The command search.** Press Ctrl+Shift+P, type "graphics" and choose **Preview on the CPU or the graphics card**. It switches back and forth like the button.
7. **A frame the card does not draw.** With the switch on GPU, add an adjustment layer (**New adjustment layer**, Ctrl+Alt+Y) and give it any effect, for example Gaussian Blur. The label ends with **drawn on CPU, not GPU**, and the button's tooltip says the frame has something the card does not draw yet. Undo with Ctrl+Z until the adjustment layer is gone. The label goes back to **drawn on GPU**.
8. **The session log.** Open **Session log…**, tick the box, and play a few seconds on GPU. The stages include "GPU: send drawings to the card" and "GPU: draw, encode and bring the picture back". Repeat step 7 with the log on. Those frames carry the warning `GPU_PREVIEW_ON_CPU`, with the reason.
9. **Export is untouched.** With the switch on GPU, export a few frames as PNG. Switch to CPU and export the same frames again. The two sets of files are the same. You can compare them in any image viewer, or ask me to check that they match byte for byte.
10. **Off on reopen.** Close the app and open it again. The label says **drawn on CPU**.

## What to report

Anything that looks different between CPU and GPU, however small, and the frame number. Also report anything that reads wrong, and any step where the picture stutters on GPU but not on CPU.
