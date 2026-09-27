# W-30: presets, favourites, copy and paste, and the FX Console search

Asked for on 2026-09-27, after W-29 passed: "1 through 6 sounds great! maybe for 6, it can work like fx console from video copilot, I thoroughly enjoyed that plugin for AE."

The six were:

1. presets
2. a one-line description of each effect
3. favourites
4. copying and pasting effects between layers
5. dropping an effect onto the viewer
6. a search at the pointer on Ctrl+Space, like FX Console

Nothing about the picture the effects make has changed, only how you find, keep and add them, so there is no fixture table. D-166, where presets and favourites are kept and how a pasted effect is checked, is proposed in document 14.

The five photographs in `verification/W-30 pictures/` were taken in a browser, with the real page and made-up settings:

- `effects_panel_favourites_presets.png`: the Effects panel with a Favourites folder and a Presets folder on top.
- `effect_card_menu.png`: the right-click menu on an effect card's title, with copy, paste and save as preset.
- `preset_name_box.png`: the box that asks for a preset's name.
- `fx_console_empty.png`: Ctrl+Space with nothing typed, showing favourites, recent effects and presets.
- `fx_console_glo.png`: the same with `glo` typed.

## What was added

- **Presets.**
  - **Save as preset…**, under a layer's effects, keeps all of that layer's effects with their settings and keys under a name. So does **Save the effects as a preset…** in an effect card's right-click menu.
  - The name starts as the effects' names joined by ` + `, for example `Glow + Tint`. Enter keeps it and Escape does not. Saving under a name that is already used replaces the old preset.
  - Presets appear in a **Presets** folder in the Effects panel. You add one as you add an effect: double click, Enter, or a drag onto a layer's row, the Effect controls or the viewer. Its effects go at the end of the layer's stack.
  - To remove a preset, press the ✕ beside it twice within three seconds, or use its right-click menu. Undo cannot bring it back.
- **Descriptions.** Hovering an effect in the panel shows one line on what it does, then how to add it. Hovering a preset lists what it holds.
- **Favourites.**
  - The star ☆ beside each effect makes it a favourite ★. Right-click an effect for **Add to Favourites** or **Take out of Favourites**.
  - Favourites show in a **Favourites** folder above Recently used.
  - While you search, Favourites and Recently used are hidden so that nothing is listed twice.
- **Copy and paste effects.**
  - Click an effect card's title and press **Ctrl+C** to copy that effect with its settings. The card's right-click menu also has **Copy this effect** and **Copy all effects**.
  - Select one or more layers and press **Ctrl+V**, or choose **Paste effects** from a layer's right-click menu. Every selected layer that takes effects gets a copy, at the end of its stack.
  - When no effects are copied, Ctrl+V still pastes layers, as before.
- **Drop onto the viewer.** Drag an effect or preset from the panel onto the picture. It is added to the layer under the pointer. Where no layer is under it, it goes to the selected layer. The line at the bottom says `Drop to add it to` and the layer's name while you drag.
- **Ctrl+Space, like FX Console.**
  - A small search box opens where the pointer is. Its grey text says which layer, or how many layers, it adds to.
  - With nothing typed it lists your favourites (★), recent effects and presets (☰). Each line has a small description.
  - Typing narrows the list, with names that start with what you typed first.
  - The arrows move, Enter adds the highlighted one to every selected layer, and Escape, Ctrl+Space again or a click elsewhere closes it.

## Before you start

Open a project with at least two drawn layers, for example the reference shot the release build opens on.

## What to check

