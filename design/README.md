# Design

Interface design work for G1-core, per `Markdown/05_UX_Specification.md`.

Five screens only: empty project, normal editing, exposure editing, missing-media recovery, export running and failed. Plus a styled system: dark theme, type scale, spacing, state colors, icon direction.

Delivered as HTML and CSS, because under ADR-004 the design is the product rather than a mockup of it.

Do not design a panel for cache state. That feature is parked under D-12.

Masks and effects are **no longer parked**: D-12 was amended on 2026-09-06 and the park on masks and mattes (B-06 / R-04) and on the effect stack (B-07 / R-05) was lifted by the owner, both together. Both are built, and both already have panels in the window that this design work has to account for rather than omit: `verification/B-12a_window_and_keyboard.md` and `verification/B-12c_effects_panel.png` show what exists.

## The redesign (G4, D-175 and D-176)

Drawings of each screen, one self-contained HTML file each, opened in any browser. `?fig=` shows one figure. They are drawings, not the program: nothing in them adds, removes or re-keys a command.

- `index.html`: all of them, with a picture and a link each.
- `W-31_timeline.html`: the timeline. Accepted 2026-09-28 (D-176).
- `W-32_sheet.html`: the Sheet and the exposure list (D-64). Proposed draft.
- `W-33_viewer.html`: the viewer. Proposed draft.
- `W-34_effect_controls.html`: Effect controls. Proposed draft.
- `W-35_project_effects.html`: the Project and Effects panels. Proposed draft.
- `W-36_strips_and_states.html`: the top and bottom strips, and document 05's states. Proposed draft.
- `W-37_shortcuts.html`: the shortcut list and its editor. Proposed draft.

Their pictures are in `verification/W-3N pictures/`, and the owner's sheets are `verification/W-31a_timeline_proposal.md` and `verification/W-32_to_W-37_design_drafts.md`.
