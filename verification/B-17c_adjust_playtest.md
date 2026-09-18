# B-17c: adjustment layers in the window, by hand

Built on 2026-09-17 against D-66, which the owner accepted the same day ("proceed").

The generated half is `verification/B-17c_panel_table.md`, 11 of 11. It calls what the window
calls and checks what comes back: the button needs no drawing chosen, the layer lands above the
chosen one, takes effects, refuses a blend mode, and comes back on Undo. Whether the pixels are
right is B-17b's table, against `Fixtures/adjust`. This sheet covers what neither can judge:
what you see on the timeline and in the panels, and whether the picture changes as it should.

## Before you start

Open any project with at least two layers whose drawings overlap on screen, or use
**Save As...** first and work on the copy. Nothing below changes the fixtures.

## What to check

1. **The button.** Beside **Add layer** is **New adjustment layer**. It is usable as soon as a
   composition is open, with nothing chosen under Drawings. Hovering it says what the layer is.
2. **Adding one.** Choose a layer in the layer list and click **New adjustment layer**. A layer
   named "Adjustment layer" appears just above the chosen one, marked "adjustment" in grey at
   the end of its row, with no blend list on its row. Its bar on the timeline is hatched rather
   than plain, runs the whole composition, and hovering it says its effects apply to everything
   drawn beneath it. The picture does not change: it has no effects yet.
3. **The key.** With nothing chosen, press Ctrl+Alt+Y. Another one appears, at the front. Press
   Ctrl+Z twice: both are gone. Ctrl+Alt+Y is also listed in the command palette and in the
   timeline's right-click menu as "New adjustment layer".
4. **The inspector.** Add one again above a layer and choose it. Under the inspector, Drawing
   reads "none: an adjustment layer" and there is no Blend row. The Exposures panel says "An
   adjustment layer has no drawings to expose." and **Add an exposure** is greyed out.
5. **An effect changes what is beneath.** With the adjustment layer chosen, pick **Exposure**
   from **Add effect...** and set its Stops to 2. Everything drawn beneath the adjustment layer
   brightens; anything above it does not. Click **Forward** and **Back** to move the layer: what
   it brightens follows it, always the layers beneath.
6. **A blur.** Add **Blur** with a radius of 8. The layers beneath blur together as one picture,
   including their overlaps, rather than each on its own.
7. **Opacity halves the change.** Set the adjustment layer's opacity to 50%. The effect is half
   as strong: brighter than without it, not as bright as at 100%. At 0% the picture is as if
   the layer were not there.
8. **Its life.** Trim the adjustment layer's bar so it ends halfway through the composition.
   Scrub past its end: the picture beneath is unchanged there, and changed before it.
9. **Hidden.** Click the eye on the adjustment layer's row: its effects stop. Click again: they
   return.
10. **Saved and reopened.** Save, then **Open** the same file. The adjustment layer is there,
    marked "adjustment", with its effects and its place in the order.
11. **Export.** Export a few frames as PNG. The exported frames show the adjusted picture, the
    same as the viewer.

## Known limits

- An adjustment layer's blend mode is normal only (D-66). A different one is refused with a
  sentence; the window does not offer the list.
- Its shape is set the way any layer's is: the Matte row of the inspector (FX-ADJ-006), and the
  mask in the file (FX-ADJ-004). There is no mask drawing tool yet.
- Draft preview scales its blur with the preview (B-17b); other effects are unchanged by it.

## What to answer

"works", or which step number did something else and what it did.
