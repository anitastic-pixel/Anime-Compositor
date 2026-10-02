# Design

Interface design work for G1-core, per `Markdown/05_UX_Specification.md`.

Five screens only: empty project, normal editing, exposure editing, missing-media recovery, export running and failed. Plus a styled system: dark theme, type scale, spacing, state colors, icon direction.

Delivered as HTML and CSS, because under ADR-004 the design is the product rather than a mockup of it.

The preview cache is no longer parked: D-37 unparked it on 2026-09-05 (B-08b), and D-223 added the finished-frame memory (RAM preview). The Sandbox below shows it as a green line of ready frames under the timeline's work area.

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

## The Sandbox (W-38, D-248 accepted 2026-10-02)

The whole window put together from the owner's picks on the design canvas (https://claude.ai/artifact/2YhV55os2pp7rtiJFLuyFh, version 25 of 2026-10-02). D-248 makes it replace the W-32 to W-37 drafts above as what the redesign builds.

- `W-38_sandbox/still_*.html`: one still picture of each board (Compose, Compose with the effects finder open, the map of compositions, Animate, Sketch, Render), opened in any browser. Nothing in them can be clicked.
- `W-38_sandbox/Sandbox.dc.html`: the canvas source of the board, the one the live canvas shows. The six boards differ only in the `START` line near the top of its script.

Their pictures are in `verification/W-38 pictures/`. The owner's sheet is `verification/W-38_sandbox_and_wiring.md`.
