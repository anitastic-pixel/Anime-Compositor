# W-38: the Sandbox as the redesign's target, and its wiring table

2026-10-02. **PROPOSED**: D-248 waits for the owner. Nothing in `app/` or `src/` has changed.

This is Step 0 of the plan the owner agreed on 2026-10-02 ("agreed"). It has four parts:

1. A decision that the Sandbox replaces the W-32 to W-37 drafts.
2. The Sandbox's pictures, saved into the project.
3. A table of every control on it: what it does in today's app and how much work it needs.
4. The automatic check that no button is wired to nothing.

## The pictures

Each board is photographed at 1920 by 1100 in `verification/W-38 pictures/`. The pink numbers match the key panel on the right of each picture.

| Board | Picture |
|---|---|
| Compose: the whole window | `sandbox_compose.png` |
| Compose with Ctrl+Space's effects finder open | `sandbox_compose_finder.png` |
| The map of compositions (double-click Precomp 1) | `sandbox_map.png` |
| Animate: graph under the timeline, the sheet beside it | `sandbox_animate.png` |
| Sketch | `sandbox_sketch.png` |
| Render: the queue and "watch it write" | `sandbox_render.png` |

The live, clickable page is https://claude.ai/artifact/2YhV55os2pp7rtiJFLuyFh (version 25). Its source and still copies are kept in `design/W-38_sandbox/`, so the target survives if the canvas changes.

## The automatic wiring check already exists

The check Step 0 asked for was built in B-12b and B-12c and is still running. Every time the tests run, four checks read the window's own page:

- **`the_page_asks_for_nothing_the_window_cannot_answer`** checks that:
  - every command the page can send (87 of them) is one the engine answers with a sentence;
  - every other request (the 26 non-command routes such as save, open and export) is one the window has;
  - each listed button sends the command written beside it.

  A button wired to a misspelt command fails it. Its table is `verification/B-12b_page_table.md`.
- **`every_command_document_24_names_is_accounted_for`** checks that every command in document 24 is reached by something in the window, or is written down as why not. Its table is `verification/B-12b_command_map_table.md`.
- **`every_command_the_page_sends_can_be_asked_for_without_a_mouse`** checks the window's 81 controls and its 46 shortcut keys. Its table is `verification/B-12c_keyboard_table.md`.
- **`every_field_the_panels_read_is_in_the_answer_the_window_gives`** checks that every number a panel shows is one the engine sends. Its table is `verification/B-12b_state_fields_table.md`.

**Run on 2026-10-02: all four pass.**

For the redesign this means each screen is rebuilt under the same checks. A redesigned button that loses its command fails a test. So does a renamed control the tests still expect, or a new command nobody wrote down. Each screen re-pins those lists in the same change as the screen, so the owner sees a list grow or shrink rather than silently change.

## The wiring table

Every control on the Sandbox, against today's window. The Status column says how much work each one needs:

- **Moves**: the window already does this. The redesign only moves or restyles the control, and the command it sends stays the same.
- **Look**: new look only. The page does it with commands that already exist, or with no command at all, so nothing in the engine changes.
- **Engine**: needs new engine work. It gets its own decision, with fixtures before code, when its screen comes up.
- **Excluded**: document 23 excludes it, so only the owner can let it in (question 2 below).

### Top bar (screen 1)

