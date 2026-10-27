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

## The other viewer drags (2026-10-27, owner: "check the other viewer drags too")

**Same private app copy and project, fixed build.** First, the top layer's anchor is put in its middle (Ctrl+Alt+Home). Then the check drags each handle in turn:

- the rotate handle, 70 screen pixels right;
- the bottom-right scale corner, 40 right and 25 down;
- the anchor mark, 50 right and 30 down, which moves position and anchor together;
- a mask point on layer 2, 70 right and 50 down.

"Wanted" is worked from where the pointer went:

- **Rotate:** the angle swept about the anchor.
- **Scale:** the distance from the anchor, after over before.
- **Anchor and mask point:** the drag in composition pixels.

**Pass rule:** rotate and scale must land within 0.5 (degrees or per cent); the anchor and mask point within 3 screen pixels. "Canvas widths seen" includes 480 wherever the smaller In motion picture appeared during the drag.

**Code reading.** Every viewer press goes through one place, which measures the pointer, so they are all covered by the one fix. That includes the pen and its handles, the text box, the anchor tool, mask points and the box around them, mesh, liquify and the eyedropper. Each gesture's anchor, handles and layer map are taken when the press begins.

| Zoom | Gesture | Canvas widths seen | Wanted | Landed | Verdict |
|---|---|---|---|---|---|
| fit | rotate (degrees) | 1920, 480 | 10.07 | 10.07 | PASS |
| fit | scale (%) | 1920, 480 | 106.21 | 106.21 | PASS |
| fit | anchor (position) | 1920 | 1032.58, 583.55 | 1032.58, 583.55 | PASS |
| fit | mask point | 1920, 480 | 1401.61, 372.58 | 1401.61, 372.58 | PASS |
| 14% | rotate (degrees) | 1920, 480 | 30.11 | 30.11 | PASS |
| 14% | scale (%) | 1920, 480 | 130.57 | 130.57 | PASS |
| 14% | anchor (position) | 1920 | 1317.15, 754.29 | 1317.15, 754.29 | PASS |
| 14% | mask point | 1920, 480 | 1800.02, 657.15 | 1800.02, 657.15 | PASS |

**Two notes:**

- **Run order at fit.** At fit, rotate and scale were measured in separate runs. The test layer fills the frame, so after one of them its handles end up outside the panel, and the second press found no handle and did nothing (no change, no jump). At 14% both ran in one go.
- **The anchor drag.** It never shrinks the picture (D-266: nothing is drawn until you let go), so it was never affected.
