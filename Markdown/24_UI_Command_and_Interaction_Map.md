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
| composition.open | Put one of the project's compositions in the viewer | none | no |
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
| layer.shift | Move the selected layer along the timeline, keeping its length | [ or ] | yes |
| layer.trim | Trim the selected layer's in or out point | Alt+[ or Alt+] | yes |
| timeline.previous_frame | Step one composition frame back | Left | no |
| timeline.next_frame | Step one composition frame forward | Right | no |
| timeline.play_pause | Toggle work-area playback | Space | no |
| timeline.set_work_start | Set work-area start | B | yes/project setting |
| timeline.set_work_end | Set work-area end | N | yes/project setting |
| exposure.set_span | Assign drawing/hold span | none | yes |
| property.set_base | Set a transform property's base value | none | yes |
| keyframe.add_remove | Toggle keyframe for focused property | none | yes |
| keyframe.move | Move a keyframe to another frame | none | yes |
| effect.add | Add effect instance | none | yes |
| effect.delete | Remove selected effect | Delete when effect-focused | yes |
| effect.toggle_bypass | Bypass selected effect | none | yes |
| effect.set_parameters | Change a parameter of an existing effect | none | yes |
| effect.move_up | Move effect one step earlier in the stack | none | yes |
| effect.move_down | Move effect one step later in the stack | none | yes |
| effect.move | Move effect to a position in the stack | none | yes |
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

`composition.open` was added on 2026-09-08, the same day, and for a reason the owner found rather than a reason a document named: `composition.create` moves the window into what it made, this table had no ID for moving it anywhere else, and nothing in the window listed the compositions a project holds. Making a composition was therefore a one-way door out of the shot somebody was working on. It is not undoable and it is not an edit — nothing about the project changes, the window looks somewhere else — which puts it with `viewer.toggle_alpha` and `viewer.toggle_checkerboard` rather than with the commands above it. No shortcut: the compositions are a list in the project panel, reachable by Tab like the drawings beside them, and this table promises Ctrl+N and Ctrl+Shift+N to two other things already. Like `composition.create`, it is a new capability and the owner may cut it; cutting it means cutting that one as well, or restoring the one-way door.

`effect.move_up` and `effect.move_down` were added on 2026-09-10 by W-03, after the first sitting with the window asked for them. They are a correction of an omission rather than a new capability, in the sense `property.set_base` above is: ADR-017 evaluates a layer's effect stack in order and `AddEffect` has taken an index since B-07, so the order has always been part of the file and part of the picture, and this table named no ID for changing it. Until now a stack built in the wrong order had to be deleted and typed again, which loses every parameter with it. Two IDs rather than one, for the same reason `layer.move_up` and `layer.move_down` are two: the window offers a step at a time, and a window that could send any position would need a position to send. No shortcut — Delete is already spoken for when an effect has the focus, and this table promises Ctrl+[ and Ctrl+] to the layer pair.

`effect.move` was added on 2026-09-11 by W-07, after the second sitting with the window asked to drag an effect up and down the stack. The paragraph above said a window that could send any position would need a position to send; a card dragged to a place in the stack has one, and it is sent once, at the drop, so a drag is one entry to undo. The two arrows stay, because they are the way to do it without a mouse. No shortcut, for the reason the pair above have none.

`layer.shift` and `layer.trim` were added on 2026-09-11 by W-05, after the second sitting with the window tried to drag a layer along the timeline and moved the playhead instead. They are a correction of an omission in the sense `property.set_base` above is: document 20 has said since its first version that "moving a layer changes in_frame/out_frame; trimming and changing source offset are distinct commands", document 19 has held both frames on the layer record since B-02, and neither this table nor the core had a command for changing them, so every layer began at the frame it was added on and ended where the composition did. Two IDs rather than one because document 20 makes them two: a move keeps the layer's length and its source offset, so every frame of it shows what it showed, later; a trim keeps the drawing under each surviving frame where it was, which moves the offset with the in point, and the core does that arithmetic so that a window cannot get it wrong. The shortcuts are After Effects' own: `[` and `]` move the layer so that it starts or ends at the playhead, and with Alt held they trim that end to the playhead instead. Neither collides with `layer.move_up` and `layer.move_down`, which take the same keys with Ctrl.

