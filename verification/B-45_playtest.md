# B-45: the card painting the viewer, by hand

Built on 2026-09-26 against the B-45 entry in document 15. Never performed.

`verification/B-45_native_viewer.md` explains what was built and has the comparison pictures and timings. This sheet covers what photographs cannot: how the viewer feels to use.

## Before you start

Use the release build. It opens on the reference shot. Click **Draw on GPU**.

## What to check

1. **The same picture.** Click **Draw on CPU** and **Draw on GPU** a few times. The picture should not move or change that you can see. The panel around it should stay the same too.
2. **Zoom and scroll.** Zoom the viewer in a long way and drag its scrollbars around. The picture follows at once, with no gap, no smear and no delay. It never draws over the scrollbars, the panel edges or the other panels.
3. **Things on top.** With the picture showing, open the command search (Ctrl+Shift+P) and the **Recent…** list. Each one draws on top of the picture.
4. **Fill the window with a panel.** Point at the Timeline and press ` (the key left of 1). The Timeline fills the window, and no part of the picture shows through. Press ` again, and the picture comes back.
5. **Resize.** Drag the window's edge smaller and larger, and maximise and restore it. The picture stays inside the viewer throughout.
6. **Minimise.** Minimise the window and bring it back. The picture is there and is the current frame.
7. **Play at Full.** Switch to **Full resolution** and press space. It should play smoothly, noticeably more so than on CPU. With **Session log…** on, the stages include "GPU: paint the picture into the window", and it should skip few or no frames.
8. **Alpha and the checkerboard.** Click **Alpha only**, then click it again. Then solo a layer so that the picture has see-through parts, and turn the checkerboard on and off. Each looks as it does on CPU.
9. **Text.** Look closely at the menu text while on GPU, then on CPU. On GPU the edges of letters are smoothed in grey rather than with a hint of colour. This is expected (see Limits in the write-up). Report it only if it bothers you.
10. **Export is untouched.** As in B-44's step 9: an export made with the switch on GPU is the same file as one made on CPU.

## What to report

Report:

- anything that looks different between CPU and GPU, and the frame number
- any moment where the picture sits in the wrong place, lags behind a scroll, or shows through something drawn on top of it
- any message saying the card stopped painting the viewer, with its exact words
