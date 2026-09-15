# W-25: blend modes, composition settings, layer copy and paste, and seven more habits

Asked for on 2026-09-15: "proceed".

No photograph: every item is a click or a key press, and the capture script can do neither.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`:

1. **Blend mode.** Select a layer. In the inspector, the Blend line is now a list. Choose screen: the layer brightens what is under it. Right-click the layer's row: the menu has a "Blend:" line for each mode. One Ctrl+Z each, and it is saved.
2. **Ctrl+K.** The composition form opens in the Project panel, filled in with this composition, and its button says Apply. Change the length to 100 and press Apply: the ruler ends at frame 99, and a marker or work area past it is gone or cut back. One Ctrl+Z brings the old length, marker and work area back. A width of 0 is refused with a sentence and nothing changes.
3. **Ctrl+C and Ctrl+V on layers.** With no keys chosen, select two layers, Ctrl+C, Ctrl+V. Two copies appear in front, keys and effects with them. One Ctrl+Z removes both. Choosing keys and pressing Ctrl+C still copies keys, as before.
4. **Reset a property.** Right-click the word Position in the inspector: position goes back to 0, 0. On an animated property it sets a key with the default value on the playhead. One Ctrl+Z.
5. **Centre and fit.** Select a layer and press Ctrl+Home: it sits in the middle of the composition. Ctrl+Alt+Home: the anchor mark jumps to the middle of the drawing and the picture does not move. Ctrl+Alt+F: the layer is stretched to fill the composition. One Ctrl+Z each.
6. **E and UU.** Select a layer with effects and press E: the inspector scrolls to its effects. Press U twice quickly: every property that is animated or changed from its default opens under the layer.
7. **Ctrl+L and Ctrl+Alt+V.** Ctrl+L locks the selected layers (the lock shows on the row), again unlocks. Ctrl+Alt+V hides them, again shows. One Ctrl+Z each.
8. **Ctrl+Shift+K.** Choose one key with keys either side of it and press Ctrl+Shift+K. A small box shows its incoming and outgoing speed and influence. Type an outgoing influence of 80 and press Enter: in the graph (Shift+F3, Speed) the handle after the key reaches further. One Ctrl+Z.
9. **The ` key.** Point at the timeline and press ` (left of 1): the timeline fills the window. Press it again: everything is back where it was. Try it over the viewer too.
10. **Error details.** Open a project with a missing drawing (the reference shot's frame 7 will do). The orange warning shows, and an Error details button appears beside it. Press it: each warning is listed with its code, such as `ASSET_MISSING`, and its detail.

## Playtest request: the W-24 fix and W-25 together

Please check the marker rename fix first: double-click a marker on the ruler, type a name, press Enter, and the name shows beside it. Then run steps 1 to 10 above in one sitting, with the rebuild watcher running if you like. For anything that does not behave as written, tell me the step number and what you saw instead.

## What checks it by machine

- `verification/B-05_model_table.md`: the blend mode and the composition settings apply, a zero width is refused, a shorter length cuts the work area and drops the last marker, and each undoes exactly.
- `verification/B-12b_command_map_table.md`: `composition.set_settings`, `layer.set_blend_mode`, `layer.copy` and `layer.paste` are answered, and Ctrl+K, Ctrl+C, Ctrl+V, Ctrl+L and Ctrl+Alt+V are bound.
- `verification/B-12c_keyboard_table.md`: E is in the key list, the Error details button is a Tab stop, and the right click that resets a property has a palette line too.
- **Not** checked by a test: the look of any of it, where centre and fit put a layer, the velocity numbers, the maximised panel and the paste of mattes. Steps 1 to 10 are the check.
- New capabilities the owner may cut: layer copy and paste, the reset, centre, fit, velocity, maximise and the error details.