| Sandbox control | Today's window | Command it sends | Status |
|---|---|---|---|
| File menu: New project, Open, Save, Import, Relink | Buttons Open and Save in the top strip; Import and Relink in the Project panel | `new`, `open`, `save`, `media.import`, `media.relink` | Moves |
| Edit menu: Undo, Redo, Copy, Paste, Preferences | Undo and Redo buttons; Copy and Paste on the keyboard and the layer menu; the Preferences button | `edit.undo`, `edit.redo`, `layer.copy`, `layer.paste` | Moves |
| Composition menu: New composition, Settings (Ctrl+K), Pre-compose (Ctrl+Shift+C) | The New composition button, Ctrl+K, Ctrl+Shift+C | `composition.create`, `composition.set_settings`, `layer.precompose` | Moves (Ctrl+N: question 3) |
| Composition menu: Map of compositions | Not today. The information is already sent to the page (which composition holds which) | `composition.open` on double-click | Look |
| Composition menu: Add to Render queue (Ctrl+M) | Ctrl+M exports straight away; there is no queue | `export` | Engine (the queue) |
| Layer menu: New solid, Posterize Time, Parent to, Timing tag | Ctrl+Y, the effect, the parent whip; the timing tag is the "held / on 2s / irregular" badge | `layer.add_solid`, `effect.add`, `layer.set_parent` | Moves |
| Effect menu: Find an effect (Ctrl+Space), Effect presets | Ctrl+Space's finder and the Presets folder | `effect.add`, the preset routes | Moves |
| Effect menu: Remove all effects | Not today; each effect has its own ✕ | `effect.delete` once per effect | Look. As one undo step it may need engine work; checked when built |
| Animation menu: Easy ease (F9) | F9 | `keyframe.set_interp` | Moves |
| Window menu: Find any command (Ctrl+Shift+P) | The command palette | Whatever is chosen | Moves |
| Help menu: Keyboard shortcuts, About | The shortcut row along the bottom, which W-37's list and editor replace (D-167) | none | Moves / Look |
| Tool icons: Selection V, Pen G, Shape Q | The keys only; no icons today | `mask.add`, `shape.add` | Moves |
| Tool icons: Hand H, Zoom Z | The middle mouse button and the `,` `.` keys; no tools | none | Look |
| Tool icon: Text (Ctrl+T) | **There are no text layers.** Ctrl+T today is the free-transform box (D-81) | none | Engine: a whole new kind of layer (question 3) |
| Search box (commands, menus, layers, help) | The palette searches commands only | none | Look |
| Effects button | Ctrl+Space's finder | `effect.add` | Moves |
| Health chip "1 drawing missing" | The red "missing" marks and each layer's Relink button | `media.relink` | Look for missing drawing numbers. Engine for files deleted from disk, which no request reports today |
| Export chip and progress | The export bar and its sentence | `export`, `cancel-export` | Moves |
| Save state "Not saved yet · recovery copy at 14:32" | "• unsaved" and "Recovery snapshot written to …" | none | Moves. **One gap:** a project never saved gets no recovery copy today. Engine, if wanted |
| Workspace tabs Compose / Animate / Render | The Workspace list: Standard, Timing and saved ones (D-85) | none (kept in the window, not the project) | Look |
| Workspace tab Sketch | none | none | Excluded (question 2) |
| Cut status chip "In progress" | Not today; a composition stores episode, scene, cut and animator, but no status | none | Engine (a new thing saved in the project) |
| Previous cut / Next cut | The composition list | `composition.open` | Look |

### Timeline (screen 2)

| Sandbox control | Today's window | Command it sends | Status |
|---|---|---|---|
| A tab for each open composition, with × | The list in the Project panel | `composition.open` | Look |
| Timeline / Both / Graph | The Timeline and Graph tabs (Shift+F3 swaps them) | none | Look (Both is new) |
| Frame number box | The frame box | none (seeks) | Moves |
| Note button, notes on the ruler | **Markers already are this:** `*` adds one at the playhead; double-click a marker to name it; drag it; Ctrl-click removes it | `timeline.set_markers` | Moves |
| Notes shown in the sheet's Action column | The Action column is a separate store, typed into the sheet | `sheet.write_action` | Look (question 4) |
| Sheet button, the sheet as a hideable column | The Sheet panel, placed by the workspace | none | Look |
| Ruler drag, work area | Both exist | `timeline.set_work_start`, `timeline.set_work_end` | Moves |
| **Green ready-frames line, "21 of 36 ready"** | Not today. The memory holds the frames (B-154, D-223), but no request tells the page which ones | none | Engine: one small new request, no new picture |
| Layer row: eye, solo, lock, shy, label colour, name, mode, parent, timing badge | All exist | `layer.toggle_visibility`, `layer.toggle_solo`, `layer.toggle_lock`, `layer.toggle_shy`, `layer.set_label`, `layer.set_blend_mode`, `layer.set_parent`, `layer.rename` | Moves |
| Key diamonds, previous and next key | Exist; J and K jump between keys | `keyframe.add_remove`, `keyframe.move` | Moves |
| Graph: property ticks, ease buttons, keys dragged | The property list, F9, Shift+F9, Ctrl+F9, Ctrl+Alt+G, dragging | `keyframe.move`, `keyframe.set_interp` | Moves |
| Sheet column, click a row to go there | The Sheet | `exposure.write`, `sheet.write_action` | Moves |

