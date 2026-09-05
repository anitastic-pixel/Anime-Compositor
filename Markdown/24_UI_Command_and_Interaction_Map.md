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
| timeline.previous_frame | Step one composition frame back | Left | no |
| timeline.next_frame | Step one composition frame forward | Right | no |
| timeline.play_pause | Toggle work-area playback | Space | no |
| timeline.set_work_start | Set work-area start | B | yes/project setting |
| timeline.set_work_end | Set work-area end | N | yes/project setting |
| exposure.set_span | Assign drawing/hold span | none | yes |
| keyframe.add_remove | Toggle keyframe for focused property | none | yes |
| effect.add | Add effect instance | none | yes |
| effect.delete | Remove selected effect | Delete when effect-focused | yes |
| effect.toggle_bypass | Bypass selected effect | none | yes |
| viewer.fit | Fit composition in viewer | Shift+/ | no |
| viewer.zoom_100 | Set 100% zoom | Ctrl+1 | no |
| viewer.toggle_checkerboard | Toggle transparency grid | none | no |
| viewer.toggle_alpha | Toggle alpha-only inspection | none | no |
| render.preview_current | Render current frame | none | no |
| export.sequence | Open PNG-sequence export | Ctrl+M | no |
| app.command_palette | Search commands | Ctrl+Shift+P | no |

Shortcuts are proposed defaults and must be tested for OS/framework conflicts. Users may remap commands later; command IDs remain stable.

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
