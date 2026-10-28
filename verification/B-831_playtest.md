# B-831 playtest: New Comp from Selection (D-951)

What changed: right-click a composition or some drawings in the Project panel and choose **New comp from selection**. A new composition opens, the same size and length as what you chose, with it already in as a layer. One Ctrl+Z takes it all away.

## Steps

1. Open a project with a sequence of drawings imported (Ctrl+I).
2. In the Project panel, right-click the drawings. The menu has **New comp from selection** above the label colours. Choose it.
3. A new composition named after the drawings opens. Ctrl+K shows its width and height are the drawings' size, and its length is one frame per drawing.
4. The timeline has one layer, the drawings, filling the frame.
5. Press **Ctrl+Z** once: the new composition and its layer are both gone and the window is back where it was.
6. Right-click a composition in the Project panel and choose **New comp from selection**: the new composition is that one's size, length and frame rate, with it inside as a layer.
7. Composition menu > **New comp from selection** does the same for the drawings you last clicked, or else for the composition on screen.
8. Right-click a sound and choose it: nothing is made, and the status line says to choose a composition or drawings.

## What you should not see

- A Ctrl+Z that takes away the layer but leaves an empty composition.
- A new composition that is 1920 by 1080 and 10 seconds when the drawings are another size.

The computer's checks are in `verification/B-831_new_comp_from_selection_table.md`; the check in a private copy of the app is `verification/B-831_private_app_table.md`.
