# UI command and interaction map

Version 0.2 | 2026-09-04 | Proposed baseline

## Command architecture

Every state-changing UI action invokes a stable command ID through the command layer. Widgets do not mutate the project model directly. Command IDs are persistence-independent and may be rebound to shortcuts. Undo semantics are defined in 26.

## Core command IDs

| Command ID | Purpose | G1 default shortcut | Undoable |
|---|---|---|---|
| project.new | Create empty project | Ctrl+N | no |
| project.open | Open project | Ctrl+O | no |
| project.save | Save current project | Ctrl+S | no history item |
| project.save_as | Save project to new path | Ctrl+Shift+S | no history item |
| composition.create | Create a composition in the open project | Ctrl+Shift+N | yes |
| edit.undo | Undo latest command | Ctrl+Z | control |
| edit.redo | Redo latest undone command | Ctrl+Shift+Z | control |
| media.import | Import still/sequence | Ctrl+I | yes |
| media.relink | Relink missing asset | none | yes |
| layer.create | Add raster layer | Ctrl+Alt+L | yes |
| layer.delete | Delete selected layer | Delete | yes |
| layer.rename | Rename selected layer | F2 | yes |
| layer.move_up | Move layer toward front | Ctrl+] | yes |
| layer.move_down | Move layer toward back | Ctrl+[ | yes |
| layer.toggle_visibility | Toggle selected layer | none | yes |
| layer.toggle_lock | Toggle selected layer lock | none | yes |
| layer.set_matte | Choose, change or clear the layer that shapes this one | none | yes |
| timeline.previous_frame | Step one composition frame back | Left | no |
| timeline.next_frame | Step one composition frame forward | Right | no |
| timeline.play_pause | Toggle work-area playback | Space | no |
| timeline.set_work_start | Set work-area start | B | yes/project setting |
| timeline.set_work_end | Set work-area end | N | yes/project setting |
| exposure.set_span | Assign drawing/hold span | none | yes |
| property.set_base | Set a transform property's base value | none | yes |
| keyframe.add_remove | Toggle keyframe for focused property | none | yes |
| effect.add | Add effect instance | none | yes |
| effect.delete | Remove selected effect | Delete when effect-focused | yes |
| effect.toggle_bypass | Bypass selected effect | none | yes |
| effect.set_parameters | Change a parameter of an existing effect | none | yes |
| viewer.fit | Fit composition in viewer | Shift+/ | no |
| viewer.zoom_100 | Set 100% zoom | Ctrl+1 | no |
| viewer.toggle_checkerboard | Toggle transparency grid | none | no |
| viewer.toggle_alpha | Toggle alpha-only inspection | none | no |
| render.preview_current | Render current frame | none | no |
| export.sequence | Open PNG-sequence export | Ctrl+M | no |
| app.command_palette | Search commands | Ctrl+Shift+P | no |

Shortcuts are proposed defaults and must be tested for OS/framework conflicts. Users may remap commands later; command IDs remain stable.

`layer.set_matte` was added on 2026-09-07 by B-12a, for the same reason and by the same reading. W-01 requires the artist to apply a matte, `SetMatte` has existed in the command layer since B-06, and this table named no ID for reaching it. One ID covers choosing a matte, clearing it and changing whether the matte layer is still drawn in its own right, because document 19 holds all three in one record and the core takes them in one command; an ID for each would give a window a way to send half of a setting the file cannot hold in halves.

`property.set_base` was added on 2026-09-07 by B-12a. W-01 requires the artist to adjust anchors and transforms, `SetPropertyBase` has existed in the command layer since B-05, and this table named no ID for reaching it, so an inspector had nothing stable to invoke. This is a correction of an omission, not a new capability; if the owner would rather it were named something else, the string is changed in one place.

`composition.create` was added on 2026-09-08 by B-12d, and it is not a correction of an omission like the two above: no command existed in the core either, so this is a new capability and the owner may cut it. The reason it is here is that W-01 lists "creates a composition" third of its thirteen steps and nothing in this build could take that step — `project.new` above makes an empty project, and an empty project has nothing to make an empty project *into*. `verification/B-12_acceptance_run.md` recorded step 3 as **not built** for that reason. One ID covers name, size, frame rate and length because document 19 line 15 makes all four properties of the composition record, and a composition that existed before its size did would be a state no file can hold.

Ctrl+Shift+N rather than Ctrl+N, because Ctrl+N belongs to `project.new` in the row above and this is not that command. This build binds Ctrl+Shift+N and leaves Ctrl+N unbound, so the day `project.new` is built it takes the shortcut this table already promised it.

An interaction transaction is not a command and has no row here. The three requests that carry one — beginning it, previewing a value inside it, and committing or cancelling it — produce no history record of their own; the single record they commit belongs to the command being dragged, and that command is the one this table names. `property.drag_update`, `property.drag_end` and `property.drag_cancel` are the names this build uses for them.

## Focus and selection

One primary selection context exists at a time: media, layer, property/keyframe or effect. Viewer selection and timeline selection must resolve to the same layer ID. Deleting uses the focused context and must show the target clearly before destructive commands.

Keyboard focus is visible. Arrow keys step frames only when timeline/viewer transport owns focus; text/numeric fields retain normal editing behavior. Escape cancels an active drag/edit before it clears selection.

## Drag transactions

A drag starts an interaction transaction, previews model values without creating hundreds of history entries, and commits one command at release. Escape restores the pre-drag value. Losing focus unexpectedly must either commit or cancel according to a documented widget rule; it may not leave half-applied state.

## Workspace wireframe contract

G1 default layout: media bin left, composition viewer center, inspector/effects right, timeline bottom, status/diagnostics strip. Panels may be resized/docked when supported by the selected UI framework, but W-01 must be completable in the default layout at 100-200% scaling.

Required dialogs/panels: new/open/save, import sequence interpretation, missing-media/relink, recovery choice, export sequence, error details and preferences for non-project UI settings.

## Error interaction

Inline validation is preferred for correctable property input. Blocking project/media failures use a dialog with a stable diagnostic code from 28 and a concrete next action. Background render/export failures remain visible after the transient notification disappears.

## Accessibility baseline

All G1 commands required by W-01 must be keyboard reachable. Icon-only controls require accessible names/tooltips. Color-coded status must also use text/icon shape. Verify at 100%, 150% and 200% Windows scaling under T-15.

Related documents: 05, 26 and 28.
