# B-770 playtest: frames are made beside the window, not in it (D-890)

What changed: the viewer's pictures (and the outlines, the Sheet grid, the graph's curves) are now
worked out on a helper of their own. The window itself no longer waits while a heavy frame is
drawn, so it keeps answering your hand. Nothing about the pictures themselves changed.

## Steps

1. Open a heavy project (many layers, or effects such as blurs and glows).
2. Set the viewer to **Full** quality.
3. Drag a layer around the viewer slowly, then quickly, for a few seconds.
   - **Expect:** the layer follows the pointer as before or a little closer; the window never
     freezes for a moment while the picture catches up.
4. While still dragging, release, then press **Ctrl+Z** (undo).
   - **Expect:** the layer jumps back to where it was, and the picture shows that at once. It
     never shows the dragged position again after the undo.
5. Drag the playhead (scrub) back and forth across the ruler.
   - **Expect:** the picture follows; when you stop, the frame shown is the frame the playhead is on.
6. Switch **Draft / Full**, and switch to another composition tab.
   - **Expect:** each time, the picture is the new setting / composition, never the old one.
7. Drag a mask point and a number in the Effect Controls (scrub a value).
   - **Expect:** the outline and the picture move together, as before.

## If something looks wrong

Say which step, and whether the picture was **late**, **wrong** (an older picture shown after a
change) or the window **froze**. The change can be switched off in one place
(`FRAMES_OFF_THE_WINDOW_THREAD` in `app/src/main.rs`), which puts everything back as it was.

The byte-for-byte check behind this is `verification/B-770_async_frames_table.md`.
