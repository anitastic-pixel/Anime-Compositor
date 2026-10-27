# B-798: a layer dragged in the viewer lands where the pointer went

Reported by the owner on 2026-10-27: "when I drag a layer down, it spontaneously jumps out of the frame, seems to go the opposite direction."

**Cause.** During a drag the viewer swaps the picture for a smaller In motion stand-in (D-898, an eighth or a quarter of the size) and, below 100% zoom, the size shown (D-899). The drag measured where it started in the first picture's pixels and where the pointer was in the new, smaller picture's pixels. So the pointer seemed to jump back toward the top left, and the layer flew up and left, out of the frame.

**Fix.** For as long as the button is held, every point on the picture is measured in the pixels of the picture that was on screen when the press began. This covers moving, scaling, turning, mask points, anchor, mesh, liquify and the text box. The drawing of boxes and handles is unchanged.

**Check.** A private copy of the app opens a four-layer 1920x1080 project at Full. The check presses on the top layer and drags it 60 screen pixels right and 90 down over 1.2 seconds, then lets go. "Wanted" is the starting position plus that drag in composition pixels. Pass: the layer lands within 3 screen pixels of it. "Canvas widths seen" are the picture sizes on screen during the drag; 480 is the In motion stand-in.

| Build | Zoom | Canvas widths seen | Wanted | Landed | Verdict |
|---|---|---|---|---|---|
| before (092f12c8) | fit | 1920, 480 | 87.1, 130.6 | -2792.9, -1489.4 | FAIL |
| before (092f12c8) | 14% | 1920, 480 | 428.6, 642.9 | -2451.4, -977.1 | FAIL |
| after | fit | 1920, 480 | 87.1, 130.6 | 87.1, 130.6 | PASS |
| after | 14% | 1920, 480 | 428.6, 642.9 | 428.6, 642.9 | PASS |
| after | 50% | 1920, 480 | 120, 180 | 120, 180 | PASS |

Each drag is one step of history (undo count 1 after it), as before.

**Playtest.** Open any project, zoom out to 25% or less, and drag a layer down in the viewer. It follows the pointer and stays where you let go, at every quality.
