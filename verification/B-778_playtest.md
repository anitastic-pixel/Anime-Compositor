# B-778 playtest: an "In motion" picture while you drag or scrub (D-898)

What changed: while you drag something in the viewer, drag a mask point or scrub the playhead
or a value, the viewer now draws a rough picture, an eighth of the size each way. Before this
it drew a Draft one, and only when Full was chosen. The rough picture is called **In motion**.
Masks are drawn at that small size too, and motion blur uses fewer samples. It is used at
**Full, Half and Draft** alike when the preview is drawn on the **CPU**. On the **GPU** the
rough picture is a Draft one instead, labelled **Draft**, because In motion measured slower
there; at Full that is how it was before, and at Half it is new. About a fifth of a second after you stop, the picture is drawn
again at the quality you chose. In motion pictures are never kept and never exported, and the
picture you see after stopping is exactly the one you saw before this change.

## Steps

1. Open a heavy project (many layers, feathered masks or blurs). Set the box beside the
   quality menu to **CPU**. Viewer on **Full**, with **Refine when idle** ticked in the
   quality menu.
2. Drag a layer around for a few seconds.
   - **Expect:** the picture follows your hand more closely than before. It is blocky while you
     move, and the quality label reads **In motion, not final**.
3. Stop moving, but keep holding the mouse button.
   - **Expect:** within about a fifth of a second the sharp Full picture replaces the blocky one,
     and the label goes back to **Full**.
4. Drag one point of a feathered mask.
   - **Expect:** as in steps 2 and 3. While you drag, the mask edge is soft and blocky. Once you
     stop, it is sharp and exactly where you left it.
5. Choose **Half**, then **Draft**, and repeat step 2 each time.
   - **Expect:** In motion while you move. Once you stop, the picture is Half (or Draft) again,
     never Full and never In motion.
6. Untick **Refine when idle** and drag again.
   - **Expect:** the viewer draws at the quality you chose the whole time, as it did before
     B-163.
7. After a drag, press **Ctrl+Z** (undo), step a frame, switch the quality and switch to
   another composition.
   - **Expect:** each time, the picture you end on is the one you chose, never a blocky leftover.
8. Set the box to **GPU** and drag at Full, then at Half, then at Draft.
   - **Expect:** the label reads **Draft** while you move, never In motion, and once you stop
     it reads the quality you chose (Full, Half or Draft) again. At Full this is how it was
     before; at Half the Draft picture while moving is new.
9. Export a frame or a clip.
   - **Expect:** exactly as before. In motion never reaches an export.

## If something looks wrong

Say which step, and whether the picture was **too blocky to work with**, **late**, **wrong**
(a blocky or older picture left on screen after you stopped) or **stuck**. If In motion is too
coarse to judge a drag by, say so: the size is one number, `MOTION_DIVISOR` in
`src/preview.rs`. Setting it to 4 makes it the old Draft size, and unticking Refine when idle
turns it off altogether.

The byte-for-byte check behind this is `verification/B-778_in_motion_table.md`.
