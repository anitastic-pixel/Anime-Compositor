# B-18c: precompositions in the window, by hand

Built on 2026-09-18 against D-67, which the owner accepted the same day ("proceed").

The generated half is `verification/B-18c_panel_table.md`, 14 of 14. It calls what the window
calls and checks what comes back: Pre-compose refuses half of a parent pair, moves the chosen
layers into Precomp 1 as one entry to undo, the Project panel's button adds a layer of a
composition, a composition cannot be put inside itself, and one a layer shows cannot be deleted.
Whether the pixels are right is B-18b's table, against `Fixtures/precomp`. This sheet covers what
neither can judge: what you see on the timeline and in the panels, and whether the picture stays
the same.

## Before you start

Open any project with at least three layers, or use **Save As...** first and work on the copy.
Nothing below changes the fixtures.

## What to check

1. **Pre-compose.** Choose two layers in the layer list (Ctrl-click the second) and press
   Ctrl+Shift+C. The two rows become one row named "Precomp 1", marked "composition" in grey at
   the end of its row, standing where the front one of the two was. **The picture does not
   change.** The message says Precomp 1 holds the layers and Ctrl+Z puts them back.
2. **The bar.** The new layer's bar on the timeline has "Precomp 1" written on it, runs the whole
   composition, and hovering it says which composition it shows.
3. **The Project panel.** "Precomp 1" is in the list of compositions, the same size as the one
   you were in.
4. **Opening it.** Double-click the Precomp 1 row in the layer list. The window moves into
   Precomp 1 and the two layers are there, in the order they had, with their keys and effects.
   Click the first composition in the Project panel to go back.
5. **An edit inside shows outside.** Open Precomp 1, move one of its layers, and go back. The
   picture in the outer composition has moved with it.
6. **It is a layer.** In the outer composition, choose the Precomp 1 layer. Move it, scale it,
   set its opacity to 50%, give it a blend mode from the list on its row, add **Blur** to it.
   Each works on the two layers together as one picture.
7. **The inspector.** With it chosen, Drawing reads "the composition Precomp 1". The Exposures
   panel says "A composition layer has no drawings to expose." and **Add an exposure** is greyed
   out.
8. **One undo.** Press Ctrl+Z until Pre-compose is undone: in one step the two layers are back
   and Precomp 1 is gone from the Project panel. Ctrl+Shift+Z makes it again, and the window
   stays in the outer composition.
9. **The menu and the palette.** Right-click a layer: **Pre-compose** is listed with
   Ctrl+Shift+C. It is in the command palette under the same name.
10. **The button.** In the Project panel, every composition except the one on screen has a small
    arrow button before Duplicate and Delete; hovering it says "Add ... to this composition as a
    layer". Click it on Precomp 1: a second layer of Precomp 1 appears, centred, above the
    chosen layer or at the front with nothing chosen.
11. **Not inside itself.** Open Precomp 1 and click the arrow button on the outer composition's
    row. It is refused: the message says it would end up inside itself. Nothing changes.
12. **Delete is refused while used.** Click Delete on Precomp 1's row in the Project panel. It
    is refused, and the message names the composition whose layer shows it. Delete that layer
    (both, if you added a second), then Delete on Precomp 1 works.
13. **Tied layers.** Give one layer a parent (the Parent row of the inspector), choose only the
    child, and press Ctrl+Shift+C. It is refused, naming both layers and saying to choose both.
    Choose both and it works.
14. **Its life.** Trim the Precomp 1 layer's bar to end halfway. Past its end the two layers are
    not drawn. Slide the bar later: the inner composition's animation starts later with it.
15. **Saved and reopened.** Save, then **Open** the same file. Precomp 1 and its layer are
    there, marked "composition", and the picture is the same.
16. **Export.** Export a few frames as PNG. They match the viewer.

## Known limits

- The new composition has the outer one's size, rate, start and length, and is named Precomp 1,
  Precomp 2 and so on (D-67). There is no dialog asking for a name; rename it afterwards in
  Composition settings (Ctrl+K).
- The entry in the undo list reads "Add composition Precomp 1 and N more", because Pre-compose
  is several edits taken back together.
- An expression that names a layer which was moved into the precomposition stops finding it and
  says so on its property (B-18b).
- A composition used by two layers is rendered once for each, so heavy precompositions used
  many times preview more slowly (B-18b).
- F2 renames a composition layer; double-click, which renames other layers, opens it instead.

## What to answer

"works", or which step number did something else and what it did.
