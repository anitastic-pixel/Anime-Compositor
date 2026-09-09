# Design

Interface design work for G1-core, per `Markdown/05_UX_Specification.md`.

Five screens only: empty project, normal editing, exposure editing, missing-media recovery, export running and failed. Plus a styled system: dark theme, type scale, spacing, state colors, icon direction.

Delivered as HTML and CSS, because under ADR-004 the design is the product rather than a mockup of it.

Do not design a panel for cache state. That feature is parked under D-12.

Masks and effects are **no longer parked**: D-12 was amended on 2026-09-06 and the park on masks and mattes (B-06 / R-04) and on the effect stack (B-07 / R-05) was lifted by the owner, both together. Both are built, and both already have panels in the window that this design work has to account for rather than omit: `verification/B-12a_window_and_keyboard.md` and `verification/B-12c_effects_panel.png` show what exists.

Empty. Recommended after a spike proves the application launches and displays a composited frame.