1. **Descriptions.** In the Effects panel, rest the pointer on **Cross Glare**. A line says what it does, then how to add it.
2. **Favourites.** Click the ☆ beside **Glow** and beside **Tint**. They turn to ★ and a **Favourites** folder appears on top holding both. Close and reopen the app: they are still there. Click ★ on Tint: it leaves the folder.
3. **Saving a preset.** Select a layer, add **Glow** and **Tint**, and change Glow's strength. Press **Save as preset…** under the effects. A box asks for the name, already filled with `Glow + Tint`. Type `Soft light` and press Enter. The line at the bottom says `Saved the preset "Soft light"` and a **Presets** folder lists it. Rest the pointer on it: it says `Preset: Glow, Tint`.
4. **Applying a preset.** Select a different layer and double-click **Soft light**. Glow and Tint appear on that layer, with the strength you set in step 3. Ctrl+Z takes both away in one step.
5. **Copy and paste.** On the first layer, click the **Glow** card's title and press Ctrl+C. The bottom line says `Glow copied with its settings. Select layers and press Ctrl+V.` Select two other layers together and press Ctrl+V. Each gets a Glow with the same settings.
6. **The card's menu.** Right-click an effect card's title. The menu offers Copy this effect, Copy all effects, Paste effects and Save the effects as a preset…, as in `effect_card_menu.png`.
7. **Drop onto the viewer.** Drag **Mosaic** from the panel onto a layer you can see in the picture that is not selected. While you drag, the viewer has a blue outline and the bottom line names that layer. Let go: Mosaic is added to that layer, not to the selected one.
8. **Ctrl+Space.** Select two layers, put the pointer over the middle of the timeline, and press Ctrl+Space. A box opens at the pointer showing your favourites and Soft light. Type `glo`: Glow and the other glows are listed. Press the down arrow, then Enter. The highlighted effect is added to both layers and the box closes. Press Ctrl+Space again, then Escape: it closes and adds nothing.
9. **Removing a preset.** Press the ✕ beside **Soft light** once: the bottom line asks you to press again. Press it again within three seconds: it is gone.
10. **Nothing is lost.** Everything from W-29 still works: search, Recently used, double click, drag onto a layer's row.

## Known limits, and questions for you

- **Ctrl+Space and keyboard languages.** Windows uses Ctrl+Space to switch the input language on some East Asian keyboard setups. If it does that for you, say which key you would like instead.
- **Presets live in this app on this machine**, as Recently used and saved workspaces do. They are not in the project and cannot be sent to someone yet. A preset file to export and import is the next step if you want that.
- **Keys keep their frames.** A pasted or preset effect with keyframes keeps them at the same frame numbers, not moved to the playhead. After Effects' Paste does the same.
- **Pasted effects go at the end** of the layer's stack, never in the middle.
- **One undo step per layer.** Pasting onto three layers at once is three undo steps, one for each layer, and the same goes for Ctrl+Space onto several layers.
- **A copy is a snapshot.** Changing the effect after Ctrl+C does not change what Ctrl+V pastes.
- **An effect this build does not have**, for example one from a newer version, is never dropped quietly. A paste holding one is refused with a message naming it, and a preset cannot be saved with one.
- **Not from FX Console:** its screenshot tool is not built. Its search, favourites and presets are.
- **Clicking a card's title** is what "holds" it for Ctrl+C. If the Effect controls redraw, for example after an undo, click the title again.
- **The star is for the mouse.** Without a mouse, use the menu key or Shift+F10 on an effect for **Add to Favourites**.

## What checks it by machine

- `verification/B-12b_command_map_table.md`: document 24 now names 96 commands, with `effect.paste` answered by the window and `app.effects_console` by the page.
- `verification/B-12c_keyboard_table.md`: `effect.paste` can be asked for without a mouse, and Save as preset… is a control the Tab key stops at. 144 of 144 checks pass.
- The app's own tests: 66 pass, 3 ignored as before. The new one pastes effects and checks that they arrive with their settings, with new names, as one undo step, and that a paste holding an effect this build does not have, or a setting it does not understand, changes nothing.
- The core tests: every group passes.
- In the browser, on the real page:
  - all 63 effects have a description
  - Ctrl+C on a card and Ctrl+V on a layer send `effect.paste` with that card's settings
  - a preset saved as `Glow + Tint` and applied to two layers sends one `effect.paste` for each
  - `soft` finds the preset, Blur and Diffusion, and `wipe` still finds only the four wipes
  - Ctrl+Space opens at the pointer, and the down arrow then Enter sends `effect.add` for both selected layers
  - a drop onto the viewer sends `effect.add` for the layer under the pointer, and a drag of plain text is ignored
  - removing a preset takes two presses

## What to answer

"works", or which step number did something else and what it did.
