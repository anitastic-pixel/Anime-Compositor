# W-29: the Effects panel

Asked for on 2026-09-27: "the effects is now a very large sliding bar list, let's improve the effects section ui, add a search, have a dedicated effects list section so it doesn't take up space for the effect controls like AE". **Playtest passed on 2026-09-27:** the owner wrote "works".

Nothing about the picture the effects make has changed, only how you find and add them, so there is no fixture table. The two photographs in `verification/W-29 pictures/` were taken in a browser, with the real page and made-up settings:

- `effects_panel.png`: the new panel under the Project panel, with Recently used on top and the folders below.
- `effects_search.png`: the same panel with `wipe` typed into the search.

## What was added

- **An Effects panel**, like After Effects' Effects & Presets without the presets. It starts at the lower left, under the Project panel, with its own border to pull. Like every other panel it can be dragged to another place, or moved from its right-click menu.
- **Folders:**
  - Blur & Sharpen
  - Color Correction
  - Light & Glow
  - Lines & Mattes
  - Distort
  - Generate
  - Stylize
  - Camera & Lens
  - Transition
  - A folder can be folded shut, and each shows how many effects it holds.
  - An effect that no folder names would show under **Other**, so none can go missing. None does today: all 63 are in the folders above.
- **Recently used:** the last five effects you added, on top.
- **A search box** at the top, which stays put while the list scrolls. It keeps the effects whose name, folder or other words match everything typed:
  - `blur` finds every blur.
  - `manga` finds Speed Lines and Halftone.
  - `glow` finds the whole Light & Glow folder.
  - `colour` and `color` are the same.
- **Adding an effect.** The line under the search says which layer it adds to. Any of these adds it:
  - double-click it, or press Enter on it
  - press Enter in the search, which adds the first one found
  - drag it onto a layer's row in the timeline, which adds it to that layer, or onto the Effect controls
- **The inspector's long list is gone.** In its place, an **Add effect…** button brings the Effects panel forward with the search ready to type into.

## Before you start

Open any project with a drawn layer, for example the reference shot the release build opens on. If you had saved your own workspace, choose **Workspace: Standard** once to see the panel where it starts. A saved workspace gets it at the lower left too.

## What to check

1. **Where it is.** The **Effects** panel sits under the Project panel. Drag the border between them up and down: both resize. The Effect controls on the right are as tall as before.
2. **Search.** Select a layer. Click in **Search effects** and type `wipe`. Only the four wipes are left: Linear Wipe, Radial Wipe, Venetian Blinds and Iris Wipe. Press Escape: the whole list comes back.
3. **Adding by keyboard.** Type `twirl` and press Enter. Twirl is added to the selected layer and its card appears in Effect controls. Press the down arrow from the search box: the first effect is highlighted, the arrows move through the list, and Enter adds the one highlighted.
4. **Adding by double-click.** Clear the search and double-click **Rain**. It is added. A single click only highlights it.
5. **Recently used.** Twirl and Rain now head the list under **Recently used**, newest first. Close and reopen the app: they are still there.
6. **Dragging.** Drag **Mosaic** onto a different layer's row in the timeline. That row gets a blue outline as you pass over it, and on letting go Mosaic is added to that layer, not the selected one. Drag another effect onto the Effect controls: it goes to the selected layer.
7. **No layer.** With no layer selected, the line under the search says `Select a layer to add effects to it.`, the names turn grey, and a double click adds nothing. On a sound or null layer it says `This layer takes no effects.`
8. **Add effect….** Select a layer and press **Add effect…** under its effects. The cursor lands in the search box, ready to type.
9. **Undo.** Ctrl+Z after any add takes that one effect away.
10. **Moving the panel.** Drag the **EFFECTS** title onto the right-hand column. It stacks with Effect controls as a second tab. Choose **Reset workspace** to put it back.

## Known limits, on purpose

- There are no presets yet, such as a saved stack of effects with their settings. The panel is where they would go.
- The folders are this project's choice, After Effects' where it has one. Lines & Mattes and Camera & Lens are new, for the anime effects.
- An effect's hover text says how to add it, not what it does. A short description of each is a recommendation below, not built.
- Dragging onto the viewer does nothing. Drag onto the layer's timeline row or the Effect controls.

## What checks it by machine

- `verification/B-12c_keyboard_table.md`: the search box is a new control the Tab key stops at, and `effect.add` can still be sent without a mouse. Rerun on 2026-09-27, all rows pass.
- `verification/B-12b_state_fields_table.md`: unchanged.
- The app's own tests: 65 of 65 pass.
- In the browser, on the real page:
  - the panel lists 63 of the 63 effects
  - `wipe` leaves the four wipes
  - Enter in the search, Enter on an effect and a double click each send `effect.add` for the selected layer
  - a drop onto a layer's row sends it for that layer, and a drop onto the Effect controls for the selected one
  - a row for a layer that does not exist takes no drop

## What to answer

"works", or which step number did something else and what it did.