The After Effects property keys were bound on 2026-09-11 by W-12, after the fourth sitting asked for them by name ("s is scale, r rotation, etc just like AE does"). With a layer selected, A, P, S, R and T open that one property under the layer on the timeline (anchor, position, scale, rotation, opacity), Shift with the key adds it to what is open, and U opens the animated ones; each pressed again closes what it opened, and the arrow beside a layer's name opens and closes all five. None of these is a command and none has a row above: they change what the timeline shows and nothing in the file, which is why they are recorded here as a paragraph rather than as rows. Alpha-only inspection, which this build had put on A alone, moved to Alt+A the same day to make room; its row still says none, because this table never promised it a key and the binding is the window's own choice, as A was.

Every number a person can change is drawn and worked the same way from 2026-09-11 by W-14, after the fifth sitting asked for it: "highlight any manipulatable value a light blue and allow click drag or click to change values like AE". A value is light blue with a line under it, it is dragged left and right to change it, a press with no drag turns it into a field with the number in it selected, Enter or leaving the field commits, Escape puts the number back, and the arrow keys move it for anyone without a mouse. This is a way of drawing and holding a value rather than a command, so it has no row: the requests a drag sends are the transaction named below, and the request a typed number sends is whichever command the field belongs to. Two of those fields changed when they were asked to: an effect's setting and an exposure's frames were sent as they were typed and when focus left, and are now sent when the number is committed, by Enter or by leaving the field, which is the rule the transform fields have kept since W-04 and the one document 26 describes as "committing/focus exit ends the transaction". The grey key beside each field that W-07 added on 2026-09-10 is gone, because the number itself is now the thing that is dragged.

Ctrl+Shift+N rather than Ctrl+N, because Ctrl+N belongs to `project.new` in the row above and this is not that command. This build binds Ctrl+Shift+N and leaves Ctrl+N unbound, so the day `project.new` is built it takes the shortcut this table already promised it.

An interaction transaction is not a command and has no row here. The three requests that carry one — beginning it, previewing a value inside it, and committing or cancelling it — produce no history record of their own; the single record they commit belongs to the command being dragged, and that command is the one this table names. `property.drag_update`, `property.drag_end` and `property.drag_cancel` are the names this build uses for them.

`keyframe.move` was added on 2026-09-12 by W-11, which is the half of the third sitting's finding 5 that W-10 left: W-10 set, removed and drew the keys, and `verification/B-12a_transform_table.md` has said since that moving one along the bar "needs a command of its own in the core so that undo replays it". This is that command, and it is a correction of an omission in the sense `property.set_base` above is: document 20 has always put a keyframe on a frame, and a keyframe put on the wrong frame could until now only be removed and set again, which is two entries in history and loses the key if the second is refused. One ID rather than two because a move is one gesture. The key keeps its value and its interpolation mode - this changes when it happens, not what it does - and a move onto a frame that already has a key of the same property is refused rather than allowed to overwrite it, because document 19 calls two keyframes at one frame invalid and losing a key silently is the worse of the two answers. No shortcut, and the row above says none for a reason: with a key focused on its property's row Left and Right move it one frame, as After Effects moves a selected key, but those arrows belong to the mark that has the keyboard rather than to the window, in the way W-14's arrows belong to the number being changed. The same mark is dragged with the pointer. A drag sends nothing until it is released, which is not the transaction below and is deliberate - redo replays a drag's commands against the state the drag began in, and a move names the frame it starts from, so an intermediate one would name a frame the key had already left. The page follows the pointer with a mark of its own instead, and either way a move is one entry to undo.

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
