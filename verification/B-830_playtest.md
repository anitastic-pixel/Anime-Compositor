# B-830 playtest: the Align panel (D-950)

What changed: the Effect controls column has an **Align** section. It lines up the chosen layers by an edge or their middles, against each other or against the composition, and can space three or more evenly.

## Steps

1. Open a project and make three small layers in different places (three text layers, or three small solids).
2. Choose all three (click one, Ctrl+click the others).
3. In the Effect controls column, find **Align**. "Align layers to" says Selection.
4. Press **Align Left**: the three left edges line up with the leftmost one.
5. Press **Ctrl+Z** once: all three go back.
6. Press **Align Middle**: their middles line up across.
7. Switch "Align layers to" to **Composition** and press **Align Centre**: all three move to the middle of the frame, across.
8. Move the three apart unevenly, then press **Distribute Centre**: the outer two stay and the middle one moves to be evenly between them.
9. Choose only two and press a Distribute button: nothing moves, and the status line says three are needed.
10. Choose one layer and press **Align Right** with Selection: it goes to the composition's right edge.
11. Parent one layer to a null and include it: the status line says one parented layer was left out.

## What you should not see

- A Ctrl+Z that only takes back one of the layers.
- A locked layer moving.

The computer's checks are in `verification/B-830_align_panel_table.md`; the check in a private copy of the app is `verification/B-830_private_app_table.md`.