### Viewer (screen 3)

| Sandbox control | Today's window | Command it sends | Status |
|---|---|---|---|
| Back, Play, Forward (Page Up, Space, Page Down) | Exist | the frame and play routes | Moves |
| How fast it is really playing | Only the sentence after playback stops | none | Look |
| Picture size, Fit | Fit, 100%, the zoom field, `,` `.` | none | Moves |
| Picture quality | Full and Draft (D) only | `frame?q=` | Moves. Engine only if a level between them is wanted |
| Drawn on graphics card / processor | The GPU switch | `gpu-switch` | Moves |
| Checkerboard | The button today labelled "grid" | `viewer.toggle_checkerboard` | Moves |
| Grid, field guide and safe areas, cut slate caption | Not today. The slate's text is already stored (Ctrl+K) | none | Look |
| Snapshot and compare | Not today | none | Look |
| Turn the view 15°, mirror, reset | Not today | none | Look |
| Region box | Not today | none | Engine (drawing only part of the picture) |
| Onion skin | Not today | none | Look for the whole picture. Engine for one layer's drawings alone |
| Sheet strip under the picture | Not today; the Sheet panel exists | none | Look |
| Map tab (flowchart / tree) | Not today; double-clicking a precomp opens it | `composition.open` | Look |
| Sketches over the viewer | none | none | Excluded (question 2) |

### Effect controls, Project, finder (screens 4 and 5)

| Sandbox control | Today's window | Command it sends | Status |
|---|---|---|---|
| Twirl, the on/off switch, stopwatch, dragging a blue number | All exist | `effect.toggle_bypass`, `keyframe.add_remove`, `property.drag_update`, `property.drag_end`, `property.drag_cancel` | Moves |
| ◀ ◆ ▶ beside a setting | The ◆ exists; J and K jump across the whole layer | `keyframe.add_remove` | Look |
| Project views (List / Pictures / Tree / By cut) | One list | none | Look |
| Label colours on compositions and footage | Layers have labels; compositions and footage do not | none | Engine (saved in the project) |
| Information line at the top of the Project panel | The viewer's tab shows the same | none | Moves |
| Finder: typing, favourites, recent, presets, "Add to Hair · Enter" | All exist in Ctrl+Space's finder | `effect.add`, `effect.paste` | Moves |
| Finder: the family tree on its left | Exists as the Effects panel's folders, not inside the finder | `effect.add` | Look |

### Animate (screen 6) and Render (screen 7)

| Sandbox control | Today's window | Command it sends | Status |
|---|---|---|---|
| Animate's layout: graph under the rows, sheet beside them | The Timing workspace is the nearest; Both is new | none | Look |
| Render queue: items, each on or off, format | No queue; one export at a time; the format list exists | `export` | Engine (the queue). The format list moves |
| Missing drawing: Stop or Write anyway | Exists | `export` with `missing=write` | Moves |
| Go and Stop | Export and Cancel | `export`, `cancel-export` | Moves |
| Watch it write: the frame being written, Pause, as a popup or its own window | Only a bar and a sentence; there is no Pause | none | Engine |

### Sketch (screen 8)

| Sandbox control | Today's window | Status |
|---|---|---|
| Brush, pencil, eraser, sizes, colours, undo a stroke; sketch layers; onion; clear; turning the paper; sketches over the viewer | none | Excluded (question 2) |

### Count

| Status | Rows |
|---|---|
| Moves | 29 |
| Look | 22 |
| Engine | 8 |
| Excluded | 3 |

