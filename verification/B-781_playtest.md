# B-781 playtest: the Profiler, where this frame's time went (D-901)

What changed: a stopwatch button on the viewer's bar (next to Region of interest) opens a small
**Profiler** panel. It shows, for the frame on screen, how long each layer took, whether the
processor or the graphics card drew it, which of its effects took the time, and whether any of its
effects came from the cache. Slowest layer first, marked with an arrow in orange. Closing the panel
switches it off. The pictures themselves are unchanged.

## Steps

1. Open a project with several layers. Put a **Gaussian Blur** with a large blurriness on one
   layer. Viewer on **Full**, preview on the **CPU**.
2. Click the stopwatch button (or find **Profiler** in the command palette).
   - **Expect:** the panel opens at the top right, says the frame number, the total time and
     "drawn on the processor". Every visible layer has a row, by its name. The blurred layer is
     first, marked, with "Gaussian blur" and a time in the Effects column.
3. Click **Measure this frame again**.
   - **Expect:** the blurred layer is now quicker and its From cache column says "1 effect": the
     blur was kept from the first time.
4. Step to another frame, drag a layer, and play, with the panel open.
   - **Expect:** the panel follows each frame drawn. A frame shown from memory says so and has no
     rows (press Measure again to draw it fresh).
5. Switch the preview to the **graphics card** and press **Measure this frame again**.
   - **Expect:** "drawn on the graphics card"; the blurred layer says **graphics card**, its blur
     "(card)" with no time of its own, and a line "Graphics card, the whole frame" under the table.
     Add an **Echo** before the blur: that layer says **card and processor**, the Echo with a time.
6. Close the panel.
   - **Expect:** the button is no longer lit, and dragging feels exactly as before.

## What it does not show

The graphics card's time is for the whole frame, not per layer. Putting the layers together is
also counted for the whole frame. Two effects of the same kind on one layer are added together.

## If something looks wrong

Say which step and what the panel showed (a screenshot is best). The panel is page only: remove
the stopwatch button and nothing else changes.

The check behind this is `verification/B-781_profiler_table.md`.
