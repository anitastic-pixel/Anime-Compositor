# W-39: the top bar and the workspaces (D-248)

The first screen of the redesign. The Sandbox's top bar (Top bar A, Search B, the workspace tabs) replaces the old row of buttons. Nothing about the picture, the project file or an export changed: this is where the controls are, not what they do.

## Before, after, and the Sandbox

| | Picture |
|---|---|
| Before W-39 | `W-39 pictures/before_compose.png` |
| After W-39 | `W-39 pictures/after_compose.png` |
| The Sandbox board it copies | `W-38 pictures/sandbox_compose.png` |

What now matches the Sandbox, left to right:

- the film icon, then nine menus: File, Edit, Composition, Layer, Effect, Animation, View, Window, Help;
- the project's name, and a warning chip (⚠ and a count) that shows only when the project has warnings, and opens the error details when clicked;
- the export's progress bar, in a chip that shows only while an export runs;
- three tool icons, Selection (V), Pen (G) and Shape (Q), with the chosen one lit;
- the Search box, which opens the command finder (Ctrl+Shift+P);
- the Effects button;
- the Compose and Animate tabs;
- Save, with an orange dot while there are unsaved changes.

**Not built yet**, each waiting for the step named:

- the ‹ Ep 03 · Cut 012 › cut switcher and the status dot (they need cuts and episodes, which do not exist yet);
- the Hand and Zoom tools (W-41, the viewer);
- the Text tool (its own decision; Ctrl+T stays free transform);
- the Render and Sketch tabs (W-45 and W-46);
- the search box finding layers and effects as well as commands (W-43).

## Click-through: what each part of the bar does

Every menu line is either a control that already existed, moved into the menu as it was, or a line of the command finder run by its name. A line the finder does not have would be a dead button, and a new automatic check now fails if one ever appears.

| Clicked | What runs | What you see |
|---|---|---|
| File > New project (Ctrl+Alt+N) | the same new-project command as before (W-24) | an empty project; with unsaved changes, a warning first and a second press within three seconds |
| File > Open project… (Ctrl+O) | the open command, unchanged | Windows' Open dialog |
| File > Open recent… | the recent list, unchanged | the chosen project opens |
| File > Save / Save As… / Collect Files… / Check Package | the same commands as the old buttons | the same dialogs and messages as before |
| File > Import drawings / Print the Sheet / Session log… | the finder's lines and the old Session log button | the same import dialog, print preview and session log as before |
| Edit > Undo / Redo | the old Undo and Redo buttons | the last change undone or redone |
| Edit > Copy, Paste, Duplicate, Delete, Select all, Select nothing | the finder's lines | the same as their keys |
| Edit > Preferences… | the old Preferences button | the Preferences card |
| Composition > New composition (Ctrl+N) | the finder's line, which presses New composition | the new-composition fields |
| Composition > Composition settings, Pre-compose, Duplicate, Delete | the finder's lines | the same as before |
| Composition > Export… (Ctrl+M), and the format list and ticks under it | the old Export button and its choices, moved as they were | the export starts, its chip shows progress |
| Layer, Effect, Animation, View menus | the finder's lines of those names | the same as the key each line shows |
| Window > Compose workspace (Alt+1) / Animate workspace (Alt+2) | the existing workspaces, renamed from Standard and Timing | the panels move; the tab at the top right lights |
| Window > the workspace list, Save workspace…, Reset workspace | the old workspace controls, moved as they were | as before |
| Help > Find any command, the Search box | the command finder | the finder opens with the cursor in it |
| Help > Error details, the ⚠ chip | the old Error details button | the error codes under the warnings |
| The tool icons | the finder's Selection, Pen and Shape lines | that tool is taken, and its icon lights |
| The Effects button | the finder's "Find an effect or preset" | the effects finder |
| Compose / Animate tabs | the same as Alt+1 / Alt+2 | as above |

## The keys that changed (D-248)

