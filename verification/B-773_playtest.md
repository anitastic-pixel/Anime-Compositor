# B-773 playtest: a frame that is no longer wanted stops being drawn (D-893)

What changed: when you click, undo, type a value or start dragging again while the viewer is
still drawing a picture nobody will see, the program now stops drawing it part way and starts on
the picture you actually want. A stopped picture is never shown and never kept. The pictures
themselves are unchanged.

## Steps

1. Open a heavy project (many layers, or effects such as blurs and glows). Viewer on **Full**,
   **Refine when idle** ticked.
2. Drag a layer, pause for about a quarter of a second without letting go, then carry on dragging.
   - **Expect:** the layer picks up again as quickly as before or quicker; the viewer never shows
     a sharp picture of the place you paused at after you have moved on.
3. Let go, then press **Ctrl+Z** (undo) straight away, and **Ctrl+Shift+Z** (redo).
   - **Expect:** the picture shows the undone, then the redone, position. It never flashes the
     other one, and never stays on an older picture.
4. Click quickly through several frames on the ruler, and switch **Draft / Full** quickly.
   - **Expect:** the picture ends on the frame and quality you chose last, never an earlier one.
5. Switch to another composition tab while a heavy frame is drawing.
   - **Expect:** the new composition's picture, never the old one.
6. Scrub the playhead and press play.
   - **Expect:** as before; neither is ever stopped.

## If something looks wrong

Say which step, and whether the picture was **late**, **wrong** (an older picture shown after a
change), **blank**, or **stuck** (no picture until you moved again). The change is switched off
in one place (`STOP_FRAMES` in `app/ui/index.html`, set to `false`), which puts everything back
as it was.

The byte-for-byte check behind this is `verification/B-773_stoppable_frames_table.md`.
