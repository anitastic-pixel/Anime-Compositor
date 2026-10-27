# B-779 playtest: the viewer draws at the size it is shown (D-899)

What changed: when you zoom the viewer out, it now draws only as many pixels as your screen
shows. When a Full picture would have more pixels than the screen has room for, the viewer
draws a Half picture instead. When even Half would have more, it draws Draft. The label then
reads **Half (size shown), not final** or **Draft (size shown), not final**. Zoom back in and
the quality you chose comes back on its own.

On this screen (Windows display scale 150%) that starts at about **33% zoom and below**. At
fit, at 50% and above, nothing changes. A new tick, **Full pixels**, in the quality menu
beside Refine when idle, keeps every pixel of the quality you chose at any zoom, as before.
Exports, frame checks and playback are not affected. It happens only when the picture is
drawn on the **CPU**: the graphics card draws a smaller picture no quicker, so a picture the
card draws stays at the quality you chose.

## Steps

1. Open a heavy project. Set the box beside the quality menu to **CPU**. Choose **Full** in
   the quality menu and leave **Full pixels** unticked. Fit the picture.
   - **Expect:** the label reads **Full**, as before.
2. Zoom out to **25%**.
   - **Expect:** within a moment the label reads **Half (size shown), not final**. The
     picture looks the same as Full at that size.
3. Tick **Full pixels**.
   - **Expect:** the label goes back to **Full**. Compare the two: they should look the same at
     25%. If you can see a difference, that is worth reporting.
4. Untick it again and change something at 25%: type a value, click a key, drag a layer.
   - **Expect:** after each change, the picture you end on is the new one at
     **Half (size shown)**, never an older one.
5. At 25%, press **Ctrl+Z**, step a frame, and switch to another composition.
   - **Expect:** each time, the picture shown is the right frame, labelled Half (size shown).
6. Zoom back to **100%**, then to fit.
   - **Expect:** the label reads **Full** again, with no "size shown".
7. Choose **Draft** and zoom to 25%.
   - **Expect:** **Draft**, as before. Size shown never draws finer than the quality you chose.
8. Choose **Full** again, set the box beside the quality menu to **GPU**, and zoom to 25%.
   - **Expect:** **Full**, with no "size shown", as before. Set it back to **CPU**: within a
     moment the label reads **Half (size shown), not final** again.
9. Export a frame.
   - **Expect:** exactly as before, at full size.
10. Close and reopen the app.
    - **Expect:** Full pixels is ticked or unticked as you left it.

## If something looks wrong

Say which step, and whether the picture looked **softer than Full pixels**, was **late**, was
**wrong** (an older picture or the wrong frame) or was **stuck** asking again and again. To
keep the old way for good, tick **Full pixels**.

The byte-for-byte check behind this is `verification/B-779_shown_size_table.md`.