| Key | Before | Now |
|---|---|---|
| Ctrl+N | new project | new composition (as After Effects) |
| Ctrl+Alt+N | nothing | new project (as After Effects) |
| Ctrl+Shift+N | new composition | nothing |
| Alt+1 | nothing | Compose workspace |
| Alt+2 | nothing | Animate workspace |

Document 24 was changed first, in its own commit (852f7c9), before this code.

## Keyboard reach

- **Tab** reaches File first. **Down** opens the menu with its first line chosen; **Up** and **Down** move through it; **Left** and **Right** go to the next menu; **Enter** runs the line; **Escape** closes the menu and goes back to its name.
- `W-39 pictures/file_menu.png` is File opened that way (Tab, Down), `composition_menu.png` is Tab, Down, Right, Right, and `window_menu.png` is Tab, Down, Left, Left. No mouse was used for these three.
- `animate_alt2.png` is Alt+2 pressed with nothing chosen: the Animate workspace, its tab lit. `compose_alt1.png` is Alt+1 afterwards.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` are the same 1280 by 800 window at 100%, 150% and 200%. At 100% and 150% the bar is one row (on a narrow window the Search box gets shorter and its key moves into its hover tip). At 200% it wraps onto a second row rather than cutting anything off.

## Checks

- The page's wiring tests all pass, with their lists updated in the same change:
  - **Controls**: two new ones, the Search box and the warning chip, so 83 now. Both are real buttons the Tab key stops at.
  - **Keys**: "2" is new, so 47 now.
  - **Shortcuts**: document 24's new Ctrl+N and Ctrl+Alt+N rows, and Ctrl+N pressing New composition.
  - **One new row**: "every menu line and top bar button the page runs by name is a line of the command palette". Its result is "all of them are". The table is `verification/B-12c_keyboard_table.md`, 155 of 155 checks.
- `verification/B-12b_command_map_table.md` and `verification/B-12d_new_composition_table.md` were regenerated with the new keys.
- All 80 of the app's tests pass.
- The export tests pass: `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The engine code was not touched; only the page and the test lists changed.
- `tools/capture_window.ps1` gained `-Alt`, the way `-Ctrl` and `-Shift` already worked, so Alt+2 could be photographed.

**One thing to know.** Choosing a workspace with Alt+1, Alt+2 or the tabs lays it out the way it was built or saved, the same as choosing it from the list always has (D-85). Any panel you moved since is put back. Taking these pictures did exactly that to this machine's remembered arrangement. I put it back as it was (Effect controls and Project on the left, Effects on the right, the same sizes), and the pictures were retaken after that.

If you would rather each workspace kept your changes until Reset workspace, as After Effects does, say so and it becomes a small follow-up.

## Playtest

Open the app as you normally do.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Look at the top row | menus at the left; tool icons, Search, Effects, Compose / Animate and Save at the right; one row | |
| 2 | Click File, then move the mouse along to Edit and Composition without clicking | each menu opens in turn | |
| 3 | Click away from the menu | it closes | |
| 4 | File > Save As… | the Save As dialog, as before | |
| 5 | Composition > New composition, then cancel | the new-composition fields, then nothing made | |
| 6 | Press Ctrl+N, then cancel | the same fields | |
| 7 | Change something, then look at Save | an orange dot on Save; it goes after Ctrl+S | |
| 8 | Press Alt+2, then Alt+1 | Animate (the Sheet beside the picture), then Compose; the lit tab follows | |
| 9 | Click the pen icon, then the arrow icon | the pen is taken and lit, then the arrow | |
| 10 | Click Search | the command finder; type "export" and press Enter | |
| 11 | Click Effects | the effects finder | |
| 12 | Press Tab once with nothing chosen, then Down, Right, Escape | File opens, then Edit, then closes | |
| 13 | Composition > Export… on a short range | the export runs and its progress shows beside the project name | |
| 14 | Press Ctrl+Alt+N with unsaved changes | the warning, and a new project only on a second press | |

Anything marked ✗, tell me the row number.