Counted by each row's first word. Some Moves and Look rows also have an engine part, so the engine work comes to eleven things:

- the render queue;
- watch it write with Pause;
- the ready-frames request;
- a text tool;
- a cut status;
- label colours for compositions and footage;
- the region box;
- recovery copies for never-saved projects;
- files deleted from disk in the health chip;
- one-layer onion skin;
- a picture quality between Full and Draft.

Each gets its own proposed decision when its screen comes up, with fixtures first. None of them changes a picture an export writes.

## Shortcuts the Sandbox uses that already mean something else

| Key | Today (document 24 and the page) | The Sandbox | Recommended |
|---|---|---|---|
| Ctrl+N | New project | New composition | Follow the Sandbox, as After Effects does: Ctrl+N new composition, Ctrl+Alt+N new project. Changes document 24 first |
| Ctrl+T | The free-transform box (D-81) | Text tool | Keep the free-transform box. Leave the Text icon out until text layers are asked for |
| Shift+F2 | Renames the layer (as F2 does) | Animate workspace | Put the workspaces on Alt+1 to Alt+4 instead, which nothing uses |
| Shift+F3 | Swaps Timeline and Graph | Sketch workspace | The same: workspaces on Alt+1 to Alt+4 |
| Ctrl+M | Export now | Add to the Render queue | Keep "export now" until the queue is built. Then it becomes "add to the queue", as in After Effects |

V, G, Q, F9, Ctrl+Space, Ctrl+Shift+P, Page Up, Page Down, Space, Ctrl+O, Ctrl+S, Ctrl+I, Ctrl+K, Ctrl+Y, Ctrl+Shift+C, Ctrl+Z and Ctrl+Shift+Z mean the same in both. H and Z on their own are free.

## How each screen will be checked

Each screen, in order, is one unit that ends with:

1. **Side by side:** the Sandbox's board beside a photograph of the built window (`tools/capture_window.ps1`).
2. **A click-through sheet:** for each control, "clicked X, the engine did Y, the picture shows Z".
3. **The four wiring checks above pass**, with their lists re-pinned in the same change.
4. **The reference exports are byte for byte unchanged.** A redesign moves controls, not pixels.
5. **Keyboard and size:** every control can be reached from the keyboard, and the window is photographed at 100, 150 and 200 per cent.
6. **A playtest sheet** for the owner.

The order:

1. W-39: the frame, the top bar and the workspaces.
2. W-40: the timeline.
3. W-41: the viewer.
4. W-42: effect controls.
5. W-43: the Project panel and the finder.
6. W-44: Animate.
7. W-45: Render.
8. W-46: Sketch, if question 2 lets it in.

## What does not change

- No command is added, removed or re-keyed by this decision; document 24 changes first, screen by screen, where a row above says so.
- No fixture changes.
- No picture an export writes changes.
- The W-31 timeline the window has today stays until W-40 replaces it.

## What to answer

1. **Is the Sandbox the target?** It would replace the W-32 to W-37 drafts, in the order above. (D-248. Recommended: yes.)
2. **Sketch.** Document 23 excludes "Drawing and vector animation", and the project's rules say only the owner can let it in. The Sandbox's Sketch workspace is drawing for notes only, never exported. You can:
   - (a) let in that narrow Sketch, recorded as a decision with its own fixtures;
   - (b) drop it;
   - (c) decide when its turn comes, as the last screen.

   Recommended: (c). Nothing before it needs Sketch; screens 1 to 7 leave out its tab and its viewer toggle until then.
3. **The shortcuts above.** Recommended:
   - Ctrl+N new composition, Ctrl+Alt+N new project;
   - Ctrl+T stays the free-transform box;
   - workspaces on Alt+1 to Alt+4;
   - Ctrl+M stays "export now" until a queue exists.
4. **Notes on the ruler.** They already exist as markers. Should a note also appear in the sheet's Action column? You can:
   - (a) show it there in grey, as a reminder; it stays a marker;
   - (b) keep the two separate, as today.

   Recommended: (a). It is page work only.
